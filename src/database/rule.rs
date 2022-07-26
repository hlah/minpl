use crate::term::Term;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Rule {
    pub head: Term,
    pub body: Vec<Term>,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.head)?;
        if !self.body.is_empty() {
            write!(f, " :- ")?;
            for i in 0..self.body.len() {
                write!(f, "{}", self.body[i])?;
                if i < self.body.len() - 1 {
                    write!(f, ", ")?;
                }
            }
        }
        Ok(())
    }
}
