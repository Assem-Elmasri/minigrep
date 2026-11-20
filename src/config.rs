use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the first argument, which is the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a query, got nothing... as usual"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Expected a file path, got nothing... as usual"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
