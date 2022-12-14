use super::super::{get_instance, router_index, setup_env};

use rocket::http::{ContentType, Status};
use rocket::local::Client;
use std::fs;

fn extract_id(from: &str) -> Option<String> {
    from.rfind('/')
        .map(|i| &from[(i + 1)..])
        .map(|s| s.trim_end().to_string())
}

#[test]
fn check_index() {
    let client = Client::new(get_instance()).unwrap();

    // Ensure the index returns what we expect.
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    assert_eq!(response.body_string(), Some(router_index::index().into()))
}

fn upload_paste(client: &Client, body: &str) -> String {
    let mut response = client.post("/").body(body).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::Plain));
    extract_id(&response.body_string().unwrap()).unwrap()
}

fn download_paste(client: &Client, id: &str) -> String {
    let mut response = client.get(format!("/{}", id)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let result = response.body_string().unwrap();

    return result;
}

fn delete_files(ids: &[&str]) {
    for id in ids {
        let filename = format!("upload/{id}", id = id);

        fs::remove_file(&filename).expect("testing expect");
    }
}

#[test]
fn pasting() {
    setup_env();

    let client = Client::new(get_instance()).unwrap();

    // Do a trivial upload, just to make sure it works.
    let body_1 = "Hello, world!";
    let id_1 = upload_paste(&client, body_1);
    assert_eq!(download_paste(&client, &id_1), body_1);

    // Make sure we can keep getting that paste.
    assert_eq!(download_paste(&client, &id_1), body_1);
    assert_eq!(download_paste(&client, &id_1), body_1);
    assert_eq!(download_paste(&client, &id_1), body_1);

    // Upload some unicode.
    let body_2 = "こんにちは";
    let id_2 = upload_paste(&client, body_2);
    assert_eq!(download_paste(&client, &id_2), body_2);

    // Make sure we can get both pastes.
    assert_eq!(download_paste(&client, &id_1), body_1);
    assert_eq!(download_paste(&client, &id_2), body_2);
    assert_eq!(download_paste(&client, &id_1), body_1);
    assert_eq!(download_paste(&client, &id_2), body_2);

    // Now a longer upload.
    let body_3 = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed
        do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
        ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut
        aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit
        in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui
        officia deserunt mollit anim id est laborum.";
    let id_3 = upload_paste(&client, body_3);
    assert_eq!(download_paste(&client, &id_3), body_3);
    assert_eq!(download_paste(&client, &id_1), body_1);
    assert_eq!(download_paste(&client, &id_2), body_2);

    delete_files(&[&id_1, &id_2, &id_3]);
}
