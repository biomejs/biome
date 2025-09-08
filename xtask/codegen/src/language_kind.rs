use std::str::FromStr;

use crate::css_kinds_src::CSS_KINDS_SRC;
use crate::graphql_kind_src::GRAPHQL_KINDS_SRC;
use crate::grit_kinds_src::GRIT_KINDS_SRC;
use crate::html_kinds_src::HTML_KINDS_SRC;
use crate::js_kinds_src::JS_KINDS_SRC;
use crate::json_kinds_src::JSON_KINDS_SRC;
use crate::kind_src::KindsSrc;
use crate::markdown_kinds_src::MARKDOWN_KINDS_SRC;
use crate::tailwind_kinds_src::TAILWIND_KINDS_SRC;
use crate::yaml_kinds_src::YAML_KINDS_SRC;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};

pub const LANGUAGE_PREFIXES: [&str; 11] = [
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
    "tailwind_",
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
    Tailwind,
}

impl std::fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Js => write!(f, "js"),
            Self::Css => write!(f, "css"),
            Self::Json => write!(f, "json"),
            Self::Graphql => write!(f, "graphql"),
            Self::Grit => write!(f, "grit"),
            Self::Html => write!(f, "html"),
            Self::Yaml => write!(f, "yaml"),
            Self::Markdown => write!(f, "markdown"),
            Self::Tailwind => write!(f, "tailwind"),
        }
    }
}

pub const ALL_LANGUAGE_KIND: [LanguageKind; 9] = [
    LanguageKind::Js,
    LanguageKind::Css,
    LanguageKind::Json,
    LanguageKind::Graphql,
    LanguageKind::Grit,
    LanguageKind::Html,
    LanguageKind::Yaml,
    LanguageKind::Markdown,
    LanguageKind::Tailwind,
];

impl FromStr for LanguageKind {
    type Err = String;

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind {
            "js" => Ok(Self::Js),
            "css" => Ok(Self::Css),
            "json" => Ok(Self::Json),
            "graphql" => Ok(Self::Graphql),
            "grit" => Ok(Self::Grit),
            "html" => Ok(Self::Html),
            "yaml" => Ok(Self::Yaml),
            "markdown" => Ok(Self::Markdown),
            "tailwind" => Ok(Self::Tailwind),
            _ => Err(format!(
                "Language {kind} not supported, please use: `js`, `css`, `json`, `grit`, `graphql`, `html`, `yaml`, `markdown`, or `tailwind`"
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
    define_language_kind_functions!([Js, Css, Json, Graphql, Grit, Html, Yaml, Markdown, Tailwind]);

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

    pub fn kinds(&self) -> KindsSrc<'_> {
        match self {
            Self::Js => JS_KINDS_SRC,
            Self::Css => CSS_KINDS_SRC,
            Self::Json => JSON_KINDS_SRC,
            Self::Graphql => GRAPHQL_KINDS_SRC,
            Self::Grit => GRIT_KINDS_SRC,
            Self::Html => HTML_KINDS_SRC,
            Self::Yaml => YAML_KINDS_SRC,
            Self::Markdown => MARKDOWN_KINDS_SRC,
            Self::Tailwind => TAILWIND_KINDS_SRC,
        }
    }

    pub fn load_grammar(&self) -> &'static str {
        match self {
            Self::Js => include_str!("../js.ungram"),
            Self::Css => include_str!("../css.ungram"),
            Self::Json => include_str!("../json.ungram"),
            Self::Graphql => include_str!("../graphql.ungram"),
            Self::Grit => include_str!("../gritql.ungram"),
            Self::Html => include_str!("../html.ungram"),
            Self::Yaml => include_str!("../yaml.ungram"),
            Self::Markdown => include_str!("../markdown.ungram"),
            Self::Tailwind => include_str!("../tailwind.ungram"),
        }
    }

    pub fn supports_grit(&self) -> bool {
        matches!(self, Self::Css | Self::Js)
    }
}
