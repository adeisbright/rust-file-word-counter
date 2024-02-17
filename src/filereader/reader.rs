use std::fs::File ; 
use std::path::Path ; 

pub fn file_reader(file_name : &String) -> &String{
    let file_path = Path::new(file_name);
    let display = file_path.display();
    let file = match File::open(file_path){
        Ok(filer) => filer ,
        Err(why) => panic!("No file as {} found {}" , file_name , why),
    };
    file_name
}