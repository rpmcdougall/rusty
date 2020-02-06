#![feature(proc_macro_hygiene, decl_macro)]

use rusty::rocket;

#[test]
fn test_hello() {
    use rocket::http::Status;
    use rocket::local::Client;
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get("/api/test").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.body_string(),
        Some("{\"test\":\"it\'s working!\"}".into())
    );
}
