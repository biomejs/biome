use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsComputedMember, AnyJsMemberExpression, AnyJsName, AnyJsObjectMemberName,
    AnyTsEnumMemberName, JsComputedMemberName, JsSyntaxKind, JsSyntaxToken, T, inner_string_text,
    static_value::StaticValue,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxNodeOptionExt, TextRange, declare_node_union};
use biome_rule_options::use_literal_keys::UseLiteralKeysOptions;
use biome_unicode_table::is_js_ident;

declare_lint_rule! {
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
        language: "js",
        sources: &[
            RuleSource::Eslint("dot-notation").same(),
            RuleSource::Eslint("no-useless-computed-key").same(),
            RuleSource::EslintTypeScript("dot-notation").same(),
        ],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseLiteralKeys {
    type Query = Ast<AnyJsMember>;
    type State = (TextRange, JsSyntaxToken, bool);
    type Signals = Option<Self::State>;
    type Options = UseLiteralKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut is_computed_member_name = false;
        let inner_expression = match node {
            AnyJsMember::AnyJsComputedMember(computed_member) => computed_member.member().ok()?,
            AnyJsMember::JsComputedMemberName(member) => {
                is_computed_member_name = true;
                member.expression().ok()?
            }
        };
        let value = inner_expression.as_static_value()?;
        match value {
            StaticValue::Number(token) => {
                if is_computed_member_name {
                    return Some((inner_expression.range(), token, is_computed_member_name));
                }
            }
            StaticValue::String(token) => {
                let value = inner_string_text(&token);
                // `{["__proto__"]: null }` and `{"__proto__": null}`/`{"__proto__": null}`
                // have different semantic.
                // The first is a regular property.
                // The second is a special property that changes the object prototype.
                // See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto
                if is_computed_member_name && value == "__proto__" {
                    return None;
                }
                // A computed property `["something"]` can always be simplified to a string literal "something",
                // unless it is a template literal inside that contains unescaped new line characters:
                //
                // const a = {
                //   [`line1
                //   line2`]: true
                // }
                //
                if (is_computed_member_name && !has_unescaped_new_line(&value))
                    || is_js_ident(&value)
                {
                    return Some((inner_expression.range(), token, is_computed_member_name));
                }
            }
            _ => {}
        }
        None
    }

    fn diagnostic(
        _ctx: &RuleContext<Self>,
        (range, _, is_computed_member_name): &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            if *is_computed_member_name {
                markup! {
                    "The computed expression can be simplified to a string literal."
                }
            } else {
                markup! {
                    "The computed expression can be simplified without the use of a string literal."
                }
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, (_, token, _): &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        match node {
            AnyJsMember::AnyJsComputedMember(node) => {
                let identifier = inner_string_text(token);
                let object = node.object().ok()?;
                let member = make::js_name(make::ident(&identifier));
                let dot_token = node
                    .optional_chain_token()
                    .unwrap_or_else(|| make::token(T![.]));

                match node {
                    AnyJsComputedMember::JsComputedMemberExpression(node) => {
                        let static_expression = make::js_static_member_expression(
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
                        let static_member = make::js_static_member_assignment(
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
            AnyJsMember::JsComputedMemberName(member) => {
                let name_token = if token.kind() == JsSyntaxKind::JS_NUMBER_LITERAL {
                    token.clone()
                } else {
                    let identifier = inner_string_text(token);
                    if ctx.preferred_quote().is_double() {
                        make::js_string_literal(&identifier)
                    } else {
                        make::js_string_literal_single_quotes(&identifier)
                    }
                };
                if member.syntax().parent().kind() == Some(JsSyntaxKind::TS_ENUM_MEMBER) {
                    let literal_enum_member_name = make::ts_literal_enum_member_name(name_token);
                    mutation.replace_node(
                        AnyTsEnumMemberName::from(member.clone()),
                        literal_enum_member_name.into(),
                    );
                } else {
                    let literal_member_name = make::js_literal_member_name(name_token);
                    mutation.replace_node(
                        AnyJsObjectMemberName::from(member.clone()),
                        literal_member_name.into(),
                    );
                }
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use a literal key instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub AnyJsMember = AnyJsComputedMember | JsComputedMemberName
}

fn has_unescaped_new_line(text: &str) -> bool {
    let mut iter = text.as_bytes().iter();
    while let Some(c) = iter.next() {
        match c {
            b'\\' => {
                iter.next();
            }
            b'\n' => {
                return true;
            }
            _ => {}
        }
    }
    false
}
