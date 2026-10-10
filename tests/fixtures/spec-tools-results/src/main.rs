use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--legacy" {
        legacy_main();
        return;
    }
    println!("hello");
}

// TODO remove
fn legacy_main() {
    println!("hello from the old entry point");
}
