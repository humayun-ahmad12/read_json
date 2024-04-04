use serde::Deserialize;
use serde_json;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u8,
}



fn main() {

    let file_path = "/home/rajib/Documents/newgithub/read_json/file.json";

    match read_json_file(file_path) {
        Ok(person) => println!("{:?}", person),
        Err(e) => println!("Error reading JSON: {}", e),
    }

    // println!("{:?}", person1);
}


fn read_json_file(file_path: &str) -> Result<Person, io::Error> {
    let file_content = fs::read_to_string(file_path)?;

    let person: Person = serde_json::from_str(&file_content)?;
    Ok(person)
}