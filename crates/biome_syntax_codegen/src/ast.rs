//! Generate SyntaxKind definitions as well as typed AST definitions for nodes and tokens.
//! This is derived from rust-analyzer/xtask/codegen

use crate::generate_node_factory::generate_node_factory;
use crate::generate_nodes_mut::generate_nodes_mut;
use crate::generate_syntax_factory::generate_syntax_factory;
use crate::language_src::LanguageSrc;
use crate::{
    generate_macros::generate_macros, generate_nodes::generate_nodes,
    generate_syntax_kinds::generate_syntax_kinds, GrammarOptions,
};
use anyhow::{Context, Result};
use biome_string_case::Case;
use biome_ungrammar::{Grammar, Rule, Token};
use quote::format_ident;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::Write;
use std::path::Path;
use std::process::Command;
use std::{fs, vec};

// these node won't generate any code
pub const SYNTAX_ELEMENT_TYPE: &str = "SyntaxElement";

pub(crate) fn load_ungrammar_file(path: &Path, should_check_unions: bool) -> Result<AstSrc> {
    let grammar_src = fs::read_to_string(path)
        .with_context(|| format!("Tried to load the path {}", { path.display() }))?;
    let grammar: Grammar = grammar_src.parse().unwrap();
    let mut ast: AstSrc = make_ast(&grammar);
    if should_check_unions {
        check_unions(&ast.unions);
    }
    ast.sort();
    Ok(ast)
}

pub(crate) fn generate_syntax<K>(
    language_src: K,
    ast: AstSrc,
    options: GrammarOptions,
) -> Result<()>
where
    K: LanguageSrc,
{
    let GrammarOptions {
        syntax_dir_path: syntax_path,
        factory_dir_path: syntax_factory_path,
        syntax_crate_name,
        ..
    } = options;

    let ast_nodes_file = syntax_path.join("nodes.rs");
    let contents = generate_nodes(&ast, &language_src)?;
    write_and_format(contents, ast_nodes_file.as_path())?;

    let ast_nodes_mut_file = syntax_path.join("nodes_mut.rs");
    let contents = generate_nodes_mut(&ast, &language_src)?;
    write_and_format(contents, ast_nodes_mut_file.as_path())?;

    let syntax_kinds_file = syntax_path.join("kind.rs");
    let contents = generate_syntax_kinds(&language_src)?;
    write_and_format(contents, syntax_kinds_file.as_path())?;

    let syntax_factory_file = syntax_factory_path.join("syntax_factory.rs");
    let contents = generate_syntax_factory(&ast, syntax_crate_name.as_str(), &language_src)?;
    write_and_format(contents, syntax_factory_file.as_path())?;

    let node_factory_file = syntax_factory_path.join("node_factory.rs");
    let contents = generate_node_factory(&ast, syntax_crate_name.as_str(), &language_src)?;
    write_and_format(contents, node_factory_file.as_path())?;

    let ast_macros_file = syntax_path.join("macros.rs");
    let contents = generate_macros(&ast, &language_src)?;
    write_and_format(contents, ast_macros_file.as_path())?;

    Ok(())
}

fn check_unions(unions: &[AstEnumSrc]) {
    // Setup a map to find the unions quickly
    let union_map: HashMap<_, _> = unions.iter().map(|en| (&en.name, en)).collect();

    // Iterate over all unions
    for union in unions {
        let mut stack_string = format!(
            "\n******** START ERROR STACK ********\nChecking {}, variants : {:?}",
            union.name, union.variants
        );
        let mut union_set: HashSet<_> = HashSet::from([&union.name]);
        let mut union_queue: VecDeque<_> = VecDeque::new();

        // Init queue for BFS
        union_queue.extend(&union.variants);

        // Loop over the queue getting the first variant
        while let Some(variant) = union_queue.pop_front() {
            if union_map.contains_key(variant) {
                // The variant is a compound variant
                // Get the struct from the map
                let current_union = union_map[variant];
                write!(
                    stack_string,
                    "\nSUB-ENUM CHECK : {}, variants : {:?}",
                    current_union.name, current_union.variants
                )
                .unwrap();
                // Try to insert the current variant into the set
                if union_set.insert(&current_union.name) {
                    // Add all variants into the BFS queue
                    union_queue.extend(&current_union.variants);
                } else {
                    // We either have a circular dependency or 2 variants referencing the same type
                    println!("{stack_string}");
                    panic!("Variant '{variant}' used twice or circular dependency");
                }
            } else {
                // The variant isn't another enum
                // stack_string.push_str(&format!());
                write!(stack_string, "\nBASE-VAR CHECK : {variant}").unwrap();
                if !union_set.insert(variant) {
                    // The variant already used
                    println!("{stack_string}");
                    panic!("Variant '{variant}' used twice");
                }
            }
        }
    }
}

fn write_and_format(contents: String, path: &Path) -> Result<()> {
    let mut rustfmt = Command::new("rustfmt");
    fs::write(path, contents)
        .with_context(|| format!("Tried to load the path {}", { path.display() }))?;
    rustfmt
        .args([path.as_os_str()])
        .output()
        .expect("format the file");

    Ok(())
}

pub(crate) fn append_css_property_value_implied_alternatives(variants: Vec<String>) -> Vec<String> {
    let mut cloned = variants.clone();
    if !cloned.iter().any(|v| v == "CssWideKeyword") {
        cloned.push(String::from("CssWideKeyword"));
    }
    if !cloned.iter().any(|v| v == "CssUnknownPropertyValue") {
        cloned.push(String::from("CssUnknownPropertyValue"));
    }
    if !cloned.iter().any(|v| v == "CssBogusPropertyValue") {
        cloned.push(String::from("CssBogusPropertyValue"));
    }
    cloned
}

fn make_ast(grammar: &Grammar) -> AstSrc {
    let mut ast = AstSrc::default();

    for node in grammar.iter() {
        let name = grammar[node].name.clone();
        if name == SYNTAX_ELEMENT_TYPE {
            continue;
        }

        let rule = &grammar[node].rule;

        match classify_node_rule(grammar, rule, &name) {
            NodeRuleClassification::Union(variants) => {
                // TODO: This is CSS-specific and would be better handled with a per-language
                // method for classifying or modifying rules before generation.
                let variants = if name.trim().starts_with("AnyCss")
                    && name.trim().ends_with("PropertyValue")
                {
                    append_css_property_value_implied_alternatives(variants)
                } else {
                    variants
                };

                ast.unions.push(AstEnumSrc {
                    documentation: vec![],
                    name,
                    variants,
                })
            }
            NodeRuleClassification::Node => {
                let mut fields = vec![];
                handle_rule(&mut fields, grammar, rule, None, false, false);
                let is_dynamic = fields.iter().any(|field| field.is_unordered());
                ast.nodes.push(AstNodeSrc {
                    documentation: vec![],
                    name,
                    fields,
                    dynamic: is_dynamic,
                })
            }
            NodeRuleClassification::DynamicNode => {
                let mut fields = vec![];
                handle_rule(&mut fields, grammar, rule, None, false, true);
                ast.nodes.push(AstNodeSrc {
                    documentation: vec![],
                    name,
                    fields,
                    dynamic: true,
                })
            }
            NodeRuleClassification::Bogus => ast.bogus.push(name),
            NodeRuleClassification::List {
                separator,
                element_name,
            } => {
                ast.push_list(
                    name.as_str(),
                    AstListSrc {
                        element_name,
                        separator,
                    },
                );
            }
        }
    }

    ast
}

/// Classification of a node rule.
/// Determined by matching the top level production of any node.
enum NodeRuleClassification {
    /// Union of the form `A = B | C`
    Union(Vec<String>),

    /// Regular node containing tokens or sub nodes of the form `A = B 'c'
    Node,

    /// Node containing tokens or sub nodes where at least some of the children
    /// can be unordered, such as the form `A = E '#' (B && C && D)?`. If any
    /// children of a node are unordered, the entire node becomes dynamically ordered
    DynamicNode,

    /// A bogus node of the form `A = SyntaxElement*`
    Bogus,

    /// A list node of the form `A = B*` or `A = (B (',' B)*)` or `A = (B (',' B)* ','?)`
    List {
        /// Name of the nodes stored in this list (`B` in the example above)
        element_name: String,

        /// [None] if this is a node list or [Some] if this is a separated list
        separator: Option<AstListSeparatorConfiguration>,
    },
}

fn classify_node_rule(grammar: &Grammar, rule: &Rule, name: &str) -> NodeRuleClassification {
    match rule {
        // this is for enums
        Rule::Alt(alternatives) => {
            let mut all_alternatives = vec![];
            for alternative in alternatives {
                match alternative {
                    Rule::Node(it) => all_alternatives.push(grammar[*it].name.clone()),
                    Rule::Token(it) if grammar[*it].name == ";" => (),
                    _ => return NodeRuleClassification::Node,
                }
            }
            NodeRuleClassification::Union(all_alternatives)
        }
        // A*
        Rule::Rep(rule) => {
            let element_type = match rule.as_ref() {
                Rule::Node(node) => &grammar[*node].name,
                _ => {
                    panic!("Lists should only be over node types");
                }
            };

            if element_type == SYNTAX_ELEMENT_TYPE {
                NodeRuleClassification::Bogus
            } else {
                NodeRuleClassification::List {
                    separator: None,
                    element_name: element_type.to_string(),
                }
            }
        }
        Rule::Seq(rules) => {
            // (T (',' T)* ','?)
            // (T (',' T)*)
            if let Some(comma_list) = handle_comma_list(grammar, rules.as_slice()) {
                NodeRuleClassification::List {
                    separator: Some(AstListSeparatorConfiguration {
                        allow_trailing: comma_list.trailing_separator,
                        separator_token: comma_list.separator_name.to_string(),
                    }),
                    element_name: comma_list.node_name.to_string(),
                }
            } else {
                NodeRuleClassification::Node
            }
        }
        Rule::UnorderedAll(_) | Rule::UnorderedSome(_) => NodeRuleClassification::DynamicNode,
        Rule::Node(node) if name.starts_with("AnyCss") && name.ends_with("PropertyValue") => {
            // TODO: This is CSS-specific and would be better handled with a per-language
            // method for classifying or modifying rules before generation.
            //
            // We use the convention `AnyCss*PropertyValue` to automatically inject
            // additional implicit variants. If there is only one normal production for
            // the node, then it won't be a `Rule::Alt`, and needs to be handled
            NodeRuleClassification::Union(vec![grammar[*node].name.clone()])
        }
        _ => NodeRuleClassification::Node,
    }
}

fn clean_token_name(grammar: &Grammar, token: &Token) -> String {
    let mut name = grammar[*token].name.clone();

    // These tokens, when parsed to proc_macro2::TokenStream, generates a stream of bytes
    // that can't be recognized by [quote].
    // Hence, they need to be decorated with single quotes.
    if "[]{}()`".contains(&name) {
        name = format!("'{name}'");
    }
    name
}

fn handle_rule(
    fields: &mut Vec<Field>,
    grammar: &Grammar,
    rule: &Rule,
    label: Option<&str>,
    optional: bool,
    unordered: bool,
) {
    match rule {
        Rule::Labeled { label, rule } => {
            // Some methods need to be manually implemented because they need some custom logic;
            // we use the prefix "manual__" to exclude labelled nodes.

            if handle_tokens_in_unions(fields, grammar, rule, label, optional, unordered) {
                return;
            }

            handle_rule(fields, grammar, rule, Some(label), optional, unordered)
        }
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let name = label.map_or_else(|| Case::Snake.convert(&ty), String::from);
            let field = Field::Node {
                name,
                ty,
                optional,
                unordered,
            };
            fields.push(field);
        }
        Rule::Token(token) => {
            let name = clean_token_name(grammar, token);

            if name == "''" {
                // array hole
                return;
            }

            let field = Field::Token {
                name: label.map_or_else(|| name.clone(), String::from),
                kind: TokenKind::Single(name),
                optional,
                unordered,
            };
            fields.push(field);
        }

        Rule::Rep(_) => {
            panic!("Create a list node for *many* children {label:?}");
        }
        Rule::Opt(rule) => {
            handle_rule(fields, grammar, rule, label, true, false);
        }
        Rule::Alt(rules) => {
            // Alts must be required. We don't support alternated rules nested
            // within an Opt, like `(A | B)?`. For those, make a new Rule.
            if optional {
                panic!(
                    "Alternates cannot be nested within an optional Rule. Use a new Node to contain the alternate {label:?}"
                );
            }
            for rule in rules {
                handle_rule(fields, grammar, rule, label, false, false);
            }
        }
        Rule::Seq(rules) => {
            for rule in rules {
                // Sequences can be optional if they are wrapped by an Opt rule, so
                // it is inherited
                handle_rule(fields, grammar, rule, label, optional, false);
            }
        }
        Rule::UnorderedAll(rules) => {
            for rule in rules {
                // UnorderedAll only implies each contained rule is unordered, while
                // optionality is inherited from the parent.
                handle_rule(fields, grammar, rule, label, optional, true);
            }
        }
        Rule::UnorderedSome(rules) => {
            for rule in rules {
                // UnorderedSome implies each contained rule is unordered _and_ optional.
                handle_rule(fields, grammar, rule, label, true, true);
            }
        }
    };
}

#[derive(Debug)]
struct CommaList<'a> {
    node_name: &'a str,
    separator_name: &'a str,
    trailing_separator: bool,
}

// (T (',' T)* ','?)
// (T (',' T)*)
fn handle_comma_list<'a>(grammar: &'a Grammar, rules: &[Rule]) -> Option<CommaList<'a>> {
    // Does it match (T * ',')?
    let (node, repeat, trailing_separator) = match rules {
        [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_separator)] => {
            (node, repeat, Some(trailing_separator))
        }
        [Rule::Node(node), Rule::Rep(repeat)] => (node, repeat, None),
        _ => return None,
    };

    // Is the repeat a ()*?
    let repeat = match &**repeat {
        Rule::Seq(it) => it,
        _ => return None,
    };

    // Does the repeat match (token)
    let comma = match repeat.as_slice() {
        [comma, Rule::Node(n)] => {
            let separator_matches_trailing = if let Some(trailing) = trailing_separator {
                &**trailing == comma
            } else {
                true
            };

            if n != node || !separator_matches_trailing {
                return None;
            }

            comma
        }
        _ => return None,
    };

    let separator_name = match comma {
        Rule::Token(token) => &grammar[*token].name,
        _ => panic!("The separator in rule {rules:?} must be a token"),
    };

    Some(CommaList {
        node_name: &grammar[*node].name,
        trailing_separator: trailing_separator.is_some(),
        separator_name,
    })
}

// handle cases like:  `op: ('-' | '+' | '*')`
fn handle_tokens_in_unions(
    fields: &mut Vec<Field>,
    grammar: &Grammar,
    rule: &Rule,
    label: &str,
    optional: bool,
    unordered: bool,
) -> bool {
    let (rule, optional) = match rule {
        Rule::Opt(rule) => (&**rule, true),
        _ => (rule, optional),
    };

    let rule = match rule {
        Rule::Alt(rule) => rule,
        _ => return false,
    };

    let mut token_kinds = vec![];
    for rule in rule.iter() {
        match rule {
            Rule::Token(token) => token_kinds.push(clean_token_name(grammar, token)),
            _ => return false,
        }
    }

    let field = Field::Token {
        name: label.to_string(),
        kind: TokenKind::Many(token_kinds),
        optional,
        unordered,
    };
    fields.push(field);
    true
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
    #[expect(dead_code)]
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
    #[expect(dead_code)]
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub variants: Vec<String>,
}

impl Field {
    pub fn method_name<K>(&self, kind_source: &K) -> proc_macro2::Ident
    where
        K: LanguageSrc,
    {
        match self {
            Field::Token { name, .. } => {
                let name = kind_source.to_method_name(name);

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
                let final_name = if kind_source.prefixes().contains(&prefix) {
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
