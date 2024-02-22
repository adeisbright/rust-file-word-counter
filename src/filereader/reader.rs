
use std::fs::File ;
use std::io::{BufReader , BufRead , Result}; 

/// Count the frequency of a word within a sentence 
///
/// # Example 
/// word_counter("Hello, Word! How are you doing?" , "you") 
/// Returns 1 because you occurs only once in the sentence 

pub fn word_counter(sentence : String , word : &String) -> i32{
    let mut word_counter = 0 ; 
    let split_words : Vec<&str> = sentence.split_whitespace().collect() ;
    for text in split_words {
        if text == word {
            word_counter  = &word_counter + 1 ; 
        }
    }
    word_counter
}

pub fn file_reader(file_name : String) -> Result<String> {
    let file =  File::open(&file_name)? ; 
    let reader = BufReader::new(file) ; 
    let mut contents = String::new() ; 

    for line_result in reader.lines(){
        let line = line_result?;
        contents.push_str(&line);
        contents.push('\n'); 
    }
    Ok(contents)
}