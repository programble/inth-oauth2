extern crate hyper;
extern crate hyper_native_tls;
extern crate inth_oauth2;

use std::io;

use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use inth_oauth2::Client;
use inth_oauth2::provider::GitHub;

fn main() {
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let https = hyper::Client::with_connector(connector);

    let client = Client::<GitHub>::new(
        String::from("01774654cd9a6051e478"),
        String::from("9f14d16d95d605e715ec1a9aecec220d2565fd5c"),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );

    let auth_uri = client.auth_uri(Some("user"), None).unwrap();
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(&https, code.trim()).unwrap();
    println!("{:?}", token);
}
