extern crate inth_oauth2;

use std::io;
use inth_oauth2::{Client, Imgur};

fn main() {
    let client = Client::<Imgur>::new(
        Default::default(),
        "505c8ca804230e0",
        "c898d8cf28404102752b2119a3a1c6aab49899c8",
        Some("https://cmcenroe.me/oauth2-paste/")
    );

    let auth_uri = client.auth_uri(None, None).unwrap();

    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token_pair = client.request_token(code.trim()).unwrap();

    println!("{:?}", token_pair);
}
