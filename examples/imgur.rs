extern crate hyper;
extern crate hyper_native_tls;
extern crate inth_oauth2;

use std::io;

use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use inth_oauth2::Client;
use inth_oauth2::provider::Imgur;

fn main() {
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let https = hyper::Client::with_connector(connector);

    let client = Client::<Imgur>::new(
        String::from("505c8ca804230e0"),
        String::from("c898d8cf28404102752b2119a3a1c6aab49899c8"),
        Some(String::from("https://cmcenroe.me/oauth2-paste/"))
    );

    let auth_uri = client.auth_uri(None, None).unwrap();
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(&https, code.trim()).unwrap();
    println!("{:?}", token);

    let token = client.refresh_token(&https, token, None).unwrap();
    println!("{:?}", token);
}
