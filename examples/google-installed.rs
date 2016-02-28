extern crate inth_oauth2;

use std::io;

use inth_oauth2::Client;
use inth_oauth2::provider::google::Installed;

fn main() {
    let client = Client::<Installed>::new(
        String::from("143225766783-ip2d9qv6sdr37276t77luk6f7bhd6bj5.apps.googleusercontent.com"),
        String::from("3kZ5WomzHFlN2f_XbhkyPd3o"),
        Some(String::from("urn:ietf:wg:oauth:2.0:oob"))
    );

    let auth_uri = client.auth_uri(Some("https://www.googleapis.com/auth/userinfo.email"), None)
        .unwrap();
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let http_client = Default::default();

    let token = client.request_token(&http_client, code.trim()).unwrap();
    println!("{:?}", token);

    let token = client.refresh_token(&http_client, token, None).unwrap();
    println!("{:?}", token);
}
