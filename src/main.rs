extern crate  rust_file_word_counter ;
use rust_file_word_counter::filereader::reader ; 
use std::env ; 
fn main() {
    println!("Hello, world!");
    //Collect values from the environment 
    let arguments : Vec<String> = env::args().collect() ; 
    let query = parse_cli_argument(&arguments);
    reader::file_reader(&query);
} 

fn parse_cli_argument(args : &[String]) -> &str{
    let query = &args[1] ; 
    query
}
