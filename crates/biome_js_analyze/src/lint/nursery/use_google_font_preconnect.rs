use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleDomain,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    jsx_ext::AnyJsxElement, AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue, T,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange, TriviaPieceKind};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Ensure the `preconnect` attribute is used when using Google Fonts.
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
        recommended: true,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for UseGoogleFontPreconnect {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.name().ok()?.name_value_token().ok()?.text_trimmed() != "link" {
            return None;
        }

        let href = node.get_attribute_inner_string_text("href")?;
        let rel = node.get_attribute_inner_string_text("rel");

        if href.starts_with("https://fonts.gstatic.com")
            && !matches!(rel.as_deref(), Some("preconnect"))
        {
            return Some(node.syntax().text_trimmed_range());
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The attribute "<Emphasis>"rel=\"preconnect\""</Emphasis>" is missing from the Google Font."
                },
            )
            .note(markup!{
                "Not using "<Emphasis>"preconnect"</Emphasis>" can cause slower font loading and increase latency."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        if node.find_attribute_by_name("rel").is_some() {
            return None;
        }

        let mut mutation = ctx.root().begin();
        let mut attributes: Vec<_> = node.attributes().iter().collect();

        let last_attr_token = match attributes.last()? {
            AnyJsxAttribute::JsxAttribute(a) => a.name_value_token().ok()?,
            AnyJsxAttribute::JsxSpreadAttribute(a) => a.l_curly_token().ok()?,
        };

        let rel = if last_attr_token.has_leading_whitespace_or_newline() {
            let pieces = last_attr_token.leading_trivia().pieces();
            make::jsx_ident("rel").with_leading_trivia_pieces(pieces)
        } else {
            make::jsx_ident("rel").with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
        };

        let new_attribute = AnyJsxAttribute::from(
            make::jsx_attribute(AnyJsxAttributeName::JsxName(make::jsx_name(rel)))
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
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add the "<Emphasis>"rel=\"preconnect\""</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}
