extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct Entry {
    html_url: String,
    name: String,
}

fn main() {
    let mut json_data = String::new();

    // Check if the user provided a filename as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let filename = &args[1];
        // Read the content from the provided file
        match fs::read_to_string(filename) {
            Ok(content) => json_data = content,
            Err(err) => {
                eprintln!("Failed to read {}: {}", filename, err);
                return;
            }
        };
    } else {
        // If no filename was provided, read from standard input
        std::io::stdin().read_to_string(&mut json_data).expect("Failed to read from stdin");
    }

    // Deserialize the JSON string
    let mut entries: Vec<Entry> = match serde_json::from_str(&json_data) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return;
        }
    };

    // Sort the entries by name
    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // Print the html_url values
    for entry in entries {
        println!("[[repo]]\nurl = \"{}\"\n", entry.html_url);
    }
}
