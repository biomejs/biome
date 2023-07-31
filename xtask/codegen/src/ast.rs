//! Generate SyntaxKind definitions as well as typed AST definitions for nodes and tokens.
//! This is derived from rust-analyzer/xtask/codegen

use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use std::vec;

use super::{
    kinds_src::{AstSrc, Field},
    to_lower_snake_case, Mode,
};
use crate::css_kinds_src::CSS_KINDS_SRC;
use crate::generate_node_factory::generate_node_factory;
use crate::generate_nodes_mut::generate_nodes_mut;
use crate::generate_syntax_factory::generate_syntax_factory;
use crate::json_kinds_src::JSON_KINDS_SRC;
use crate::kinds_src::{AstListSeparatorConfiguration, AstListSrc, TokenKind};
use crate::termcolorful::{println_string_with_fg_color, Color};
use crate::ALL_LANGUAGE_KIND;
use crate::{
    generate_macros::generate_macros,
    generate_nodes::generate_nodes,
    generate_syntax_kinds::generate_syntax_kinds,
    generate_v8::generate_v8,
    kinds_src::{AstEnumSrc, AstNodeSrc, JS_KINDS_SRC},
    update, LanguageKind,
};
use std::fmt::Write;
use ungrammar::{Grammar, Rule, Token};
use xtask::{project_root, Result};
// these node won't generate any code
pub const SYNTAX_ELEMENT_TYPE: &str = "SyntaxElement";

pub fn generate_ast(mode: Mode, language_kind_list: Vec<String>) -> Result<()> {
    let codegen_language_kinds = if language_kind_list.is_empty() {
        ALL_LANGUAGE_KIND.clone().to_vec()
    } else {
        language_kind_list
            .iter()
            .filter_map(|kind| match LanguageKind::from_str(kind) {
                Ok(kind) => Some(kind),
                Err(err) => {
                    println_string_with_fg_color(err, Color::Red);
                    None
                }
            })
            .collect::<Vec<_>>()
    };
    for kind in codegen_language_kinds {
        println_string_with_fg_color(
            format!(
                "-------------------Generating Grammar for {}-------------------",
                kind
            ),
            Color::Green,
        );
        let mut ast = load_ast(kind);
        ast.sort();
        generate_syntax(ast, &mode, kind)?;
    }

    Ok(())
}

pub(crate) fn load_ast(language: LanguageKind) -> AstSrc {
    match language {
        LanguageKind::Js => load_js_ast(),
        LanguageKind::Css => load_css_ast(),
        LanguageKind::Json => load_json_ast(),
    }
}

pub(crate) fn generate_syntax(ast: AstSrc, mode: &Mode, language_kind: LanguageKind) -> Result<()> {
    let syntax_generated_path = project_root()
        .join("crates")
        .join(language_kind.syntax_crate_name())
        .join("src/generated");
    let factory_generated_path = project_root()
        .join("crates")
        .join(language_kind.factory_crate_name())
        .join("src/generated");
    let v8_bindings_generated_path = project_root()
        .join("crates/rome_v8/src/bindings")
        .join(language_kind.v8_bindings_create_name());

    let kind_src = match language_kind {
        LanguageKind::Js => JS_KINDS_SRC,
        LanguageKind::Css => CSS_KINDS_SRC,
        LanguageKind::Json => JSON_KINDS_SRC,
    // let (nodes, nodes_mut, kinds, factory, node_factory, macros, v8, kind_src) = match language_kind
    // {
    //     LanguageKind::Js => (
    //         crate::JS_AST_NODES,
    //         crate::JS_AST_NODES_MUT,
    //         crate::JS_SYNTAX_KINDS,
    //         crate::JS_SYNTAX_FACTORY,
    //         crate::JS_NODE_FACTORY,
    //         crate::JS_AST_MACROS,
    //         crate::JS_V8,
    //         JS_KINDS_SRC,
    //     ),
    //     LanguageKind::Css => (
    //         crate::CSS_AST_NODES,
    //         crate::CSS_AST_NODES_MUT,
    //         crate::CSS_SYNTAX_KINDS,
    //         crate::CSS_SYNTAX_FACTORY,
    //         crate::CSS_NODE_FACTORY,
    //         crate::CSS_AST_MACROS,
    //         crate::CSS_V8,
    //         CSS_KINDS_SRC,
    //     ),
    //     LanguageKind::Json => (
    //         crate::JSON_AST_NODES,
    //         crate::JSON_AST_NODES_MUT,
    //         crate::JSON_SYNTAX_KINDS,
    //         crate::JSON_SYNTAX_FACTORY,
    //         crate::JSON_NODE_FACTORY,
    //         crate::JSON_AST_MACROS,
    //         crate::JSON_V8,
    //         JSON_KINDS_SRC,
    //     ),
    // };

    let ast_nodes_file = syntax_generated_path.join("nodes.rs");
    let contents = generate_nodes(&ast, language_kind)?;
    update(ast_nodes_file.as_path(), &contents, mode)?;

    let ast_nodes_mut_file = syntax_generated_path.join("nodes_mut.rs");
    let contents = generate_nodes_mut(&ast, language_kind)?;
    update(ast_nodes_mut_file.as_path(), &contents, mode)?;

    let syntax_kinds_file = syntax_generated_path.join("kind.rs");
    let contents = generate_syntax_kinds(&kind_src, language_kind)?;
    update(syntax_kinds_file.as_path(), &contents, mode)?;

    let syntax_factory_file = factory_generated_path.join("syntax_factory.rs");
    let contents = generate_syntax_factory(&ast, language_kind)?;
    update(syntax_factory_file.as_path(), &contents, mode)?;

    let node_factory_file = factory_generated_path.join("node_factory.rs");
    let contents = generate_node_factory(&ast, language_kind)?;
    update(node_factory_file.as_path(), &contents, mode)?;

    let ast_macros_file = syntax_generated_path.join("macros.rs");
    let contents = generate_macros(&ast, language_kind)?;
    update(ast_macros_file.as_path(), &contents, mode)?;

    let ast_v8_file = project_root().join(v8_bindings_generated_path);
    let contents = generate_v8(&kind_src, &ast, language_kind)?;
    update(ast_v8_file.as_path(), &contents, mode)?;

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
                    println!("{}", stack_string);
                    panic!("Variant '{variant}' used twice or circular dependency");
                }
            } else {
                // The variant isn't another enum
                // stack_string.push_str(&format!());
                write!(stack_string, "\nBASE-VAR CHECK : {}", variant).unwrap();
                if !union_set.insert(variant) {
                    // The variant already used
                    println!("{}", stack_string);
                    panic!("Variant '{variant}' used twice");
                }
            }
        }
    }
}

pub(crate) fn load_js_ast() -> AstSrc {
    let grammar_src = include_str!("../js.ungram");
    let grammar: Grammar = grammar_src.parse().unwrap();
    let ast: AstSrc = make_ast(&grammar);
    check_unions(&ast.unions);
    ast
}

pub(crate) fn load_css_ast() -> AstSrc {
    let grammar_src = include_str!("../css.ungram");
    let grammar: Grammar = grammar_src.parse().unwrap();
    make_ast(&grammar)
}

pub(crate) fn load_json_ast() -> AstSrc {
    let grammar_src = include_str!("../json.ungram");
    let grammar: Grammar = grammar_src.parse().unwrap();
    make_ast(&grammar)
}

fn make_ast(grammar: &Grammar) -> AstSrc {
    let mut ast = AstSrc::default();

    for node in grammar.iter() {
        let name = grammar[node].name.clone();
        if name == SYNTAX_ELEMENT_TYPE {
            continue;
        }

        let rule = &grammar[node].rule;

        match classify_node_rule(grammar, rule) {
            NodeRuleClassification::Union(variants) => ast.unions.push(AstEnumSrc {
                documentation: vec![],
                name,
                variants,
            }),
            NodeRuleClassification::Node => {
                let mut fields = vec![];
                handle_rule(&mut fields, grammar, rule, None, false);
                ast.nodes.push(AstNodeSrc {
                    documentation: vec![],
                    name,
                    fields,
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

fn classify_node_rule(grammar: &Grammar, rule: &Rule) -> NodeRuleClassification {
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
        _ => NodeRuleClassification::Node,
    }
}

fn clean_token_name(grammar: &Grammar, token: &Token) -> String {
    let mut name = grammar[*token].name.clone();

    // These tokens, when parsed to proc_macro2::TokenStream, generates a stream of bytes
    // that can't be recognized by [quote].
    // Hence, they need to be decorated with single quotes.
    if "[]{}()`".contains(&name) {
        name = format!("'{}'", name);
    }
    name
}

fn handle_rule(
    fields: &mut Vec<Field>,
    grammar: &Grammar,
    rule: &Rule,
    label: Option<&str>,
    optional: bool,
) {
    match rule {
        Rule::Labeled { label, rule } => {
            // Some methods need to be manually implemented because they need some custom logic;
            // we use the prefix "manual__" to exclude labelled nodes.

            if handle_tokens_in_unions(fields, grammar, rule, label, optional) {
                return;
            }

            handle_rule(fields, grammar, rule, Some(label), optional)
        }
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let name = label
                .map(String::from)
                .unwrap_or_else(|| to_lower_snake_case(&ty));
            let field = Field::Node { name, ty, optional };
            fields.push(field);
        }
        Rule::Token(token) => {
            let name = clean_token_name(grammar, token);

            if name == "''" {
                // array hole
                return;
            }

            let field = Field::Token {
                name: label.map(String::from).unwrap_or_else(|| name.clone()),
                kind: TokenKind::Single(name),
                optional,
            };
            fields.push(field);
        }

        Rule::Rep(_) => {
            panic!("Create a list node for *many* children {:?}", label);
        }
        Rule::Opt(rule) => {
            handle_rule(fields, grammar, rule, label, true);
        }
        Rule::Alt(rules) => {
            for rule in rules {
                handle_rule(fields, grammar, rule, label, false);
            }
        }

        Rule::Seq(rules) => {
            for rule in rules {
                handle_rule(fields, grammar, rule, label, false);
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

    // Does the repeat match (token node))
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
        _ => panic!("The separator in rule {:?} must be a token", rules),
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
    };
    fields.push(field);
    true
}
