
extern crate csv;
use std::{error::Error, fs::File};

#[derive(Debug)]
struct Company{
 name:String,
 email:String,
 webaddress:String,
}
fn main() -> Result<(),Box<dyn Error>>{
    let file=File::open("SD.csv")?;
    let mut readcsv=csv::Reader::from_reader(file);
    let mut counter=0;
    for data in readcsv.records(){
        counter+=1;
        let record=data.ok().unwrap();
        let userdata=Company{
            name:record.iter().nth(2).unwrap().to_string().replace('"',''),
            email:record.iter().nth(3).unwrap().to_string(),
            webaddress:record.iter().nth(12).unwrap().to_string(),
        };
        println!("{:?}",userdata);
        if counter==10{
            break;
        }
    }
    Ok(())
}
