use super::{
    PatternCompiler, compilation_context::NodeCompilationContext,
    predicate_compiler::PredicateCompiler,
};
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::{
    GritFunctionDefinition, GritPatternAnd, GritPatternList, GritPredicateAnd, GritPredicateList,
};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{
    And, Assignment, BooleanConstant, Container, Match, Pattern, PrAnd, Predicate, Where,
};

pub(crate) struct AndCompiler;

impl AndCompiler {
    pub(crate) fn from_node(
        node: &GritPatternAnd,
        context: &mut NodeCompilationContext,
    ) -> Result<And<GritQueryContext>, CompileError> {
        Self::from_patterns(node.patterns(), context)
    }

    pub(crate) fn from_patterns(
        patterns: GritPatternList,
        context: &mut NodeCompilationContext,
    ) -> Result<And<GritQueryContext>, CompileError> {
        let patterns = patterns
            .into_iter()
            .map(|pattern| match pattern {
                Ok(pattern) => Ok(PatternCompiler::from_node(&pattern, context)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(And::new(patterns))
    }
}

pub(crate) struct PrAndCompiler;

impl PrAndCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateAnd,
        context: &mut NodeCompilationContext,
    ) -> Result<PrAnd<GritQueryContext>, CompileError> {
        let and = Self::from_predicates(node.predicates(), context)?;

        // Hoisting moves predicates into a `where` side condition, which
        // discards `return` values, so function bodies are left untouched.
        let in_function = node
            .syntax()
            .ancestors()
            .any(|ancestor| GritFunctionDefinition::can_cast(ancestor.kind()));
        if in_function {
            return Ok(and);
        }

        Ok(PrAnd::new(hoist_into_contains(and.predicates, context)))
    }

    pub(crate) fn from_predicates(
        predicates: GritPredicateList,
        context: &mut NodeCompilationContext,
    ) -> Result<PrAnd<GritQueryContext>, CompileError> {
        let predicates = predicates
            .into_iter()
            .map(|predicate| match predicate {
                Ok(predicate) => Ok(PredicateCompiler::from_node(&predicate, context)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(PrAnd::new(predicates))
    }
}

/// Moves the predicates that follow a `$x <: contains P` match into the
/// `contains` itself, so they are evaluated for every candidate instead of only
/// for the first node matched by `P`:
///
/// ```grit
/// $x <: contains P, rest
/// ```
///
/// becomes:
///
/// ```grit
/// $guard = false,
/// $x <: contains P where { $guard <: false, rest, $guard = true }
/// ```
///
/// `and` doesn't backtrack, so without this, `rest` can only ever see the
/// variables bound by the first node matching `P`, even if a later node would
/// have satisfied `rest`.
///
/// The guard makes `contains` stop accepting candidates once one has satisfied
/// `rest`, so its effects (rewrites, diagnostics) still apply only once. It is
/// reset before the `contains` because the whole `and` may run once per
/// candidate of an enclosing `contains`, and local variables are not reset
/// between those.
fn hoist_into_contains(
    mut predicates: Vec<Predicate<GritQueryContext>>,
    context: &mut NodeCompilationContext,
) -> Vec<Predicate<GritQueryContext>> {
    let Some(index) = predicates.iter().position(is_contains_match) else {
        return predicates;
    };
    if index + 1 == predicates.len() {
        return predicates;
    }

    let rest = hoist_into_contains(predicates.split_off(index + 1), context);
    let Some(Predicate::Match(mut contains_match)) = predicates.pop() else {
        unreachable!("the predicate at `index` is a contains match");
    };
    let Some(Pattern::Contains(contains)) = contains_match.pattern.as_mut() else {
        unreachable!("the predicate at `index` is a contains match");
    };

    let guard_name = format!(
        "$__contains_guard_{}",
        context.vars_array[context.scope_index].len()
    );
    let guard = context.variable_from_name(guard_name);
    let set_guard = |value| {
        Predicate::Assignment(Box::new(Assignment::new(
            Container::Variable(guard.clone()),
            Pattern::BooleanConstant(BooleanConstant::new(value)),
        )))
    };

    let mut side_condition = Vec::with_capacity(rest.len() + 2);
    side_condition.push(Predicate::Match(Box::new(Match::new(
        Container::Variable(guard.clone()),
        Some(Pattern::BooleanConstant(BooleanConstant::new(false))),
    ))));
    side_condition.extend(rest);
    side_condition.push(set_guard(true));

    let contained = std::mem::replace(&mut contains.contains, Pattern::Top);
    contains.contains = Pattern::Where(Box::new(Where::new(
        contained,
        Predicate::And(Box::new(PrAnd::new(side_condition))),
    )));

    predicates.push(set_guard(false));
    predicates.push(Predicate::Match(contains_match));
    predicates
}

fn is_contains_match(predicate: &Predicate<GritQueryContext>) -> bool {
    matches!(
        predicate,
        Predicate::Match(m) if matches!(m.pattern, Some(Pattern::Contains(_)))
    )
}
