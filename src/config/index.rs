pub struct Config {
    pub file_name :String ,
    pub word : String ,
}

impl  Config {
   pub  fn build(args : &[String]) -> Result<Config , &'static str>{
        if args.len() < 1 {
            return Err("Not Enough Argument Error : Please provide file name");
        }
        let file_name = args[1].clone() ; 
        let word = args[2].clone() ; 
        Ok(Config {file_name , word})
    }
}