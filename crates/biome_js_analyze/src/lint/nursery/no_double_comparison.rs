use crate::{
    JsRuleAction,
    services::semantic::Semantic,
    utils::same_reference::{is_same_reference, omit_type_wrappers},
};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsLogicalExpression, JsLogicalOperator, JsSyntaxKind, T, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, BatchMutationExt, Direction, TextRange};
use biome_rule_options::no_double_comparison::NoDoubleComparisonOptions;

declare_lint_rule! {
    /// Disallow two comparisons of the same values that can be combined into one.
    ///
    /// When `||` or `&&` joins two comparisons of the same two values, a single comparison
    /// often means the same thing. For example, `x === y || x < y` is the same as `x <= y`.
    /// A single comparison is shorter, easier to read, and evaluates each value only once.
    ///
    /// The values can appear in either order: `x === y || y < x` is the same as `x >= y`.
    ///
    /// The rule reports these pairs:
    ///
    /// | Code | Combined |
    /// | --- | --- |
    /// | `x === y \|\| x < y` | `x <= y` |
    /// | `x === y \|\| x > y` | `x >= y` |
    /// | `x < y \|\| x > y` | `x !== y` |
    /// | `x <= y && x >= y` | `x === y` |
    /// | `x <= y && x !== y` | `x < y` |
    /// | `x >= y && x !== y` | `x > y` |
    ///
    /// The rule only checks comparisons of variables, properties, `this`, and literal values
    /// such as numbers and strings. It ignores comparisons that use `==` or `!=`, call a
    /// function, or use optional chaining (`?.`).
    ///
    /// The fix is unsafe because the combined comparison can give a different result when the
    /// two values are of different types, or when a value is `undefined` or `NaN`. For example,
    /// `undefined === undefined || undefined < undefined` is `true`, but
    /// `undefined <= undefined` is `false`. With `x = "5"`, `x === 5 || x < 5` is `false`, but
    /// `x <= 5` is `true`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// x === y || x < y;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// x <= y && x >= y;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// x <= y && x !== y;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// x === y || y < x;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// x <= y;
    /// x >= y;
    /// x === y;
    /// x < y;
    /// x <= y || x < y;
    /// x === y || x < z;
    /// ```
    ///
    pub NoDoubleComparison {
        version: "next",
        name: "noDoubleComparison",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-double-comparison").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoDoubleComparison {
    type Query = Semantic<JsLogicalExpression>;
    /// The operator of the single comparison that replaces both comparisons.
    type State = ComparisonOperator;
    type Signals = Option<Self::State>;
    type Options = NoDoubleComparisonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let logical_operator = node.operator().ok()?;
        if logical_operator == JsLogicalOperator::NullishCoalescing {
            return None;
        }

        let left = as_comparison(node.left().ok()?)?;
        let right = as_comparison(node.right().ok()?)?;
        let left_operator = ComparisonOperator::from_binary_operator(left.operator().ok()?)?;
        let right_operator = ComparisonOperator::from_binary_operator(right.operator().ok()?)?;

        let first = left.left().ok()?;
        let second = left.right().ok()?;
        let right_first = right.left().ok()?;
        let right_second = right.right().ok()?;
        let right_operator = if is_same_reference(first.clone(), right_first.clone())?
            && is_same_reference(second.clone(), right_second.clone())?
        {
            right_operator
        } else if is_same_reference(first.clone(), right_second)?
            && is_same_reference(second.clone(), right_first)?
        {
            // Read the right comparison with its values swapped, so `y < x` becomes `x > y`.
            right_operator.flip()
        } else {
            return None;
        };

        let combined = combine(logical_operator, left_operator, right_operator)?;

        if has_optional_chain(&first) || has_optional_chain(&second) {
            return None;
        }

        // A regular expression is an object: `===` checks whether both values are the same
        // object, while `<` and `>` compare their text.
        let model = ctx.model();
        if is_const_regex(model, &first) || is_const_regex(model, &second) {
            return None;
        }

        Some(combined)
    }

    fn diagnostic(ctx: &RuleContext<Self>, combined: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let left = as_comparison(node.left().ok()?)?;
        let first = left.left().ok()?;
        let second = left.right().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "These two comparisons can be combined into "<Emphasis>{format_args!("{}", first.syntax().text_trimmed())}" "{combined}" "{format_args!("{}", second.syntax().text_trimmed())}</Emphasis>"."
                },
            )
            .note(markup! {
                "Both comparisons check the same two values, so a single comparison usually means the same thing with less code."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, combined: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let left = as_comparison(node.left().ok()?)?;
        let first = left.left().ok()?;
        let second = left.right().ok()?;

        // The replacement keeps only the two values of the left comparison, so a comment
        // anywhere else in the expression would be lost.
        if has_comment_outside(node, &[first.range(), second.range()]) {
            return None;
        }

        let mut second = second.trim_leading_trivia()?;
        // `as` and `satisfies` bind as tightly as `<`, so `x <= y as T` would read as
        // `(x <= y) as T`. Equality operators bind more loosely, so `x === y as T` doesn't
        // need the parentheses.
        if matches!(
            second,
            AnyJsExpression::TsAsExpression(_) | AnyJsExpression::TsSatisfiesExpression(_)
        ) {
            second = AnyJsExpression::JsParenthesizedExpression(make::js_parenthesized_expression(
                make::token(T!['(']),
                second.trim_trailing_trivia()?,
                make::token(T![')']),
            ));
        }
        let replacement = make::js_binary_expression(
            first.trim_trailing_trivia()?,
            make::token_decorated_with_space(combined.token_kind()),
            second,
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::JsLogicalExpression(node.clone()),
            AnyJsExpression::JsBinaryExpression(replacement),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Combine them into a single comparison." }.to_owned(),
            mutation,
        ))
    }
}

/// A comparison operator that the rule can combine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    StrictEquality,
    StrictInequality,
}

impl ComparisonOperator {
    /// Every variant, in [`Self::index`] order.
    const ALL: [Self; 6] = [
        Self::LessThan,
        Self::LessThanOrEqual,
        Self::GreaterThan,
        Self::GreaterThanOrEqual,
        Self::StrictEquality,
        Self::StrictInequality,
    ];

    /// The position of `self` in [`Self::ALL`].
    ///
    /// This is spelled out instead of using `self as usize` so that adding a variant fails to
    /// compile here, as a reminder to also add it to [`Self::ALL`], which [`COMBINED`] is built
    /// from. The compiler reduces this `match` to the discriminant, so it costs nothing.
    const fn index(self) -> usize {
        match self {
            Self::LessThan => 0,
            Self::LessThanOrEqual => 1,
            Self::GreaterThan => 2,
            Self::GreaterThanOrEqual => 3,
            Self::StrictEquality => 4,
            Self::StrictInequality => 5,
        }
    }

    /// Returns `None` for operators the rule doesn't combine, including `==` and `!=`.
    const fn from_binary_operator(operator: JsBinaryOperator) -> Option<Self> {
        Some(match operator {
            JsBinaryOperator::LessThan => Self::LessThan,
            JsBinaryOperator::LessThanOrEqual => Self::LessThanOrEqual,
            JsBinaryOperator::GreaterThan => Self::GreaterThan,
            JsBinaryOperator::GreaterThanOrEqual => Self::GreaterThanOrEqual,
            JsBinaryOperator::StrictEquality => Self::StrictEquality,
            JsBinaryOperator::StrictInequality => Self::StrictInequality,
            _ => return None,
        })
    }

    /// Returns the operator that gives the same result when the two values swap sides.
    const fn flip(self) -> Self {
        match self {
            Self::LessThan => Self::GreaterThan,
            Self::LessThanOrEqual => Self::GreaterThanOrEqual,
            Self::GreaterThan => Self::LessThan,
            Self::GreaterThanOrEqual => Self::LessThanOrEqual,
            Self::StrictEquality | Self::StrictInequality => self,
        }
    }

    const fn token_kind(self) -> JsSyntaxKind {
        match self {
            Self::LessThan => T![<],
            Self::LessThanOrEqual => T![<=],
            Self::GreaterThan => T![>],
            Self::GreaterThanOrEqual => T![>=],
            Self::StrictEquality => T![===],
            Self::StrictInequality => T![!==],
        }
    }
}

impl biome_console::fmt::Display for ComparisonOperator {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
            Self::StrictEquality => "===",
            Self::StrictInequality => "!==",
        })
    }
}

/// Returns the single comparison operator equivalent to joining two comparisons with
/// `logical`, where both comparisons have their values in the same order. The two
/// comparisons can appear in either order.
///
/// The answers are defined by [`combine_uncached`]. This function reads them from
/// [`COMBINED`], a table built from [`combine_uncached`] at compile time.
fn combine(
    logical: JsLogicalOperator,
    left: ComparisonOperator,
    right: ComparisonOperator,
) -> Option<ComparisonOperator> {
    // `get` can't fail, since `combined_index` is always less than `COMBINED.len()`. The
    // compiler proves this and removes the bounds check.
    COMBINED
        .get(combined_index(logical, left, right))
        .copied()
        .flatten()
}

/// The result of [`combine_uncached`] for every possible input, at [`combined_index`].
///
/// This table only exists for speed: it must always agree with [`combine_uncached`], and it
/// does because it's built by calling that function on every input while compiling.
///
/// The `match` in [`combine_uncached`] compiles to a chain of compares and jumps. The
/// operators in real code vary unpredictably, so the CPU guesses about one of those jumps
/// wrong per call, and each wrong guess costs around 20 cycles. Reading this table is one
/// index calculation and one byte load, with no jumps. In a benchmark of the function on its
/// own, the `match` took about 31 CPU cycles per call and this table about 2.
///
/// Each entry is one byte, so the table is 108 bytes.
const COMBINED: [Option<ComparisonOperator>; COMBINED_LEN] = {
    const LOGICAL_OPERATORS: [JsLogicalOperator; LOGICAL_OPERATOR_COUNT] = [
        JsLogicalOperator::NullishCoalescing,
        JsLogicalOperator::LogicalOr,
        JsLogicalOperator::LogicalAnd,
    ];

    let mut table = [None; COMBINED_LEN];
    // `for` loops aren't allowed in a `const` initializer, hence `while`. Indexing here can't
    // panic at run time: an out-of-range index would fail to compile.
    let mut i = 0;
    while i < LOGICAL_OPERATORS.len() {
        let logical = LOGICAL_OPERATORS[i];
        let mut j = 0;
        while j < ComparisonOperator::ALL.len() {
            let left = ComparisonOperator::ALL[j];
            let mut k = 0;
            while k < ComparisonOperator::ALL.len() {
                let right = ComparisonOperator::ALL[k];
                table[combined_index(logical, left, right)] =
                    combine_uncached(logical, left, right);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    table
};

const LOGICAL_OPERATOR_COUNT: usize = 3;
const COMBINED_LEN: usize =
    LOGICAL_OPERATOR_COUNT * ComparisonOperator::ALL.len() * ComparisonOperator::ALL.len();

/// Returns the position of `(logical, left, right)` in [`COMBINED`], treating the table as a
/// three-dimensional array of size 3 × 6 × 6.
const fn combined_index(
    logical: JsLogicalOperator,
    left: ComparisonOperator,
    right: ComparisonOperator,
) -> usize {
    // Spelled out instead of `logical as usize` so that a new `JsLogicalOperator` variant fails
    // to compile here, as a reminder to add it to `COMBINED`. The compiler reduces this `match`
    // to the discriminant.
    let logical = match logical {
        JsLogicalOperator::NullishCoalescing => 0,
        JsLogicalOperator::LogicalOr => 1,
        JsLogicalOperator::LogicalAnd => 2,
    };
    (logical * ComparisonOperator::ALL.len() + left.index()) * ComparisonOperator::ALL.len()
        + right.index()
}

/// Defines [`combine`]. Only called while compiling, to build [`COMBINED`].
const fn combine_uncached(
    logical: JsLogicalOperator,
    left: ComparisonOperator,
    right: ComparisonOperator,
) -> Option<ComparisonOperator> {
    use ComparisonOperator::*;
    use JsLogicalOperator::{LogicalAnd, LogicalOr};

    Some(match (logical, left, right) {
        (LogicalOr, StrictEquality, LessThan) | (LogicalOr, LessThan, StrictEquality) => {
            LessThanOrEqual
        }
        (LogicalOr, StrictEquality, GreaterThan) | (LogicalOr, GreaterThan, StrictEquality) => {
            GreaterThanOrEqual
        }
        (LogicalOr, LessThan, GreaterThan) | (LogicalOr, GreaterThan, LessThan) => StrictInequality,
        (LogicalAnd, LessThanOrEqual, GreaterThanOrEqual)
        | (LogicalAnd, GreaterThanOrEqual, LessThanOrEqual) => StrictEquality,
        (LogicalAnd, LessThanOrEqual, StrictInequality)
        | (LogicalAnd, StrictInequality, LessThanOrEqual) => LessThan,
        (LogicalAnd, GreaterThanOrEqual, StrictInequality)
        | (LogicalAnd, StrictInequality, GreaterThanOrEqual) => GreaterThan,
        _ => return None,
    })
}

/// Returns the comparison inside `expression`, looking through parentheses and TypeScript
/// type assertions.
fn as_comparison(expression: AnyJsExpression) -> Option<JsBinaryExpression> {
    match omit_type_wrappers(expression)? {
        AnyJsExpression::JsBinaryExpression(binary) => Some(binary),
        _ => None,
    }
}

/// Returns whether `expression` uses optional chaining (`?.`) anywhere.
fn has_optional_chain(expression: &AnyJsExpression) -> bool {
    expression
        .syntax()
        .descendants_tokens(Direction::Next)
        .any(|token| token.kind() == T![?.])
}

/// Returns whether `expression` is a variable declared with `const` whose value is a regular
/// expression literal, either directly or through other `const` variables, as in
/// `const a = /x/; const b = a;`.
fn is_const_regex(model: &SemanticModel, expression: &AnyJsExpression) -> bool {
    let mut expression = expression.clone();
    // Variables that were already followed. Invalid code such as `const a = b; const b = a;`
    // would otherwise loop forever.
    let mut visited = Vec::new();
    loop {
        let Some(AnyJsExpression::JsIdentifierExpression(identifier)) =
            omit_type_wrappers(expression)
        else {
            return false;
        };
        let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) = identifier
            .name()
            .ok()
            .and_then(|reference| model.binding(&reference))
            .and_then(|binding| binding.tree().declaration())
        else {
            return false;
        };
        if visited.contains(&declarator.range())
            || !declarator
                .declaration()
                .is_some_and(|declaration| declaration.is_const())
        {
            return false;
        }
        visited.push(declarator.range());
        let Some(initializer) = declarator
            .initializer()
            .and_then(|initializer| initializer.expression().ok())
            .and_then(omit_type_wrappers)
        else {
            return false;
        };
        if matches!(
            initializer,
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsRegexLiteralExpression(_)
            )
        ) {
            return true;
        }
        expression = initializer;
    }
}

/// Returns whether `node` contains a comment that isn't inside any of `kept_ranges`.
///
/// Comments before the first token or after the last token of `node` don't count, since
/// replacing `node` keeps them.
fn has_comment_outside(node: &JsLogicalExpression, kept_ranges: &[TextRange]) -> bool {
    if !node.syntax().has_comments_descendants() {
        return false;
    }
    let node_range = node.range();
    node.syntax()
        .descendants_tokens(Direction::Next)
        .flat_map(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
        })
        .filter(|piece| piece.is_comments())
        .any(|piece| {
            let range = piece.text_range();
            node_range.contains_range(range)
                && !kept_ranges
                    .iter()
                    .any(|kept_range| kept_range.contains_range(range))
        })
}
