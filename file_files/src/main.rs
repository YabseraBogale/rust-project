use std::{fs::File, os::unix::process::CommandExt, process::Command};

fn main() {
    let res=File::create_new("helloworld.txt");
    println!("{0}",res.is_err());

}
