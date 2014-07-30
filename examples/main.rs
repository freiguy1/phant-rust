extern crate phant;

use phant::Phant;

fn main() {

    // Create a phant object for sparkfun's publically writable repository
    let mut phant = Phant::new("data.sparkfun.com", "Jxyjr7DmxwTD5dG1D1Kv", "gzgnB4VazkIg7GN1g1qA");

    // Add one key/value pair of data.  The keys need to be same
    phant.add("brewTemp", "posting from the rust library @ github.com/freiguy1/phant-rust");

    println!("------ url paste-able into a browser -------\n{}\n", phant.get_url());

    match phant.push() {
        Ok(result) => println!("------- result -------\n{}", result),
        Err(e) => println!("-------- error --------\n{}", e)
    }

}
