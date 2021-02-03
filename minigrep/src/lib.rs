use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
   pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments"); 
        }
        
        let query = args[1].clone();
        let filename = args[2].clone(); 

        Ok(Config { query, filename })   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_few_args(){
        let args = vec!(
                    String::from("debug\\target\\minigrep.exe"),
                    String::from("needle"));

        if let Ok(_c) = Config::new(&args) {
            panic!("Config::new should have returned an error.");
        }
    }

    #[test]
    fn config_many_args(){
        let args = vec!(
                    String::from("debug\\target\\minigrep.exe"),
                    String::from("needle"),
                    String::from("poem.txt"),
                    String::from("dummy")
                    );

        if let Err(_e) = Config::new(&args) {
            panic!("Config::new should NOT have returned an error.");
        }
    }

    #[test]
    fn run_file_exists(){
        let config = Config { query: String::from("needle"), filename: String::from("poem.txt")};
        
        if let Err(_e) = run(config) {
            panic!("run should NOT have returned an error!");
        }
    }

    #[test]
    fn run_file_not_exists(){
        let config = Config { query: String::from("needle"), filename: String::from("poem.txt")};
        
        if let Err(_e) = run(config) {
            panic!("run should NOT have returned an error!");
        }
    }
}
