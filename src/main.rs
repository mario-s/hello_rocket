#[macro_use] extern crate rocket;

use hello_rocket::api::*;
use rocket_okapi::{openapi_get_routes, rapidoc::*, swagger_ui::*};
use rocket_okapi::settings::UrlObject;


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
