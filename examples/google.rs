extern crate inth_oauth2;

use inth_oauth2::Client;

fn main() {
    let client = Client {
        authorization_uri: String::from("https://accounts.google.com/o/oauth2/auth"),
        client_id: String::from("143225766783-ip2d9qv6sdr37276t77luk6f7bhd6bj5.apps.googleusercontent.com"),
        client_secret: String::from("3kZ5WomzHFlN2f_XbhkyPd3o"),
        redirect_uri: Some(String::from("urn:ietf:wg:oauth:2.0:oob")),
    };

    let auth_uri = client.authorization_uri(
        Some("https://www.googleapis.com/auth/userinfo.email"),
        None
    ).unwrap();

    println!("{}", auth_uri);
}
