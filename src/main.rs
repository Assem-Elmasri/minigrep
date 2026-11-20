use std::env;
use std::fs;
use std::process;
use std::error::Error; // to bring the Error trait into scope
use minigrep::{search, search_case_insensitive};


fn main() {
    let args: Vec<String> = env::args().collect();
        
    let cfg: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(cfg) {
        println!("Application error: {}", e);
        process::exit(1);
    }

}

fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file_path)?;

    let results = if cfg.ignore_case {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}



impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}