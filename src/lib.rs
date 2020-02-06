#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

mod router;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/api",
            routes![
               router::hello::get_test,
            ],
        )
        .register(catchers![not_found])
}