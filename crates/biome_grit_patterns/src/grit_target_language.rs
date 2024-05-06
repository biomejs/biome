mod js_target_language;

pub use js_target_language::JsTargetLanguage;

use crate::grit_target_node::{GritTargetNode, GritTargetSyntaxKind};
use biome_rowan::SyntaxKind;
use grit_util::Language;

/// Generates the `GritTargetLanguage` enum.
///
/// This enum contains a variant for every language that we support running Grit
/// queries on. We implement Grit's [`Language`] trait on this enum, and
/// implement the slightly more convenient [`GritTargetLanguageImpl`] for
/// creating language-specific implementations.
macro_rules! generate_target_language {
    ($($language:ident),+) => {
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

            fn is_alternative_metavariable_kind(&self, kind: GritTargetSyntaxKind) -> bool {
                match self {
                    $(Self::$language(_) => $language::is_alternative_metavariable_kind(kind)),+
                }
            }
        }

        impl Language for GritTargetLanguage {
            type Node<'a> = GritTargetNode;

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
                        && self.exact_replaced_variable_regex().is_match(&node.text().to_string()))
            }
        }
    };
}

generate_target_language! {
    JsTargetLanguage
}

/// Trait to be implemented by the language-specific implementations.
///
/// This is used to make language implementations a little easier, by not
/// forcing them to reimplement methods that are common across implementations.
trait GritTargetLanguageImpl {
    type Kind: SyntaxKind;

    fn language_name(&self) -> &'static str;

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)];

    fn is_comment(&self, node: &GritTargetNode) -> bool;

    fn metavariable_kind() -> Self::Kind;

    fn is_alternative_metavariable_kind(_kind: GritTargetSyntaxKind) -> bool {
        false
    }
}
