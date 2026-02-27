use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlAttributeInitializer, VueDirective, inner_string_text};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_vue_valid_v_text::UseVueValidVTextOptions;

declare_lint_rule! {
    /// Enforce valid `v-text` Vue directives.
    ///
    /// This rule reports `v-text` directives in the following cases:
    /// - The directive has an argument. E.g. `<div v-text:aaa></div>`
    /// - The directive has any modifiers. E.g. `<div v-text.bbb></div>`
    /// - The directive does not have a value. E.g. `<div v-text></div>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-text />
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-text:aaa="foo"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-text.bbb="foo"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-text="foo" />
    /// ```
    ///
    pub UseVueValidVText {
        version: "2.3.7",
        name: "useVueValidVText",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-text").same()],
    }
}

pub enum ViolationKind {
    UnexpectedArgument(TextRange),
    UnexpectedModifier(TextRange),
    MissingValue,
}

impl Rule for UseVueValidVText {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVTextOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let vue_directive = ctx.query();
        if vue_directive.name_token().ok()?.text_trimmed() != "v-text" {
            return None;
        }

        if let Some(arg) = vue_directive.arg() {
            return Some(ViolationKind::UnexpectedArgument(arg.range()));
        }

        if !vue_directive.modifiers().is_empty() {
            let first_modifier = vue_directive.modifiers().iter().next()?;
            return Some(ViolationKind::UnexpectedModifier(first_modifier.range()));
        }

        if let Some(initializer) = vue_directive.initializer()
            && let Ok(AnyHtmlAttributeInitializer::HtmlString(html_string)) = initializer.value()
            && let Ok(token) = html_string.value_token()
        {
            if inner_string_text(&token).trim().is_empty() {
                Some(ViolationKind::MissingValue)
            } else {
                None
            }
        } else {
            Some(ViolationKind::MissingValue)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            match state {
                ViolationKind::UnexpectedArgument(range) => RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The v-text directive does not accept an argument."
                    },
                )
                .note(markup! {
                    "v-text directives should be used without arguments, like " <Emphasis>"v-text=\"content\""</Emphasis>"."
                }),
                ViolationKind::UnexpectedModifier(range) => RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The v-text directive does not support modifiers."
                    },
                )
                .note(markup! {
                    "v-text directives do not support any modifiers. Remove the modifier."
                }),
                ViolationKind::MissingValue => RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().range(),
                    markup! {
                        "The v-text directive is missing a value."
                    },
                )
                .note(markup! {
                    "v-text directives require a value containing the text content to render."
                }).note(markup! {
                    "For example, use " <Emphasis>"v-text=\"foo\""</Emphasis> " to render the content of the " <Emphasis>"foo"</Emphasis> " variable."
                }),
            }
        )
    }
}
