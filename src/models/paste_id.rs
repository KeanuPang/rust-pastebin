use rocket::http::RawStr;
use rocket::request::FromParam;
use std::borrow::Cow;
use std::fmt;

use rand::{self, Rng};

/// A _probably_ unique paste ID.
pub struct PasteId<'a>(Cow<'a, str>);

impl<'a> PasteId<'a> {
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
}

impl<'a> fmt::Display for PasteId<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

/// Returns an instance of `PasteId` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for PasteId<'a> {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        match valid_id(param) {
            true => Ok(PasteId(Cow::Borrowed(param))),
            false => Err(param),
        }
    }
}

/// Returns `true` if `id` is a valid paste ID and `false` otherwise.
fn valid_id(id: &str) -> bool {
    id.chars().all(|c| {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9');
    })
}
