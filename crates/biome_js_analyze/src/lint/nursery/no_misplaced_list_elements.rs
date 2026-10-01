use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsConditionalExpression, JsSyntaxKind, JsSyntaxNode,
    JsxElement, jsx_ext::AnyJsxElement,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_misplaced_list_elements::NoMisplacedListElementsOptions;

declare_lint_rule! {
    /// Require `<li>` elements with an HTML element parent to be children of `<ul>`, `<ol>`, or `<menu>`.
    ///
    /// List items need a list container to define their relationship to the other items.
    /// Placing a list item outside a list container produces invalid HTML.
    ///
    /// The parent is the element the item is rendered into. Fragments, conditional and
    /// logical expressions, arrays, and callbacks passed to `map`, `flatMap`, or `Array.from`
    /// are looked through to find it.
    ///
    /// List items without an HTML element parent are ignored, including standalone items,
    /// items returned from components, items stored in variables, and component children.
    /// Their container may be supplied where they are rendered.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div><li>Item</li></div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <ul><div><li /></div></ul>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>{items.map((item) => <li key={item}>{item}</li>)}</div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <ul><li>Item</li></ul>
    ///     <ol><li>Item</li></ol>
    ///     <menu><li /></menu>
    /// </>
    /// ```
    ///
    /// ```jsx
    /// <ul>{items.map((item) => <li key={item}>{item}</li>)}</ul>
    /// ```
    ///
    /// ```jsx
    /// <li>Item rendered in a list elsewhere</li>
    /// ```
    ///
    /// ```jsx
    /// const item = <li>Item rendered in a list elsewhere</li>;
    /// ```
    ///
    /// ```jsx
    /// function Item() {
    ///     return <li>Item rendered in a list elsewhere</li>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// <List><li>Component child</li></List>
    /// ```
    ///
    pub NoMisplacedListElements {
        version: "2.5.15",
        name: "noMisplacedListElements",
        language: "jsx",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::HtmlEslint("require-li-container").inspired()],
    }
}

const LIST_CONTAINERS: [&str; 3] = ["ul", "ol", "menu"];

impl Rule for NoMisplacedListElements {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoMisplacedListElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        if name.text_trimmed() != "li" {
            return None;
        }

        let element = match node {
            AnyJsxElement::JsxOpeningElement(opening) => opening.syntax().parent()?,
            AnyJsxElement::JsxSelfClosingElement(element) => element.syntax().clone(),
        };
        let parent = rendered_parent(element)?;
        let name = parent
            .opening_element()
            .ok()?
            .name()
            .ok()?
            .as_jsx_name()?
            .value_token()
            .ok()?;
        if LIST_CONTAINERS.contains(&name.text_trimmed()) {
            return None;
        }

        Some(node.syntax().text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "This "<Emphasis>"<li>"</Emphasis>" element is outside a list container."
                },
            )
            .note("List items need a list container to form a valid HTML list.")
            .note(markup! {
                "Make this element a direct child of "<Emphasis>"<ul>"</Emphasis>", "<Emphasis>"<ol>"</Emphasis>", or "<Emphasis>"<menu>"</Emphasis>"."
            }),
        )
    }
}

/// Returns the JSX element that `element` is rendered into.
///
/// Returns `None` when the element's value leaves the JSX tree, for example when it is
/// returned from a function declaration, assigned to a variable, or passed as an attribute
/// value or a function argument.
fn rendered_parent(element: JsSyntaxNode) -> Option<JsxElement> {
    let mut current = element;
    loop {
        let parent = current.parent()?;
        match parent.kind() {
            JsSyntaxKind::JSX_ELEMENT => return JsxElement::cast(parent),
            JsSyntaxKind::JSX_CHILD_LIST
            | JsSyntaxKind::JSX_FRAGMENT
            | JsSyntaxKind::JSX_EXPRESSION_CHILD
            | JsSyntaxKind::JSX_TAG_EXPRESSION
            | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
            | JsSyntaxKind::JS_LOGICAL_EXPRESSION
            | JsSyntaxKind::JS_ARRAY_ELEMENT_LIST
            | JsSyntaxKind::JS_ARRAY_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_SATISFIES_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => current = parent,
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                let conditional = JsConditionalExpression::cast_ref(&parent)?;
                // The test of a conditional expression is not rendered.
                if conditional.test().ok()?.syntax() == &current {
                    return None;
                }
                current = parent;
            }
            JsSyntaxKind::JS_RETURN_STATEMENT => {
                // Continue from the function that returns the element. Whether its return
                // value is rendered depends on where the function itself is used.
                current = parent
                    .ancestors()
                    .find(|node| node.kind() == JsSyntaxKind::JS_FUNCTION_BODY)?
                    .parent()?;
            }
            JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                let call = parent.grand_parent().and_then(JsCallExpression::cast)?;
                if !matches!(
                    current.kind(),
                    JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                        | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                ) || !returns_callback_results(&call)
                {
                    return None;
                }
                current = call.into_syntax();
            }
            _ => return None,
        }
    }
}

/// Returns `true` if `call` returns the values produced by its callback argument, as
/// `items.map(callback)`, `items.flatMap(callback)`, and `Array.from(items, callback)` do.
fn returns_callback_results(call: &JsCallExpression) -> bool {
    let Some(callee) = call
        .callee()
        .ok()
        .and_then(|callee| callee.as_js_static_member_expression().cloned())
    else {
        return false;
    };
    let Some(member) = callee
        .member()
        .ok()
        .and_then(|member| member.as_js_name()?.value_token().ok())
    else {
        return false;
    };
    match member.text_trimmed() {
        "map" | "flatMap" => true,
        "from" => callee.object().is_ok_and(|object| {
            matches!(object, AnyJsExpression::JsIdentifierExpression(identifier)
                if identifier.name().is_ok_and(|name| name.has_name("Array")))
        }),
        _ => false,
    }
}
