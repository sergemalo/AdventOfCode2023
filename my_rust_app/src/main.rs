//use serde::Serialize;
//use  std::fs;
//use std::io;
//use std::env;
use std::fs;
use regex::Regex;

/* 

#[derive(Serialize)]
struct Record<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

fn csv_example() -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let rec1 = Record { name: "Mark", place: "Melbourne", id: 56};
    let rec2 = Record { name: "Ashley", place: "Sydney", id: 64};
    let rec3 = Record { name: "Akshat", place: "Delhi", id: 98};

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush()?;

    Ok(())

}
*/

fn msg_wrap(s: &str) {

    println!("Beau message: {}", s)

}

#[derive(Debug)]   // Indicate our class (Rectangle) implements the "Debug" trait
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1); // {:?} Indicate using the "debug" trait to format the output
    dbg!(&rect1);

    let msg = String::from("Mon message");

    msg_wrap(&msg);

    //panic!("crash and burn");

    msg_wrap(&msg);


    let file_path = String::from("2023-12-04-input-a.txt");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    //for c in contents.chars() {
    //    println!("-{}", c)
    //}
    
    let re = Regex::new(r"^Card (\d+)\: (.+) \| (.+)$").unwrap();
    let reWins = Regex::new(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)(.*)$").unwrap();
    let reNumbers = Regex::new(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)(.*)$").unwrap();
    let mut winning_nbs: [i32; 5];
    let mut my_numbers: [i32; 8];
    let mut total_points = 0;
    let mut card_points = 0;

    let split = contents.split('\n');
    for line in split {
        println!("zz ===> {}", line);
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            println!("C: {}", &caps[1]);

            winning_nbs = [0; 5];
            
            if reWins.is_match(&caps[2]) {
                let capsWins = reWins.captures(&caps[2]).unwrap();
                winning_nbs[0] = capsWins[1].parse::<i32>().unwrap();
                winning_nbs[1] = capsWins[2].parse::<i32>().unwrap();
                winning_nbs[2] = capsWins[3].parse::<i32>().unwrap();
                winning_nbs[3] = capsWins[4].parse::<i32>().unwrap();
                winning_nbs[4] = capsWins[5].parse::<i32>().unwrap();
            }

            println!("Winning numbers: {:?}", winning_nbs);


            my_numbers = [0; 8];
            
            if reNumbers.is_match(&caps[3]) {
                let capsNbs = reNumbers.captures(&caps[3]).unwrap();
                my_numbers[0] = capsNbs[1].parse::<i32>().unwrap();
                my_numbers[1] = capsNbs[2].parse::<i32>().unwrap();
                my_numbers[2] = capsNbs[3].parse::<i32>().unwrap();
                my_numbers[3] = capsNbs[4].parse::<i32>().unwrap();
                my_numbers[4] = capsNbs[5].parse::<i32>().unwrap();
                my_numbers[5] = capsNbs[6].parse::<i32>().unwrap();
                my_numbers[6] = capsNbs[7].parse::<i32>().unwrap();
                my_numbers[7] = capsNbs[8].parse::<i32>().unwrap();
            }

            println!("My numbers: {:?}", my_numbers);

            card_points = 0;
            for win_nb in winning_nbs {
                if my_numbers.contains(&win_nb) {
                    if card_points == 0 {
                        card_points = 1;
                    }
                    else {
                        card_points *= 2;
                    }

                }
            }

            println!("Card Points:{}", card_points);
            total_points += card_points;
        }
    }

    println!("Total Points: {}", total_points);

}