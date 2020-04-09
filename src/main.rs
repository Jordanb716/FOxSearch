use std::env;
use std::fs;

use regex::Regex;

fn main() {
    // Get file path and regex.
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No path provided!");
    let regex = args.get(2).expect("No regex provided!");

    //Read file.
    let file_contents = fs::read_to_string(file_path).expect("File read error!");

    // Build regex.
    let regex = Regex::new(regex).expect("Failed to build provided regex!");
    let search_results = regex.find_iter(&file_contents);

    for hit in search_results {
        println!(
            "At {} hit {:20}, context: {}",
            hit.start(),
            hit.as_str(),
            &file_contents[hit.start() - 10..hit.end() + 10]
        );
    }
}
