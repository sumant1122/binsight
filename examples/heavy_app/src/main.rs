use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
}

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Is date? {}", re.is_match("2023-10-01"));

    let user = User { name: "Alice".into(), email: "alice@example.com".into() };
    let json = serde_json::to_string(&user).unwrap();
    println!("User: {}", json);
}
