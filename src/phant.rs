use std::collections::HashMap;
use std::io::Read;
use url::percent_encoding::{ utf8_percent_encode, DEFAULT_ENCODE_SET };
use std::convert::{ From, AsRef };

use serde_json::builder::ObjectBuilder;
use serde_json as json;

use ::error::Error;

header! { (PhantPrivateKey, "Phant-Private-Key") => [String] }

/// Contains data for creating a new stream
pub struct StreamSpec {
    pub title: String,
    pub description: String,
    pub fields: Vec<String>,
    pub hidden: bool,
    pub alias: Option<String>,
    pub tags: Option<Vec<String>>
}

impl StreamSpec {
    fn serialize(&self) -> String {
        let mut object_builder = ObjectBuilder::new()
            .insert("title".to_string(), &self.title)
            .insert("description".to_string(), &self.description)
            .insert("fields".to_string(), self.fields.connect(","))
            .insert("hidden".to_string(), if self.hidden { 1 } else { 0 });
        if let Some(ref alias) = self.alias {
            object_builder = object_builder.insert(
                "alias".to_string(),
                &alias);
        }
        if let Some(ref tags) = self.tags {
            if tags.len() > 0 {
                object_builder = object_builder.insert(
                    "tags".to_string(),
                    tags.connect(","));
            }
        }
        json::to_string(&object_builder.unwrap()).ok().unwrap()
    }
}


/// A structure which holds the data for a Phant data stream
///
/// # Example
/// ```
/// let mut phant = phant::Phant::new("https://data.sparkfun.com", "Jxyjr7DmxwTD5dG1D1Kv",
/// "gzgnB4VazkIg7GN1g1qA", None);
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
    delete_key: Option<String>,
    data: HashMap<String, String>
}

impl Phant {

    /// Create a brand new stream on a phant server.
    ///
    /// # Example
    /// ```
    /// let phant_result = phant::Phant::create_stream("https://data.sparkfun.com",
    ///     phant::StreamSpec {
    ///         title: "My Title".to_string(),
    ///         description: "My description".to_string(),
    ///         fields: vec!["a".to_string(), "b".to_string()],
    ///         hidden: true,
    ///         tags: None,
    ///         alias: None
    ///     });
    /// 
    /// assert!(phant_result.is_ok());
    ///
    /// // Clean up!
    /// let delete_result = phant_result.ok().unwrap().delete_stream();
    ///
    /// assert!(delete_result.is_ok());
    /// ```
    pub fn create_stream(hostname: &str, spec: StreamSpec) -> Result<Phant, Error> {
        let serialized_spec = spec.serialize();
        let mut response = try!(::web::create_stream(&hostname.to_string(), &serialized_spec));
        let mut response_body = String::new();
        try!(response.read_to_string(&mut response_body));
        let data: json::Value = json::from_str(&response_body).unwrap();
        let data_object = data.as_object().unwrap();
        let success_value = data_object.get("success").unwrap();
        match success_value.as_boolean().unwrap() {
            true => {
                let public_key_value = data_object.get("publicKey").unwrap();
                let private_key_value = data_object.get("privateKey").unwrap();
                let delete_key_value = data_object.get("deleteKey").unwrap();
                Ok(Phant {
                    hostname: hostname.to_string(),
                    public_key: public_key_value.as_string().unwrap().to_string(),
                    private_key: private_key_value.as_string().unwrap().to_string(),
                    delete_key: Some(delete_key_value.as_string().unwrap().to_string()),
                    data: HashMap::new()
                })
            }
            false => {
                let message_value = data_object.get("message").unwrap();
                Err(Error::Phant(message_value.as_string().unwrap().to_string()))
            }
        }
    }

    pub fn new(
        hostname: &str,
        public_key: &str,
        private_key: &str,
        delete_key: Option<&str>) -> Phant {
        Phant {
            hostname: String::from(hostname),
            public_key: String::from(public_key),
            private_key: String::from(private_key),
            delete_key: delete_key.map(|dk| dk.to_string()),
            data: HashMap::new()
        }
    }

    pub fn delete_stream(&self) -> Result<(), Error> {
        if let Some(ref delete_key) = self.delete_key {
            try!(::web::delete_stream(
                    &self.hostname,
                    &self.public_key,
                    &delete_key));
            Ok(())
        } else {
            Err(Error::Phant("Delete key not provided".to_string()))
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
        let mut post_response = try!(::web::push_data(&self.hostname,
                                                     &self.public_key,
                                                     &self.private_key,
                                                     &query_string));
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
        let mut clear_response = try!(::web::clear_data(&self.hostname,
                                                     &self.public_key,
                                                     &self.private_key));
        let mut response_body = String::new();
        try!(clear_response.read_to_string(&mut response_body));
        Ok(response_body)
    }

    /// Gets a formatted URL in string form for adding the current row to the server.
    /// Local data is _not_ cleared, unlike the `push()` method.
    ///
    /// # Example
    /// ```
    /// let mut phant = phant::Phant::new("https://data.sparkfun.com", "your_public_key", "your_private_key", None);
    ///
    /// phant.add("apple_color", "red");
    /// let url = phant.get_url();
    /// assert_eq!(url, String::from("https://data.sparkfun.com/input/your_public_key?private_key=your_private_key&apple_color=red"))
    /// ```
    pub fn get_url(&self) -> String {
        format!("{}/input/{}?private_key={}&{}", self.hostname, self.public_key, self.private_key, self.data_query_string())
    }

    /// Returns a clone of the data for the current row being added to. Notice: it's a
    /// clone, so manipulations done to this returned object will _not_ modify the
    /// data being held locally in the `Phant` object.
    pub fn row_data(&self) -> HashMap<String, String> {
        self.data.clone()
    }

    /// Returns a clone of the public key for this phant struct
    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }

    /// Returns a clone of the private key for this phant struct
    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }

    /// Returns a clone of the optional delete key for this phant struct
    pub fn delete_key(&self) -> Option<String> {
        self.delete_key.clone()
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
        let expected = "https://data.com/input/pub?private_key=priv&color=red".to_string();
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
        let mut p = Phant::new("https://data.com", "pub", "priv", None);
        p.add("color", "red");
        p
    }
}
