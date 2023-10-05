extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
struct Entry {
    html_url: String,
}

fn main() {
    // Get the filename from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_json_file>", args[0]);
        return;
    }
    let filename = &args[1];

    // Read the file content
    let json_data = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read {}: {}", filename, err);
            return;
        }
    };

    // Deserialize the JSON string
    let entries: Vec<Entry> = match serde_json::from_str(&json_data) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return;
        }
    };

    // Print the html_url values
    for entry in entries {
        println!("[[repo]]\nurl = \"{}\"\n", entry.html_url);
    }
}
