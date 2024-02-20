use std::fs ; 
//use std::io::{BufReader , BufRead};
// use std::path::Path ; 
//Problem 1.1 => I want to be able to return the content that was read 
// Then I want to use another function to count how many times a word appears

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