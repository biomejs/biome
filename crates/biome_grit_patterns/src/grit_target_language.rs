mod js_target_language;

pub use js_target_language::JsTargetLanguage;

use crate::grit_js_parser::GritJsParser;
use crate::grit_target_node::{GritTargetNode, GritTargetSyntaxKind};
use crate::grit_tree::GritTargetTree;
use biome_rowan::SyntaxKind;
use grit_util::{Ast, CodeRange, EffectRange, Language, Parser, SnippetTree};
use std::borrow::Cow;

/// Generates the `GritTargetLanguage` enum.
///
/// This enum contains a variant for every language that we support running Grit
/// queries on. We implement Grit's [`Language`] trait on this enum, and
/// implement the slightly more convenient [`GritTargetLanguageImpl`] for
/// creating language-specific implementations.
macro_rules! generate_target_language {
    ($([$language:ident, $parser:ident]),+) => {
        #[derive(Clone, Debug)]
        pub enum GritTargetLanguage {
            $($language($language)),+
        }

        $(impl From<$language> for GritTargetLanguage {
            fn from(value: $language) -> Self {
                Self::$language(value)
            }
        })+

        impl GritTargetLanguage {
            fn metavariable_kind(&self) -> GritTargetSyntaxKind {
                match self {
                    $(Self::$language(_) => $language::metavariable_kind().into()),+
                }
            }

            fn get_parser(&self) -> Box<dyn Parser<Tree = GritTargetTree>> {
                match self {
                    $(Self::$language(_) => Box::new($parser)),+
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
        }

        impl Language for GritTargetLanguage {
            type Node<'a> = GritTargetNode<'a>;

            fn language_name(&self) -> &'static str {
                match self {
                    $(Self::$language(language) => language.language_name()),+
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
    [JsTargetLanguage, GritJsParser]
}

impl GritTargetLanguage {
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
        self.snippet_context_strings()
            .iter()
            .map(|(pre, post)| self.get_parser().parse_snippet(pre, &source, post))
            .filter(|result| {
                result
                    .tree
                    .root_node()
                    .descendants()
                    .map_or(false, |mut descendants| {
                        !descendants.any(|descendant| descendant.kind().is_bogus())
                    })
            })
            .collect()
    }
}

/// Trait to be implemented by the language-specific implementations.
///
/// This is used to make language implementations a little easier, by not
/// forcing them to reimplement methods that are common across implementations.
trait GritTargetLanguageImpl {
    type Kind: SyntaxKind;

    fn language_name(&self) -> &'static str;

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
}
