use core::panic;
use std::fs ; 
use std::fs::File ;
use std::io::{BufReader , BufRead}; 

pub fn file_reader(file_name : String){
    let contents = fs::read_to_string(file_name)
    .expect("Should have been able to read file");
    println!("\n{contents}");
    
}

pub fn word_counter() -> i32{
    let sentence = String::from("Every Good Boy Deserves Favour");
    let word = String::from("Boy") ; 
    let mut word_counter = 0 ; 
    let split_words : Vec<&str> = sentence.split_whitespace().collect() ;
    for text in split_words {
        if text == word {
            word_counter  = &word_counter + 1 ; 
        }
    }
    word_counter
}

pub fn file_reader_copy(file_name : String) {
    let file = match File::open(file_name){
       Ok(file) => file , 
       Err(_) => panic!("Wahala")
    };

    let reader = BufReader::new(file) ; 
    for line_result in reader.lines(){
        let line = match line_result { 
            Ok(line) => line , 
            Err(_) => panic!("Not able to read line"),
        };
        println!("Based on {}" , line)
    }
    
}