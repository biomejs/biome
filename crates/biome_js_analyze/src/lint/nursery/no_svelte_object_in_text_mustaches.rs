use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsExpressionTemplateRoot};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_rule_options::no_svelte_object_in_text_mustaches::NoSvelteObjectInTextMustachesOptions;

declare_lint_rule! {
    /// Disallow objects, arrays, functions, and classes in Svelte text mustaches.
    ///
    /// When a mustache tag is rendered as text, Svelte converts its value to a string.
    /// An object literal renders as `[object Object]`, an array as its comma-separated
    /// elements, and a function or class as its source code. This is rarely intended,
    /// and often comes from writing `{{ value }}` out of habit from other template
    /// languages.
    ///
    /// Mustaches that make up an entire attribute value, such as `prop={{ value }}`, are
    /// allowed because the value is passed as is instead of being converted to a string.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// {{ name }}
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// {() => name}
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <input class="{[name]} input" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// {name}
    /// <Component prop={{ name }} />
    /// ```
    ///
    pub NoSvelteObjectInTextMustaches {
        version: "next",
        name: "noSvelteObjectInTextMustaches",
        language: "js",
        domains: &[RuleDomain::Svelte],
        recommended: true,
        sources: &[RuleSource::EslintSvelte("no-object-in-text-mustaches").same()],
    }
}

impl Rule for NoSvelteObjectInTextMustaches {
    type Query = Ast<JsExpressionTemplateRoot>;
    type State = LiteralKind;
    type Signals = Option<Self::State>;
    type Options = NoSvelteObjectInTextMustachesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_svelte_text_interpolation()
        {
            return None;
        }

        match ctx.query().expression()?.omit_parentheses() {
            AnyJsExpression::JsObjectExpression(_) => Some(LiteralKind::Object),
            AnyJsExpression::JsArrayExpression(_) => Some(LiteralKind::Array),
            AnyJsExpression::JsArrowFunctionExpression(_)
            | AnyJsExpression::JsFunctionExpression(_) => Some(LiteralKind::Function),
            AnyJsExpression::JsClassExpression(_) => Some(LiteralKind::Class),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, kind: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().expression()?.range(),
                markup! {
                    "Unexpected "{kind}" in a text mustache."
                },
            )
            .note(markup! {
                "Svelte converts this value to a string when rendering it, which produces output such as "<Emphasis>"[object Object]"</Emphasis>" or the source code of a function."
            })
            .note(markup! {
                "Render a property of the value, or the result of calling a function, instead of the "{kind}" itself."
            }),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LiteralKind {
    Object,
    Array,
    Function,
    Class,
}

impl biome_console::fmt::Display for LiteralKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Object => "object",
            Self::Array => "array",
            Self::Function => "function",
            Self::Class => "class",
        })
    }
}
