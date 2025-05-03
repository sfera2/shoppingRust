use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::Value;


pub fn number(num: u32) -> u32 {
    5
}


pub fn read_json(json_file: &str) -> Value {
    let mut file = File::open(json_file).unwrap();

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the JSON data into a struct
    let data: Value = serde_json::from_str(&contents).unwrap();

    // Print the data for debugging purposes
    return data;
}