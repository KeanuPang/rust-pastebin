use rand::{self, Rng};
use rocket::http::RawStr;
use rocket::request::FromParam;
use std::borrow::Cow;
use std::fmt;
use std::path::{Path, PathBuf};

/// A _probably_ unique paste ID.
#[derive(UriDisplayPath)]
pub struct PasteId<'a>(Cow<'a, str>);

// impl<'a> PasteId<'a> {
impl PasteId<'_> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize, by_base: &[u8]) -> PasteId<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();

        let count = by_base.len();
        for _ in 0..size {
            id.push(by_base[rng.gen::<usize>() % count] as char);
        }

        return PasteId(Cow::Owned(id));
    }

    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        return Path::new(root).join(self.0.as_ref());
    }
}

impl<'a> fmt::Display for PasteId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

/// Returns an instance of `PasteId` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        return param
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
            .then(|| PasteId(Cow::Borrowed(param)))
            .ok_or(param);
    }
}
