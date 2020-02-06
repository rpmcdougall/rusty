use rocket_contrib::json::{JsonValue};

#[get("/test", format="json")]
pub fn get_test() -> JsonValue {
    json!({"test": "it's working!"})
}