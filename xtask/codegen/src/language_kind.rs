use std::str::FromStr;

use crate::css_kinds_src::CSS_KINDS_SRC;
use crate::graphql_kind_src::GRAPHQL_KINDS_SRC;
use crate::grit_kinds_src::GRIT_KINDS_SRC;
use crate::html_kinds_src::HTML_KINDS_SRC;
use crate::js_kinds_src::JS_KINDS_SRC;
use crate::json_kinds_src::JSON_KINDS_SRC;
use crate::kind_src::KindsSrc;
use crate::markdown_kinds_src::MARKDOWN_KINDS_SRC;
use crate::yaml_kinds_src::YAML_KINDS_SRC;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

pub const LANGUAGE_PREFIXES: [&str; 10] = [
    "js_",
    "ts_",
    "jsx_",
    "tsx_",
    "css_",
    "json_",
    "grit_",
    "html_",
    "yaml_",
    "markdown_",
];

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum LanguageKind {
    Js,
    Css,
    Json,
    Graphql,
    Grit,
    Html,
    Yaml,
    Markdown,
}

impl std::fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageKind::Js => write!(f, "js"),
            LanguageKind::Css => write!(f, "css"),
            LanguageKind::Json => write!(f, "json"),
            LanguageKind::Graphql => write!(f, "graphql"),
            LanguageKind::Grit => write!(f, "grit"),
            LanguageKind::Html => write!(f, "html"),
            LanguageKind::Yaml => write!(f, "yaml"),
            LanguageKind::Markdown => write!(f, "markdown"),
        }
    }
}

pub const ALL_LANGUAGE_KIND: [LanguageKind; 8] = [
    LanguageKind::Js,
    LanguageKind::Css,
    LanguageKind::Json,
    LanguageKind::Graphql,
    LanguageKind::Grit,
    LanguageKind::Html,
    LanguageKind::Yaml,
    LanguageKind::Markdown,
];

impl FromStr for LanguageKind {
    type Err = String;

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind {
            "js" => Ok(LanguageKind::Js),
            "css" => Ok(LanguageKind::Css),
            "json" => Ok(LanguageKind::Json),
            "graphql" => Ok(LanguageKind::Graphql),
            "grit" => Ok(LanguageKind::Grit),
            "html" => Ok(LanguageKind::Html),
            "yaml" => Ok(LanguageKind::Yaml),
            "markdown" => Ok(LanguageKind::Markdown),
            _ => Err(format!(
                "Language {kind} not supported, please use: `js`, `css`, `json`, `grit`, `graphql`, `html`, `yaml` or `markdown`"
            )),
        }
    }
}

/// A helper macro to make it easier to define functions that return tokens for a specific language kind.
macro_rules! define_language_kind_function {
    ([$($kind:ident),*],$func:ident,$out:expr) => {
        pub(crate) fn $func(&self) -> TokenStream {
            match self {
                $( LanguageKind::$kind => {
                    // HACK: workaround for $kind$out adding an extra space between the two
                    let ident = format_ident!("{}{}", stringify!($kind), stringify!($out));
                    quote! { #ident }
                },)*
            }
        }
    }
}

/// A helper macro to define functions for each language kind to make it slightly less tedious to add new languages.
macro_rules! define_language_kind_functions {
    ([$($kind:ident),*]) => {
        define_language_kind_function!([$($kind),*], syntax_kind, SyntaxKind);
        define_language_kind_function!([$($kind),*], syntax_factory, SyntaxFactory);
        define_language_kind_function!([$($kind),*], syntax_node, SyntaxNode);
        define_language_kind_function!([$($kind),*], syntax_element, SyntaxElement);
        define_language_kind_function!([$($kind),*], syntax_token, SyntaxToken);
        define_language_kind_function!([$($kind),*], syntax_element_children, SyntaxElementChildren);
        define_language_kind_function!([$($kind),*], syntax_list, SyntaxList);
        define_language_kind_function!([$($kind),*], language, Language);
    }
}

impl LanguageKind {
    define_language_kind_functions!([Js, Css, Json, Graphql, Grit, Html, Yaml, Markdown]);

    pub(crate) fn syntax_crate_ident(&self) -> Ident {
        Ident::new(self.syntax_crate_name().as_str(), Span::call_site())
    }

    pub fn formatter_crate_name(&self) -> String {
        format!("biome_{self}_formatter")
    }

    pub fn syntax_crate_name(&self) -> String {
        format!("biome_{self}_syntax")
    }

    pub fn factory_crate_name(&self) -> String {
        format!("biome_{self}_factory")
    }

    pub fn grit_target_language_module_name(&self) -> String {
        format!("{self}_target_language")
    }

    pub fn kinds(&self) -> KindsSrc {
        match self {
            LanguageKind::Js => JS_KINDS_SRC,
            LanguageKind::Css => CSS_KINDS_SRC,
            LanguageKind::Json => JSON_KINDS_SRC,
            LanguageKind::Graphql => GRAPHQL_KINDS_SRC,
            LanguageKind::Grit => GRIT_KINDS_SRC,
            LanguageKind::Html => HTML_KINDS_SRC,
            LanguageKind::Yaml => YAML_KINDS_SRC,
            LanguageKind::Markdown => MARKDOWN_KINDS_SRC,
        }
    }

    pub fn load_grammar(&self) -> &'static str {
        match self {
            LanguageKind::Js => include_str!("../js.ungram"),
            LanguageKind::Css => include_str!("../css.ungram"),
            LanguageKind::Json => include_str!("../json.ungram"),
            LanguageKind::Graphql => include_str!("../graphql.ungram"),
            LanguageKind::Grit => include_str!("../gritql.ungram"),
            LanguageKind::Html => include_str!("../html.ungram"),
            LanguageKind::Yaml => include_str!("../yaml.ungram"),
            LanguageKind::Markdown => include_str!("../markdown.ungram"),
        }
    }

    pub fn supports_grit(&self) -> bool {
        matches!(self, Self::Css | Self::Js)
    }
}
