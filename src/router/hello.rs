#[get("/hello", format = "text/html")]
pub fn hello() -> &'static str {
    "Hello, world!"
}