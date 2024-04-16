use crate::css_kinds_src::CSS_KINDS_SRC;
use crate::graphql_kind_src::GRAPHQL_KINDS_SRC;
use crate::grit_kinds_src::GRIT_KINDS_SRC;
use crate::html_kinds_src::HTML_KINDS_SRC;
use crate::js_kinds_src::JS_KINDS_SRC;
use crate::json_kinds_src::JSON_KINDS_SRC;
use crate::kind_src::KindsSrc;
use crate::yaml_kinds_src::YAML_KINDS_SRC;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::str::FromStr;

pub const LANGUAGE_PREFIXES: [&str; 9] = [
    "js_", "ts_", "jsx_", "tsx_", "css_", "json_", "grit_", "html_", "yaml_",
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
        }
    }
}

pub const ALL_LANGUAGE_KIND: [LanguageKind; 7] = [
    LanguageKind::Js,
    LanguageKind::Css,
    LanguageKind::Json,
    LanguageKind::Graphql,
    LanguageKind::Grit,
    LanguageKind::Html,
    LanguageKind::Yaml,
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
            _ => Err(format!(
                "Language {} not supported, please use: `js`, `css`, `json`, `grit`, `graphql`, `html`, `yaml` or yml",
                kind
            )),
        }
    }
}

impl LanguageKind {
    pub(crate) fn syntax_crate_ident(&self) -> Ident {
        Ident::new(self.syntax_crate_name().as_str(), Span::call_site())
    }

    pub(crate) fn syntax_kind(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxKind },
            LanguageKind::Css => quote! { CssSyntaxKind },
            LanguageKind::Json => quote! { JsonSyntaxKind },
            LanguageKind::Graphql => quote! { GraphqlSyntaxKind },
            LanguageKind::Grit => quote! { GritSyntaxKind },
            LanguageKind::Html => quote! { HtmlSyntaxKind },
            LanguageKind::Yaml => quote! { YamlSyntaxKind },
        }
    }
    pub(crate) fn syntax_factory(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxFactory },
            LanguageKind::Css => quote! { CssSyntaxFactory },
            LanguageKind::Json => quote! { JsonSyntaxFactory },
            LanguageKind::Graphql => quote! { GraphqlSyntaxFactory },
            LanguageKind::Grit => quote! { GritSyntaxFactory },
            LanguageKind::Html => quote! { HtmlSyntaxFactory },
            LanguageKind::Yaml => quote! { YamlSyntaxFactory },
        }
    }

    pub(crate) fn syntax_node(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxNode },
            LanguageKind::Css => quote! { CssSyntaxNode },
            LanguageKind::Json => quote! { JsonSyntaxNode },
            LanguageKind::Graphql => quote! { GraphqlSyntaxNode },
            LanguageKind::Grit => quote! { GritSyntaxNode },
            LanguageKind::Html => quote! { HtmlSyntaxNode },
            LanguageKind::Yaml => quote! { YamlSyntaxNode },
        }
    }

    pub(crate) fn syntax_element(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxElement },
            LanguageKind::Css => quote! { CssSyntaxElement },
            LanguageKind::Json => quote! { JsonSyntaxElement },
            LanguageKind::Graphql => quote! { GraphqlSyntaxElement },
            LanguageKind::Grit => quote! { GritSyntaxElement },
            LanguageKind::Html => quote! { HtmlSyntaxElement },
            LanguageKind::Yaml => quote! { YamlSyntaxElement },
        }
    }

    pub(crate) fn syntax_token(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxToken },
            LanguageKind::Css => quote! { CssSyntaxToken },
            LanguageKind::Json => quote! { JsonSyntaxToken },
            LanguageKind::Graphql => quote! { GraphqlSyntaxToken },
            LanguageKind::Grit => quote! { GritSyntaxToken },
            LanguageKind::Html => quote! { HtmlSyntaxToken },
            LanguageKind::Yaml => quote! { YamlSyntaxToken },
        }
    }

    pub(crate) fn syntax_element_children(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxElementChildren },
            LanguageKind::Css => quote! { CssSyntaxElementChildren },
            LanguageKind::Json => quote! { JsonSyntaxElementChildren },
            LanguageKind::Graphql => quote! { GraphqlSyntaxElementChildren },
            LanguageKind::Grit => quote! { GritSyntaxElementChildren },
            LanguageKind::Html => quote! { HtmlSyntaxElementChildren },
            LanguageKind::Yaml => quote! { YamlSyntaxElementChildren },
        }
    }

    pub(crate) fn syntax_list(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxList },
            LanguageKind::Css => quote! { CssSyntaxList },
            LanguageKind::Json => quote! { JsonSyntaxList },
            LanguageKind::Graphql => quote! { GraphqlSyntaxList },
            LanguageKind::Grit => quote! { GritSyntaxList },
            LanguageKind::Html => quote! { HtmlSyntaxList },
            LanguageKind::Yaml => quote! { YamlSyntaxList },
        }
    }

    pub(crate) fn language(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsLanguage },
            LanguageKind::Css => quote! { CssLanguage },
            LanguageKind::Json => quote! { JsonLanguage },
            LanguageKind::Graphql => quote! { GraphqlLanguage },
            LanguageKind::Grit => quote! { GritLanguage },
            LanguageKind::Html => quote! { HtmlLanguage },
            LanguageKind::Yaml => quote! { YamlLanguage },
        }
    }

    pub fn formatter_crate_name(&self) -> String {
        format!("biome_{}_formatter", self)
    }

    pub fn syntax_crate_name(&self) -> String {
        format!("biome_{}_syntax", self)
    }

    pub fn factory_crate_name(&self) -> String {
        format!("biome_{}_factory", self)
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
        }
    }
}
