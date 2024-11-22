use log::{info, warn};

use std::path::PathBuf;

use rocket_okapi::openapi;
use rocket::{get, post};
use rocket::http::ContentType;
use rocket::tokio::fs::File;
use rocket::form::Form;

use crate::doc_id::DocId;
use crate::upload::UploadFile;

// In a real application, these would be retrieved dynamically from a config.
const ID_LENGTH: usize = 6;

/// Sends a greeting to the user. The name is optional.
#[openapi(tag = "greeting", operation_id = "0")]
#[get("/greet?<name>")]
//if we don't use an option here the argument is required
pub fn greet(name: Option<String>) -> String {
    match name {
        Some(n) => format!("Hello {n}!"),
        _ => String::from("Hello World!")
    }
}

/// Retrieves a file by it's identifier, if it exists.
/// If no such file exists a 404 error is the result.
#[openapi(tag = "document", operation_id = "1")]
#[get("/doc/<id>")]
pub async fn retrieve(id: DocId<'_>) -> Option<(ContentType, File)> {
    let file_path = id.file_path();
    let mime_type = mine_type(&file_path);
    File::open(file_path).await.ok().map(|file| (mime_type, file))
}

fn mine_type(path: &PathBuf) -> ContentType {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("pdf") => ContentType::PDF,
        Some("txt") => ContentType::Plain,
        Some("html") => ContentType::HTML,
        _ => ContentType::Binary, // Fallback für unbekannte Typen
    }
}

/// Stores an uploaded file and returns the ID of that file to the client.
/// Maximum allowed file size is 3 MB.
#[openapi(tag = "document", operation_id = "2", ignore="upload")]
#[post("/doc", data = "<upload>")]
pub async fn upload(upload: Form<UploadFile>) -> std::io::Result<String> {
    let id = DocId::new(ID_LENGTH);
    let name = upload.orig_file_name();
    match name {
       Some(n) => info!("filename: {}", n),
       None => warn!("no filename")
    };

    upload.save_as(id.file_path()).await?;

    Ok(id.to_string())
}
