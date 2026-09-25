use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsObjectMember, AnyJsObjectMemberName,
    JsArrayElementList, JsArrayExpression, JsConditionalExpression, JsExpressionTemplateRoot,
    JsParenthesizedExpression, JsSyntaxToken, OperatorPrecedence, T, unescape_js_string,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, BatchMutationExt, Text};
use biome_rule_options::use_astro_object_class_list::UseAstroObjectClassListOptions;

declare_lint_rule! {
    /// Promotes object entries for conditional classes in Astro's `class:list` directive.
    ///
    /// A conditional with one empty string branch can be expressed as an object whose value controls whether the class is included.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic,ignore
    /// <div class:list={active ? "active" : ""}></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro,ignore
    /// <div class:list={{ active }}></div>
    /// <div class:list={[active && "active"]}></div>
    /// ```
    pub UseAstroObjectClassList {
        version: "next",
        name: "useAstroObjectClassList",
        language: "js",
        sources: &[RuleSource::EslintAstro("prefer-object-class-list").inspired()],
        recommended: false,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Astro],
    }
}

#[derive(Clone)]
pub struct UseAstroObjectClassListState {
    class_token: JsSyntaxToken,
    class_name: Text,
    fix: Option<ConditionalClassFix>,
}

#[derive(Clone, Copy)]
enum ConditionalClassFix {
    Consequent,
    Alternate,
}

impl Rule for UseAstroObjectClassList {
    type Query = Ast<JsConditionalExpression>;
    type State = UseAstroObjectClassListState;
    type Signals = Option<Self::State>;
    type Options = UseAstroObjectClassListOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_class_list_attribute()
        {
            return None;
        }
        let mut expression = AnyJsExpression::JsConditionalExpression(ctx.query().clone());
        loop {
            let parent = expression.syntax().parent()?;
            if let Some(parenthesized) = JsParenthesizedExpression::cast(parent.clone()) {
                expression = parenthesized.into();
            } else if JsArrayElementList::can_cast(parent.kind()) {
                expression = JsArrayExpression::cast(parent.parent()?)?.into();
            } else {
                let root = JsExpressionTemplateRoot::cast(parent)?;
                if root.expression()?.syntax() != expression.syntax() {
                    return None;
                }
                break;
            }
        }

        let consequent = string_value(&ctx.query().consequent().ok()?.omit_parentheses())?;
        let alternate = string_value(&ctx.query().alternate().ok()?.omit_parentheses())?;
        let consequent_is_empty = consequent.1.is_empty();
        let alternate_is_empty = alternate.1.is_empty();

        if consequent_is_empty && alternate_is_empty {
            return None;
        }

        let ((class_token, class_name), mut fix) = match (consequent_is_empty, alternate_is_empty) {
            (false, true) => (consequent, Some(ConditionalClassFix::Consequent)),
            (true, false) => (alternate, Some(ConditionalClassFix::Alternate)),
            (false, false) => (consequent, None),
            (true, true) => return None,
        };

        if class_name == "__proto__"
            || ctx.query().syntax().has_comments_direct()
            || ctx.query().syntax().has_comments_descendants()
        {
            fix = None;
        }

        Some(UseAstroObjectClassListState {
            class_token,
            class_name,
            fix,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Use an object entry instead of a conditional class."
                },
            )
            .note(markup! {
                "Astro's object form associates the class "<Emphasis>{state.class_name.text()}</Emphasis>" directly with the condition that controls it."
            })
            .note(markup! {
                "Replace the conditional with an object entry when its branches can be represented without changing the class-list result."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let fix = state.fix?;
        let condition = ctx.query().test().ok()?.trim_trivia()?;
        let condition = match fix {
            ConditionalClassFix::Consequent => condition,
            ConditionalClassFix::Alternate => {
                let condition = if condition.precedence().ok()? < OperatorPrecedence::Unary {
                    make::parenthesized(condition).into()
                } else {
                    condition
                };
                AnyJsExpression::JsUnaryExpression(make::js_unary_expression(
                    make::token(T![!]),
                    condition,
                ))
            }
        };

        let member = make::js_property_object_member(
            AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(
                state
                    .class_token
                    .trim_leading_trivia()
                    .trim_trailing_trivia(),
            )),
            make::token(T![:]).with_trailing_trivia([(
                biome_rowan::TriviaPieceKind::Whitespace,
                " ",
            )]),
            condition,
        );
        let replacement = make::js_object_expression(
            make::token(T!['{']),
            make::js_object_member_list([AnyJsObjectMember::JsPropertyObjectMember(member)], []),
            make::token(T!['}']),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::JsConditionalExpression(ctx.query().clone()),
            AnyJsExpression::JsObjectExpression(replacement),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use an object entry for the conditional class." }.to_owned(),
            mutation,
        ))
    }
}

fn string_value(expression: &AnyJsExpression) -> Option<(JsSyntaxToken, Text)> {
    let AnyJsExpression::AnyJsLiteralExpression(
        AnyJsLiteralExpression::JsStringLiteralExpression(string),
    ) = expression
    else {
        return None;
    };
    let token = string.value_token().ok()?;
    let text = string.inner_string_text().ok()?;
    let bytes = text.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            index += 1;
            let &escaped = bytes.get(index)?;
            if escaped.is_ascii_digit()
                && (escaped != b'0' || bytes.get(index + 1).is_some_and(u8::is_ascii_digit))
            {
                return None;
            }
        }
        index += 1;
    }
    Some((token, unescape_js_string(text)))
}
