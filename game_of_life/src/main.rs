use rand::Rng;

extern crate rand;
fn main() {
    let num=rand::thread_rng().gen_range(0..2);
    println!("number is {0}",num);
}
