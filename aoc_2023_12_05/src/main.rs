use std::fs;
use regex::Regex;

/// .
fn main() {
    println!("AOC 2023-12-05");

    let file_path = "2023-12-05-input-a.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect(String::from(format!("Unable to read the file {}", file_path)).as_str());

    //println!("With text:\n{contents}");

    let re_seeds_line = Regex::new(r"^seeds: (.+)$").expect("Can't create regex");

    let lines = contents.split('\n');
    for line in lines {
        println!("{line}");
        if re_seeds_line.is_match(line) {
            let caps = re_seeds_line.captures(line).expect("Can't get captures from seeds line");
            let seeds = parse_seed(&caps[1]);
            for s in seeds {
                println!("s: {}", s);
            }
        }
    }
}

fn parse_seed(seeds_slice: &str) -> Vec<&str> {
    seeds_slice.split(' ').collect::<Vec<&str>>()
}