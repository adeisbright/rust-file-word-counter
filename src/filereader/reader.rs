use std::fs ; 
//use std::io::{BufReader , BufRead};
// use std::path::Path ; 

pub fn file_reader(file_name : String){
    let contents = fs::read_to_string(file_name)
    .expect("Should have been able to read file");
    println!("\n{contents}");
}