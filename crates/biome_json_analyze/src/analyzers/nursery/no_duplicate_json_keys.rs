use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_json_syntax::{JsonMemberName, JsonObjectValue, TextRange};
use biome_rowan::{AstNode, AstSeparatedList};
use rustc_hash::FxHashMap;

declare_rule! {
    /// Disallow two keys with the same name inside a JSON object.
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
    pub NoDuplicateJsonKeys {
        version: "1.0.0",
        name: "noDuplicateJsonKeys",
        recommended: true,
    }
}

pub struct DuplicatedKeys {
    /// The fist key, which should be the correct one
    original_key: JsonMemberName,
    /// The ranges where the duplicated keys are found
    duplicated_keys: Vec<TextRange>,
}

impl Rule for NoDuplicateJsonKeys {
    type Query = Ast<JsonObjectValue>;
    type State = DuplicatedKeys;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let mut names = FxHashMap::<String, Vec<TextRange>>::default();
        let mut original_key = None;
        for (index, member) in query.json_member_list().iter().flatten().enumerate() {
            let name = member.name().ok()?;
            if index == 0 {
                original_key = Some(name.clone());
            }
            let text = name.inner_string_text().ok()?;
            if let Some(ranges) = names.get_mut(text.text()) {
                ranges.push(name.range());
            } else {
                names.insert(text.text().to_string(), vec![]);
            }
        }

        let duplicated_keys: Vec<_> = names
            .into_values()
            .filter(|ranges| !ranges.is_empty())
            .flatten()
            .collect();

        if !duplicated_keys.is_empty() {
            let Some(original_key) = original_key else {
                return None;
            };

            return Some(DuplicatedKeys {
                original_key,
                duplicated_keys,
            });
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicatedKeys {
            duplicated_keys,
            original_key,
        } = state;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            original_key.range(),
            markup! {
                "The key "<Emphasis>{{original_key.inner_string_text().ok()?.text()}}</Emphasis>" was already declared."
            },
        );
        for range in duplicated_keys {
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
