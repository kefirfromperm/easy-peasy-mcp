//! Several queries in one call (OUTPUT.md#batch).

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_arguments_of_an_item() {
        let shared = json!({
            "dirs": ["."],
            "output": { "detail": "files", "context": 1 },
            "traversal": { "hidden": true }
        });
        let item = json!({
            "label": "x",
            "query": "lang = rust",
            "dirs": ["src"],
            "output": { "detail": "count" }
        });
        assert_eq!(
            item_arguments(&shared, &item).unwrap(),
            json!({
                "query": "lang = rust",
                "dirs": ["src"],
                "output": { "detail": "count", "context": 1 },
                "traversal": { "hidden": true }
            })
        );
        assert_eq!(
            item_arguments(&shared, &json!({ "label": 1, "query": "x" }))
                .unwrap_err()
                .to_text(),
            "error[invalid_option]: 'label' must be a string\n"
        );
        assert_eq!(
            item_arguments(&shared, &json!({ "label": "x" })).unwrap_err().to_text(),
            "error[invalid_option]: 'query' must be a string\n"
        );
    }

    #[test]
    fn the_budget_is_split() {
        // 300 tokens for three queries, the second has its own 50
        let mut split = Split::new(300, &[None, Some(50), None]);
        assert_eq!(split.next(0), 125);
        split.used(100);
        assert_eq!(split.next(1), 50);
        split.used(50);
        // the first left 25 of its 125 for the last one
        assert_eq!(split.next(2), 150);
    }
}
