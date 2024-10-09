use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    jsx_ext::AnyJsxElement, AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TriviaPieceKind};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Ensure `preconnect` is used with Google Fonts.
    ///
    /// When using Google Fonts, adding the `rel="preconnect"` attribute to the `<link>` tag
    /// that points to `https://fonts.gstatic.com` is recommended to initiate an early
    /// connection to the font's origin. This improves page load performance by reducing latency.
    ///
    /// Failing to use `preconnect` may result in slower font loading times, affecting user experience.
    ///
    /// Note: Next.js automatically adds this preconnect link starting from version 12.0.1, but in cases
    /// where it's manually added, this rule ensures the `preconnect` attribute is properly used.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <link href="https://fonts.gstatic.com"/>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <link rel="preload" href="https://fonts.gstatic.com"/>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <link rel="preconnect" href="https://fonts.gstatic.com"/>
    /// ```
    ///
    /// ```jsx
    /// <link href="/logo.svg" rel="icon" />
    /// ```
    ///
    pub UseGoogleFontPreconnect {
        version: "next",
        name: "useGoogleFontPreconnect",
        language: "jsx",
        sources: &[RuleSource::EslintNext("google-font-preconnect")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseGoogleFontPreconnect {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.name().ok()?.name_value_token()?.text_trimmed() != "link" {
            return None;
        }

        if node
            .get_attribute_inner_string_text("href")
            .map_or(false, |href| href.starts_with("https://fonts.gstatic.com"))
            && node
                .get_attribute_inner_string_text("rel")
                .map_or(true, |rel| rel != "preconnect")
        {
            return Some(node.syntax().text_range());
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        return Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! { ""<Emphasis>"rel=\"preconnect\""</Emphasis>" is missing from Google Font." },
        ));
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        if node.find_attribute_by_name("rel").is_some() {
            return None;
        }

        let mut mutation = ctx.root().begin();
        let mut attributes: Vec<_> = node.attributes().into_iter().collect();

        let new_attribute = AnyJsxAttribute::from(
            make::jsx_attribute(AnyJsxAttributeName::JsxName(make::jsx_name(
                make::jsx_ident("rel").with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
            )))
            .with_initializer(make::jsx_attribute_initializer_clause(
                make::token(T![=]),
                AnyJsxAttributeValue::JsxString(make::jsx_string(make::jsx_string_literal(
                    "preconnect",
                ))),
            ))
            .build(),
        );

        attributes.push(new_attribute);
        mutation.replace_node(node.attributes(), make::jsx_attribute_list(attributes));

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Add "<Emphasis>"rel=\"preconnect\""</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}
