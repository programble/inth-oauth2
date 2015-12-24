extern crate inth_oauth2;

use std::io;

use inth_oauth2::client::Client;
use inth_oauth2::provider::Google;

fn main() {
    let client = Client::<Google>::new(
        Default::default(),
        "143225766783-ip2d9qv6sdr37276t77luk6f7bhd6bj5.apps.googleusercontent.com",
        "3kZ5WomzHFlN2f_XbhkyPd3o",
        Some("urn:ietf:wg:oauth:2.0:oob")
    );

    let auth_uri = client.auth_uri(Some("https://www.googleapis.com/auth/userinfo.email"), None)
        .unwrap();
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(code.trim()).unwrap();
    println!("{:?}", token);
}
