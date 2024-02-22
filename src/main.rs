extern crate  rust_file_word_counter ;
use rust_file_word_counter::filereader::reader ; 
use rust_file_word_counter::arg_reader;
use std::{env, process} ; 
fn main() {
    let arguments : Vec<String> = env::args().collect() ; 
    let config = arg_reader::index::Config::build(&arguments)
    .unwrap_or_else(|err|{
        eprintln!("Problems with reading arguments {}" , err);
        process::exit(1)
    }) ;
   
    match reader::file_reader(String::from(config.file_name)){
        Ok(file_content) => {
            let my_count = reader::word_counter(file_content  , &config.word );
            println!("The word  {} appears {} times in the file" , &config.word , my_count );
        }
        Err(err) => eprintln!("{}" , err),
    };
} 

