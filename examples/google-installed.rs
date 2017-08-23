extern crate hyper;
extern crate hyper_native_tls;
extern crate inth_oauth2;

use std::io;

use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;
use inth_oauth2::Client;
use inth_oauth2::provider::google::{Installed, REDIRECT_URI_OOB};

fn main() {
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let https = hyper::Client::with_connector(connector);

    let client = Client::new(
        Installed,
        String::from("143225766783-ip2d9qv6sdr37276t77luk6f7bhd6bj5.apps.googleusercontent.com"),
        String::from("3kZ5WomzHFlN2f_XbhkyPd3o"),
        Some(String::from(REDIRECT_URI_OOB)),
    );

    let auth_uri = client.auth_uri(Some("https://www.googleapis.com/auth/userinfo.email"), None)
        .unwrap();
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(&https, code.trim()).unwrap();
    println!("{:?}", token);

    let token = client.refresh_token(&https, token, None).unwrap();
    println!("{:?}", token);
}
