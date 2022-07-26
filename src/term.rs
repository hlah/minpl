use crate::assignments::Assignments;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Functor { name: String, arguments: Vec<Term> },
    Variable { name: String },
}

impl Term {
    pub fn variable<S: ToString>(name: S) -> Self {
        Self::Variable {
            name: name.to_string(),
        }
    }

    pub fn atom<S: ToString>(name: S) -> Self {
        Self::Functor {
            name: name.to_string(),
            arguments: vec![],
        }
    }

    pub fn functor<S: ToString, I: IntoIterator<Item = Term>>(name: S, arguments: I) -> Self {
        let arguments = arguments.into_iter().collect();
        Self::Functor {
            name: name.to_string(),
            arguments,
        }
    }

    pub fn substitute(self, var_name: &str, term: &Term) -> Self {
        match self {
            Self::Variable { name } if name == var_name => term.clone(),
            Self::Functor { arguments, name } => Self::Functor {
                name,
                arguments: arguments
                    .into_iter()
                    .map(|arg| arg.substitute(var_name, term))
                    .collect(),
            },
            _ => self,
        }
    }

    pub fn substitute_all(mut self, assignments: &Assignments) -> Self {
        for (name, term) in assignments.substitutions() {
            self = self.substitute(name, term);
        }
        self
    }

    pub fn free_variables(&self) -> HashSet<String> {
        match self {
            Self::Variable { name } => [String::from(name)].into_iter().collect(),
            Self::Functor { arguments, .. } => arguments
                .iter()
                .flat_map(|arg| arg.free_variables().into_iter())
                .collect(),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable { name } => {
                write!(f, "{}", name)?;
            }
            Self::Functor { name, arguments } => {
                write!(f, "{}", name)?;
                if !arguments.is_empty() {
                    write!(f, "(")?;
                    for i in 0..arguments.len() {
                        write!(f, "{}", arguments[i])?;
                        if i < arguments.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }
                    write!(f, ")")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_term() {
        let term = Term::functor("father", [Term::atom("a"), Term::variable("X")]);

        use Term::*;
        assert_eq!(
            term,
            Functor {
                name: String::from("father"),
                arguments: vec![
                    Functor {
                        name: String::from("a"),
                        arguments: vec![]
                    },
                    Variable {
                        name: String::from("X")
                    },
                ]
            }
        )
    }

    #[test]
    fn substitute_variable() {
        let original_term = Term::variable("X");

        let subistituded = original_term.substitute("X", &Term::atom("a"));

        assert_eq!(subistituded, Term::atom("a"));
    }

    #[test]
    fn do_not_substitute_different_variable() {
        let original_term = Term::variable("Y");

        let subistituded = original_term.substitute("X", &Term::atom("a"));

        assert_eq!(subistituded, Term::variable("Y"));
    }

    #[test]
    fn do_not_substitute_atom() {
        let original_term = Term::atom("b");

        let subistituded = original_term.substitute("X", &Term::atom("a"));

        assert_eq!(subistituded, Term::atom("b"));
    }

    #[test]
    fn substitute_fuctor_arguments() {
        let original_term = Term::functor(
            "test",
            [Term::atom("b"), Term::variable("X"), Term::variable("Y")],
        );

        let subistituded = original_term.substitute("X", &Term::atom("a"));

        let expected_term = Term::functor(
            "test",
            [Term::atom("b"), Term::atom("a"), Term::variable("Y")],
        );
        assert_eq!(subistituded, expected_term)
    }

    #[test]
    fn free_variable_of_atom_is_empty() {
        let term = Term::atom("a");

        assert_eq!(term.free_variables(), HashSet::new());
    }

    #[test]
    fn free_variable_of_variable_is_variable() {
        let term = Term::variable("X");

        let expected_fvs = [String::from("X")].into_iter().collect();
        assert_eq!(term.free_variables(), expected_fvs);
    }

    #[test]
    fn free_variable_of_functor_is_free_variable_of_arguments() {
        let term = Term::functor(
            "test",
            [Term::variable("X"), Term::atom("a"), Term::variable("Y")],
        );

        let expected_fvs = [String::from("X"), String::from("Y")].into_iter().collect();
        assert_eq!(term.free_variables(), expected_fvs);
    }

    #[test]
    fn free_variable_of_complex_term() {
        let term = Term::functor(
            "test",
            [
                Term::functor("other", [Term::variable("X"), Term::atom("b")]),
                Term::atom("a"),
                Term::functor("thing", [Term::variable("Z")]),
            ],
        );

        let expected_fvs = [String::from("X"), String::from("Z")].into_iter().collect();
        assert_eq!(term.free_variables(), expected_fvs);
    }
}
