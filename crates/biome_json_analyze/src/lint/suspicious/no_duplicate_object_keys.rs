use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_json_syntax::{JsonMemberName, JsonObjectValue, TextRange};
use biome_rowan::{AstNode, AstSeparatedList};
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Disallow two keys with the same name inside objects.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "title": "New title",
    ///   "title": "Second title"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///   "title": "New title",
    ///   "secondTitle": "Second title"
    /// }
    /// ```
    pub NoDuplicateObjectKeys {
        version: "1.0.0",
        name: "noDuplicateObjectKeys",
        language: "json",
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoDuplicateObjectKeys {
    type Query = Ast<JsonObjectValue>;
    type State = (JsonMemberName, Vec<TextRange>);
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let mut names = FxHashMap::<JsonMemberName, Vec<TextRange>>::default();
        let mut keys_found = FxHashMap::<String, JsonMemberName>::default();
        for member in query.json_member_list().iter().flatten() {
            let name = member.name();

            if let Ok(name) = name {
                let text = name.inner_string_text();
                if let Ok(text) = text {
                    if let Some(original_member) = keys_found.get(text.text()) {
                        if let Some(ranges) = names.get_mut(original_member) {
                            ranges.push(name.range());
                        } else {
                            names.insert(original_member.clone(), vec![name.range()]);
                        }
                    } else {
                        keys_found.insert(text.to_string(), name);
                    }
                }
            }
        }
        let duplicated_keys: Vec<_> = names.into_iter().collect();
        duplicated_keys.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (original, ranges) = state;
        let name = original.inner_string_text().ok()?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            original.range(),
            markup! {
                "The key "<Emphasis>{name.text()}</Emphasis>" was already declared."
            },
        );
        for range in ranges {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "This where a duplicated key was declared again."
                },
            );
        }
        Some(diagnostic.note(
            markup! {
                "If a key is defined multiple times, only the last definition takes effect. Previous definitions are ignored."
            },
        ))
    }
}
