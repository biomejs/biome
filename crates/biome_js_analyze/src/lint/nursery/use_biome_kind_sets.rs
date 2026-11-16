use crate::{
    frameworks::biome_plugin::{KindCategory, resolve_constant},
    services::semantic::Semantic,
    utils::same_reference::is_same_reference,
};
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::{
    fmt::{Display, Formatter},
    markup,
};
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, JsBinaryOperator, JsCallExpression,
    JsLogicalExpression, JsLogicalOperator, JsSyntaxKind, static_value::StaticValue,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::use_biome_kind_sets::UseBiomeKindSetsOptions;

declare_lint_rule! {
    /// Require kind sets instead of comparing kind names one by one in Biome JS plugins.
    ///
    /// In a [Biome JS plugin](https://biomejs.dev/linter/plugins/), every node and token has
    /// a `kind`, which is a string such as `"JS_CALL_EXPRESSION"` or `"IDENT"`. To check whether
    /// a node has one of several kinds, plugins can create a kind set with `nodeKinds()`, or
    /// `tokenKinds()` for tokens, from `@biomejs/runtime/plugin`, and call its `has()` method.
    ///
    /// A kind set checks the kind of a node or token directly, without comparing strings, so it's
    /// faster. In TypeScript, a `has()` check also tells TypeScript which kinds of node the value
    /// can be, just as the comparisons do.
    ///
    /// This rule reports checks of the same node or token against two or more kind names:
    ///
    /// - comparisons joined with `||`, such as `node.kind === "A" || node.kind === "B"`;
    /// - negated comparisons joined with `&&`, such as `node.kind !== "A" && node.kind !== "B"`;
    /// - `includes()` calls on an array of kind names, such as `["A", "B"].includes(node.kind)`;
    /// - `has()` calls on a `Set` of kind names, such as `new Set(["A", "B"]).has(node.kind)`.
    ///
    /// The kind names, the array, and the `Set` can also be stored in `const` variables, such as
    /// `const CALL = "JS_CALL_EXPRESSION"`.
    ///
    /// The rule only reports a check when every string is a node kind, or every string is a
    /// token kind. Other objects in the plugin API also have a `kind` property, such as
    /// `context.sourceType.language`. Their values aren't syntax kinds, so the rule ignores
    /// them.
    ///
    /// Create each kind set once, in a constant at the top level of the plugin, and reuse it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (node.kind === "JS_CALL_EXPRESSION" || node.kind === "JS_NEW_EXPRESSION") {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (token.kind !== "EQ2" && token.kind !== "NEQ") {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const CALLS = ["JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION"];
    /// if (CALLS.includes(node.kind)) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const CALLS = new Set(["JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION"]);
    /// if (CALLS.has(node.kind)) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { nodeKinds } from "@biomejs/runtime/plugin";
    ///
    /// const CALLS = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
    /// if (CALLS.has(node)) {}
    ///
    /// if (node.kind === "JS_CALL_EXPRESSION") {}
    ///
    /// const language = context.sourceType.language;
    /// if (language.kind === "javascript" || language.kind === "typescript") {}
    /// ```
    ///
    /// ## See Also
    ///
    /// - To avoid creating the same kind set again every time a function runs, see [`useBiomeTopLevelTypes`](https://biomejs.dev/linter/rules/use-biome-top-level-types/).
    ///
    pub UseBiomeKindSets {
        version: "next",
        name: "useBiomeKindSets",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::BiomePlugin],
    }
}

declare_node_union! {
    pub AnyKindCheck = JsLogicalExpression | JsCallExpression
}

/// A check of whether a node or token has one of several kinds.
pub struct KindCheck {
    /// The node or token whose `kind` is checked, such as `node` in `node.kind`.
    element: AnyJsExpression,
    category: KindCategory,
    /// The range of the comparisons, or of the `includes()` or `has()` call.
    range: TextRange,
    /// Whether the check passes when the element has none of the kinds, as in
    /// `node.kind !== "A" && node.kind !== "B"`.
    negated: bool,
}

impl Rule for UseBiomeKindSets {
    type Query = Semantic<AnyKindCheck>;
    type State = KindCheck;
    type Signals = Box<[Self::State]>;
    type Options = UseBiomeKindSetsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyKindCheck::JsLogicalExpression(expression) => {
                comparison_checks(expression, ctx.model())
            }
            AnyKindCheck::JsCallExpression(call) => lookup_check(call, ctx.model())
                .into_iter()
                .collect(),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let element = SourceText(&state.element);
        let negation = if state.negated { "!" } else { "" };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This code checks the kind of "<Emphasis>{element}</Emphasis>" by comparing it with several strings."
                },
            )
            .note(markup! {
                "A kind set checks the kind directly, without comparing strings, so it's faster."
            })
            .note(markup! {
                "Create the set once in a top-level constant, such as "<Emphasis>"const KINDS = "{state.category}"(...)"</Emphasis>", then replace this check with "<Emphasis>{negation}"KINDS.has("{element}")"</Emphasis>"."
            }),
        )
    }
}

/// Returns the checks in a chain of comparisons such as `node.kind === "A" || node.kind === "B"`,
/// one for each node or token that the chain compares with two or more kind names.
fn comparison_checks(expression: &JsLogicalExpression, model: &SemanticModel) -> Box<[KindCheck]> {
    let Ok(operator) = expression.operator() else {
        return Box::default();
    };
    let negated = match operator {
        JsLogicalOperator::LogicalOr => false,
        JsLogicalOperator::LogicalAnd => true,
        JsLogicalOperator::NullishCoalescing => return Box::default(),
    };

    // Only the outermost expression of a chain is checked, so `a || b || c` is reported once
    // rather than once more for `a || b`.
    let is_nested = expression
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
        .and_then(JsLogicalExpression::cast)
        .is_some_and(|parent| parent.operator() == Ok(operator));
    if is_nested {
        return Box::default();
    }

    let mut groups: Vec<KindGroup> = Vec::new();
    let mut operands = vec![AnyJsExpression::JsLogicalExpression(expression.clone())];
    while let Some(operand) = operands.pop() {
        let operand = operand.omit_parentheses();
        if let AnyJsExpression::JsLogicalExpression(logical) = &operand
            && logical.operator() == Ok(operator)
        {
            // The right operand is pushed first so that operands are visited in source order.
            operands.extend(logical.right().ok());
            operands.extend(logical.left().ok());
            continue;
        }
        let Some((element, name)) = kind_comparison(&operand, negated, model) else {
            continue;
        };
        let group = groups.iter_mut().find(|group| {
            is_same_reference(group.element.clone(), element.clone()) == Some(true)
        });
        match group {
            Some(group) => group.add(name, operand.range()),
            None => groups.push(KindGroup::new(element, name, operand.range())),
        }
    }

    groups
        .into_iter()
        .filter_map(|group| group.into_check(negated))
        .collect()
}

/// Returns the node or token and the kind name of a comparison such as `node.kind === "A"`,
/// or `node.kind !== "A"` when `negated`. The string can be on either side, and can be stored
/// in a `const` variable.
fn kind_comparison(
    expression: &AnyJsExpression,
    negated: bool,
    model: &SemanticModel,
) -> Option<(AnyJsExpression, StaticValue)> {
    let AnyJsExpression::JsBinaryExpression(comparison) = expression else {
        return None;
    };
    let operator = comparison.operator().ok()?;
    let is_expected_operator = if negated {
        matches!(
            operator,
            JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality
        )
    } else {
        matches!(
            operator,
            JsBinaryOperator::StrictEquality | JsBinaryOperator::Equality
        )
    };
    if !is_expected_operator {
        return None;
    }
    let left = comparison.left().ok()?;
    let right = comparison.right().ok()?;
    match kind_reader(&left) {
        Some(element) => Some((element, kind_name(right, model)?)),
        None => Some((kind_reader(&right)?, kind_name(left, model)?)),
    }
}

/// Returns the check in a call such as `["A", "B"].includes(node.kind)` or
/// `new Set(["A", "B"]).has(node.kind)`. The array, the `Set`, and the kind names can be
/// stored in `const` variables.
fn lookup_check(call: &JsCallExpression, model: &SemanticModel) -> Option<KindCheck> {
    let AnyJsExpression::JsStaticMemberExpression(callee) = call.callee().ok()?.omit_parentheses()
    else {
        return None;
    };
    let method = callee.member().ok()?.as_js_name()?.value_token().ok()?;
    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }
    let AnyJsCallArgument::AnyJsExpression(argument) = arguments.first()?.ok()? else {
        return None;
    };
    let element = kind_reader(&argument)?;

    let collection = resolve_constant(callee.object().ok()?, model)?;
    let array = match method.text_trimmed() {
        "includes" => collection,
        "has" => set_source(&collection, model)?,
        _ => return None,
    };
    let AnyJsExpression::JsArrayExpression(array) = array else {
        return None;
    };

    let mut group: Option<KindGroup> = None;
    for item in array.elements() {
        let AnyJsArrayElement::AnyJsExpression(item) = item.ok()? else {
            return None;
        };
        let name = kind_name(item, model)?;
        match &mut group {
            Some(group) => group.add(name, call.range()),
            None => group = Some(KindGroup::new(element.clone(), name, call.range())),
        }
    }
    group?.into_check(false)
}

/// Returns the array that `new Set(array)` is created from, following `const` variables.
/// `Set` must be the global `Set`.
fn set_source(expression: &AnyJsExpression, model: &SemanticModel) -> Option<AnyJsExpression> {
    let AnyJsExpression::JsNewExpression(set) = expression else {
        return None;
    };
    let AnyJsExpression::JsIdentifierExpression(constructor) = set.callee().ok()?.omit_parentheses()
    else {
        return None;
    };
    let constructor = constructor.name().ok()?;
    if !constructor.has_name("Set") || model.binding(&constructor).is_some() {
        return None;
    }
    let arguments = set.arguments()?.args();
    if arguments.len() != 1 {
        return None;
    }
    let AnyJsCallArgument::AnyJsExpression(source) = arguments.first()?.ok()? else {
        return None;
    };
    resolve_constant(source, model)
}

/// Returns `node` when `expression` reads `node.kind` or `node?.kind`, without the
/// parentheses around `node`.
fn kind_reader(expression: &AnyJsExpression) -> Option<AnyJsExpression> {
    let AnyJsExpression::JsStaticMemberExpression(member) = expression.clone().omit_parentheses()
    else {
        return None;
    };
    let property = member.member().ok()?.as_js_name()?.value_token().ok()?;
    if property.text_trimmed() != "kind" {
        return None;
    }
    Some(member.object().ok()?.omit_parentheses())
}

/// Returns the value of a string, such as `"A"` or `` `A` ``, following `const` variables to
/// their initial value. Parentheses and TypeScript-only syntax such as `as const` are ignored.
fn kind_name(expression: AnyJsExpression, model: &SemanticModel) -> Option<StaticValue> {
    let value = resolve_constant(expression, model)?.as_static_value()?;
    value.as_string_constant().is_some().then_some(value)
}

/// The kind names that one node or token is checked against.
struct KindGroup {
    element: AnyJsExpression,
    /// The distinct kind names, in source order.
    names: Vec<StaticValue>,
    range: TextRange,
}

impl KindGroup {
    fn new(element: AnyJsExpression, name: StaticValue, range: TextRange) -> Self {
        Self {
            element,
            names: vec![name],
            range,
        }
    }

    fn add(&mut self, name: StaticValue, range: TextRange) {
        self.range = self.range.cover(range);
        if !self.names.iter().any(|existing| existing.text() == name.text()) {
            self.names.push(name);
        }
    }

    /// Returns the check for this group, or `None` when it has fewer than two distinct names,
    /// a name that isn't a kind, or both node kinds and token kinds.
    fn into_check(self, negated: bool) -> Option<KindCheck> {
        if self.names.len() < 2 {
            return None;
        }
        let mut categories = self
            .names
            .iter()
            .map(|name| KindCategory::from_kind_name(name.text()));
        let category = categories.next()??;
        categories
            .all(|other| other == Some(category))
            .then_some(KindCheck {
                element: self.element,
                category,
                range: self.range,
                negated,
            })
    }
}

/// Writes the source text of an expression, without leading or trailing trivia.
struct SourceText<'a>(&'a AnyJsExpression);

impl Display for SourceText<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        fmt.write_fmt(format_args!("{}", self.0.syntax().text_trimmed()))
    }
}
