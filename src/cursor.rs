//! The cursor of the next page (OUTPUT.md#pagination).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The version of the cursor format.
const VERSION: u32 = 1;

/// The alphabet of base64url.
const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Where the next page of a request starts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    /// The hash of the request, see `request_hash`.
    pub hash: String,
    /// The index of the first file of the page in the sorted result.
    pub file: usize,
    /// The index of the first fragment of that file which is listed.
    pub fragment: usize,
    /// The counts of the result: files, lines, matches.
    pub totals: [usize; 3],
}

/// The cursor as it is encoded: the fields in this order.
#[derive(Serialize, Deserialize)]
struct Encoded {
    v: u32,
    h: String,
    f: usize,
    #[serde(default, skip_serializing_if = "is_zero")]
    m: usize,
    t: [usize; 3],
}

#[allow(clippy::trivially_copy_pass_by_ref)] // the signature of `skip_serializing_if`
fn is_zero(n: &usize) -> bool {
    *n == 0
}

impl Cursor {
    /// The opaque text: base64url without padding of the compact JSON.
    #[must_use]
    pub fn encode(&self) -> String {
        let encoded = Encoded {
            v: VERSION,
            h: self.hash.clone(),
            f: self.file,
            m: self.fragment,
            t: self.totals,
        };
        base64url(
            serde_json::to_string(&encoded)
                .expect("a cursor serializes")
                .as_bytes(),
        )
    }

    /// The cursor of a text made by `encode`; `None` when it can't be decoded.
    #[must_use]
    pub fn decode(text: &str) -> Option<Self> {
        let bytes = from_base64url(text)?;
        let encoded: Encoded = serde_json::from_slice(&bytes).ok()?;
        (encoded.v == VERSION).then_some(Self {
            hash: encoded.h,
            file: encoded.f,
            fragment: encoded.m,
            totals: encoded.t,
        })
    }
}

/// The hash of the arguments of a request without `output.cursor`: FNV-1a 32 of their compact JSON (keys sorted), 8
/// hex digits.
#[must_use]
pub fn request_hash(arguments: &Value) -> String {
    let mut request = arguments.clone();
    if let Some(output) = request.get_mut("output").and_then(Value::as_object_mut) {
        output.remove("cursor");
    }
    let text = serde_json::to_string(&request).expect("a JSON value serializes");
    let mut hash: u32 = 0x811c_9dc5;
    for byte in text.bytes() {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(0x0100_0193);
    }
    format!("{hash:08x}")
}

/// Base64url without padding.
fn base64url(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0_u32, |n, (i, b)| n | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..=chunk.len() {
            out.push(char::from(ALPHABET[((n >> (18 - 6 * i)) & 63) as usize]));
        }
    }
    out
}

/// The bytes of base64url without padding; `None` for another text.
fn from_base64url(text: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(text.len() * 3 / 4);
    for chunk in text.as_bytes().chunks(4) {
        if chunk.len() == 1 {
            return None;
        }
        let mut n = 0_u32;
        for (i, c) in chunk.iter().enumerate() {
            let value = ALPHABET.iter().position(|a| a == c)?;
            n |= u32::try_from(value).ok()? << (18 - 6 * i);
        }
        for i in 0..chunk.len() - 1 {
            out.push(u8::try_from((n >> (16 - 8 * i)) & 255).ok()?);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_file_is_not_a_part_of_the_request() {
        let plain = json!({ "query": "x", "output": { "max_files": 2 } });
        let filed = json!({ "query": "x", "output": { "max_files": 2, "file": "a.txt", "cursor": "c" } });
        assert_eq!(request_hash(&plain), request_hash(&filed));
    }

    #[test]
    fn encoded_and_decoded() {
        let request = json!({
            "query": "text has 'TODO'",
            "output": { "detail": "files", "sort": "path asc", "max_files": 2 }
        });
        let hash = request_hash(&request);
        assert_eq!(hash, "9f1ed223");
        let cursor = Cursor {
            hash: hash.clone(),
            file: 2,
            fragment: 0,
            totals: [4, 4, 8],
        };
        let text = cursor.encode();
        assert_eq!(
            text,
            "eyJ2IjoxLCJoIjoiOWYxZWQyMjMiLCJmIjoyLCJ0IjpbNCw0LDhdfQ"
        );
        assert_eq!(Cursor::decode(&text), Some(cursor.clone()));
        let inside = Cursor {
            fragment: 1,
            ..cursor
        };
        assert_eq!(Cursor::decode(&inside.encode()), Some(inside));
        let mut with_cursor = request.clone();
        with_cursor["output"]["cursor"] = json!(text);
        assert_eq!(request_hash(&with_cursor), hash, "the cursor is not hashed");
        for bad in ["", "not a cursor", "eyJ2IjoyfQ", "e30"] {
            assert_eq!(Cursor::decode(bad), None, "{bad}");
        }
    }
}
