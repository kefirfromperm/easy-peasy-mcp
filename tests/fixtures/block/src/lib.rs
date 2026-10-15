use std::fs;

pub fn load(path: &str) -> String {
    let raw = read(path);
    if raw.is_empty() {
        return String::new();
    }
    raw
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn parse(
    text: &str,
) -> Vec<String> {
    text.lines()
        .map(str::to_string)
        .collect()
}
