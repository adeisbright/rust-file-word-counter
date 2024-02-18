extern crate  rust_file_word_counter ;
use rust_file_word_counter::filereader::reader ; 

fn main() {
    println!("Hello, world!");
    //let the_file = String::from("hello.txt");
    //let my_file = reader::file_reader(&the_file);
    //print!("This is my file name {}" , my_file);
    reader::file_reader("Cargo.toml");
}
