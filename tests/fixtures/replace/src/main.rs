use serde_json::Value;

fn main() {
    let apple = serde_json::json!({"fruit": "apple"});
    println!("{}", serde_json::to_string(&apple).unwrap());
}
