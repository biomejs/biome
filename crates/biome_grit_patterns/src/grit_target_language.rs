mod css_target_language;
mod js_target_language;

pub use css_target_language::CssTargetLanguage;
pub use js_target_language::JsTargetLanguage;

use camino::Utf8Path;
use grit_util::{AnalysisLogs, Ast, CodeRange, EffectRange, Language, Parser, SnippetTree};
use std::borrow::Cow;
use std::path::Path;
use std::str::FromStr;

use biome_grit_syntax::{GritLanguageDeclaration, GritSyntaxKind};
use biome_parser::AnyParse;
use biome_rowan::SyntaxKind;
use biome_string_case::StrOnlyExtension;

use crate::grit_css_parser::GritCssParser;
use crate::grit_js_parser::GritJsParser;
use crate::grit_target_node::{GritTargetNode, GritTargetSyntaxKind};
use crate::grit_tree::GritTargetTree;
use crate::CompileError;

/// Generates the `GritTargetLanguage` enum.
///
/// This enum contains a variant for every language that we support running Grit
/// queries on. We implement Grit's [`Language`] trait on this enum, and
/// implement the slightly more convenient [`GritTargetLanguageImpl`] for
/// creating language-specific implementations.
macro_rules! generate_target_language {
    ($([$language:ident, $parser:ident, $name:literal]),+) => {
        #[derive(Clone, Debug)]
        pub enum GritTargetLanguage {
            $($language($language)),+
        }

        $(impl From<$language> for GritTargetLanguage {
            fn from(value: $language) -> Self {
                Self::$language(value)
            }
        })+

        #[cfg(feature = "serde")]
        impl serde::Serialize for GritTargetLanguage {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(Self::$language(_) => serde::Serialize::serialize($name, serializer)),+
                }
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for GritTargetLanguage {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Self::from_str(String::deserialize(deserializer)?.as_str())
                    .map_err(serde::de::Error::custom)
            }
        }

        impl FromStr for GritTargetLanguage {
            type Err = String;

            fn from_str(string: &str) -> Result<Self, Self::Err> {
                match string.to_lowercase_cow() {
                    $(name if $name.to_lowercase_cow() == name => Ok(Self::$language($language))),+,
                    other => Err(format!("Unexpected target language: {other}")),
                }
            }
        }

        #[cfg(feature = "schema")]
        impl schemars::JsonSchema for GritTargetLanguage {
            fn schema_name() -> String {
                "GritTargetLanguage".to_owned()
            }

            fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
                schemars::schema::Schema::Object(schemars::schema::SchemaObject {
                    enum_values: Some(vec![
                        $(serde_json::json!($name)),+
                    ]),
                    ..Default::default()
                })
            }
        }

        impl GritTargetLanguage {
            fn metavariable_kind(&self) -> GritTargetSyntaxKind {
                match self {
                    $(Self::$language(_) => $language::metavariable_kind().into()),+
                }
            }

            pub fn get_parser(&self) -> Box<dyn GritTargetParser> {
                match self {
                    $(Self::$language(_) => Box::new($parser)),+
                }
            }

            pub fn kind_by_name(&self, name: &str) -> Option<GritTargetSyntaxKind> {
                match self {
                    $(Self::$language(lang) => lang.kind_by_name(name).map(Into::into)),+
                }
            }

            pub fn name_for_kind(&self, name: GritTargetSyntaxKind) -> &'static str {
                match self {
                    $(Self::$language(lang) => lang.name_for_kind(name)),+
                }
            }

            pub fn named_slots_for_kind(&self, kind: GritTargetSyntaxKind) -> &'static [(&'static str, u32)] {
                match self {
                    $(Self::$language(lang) => lang.named_slots_for_kind(kind)),+
                }
            }

            fn is_alternative_metavariable_kind(&self, kind: GritTargetSyntaxKind) -> bool {
                match self {
                    $(Self::$language(_) => $language::is_alternative_metavariable_kind(kind)),+
                }
            }

            pub fn is_comment_kind(&self, kind: GritTargetSyntaxKind) -> bool {
                match self {
                    $(Self::$language(_) => $language::is_comment_kind(kind)),+
                }
            }

            pub fn is_disregarded_snippet_field(
                &self,
                kind: GritTargetSyntaxKind,
                slot_index: u32,
                node: Option<GritTargetNode<'_>>,
            ) -> bool {
                match self {
                    $(Self::$language(lang) => lang.is_disregarded_snippet_field(kind, slot_index, node)),+
                }
            }

            pub fn get_equivalence_class(
                &self,
                kind: GritTargetSyntaxKind,
                text: &str,
            ) -> Result<Option<LeafEquivalenceClass>, CompileError> {
                match self {
                    $(Self::$language(lang) => lang.get_equivalence_class(kind, text)),+
                }
            }
        }

        impl Language for GritTargetLanguage {
            type Node<'a> = GritTargetNode<'a>;

            fn language_name(&self) -> &'static str {
                match self {
                    $(Self::$language(_) => $name),+
                }
            }

            fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
                match self {
                    $(Self::$language(language) => language.snippet_context_strings()),+
                }
            }

            fn is_comment(&self, node: &GritTargetNode) -> bool {
                match self {
                    $(Self::$language(language) => language.is_comment(node)),+
                }
            }

            fn is_metavariable(&self, node: &GritTargetNode) -> bool {
                node.kind() == self.metavariable_kind()
                    || (self.is_alternative_metavariable_kind(node.kind())
                        && self.exact_replaced_variable_regex().is_match(node.text()))
            }

            fn align_padding<'a>(
                &self,
                _node: &Self::Node<'a>,
                _range: &CodeRange,
                _skip_ranges: &[CodeRange],
                _new_padding: Option<usize>,
                _offset: usize,
                _substitutions: &mut [(EffectRange, String)],
            ) -> Cow<'a, str> {
                todo!()
            }

            fn pad_snippet<'a>(&self, _snippet: &'a str, _padding: &str) -> Cow<'a, str> {
                todo!()
            }

            fn get_skip_padding_ranges(&self, _node: &Self::Node<'_>) -> Vec<CodeRange> {
                Vec::new()
            }
        }
    }
}

generate_target_language! {
    [CssTargetLanguage, GritCssParser, "CSS"],
    [JsTargetLanguage, GritJsParser, "JavaScript"]
}

impl Default for GritTargetLanguage {
    fn default() -> Self {
        Self::JsTargetLanguage(JsTargetLanguage)
    }
}

impl GritTargetLanguage {
    /// Returns the target language based on the language declaration given
    /// inside a Grit pattern.
    pub fn from_declaration(language_decl: &GritLanguageDeclaration) -> Option<Self> {
        match language_decl
            .name()
            .ok()?
            .as_grit_language_name()?
            .language_kind()
            .ok()?
            .kind()
        {
            GritSyntaxKind::CSS_KW => Some(Self::CssTargetLanguage(CssTargetLanguage)),
            GritSyntaxKind::JS_KW => Some(Self::JsTargetLanguage(JsTargetLanguage)),
            _ => None,
        }
    }

    /// Returns the target language to use for the given file extension.
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "css" => Some(Self::CssTargetLanguage(CssTargetLanguage)),
            "cjs" | "js" | "jsx" | "mjs" | "ts" | "tsx" => {
                Some(Self::JsTargetLanguage(JsTargetLanguage))
            }
            _ => None,
        }
    }

    /// Returns `true` when the text `content` contains an identifier for a
    /// metavariable using bracket syntax.
    ///
    /// The metavariable may occur anywhere inside `content`.
    pub fn matches_bracket_metavariable(&self, content: &str) -> bool {
        self.metavariable_bracket_regex().is_match(content)
    }

    /// Returns `true` when the text `content` is a metavariable identifier.
    ///
    /// No other text is allowed inside `content`.
    pub fn matches_exact_metavariable(&self, content: &str) -> bool {
        self.exact_variable_regex().is_match(content)
    }

    /// Returns `true` when the text `content` contains a metavariable
    /// identifier with its prefix replaced with the
    /// `[Self::metavariable_prefix_substitute()].
    ///
    /// The metavariable may occur anywhere inside `content`.
    pub fn matches_replaced_metavariable(&self, content: &str) -> bool {
        self.replaced_metavariable_regex().is_match(content)
    }

    pub fn parse_snippet_contexts(&self, source: &str) -> Vec<SnippetTree<GritTargetTree>> {
        let source = self.substitute_metavariable_prefix(source);

        let mut snippet_trees: Vec<SnippetTree<GritTargetTree>> = Vec::new();
        for (pre, post) in self.snippet_context_strings() {
            let parse_result = self.get_parser().parse_snippet(pre, &source, post);

            let has_errors = parse_result
                .tree
                .root_node()
                .descendants()
                .any(|descendant| descendant.kind().is_bogus());
            if has_errors {
                continue;
            }

            if !snippet_trees.iter().any(|tree| {
                tree.tree
                    .root_node()
                    .matches_kinds_recursively_with(&parse_result.tree.root_node())
            }) {
                snippet_trees.push(parse_result);
            }
        }

        snippet_trees
    }
}

/// Trait to be implemented by the language-specific implementations.
///
/// This is used to make language implementations a little easier, by not
/// forcing them to reimplement methods that are common across implementations.
trait GritTargetLanguageImpl {
    type Kind: SyntaxKind;

    /// Returns the syntax kind for a node by name.
    ///
    /// This is the inverse of [Self::name_for_kind()].
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_symbol_for_name()`.
    fn kind_by_name(&self, node_name: &str) -> Option<Self::Kind>;

    /// Returns the node name for a given syntax kind.
    ///
    /// This is the inverse of [Self::kind_by_name()].
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_symbol_name()`.
    fn name_for_kind(&self, kind: GritTargetSyntaxKind) -> &'static str;

    /// Returns the slots with their names for the given node kind.
    ///
    /// For compatibility with existing Grit snippets (as well as the online
    /// Grit playground), node names should be aligned with TreeSitter's
    /// `ts_language_field_name_for_id()`.
    fn named_slots_for_kind(&self, kind: GritTargetSyntaxKind) -> &'static [(&'static str, u32)];

    /// Strings that provide context for parsing snippets.
    ///
    /// Snippet contexts help when a snippet is a valid AST subtree, but needs
    /// to be in a larger tree to parse. For example, matching on a table name
    /// like ` $schema.$table` in SQL is not valid SQL by itself, only when
    /// surrounded by something like `SELECT x from $schema.$table` is the
    /// snippet valid.
    ///
    /// This method returns a list of strings that are used to match the snippet
    /// in the larger tree. For example, the SQL implementation returns
    /// `["SELECT 1 from ", ";"]` to match a table name in a SQL query.
    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)];

    /// Determines whether the given target node is a comment.
    ///
    /// This is allowed to return `true` for nodes whose kind would not return
    /// `true` when passed directly to [is_comment_kind()].
    fn is_comment(&self, node: &GritTargetNode) -> bool {
        Self::is_comment_kind(node.kind())
    }

    /// Determines whether the given kind is a comment kind.
    fn is_comment_kind(kind: GritTargetSyntaxKind) -> bool;

    /// Returns the syntax kind for metavariables.
    fn metavariable_kind() -> Self::Kind;

    /// Returns whether the given syntax kind is an "alternative" kind for
    /// metavariables.
    ///
    /// For example, in JavaScript, the content of a template string may also
    /// contain metavariables.
    ///
    /// Note that any node kind for which this returns `true` should have a
    /// (trimmed) text representation which corresponds exactly to the
    /// metavariable representation.
    fn is_alternative_metavariable_kind(_kind: GritTargetSyntaxKind) -> bool {
        false
    }

    /// Ordinarily, we want to match on all possible fields, including the absence of nodes within a field.
    /// e.g., `my_function()` should not match `my_function(arg)`.
    ///
    /// However, some fields are trivial or not expected to be part of the snippet, and should be disregarded.
    /// For example, in JavaScript, we want to match both `function name() {}` and `async function name() {}` with the same snippet.
    ///
    /// You can still match on the presence/absence of the field in the snippet by including a metavariable and checking its value.
    /// For example, in JavaScript:
    /// ```grit
    /// `$async func name(args)` where $async <: .
    /// ```
    ///
    /// This method allows you to specify fields that should be (conditionally) disregarded in snippets.
    /// The actual value of the field from the snippet, if any, is passed in as the third argument.
    ///
    /// Note that if a field is always disregarded, you can still switch to ast_node syntax to match on these fields.
    /// For example, in react_to_hooks we match on `arrow_function` and capture `$parenthesis` for inspection.
    ///
    /// ```grit
    /// arrow_function(parameters=$props, $body, $parenthesis) where {
    ///     $props <: contains or { `props`, `inputProps` },
    ///     $body <: not contains `props`,
    ///    if ($parenthesis <: .) {
    ///         $props => `()`
    ///     } else {
    ///         $props => .
    ///     }
    /// }
    /// ```
    fn is_disregarded_snippet_field(
        &self,
        _kind: GritTargetSyntaxKind,
        _slot_index: u32,
        _node: Option<GritTargetNode<'_>>,
    ) -> bool {
        false
    }

    /// Returns an optional "equivalence class" for the given syntax kind.
    ///
    /// Equivalence classes allow leaf nodes to be classified as being equal,
    /// even when their text representations or syntax kinds differ.
    fn get_equivalence_class(
        &self,
        _kind: GritTargetSyntaxKind,
        _text: &str,
    ) -> Result<Option<LeafEquivalenceClass>, CompileError> {
        Ok(None)
    }
}

pub trait GritTargetParser: Parser<Tree = GritTargetTree> {
    #[expect(clippy::wrong_self_convention)]
    fn from_cached_parse_result(
        &self,
        parse: &AnyParse,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
    ) -> Option<GritTargetTree>;

    fn parse_with_path(&self, source: &str, path: &Utf8Path) -> AnyParse;
}

#[derive(Clone, Debug)]
pub struct LeafEquivalenceClass {
    representative: String,
    class: Vec<LeafNormalizer>,
}

impl LeafEquivalenceClass {
    pub fn are_equivalent(&self, kind: GritTargetSyntaxKind, text: &str) -> bool {
        self.class
            .iter()
            .find(|eq| eq.kind == kind)
            .is_some_and(|normalizer| {
                normalizer
                    .normalize(text)
                    .is_some_and(|s| s == self.representative)
            })
    }

    pub(crate) fn new(
        representative: &str,
        kind: GritTargetSyntaxKind,
        members: &[LeafNormalizer],
    ) -> Result<Option<Self>, CompileError> {
        if let Some(normalizer) = members.iter().find(|norm| norm.kind == kind) {
            let rep = normalizer
                .normalize(representative)
                .ok_or(CompileError::NormalizationError)?;
            Ok(Some(Self {
                representative: rep.to_owned(),
                class: members.to_owned(),
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct LeafNormalizer {
    kind: GritTargetSyntaxKind,
    normalizer: fn(&str) -> Option<&str>,
}

impl LeafNormalizer {
    fn normalize<'a>(&self, s: &'a str) -> Option<&'a str> {
        (self.normalizer)(s)
    }

    pub(crate) const fn new(
        kind: GritTargetSyntaxKind,
        normalizer: fn(&str) -> Option<&str>,
    ) -> Self {
        Self { kind, normalizer }
    }

    pub(crate) fn kind(&self) -> GritTargetSyntaxKind {
        self.kind
    }
}

fn normalize_quoted_string(string: &str) -> Option<&str> {
    // Strip the quotes, regardless of type:
    (string.len() >= 2).then(|| &string[1..string.len() - 1])
}

#[derive(Debug, Clone)]
enum DisregardedSlotCondition {
    Always,
    OnlyIf(&'static [&'static str]),
}
