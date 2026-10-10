pub fn session(input: &str) -> usize {
    let a = input.trim().parse::<usize>().unwrap();
    let b = input.len().checked_add(a).unwrap();
    let c = b.checked_mul(2).unwrap();
    c
}
