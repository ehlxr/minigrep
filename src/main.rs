use minigrep::Config;
use std::{env, process};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let args: [String; 2] = [String::from("Blue"), String::from("Yellow")];

    // let config = match Config::new(&args) {
    //     Ok(c) => c,
    //     Err(e) => {
    //         println!("Problem parsing arguments: {}", e);
    //         process::exit(1);
    //     }
    // };
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // let contents = fs::read_to_string(config.filename)
    //     .expect("Something went wrong reading the file");
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
