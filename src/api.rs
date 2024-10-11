use rocket_okapi::openapi;
use rocket::get;

/// Sends a greeting to the user. The name is optional.
#[openapi(tag = "greeting", operation_id = "1")]
#[get("/greet?<name>")]
//if we don't use an option here the argument is required
pub fn greet(name: Option<String>) -> String {
    match name {
        Some(n) => format!("Hello {n}!"),
        _ => String::from("Hello World!")
    }
}