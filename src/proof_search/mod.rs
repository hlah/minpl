mod temp_provider;

use crate::{
    assignments::Assignments,
    database::{Database, Rule},
    term::Term,
    unification::*,
};
use std::collections::HashSet;
use temp_provider::TempProvider;

pub fn prove(database: &Database, goal: Term) -> Vec<Assignments> {
    let fvs = goal.free_variables();
    prove_goals(database, vec![goal], &mut TempProvider::default())
        .into_iter()
        .map(|assignment| assignment.normalized(fvs.iter().map(|s| s.as_str())))
        .collect()
}

fn prove_goals(
    database: &Database,
    mut goals: Vec<Term>,
    temp_provider: &mut TempProvider,
) -> Vec<Assignments> {
    if let Some(goal) = goals.pop() {
        let mut solutions = vec![];
        for rule in database.rules() {
            let rule = fresh_variables(rule, temp_provider);
            if let UnifyResult::True(assignments) = unify(goal.clone(), rule.head.clone()) {
                let subgoals: Vec<_> = rule
                    .body
                    .iter()
                    .cloned()
                    .map(|subgoal| subgoal.substitute_all(&assignments))
                    .rev()
                    .collect();
                solutions.extend(
                    prove_goals(database, subgoals, temp_provider)
                        .into_iter()
                        .map(|solution| assignments.clone().merge(solution).unwrap()),
                );
            }
        }
        solutions
            .iter()
            .flat_map(|solution| {
                let remaining_goals: Vec<_> = goals
                    .iter()
                    .cloned()
                    .map(|goal| goal.substitute_all(&solution))
                    .collect();
                prove_goals(database, remaining_goals, temp_provider)
                    .into_iter()
                    .flat_map(|tail_solution| solution.clone().merge(tail_solution))
            })
            .collect()
    } else {
        vec![Assignments::empty()]
    }
}

fn fresh_variables(rule: &Rule, temp_provider: &mut TempProvider) -> Rule {
    let mut fvs = HashSet::new();
    fvs.extend(rule.head.free_variables());
    fvs.extend(
        rule.body
            .iter()
            .flat_map(|goal| goal.free_variables().into_iter()),
    );
    let fv_to_temp = fvs
        .into_iter()
        .map(|fv| (fv, Term::variable(temp_provider.get())));
    let fv_assignments = Assignments::new(fv_to_temp);
    let head = rule.head.clone().substitute_all(&fv_assignments);
    let body = rule
        .body
        .iter()
        .cloned()
        .map(|goal| goal.substitute_all(&fv_assignments))
        .collect();

    Rule { head, body }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proves_given_fact() {
        let database = Database::empty().with_fact(Term::functor("f", [Term::atom("a")]));

        let result = prove(&database, Term::functor("f", [Term::atom("a")]));

        assert_eq!(result, vec![Assignments::empty()]);
    }

    #[test]
    fn proves_from_rule() {
        let database = Database::empty()
            .with_fact(Term::functor("f", [Term::atom("a")]))
            .with_rule(
                Term::functor("g", [Term::atom("a")]),
                [Term::functor("f", [Term::atom("a")])],
            );

        let result = prove(&database, Term::functor("g", [Term::atom("a")]));

        assert_eq!(result, vec![Assignments::empty()]);
    }

    #[test]
    fn do_not_proves_if_does_not_follow_from_rules() {
        let database = Database::empty().with_rule(
            Term::functor("g", [Term::atom("a")]),
            [Term::functor("f", [Term::atom("a")])],
        );

        let result = prove(&database, Term::functor("g", [Term::atom("a")]));

        assert_eq!(result, vec![]);
    }

    #[test]
    fn proves_from_rule_with_multiple_goals() {
        let database = Database::empty()
            .with_fact(Term::functor("f", [Term::atom("a")]))
            .with_fact(Term::functor("g", [Term::atom("a")]))
            .with_fact(Term::functor("h", [Term::atom("a")]))
            .with_fact(Term::functor("f", [Term::atom("b")]))
            .with_rule(
                Term::functor("k", [Term::atom("a")]),
                [
                    Term::functor("f", [Term::atom("a")]),
                    Term::functor("g", [Term::atom("a")]),
                    Term::functor("h", [Term::atom("a")]),
                ],
            );

        let result = prove(&database, Term::functor("k", [Term::atom("a")]));

        assert_eq!(result, vec![Assignments::empty()]);
    }

    #[test]
    fn do_not_proves_from_rule_with_multiple_goals_if_some_goal_are_false() {
        let database = Database::empty()
            .with_fact(Term::functor("f", [Term::atom("a")]))
            .with_fact(Term::functor("g", [Term::atom("a")]))
            .with_fact(Term::functor("h", [Term::atom("a")]))
            .with_fact(Term::functor("f", [Term::atom("b")]))
            .with_rule(
                Term::functor("k", [Term::atom("b")]),
                [
                    Term::functor("f", [Term::atom("b")]),
                    Term::functor("g", [Term::atom("b")]),
                    Term::functor("h", [Term::atom("b")]),
                ],
            );

        let result = prove(&database, Term::functor("k", [Term::atom("b")]));

        assert_eq!(result, vec![]);
    }

    #[test]
    fn proves_fact_with_variable() {
        let database = Database::empty().with_fact(Term::functor("test", [Term::variable("X")]));

        let result = prove(&database, Term::functor("test", [Term::atom("a")]));

        assert_eq!(result, vec![Assignments::empty()]);
    }

    #[test]
    fn proves_goal_with_variable() {
        let database = Database::empty().with_fact(Term::functor("test", [Term::atom("a")]));

        let result = prove(&database, Term::functor("test", [Term::variable("X")]));

        assert_eq!(
            result,
            vec![Assignments::empty().with("X", Term::atom("a"))]
        );
    }

    #[test]
    fn prove_goal_and_rule_with_same_variable_name() {
        let database = Database::empty()
            .with_fact(Term::functor("one", [Term::atom("a")]))
            .with_rule(
                Term::functor("test", [Term::variable("X"), Term::atom("b")]),
                [Term::functor("one", [Term::variable("X")])],
            );

        let result = prove(
            &database,
            Term::functor("test", [Term::atom("a"), Term::variable("X")]),
        );

        assert_eq!(
            result,
            vec![Assignments::empty().with("X", Term::atom("b"))]
        );
    }
}
