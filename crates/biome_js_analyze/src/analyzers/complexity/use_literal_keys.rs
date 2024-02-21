use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{
    self, ident, js_literal_member_name, js_name, js_static_member_assignment,
    js_static_member_expression, token,
};
use biome_js_syntax::{
    AnyJsAssignment, AnyJsComputedMember, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsMemberExpression, AnyJsName, AnyJsObjectMemberName, JsComputedMemberName,
    JsLiteralMemberName, JsSyntaxKind, T,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, TextRange};
use biome_unicode_table::is_js_ident;

declare_rule! {
    /// Enforce the usage of a literal access to properties over computed property access.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// a.b["c"];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a.c[`d`]
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a.c[`d`] = "something"
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a = {
    /// 	['b']: d
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// a["c" + "d"];
    /// a[d.c];
    /// ```
    ///
    pub UseLiteralKeys {
        version: "1.0.0",
        name: "useLiteralKeys",
        source: RuleSource::Eslint("dot-notation"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseLiteralKeys {
    type Query = Ast<AnyJsMember>;
    type State = (TextRange, String);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let inner_expression = match node {
            AnyJsMember::AnyJsComputedMember(computed_member) => computed_member.member().ok()?,
            AnyJsMember::JsLiteralMemberName(member) => {
                if member.value().ok()?.kind() == JsSyntaxKind::JS_STRING_LITERAL {
                    let name = member.name().ok()?;
                    if is_js_ident(&name) {
                        return Some((member.range(), name.to_string()));
                    }
                }
                return None;
            }
            AnyJsMember::JsComputedMemberName(member) => member.expression().ok()?,
        };
        match inner_expression {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
            ) => {
                let value = string_literal.inner_string_text().ok()?;
                // A computed property `["something"]` can always be simplified to a string literal "something".
                if matches!(node, AnyJsMember::JsComputedMemberName(_)) || is_js_ident(&value) {
                    return Some((string_literal.range(), value.to_string()));
                }
            }
            AnyJsExpression::JsTemplateExpression(template_expression) => {
                let mut value = String::new();
                for element in template_expression.elements() {
                    let chunk = element.as_js_template_chunk_element()?;
                    value.push_str(chunk.template_chunk_token().ok()?.text_trimmed());
                }
                // A computed property ``[`something`]`` can always be simplified to a string literal "something".
                if matches!(node, AnyJsMember::JsComputedMemberName(_)) || is_js_ident(&value) {
                    return Some((template_expression.range(), value));
                }
            }
            _ => {}
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (range, _): &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "The computed expression can be simplified without the use of a string literal."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, (_, identifier): &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        match node {
            AnyJsMember::AnyJsComputedMember(node) => {
                let object = node.object().ok()?;
                let member = js_name(ident(identifier));
                let dot_token = node.optional_chain_token().unwrap_or_else(|| token(T![.]));

                match node {
                    AnyJsComputedMember::JsComputedMemberExpression(node) => {
                        let static_expression = js_static_member_expression(
                            object,
                            dot_token,
                            AnyJsName::JsName(member),
                        );
                        mutation.replace_node(
                            AnyJsMemberExpression::from(node.clone()),
                            static_expression.into(),
                        );
                    }
                    AnyJsComputedMember::JsComputedMemberAssignment(node) => {
                        let static_member = js_static_member_assignment(
                            object,
                            dot_token,
                            AnyJsName::JsName(member),
                        );
                        mutation.replace_node(
                            AnyJsAssignment::from(node.clone()),
                            static_member.into(),
                        );
                    }
                }
            }
            AnyJsMember::JsLiteralMemberName(node) => {
                mutation.replace_token(node.value().ok()?, make::ident(identifier));
            }
            AnyJsMember::JsComputedMemberName(member) => {
                let name_token = if is_js_ident(identifier) {
                    make::ident(identifier)
                } else {
                    make::js_string_literal(identifier)
                };
                let literal_member_name = js_literal_member_name(name_token);
                mutation.replace_node(
                    AnyJsObjectMemberName::from(member.clone()),
                    literal_member_name.into(),
                );
            }
        }
        Some(JsRuleAction {
            mutation,
            applicability: Applicability::MaybeIncorrect,
            category: ActionCategory::QuickFix,
            message: markup! {
                "Use a literal key instead."
            }
            .to_owned(),
        })
    }
}

declare_node_union! {
    pub AnyJsMember = AnyJsComputedMember | JsLiteralMemberName | JsComputedMemberName
}
