//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
mod ast;
mod css_kinds_src;
mod formatter;
mod generate_analyzer;
mod generate_macros;
pub mod generate_new_lintrule;
mod generate_node_factory;
mod generate_nodes;
mod generate_nodes_mut;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod js_kinds_src;
mod json_kinds_src;

mod html_kinds_src;
mod kind_src;
mod parser_tests;
pub mod promote_rule;
mod termcolorful;
mod unicode;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::path::Path;
use std::str::FromStr;

use xtask::{glue::fs2, Mode, Result};

pub use self::ast::generate_ast;
pub use self::formatter::generate_formatters;
pub use self::generate_analyzer::generate_analyzer;
pub use self::parser_tests::generate_parser_tests;
pub use self::unicode::generate_tables;

pub enum UpdateResult {
    NotUpdated,
    Updated,
}

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum LanguageKind {
    Js,
    Css,
    Json,
    Html,
}

impl std::fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageKind::Js => write!(f, "js"),
            LanguageKind::Css => write!(f, "css"),
            LanguageKind::Json => write!(f, "json"),
            LanguageKind::Html => write!(f, "html"),
        }
    }
}

pub const ALL_LANGUAGE_KIND: [LanguageKind; 4] = [
    LanguageKind::Js,
    LanguageKind::Css,
    LanguageKind::Json,
    LanguageKind::Html,
];

impl FromStr for LanguageKind {
    type Err = String;

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind {
            "js" => Ok(LanguageKind::Js),
            "css" => Ok(LanguageKind::Css),
            "json" => Ok(LanguageKind::Json),
            "html" => Ok(LanguageKind::Html),
            _ => Err(format!(
                "Language {} not supported, please use: `js`, `css` or `json`",
                kind
            )),
        }
    }
}

impl LanguageKind {
    pub(crate) fn syntax_crate_ident(&self) -> Ident {
        Ident::new(self.syntax_crate_name(), Span::call_site())
    }

    pub(crate) fn syntax_kind(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxKind },
            LanguageKind::Css => quote! { CssSyntaxKind },
            LanguageKind::Json => quote! { JsonSyntaxKind },
            LanguageKind::Html => quote! { HtmlSyntaxKind },
        }
    }

    pub(crate) fn syntax_node(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxNode },
            LanguageKind::Css => quote! { CssSyntaxNode },
            LanguageKind::Json => quote! { JsonSyntaxNode },
            LanguageKind::Html => quote! { HtmlSyntaxNode },
        }
    }

    pub(crate) fn syntax_element(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxElement },
            LanguageKind::Css => quote! { CssSyntaxElement },
            LanguageKind::Json => quote! { JsonSyntaxElement },
            LanguageKind::Html => quote! { HtmlSyntaxElement },
        }
    }

    pub(crate) fn syntax_token(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxToken },
            LanguageKind::Css => quote! { CssSyntaxToken },
            LanguageKind::Json => quote! { JsonSyntaxToken },
            LanguageKind::Html => quote! { HtmlSyntaxToken },
        }
    }

    pub(crate) fn syntax_element_children(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxElementChildren },
            LanguageKind::Css => quote! { CssSyntaxElementChildren },
            LanguageKind::Json => quote! { JsonSyntaxElementChildren },
            LanguageKind::Html => quote! { HtmlSyntaxElementChildren },
        }
    }

    pub(crate) fn syntax_list(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxList },
            LanguageKind::Css => quote! { CssSyntaxList },
            LanguageKind::Json => quote! { JsonSyntaxList },
            LanguageKind::Html => quote! { HtmlSyntaxList },
        }
    }

    pub(crate) fn language(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsLanguage },
            LanguageKind::Css => quote! { CssLanguage },
            LanguageKind::Json => quote! { JsonLanguage },
            LanguageKind::Html => quote! { HtmlLanguage },
        }
    }

    pub fn formatter_crate_name(&self) -> &'static str {
        match self {
            LanguageKind::Js => "biome_js_formatter",
            LanguageKind::Css => "biome_css_formatter",
            LanguageKind::Json => "biome_json_formatter",
            LanguageKind::Html => "biome_html_formatter",
        }
    }

    pub fn syntax_crate_name(&self) -> &'static str {
        match self {
            LanguageKind::Js => "biome_js_syntax",
            LanguageKind::Css => "biome_css_syntax",
            LanguageKind::Json => "biome_json_syntax",
            LanguageKind::Html => "biome_html_syntax",
        }
    }

    pub fn factory_crate_name(&self) -> &'static str {
        match self {
            LanguageKind::Js => "biome_js_factory",
            LanguageKind::Css => "biome_css_factory",
            LanguageKind::Json => "biome_json_factory",
            LanguageKind::Html => "biome_html_factory",
        }
    }
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
pub fn update(path: &Path, contents: &str, mode: &Mode) -> Result<UpdateResult> {
    match fs2::read_to_string(path) {
        Ok(old_contents) if old_contents == contents => {
            return Ok(UpdateResult::NotUpdated);
        }
        _ => (),
    }

    if *mode == Mode::Verify {
        anyhow::bail!("`{}` is not up-to-date", path.display());
    }

    eprintln!("updating {}", path.display());
    fs2::write(path, contents)?;
    Ok(UpdateResult::Updated)
}

pub fn to_capitalized(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn to_lower_camel_case(s: &str) -> String {
    to_pascal_camel_case(s, false)
}

pub fn to_pascal_case(s: &str) -> String {
    to_pascal_camel_case(s, true)
}

fn to_pascal_camel_case(s: &str, is_ascal: bool) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = is_ascal;
    for c in s.chars() {
        if c == '_' {
            prev = true;
        } else if prev {
            buf.push(c.to_ascii_uppercase());
            prev = false;
        } else {
            buf.push(c);
        }
    }
    buf
}

pub fn to_upper_snake_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = false;
    for c in s.chars() {
        if c.is_ascii_uppercase() && prev {
            buf.push('_')
        }
        prev = true;

        buf.push(c.to_ascii_uppercase());
    }
    buf
}

pub fn to_lower_snake_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = false;
    for c in s.chars() {
        if c.is_ascii_uppercase() && prev {
            buf.push('_')
        }
        prev = true;

        buf.push(c.to_ascii_lowercase());
    }
    buf
}
