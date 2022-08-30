/// Default PasteID length.
pub const ID_LENGTH: usize = 5;
/// Table to retrieve base64 values from.
pub const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub const UPLOAD_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
