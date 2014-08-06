extern crate url;

use std::collections::hashmap::HashMap;
use std::io::TcpStream;

/// A structure which holds the data for a Phant data stream
///
/// # Example
/// ```
/// let mut phant = phant::Phant::new("data.sparkfun.com", "your_public_key", "your_private_key");
///
/// phant.add("computer_name", "my-computer");
/// phant.add("external_ip", "123.321.111.222");
/// phant.add("internal_ip", "192.168.1.104");
///
/// phant.push().ok().expect("Pushing to server did not succeed");
/// ```

#[deriving(Show)]
pub struct Phant {
    hostname: String,
    public_key: String,
    private_key: String,
    data: HashMap<String, String>
}

impl Phant {
    pub fn new(hostname: &str, public_key: &str, private_key: &str) -> Phant {
        Phant {
            hostname: String::from_str(hostname),
            public_key: String::from_str(public_key),
            private_key: String::from_str(private_key),
            data: HashMap::new()
        }
    }

    /// Adds a key-value pair to the current phant dataset.  The `key` is the
    /// phant column name.  The `value` is a `.to_string()`-able data item. This function
    /// does not manipulate server data.
    pub fn add<T: ToString>(&mut self, key: &str, value: T) {
        self.data.insert(key.to_string(), value.to_string());
    }

    /// Pushes data to the server. All the data that has been `add`ed will be pushed as a row
    /// to the phant server located at `hostname`. The local row will then be cleared so `add`
    /// can be called again to repeat the process.
    ///
    /// Returns an [IoResult](http://doc.rust-lang.org/std/io/type.IoResult.html) where a successful
    /// push will return the response from the server in `String` form.
    pub fn push(&mut self) -> ::std::io::IoResult<String> {
        let query_string = self.data_query_string();
        let request: String = format!("POST /input/{} HTTP/1.0\nPhant-Private-Key: {}\nContent-Type: application/x-www-form-urlencoded\nContent-Length: {}\n\n{}\n\n",
            self.public_key, self.private_key, query_string.len(), query_string);
        let mut socket = try!(TcpStream::connect(self.hostname.as_slice(), 80));
        try!(socket.write_str(request.as_slice()));
        let result = try!(socket.read_to_string());
        self.clear_local();
        Ok(result)
    }

    /// Clears the data locally.  Any key-value pairs that have been added since creation or
    /// previous clear will be removed.
    pub fn clear_local(&mut self) {
        self.data.clear();
    }

    /// Clears the data on the server.
    pub fn clear_server(&mut self) -> ::std::io::IoResult<String> {
        let request: String = format!("DELETE /input/{} HTTP/1.0\nPhant-Private-Key: {}\n\n", self.public_key, self.private_key);
        let mut socket = TcpStream::connect(self.hostname.as_slice(), 80).unwrap();
        try!(socket.write_str(request.as_slice()));
        socket.read_to_string()
    }

    /// Gets a formatted URL in string form for adding the current row to the server.
    /// Local data is _not_ cleared, unlike the `push()` method.
    ///
    /// # Example
    /// ```
    /// let mut phant = phant::Phant::new("data.sparkfun.com", "your_public_key", "your_private_key");
    ///
    /// phant.add("apple_color", "red");
    /// let url = phant.get_url();
    /// assert!(url.as_slice() == "http://data.sparkfun.com/input/your_public_key?private_key=your_private_key&apple_color=red")
    /// ```
    pub fn get_url(&self) -> String {
        format!("http://{}/input/{}?private_key={}&{}", self.hostname, self.public_key, self.private_key, self.data_query_string())
    }

    /// Returns a clone of the data for the current row being added to. Notice: it's a
    /// clone, so manipulations done to this returned object will _not_ modify the
    /// data being held locally in the `Phant` object.
    pub fn row_data(&self) -> HashMap<String, String> {
        self.data.clone()
    }

    fn data_query_string(&self) -> String {
        let mut result_list = Vec::new();
        for (key, value) in self.data.iter() {
            result_list.push(format!("{}={}", key, value));
        }
        url::encode(result_list.connect("&"))
    }
}

#[cfg(test)]
mod test {
    use super::Phant;

    #[test]
    fn test_get_url() {
        let expected = "http://data.com/input/pub?private_key=priv&color=red".to_string();
        let p = basic_phant();
        assert_eq!(p.row_data().len(), 1);
        assert_eq!(p.get_url(), expected);
        assert_eq!(p.row_data().len(), 1);
    }

    #[test]
    fn test_row_data() {
        let expected_key = "color".to_string();
        let expected_value = "red".to_string();
        let p = basic_phant();
        let mut row_data = p.row_data();
        assert_eq!(row_data.len(), 1);
        assert_eq!(*row_data.get(&expected_key), expected_value);

        // Test that row_data actually makes a clone of the data, and not returning it directly
        row_data.insert("size".to_string(), "large".to_string());
        assert_eq!(p.row_data().len(), 1);
    }

    #[test]
    fn test_overwrite_column() {
        let key = "color".to_string();
        let first_value = "red".to_string();
        let second_value = "green".to_string();
        let mut p = basic_phant();
        assert_eq!(*p.row_data().get(&key), first_value);

        p.add(key.as_slice(), second_value.as_slice());
        assert_eq!(p.row_data().len(), 1);
        assert_eq!(p.row_data().get(&key), &second_value);
    }

    #[test]
    fn test_clear_local() {
        let mut p = basic_phant();
        assert!(p.row_data().len() > 0);
        p.clear_local();
        assert_eq!(p.row_data().len(), 0);
    }

    fn basic_phant() -> Phant {
        let mut p = Phant::new("data.com", "pub", "priv");
        p.add("color", "red");
        p
    }
}
