use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_yaml_empty_map_keys::NoYamlEmptyMapKeysOptions;
use biome_yaml_syntax::{
    AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlMappingImplicitKey, YamlBlockMapExplicitEntry,
    YamlBlockMapImplicitEntry, YamlFlowMapExplicitEntry, YamlFlowMapImplicitEntry,
};

declare_lint_rule! {
    /// Disallow empty keys in YAML maps.
    ///
    /// A key is empty when a map entry has no key, as in `: value`.
    /// A key is also empty when it has only a tag (a type marker that starts with `!`, such as `!!str`)
    /// or an anchor (a name that starts with `&`, used to reuse a value elsewhere), as in `&name : value`.
    ///
    /// An empty key is usually a mistake, such as a key name that was deleted or forgotten.
    /// Tools that read the file can reject an empty key, or read it as null or as empty text.
    ///
    /// Keys written as empty quoted text, such as `""`, are not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,expect_diagnostic
    /// : value
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// ?
    /// : value
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// settings: { : value }
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// !!str : value
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml
    /// name: value
    /// "": empty text
    /// ? key
    /// : value
    /// settings: { name: value }
    /// ```
    ///
    pub NoYamlEmptyMapKeys {
        version: "next",
        name: "noYamlEmptyMapKeys",
        language: "yaml",
        sources: &[RuleSource::EslintYml("no-empty-key").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub AnyNoYamlEmptyMapKeysQuery =
        YamlBlockMapImplicitEntry
        | YamlBlockMapExplicitEntry
        | YamlFlowMapImplicitEntry
        | YamlFlowMapExplicitEntry
}

impl Rule for NoYamlEmptyMapKeys {
    type Query = Ast<AnyNoYamlEmptyMapKeysQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoYamlEmptyMapKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().has_empty_key().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().indicator_range(),
                markup! {
                    "This map entry has an empty key."
                },
            )
            .note(markup! {
                "An empty key is usually a mistake. Tools that read this file can reject it, or read it as null or as empty text."
            })
            .note(markup! {
                "Write a key for this entry, or remove the entry."
            }),
        )
    }
}

impl AnyNoYamlEmptyMapKeysQuery {
    /// Whether the entry has no key, or a key that has only a tag or an anchor.
    fn has_empty_key(&self) -> bool {
        match self {
            Self::YamlBlockMapImplicitEntry(entry) => {
                entry.key().is_none_or(|key| is_empty_implicit_key(&key))
            }
            Self::YamlFlowMapImplicitEntry(entry) => {
                entry.key().is_none_or(|key| is_empty_implicit_key(&key))
            }
            Self::YamlFlowMapExplicitEntry(entry) => {
                entry.key().is_none_or(|key| is_empty_implicit_key(&key))
            }
            Self::YamlBlockMapExplicitEntry(entry) => {
                entry.key().is_none_or(|key| is_empty_block_key(&key))
            }
        }
    }

    /// Returns the range of the `?` that starts an explicit entry, or of the
    /// `:` of an implicit entry. Falls back to the range of the whole entry
    /// when that token is missing.
    ///
    /// The tag or anchor of an empty key is not used, because the parser can
    /// store the tag or anchor of a block map on its first key.
    fn indicator_range(&self) -> TextRange {
        let token = match self {
            Self::YamlBlockMapImplicitEntry(entry) => entry.colon_token().ok(),
            Self::YamlFlowMapImplicitEntry(entry) => entry.colon_token(),
            Self::YamlBlockMapExplicitEntry(entry) => entry.question_mark_token().ok(),
            Self::YamlFlowMapExplicitEntry(entry) => entry.question_mark_token().ok(),
        };
        token.map_or_else(|| self.range(), |token| token.text_trimmed_range())
    }
}

/// Whether `key` has only a tag or an anchor, and no text.
fn is_empty_implicit_key(key: &AnyYamlMappingImplicitKey) -> bool {
    matches!(key, AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) if node.content().is_none())
}

/// Whether the key of an explicit block entry has only a tag or an anchor,
/// and no text.
fn is_empty_block_key(key: &AnyYamlBlockNode) -> bool {
    match key {
        // The parser leaves the content missing when the node has only
        // properties, without reporting an error.
        AnyYamlBlockNode::YamlBlockInBlockNode(node) => node.content().is_err(),
        AnyYamlBlockNode::YamlFlowInBlockNode(node) => matches!(
            node.flow(),
            Ok(AnyYamlFlowNode::YamlFlowYamlNode(node)) if node.content().is_none()
        ),
        AnyYamlBlockNode::YamlBogusBlockNode(_) => false,
    }
}
