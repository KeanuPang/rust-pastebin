use super::super::{get_instance, setup_env};

use rocket::local::Client;

#[test]
#[ignore]
fn rewrite_get_put() {
    setup_env();

    let client = Client::new(get_instance()).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello, fairings!".into()));
}

#[test]
fn counts() {
    setup_env();

    let client = Client::new(get_instance()).unwrap();

    // Issue 1 GET request.
    client.get("/").dispatch();

    // Check the GET count, taking into account _this_ GET request.
    let mut response = client.get("/counts").dispatch();
    assert_eq!(response.body_string(), Some("Get: 2\nPost: 0".into()));

    // Issue 1 more GET request and a POST.
    client.get("/").dispatch();
    client.post("/").dispatch();

    // Check the counts.
    let mut response = client.get("/counts").dispatch();
    assert_eq!(response.body_string(), Some("Get: 4\nPost: 1".into()));
}

#[test]
#[ignore]
fn token() {
    setup_env();

    let client = Client::new(get_instance()).unwrap();

    // Ensure the token is '123', which is what we have in `Rocket.toml`.
    let mut res = client.get("/token").dispatch();
    assert_eq!(res.body_string(), Some("123".into()));
}
