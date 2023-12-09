use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() ->  io::Result<()> {
    println!("Advent Of Code 2023 - 12 - 01 - By Serge Malo");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
        println!("{}", line?);
    }





    Ok(())

}
