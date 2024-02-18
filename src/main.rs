extern crate  rust_file_word_counter ;
use rust_file_word_counter::filereader::reader ; 
use std::env ; 
fn main() {
    println!("Hello, world!");
    //Collect values from the environment 
    let arguments : Vec<String> = env::args().collect() ; 
    //Reference the file name 
    let file_name = &arguments[1] ; 

    //let the_file = String::from("hello.txt");
    //let my_file = reader::file_reader(&the_file);
    //print!("This is my file name {}" , my_file);
    reader::file_reader(&file_name);
}
