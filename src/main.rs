extern crate  rust_file_word_counter ;
use rust_file_word_counter::filereader::reader ; 
use std::{env, process} ; 
fn main() {
    let arguments : Vec<String> = env::args().collect() ; 
    let config = Config::build(&arguments)
    .unwrap_or_else(|err|{
        println!("Problems with reading arguments {err}");
        process::exit(1)
    }) ;
   
    match reader::file_reader(String::from(config.file_name)){
        Ok(contents) => {
            let word =  String::from("edition");
            let my_count = reader::word_counter(contents , &word );
            println!("The word  {} appears {} times in the file" , &word , my_count );
        }
        Err(err) => eprintln!("{}" , err),
    };
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
