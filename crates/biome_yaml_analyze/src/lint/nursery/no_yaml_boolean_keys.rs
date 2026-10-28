use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList, Direction, TextRange, TextSize, declare_node_union};
use biome_rule_options::no_yaml_boolean_keys::NoYamlBooleanKeysOptions;
use biome_yaml_syntax::{
    AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlMappingImplicitKey, AnyYamlProperty, YamlAliasNode,
    YamlAnchorProperty, YamlBlockInBlockNode, YamlBlockMapExplicitEntry, YamlBlockMapImplicitEntry,
    YamlDirective, YamlDocument, YamlFlowMapExplicitEntry, YamlFlowMapImplicitEntry,
    YamlPlainScalar, YamlSyntaxKind,
};
use std::iter;

declare_lint_rule! {
    /// Disallow boolean values as keys in YAML.
    ///
    /// YAML reads some unquoted words as boolean values (true or false) instead of text.
    /// A key written as one of these words is a boolean, which is rarely intended.
    /// Tools that read the file can reject boolean keys, or convert them to text
    /// that differs from what is written in the file.
    ///
    /// Which words are booleans depends on the YAML version of the document:
    ///
    /// - YAML 1.2, the default: `true`, `True`, `TRUE`, `false`, `False`, and `FALSE`.
    /// - YAML 1.1: the words above, and also `y`, `Y`, `yes`, `Yes`, `YES`, `n`, `N`, `no`, `No`, `NO`,
    ///   `on`, `On`, `ON`, `off`, `Off`, and `OFF`.
    ///
    /// A document uses YAML 1.1 when a `%YAML 1.1` line comes before its `---` line.
    /// A document that has no lines starting with `%` uses the same version as the document before it.
    ///
    /// This rule also reports keys marked as booleans with the `!!bool` tag,
    /// and keys that are aliases (such as `*flag`) to a boolean value.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,expect_diagnostic
    /// true: enabled
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// %YAML 1.1
    /// ---
    /// no: Norway
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// !!bool "true": enabled
    /// ```
    ///
    /// ```yaml,expect_diagnostic
    /// default: &flag true
    /// labels: { *flag : enabled }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml
    /// "true": enabled
    /// !!str false: disabled
    /// # Without a `%YAML 1.1` line, `no` is text.
    /// no: Norway
    /// ```
    ///
    pub NoYamlBooleanKeys {
        version: "2.6.4",
        name: "noYamlBooleanKeys",
        language: "yaml",
        sources: &[RuleSource::EslintYml("no-boolean-key").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnyNoYamlBooleanKeysQuery =
        YamlBlockMapImplicitEntry
        | YamlBlockMapExplicitEntry
        | YamlFlowMapImplicitEntry
        | YamlFlowMapExplicitEntry
}

impl Rule for NoYamlBooleanKeys {
    type Query = Ast<AnyNoYamlBooleanKeysQuery>;
    type State = BooleanKey;
    type Signals = Option<Self::State>;
    type Options = NoYamlBooleanKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query().key()? {
            AnyYamlKeyNode::AnyYamlMappingImplicitKey(
                AnyYamlMappingImplicitKey::YamlAliasNode(alias),
            ) => {
                let anchor = find_anchor(&alias)?;
                let anchored = AnyYamlKeyNode::cast(anchor.syntax().grand_parent()?)?;
                boolean_kind(&anchored, Some(&anchor))?;
                Some(BooleanKey::Alias(anchor.range()))
            }
            key => boolean_kind(&key, None),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            BooleanKey::Word(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This key is a boolean, not text."
                },
            )
            .note(markup! {
                "YAML reads this unquoted word as a boolean value. Tools that read this file can reject boolean keys, or convert them to text that differs from what is written here."
            })
            .note(markup! {
                "Wrap the key in quotes to keep it as text."
            }),
            BooleanKey::Tag(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This key is tagged as a boolean."
                },
            )
            .note(markup! {
                "Tools that read this file can reject boolean keys, or convert them to text that differs from what is written here."
            })
            .note(markup! {
                "Remove the boolean tag and use a text key."
            }),
            BooleanKey::Alias(anchor_range) => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().key()?.range(),
                markup! {
                    "This key is an alias to a boolean value."
                },
            )
            .detail(
                anchor_range,
                markup! {
                    "The alias refers to the value marked with this anchor."
                },
            )
            .note(markup! {
                "Tools that read this file can reject boolean keys, or convert them to text that differs from what is written here."
            })
            .note(markup! {
                "Use a text key instead of the alias."
            }),
        };
        Some(diagnostic)
    }
}

/// The reason a mapping key is a boolean.
#[derive(Clone, Copy, Debug)]
pub enum BooleanKey {
    /// An unquoted word that YAML reads as a boolean, such as `true`. Holds
    /// the range of the word.
    Word(TextRange),
    /// A key tagged as a boolean, such as `!!bool "true"`. Holds the range of
    /// the tag.
    Tag(TextRange),
    /// An alias to a boolean, such as `*flag`. Holds the range of the anchor
    /// that the alias refers to.
    Alias(TextRange),
}

declare_node_union! {
    /// A node that can be a mapping key, or the target of an alias.
    pub AnyYamlKeyNode = AnyYamlMappingImplicitKey | YamlBlockInBlockNode
}

impl AnyNoYamlBooleanKeysQuery {
    /// Returns the key of the entry. The key of a block entry that is written
    /// in flow style is returned without its block wrapper.
    fn key(&self) -> Option<AnyYamlKeyNode> {
        match self {
            Self::YamlBlockMapImplicitEntry(entry) => entry.key().map(AnyYamlKeyNode::from),
            Self::YamlFlowMapImplicitEntry(entry) => entry.key().map(AnyYamlKeyNode::from),
            Self::YamlFlowMapExplicitEntry(entry) => entry.key().map(AnyYamlKeyNode::from),
            Self::YamlBlockMapExplicitEntry(entry) => match entry.key()? {
                AnyYamlBlockNode::YamlBlockInBlockNode(node) => Some(node.into()),
                AnyYamlBlockNode::YamlFlowInBlockNode(node) => {
                    let key = match node.flow().ok()? {
                        AnyYamlFlowNode::YamlAliasNode(node) => node.into(),
                        AnyYamlFlowNode::YamlFlowJsonNode(node) => node.into(),
                        AnyYamlFlowNode::YamlFlowYamlNode(node) => node.into(),
                        AnyYamlFlowNode::YamlBogusFlowNode(_) => return None,
                    };
                    Some(AnyYamlKeyNode::AnyYamlMappingImplicitKey(key))
                }
                AnyYamlBlockNode::YamlBogusBlockNode(_) => None,
            },
        }
    }
}

/// Returns the reason `node` is a boolean, or `None` when it is not one.
/// An alias is not followed to the node it refers to, so it is never a
/// boolean here.
///
/// Pass `anchor` when `node` was reached through an alias. It is the anchor
/// that the alias refers to, and it selects which node is checked when `node`
/// is the first key of a block mapping: the key, or the mapping.
fn boolean_kind(node: &AnyYamlKeyNode, anchor: Option<&YamlAnchorProperty>) -> Option<BooleanKey> {
    let key = match node {
        AnyYamlKeyNode::YamlBlockInBlockNode(node) => {
            return tagged_or_word(node.properties().iter(), None);
        }
        AnyYamlKeyNode::AnyYamlMappingImplicitKey(key) => key,
    };
    let (properties, scalar) = match key {
        AnyYamlMappingImplicitKey::YamlAliasNode(_) => return None,
        AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) => (node.properties(), node.content()),
        AnyYamlMappingImplicitKey::YamlFlowJsonNode(node) => (node.properties(), None),
    };
    // The parser stores the tag and anchor of a block mapping on the first
    // key of that mapping, before the key's own tag and anchor.
    let mapping_properties = if properties.is_empty() {
        0
    } else {
        key.enclosing_mapping_property_count()
    };
    if anchor.is_some_and(|anchor| anchor.syntax().index() < mapping_properties) {
        tagged_or_word(properties.iter().take(mapping_properties), None)
    } else {
        tagged_or_word(properties.iter().skip(mapping_properties), scalar)
    }
}

/// Returns the reason a node with the given `properties` and unquoted content
/// is a boolean, or `None` when it is not one.
fn tagged_or_word(
    mut properties: impl Iterator<Item = AnyYamlProperty>,
    scalar: Option<YamlPlainScalar>,
) -> Option<BooleanKey> {
    let tag = properties.find_map(|property| property.as_yaml_tag_property()?.value_token().ok());
    // A tag sets the type of the node, whatever its content is.
    if let Some(tag) = tag {
        return BOOLEAN_TAGS
            .contains(&tag.text_trimmed())
            .then(|| BooleanKey::Tag(tag.text_trimmed_range()));
    }
    boolean_word_range(&scalar?).map(BooleanKey::Word)
}

/// Returns the range of the word in the unquoted `scalar` when YAML reads it
/// as a boolean.
fn boolean_word_range(scalar: &YamlPlainScalar) -> Option<TextRange> {
    let token = scalar.value_token().ok()?;
    // The token of a key includes the spaces between the key and its `:`.
    let text = token.text_trimmed().trim_end_matches([' ', '\t']);
    let is_boolean = BOOLEAN_WORDS.binary_search(&text).is_ok()
        || (YAML_1_1_BOOLEAN_WORDS.binary_search(&text).is_ok() && is_in_yaml_1_1_document(scalar));
    is_boolean.then(|| TextRange::at(token.text_trimmed_range().start(), TextSize::of(text)))
}

/// Whether the document that contains `scalar` uses YAML 1.1.
fn is_in_yaml_1_1_document(scalar: &YamlPlainScalar) -> bool {
    let Some(document) = scalar.syntax().ancestors().find_map(YamlDocument::cast) else {
        return false;
    };
    // A document without directives keeps the version of the document before it.
    document
        .syntax()
        .siblings(Direction::Prev)
        .filter_map(YamlDocument::cast)
        .map(|document| document.directives())
        .find(|directives| !directives.is_empty())
        .is_some_and(|directives| directives.iter().any(|directive| is_yaml_1_1(&directive)))
}

/// Whether `directive` is `%YAML 1.1`.
fn is_yaml_1_1(directive: &YamlDirective) -> bool {
    directive.value_token().is_ok_and(|token| {
        let mut parts = token.text_trimmed().split_ascii_whitespace();
        parts.next() == Some("%YAML") && parts.next() == Some("1.1") && parts.next().is_none()
    })
}

/// Finds the anchor that `alias` refers to: the closest anchor with the same
/// name that comes before the alias in the same document.
fn find_anchor(alias: &YamlAliasNode) -> Option<YamlAnchorProperty> {
    let alias_token = alias.value_token().ok()?;
    let name = alias_token.text_trimmed().strip_prefix('*')?;
    let document = alias
        .syntax()
        .ancestors()
        .find_map(YamlDocument::cast)?
        .into_syntax();
    let document_start = document.text_range_with_trivia().start();
    iter::successors(alias_token.prev_token(), |token| token.prev_token())
        .take_while(|token| token.text_range().start() >= document_start)
        .filter(|token| token.kind() == YamlSyntaxKind::ANCHOR_PROPERTY_LITERAL)
        .find(|token| token.text_trimmed().strip_prefix('&') == Some(name))
        .and_then(|token| YamlAnchorProperty::cast(token.parent()?))
}

/// The tag of a [boolean](https://yaml.org/type/bool.html), as a
/// [shorthand](https://yaml.org/spec/1.2.2/#691-node-tags) and in verbatim
/// form.
const BOOLEAN_TAGS: [&str; 2] = ["!!bool", "!<tag:yaml.org,2002:bool>"];

/// The words that every YAML version reads as booleans.
///
/// Sorted for binary search.
const BOOLEAN_WORDS: [&str; 6] = ["FALSE", "False", "TRUE", "True", "false", "true"];

/// The words that only [YAML 1.1](https://yaml.org/type/bool.html) reads as
/// booleans.
///
/// Sorted for binary search.
const YAML_1_1_BOOLEAN_WORDS: [&str; 16] = [
    "N", "NO", "No", "OFF", "ON", "Off", "On", "Y", "YES", "Yes", "n", "no", "off", "on", "y",
    "yes",
];

#[cfg(test)]
mod tests {
    use super::{BOOLEAN_WORDS, YAML_1_1_BOOLEAN_WORDS};

    #[test]
    fn boolean_words_are_sorted() {
        for words in [BOOLEAN_WORDS.as_slice(), YAML_1_1_BOOLEAN_WORDS.as_slice()] {
            for pair in words.windows(2) {
                assert!(
                    pair[0] < pair[1],
                    "{} must come before {}",
                    pair[0],
                    pair[1]
                );
            }
        }
    }
}
