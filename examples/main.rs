extern crate phant;

use phant::Phant;

fn main() {

    // Create a phant object for sparkfun's publically writable repository.
    // The three params are hostname, public key, and private key (provided to you
    // by the phant server).
    let mut phant = Phant::new("data.sparkfun.com", "Jxyjr7DmxwTD5dG1D1Kv", "gzgnB4VazkIg7GN1g1qA");

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

}
