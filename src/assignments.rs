use crate::term::Term;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignments {
    assignments: HashMap<String, Term>,
}

impl Display for Assignments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut length = self.assignments.len();
        for (key, value) in &self.assignments {
            length -= 1;
            write!(f, "{} := {}", key, value)?;
            if length > 0 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Assignments {
    pub fn new<I: IntoIterator<Item = (String, Term)>>(assignments: I) -> Self {
        Self {
            assignments: assignments.into_iter().collect(),
        }
    }

    pub fn empty() -> Self {
        Self {
            assignments: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    pub fn add<S: ToString>(&mut self, variable: S, value: Term) {
        self.assignments.insert(variable.to_string(), value);
    }

    pub fn with<S: ToString>(mut self, variable: S, value: Term) -> Self {
        self.assignments.insert(variable.to_string(), value);
        self
    }

    pub fn merge(mut self, other: Self) -> Option<Self> {
        for (key, value) in other.assignments.into_iter() {
            if let Some(old_value) = self.assignments.get(&key) {
                if *old_value != value {
                    return None;
                }
            } else {
                self.assignments.insert(key, value);
            }
        }
        Some(self)
    }

    pub fn substitutions(&self) -> impl Iterator<Item = (&String, &Term)> {
        self.assignments.iter()
    }

    pub fn normalized<'a, I: IntoIterator<Item = &'a str>>(self, scope: I) -> Self {
        let scope: HashSet<_> = scope.into_iter().collect();
        let mut normalised_assignments = HashMap::new();
        for name in scope {
            if let Some(value) = self.assignments.get(name) {
                normalised_assignments.insert(String::from(name), self.normalize_term(&value));
            }
        }
        Self {
            assignments: normalised_assignments,
        }
    }

    pub fn normalize_term(&self, term: &Term) -> Term {
        match term {
            Term::Variable { name } => {
                if let Some(value) = self.assignments.get(name) {
                    self.normalize_term(value).clone()
                } else {
                    term.clone()
                }
            }
            Term::Functor { name, arguments } => {
                let arguments = arguments.iter().map(|term| self.normalize_term(term));
                Term::functor(name, arguments)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_normalized_scoped_variables() {
        let assignments = Assignments::empty()
            .with("X", Term::variable("Y"))
            .with("Y", Term::atom("a"));

        let normalized = assignments.normalized(["X"]);

        assert_eq!(normalized, Assignments::empty().with("X", Term::atom("a")));
    }

    #[test]
    fn get_normalized_scoped_variables_inside_functor() {
        let assignments = Assignments::empty()
            .with(
                "X",
                Term::functor("test", [Term::atom("a"), Term::variable("Y")]),
            )
            .with("Y", Term::atom("b"));

        let normalized = assignments.normalized(["X"]);

        assert_eq!(
            normalized,
            Assignments::empty().with(
                "X",
                Term::functor("test", [Term::atom("a"), Term::atom("b")])
            )
        );
    }
}
