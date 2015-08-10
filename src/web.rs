use hyper::Client;
use hyper::client::response::Response;
use hyper::error::Error;
use hyper::header::{ qitem, Accept, ContentType };
use hyper::mime::{Mime, TopLevel, SubLevel, Attr };
use hyper::mime::Value as HyperValue;

header! { (PhantPrivateKey, "Phant-Private-Key") => [String] }
header! { (PhantDeleteKey, "Phant-Delete-Key") => [String] }

pub fn create_stream(hostname: &String, body: &String) -> Result<Response, Error> {
    let client = Client::new();
    let response = client
        .post(&format!("{}/streams", hostname))
        .body(body)
        .header(ContentType::json())
        .header(Accept(vec![
                       qitem(Mime(TopLevel::Application, SubLevel::Json,
                                  vec![(Attr::Charset, HyperValue::Utf8)]))]))
        .send();
    response
}

pub fn push_data(hostname: &String,
                 public_key: &String,
                 private_key: &String,
                 data: &String) -> Result<Response, Error> {
    let client = Client::new();
    let response = client
        .post(&format!("{}/input/{}", hostname, public_key))
        .body(data)
        .header(PhantPrivateKey(private_key.clone()))
        .header(ContentType::form_url_encoded())
        .header(Accept(vec![
                       qitem(Mime(TopLevel::Application, SubLevel::Json,
                                  vec![(Attr::Charset, HyperValue::Utf8)]))]))
        .send();
    response
}

pub fn clear_data(hostname: &String,
                  public_key: &String,
                  private_key: &String) -> Result<Response, Error> {
    let client = Client::new();
    let response = client
        .delete(&format!("{}/input/{}", hostname, public_key))
        .header(PhantPrivateKey(private_key.clone()))
        .send();
    response
}

pub fn delete_stream(hostname: &String,
                     public_key: &String,
                     delete_key: &String) -> Result<Response, Error> {
    let client = Client::new();
    let response = client
        .delete(&format!("{}/input/{}", hostname, public_key))
        .header(PhantDeleteKey(delete_key.clone()))
        .send();
    response
}
