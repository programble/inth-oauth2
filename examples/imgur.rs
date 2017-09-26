extern crate reqwest;
extern crate inth_oauth2;

use std::io;

use inth_oauth2::Client;
use inth_oauth2::provider::Imgur;

fn main() {
    let http_client = reqwest::Client::new().unwrap();

    let client = Client::new(
        Imgur,
        String::from("505c8ca804230e0"),
        String::from("c898d8cf28404102752b2119a3a1c6aab49899c8"),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );

    let auth_uri = client.auth_uri(None, None);
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(&http_client, code.trim()).unwrap();
    println!("{:?}", token);

    let token = client.refresh_token(&http_client, token, None).unwrap();
    println!("{:?}", token);
}
