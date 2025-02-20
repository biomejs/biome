use crate::react::{jsx_member_name_is_react_fragment, jsx_reference_identifier_is_fragment};
use crate::services::semantic::Semantic;
use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{
    js_string_literal_expression, jsx_expression_attribute_value, jsx_expression_child, jsx_string,
    jsx_string_literal, jsx_tag_expression, token, JsxExpressionChildBuilder,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsxChild, AnyJsxElementName, AnyJsxTag, JsLanguage, JsLogicalExpression,
    JsParenthesizedExpression, JsSyntaxKind, JsxAttributeInitializerClause, JsxChildList,
    JsxElement, JsxExpressionAttributeValue, JsxExpressionChild, JsxFragment, JsxTagExpression,
    JsxText, T,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutation, BatchMutationExt};

declare_lint_rule! {
    /// Disallow unnecessary fragments
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <>
    /// foo
    /// </>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <React.Fragment>
    /// foo
    /// </React.Fragment>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <>
    ///     <>foo</>
    ///     <SomeComponent />
    /// </>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <></>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <Foo />
    ///     <Bar />
    /// </>
    /// ```
    ///
    /// ```jsx
    /// <>foo {bar}</>
    /// ```
    ///
    pub NoUselessFragments {
        version: "1.0.0",
        name: "noUselessFragments",
        language: "jsx",
        sources: &[RuleSource::EslintReact("jsx-no-useless-fragment")],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug)]
pub enum NoUselessFragmentsState {
    Empty,
    Child(AnyJsxChild),
    Children(JsxChildList),
}

declare_node_union! {
    pub NoUselessFragmentsQuery = JsxFragment | JsxElement
}

impl NoUselessFragmentsQuery {
    fn replace_node(&self, mutation: &mut BatchMutation<JsLanguage>, new_node: AnyJsxChild) {
        match self {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let old_node = AnyJsxChild::JsxFragment(fragment.clone());
                mutation.replace_node(old_node, new_node);
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let old_node = AnyJsxChild::JsxElement(element.clone());
                mutation.replace_node(old_node, new_node);
            }
        }
    }

    fn remove_node_from_list(&self, mutation: &mut BatchMutation<JsLanguage>) {
        match self {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let old_node = AnyJsxChild::JsxFragment(fragment.clone());
                mutation.remove_node(old_node);
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let old_node = AnyJsxChild::JsxElement(element.clone());
                mutation.remove_node(old_node);
            }
        }
    }

    fn children(&self) -> JsxChildList {
        match self {
            NoUselessFragmentsQuery::JsxFragment(element) => element.children(),
            NoUselessFragmentsQuery::JsxElement(element) => element.children(),
        }
    }
}

impl Rule for NoUselessFragments {
    type Query = Semantic<NoUselessFragmentsQuery>;
    type State = NoUselessFragmentsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let mut in_jsx_attr_expr = false;
        let mut in_js_logical_expr = false;
        let mut in_jsx_expr = false;
        let mut in_jsx_list = false;
        match node {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let parents_where_fragments_must_be_preserved =
                    node.syntax().parent().is_some_and(|parent| {
                        match JsxTagExpression::try_cast(parent.clone()) {
                            Ok(parent) => parent
                                .syntax()
                                .parent()
                                .and_then(|parent| {
                                    if JsxExpressionAttributeValue::can_cast(parent.kind()) {
                                        in_jsx_attr_expr = true;
                                    }
                                    if JsLogicalExpression::can_cast(parent.kind()) {
                                        in_js_logical_expr = true;
                                    }
                                    if JsxExpressionChild::can_cast(parent.kind()) {
                                        in_jsx_expr = true;
                                    }
                                    match JsParenthesizedExpression::try_cast(parent) {
                                        Ok(parenthesized_expression) => {
                                            parenthesized_expression.syntax().parent()
                                        }
                                        Err(parent) => Some(parent),
                                    }
                                })
                                .is_some_and(|parent| {
                                    matches!(
                                        parent.kind(),
                                        JsSyntaxKind::JS_RETURN_STATEMENT
                                            | JsSyntaxKind::JS_INITIALIZER_CLAUSE
                                            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                                            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                                            | JsSyntaxKind::JS_FUNCTION_DECLARATION
                                            | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER
                                    )
                                }),
                            Err(_) => {
                                if JsxChildList::try_cast(parent.clone()).is_ok() {
                                    in_jsx_list = true;
                                    false
                                } else {
                                    JsxAttributeInitializerClause::try_cast(parent.clone()).is_ok()
                                }
                            }
                        }
                    });

                let child_list = fragment.children();

                if !parents_where_fragments_must_be_preserved {
                    let mut significant_children = 0;
                    let mut first_significant_child = None;
                    let mut children_where_fragments_must_preserved = false;

                    for child in child_list.iter() {
                        match child.syntax().kind() {
                            JsSyntaxKind::JSX_EXPRESSION_CHILD => {
                                if !in_js_logical_expr {
                                    significant_children += 1;
                                    if first_significant_child.is_none() {
                                        first_significant_child = Some(child);
                                    }
                                } else {
                                    children_where_fragments_must_preserved = true;
                                }
                            }
                            JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT
                            | JsSyntaxKind::JSX_ELEMENT
                            | JsSyntaxKind::JSX_FRAGMENT => {
                                significant_children += 1;
                                if first_significant_child.is_none() {
                                    first_significant_child = Some(child);
                                }
                            }
                            JsSyntaxKind::JSX_TEXT => {
                                // We need to whitespaces and newlines from the original string.
                                // Since in the JSX newlines aren't trivia, we require to allocate a string to trim from those characters.
                                let original_text = child.to_trimmed_string();
                                let child_text = original_text.trim();

                                if (in_jsx_expr || in_js_logical_expr)
                                    && contains_html_character_references(child_text)
                                {
                                    children_where_fragments_must_preserved = true;
                                    break;
                                }

                                if !child_text.is_empty() {
                                    significant_children += 1;
                                    if first_significant_child.is_none() {
                                        first_significant_child = Some(child);
                                    }
                                }
                            }
                            _ => {}
                        }
                        if significant_children > 1 || children_where_fragments_must_preserved {
                            break;
                        }
                    }

                    if children_where_fragments_must_preserved {
                        return None;
                    }

                    match significant_children {
                        0 => Some(NoUselessFragmentsState::Empty),
                        1 => {
                            if let Some(first) = first_significant_child {
                                if JsxText::can_cast(first.syntax().kind()) && in_jsx_attr_expr {
                                    None
                                } else {
                                    Some(NoUselessFragmentsState::Child(first))
                                }
                            } else {
                                None
                            }
                        }
                        _ => in_jsx_list.then_some(NoUselessFragmentsState::Children(child_list)),
                    }
                } else {
                    None
                }
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                let name = opening_element.name().ok()?;

                let is_valid_react_fragment = match name {
                    AnyJsxElementName::JsxMemberName(member_name) => {
                        jsx_member_name_is_react_fragment(&member_name, model)?
                    }
                    AnyJsxElementName::JsxReferenceIdentifier(identifier) => {
                        jsx_reference_identifier_is_fragment(&identifier, model)?
                    }
                    AnyJsxElementName::JsxName(_) | AnyJsxElementName::JsxNamespaceName(_) => false,
                };

                if is_valid_react_fragment {
                    let child_list = element.children();
                    // The `Fragment` component supports only the "key" prop and react emits a warning for not supported props.
                    // We assume that the user knows - and fixed - that and only care about the prop that is actually supported.
                    let attribute_key =
                        opening_element
                            .attributes()
                            .into_iter()
                            .find_map(|attribute| {
                                let attribute = attribute.as_jsx_attribute()?;
                                let attribute_name = attribute.name().ok()?;
                                let attribute_name = attribute_name.as_jsx_name()?;

                                if attribute_name.value_token().ok()?.text_trimmed() == "key" {
                                    Some(())
                                } else {
                                    None
                                }
                            });
                    if attribute_key.is_none() {
                        return match child_list.first() {
                            Some(first) if child_list.len() == 1 => {
                                Some(NoUselessFragmentsState::Child(first))
                            }
                            None => Some(NoUselessFragmentsState::Empty),
                            _ => None,
                        };
                    }
                }

                None
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let is_in_jsx_attr = node
            .syntax()
            .grand_parent()
            .is_some_and(|parent| JsxExpressionAttributeValue::can_cast(parent.kind()));

        let is_in_list = node
            .syntax()
            .parent()
            .is_some_and(|parent| JsxChildList::can_cast(parent.kind()));
        if is_in_list {
            match state {
                NoUselessFragmentsState::Child(child) => {
                    node.replace_node(&mut mutation, child.clone());
                }
                NoUselessFragmentsState::Children(children) => {
                    if let Some(old_children) = node
                        .syntax()
                        .parent()
                        .and_then(|parent| JsxChildList::cast(parent.clone()))
                    {
                        mutation.replace_node(old_children, children.clone());
                    }
                }
                _ => {
                    node.remove_node_from_list(&mut mutation);
                }
            }
        } else if let Some(parent) = node.parent::<JsxTagExpression>() {
            let parent = match parent.parent::<JsxExpressionAttributeValue>() {
                Some(grand_parent) => grand_parent.into_syntax(),
                None => parent.into_syntax(),
            };
            let child = node
                .children()
                .iter()
                .find(|child| match child.syntax().kind() {
                    JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT
                    | JsSyntaxKind::JSX_ELEMENT
                    | JsSyntaxKind::JSX_EXPRESSION_CHILD
                    | JsSyntaxKind::JSX_FRAGMENT => true,
                    JsSyntaxKind::JSX_TEXT => !child
                        .syntax()
                        .text_with_trivia()
                        .to_string()
                        .trim()
                        .is_empty(),
                    _ => false,
                });

            if let Some(child) = child {
                let new_node = match child {
                    AnyJsxChild::JsxElement(node) => {
                        let jsx_tag_expr = jsx_tag_expression(AnyJsxTag::JsxElement(node));
                        if is_in_jsx_attr {
                            let jsx_expr_attr_value = jsx_expression_attribute_value(
                                token(T!['{']),
                                AnyJsExpression::JsxTagExpression(jsx_tag_expr.clone()),
                                token(T!['}']),
                            );
                            Some(jsx_expr_attr_value.into_syntax())
                        } else {
                            Some(jsx_tag_expr.into_syntax())
                        }
                    }
                    AnyJsxChild::JsxFragment(node) => {
                        Some(jsx_tag_expression(AnyJsxTag::JsxFragment(node)).into_syntax())
                    }
                    AnyJsxChild::JsxSelfClosingElement(node) => Some(
                        jsx_tag_expression(AnyJsxTag::JsxSelfClosingElement(node)).into_syntax(),
                    ),
                    AnyJsxChild::JsxText(text) => {
                        let new_value = text.value_token().ok()?.token_text();
                        let new_value = new_value.trim();
                        if parent.kind() == JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE {
                            Some(jsx_string(jsx_string_literal(new_value)).into_syntax())
                        } else {
                            Some(
                                js_string_literal_expression(jsx_string_literal(new_value))
                                    .into_syntax(),
                            )
                        }
                    }
                    AnyJsxChild::JsxExpressionChild(child) => {
                        if is_in_jsx_attr
                            || !JsxTagExpression::can_cast(node.syntax().parent()?.kind())
                        {
                            child.expression().map(|expression| {
                                let jsx_expr_child =
                                    jsx_expression_child(token(T!['{']), token(T!['}']));
                                JsxExpressionChildBuilder::with_expression(
                                    jsx_expr_child,
                                    expression,
                                )
                                .build()
                                .into_syntax()
                            })
                        } else {
                            child
                                .expression()
                                .map(|expression| expression.into_syntax())
                        }
                    }

                    // can't apply a code action because it will create invalid syntax
                    // for example `<>{...foo}</>` would become `{...foo}` which would produce
                    // a syntax error
                    AnyJsxChild::JsxSpreadChild(_) => return None,
                };
                if let Some(new_node) = new_node {
                    mutation.replace_element(parent.into(), new_node.into());
                } else {
                    mutation.remove_element(parent.into());
                }
            } else {
                // can't apply a code action when there is no children because it will create invalid syntax
                // for example `<div x-some-prop={<></>}` would become `<div x-some-prop=` which would produce
                // a syntax error
                return None;
            }
        } else if let Some(_parent) = node.parent::<JsxAttributeInitializerClause>() {
            return None;
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the Fragment" }.to_owned(),
            mutation,
        ))
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "This fragment is unnecessary."
            },
        ).note(markup! {
            "A fragment is redundant if it contains only one child, or if it is the child of a html element, and is not a keyed "<Hyperlink href="https://legacy.reactjs.org/docs/fragments.html#keyed-fragments">"fragment"</Hyperlink>"."
        }))
    }
}

fn contains_html_character_references(s: &str) -> bool {
    let and = s.find('&');
    let semi = s.find(';');
    matches!((and, semi), (Some(and), Some(semi)) if and < semi)
}
