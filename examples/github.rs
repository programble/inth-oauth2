extern crate hyper;
extern crate inth_oauth2;

use std::io;
use inth_oauth2::Client;

fn main() {
    let client = Client::github(
        "01774654cd9a6051e478",
        "9f14d16d95d605e715ec1a9aecec220d2565fd5c",
        Some("https://cmcenroe.me/oauth2-paste")
    );

    let auth_uri = client.auth_uri(Some("user"), None).unwrap();

    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(hyper::Client::new(), code.trim()).unwrap();

    println!("{:?}", token);
}
