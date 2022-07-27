pub mod assignments;
pub mod database;
pub mod parser;
pub mod proof_search;
pub mod term;
pub mod unification;

use database::Database;
use parser::MinplParser;
use proof_search::prove;
use rustyline::Editor;
use std::{env, fs::File, io::Read};

pub fn main() {
    let database = load_database();

    let mut rl = Editor::<()>::new().unwrap();
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let query = MinplParser::parse_query(&line);
                let results = prove(&database, query);
                if results.is_empty() {
                    println!("false.")
                } else {
                    for result in results {
                        if result.is_empty() {
                            println!("true.");
                        } else {
                            println!("{}", result);
                        }
                    }
                }
            }
            _ => break,
        }
    }
}

fn load_database() -> Database {
    if let Some(filename) = env::args().skip(1).next() {
        match File::open(filename) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                MinplParser::parse_database(&content)
            }
            Err(error) => panic!("Failed to load file: {}", error),
        }
    } else {
        Database::empty()
    }
}
