use crate::{database::Database, term::Term};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MinplParser;

impl MinplParser {
    pub fn parse_query(code: &str) -> Term {
        Self::build_query(Self::parse(Rule::query, code).unwrap().next().unwrap())
    }

    pub fn parse_database(code: &str) -> Database {
        Self::build_database(Self::parse(Rule::database, code).unwrap().next().unwrap())
    }

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
            _ => unreachable!(),
        }
    }

    fn build_database(pair: Pair<Rule>) -> Database {
        let mut database = Database::empty();
        let pair = pair.into_inner().next().unwrap();
        for rule in pair.into_inner() {
            database.add(Self::build_rule(rule));
        }
        database
    }

    fn build_rule(pair: Pair<Rule>) -> crate::database::Rule {
        let mut inner = pair.into_inner();
        let head = Self::build_term(inner.next().unwrap());
        let body = inner.map(|term| Self::build_term(term)).collect();
        crate::database::Rule { head, body }
    }

    fn build_query(pair: Pair<Rule>) -> Term {
        Self::build_term(pair.into_inner().next().unwrap())
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

    #[test]
    fn parse_database() {
        let code = "father(peter, john). father(peter, adam). brother(X, Y) :- father(Z, X), father(Z, Y).";

        let database = MinplParser::parse_database(code);

        let expected_database = Database::empty()
            .with_fact(Term::functor(
                "father",
                [Term::atom("peter"), Term::atom("john")],
            ))
            .with_fact(Term::functor(
                "father",
                [Term::atom("peter"), Term::atom("adam")],
            ))
            .with_rule(
                Term::functor("brother", [Term::variable("X"), Term::variable("Y")]),
                [
                    Term::functor("father", [Term::variable("Z"), Term::variable("X")]),
                    Term::functor("father", [Term::variable("Z"), Term::variable("Y")]),
                ],
            );

        assert_eq!(expected_database, database);
    }

    #[test]
    fn parse_query() {
        let code = "brother(john, X).";

        let query = MinplParser::parse_query(code);

        let expected_query = Term::functor("brother", [Term::atom("john"), Term::variable("X")]);
        assert_eq!(expected_query, query);
    }
}
