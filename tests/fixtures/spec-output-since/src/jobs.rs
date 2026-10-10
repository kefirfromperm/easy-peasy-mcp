pub fn jobs(input: &str) -> usize {
    let a = input.trim().parse::<usize>().unwrap();
    a.checked_add(1).unwrap()
}
