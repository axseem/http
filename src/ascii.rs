use thiserror::Error;

#[derive(Error, Debug)]
#[error("non ASCII character")]
pub struct NonAsciiCharacterError;

pub fn string_from_ascii(v: &[u8]) -> Result<String, NonAsciiCharacterError> {
    return match v.into_iter().any(|b| !b.is_ascii()) {
        true => Err(NonAsciiCharacterError),
        // probably safe since the bytes have been checked
        false => Ok(unsafe {std::str::from_utf8_unchecked(v).to_string()})
    }
}