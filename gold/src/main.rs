extern crate sqlite;

fn main() {
    sqlite::open("gold.db").unwrap();
}
