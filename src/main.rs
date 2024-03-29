use std::boxed::Box;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread;

use regex::Regex;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct RegexResult {
    path: PathBuf,    // Path where self was found.
    sentence: String, // Sentence search result is in.
    line: usize,      // Line number where search result was found.
    start: usize,     // Start location of search result in sentence.
    end: usize,       // End location of search result in sentence.
}

fn main() {
    // Get file path and regex.
    let args: Vec<String> = env::args().collect();
    let file_path = PathBuf::from(args.get(1).expect("No path provided!"));
    let regex = args.get(2).expect("No regex provided!");

    // Build regex.
    let regex = Box::new(Regex::new(regex).expect("Failed to build provided regex!"));

    // Start search.
    let thread_handle =
        thread::spawn(move || -> Vec<RegexResult> { return path_handler(file_path, regex) });
    let mut search_results = thread_handle.join().expect("Thread expired unexpectedly!");

    // Parse results.
    search_results.sort();

    for hit in search_results {
        println!(
            "At {:?} line {} found {}-{}-{}",
            hit.path,
            hit.line,
            &hit.sentence[..hit.start],
            &hit.sentence[hit.start..hit.end],
            &hit.sentence[hit.end..]
        );
    }
}

/// Is called with the path to an unknown object, checks if the object is a
/// directory or a file. If file, parse contents, if directory recursively
/// search it.
fn path_handler(path: PathBuf, regex: Box<Regex>) -> Vec<RegexResult> {
    let mut search_results: Vec<RegexResult> = Vec::new();
    if path.is_dir() {
        println!("{:?} is a directory!", path);
        let entries = fs::read_dir(path).expect("Failed to read directory!");
        let mut thread_handles: Vec<thread::JoinHandle<Vec<RegexResult>>> = Vec::new();
        for entry in entries {
            let dir_path = entry.expect("Failed to read directory entry!").path();
            let regex_copy = regex.clone();
            thread_handles.push(thread::spawn(move || {
                return path_handler(dir_path, regex_copy);
            }));
        }
        for thread in thread_handles {
            let mut directory_hits = thread.join().expect("Thread expired unexpectedly!");
            search_results.append(&mut directory_hits);
        }
    } else {
        let file_contents = fs::read_to_string(&path).expect("File read error!");
        let results = regex.find_iter(&file_contents);
        for hit in results {
            let sentence_start = file_contents[..hit.start()].rfind("\n").unwrap_or(0);
            let sentence_end = file_contents[hit.end()..]
                .find("\n")
                .unwrap_or(file_contents.len() - hit.end())
                + hit.end();
            let packed_result = RegexResult {
                path: path.clone(),
                line: file_contents[..hit.start()].matches("\n").count() + 1,
                start: hit.start() - sentence_start,
                end: hit.end() - sentence_start,
                sentence: file_contents[sentence_start..sentence_end].to_string(),
            };
            search_results.push(packed_result);
        }
    }
    return search_results;
}
