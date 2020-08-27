use std::env::Args;
use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // println!("With text:\n{}", contents);
    let results = if config.case_sensitive {
        // search(config.query, &contents)
        search(&config.query, &contents)
    } else {
        // search_case_insensitive(config.query, &contents)
        search_case_insensitive(&config.query, &contents)
    };

    for v in results {
        println!("{}", v)
    }
    Ok(())
}

// pub struct Config<'a> {
//     pub query: &'a String,
//     pub filename: &'a String,
//     pub case_sensitive: bool,
// }

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// impl<'a> Config<'a> {
//     pub fn new(args: &'a Vec<String>) -> Result<Config, &str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//
//         let query = &args[1];
//         let filename = &args[2];
//
//         let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
//
//         Ok(Config {
//             query,
//             filename,
//             case_sensitive,
//         })
//     }
// }

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(f) => f,
            None => return Err("Didn't get a filename string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut vec = vec![];
    // for line in content.lines() {
    //     if line.contains(query) {
    //         vec.push(line);
    //     }
    // }

    // content
    //     .lines()
    //     .into_iter()
    //     .filter(|line| line.contains(query))
    //     .map(|line| vec.push(line))
    //     .next();

    // vec

    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // let mut vec = Vec::new();
    // for line in content.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         vec.push(line);
    //     };
    // }
    // vec

    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    // use crate::{search, search_case_insensitive};
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
