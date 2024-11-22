use std::borrow::Cow;
use std::fmt;
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use rand::{self, Rng};
use rocket::UriDisplayPath;
use rocket::request::FromParam;

/// A unique document ID.
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone, UriDisplayPath)]
pub struct DocId<'a>(Cow<'a, str>);

impl DocId<'_> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> DocId<'static> {
        const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        DocId(Cow::Owned(id))
    }

    /// Returns the path to the document in `upload/` corresponding to this ID.
    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(self.0.as_ref())
    }

}

impl<'a> fmt::Display for DocId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Returns an instance of `DocId` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
/// Rocket uses FromParam to automatically validate and parse dynamic path parameters.
impl<'a> FromParam<'a> for DocId<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        //could be improved with checking that the length of the id is within some
        //known bound, introducing stricter character checks, checking for the
        //existing of a file
        param.chars().all(|c| c.is_ascii_alphanumeric())
            .then(|| DocId(param.into()))
            .ok_or(param)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SIZE: usize = 3;

    #[test]
    fn test_filepath() {
        let id = DocId::new(SIZE);
        let p = id.file_path();
        let r = p.iter().any(|name| name.to_str() == Some("upload"));
        assert!(r);
    }

    #[test]
    fn test_from_param() {
        let id = DocId::from_param("foo");
        assert!(id.is_ok());
    }
}