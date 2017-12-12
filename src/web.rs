use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType, Headers, Accept, qitem};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use ::error::Error;

header! { (PhantPrivateKey, "Phant-Private-Key") => [String] }
header! { (PhantDeleteKey, "Phant-Delete-Key") => [String] }

pub fn create_stream(hostname: &String, body: &String) -> Result<String, Error> {
    // let client = Client::new();
    // let response = client
        // .post(&format!("{}/streams", hostname))
        // .body(body)
        // .header(ContentType::json())
        // .header(Accept(vec![
                       // qitem(Mime(TopLevel::Application, SubLevel::Json,
                                  // vec![(Attr::Charset, HyperValue::Utf8)]))]))
        // .send();
    // response

    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let uri = format!("{}/streams", hostname).parse()?;
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(body.len() as u64));
    req.headers_mut().set(Accept::json());
    req.set_body(body.clone().into_bytes());
    let work = client.request(req).and_then(|res| {
        res.body().concat2().map(move |body| {
            ::std::str::from_utf8(&body).unwrap().to_string()
        })
    });
    Ok(core.run(work)?)
}

pub fn push_data(hostname: &String,
                 public_key: &String,
                 private_key: &String,
                 body: &String) -> Result<String, Error> {
    // let client = Client::new();
    // let response = client
        // .post(&format!("{}/input/{}", hostname, public_key))
        // .body(data)
        // .header(PhantPrivateKey(private_key.clone()))
        // .header(ContentType::form_url_encoded())
        // .header(Accept(vec![
                       // qitem(Mime(TopLevel::Application, SubLevel::Json,
                                  // vec![(Attr::Charset, HyperValue::Utf8)]))]))
        // .send();
    // response

    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let uri = format!("{}/input/{}", hostname, public_key).parse()?;
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(body.len() as u64));
    req.headers_mut().set(PhantPrivateKey(private_key.clone()));
    req.headers_mut().set(Accept::json());
    req.set_body(body.clone().into_bytes());
    let work = client.request(req).and_then(|res| {
        res.body().concat2().map(move |body| {
            ::std::str::from_utf8(&body).unwrap().to_string()
        })
    });
    Ok(core.run(work)?)
}

pub fn clear_data(hostname: &String,
                  public_key: &String,
                  private_key: &String) -> Result<String, Error> {
    // let client = Client::new();
    // let response = client
        // .delete(&format!("{}/input/{}", hostname, public_key))
        // .header(PhantPrivateKey(private_key.clone()))
        // .send();
    // response
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let uri = format!("{}/input/{}", hostname, public_key).parse()?;
    let mut req = Request::new(Method::Delete, uri);
    req.headers_mut().set(PhantPrivateKey(private_key.clone()));
    let work = client.request(req).and_then(|res| {
        res.body().concat2().map(move |body| {
            ::std::str::from_utf8(&body).unwrap().to_string()
        })
    });
    Ok(core.run(work)?)
}

pub fn delete_stream(hostname: &String,
                     public_key: &String,
                     delete_key: &String) -> Result<String, Error> {
    // let client = Client::new();
    // let response = client
        // .delete(&format!("{}/input/{}", hostname, public_key))
        // .header(PhantDeleteKey(delete_key.clone()))
        // .send();
    // response

    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let uri = format!("{}/input/{}", hostname, public_key).parse()?;
    let mut req = Request::new(Method::Delete, uri);
    req.headers_mut().set(PhantDeleteKey(delete_key.clone()));
    let work = client.request(req).and_then(|res| {
        res.body().concat2().map(move |body| {
            ::std::str::from_utf8(&body).unwrap().to_string()
        })
    });
    Ok(core.run(work)?)
}
