extern crate csv;
use std::{error::Error, fs::File};
fn main() -> Result<(),Box<dyn Error>>{
    let file=File::open("SD.csv")?;
    let mut readcsv=csv::Reader::from_reader(file);
    let mut counter=0;
    for data in readcsv.records(){
        counter+=1;
        let record=data.ok().unwrap();
        println!("{:?}",record.iter().nth(5).unwrap().to_ascii_uppercase());
        if counter==10{
            break;
        }
    }
    Ok(())
}
