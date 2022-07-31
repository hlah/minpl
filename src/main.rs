pub mod assignments;
pub mod database;
pub mod parser;
pub mod proof_search;
pub mod repl;
pub mod term;
pub mod unification;

use repl::MinplRepl;

fn main() {
    MinplRepl::new().run();
}
