use std::io::Result;
use std::path::PathBuf;

use rocket::tokio::io::AsyncWriteExt;
use rocket::FromForm;
use rocket::data::ToByteUnit;
use rocket::form::{DataField, FromFormField};
use rocket::fs::FileName;
use rocket::http::ContentType;
use rocket::tokio::fs::File as AsyncFile;
use schemars::JsonSchema;

#[derive(JsonSchema, Clone)]
pub struct FileNameWrapper(String);

impl FileNameWrapper {
    fn as_str(&self) -> &str {
        &self.0 // Access the inner String
    }
}

impl From<&FileName> for FileNameWrapper {
    fn from(file_name: &FileName) -> Self {
        FileNameWrapper(file_name.as_str().unwrap().to_string())
    }
}

#[derive(JsonSchema)]
pub struct ContentTypeWrapper(String);

impl From<&ContentType> for ContentTypeWrapper {
    fn from(content_type: &ContentType) -> Self {
        ContentTypeWrapper(content_type.to_string()) // Use `to_string()` to convert ContentType to a String
    }
}

#[derive(JsonSchema)]
pub struct File {
    file_name: Option<FileNameWrapper>,
    content_type: ContentTypeWrapper,
    data: Vec<u8>,
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for File {
    async fn from_data(field: DataField<'v, '_>) -> rocket::form::Result<'v, Self> {
        let stream = field.data.open(u32::MAX.bytes());
        let bytes = stream.into_bytes().await?;
        Ok(File {
            file_name: field.file_name.map(FileNameWrapper::from),
            content_type: ContentTypeWrapper::from(&field.content_type),
            data: bytes.value,
        })
    }
}

#[derive(FromForm, JsonSchema)]
pub struct UploadFile {
    pub file: File,
}

impl UploadFile {
    pub async fn save_as(&self, path: PathBuf) -> Result<()>{
        let mut af = AsyncFile::create(path).await?;
        let _ = af.write_all(&self.file.data);
        Ok(())
    }

    pub fn orig_file_name(&self) -> Option<String> {
        let name = &self.file.file_name;
        name.as_ref().map(|wrapper| String::from(wrapper.as_str()))
    }
}