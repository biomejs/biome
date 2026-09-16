use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList, TokenText, declare_node_union};
use biome_rule_options::no_duplicate_map_keys::NoDuplicateMapKeysOptions;
use biome_yaml_syntax::{
    AnyYamlBlockMapEntry, AnyYamlBlockNode, AnyYamlMappingImplicitKey, TextRange, YamlBlockMapping,
};
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Disallow two keys with the same name inside YAML maps.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,expect_diagnostic
    /// person:
    ///   name: John Doe
    ///   name: Jane Doe
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml
    /// person:
    ///   name: John Doe
    /// ```
    pub NoDuplicateMapKeys {
        version: "next",
        name: "noDuplicateMapKeys",
        language: "yaml",
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoDuplicateMapKeys {
    type Query = Ast<YamlBlockMapping>;
    type State = (YamlMappingKey, Vec<TextRange>);
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateMapKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mapping = ctx.query();
        let mut names = FxHashMap::<YamlMappingKey, Vec<TextRange>>::default();
        let mut keys_found = FxHashMap::<TokenText, YamlMappingKey>::default();
        for entry in mapping.entries().iter() {
            let Some(key) = mapping_key(&entry) else {
                continue;
            };
            let Some(text) = key.text() else {
                continue;
            };
            if let Some(original) = keys_found.get(text.text()) {
                names.entry(original.clone()).or_default().push(key.range());
            } else {
                keys_found.insert(text, key);
            }
        }
        names.into_iter().collect::<Vec<_>>().into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (original, duplicates) = state;
        let name = original.text()?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            original.range(),
            markup! {
                "The key "<Emphasis>{name.text()}</Emphasis>" was already declared."
            },
        );
        for range in duplicates {
            diagnostic = diagnostic.detail(
                *range,
                markup! {
                    "This is where a duplicated key was declared again."
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

declare_node_union! {
    pub YamlMappingKey = AnyYamlMappingImplicitKey | AnyYamlBlockNode
}

impl YamlMappingKey {
    /// Returns the token-backed text of the key when it is a single scalar token.
    fn text(&self) -> Option<TokenText> {
        let node = self.syntax();
        let token = node.first_token()?;
        (token == node.last_token()?).then(|| token.token_text_trimmed())
    }
}

fn mapping_key(entry: &AnyYamlBlockMapEntry) -> Option<YamlMappingKey> {
    match entry {
        AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(entry) => {
            entry.key().map(YamlMappingKey::from)
        }
        AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(entry) => {
            entry.key().map(YamlMappingKey::from)
        }
        AnyYamlBlockMapEntry::YamlBogusBlockMapEntry(_) => None,
    }
}
