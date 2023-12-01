use serde::Serialize;
use std::io;
use  std::fs;


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

fn main() -> Result<(), csv::Error> {
    //let _err =  csv_example();


    let mut file = fs::File::create("favorite_websites.txt")?;
    file.write_all(b"opensource.com\n")?;


    let file = fs::File::open("favorite_websites.txt")?;
    let lines = io::BufReader::new(file).lines();
    
    for line in lines {
        if let Ok(_line) = line {
            println!(">>> {}", _line);
        }
    }

    Ok(())
}