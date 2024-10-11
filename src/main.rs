#[macro_use] extern crate rocket;

use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use rocket_okapi::settings::UrlObject;

/// Sends a greeting to the user. The name is optional.
#[openapi(tag = "greeting", operation_id = "1")]
#[get("/greet?<name>")]
//if we don't use an option here the argument is required
fn greet(name: Option<String>) -> String {
    match name {
        Some(n) => format!("Hello {n}!"),
        _ => String::from("Hello World!")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![greet])
        .mount("/swagger/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                display_operation_id: true,
                display_request_duration: true,
                ..Default::default()
            }),
        )
        .mount("/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("Hello Rocket! documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}
