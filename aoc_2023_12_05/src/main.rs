use std::fs;
use regex::Regex;

/// .
fn main() {
    println!("AOC 2023-12-05");

    let file_path = String::from("2023-12-05-input-a.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //println!("With text:\n{contents}");

    let re_seeds_line = Regex::new(r"^seeds: (.+)$").unwrap();

    let split = contents.split('\n');
    for line in split {
        println!("{line}");
        if re_seeds_line.is_match(line) {
            let caps = re_seeds_line.captures(line).unwrap();
            println!("Seeds: {}", &caps[1]);
        }
    }
}
