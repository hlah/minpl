use crate::assignments::Assignments;
use crate::database::Database;
use crate::parser::MinplParser;
use crate::proof_search::prove;
use colored::*;
use rustyline::{error::ReadlineError, Editor};
use std::{env, fs::File, io::Read};

pub struct MinplRepl {
    rl: Editor<()>,
    database: Database,
    running: bool,
}

impl MinplRepl {
    pub fn new() -> Self {
        let rl = Editor::new().unwrap();
        Self {
            rl,
            database: Database::empty(),
            running: false,
        }
    }

    pub fn run(mut self) {
        match self.load_database() {
            Ok(_) => {
                self.running = true;
                self.repl_loop();
            }
            Err(error) => self.print_error(error),
        }
    }

    fn load_database(&mut self) -> anyhow::Result<()> {
        if let Some(filename) = env::args().skip(1).next() {
            println!("Loading database '{}'...", filename);
            let mut file = File::open(filename)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            self.database = MinplParser::parse_database(&content)?;
        }
        Ok(())
    }

    fn repl_loop(&mut self) {
        while self.running {
            if let Err(error) = self.repl_iteration() {
                self.print_error(error);
            }
        }
    }

    fn repl_iteration(&mut self) -> anyhow::Result<()> {
        match self.rl.readline("?: ") {
            Ok(line) => {
                self.rl.add_history_entry(line.as_str());
                let query = MinplParser::parse_query(&line)?;
                let results = prove(&self.database, query);
                self.print_result(results);
            }
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                println!("{}", "quitting.".bold());
                self.running = false;
            }
            Err(error) => return Err(error.into()),
        }
        Ok(())
    }

    fn print_result(&self, results: Vec<Assignments>) {
        if results.is_empty() {
            println!("{}", "false.".bold().red());
        } else {
            for result in results {
                if result.is_empty() {
                    println!("{}", "true.".bold().bright_green());
                } else {
                    println!("{}", result);
                }
            }
        }
    }

    fn print_error(&self, error: anyhow::Error) {
        println!("{}", format!("error: {}", error).bold().red());
    }
}
