#![crate_name = "phant"]

extern crate url;

use std::collections::hashmap::HashMap;
use std::io::TcpStream;

/// A structure which holds the data for a Phant data stream
///
/// # Example
/// ```
/// let mut phant = phant::Phant::new("data.sparkfun.com", "123abc", "456def");
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
    pub fn push(&mut self) -> std::io::IoResult<String> {
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
    pub fn clear_server(&mut self) -> std::io::IoResult<String> {
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
    /// let mut phant = phant::Phant::new("data.sparkfun.com".to_string(), "123abc", "456def");
    ///
    /// phant.add("apple_color", "red");
    /// let url = phant.get_url();
    /// assert!(url.as_slice() == "http://data.sparkfun.com/input/123abc?private_key=456def&apple_color=red")
    /// ```
    pub fn get_url(&self) -> String {
        format!("http://{}/input/{}?private_key={}&{}", self.hostname, self.public_key, self.private_key, self.data_query_string())
    }

    fn data_query_string(&self) -> String {
        let mut result_list = Vec::new();
        for (key, value) in self.data.iter() {
            result_list.push(format!("{}={}", key, value));
        }
        url::encode(result_list.connect("&"))
    }
}
