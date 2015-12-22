extern crate inth_oauth2;

use inth_oauth2::client::Client;
use inth_oauth2::provider::Imgur;

fn main() {
    let client = Client::<Imgur>::new(
        Default::default(),
        "505c8ca804230e0",
        "c898d8cf28404102752b2119a3a1c6aab49899c8",
        Some("https://cmcenroe.me/oauth2-paste/")
    );

    let auth_uri = client.auth_uri(None, None).unwrap();
    println!("{}", auth_uri);
}
