extern crate  rust_file_word_counter ;
use rust_file_word_counter::filereader::reader ; 
use std::env ; 
fn main() {
    println!("Hello, world!");
    //Collect values from the environment 
    let arguments : Vec<String> = env::args().collect() ; 
    let config = Config::new(&arguments);
    reader::file_reader(config.file_name);
} 

struct Config {
    file_name :String ,
}
impl  Config {
    fn new(args : &[String]) -> Config{
        let file_name = args[1].clone() ; 
        Config {file_name}
    }
}
