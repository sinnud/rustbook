/*
pub fn main11(){
    let args: Vec<String> = std::env::args().collect();
    println!("in main11, {:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("           Searching for '{}'", query);
    println!("           In file {}", filename);

    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("           With text:\n{}", contents);
}

pub fn main31(){
    let args: Vec<String> = std::env::args().collect();
    let (query, filename) = parse_config1(&args);
    println!("in main31, Searching for '{}'", query);
    println!("           In file {}", filename);
}

fn parse_config1(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
pub fn main32(){
    let args: Vec<String> = std::env::args().collect();
    let config = parse_config(&args);

    println!("in main32, Searching for {}", config.query);
    println!("           In file {}", config.filename);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config{query, filename}
}

pub fn main33(){
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args);

    println!("in main33, Searching for {}", config.query);
    println!("           In file {}", config.filename);
}

impl Config{
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config{query, filename}
    }
}

pub fn main34(){
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("in main34, Problem parsing arguments: {}", err);
        println!("           Usage:\n    rustbook query filename");
        std::process::exit(1);
    });
    println!("in main34, Searching for '{}'", config.query);
    println!("           In file {}", config.filename);

    run1(config);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run1(config: Config) {
    let contents = std::fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("           With text:\n{}", contents);
}
pub fn main35(){
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("in main35, Problem parsing arguments: {}", err);
        println!("           Usage:\n    rustbook query filename");
        std::process::exit(1);
    });
    println!("in main35, Searching for '{}'", config.query);
    println!("           In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        std::process::exit(1);
    }
    // run(config);
}
use std::error::Error;
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filename)?;

    println!("           With text:\n{}", contents);

    Ok(())
}

use rustbook::Config;
pub fn main36(){
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("in main36, Problem parsing arguments: {}", err);
        println!("           Usage:\n    rustbook query filename");
        std::process::exit(1);
    });
    println!("in main36, Searching for '{}'", config.query);
    println!("           In file {}", config.filename);

    if let Err(e) = rustbook::run(config) {
        println!("Application error: {}", e);

        std::process::exit(1);
    }
}
*/
use rustbook::Config;
pub fn main61(){
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("(E)in main61, Problem parsing arguments: {}", err);
        eprintln!("(E)           Usage:\n    rustbook query filename");
        std::process::exit(1);
    });
    println!("in main61, Searching for '{}'", config.query);
    println!("           In file {}", config.filename);

    if let Err(e) = rustbook::run(config) {
        eprintln!("(E)Application error: {}", e);

        std::process::exit(1);
    }
}