extern crate csv;
use rusqlite::{Connection,Result};
use std::{error::Error,fs::File};
#[derive(Debug)]
struct Userdata{
    name: String,
    email: String,
    webaddress: String
}
fn main() -> Result<(),Box<dyn Error>>{
    let connection=Connection::open("gold.db").unwrap();
    let file=File::open("SD.csv")?;
    let mut rawdata=csv::Reader::from_reader(file);
    let mut counter=0;
    for data in rawdata.records(){
        counter+=1;
        let record=data.ok().unwrap();
        let userdata=Userdata{
                name:record.iter().nth(2).unwrap().to_string(),
                email:record.iter().nth(3).unwrap().to_string(),
                webaddress:record.iter().nth(12).unwrap().to_string(),
        };
        let result=connection.execute("Insert into userdata(name,email,webaddress) values(?1,?2,?3)",(userdata.name,userdata.email,userdata.webaddress));
        match result {
            Ok(result)=> println!("result {}",result),
            Err(error)=> println!("result {}",error),
        }
        if counter==5{
            break;
        }

    }
    Ok(())
}
