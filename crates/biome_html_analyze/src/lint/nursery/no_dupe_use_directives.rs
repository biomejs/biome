use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttributeList;
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_dupe_use_directives::NoDupeUseDirectivesOptions;

declare_lint_rule! {
    /// Disallow duplicate `use:` directives on the same Svelte element.
    ///
    /// Having two `use:` directives with the same action name and parameters on a single element
    /// is redundant and likely a mistake.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <div use:tooltip use:tooltip></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <div use:tooltip use:focusTrap></div>
    /// ```
    ///
    pub NoDupeUseDirectives {
        version: "next",
        name: "noDupeUseDirectives",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: true,
        sources: &[RuleSource::EslintSvelte("no-dupe-use-directives").same()],
    }
}

pub struct State {
    /// Range of the duplicate directive.
    duplicate_range: TextRange,
    /// The action name.
    name: String,
    /// Range of the first occurrence.
    original_range: TextRange,
}

/// Key used to detect duplicate `use:` directives.
/// Two directives are duplicates when they share the same action name
/// and the same optional initializer expression text.
#[derive(PartialEq, Eq, Hash)]
struct DirectiveKey {
    name: String,
    initializer: Option<String>,
}

impl Rule for NoDupeUseDirectives {
    type Query = Ast<HtmlAttributeList>;
    type State = State;
    type Signals = Box<[Self::State]>;
    type Options = NoDupeUseDirectivesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut seen: Vec<(DirectiveKey, TextRange)> = Vec::new();
        let mut violations: Vec<State> = Vec::new();

        for attribute in node.iter() {
            let Some(directive) = attribute
                .as_any_svelte_directive()
                .and_then(|dir| dir.as_svelte_use_directive())
            else {
                continue;
            };

            let Ok(value) = directive.value() else {
                continue;
            };
            let Ok(property) = value.property() else {
                continue;
            };
            let Some(svelte_name) = property.as_svelte_name() else {
                continue;
            };
            let Ok(ident) = svelte_name.ident_token() else {
                continue;
            };

            let name_text = ident.text_trimmed().to_string();
            let initializer_text = value
                .initializer()
                .map(|init| init.syntax().text_trimmed().to_string());

            let key = DirectiveKey {
                name: name_text,
                initializer: initializer_text,
            };

            if let Some((_, original_range)) =
                seen.iter().find(|(prev_key, _)| prev_key == &key)
            {
                violations.push(State {
                    duplicate_range: directive.range(),
                    name: ident.text_trimmed().to_string(),
                    original_range: *original_range,
                });
            } else {
                seen.push((key, directive.range()));
            }
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name.as_str();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.duplicate_range,
                markup! {
                    "Duplicate "<Emphasis>"use:"</Emphasis>" directive "<Emphasis>{name}</Emphasis>"."
                },
            )
            .detail(
                state.original_range,
                "This is the first occurrence of the directive.",
            ),
        )
    }
}
