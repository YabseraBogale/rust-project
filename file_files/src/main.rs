use std::fs;
fn main() {
   let res=remove(String::from("hello world.txt"));
   println!("{0}",res.is_ok());
   let gg=|name:String|-> std::io::Result<()> {
    fs::remove_file(name)?;
    Ok(())
};
println!("{0}",gg(String::from("hello world.txt")).is_ok());
  
}
fn remove(name:String) -> std::io::Result<()> {
    fs::remove_file(name)?;
    Ok(())
}