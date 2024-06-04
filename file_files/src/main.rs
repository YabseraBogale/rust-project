use std::{any::Any, fs::{self, File}, io::{Read, Write}};

fn main() {
    let res=File::create_new("helloworld.txt");
    println!("{0}",res.is_err());
    let res=fs::remove_file("./hello world");
    println!("{0}",res.is_err());
    
    
}
