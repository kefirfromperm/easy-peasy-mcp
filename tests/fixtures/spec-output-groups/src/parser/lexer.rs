pub fn tokens(text: &str) -> Vec<char> {
    let first = text.chars().next().unwrap();
    let last = text.chars().last().unwrap();
    vec![first, last]
}
