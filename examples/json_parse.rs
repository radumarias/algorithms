use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Status {
    Active,
    Inactive,
    Pending,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Person {
    name: String,
    age: u8,
    email: Option<String>,
    status: Status,
}

fn main() {
    let json_data = r#"
[
  { "name": "Alice", "age": 30, "email": "alice@example.com", "status": "Active" },
  { "name": "Bob", "age": 25, "email": "bob@example.com", "status": "Inactive" },
  { "name": "Charlie", "age": 28, "email": null, "status": "Pending" }
]
"#;

    let person: Vec<Person> = serde_json::from_str(json_data).unwrap();
    println!("{person:?}");
}

#[cfg(test)]
mod tests {
    use crate::{Person, Status};

    #[test]
    fn test1() {
        let json_data = r#"
[
  { "name": "Alice", "age": 30, "email": "alice@example.com", "status": "Active" }
]
"#;
        let person: Vec<Person> = serde_json::from_str(json_data).unwrap();
        assert_eq!(person, vec![Person {
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
            status: Status::Active,
        }])
    }
}