extern crate  rust_file_word_counter ;
use rust_file_word_counter::filereader::reader ; 
use std::{env, process} ; 
fn main() {
    //Collect values from the environment 
    let arguments : Vec<String> = env::args().collect() ; 
    let config = Config::build(&arguments)
    .unwrap_or_else(|err|{
        println!("Problems with reading arguments {err}");
        process::exit(1)
    }) ;
    reader::file_reader(config.file_name);
} 

struct Config {
    file_name :String ,
}
impl  Config {
    fn build(args : &[String]) -> Result<Config , &'static str>{
        if args.len() < 1 {
            return Err("Not Enough Argument Error : Please provide file name");
        }
        let file_name = args[1].clone() ; 
        Ok(Config {file_name})
    }
}
