use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
    options::PreferredQuote,
};
use biome_console::markup;
use biome_js_factory::make::{self, js_string_literal_expression, js_string_literal_single_quotes};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsCallExpression, JsComputedMemberExpression,
    JsStaticMemberExpression, JsSyntaxNode, static_value::StaticValue,
};
use biome_rowan::{AstNode, BatchMutationExt, Direction, declare_node_union};
use biome_rule_options::use_better_dom_traversing::UseBetterDomTraversingOptions;

/// `Number.MAX_SAFE_INTEGER` (`2^53 - 1`). Index literals above this are not safe integers.
const JS_MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

declare_lint_rule! {
    /// Prefer modern DOM traversal APIs over positional indexes and chained walks.
    ///
    /// Named first-child accessors, `querySelector()`, and `closest()` describe intent more clearly
    /// than `childNodes[0]`, `children[n]`, and repeated `.parentElement` access.
    /// Merging chained `.querySelector()` calls with static selectors has the same benefit.
    ///
    /// Fixes are unsafe because the replacement is not always equivalent:
    ///
    /// - `.childNodes[0]` is `undefined` when empty; `.firstChild` is `null`
    /// - `.closest()` looks for any matching ancestor, not an exact number of `.parentElement` hops
    /// - chained `.querySelector()` calls search inside the first match, while a combined selector
    ///   searches from the original node
    ///
    /// `props.children` is ignored because that is component data, not DOM traversal.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// element.childNodes[0];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.children[0];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.children[2];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.parentElement.parentElement;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.querySelector("a").querySelector("b");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// element.firstChild;
    /// element.firstElementChild;
    /// element.querySelector("li");
    /// element.closest("form");
    /// const child = props.children[0];
    /// ```
    ///
    pub UseBetterDomTraversing {
        version: "next",
        name: "useBetterDomTraversing",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("better-dom-traversing").inspired()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyUseBetterDomTraversingQuery =
        JsComputedMemberExpression |
        JsStaticMemberExpression |
        JsCallExpression
}

/// Pattern that triggered the rule.
pub enum UseBetterDomTraversingState {
    /// `node.childNodes[0]` → `.firstChild`
    FirstChild,
    /// `node.children[0]` → `.firstElementChild`
    FirstElementChild,
    /// `node.children[n]` for `n > 0` (diagnostic only)
    PositionalChildren,
    /// `node.parentElement.parentElement` (diagnostic only)
    Closest,
    /// Chained `.querySelector()` calls with static selectors
    MergeQuerySelector { can_fix: bool },
}

impl Rule for UseBetterDomTraversing {
    type Query = Ast<AnyUseBetterDomTraversingQuery>;
    type State = UseBetterDomTraversingState;
    type Signals = Option<Self::State>;
    type Options = UseBetterDomTraversingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyUseBetterDomTraversingQuery::JsComputedMemberExpression(node) => {
                indexed_collection_state(node)
            }
            AnyUseBetterDomTraversingQuery::JsStaticMemberExpression(node) => {
                parent_element_chain_state(node)
            }
            AnyUseBetterDomTraversingQuery::JsCallExpression(node) => {
                merge_query_selector_state(node)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().range();
        Some(match state {
            UseBetterDomTraversingState::FirstChild => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This indexes "<Emphasis>".childNodes"</Emphasis>" to get the first child."
                },
            )
            .note(markup! {
                "A positional index is harder to read than a named first-child accessor."
            }),
            UseBetterDomTraversingState::FirstElementChild => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This indexes "<Emphasis>".children"</Emphasis>" to get the first element child."
                },
            )
            .note(markup! {
                "A positional index is harder to read than a named first-child accessor."
            }),
            UseBetterDomTraversingState::PositionalChildren => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This uses a positional index on "<Emphasis>".children"</Emphasis>"."
                },
            )
            .note(markup! {
                "A CSS selector describes which child you want without relying on a numeric index."
            })
            .note(markup! {
                "Replace this with "<Emphasis>".querySelector()"</Emphasis>" and a selector for the child."
            }),
            UseBetterDomTraversingState::Closest => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This walks ancestors by chaining "<Emphasis>".parentElement"</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>".closest()"</Emphasis>" looks up a matching ancestor without repeating "<Emphasis>".parentElement"</Emphasis>"."
            })
            .note(markup! {
                "Replace this chain with "<Emphasis>".closest()"</Emphasis>" and a selector for the ancestor."
            }),
            UseBetterDomTraversingState::MergeQuerySelector { can_fix } => {
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "These "<Emphasis>".querySelector()"</Emphasis>" calls are chained."
                    },
                )
                .note(markup! {
                    "One "<Emphasis>".querySelector()"</Emphasis>" call with a combined selector is easier to read."
                });
                if !can_fix {
                    diag = diag.note(markup! {
                        "Merge the selectors."
                    });
                }
                diag
            }
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        match (ctx.query(), state) {
            (
                AnyUseBetterDomTraversingQuery::JsComputedMemberExpression(node),
                UseBetterDomTraversingState::FirstChild
                | UseBetterDomTraversingState::FirstElementChild,
            ) => {
                if has_comments_inside(node.syntax()) {
                    return None;
                }
                let replacement_name = match state {
                    UseBetterDomTraversingState::FirstChild => "firstChild",
                    _ => "firstElementChild",
                };
                let replacement = first_child_replacement(node, replacement_name)?;
                mutation.replace_node(
                    AnyJsExpression::JsComputedMemberExpression(node.clone()),
                    replacement,
                );
                let message = match state {
                    UseBetterDomTraversingState::FirstChild => {
                        markup! { "Use "<Emphasis>".firstChild"</Emphasis>" instead." }.to_owned()
                    }
                    _ => markup! { "Use "<Emphasis>".firstElementChild"</Emphasis>" instead." }
                        .to_owned(),
                };
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    message,
                    mutation,
                ))
            }
            (
                AnyUseBetterDomTraversingQuery::JsCallExpression(node),
                UseBetterDomTraversingState::MergeQuerySelector { can_fix: true },
            ) => {
                if has_comments_inside(node.syntax()) {
                    return None;
                }
                let (root, selectors) = query_selector_chain(node)?;
                let merged_selector = if is_document_object(&root) {
                    selectors.join(" ")
                } else {
                    format!(":scope {}", selectors.join(" "))
                };
                let argument = first_and_only_argument(node)?;
                let callee = node.callee().ok()?.omit_parentheses();
                let member = callee.as_js_static_member_expression()?;
                let inner_object = member.object().ok()?;
                mutation.replace_node(
                    argument,
                    make_string_literal_expression(&merged_selector, ctx.preferred_quote()),
                );
                mutation.replace_node(inner_object, root.clone());
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Merge the "<Emphasis>".querySelector()"</Emphasis>" calls." }
                        .to_owned(),
                    mutation,
                ))
            }
            _ => None,
        }
    }
}

fn indexed_collection_state(
    node: &JsComputedMemberExpression,
) -> Option<UseBetterDomTraversingState> {
    let index = numeric_index(node)?;
    let object = node.object().ok()?;
    let collection = static_member_named(&object, "childNodes")
        .or_else(|| static_member_named(&object, "children"))?;

    let collection_name = collection.member().ok()?.as_js_name()?.value_token().ok()?;
    let collection_name = collection_name.text_trimmed();

    if is_definitely_not_dom_node(&collection.object().ok()?.omit_parentheses()) {
        return None;
    }
    if collection_name == "children" && is_props_children(&collection) {
        return None;
    }
    if is_nested_indexed_children(node) {
        return None;
    }

    match (collection_name, index == 0.0) {
        ("childNodes", true) => Some(UseBetterDomTraversingState::FirstChild),
        ("children", true) => Some(UseBetterDomTraversingState::FirstElementChild),
        ("children", false) => Some(UseBetterDomTraversingState::PositionalChildren),
        _ => None,
    }
}

fn parent_element_chain_state(
    node: &JsStaticMemberExpression,
) -> Option<UseBetterDomTraversingState> {
    if !is_parent_element_member(node) {
        return None;
    }
    let object = node.object().ok()?.omit_parentheses();
    let inner = object.as_js_static_member_expression()?;
    if !is_parent_element_member(inner) {
        return None;
    }
    if !is_outermost_parent_element(node) {
        return None;
    }
    if is_definitely_not_dom_node(&parent_element_chain_root(AnyJsExpression::from(
        node.clone(),
    ))) {
        return None;
    }
    Some(UseBetterDomTraversingState::Closest)
}

fn merge_query_selector_state(node: &JsCallExpression) -> Option<UseBetterDomTraversingState> {
    if !is_query_selector_call(node)
        || is_followed_by_static_query_selector(node)
        || is_inside_optional_chain(node.syntax())
    {
        return None;
    }
    let (_root, selectors) = query_selector_chain(node)?;
    let can_fix = selectors
        .iter()
        .all(|selector| can_auto_merge_selector(selector));
    Some(UseBetterDomTraversingState::MergeQuerySelector { can_fix })
}

/// Comma lists, CSS escapes, and `:scope` (ASCII case-insensitive) are
/// reported but not rewritten.
fn can_auto_merge_selector(selector: &str) -> bool {
    !selector.contains(',')
        && !selector.contains('\\')
        && !selector.to_ascii_lowercase().contains(":scope")
}

/// Returns the numeric index when `node` is a non-optional computed access
/// with a safe, non-negative integer literal.
fn numeric_index(node: &JsComputedMemberExpression) -> Option<f64> {
    if node.is_optional() {
        return None;
    }
    let member = node.member().ok()?.omit_parentheses();
    let number = member
        .as_any_js_literal_expression()?
        .as_js_number_literal_expression()?;
    let value = number.as_number()?;
    if value.is_finite() && value.fract() == 0.0 && (0.0..=JS_MAX_SAFE_INTEGER).contains(&value) {
        Some(value)
    } else {
        None
    }
}

/// Returns the static member `expr.name` when it is a non-optional, non-computed access.
fn static_member_named(expr: &AnyJsExpression, name: &str) -> Option<JsStaticMemberExpression> {
    let expr = expr.clone().omit_parentheses();
    let member = expr.as_js_static_member_expression()?.clone();
    if member.is_optional() {
        return None;
    }
    let member_name = member.member().ok()?.as_js_name()?.value_token().ok()?;
    if member_name.text_trimmed() == name {
        Some(member)
    } else {
        None
    }
}

fn is_parent_element_member(member: &JsStaticMemberExpression) -> bool {
    !member.is_optional()
        && member
            .member()
            .ok()
            .and_then(|name| name.as_js_name().cloned())
            .and_then(|name| name.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == "parentElement")
}

fn is_outermost_parent_element(member: &JsStaticMemberExpression) -> bool {
    let Some(parent) = member.syntax().parent() else {
        return true;
    };
    let Some(outer) = JsStaticMemberExpression::cast(parent) else {
        return true;
    };
    if !is_parent_element_member(&outer) {
        return true;
    }
    outer
        .object()
        .ok()
        .is_none_or(|object| object.syntax() != member.syntax())
}

fn parent_element_chain_root(mut expr: AnyJsExpression) -> AnyJsExpression {
    loop {
        let Some(member) = expr
            .clone()
            .omit_parentheses()
            .as_js_static_member_expression()
            .cloned()
            .filter(is_parent_element_member)
        else {
            return expr.omit_parentheses();
        };
        match member.object() {
            Ok(object) => expr = object,
            Err(_) => return expr.omit_parentheses(),
        }
    }
}

/// `element.children[1].children[2]` and `element.childNodes[0].childNodes[0]`
/// should only report the outer access.
fn is_nested_indexed_children(node: &JsComputedMemberExpression) -> bool {
    let Some(parent) = node.syntax().parent() else {
        return false;
    };
    let Some(static_member) = JsStaticMemberExpression::cast(parent) else {
        return false;
    };
    if static_member
        .object()
        .ok()
        .is_none_or(|object| object.syntax() != node.syntax())
        || static_member.is_optional()
        || static_member
            .member()
            .ok()
            .and_then(|name| name.as_js_name().cloned())
            .and_then(|name| name.value_token().ok())
            .is_none_or(|token| {
                let name = token.text_trimmed();
                name != "children" && name != "childNodes"
            })
    {
        return false;
    }
    let Some(grand) = static_member.syntax().parent() else {
        return false;
    };
    JsComputedMemberExpression::cast(grand)
        .and_then(|outer| numeric_index(&outer))
        .is_some()
}

fn is_props_children(collection: &JsStaticMemberExpression) -> bool {
    let Ok(object) = collection.object() else {
        return false;
    };
    let object = object.omit_parentheses();
    if object
        .as_js_identifier_expression()
        .and_then(|id| id.name().ok())
        .is_some_and(|name| name.has_name("props"))
    {
        return true;
    }
    static_member_named(&object, "props").is_some()
}

fn is_query_selector_call(call: &JsCallExpression) -> bool {
    if call.is_optional() {
        return false;
    }
    let Ok(callee) = call.callee() else {
        return false;
    };
    static_member_named(&callee, "querySelector").is_some()
        && first_and_only_argument(call).is_some()
}

fn is_followed_by_static_query_selector(call: &JsCallExpression) -> bool {
    let Some(parent) = call.syntax().parent() else {
        return false;
    };
    let Some(member) = JsStaticMemberExpression::cast(parent) else {
        return false;
    };
    if member
        .object()
        .ok()
        .is_none_or(|object| object.syntax() != call.syntax())
        || member.is_optional()
        || member
            .member()
            .ok()
            .and_then(|name| name.as_js_name().cloned())
            .and_then(|name| name.value_token().ok())
            .is_none_or(|token| token.text_trimmed() != "querySelector")
    {
        return false;
    }
    let Some(grand) = member.syntax().parent() else {
        return false;
    };
    let Some(outer) = JsCallExpression::cast(grand) else {
        return false;
    };
    if outer.is_optional()
        || outer
            .callee()
            .ok()
            .is_none_or(|callee| callee.syntax() != member.syntax())
    {
        return false;
    }
    first_and_only_argument(&outer)
        .and_then(|argument| static_selector(&argument))
        .is_some()
}

fn query_selector_chain(call: &JsCallExpression) -> Option<(AnyJsExpression, Vec<String>)> {
    let mut current_expr = AnyJsExpression::JsCallExpression(call.clone());
    let mut raw_root = current_expr.clone();
    let mut selectors = Vec::new();

    loop {
        let AnyJsExpression::JsCallExpression(current) = current_expr.clone() else {
            break;
        };
        if !is_query_selector_call(&current) {
            break;
        }
        let argument = first_and_only_argument(&current)?;
        let Some(selector) = static_selector(&argument) else {
            break;
        };
        selectors.push(selector);
        let callee = current.callee().ok()?;
        let member = static_member_named(&callee, "querySelector")?;
        raw_root = member.object().ok()?;
        current_expr = raw_root.clone().omit_parentheses();
    }

    if selectors.len() < 2
        || is_definitely_not_dom_node(&current_expr)
        || expression_is_optional_chain(&current_expr)
    {
        return None;
    }
    selectors.reverse();
    Some((raw_root, selectors))
}

fn first_child_replacement(
    computed: &JsComputedMemberExpression,
    replacement_name: &str,
) -> Option<AnyJsExpression> {
    let object = computed.object().ok()?;
    let collection = object
        .clone()
        .omit_parentheses()
        .as_js_static_member_expression()
        .cloned()?;
    let receiver = AnyJsExpression::cast(collection.object().ok()?.into_syntax().detach())?;
    let new_member = make::js_static_member_expression(
        receiver,
        collection.operator_token().ok()?.detach(),
        make::js_name(make::ident(replacement_name)).into(),
    );
    if let Some(paren) = object.as_js_parenthesized_expression() {
        Some(AnyJsExpression::from(make::js_parenthesized_expression(
            paren.l_paren_token().ok()?.detach(),
            AnyJsExpression::from(new_member),
            paren.r_paren_token().ok()?.detach(),
        )))
    } else {
        Some(AnyJsExpression::from(new_member))
    }
}

/// ESTree wraps an entire optional chain in `ChainExpression`. Skip merge
/// diagnostics when this call is the object/callee of a `?.` access further up.
fn is_inside_optional_chain(node: &JsSyntaxNode) -> bool {
    for ancestor in node.ancestors().skip(1) {
        if let Some(member) = JsStaticMemberExpression::cast_ref(&ancestor) {
            if member.is_optional() {
                return true;
            }
        } else if let Some(member) = JsComputedMemberExpression::cast_ref(&ancestor) {
            if member.is_optional() {
                return true;
            }
        } else if let Some(call) = JsCallExpression::cast_ref(&ancestor) {
            if call.is_optional() {
                return true;
            }
        } else {
            break;
        }
    }
    false
}

fn first_and_only_argument(call: &JsCallExpression) -> Option<AnyJsExpression> {
    let mut args = call.arguments().ok()?.args().into_iter();
    let argument = args.next()?.ok()?.as_any_js_expression()?.clone();
    if args.next().is_none() {
        Some(argument)
    } else {
        None
    }
}

fn static_selector(expr: &AnyJsExpression) -> Option<String> {
    let expr = expr.clone().omit_parentheses();
    if let AnyJsExpression::JsTemplateExpression(template) = &expr
        && template.tag().is_some()
    {
        return None;
    }
    expr.as_static_value()?
        .as_string_constant()
        .map(str::to_string)
}

fn is_document_object(expr: &AnyJsExpression) -> bool {
    let expr = expr.clone().omit_parentheses();
    if expr
        .as_js_identifier_expression()
        .and_then(|id| id.name().ok())
        .is_some_and(|name| name.has_name("document"))
    {
        return true;
    }
    let Some(member) = static_member_named(&expr, "document") else {
        return false;
    };
    member
        .object()
        .ok()
        .and_then(|object| {
            object
                .omit_parentheses()
                .as_js_identifier_expression()
                .cloned()
        })
        .and_then(|id| id.name().ok())
        .is_some_and(|name| name.has_name("window") || name.has_name("globalThis"))
}

fn expression_is_optional_chain(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsCallExpression(call) => call.is_optional_chain(),
        AnyJsExpression::JsStaticMemberExpression(member) => member.is_optional_chain(),
        AnyJsExpression::JsComputedMemberExpression(member) => member.is_optional_chain(),
        _ => false,
    }
}

/// Comments on the first token's leading trivia (file-level comments) are ignored.
fn has_comments_inside(node: &JsSyntaxNode) -> bool {
    let mut tokens = node.descendants_tokens(Direction::Next);
    let Some(first) = tokens.next() else {
        return false;
    };
    first.has_trailing_comments()
        || tokens.any(|token| token.has_leading_comments() || token.has_trailing_comments())
}

/// Returns `true` when the receiver cannot be a DOM node.
///
/// Copied from the approach in `useDomQuerySelector`: only exclude syntax that
/// is guaranteed not to be a node, such as literals, arrays, objects, and functions.
fn is_definitely_not_dom_node(expr: &AnyJsExpression) -> bool {
    let expr = expr.clone().omit_parentheses();
    matches!(
        expr,
        AnyJsExpression::AnyJsLiteralExpression(_)
            | AnyJsExpression::JsArrayExpression(_)
            | AnyJsExpression::JsArrowFunctionExpression(_)
            | AnyJsExpression::JsClassExpression(_)
            | AnyJsExpression::JsFunctionExpression(_)
            | AnyJsExpression::JsObjectExpression(_)
            | AnyJsExpression::JsTemplateExpression(_)
    ) || expr
        .as_static_value()
        .is_some_and(|value| matches!(value, StaticValue::Undefined(_)))
}

fn make_string_literal_expression(value: &str, preferred_quote: PreferredQuote) -> AnyJsExpression {
    let use_double = if preferred_quote.is_double() {
        !value.contains('"')
    } else {
        value.contains('\'')
    };
    AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
        js_string_literal_expression(if use_double {
            make::js_string_literal(value)
        } else {
            js_string_literal_single_quotes(value)
        }),
    ))
}
