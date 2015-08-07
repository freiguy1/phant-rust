use std::collections::HashMap;
use std::io::Read;
use url::percent_encoding::{ utf8_percent_encode, DEFAULT_ENCODE_SET };
use std::convert::{ From, AsRef };

use hyper::Client;
use hyper::header::ContentType;

use ::phant::error::Error;

pub mod error;


header! { (PhantPrivateKey, "Phant-Private-Key") => [String] }

/// A structure which holds the data for a Phant data stream
///
/// # Example
/// ```
/// let mut phant = phant::Phant::new("http://data.sparkfun.com", "Jxyjr7DmxwTD5dG1D1Kv",
/// "gzgnB4VazkIg7GN1g1qA");
///
/// phant.add("brewTemp", "posting from the rust library @ github.com/freiguy1/phant-rust");
///
/// phant.push().ok().expect("Pushing to server did not succeed");
/// ```

#[derive(Debug)]
pub struct Phant {
    hostname: String,
    public_key: String,
    private_key: String,
    data: HashMap<String, String>
}

impl Phant {
    pub fn new(hostname: &str, public_key: &str, private_key: &str) -> Phant {
        Phant {
            hostname: String::from(hostname),
            public_key: String::from(public_key),
            private_key: String::from(private_key),
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
    /// Returns a Result with error type being custom type phant::Error
    pub fn push(&mut self) -> Result<String, Error> {
        let query_string = self.data_query_string();
        let client = Client::new();
        let mut post_response = try!(client
            .post(&format!("{}/input/{}", self.hostname, self.public_key))
            .body(&query_string)
            .header(PhantPrivateKey(self.private_key.clone()))
            .header(ContentType::form_url_encoded())
            .send()
            );
        let mut response_body = String::new();
        try!(post_response.read_to_string(&mut response_body));
        self.clear_local();
        Ok(response_body)
    }

    /// Clears the data locally.  Any key-value pairs that have been added since creation or
    /// previous clear will be removed.
    pub fn clear_local(&mut self) {
        self.data.clear();
    }

    /// Clears the data on the server.
    pub fn clear_server(&mut self) -> Result<String, Error> {
        let client = Client::new();
        let mut delete_response = try!(client
            .delete(&format!("{}/input/{}", self.hostname, self.public_key))
            .header(PhantPrivateKey(self.private_key.clone()))
            .send());
        let mut response_body = String::new();
        try!(delete_response.read_to_string(&mut response_body));
        Ok(response_body)
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
    /// assert_eq!(url, String::from("http://data.sparkfun.com/input/your_public_key?private_key=your_private_key&apple_color=red"))
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
        utf8_percent_encode(result_list.connect("&").as_ref(), DEFAULT_ENCODE_SET)
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
        assert_eq!(row_data[&expected_key], expected_value);

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
        assert_eq!(p.row_data()[&key], first_value);

        p.add(&key, &second_value);
        assert_eq!(p.row_data().len(), 1);
        assert_eq!(p.row_data()[&key], second_value);
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
