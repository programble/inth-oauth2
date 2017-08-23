extern crate hyper;
extern crate hyper_native_tls;
extern crate inth_oauth2;

use std::io;

use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use inth_oauth2::Client;
use inth_oauth2::provider::google::Web;

fn main() {
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let https = hyper::Client::with_connector(connector);

    let client = Client::new(
        Web,
        String::from("143225766783-0h4h5ktpvhc7kqp6ohbpd2sssqrap57n.apps.googleusercontent.com"),
        String::from("7Xjn-vRN-8qsz3Zh9zZGkHsM"),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );

    let auth_uri = client.auth_uri(Some("https://www.googleapis.com/auth/userinfo.email"), None)
        .unwrap();
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(&https, code.trim()).unwrap();
    println!("{:?}", token);
}
