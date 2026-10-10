//! The cursor of the next page (OUTPUT.md#pagination).

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
        assert_eq!(text, "eyJ2IjoxLCJoIjoiOWYxZWQyMjMiLCJmIjoyLCJ0IjpbNCw0LDhdfQ");
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
