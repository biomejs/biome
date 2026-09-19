use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TokenText, declare_node_union};
use biome_rule_options::no_duplicate_map_keys::NoDuplicateMapKeysOptions;
use biome_yaml_syntax::{
    AnyYamlBlockMapEntry, AnyYamlBlockNode, AnyYamlFlowMapEntry, AnyYamlMappingImplicitKey,
    TextRange, YamlBlockMapping, YamlFlowMapping,
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
    ///   'name': Jane Doe
    ///   "name": John Smith
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// person: { name: John Doe, name: Jane Doe }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml
    /// person:
    ///   name: John Doe
    /// ```
    ///
    /// ```yaml
    /// person: { name: John Doe }
    /// ```
    ///
    pub NoDuplicateMapKeys {
        version: "next",
        name: "noDuplicateMapKeys",
        language: "yaml",
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoDuplicateMapKeys {
    type Query = Ast<AnyNoDuplicateMapKeysQuery>;
    type State = (AnyYamlMappingKey, Vec<TextRange>);
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateMapKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mapping = ctx.query();
        let mut names = FxHashMap::<AnyYamlMappingKey, Vec<TextRange>>::default();
        let mut keys_found = FxHashMap::<TokenText, AnyYamlMappingKey>::default();
        for key in mapping.keys() {
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
    pub AnyNoDuplicateMapKeysQuery = YamlBlockMapping | YamlFlowMapping
}

impl AnyNoDuplicateMapKeysQuery {
    /// Collects the keys of every entry, regardless of block or flow style.
    fn keys(&self) -> Vec<AnyYamlMappingKey> {
        match self {
            Self::YamlBlockMapping(mapping) => mapping
                .entries()
                .iter()
                .filter_map(|entry| block_mapping_key(&entry))
                .collect(),
            Self::YamlFlowMapping(mapping) => mapping
                .entries()
                .iter()
                .filter_map(|entry| flow_mapping_key(&entry.ok()?))
                .collect(),
        }
    }
}

declare_node_union! {
    pub AnyYamlMappingKey = AnyYamlMappingImplicitKey | AnyYamlBlockNode
}

impl AnyYamlMappingKey {
    /// The key's text with surrounding quotes removed, when it is a scalar.
    fn text(&self) -> Option<TokenText> {
        match self {
            Self::AnyYamlMappingImplicitKey(key) => key.inner_string_text().ok(),
            Self::AnyYamlBlockNode(node) => node.inner_string_text().ok(),
        }
    }
}

fn block_mapping_key(entry: &AnyYamlBlockMapEntry) -> Option<AnyYamlMappingKey> {
    match entry {
        AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(entry) => {
            entry.key().map(AnyYamlMappingKey::from)
        }
        AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(entry) => {
            entry.key().map(AnyYamlMappingKey::from)
        }
        AnyYamlBlockMapEntry::YamlBogusBlockMapEntry(_) => None,
    }
}

fn flow_mapping_key(entry: &AnyYamlFlowMapEntry) -> Option<AnyYamlMappingKey> {
    match entry {
        AnyYamlFlowMapEntry::YamlFlowMapImplicitEntry(entry) => {
            entry.key().map(AnyYamlMappingKey::from)
        }
        AnyYamlFlowMapEntry::YamlFlowMapExplicitEntry(entry) => {
            entry.key().map(AnyYamlMappingKey::from)
        }
    }
}
