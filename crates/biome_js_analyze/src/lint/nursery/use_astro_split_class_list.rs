use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsLiteralExpression, JsExpressionTemplateRoot,
    JsStringLiteralExpression, T,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TokenText, TriviaPieceKind};
use biome_rule_options::use_astro_split_class_list::UseAstroSplitClassListOptions;

declare_lint_rule! {
    /// Splits space-separated strings into separate entries in Astro's `class:list` directive.
    ///
    /// The rule only reports unescaped string literals whose non-empty tokens are separated by one ASCII space.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```astro,expect_diagnostic,ignore
    /// <div class:list={"card active"}></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```astro,ignore
    /// <div class:list={["card", "active"]}></div>
    /// <div class:list={`card ${state}`}></div>
    /// ```
    pub UseAstroSplitClassList {
        version: "next",
        name: "useAstroSplitClassList",
        language: "js",
        sources: &[RuleSource::EslintAstro("prefer-split-class-list").inspired()],
        recommended: false,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Astro],
    }
}

impl Rule for UseAstroSplitClassList {
    type Query = Ast<JsStringLiteralExpression>;
    type State = TokenText;
    type Signals = Option<Self::State>;
    type Options = UseAstroSplitClassListOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_class_list_attribute()
            || !is_direct_class_list_value(ctx.query())
        {
            return None;
        }

        let value = ctx.query().inner_string_text().ok()?;
        let text = value.text();
        if text.contains('\\')
            || text.starts_with(' ')
            || text.ends_with(' ')
            || text.contains("  ")
            || text
                .bytes()
                .any(|byte| byte.is_ascii_whitespace() && byte != b' ')
            || text.split(' ').count() < 2
        {
            return None;
        }

        Some(value)
    }

    fn diagnostic(ctx: &RuleContext<Self>, value: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Split this string into separate "<Emphasis>"class:list"</Emphasis>" entries."
                },
            )
            .note(markup! {
                "The string "<Emphasis>{value.text()}</Emphasis>" combines classes that Astro can represent as distinct list entries."
            })
            .note(markup! {
                "Replace the string with an array containing one entry for each class."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, value: &Self::State) -> Option<JsRuleAction> {
        let single_quotes = ctx.query().value_token().ok()?.text_trimmed().starts_with('\'');
        let elements: Vec<_> = value.text().split(' ').map(|class_name| {
            let token = if single_quotes {
                make::js_string_literal_single_quotes(class_name)
            } else {
                make::js_string_literal(class_name)
            };
            AnyJsArrayElement::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(token),
                ),
            ))
        }).collect();
        let separator_count = value.text().split(' ').count() - 1;
        let separators = std::iter::repeat_n(
            make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            separator_count,
        );
        let replacement = make::js_array_expression(
            make::token(T!['[']),
            make::js_array_element_list(elements, separators),
            make::token(T![']']),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(ctx.query().clone()),
            ),
            AnyJsExpression::JsArrayExpression(replacement),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Split the class string into separate entries." }.to_owned(),
            mutation,
        ))
    }
}

fn is_direct_class_list_value(string: &JsStringLiteralExpression) -> bool {
    let Some(root) = string
        .syntax()
        .ancestors()
        .find_map(JsExpressionTemplateRoot::cast)
    else {
        return false;
    };
    let Ok(root_expression) = root.expression() else {
        return false;
    };
    let root_expression = root_expression.omit_parentheses();
    if root_expression.syntax() == string.syntax() {
        return true;
    }

    let Some(array) = root_expression.as_js_array_expression() else {
        return false;
    };
    array.elements().iter().any(|element| {
        matches!(
            element,
            Ok(AnyJsArrayElement::AnyJsExpression(expression))
                if expression.clone().omit_parentheses().syntax() == string.syntax()
        )
    })
}
