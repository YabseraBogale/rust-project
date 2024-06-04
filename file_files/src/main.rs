use std::{fs,fs::File};

fn main() {
    let res=File::create_new("helloworld.txt");
    println!("{0}",res.is_err());
    let res=fs::remove_file("./hello world");
    println!("{0}",res.is_err());
}
