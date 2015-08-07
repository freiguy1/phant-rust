extern crate phant;

use phant::Phant;

fn main() {

    // Create a phant object for sparkfun's publically writable repository.
    // The three params are hostname, public key, and private key (provided to you
    // by the phant server).
    let mut phant = Phant::new("http://data.sparkfun.com", "Jxyjr7DmxwTD5dG1D1Kv", "gzgnB4VazkIg7GN1g1qA");

    // Add one key/value pair of data.  The keys need to be same
    phant.add("brewTemp", "posting from the rust library @ github.com/freiguy1/phant-rust");

    // The number of columns (or key/value pairs) in the row should be 1.
    assert!(phant.row_data().len() == 1);

    // Using get_url() does not erase the local row data you've added.
    println!("------ url paste-able into a browser -------\n{}\n", phant.get_url());

    // Push the row of data to the server and evaluate the response.
    // Using push() does erase the local row data you've added.
    match phant.push() {
        Ok(result) => println!("------- result -------\n{}", result),
        Err(e) => println!("-------- error --------\n{}", e)
    }

    // The number of columns in the row should now be 0.
    assert!(phant.row_data().len() == 0);


    // Creating a brand new stream
    let phant_result = phant::Phant::create_stream("https://data.sparkfun.com",
        phant::StreamSpec {
            title: "My Title".to_string(),
            description: "My description".to_string(),
            fields: vec!["a".to_string(), "b".to_string()],
            hidden: true 
        });

    if let Err(e) = phant_result {
        panic!("{}", e);
    }

    // Push to this new stream
    let mut phant = phant_result.ok().unwrap();
    phant.add("a", "1234");
    phant.add("b", "Chicago");
    match phant.push() {
        Ok(result) => println!("------- result -------\n{}", result),
        Err(e) => println!("-------- error --------\n{}", e)
    }

    println!("public: {} private: {}", phant.public_key(), phant.private_key());
}
