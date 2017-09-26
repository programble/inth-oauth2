extern crate reqwest;
extern crate inth_oauth2;
extern crate url;

use inth_oauth2::Client;
use inth_oauth2::provider::*;
use url::Url;

fn assert_get_uri_ok(uri: Url) {
    let response = reqwest::get(uri).unwrap();
    assert_eq!(reqwest::StatusCode::Ok, response.status());
}

#[test]
fn google_web_auth_uri_ok() {
    let client = Client::new(
        google::Web,
        String::from("143225766783-0h4h5ktpvhc7kqp6ohbpd2sssqrap57n.apps.googleusercontent.com"),
        String::new(),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );
    let auth_uri = client.auth_uri(
        Some("https://www.googleapis.com/auth/userinfo.email"),
        Some("state"),
    );
    assert_get_uri_ok(auth_uri);
}

#[test]
fn google_installed_auth_uri_ok() {
    let client = Client::new(
        google::Installed,
        String::from("143225766783-ip2d9qv6sdr37276t77luk6f7bhd6bj5.apps.googleusercontent.com"),
        String::new(),
        Some(String::from("urn:ietf:wg:oauth:2.0:oob")),
    );
    let auth_uri = client.auth_uri(
        Some("https://www.googleapis.com/auth/userinfo.email"),
        Some("state"),
    );
    assert_get_uri_ok(auth_uri);
}

#[test]
fn github_auth_uri_ok() {
    let client = Client::new(
        GitHub,
        String::from("01774654cd9a6051e478"),
        String::new(),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );
    let auth_uri = client.auth_uri(Some("user"), Some("state"));
    assert_get_uri_ok(auth_uri);
}

#[test]
fn imgur_auth_uri_ok() {
    let client = Client::new(
        Imgur,
        String::from("505c8ca804230e0"),
        String::new(),
        Some(String::from("https://cmcenroe.me/oauth2-paste/")),
    );
    let auth_uri = client.auth_uri(None, Some("state"));
    assert_get_uri_ok(auth_uri);
}
