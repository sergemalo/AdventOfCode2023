use std::fs;
use regex::Regex;
//use std::path;

/// .
fn main() {
    println!("AOC 2023-12-05");

    let file_path = "2023-12-05-input-a.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect(String::from(format!("Unable to read the file {}", file_path)).as_str());

    //println!("With text:\n{contents}");

    // Matching: seeds: 79 14 55 13
    let re_seeds_line = Regex::new(r"^seeds: (.+)$").expect("Can't create regex");

    let mut lines_it = contents.split('\n');

    while let Some(line) = lines_it.next() {
        println!("L: {}", line);
        if re_seeds_line.is_match(line) {
            let caps = re_seeds_line.captures(line).expect("Can't get captures from seeds line");
            let seeds = parse_seed(&caps[1]);
            for s in seeds {
                println!("s: {}", s);
            }
        }
        if line.contains("soil-to-fertilizer map:") {
            parse_soil_to_fert_map(lines_it.clone());
        }
    }
    

}

fn parse_seed(seeds_slice: &str) -> Vec<&str> {
    seeds_slice.split(' ').collect::<Vec<&str>>()
}

fn parse_soil_to_fert_map<'a, I>(mut lines_it: I) 
where 
    I: Iterator<Item = &'a str>,
{
    let mut line = lines_it.next().unwrap(); // else { return; };
    line = line.trim_end_matches(|c| c == '\n' || c == '\r');
    while  !line.is_empty() {
        println!("S TO F: {}", line);
        println!("S TO F: size: {}", line.len());
        line = lines_it.next().unwrap(); // else { return; };
        line = line.trim_end_matches(|c| c == '\n' || c == '\r');
    }
    // 0 15 37
    // 37 52 2
    // 39 0 15
}