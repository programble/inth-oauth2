extern crate inth_oauth2;

use std::io;

use inth_oauth2::Client;
use inth_oauth2::provider::google::Web;

fn main() {
    let client = Client::<Web>::new(
        String::from("143225766783-0h4h5ktpvhc7kqp6ohbpd2sssqrap57n.apps.googleusercontent.com"),
        String::from("7Xjn-vRN-8qsz3Zh9zZGkHsM"),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );

    let auth_uri = client.auth_uri(Some("https://www.googleapis.com/auth/userinfo.email"), None)
        .unwrap();
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    let token = client.request_token(&Default::default(), code.trim()).unwrap();
    println!("{:?}", token);
}
