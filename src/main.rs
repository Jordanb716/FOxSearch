use std::env;
use std::fs;

use regex::Regex;

fn main() {
    // Get file path and regex.
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1);
    let regex = args.get(2);

    let file_path = file_path.expect("No path provided!");
    let regex = regex.expect("No regex provided!");

    //Read file.
    let file_contents = fs::read_to_string(file_path).expect("File read error!");

    // Build regex.
    let regex = Regex::new(regex).expect("Failed to build provided regex!");
    let matches = regex.find_iter(&file_contents);

    for found in matches {
        println!(
            "At {} found {:20}, context: {}",
            found.start(),
            found.as_str(),
            &file_contents[found.start() - 10..found.end() + 10]
        );
    }
}
