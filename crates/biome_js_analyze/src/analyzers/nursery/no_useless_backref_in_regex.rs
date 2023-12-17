use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsRegexLiteralExpression;
use biome_rowan::AstNode;
use fancy_regex::Regex;
use std::clone::Clone;
use std::collections::{HashMap, HashSet};

declare_rule! {
    /// Detects and warns about unnecessary backreferences in regular expressions.
    ///
    /// Regular expressions in JavaScript allow backreferences using \1, \2, etc., to match the same text as previously matched by a capturing group.
    /// However, misusing or overusing backreferences can make regular expressions hard to read and inefficient.
    /// This rule identifies such unnecessary backreferences.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var regex = /(a)\1/;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var regex = /(a)\1b\2/; // Valid if there's a corresponding second group
    /// var regex = /(a)b\1/;   // Valid use of backreference
    /// ```
    ///
    pub(crate) NoUselessBackrefInRegex {
        version: "1.5.0",
        name: "noUselessBackrefInRegex",
        recommended: true,
    }
}

#[derive(Clone)]
enum UselessBackreferenceType {
    Nested,
    Forward,
    Backward,
    Disjunctive,
    IntoNegativeLookaround,
}

type Backref = (String, String, UselessBackreferenceType);

pub(crate) struct BackrefState {
    backref: String,
    group: String,
    backref_type: UselessBackreferenceType,
}

impl Rule for NoUselessBackrefInRegex {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = BackrefState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex_literal = ctx.query();

        // Ignore regular expressions with syntax errors, except for invalid backreferences.
        match Regex::new(&regex_literal.text()) {
            Ok(_) => {}
            Err(e) => match e {
                fancy_regex::Error::ParseError(_pos, parse_err) => {
                    // If the parse error other than invalid backreference, ignore it.
                    if !parse_err.to_string().contains("backreference") {
                        return None;
                    }
                }
                _ => {
                    return None;
                }
            },
        }

        if let Some(backref) = find_useless_backref(regex_literal) {
            Some(backref)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let BackrefState {
            backref,
            group,
            backref_type,
        } = state;
        let type_msg = match backref_type {
            UselessBackreferenceType::Nested => "from within that group",
            UselessBackreferenceType::Forward => "which appears later in the pattern",
            UselessBackreferenceType::Backward => "which appears before in the same lookbehind",
            UselessBackreferenceType::Disjunctive => "which is in another alternative",
            UselessBackreferenceType::IntoNegativeLookaround => "which is in a negative lookaround",
        };
        let diag = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup!(
                "Backreference '"{ backref }"' will be ignored. It references group '"{ group }"' "{type_msg}"."
            ),
        );
        Some(diag)
    }
}

fn get_path_to_root(node: &JsRegexLiteralExpression) -> Vec<JsRegexLiteralExpression> {
    let mut path = vec![];
    let mut current_node = node.clone();
    while let Some(parent) = current_node.syntax().parent() {
        if let Some(regex) = JsRegexLiteralExpression::cast(parent.clone()) {
            path.push(regex);
        }
        current_node = parent;
    }
    path
}

fn is_lookaround(node: &JsRegexLiteralExpression) -> bool {
    let path = get_path_to_root(node);
    for regex in path {
        if let Some(parent) = regex.syntax().parent() {
            if let Some(lookaround) = JsRegexLookaround::cast(parent.clone()) {
                if lookaround.is_negative() {
                    return true;
                }
            }
        }
    }
    false
}

fn is_negative_lookaround(node: &JsRegexLiteralExpression) -> bool {
    let path = get_path_to_root(node);
    for regex in path {
        if let Some(parent) = regex.syntax().parent() {
            if let Some(lookaround) = JsRegexLookaround::cast(parent.clone()) {
                if lookaround.is_negative() {
                    return true;
                }
            }
        }
    }
    false
}
