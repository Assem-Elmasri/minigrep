use minigrep::config::Config;
use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error; // to bring the Error trait into scope
use std::fs;
use std::process;

fn main() {
    let cfg: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(cfg) {
        eprintln!("Application error: {}", e);
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
