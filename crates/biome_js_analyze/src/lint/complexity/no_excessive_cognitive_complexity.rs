use biome_analyze::{
    context::RuleContext, declare_lint_rule, AddVisitor, Phases, QueryMatch, Queryable, Rule,
    RuleDiagnostic, RuleSource, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    AnyFunctionLike, JsBreakStatement, JsContinueStatement, JsElseClause, JsLanguage,
    JsLogicalExpression, JsLogicalOperator, JsSyntaxNode,
};
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange, WalkEvent};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

const MAX_FUNCTION_DEPTH: usize = 10;
const MAX_SCORE: u8 = u8::MAX;

declare_lint_rule! {
    /// Disallow functions that exceed a given Cognitive Complexity score.
    ///
    /// The more complexity a function contains, the harder it is to understand
    /// later on.
    ///
    /// Reducing complexity helps to make code more maintenable, both by making
    /// it easier to understand as well as by reducing chances of accidental
    /// side-effects when making changes.
    ///
    /// This rule calculates a complexity score for every function and disallows
    /// those that exceed a configured complexity threshold (default: 15).
    ///
    /// The complexity score is calculated based on the Cognitive Complexity
    /// algorithm: https://redirect.sonarsource.com/doc/cognitive-complexity.html
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function tooComplex() {
    ///     for (let x = 0; x < 10; x++) {
    ///         for (let y = 0; y < 10; y++) {
    ///             for (let z = 0; z < 10; z++) {
    ///                 if (x % 2 === 0) {
    ///                     if (y % 2 === 0) {
    ///                         console.log(x > y ? `${x} > ${y}` : `${y} > ${x}`);
    ///                     }
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// Allows to specify the maximum allowed complexity.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxAllowedComplexity": 15
    ///     }
    /// }
    /// ```
    ///
    /// The allowed values range from 1 through 254. The default is 15.
    ///
    pub NoExcessiveCognitiveComplexity {
        version: "1.0.0",
        name: "noExcessiveCognitiveComplexity",
        language: "js",
        sources: &[RuleSource::EslintSonarJs("cognitive-complexity")],
        recommended: false,
    }
}

impl Rule for NoExcessiveCognitiveComplexity {
    type Query = CognitiveComplexity;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ComplexityOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let calculated_score = ctx.query().score.calculated_score;
        (calculated_score > ctx.options().max_allowed_complexity.get()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let CognitiveComplexity {
            function_like,
            score: ComplexityScore { calculated_score },
        } = ctx.query();

        let ComplexityOptions {
            max_allowed_complexity,
        } = ctx.options();

        let range = function_like
            .name_range()
            .or_else(|| {
                function_like
                    .function_token()
                    .map(|token| token.text_range())
            })
            .or_else(|| {
                function_like
                    .fat_arrow_token()
                    .map(|token| token.text_range())
            })
            .or_else(|| function_like.body().ok().map(|body| body.range()))?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup!({
                    format!(
                        "Excessive complexity of {calculated_score} detected \
                    (max: {max_allowed_complexity})."
                    )
                }),
            )
            .note(if calculated_score == &MAX_SCORE {
                "Please refactor this function to reduce its complexity. \
                It's currently too complex or too deeply nested to calculate an accurate score."
                    .to_owned()
            } else {
                format!(
                    "Please refactor this function to reduce its complexity score from \
                    {calculated_score} to the max allowed complexity {max_allowed_complexity}."
                )
            }),
        )
    }
}

#[derive(Clone)]
pub struct CognitiveComplexity {
    function_like: AnyFunctionLike,
    score: ComplexityScore,
}

impl QueryMatch for CognitiveComplexity {
    fn text_range(&self) -> TextRange {
        self.function_like.range()
    }
}

impl Queryable for CognitiveComplexity {
    type Input = Self;
    type Language = JsLanguage;
    type Output = Self;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, CognitiveComplexityVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.clone()
    }
}

struct CognitiveComplexityFunctionState {
    function_like: AnyFunctionLike,
    score: u8,
    nesting_level: u8,

    /// Cognitive complexity does not increase for every logical operator,
    /// but for every *sequence* of identical logical operators. Therefore, we
    /// track which operator was last seen and incur a penalty when a different
    /// operator is encountered.
    last_seen_operator: Option<JsLogicalOperator>,
}

#[derive(Default)]
struct CognitiveComplexityVisitor {
    stack: Vec<CognitiveComplexityFunctionState>,
}

impl Visitor for CognitiveComplexityVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => self.on_enter(node),
            WalkEvent::Leave(node) => self.on_leave(node, ctx),
        }
    }
}

impl CognitiveComplexityVisitor {
    fn on_enter(&mut self, node: &JsSyntaxNode) {
        let parent = self.stack.last();
        if parent.is_some_and(|parent| parent.score == MAX_SCORE) {
            return; // No need for further processing if we're already at the max.
        }

        // When the visitor enters a function node, push a new entry on the stack
        if let Some(function_like) = AnyFunctionLike::cast_ref(node) {
            if self.stack.len() < MAX_FUNCTION_DEPTH {
                self.stack.push(CognitiveComplexityFunctionState {
                    function_like,
                    score: 0,
                    nesting_level: parent
                        .map(|parent| parent.nesting_level + 1)
                        .unwrap_or_default(),
                    last_seen_operator: None,
                });
            } else if let Some(parent) = self.stack.last_mut() {
                // Just mark the parent as being too complex. It already had a
                // crazy level of nesting, so there's no point in reporting even
                // deeper nested functions individually.
                parent.score = MAX_SCORE;
            }
        }

        if let Some(state) = self.stack.last_mut() {
            if receives_structural_penalty(node) {
                state.score = state.score.saturating_add(1);

                if receives_nesting_penalty(node) {
                    state.score = state.score.saturating_add(state.nesting_level);
                }
            }

            if increases_nesting(node) {
                state.last_seen_operator = None;
                state.nesting_level = state.nesting_level.saturating_add(1);
            } else if let Some(operator) = JsLogicalExpression::cast_ref(node)
                .and_then(|expression| expression.operator().ok())
            {
                if state.last_seen_operator != Some(operator) {
                    state.score = state.score.saturating_add(1);
                    state.last_seen_operator = Some(operator);
                }
            } else if let Some(alternate) =
                JsElseClause::cast_ref(node).and_then(|js_else| js_else.alternate().ok())
            {
                if alternate.as_js_if_statement().is_some() {
                    // Prevent double nesting inside else-if.
                    state.nesting_level = state.nesting_level.saturating_sub(1);
                } else {
                    state.score = state.score.saturating_add(1);
                }
            } else {
                // Reset the operator for every other type of node.
                state.last_seen_operator = None;
            }
        }
    }

    fn on_leave(&mut self, node: &JsSyntaxNode, mut ctx: VisitorContext<JsLanguage>) {
        if let Some(exit_node) = AnyFunctionLike::cast_ref(node) {
            if let Some(function_state) = self.stack.pop() {
                if function_state.function_like == exit_node {
                    ctx.match_query(CognitiveComplexity {
                        function_like: exit_node,
                        score: ComplexityScore {
                            calculated_score: function_state.score,
                        },
                    });
                } else {
                    // Push it back. This really should only be necessary
                    // if the max complexity or max nesting level was reached.
                    self.stack.push(function_state);
                }
            }
        } else if let Some(state) = self.stack.last_mut() {
            if state.score < MAX_SCORE {
                if increases_nesting(node) {
                    state.nesting_level = state.nesting_level.saturating_sub(1);
                } else if let Some(alternate) =
                    JsElseClause::cast_ref(node).and_then(|js_else| js_else.alternate().ok())
                {
                    state.nesting_level = if alternate.as_js_if_statement().is_some() {
                        // Prevent double nesting inside else-if.
                        state.nesting_level.saturating_add(1)
                    } else {
                        state.nesting_level.saturating_sub(1)
                    };
                }
            }
        }
    }
}

/// Returns whether the node is considered to increase the nesting level inside
/// the function.
///
/// Note: These are mostly nodes that increase the complexity of the function's
/// control flow.
fn increases_nesting(node: &JsSyntaxNode) -> bool {
    use biome_js_syntax::JsSyntaxKind::*;
    is_loop_node(node)
        || matches!(
            node.kind(),
            JS_CATCH_CLAUSE | JS_CONDITIONAL_EXPRESSION | JS_IF_STATEMENT | JS_SWITCH_STATEMENT
        )
}

fn is_loop_node(node: &JsSyntaxNode) -> bool {
    use biome_js_syntax::JsSyntaxKind::*;
    matches!(
        node.kind(),
        JS_DO_WHILE_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_FOR_STATEMENT
            | JS_WHILE_STATEMENT
    )
}

/// Returns whether use of the given node results in a penalty for increasing
/// the complexity of the structure of the function.
///
/// The structure of a function is mostly defined by its control flow, although
/// there are some node types that we consider as increasing its structural
/// complexity even though they do not affect its control flow.
///
/// A prime example of this is the `with` statement, which does not affect
/// control flow, but which is considered to increase structural complexity
/// since developers will need to spend additional effort tracing the scope of
/// variables.
///
/// Do note that the SonarSource paper makes no mention of the `with` statement
/// specifically (probably because it's highly specific to JavaScript), so its
/// inclusion here is a personal judgement call.
fn receives_structural_penalty(node: &JsSyntaxNode) -> bool {
    use biome_js_syntax::JsSyntaxKind::*;
    receives_nesting_penalty(node)
        || matches!(node.kind(), JS_FINALLY_CLAUSE | JS_WITH_STATEMENT)
        || JsBreakStatement::cast_ref(node)
            .and_then(|js_break| js_break.label_token())
            .is_some()
        || JsContinueStatement::cast_ref(node)
            .and_then(|js_continue| js_continue.label_token())
            .is_some()
}

/// Returns whether use of the given node receives an additional penalty based
/// on the level of nesting in which it occurs.
///
/// Note: This is a strict subset of the nodes that receive a structural penalty.
fn receives_nesting_penalty(node: &JsSyntaxNode) -> bool {
    use biome_js_syntax::JsSyntaxKind::*;
    is_loop_node(node)
        || matches!(
            node.kind(),
            JS_CATCH_CLAUSE | JS_CONDITIONAL_EXPRESSION | JS_IF_STATEMENT | JS_SWITCH_STATEMENT
        )
}

#[derive(Clone, Default)]
pub struct ComplexityScore {
    calculated_score: u8,
}

/// Options for the rule `noExcessiveCognitiveComplexity`.
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct ComplexityOptions {
    /// The maximum complexity score that we allow. Anything higher is considered excessive.
    pub max_allowed_complexity: NonZeroU8,
}

impl Default for ComplexityOptions {
    fn default() -> Self {
        Self {
            max_allowed_complexity: NonZeroU8::new(15).unwrap(),
        }
    }
}
