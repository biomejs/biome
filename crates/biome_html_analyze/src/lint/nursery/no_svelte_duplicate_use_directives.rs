use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::HtmlAttributeList;
use biome_rowan::{AstNode, AstNodeList, TextRange, TokenText};
use biome_rule_options::no_svelte_duplicate_use_directives::NoSvelteDuplicateUseDirectivesOptions;

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
    pub NoSvelteDuplicateUseDirectives {
        version: "next",
        name: "noSvelteDuplicateUseDirectives",
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
    name: TokenText,
    /// Range of the first occurrence.
    original_range: TextRange,
}

/// Key used to detect duplicate `use:` directives.
/// Two directives are duplicates when they share the same action name
/// and the same optional initializer expression text.
#[derive(PartialEq, Eq, Hash)]
struct DirectiveKey {
    name: TokenText,
    initializer: Option<String>,
}

impl Rule for NoSvelteDuplicateUseDirectives {
    type Query = Ast<HtmlAttributeList>;
    type State = State;
    type Signals = Box<[Self::State]>;
    type Options = NoSvelteDuplicateUseDirectivesOptions;

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

            let name_text = ident.token_text_trimmed();
            let initializer_text = value
                .initializer()
                .map(|init| init.syntax().text_trimmed().to_string());

            let key = DirectiveKey {
                name: name_text.clone(),
                initializer: initializer_text,
            };

            if let Some((_, original_range)) =
                seen.iter().find(|(prev_key, _)| prev_key == &key)
            {
                violations.push(State {
                    duplicate_range: directive.range(),
                    name: name_text,
                    original_range: *original_range,
                });
            } else {
                seen.push((key, directive.range()));
            }
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name.text();
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
            )
            .note(markup! {
                "Using the same action twice on an element is redundant. Remove the duplicate "<Emphasis>"use:"</Emphasis>" directive."
            }),
        )
    }
}
