use crate::term::Term;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MinplParser;

impl MinplParser {
    fn parse_term(code: &str) -> Term {
        Self::build_term(Self::parse(Rule::term, code).unwrap().next().unwrap())
    }

    fn build_term(pair: Pair<Rule>) -> Term {
        match pair.as_rule() {
            Rule::variable => Term::variable(pair.as_str()),
            Rule::atom => Term::atom(pair.as_str()),
            Rule::functor => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let args = inner.map(|pair| Self::build_term(pair));
                Term::functor(name, args)
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_variable() {
        let code = "X";

        let term = MinplParser::parse_term(code);

        assert_eq!(term, Term::variable("X"));
    }

    #[test]
    fn parse_variable_with_long_name() {
        let code = "Test";

        let term = MinplParser::parse_term(code);

        assert_eq!(term, Term::variable("Test"));
    }

    #[test]
    fn parse_atom() {
        let code = "a";

        let term = MinplParser::parse_term(code);

        assert_eq!(term, Term::atom("a"));
    }

    #[test]
    fn parse_atom_with_long_same() {
        let code = "aTest";

        let term = MinplParser::parse_term(code);

        assert_eq!(term, Term::atom("aTest"));
    }

    #[test]
    fn parse_functor() {
        let code = "test(a , X)";

        let term = MinplParser::parse_term(code);

        assert_eq!(
            term,
            Term::functor("test", [Term::atom("a"), Term::variable("X")])
        );
    }

    #[test]
    fn parse_complex_term() {
        let code = "test(aPredicate(atom, Y, other(X, b)), X)";

        let term = MinplParser::parse_term(code);

        assert_eq!(
            term,
            Term::functor(
                "test",
                [
                    Term::functor(
                        "aPredicate",
                        [
                            Term::atom("atom"),
                            Term::variable("Y"),
                            Term::functor("other", [Term::variable("X"), Term::atom("b")])
                        ]
                    ),
                    Term::variable("X"),
                ]
            )
        );
    }
}
