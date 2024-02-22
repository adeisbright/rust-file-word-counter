pub struct Config {
    pub file_name :String ,
}

impl  Config {
   pub  fn build(args : &[String]) -> Result<Config , &'static str>{
        if args.len() < 1 {
            return Err("Not Enough Argument Error : Please provide file name");
        }
        let file_name = args[1].clone() ; 
        Ok(Config {file_name})
    }
}