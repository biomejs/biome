use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;

pub trait KindsSrc<'a> {
    fn syntax_kind(&self) -> TokenStream {
        let ident = format_ident!("{}{}", "syntax_kind", "SyntaxKind");
        quote! { #ident }
    }
    fn syntax_factory(&self) -> TokenStream {
        let ident = format_ident!("{}{}", "syntax_factory", "SyntaxFactory");
        quote! { #ident }
    }
    fn syntax_node(&self) -> TokenStream {
        let ident = format_ident!("{}{}", "syntax_node", "SyntaxNode");
        quote! { #ident }
    }
    fn syntax_element(&self) -> TokenStream {
        let ident = format_ident!("{}{}", "syntax_element", "SyntaxElement");
        quote! { #ident }
    }
    fn syntax_token(&self) -> TokenStream {
        let ident = format_ident!("{}{}", "syntax_token", "SyntaxToken");
        quote! { #ident }
    }
    fn syntax_element_children(&self) -> TokenStream {
        let ident = format_ident!("{}{}", "syntax_element_children", "SyntaxElementChildren");
        quote! { #ident }
    }
    fn syntax_list(&self) -> TokenStream {
        let ident = format_ident!("{}{}", "syntax_list", "SyntaxList");
        quote! { #ident }
    }

    fn language(&self) -> TokenStream {
        let ident = format_ident!("{}{}", "language", "Language");
        quote! { #ident }
    }

    /// Special characters of the language. Usually these are parenthesis, dots, commas, etc.
    fn punct(&self) -> &'a [(&'a str, &'a str)];
    /// Reserved keywords of the language
    fn keywords(&self) -> &'a [&'a str];
    /// Literals are special nodes that holds some **values** inside the language, for example: strings, numbers, etc.
    fn literals(&self) -> &'a [&'a str];
    /// Whitespaces, comments, identifiers, etc.
    fn tokens(&self) -> &'a [&'a str];
    /// Nodes of the CST. Usually you want to map these names from the `.ungram` file. For example:
    ///
    /// HtmlAttribute -> HTML_ATTRIBUTE
    /// HtmlBogus -> HTML_BOGUS
    fn nodes(&self) -> &'a [&'a str];

    fn prefix(&self) -> &str;
}

#[derive(Default, Debug)]
pub struct AstSrc {
    pub nodes: Vec<AstNodeSrc>,
    pub unions: Vec<AstEnumSrc>,
    pub lists: BTreeMap<String, AstListSrc>,
    pub bogus: Vec<String>,
}

impl AstSrc {
    pub fn push_list(&mut self, name: &str, src: AstListSrc) {
        self.lists.insert(String::from(name), src);
    }

    pub fn lists(&self) -> std::collections::btree_map::Iter<String, AstListSrc> {
        self.lists.iter()
    }

    pub fn is_list(&self, name: &str) -> bool {
        self.lists.contains_key(name)
    }

    /// Sorts all nodes, enums, etc. for a stable code gen result
    pub fn sort(&mut self) {
        // No need to sort lists, they're stored in a btree
        self.nodes.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        self.unions.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        self.bogus.sort_unstable();

        for union in self.unions.iter_mut() {
            union.variants.sort_unstable();
        }
    }
}

#[derive(Debug)]
pub struct AstListSrc {
    pub element_name: String,
    pub separator: Option<AstListSeparatorConfiguration>,
}

#[derive(Debug)]
pub struct AstListSeparatorConfiguration {
    /// Name of the separator token
    pub separator_token: String,
    /// Whatever the list allows a trailing comma or not
    pub allow_trailing: bool,
}

#[derive(Debug)]
pub struct AstNodeSrc {
    #[allow(dead_code)]
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub fields: Vec<Field>,
    /// Whether the fields of the node should be ordered dynamically using a
    /// slot map for accesses.
    pub dynamic: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Single(String),
    Many(Vec<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Field {
    Token {
        name: String,
        kind: TokenKind,
        optional: bool,
        unordered: bool,
    },
    Node {
        name: String,
        ty: String,
        optional: bool,
        unordered: bool,
    },
}

#[derive(Debug, Clone)]
pub struct AstEnumSrc {
    #[allow(dead_code)]
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub variants: Vec<String>,
}

impl Field {
    pub fn method_name<'a, K>(&self, kind_source: &K) -> proc_macro2::Ident
    where
        K: KindsSrc<'a>,
    {
        match self {
            Field::Token { name, .. } => {
                let name = match name.as_str() {
                    ";" => "semicolon",
                    "'{'" => "l_curly",
                    "'}'" => "r_curly",
                    "'('" => "l_paren",
                    "')'" => "r_paren",
                    "'['" => "l_brack",
                    "']'" => "r_brack",
                    "'`'" => "backtick",
                    "<" => "l_angle",
                    ">" => "r_angle",
                    "=" => "eq",
                    "!" => "excl",
                    "*" => "star",
                    "&" => "amp",
                    "." => "dot",
                    "..." => "dotdotdot",
                    "=>" => "fat_arrow",
                    ":" => "colon",
                    "::" => "double_colon",
                    "?" => "question_mark",
                    "+" => "plus",
                    "-" => "minus",
                    "#" => "hash",
                    "@" => "at",
                    "+=" => "add_assign",
                    "-=" => "subtract_assign",
                    "*=" => "times_assign",
                    "%=" => "remainder_assign",
                    "**=" => "exponent_assign",
                    ">>=" => "left_shift_assign",
                    "<<=" => "right_shift_assign",
                    ">>>=" => "unsigned_right_shift_assign",
                    "~" => "bitwise_not",
                    "&=" => "bitwise_and_assign",
                    "|=" => "bitwise_or_assign",
                    "^=" => "bitwise_xor_assign",
                    "&&=" => "bitwise_logical_and_assign",
                    "||=" => "bitwise_logical_or_assign",
                    "??=" => "bitwise_nullish_coalescing_assign",
                    "++" => "increment",
                    "--" => "decrement",
                    "<=" => "less_than_equal",
                    ">=" => "greater_than_equal",
                    "==" => "equality",
                    "===" => "strict_equality",
                    "!=" => "inequality",
                    "!==" => "strict_inequality",
                    "/" => "slash",
                    "%" => "percent",
                    "**" => "exponent",
                    "<<" => "left_shift",
                    ">>" => "right_shift",
                    ">>>" => "unsigned_right_shift",
                    "|" => "bitwise_or",
                    "^" => "bitwise_xor",
                    "??" => "nullish_coalescing",
                    "||" => "logical_or",
                    "&&" => "logical_and",
                    "$=" => "suffix",
                    "$" => "dollar",
                    "~=" => "whitespace_like",
                    "," => "comma",
                    "---" => "dashdashdash",
                    "<!--" => "comment_start",
                    "-->" => "comment_end",
                    _ => name,
                };

                // we need to replace "-" with "_" for the keywords
                // e.g. we have `color-profile` in css but it's an invalid ident in rust code
                if kind_source.keywords().contains(&name) {
                    format_ident!("{}_token", name.replace('-', "_"))
                } else {
                    format_ident!("{}_token", name)
                }
            }
            Field::Node { name, .. } => {
                let (prefix, tail) = name.split_once('_').unwrap_or(("", name));
                let final_name = if kind_source.prefix().contains(prefix) {
                    tail
                } else {
                    name.as_str()
                };

                // this check here is to avoid emitting methods called "type()",
                // where "type" is a reserved word
                if final_name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{}", final_name)
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token { .. } => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Field::Node { optional, .. } => *optional,
            Field::Token { optional, .. } => *optional,
        }
    }

    pub fn is_unordered(&self) -> bool {
        match self {
            Field::Node { unordered, .. } => *unordered,
            Field::Token { unordered, .. } => *unordered,
        }
    }
}
