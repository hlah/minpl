use crate::assignments::Assignments;
use crate::term::Term;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnifyResult {
    True(Assignments),
    False,
}

pub fn unify(a: Term, b: Term) -> UnifyResult {
    unify_terms(vec![(a, b)])
}

fn unify_terms(mut constraints: Vec<(Term, Term)>) -> UnifyResult {
    if let Some((lhs, rhs)) = constraints.pop() {
        if lhs == rhs {
            unify_terms(constraints)
        } else {
            match (lhs, rhs) {
                (Term::Variable { name }, term) => {
                    let subistituded_constraints = constraints
                        .into_iter()
                        .map(|(ta, tb)| (ta.substitute(&name, &term), tb.substitute(&name, &term)))
                        .collect();
                    if let UnifyResult::True(assignments) = unify_terms(subistituded_constraints) {
                        UnifyResult::True(assignments.with(name, term))
                    } else {
                        UnifyResult::False
                    }
                }
                (term, Term::Variable { name }) => {
                    let subistituded_constraints = constraints
                        .into_iter()
                        .map(|(ta, tb)| (ta.substitute(&name, &term), tb.substitute(&name, &term)))
                        .collect();
                    if let UnifyResult::True(assignments) = unify_terms(subistituded_constraints) {
                        UnifyResult::True(assignments.with(name, term))
                    } else {
                        UnifyResult::False
                    }
                }
                (
                    Term::Functor {
                        name: a_name,
                        arguments: a_args,
                    },
                    Term::Functor {
                        name: b_name,
                        arguments: b_args,
                    },
                ) if a_name == b_name && a_args.len() == b_args.len() => {
                    constraints
                        .append(&mut a_args.into_iter().zip(b_args.into_iter()).rev().collect());
                    unify_terms(constraints)
                }
                _ => UnifyResult::False,
            }
        }
    } else {
        UnifyResult::True(Assignments::empty())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn unify_equal_atoms() {
        let term_a = Term::atom("a");
        let term_b = Term::atom("a");

        assert_eq!(
            unify(term_a, term_b),
            UnifyResult::True(Assignments::empty())
        );
    }

    #[test]
    fn do_not_unify_different_atoms() {
        let term_a = Term::atom("a");
        let term_b = Term::atom("b");

        assert_eq!(unify(term_a, term_b), UnifyResult::False);
    }

    #[test_case(Term::atom("a"))]
    #[test_case(Term::variable("Y"))]
    #[test_case(Term::functor("test", [Term::variable("Z"), Term::atom("k")]))]
    fn unify_variable_left_with_anything(something: Term) {
        let var = Term::variable("X");

        let expected_assignments = Assignments::empty().with("X", something.clone());
        assert_eq!(
            unify(var, something),
            UnifyResult::True(expected_assignments)
        );
    }

    #[test_case(Term::atom("a"))]
    #[test_case(Term::functor("test", [Term::variable("Z"), Term::atom("k")]))]
    fn unify_variable_right_with_anything(something: Term) {
        let var = Term::variable("X");

        let expected_assignments = Assignments::empty().with("X", something.clone());
        assert_eq!(
            unify(something, var),
            UnifyResult::True(expected_assignments)
        );
    }

    #[test]
    fn unify_functors() {
        let term_a = Term::functor("test", [Term::atom("a"), Term::atom("b")]);
        let term_b = Term::functor("test", [Term::atom("a"), Term::atom("b")]);

        assert_eq!(
            unify(term_a, term_b),
            UnifyResult::True(Assignments::empty())
        )
    }

    #[test]
    fn do_not_unify_functors_with_different_names() {
        let term_a = Term::functor("test", [Term::atom("a"), Term::atom("b")]);
        let term_b = Term::functor("atest", [Term::atom("a"), Term::atom("b")]);

        assert_eq!(unify(term_a, term_b), UnifyResult::False,)
    }

    #[test]
    fn do_not_unify_functors_with_different_arities() {
        let term_a = Term::functor("test", [Term::atom("a"), Term::atom("b")]);
        let term_b = Term::functor("test", [Term::atom("a"), Term::atom("b"), Term::atom("c")]);

        assert_eq!(unify(term_a, term_b), UnifyResult::False,)
    }

    #[test]
    fn unify_functors_with_variables() {
        let term_a = Term::functor("test", [Term::variable("X"), Term::atom("b")]);
        let term_b = Term::functor("test", [Term::atom("a"), Term::variable("Y")]);

        assert_eq!(
            unify(term_a, term_b),
            UnifyResult::True(
                Assignments::empty()
                    .with("X", Term::atom("a"))
                    .with("Y", Term::atom("b"))
            )
        )
    }

    #[test]
    fn do_not_unify_functors_with_variables_with_incompatible_assignments() {
        let term_a = Term::functor("test", [Term::variable("X"), Term::atom("b")]);
        let term_b = Term::functor("test", [Term::atom("a"), Term::variable("X")]);

        assert_eq!(unify(term_a, term_b), UnifyResult::False)
    }

    #[test]
    fn unify_complex_term() {
        let term_a = Term::functor(
            "vertical",
            [Term::functor(
                "line",
                [
                    Term::functor("point", [Term::variable("X"), Term::variable("Y")]),
                    Term::functor("point", [Term::variable("X"), Term::variable("Z")]),
                ],
            )],
        );

        let term_b = Term::functor(
            "vertical",
            [Term::functor(
                "line",
                [
                    Term::functor("point", [Term::atom("1"), Term::atom("1")]),
                    Term::functor("point", [Term::atom("1"), Term::atom("3")]),
                ],
            )],
        );

        assert_eq!(
            unify(term_a, term_b),
            UnifyResult::True(
                Assignments::empty()
                    .with("X", Term::atom("1"))
                    .with("Y", Term::atom("1"))
                    .with("Z", Term::atom("3"))
            )
        )
    }

    #[test]
    fn do_not_unify_complex_term() {
        let term_a = Term::functor(
            "vertical",
            [Term::functor(
                "line",
                [
                    Term::functor("point", [Term::variable("X"), Term::variable("Y")]),
                    Term::functor("point", [Term::variable("X"), Term::variable("Z")]),
                ],
            )],
        );

        let term_b = Term::functor(
            "vertical",
            [Term::functor(
                "line",
                [
                    Term::functor("point", [Term::atom("1"), Term::atom("1")]),
                    Term::functor("point", [Term::atom("2"), Term::atom("3")]),
                ],
            )],
        );

        assert_eq!(unify(term_a, term_b), UnifyResult::False)
    }

    #[test]
    fn unify_complex_term_with_variables_on_both_sides() {
        let term_a = Term::functor(
            "vertical",
            [Term::functor(
                "line",
                [
                    Term::functor("point", [Term::variable("X"), Term::variable("Y")]),
                    Term::functor("point", [Term::variable("X"), Term::variable("Z")]),
                ],
            )],
        );

        let term_b = Term::functor(
            "vertical",
            [Term::functor(
                "line",
                [
                    Term::functor("point", [Term::atom("1"), Term::variable("K")]),
                    Term::functor("point", [Term::atom("1"), Term::atom("3")]),
                ],
            )],
        );

        assert_eq!(
            unify(term_a, term_b),
            UnifyResult::True(
                Assignments::empty()
                    .with("X", Term::atom("1"))
                    .with("Y", Term::variable("K"))
                    .with("Z", Term::atom("3"))
            )
        )
    }

    #[test]
    fn unify_terms_with_transitional_non_conflicting_assignments() {
        let term_a = Term::functor(
            "test",
            [
                Term::variable("X"),
                Term::variable("X"),
                Term::variable("Y"),
            ],
        );

        let term_b = Term::functor(
            "test",
            [Term::variable("Y"), Term::atom("a"), Term::atom("a")],
        );

        assert_eq!(
            unify(term_a, term_b),
            UnifyResult::True(
                Assignments::empty()
                    .with("X", Term::variable("Y"))
                    .with("Y", Term::atom("a"))
            )
        )
    }

    #[test]
    fn do_not_unify_terms_with_transitional_conflicting_assignments() {
        let term_a = Term::functor(
            "test",
            [
                Term::variable("X"),
                Term::variable("X"),
                Term::variable("Y"),
            ],
        );

        let term_b = Term::functor(
            "test",
            [Term::variable("Y"), Term::atom("a"), Term::atom("b")],
        );

        assert_eq!(unify(term_a, term_b), UnifyResult::False)
    }

    #[test]
    fn process_argumens_in_given_order() {
        let term_a = Term::functor("test", [Term::variable("X"), Term::variable("Y")]);
        let term_b = Term::functor("test", [Term::variable("Y"), Term::variable("X")]);

        assert_eq!(
            unify(term_a, term_b),
            UnifyResult::True(Assignments::empty().with("X", Term::variable("Y")))
        )
    }
}
