mod rule;

use crate::term::Term;
pub use rule::*;

#[derive(Debug, Clone)]
pub struct Database {
    rules: Vec<Rule>,
}

impl Database {
    pub fn empty() -> Self {
        Self { rules: vec![] }
    }

    pub fn with_fact(mut self, fact: Term) -> Self {
        self.rules.push(Rule {
            head: fact,
            body: vec![],
        });
        self
    }

    pub fn with_rule<I: IntoIterator<Item = Term>>(mut self, head: Term, body: I) -> Self {
        self.rules.push(Rule {
            head,
            body: body.into_iter().collect(),
        });
        self
    }

    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
}
