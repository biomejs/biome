use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsRegexLiteralExpression;
use biome_rowan::AstNode;
use fancy_regex::Regex;
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

impl Rule for NoUselessBackrefInRegex {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex_literal = ctx.query();

        // Ignore regular expressions with syntax errors, except for invalid backreferences.
        match Regex::new(&regex_literal.text()) {
            Ok(_) => {}
            Err(e) => match e {
                fancy_regex::Error::ParseError(_pos, parse_err) => {
                    if let fancy_regex::ParseError::InvalidBackref = parse_err {
                        return None;
                    }

                    return Some(());
                }
                _ => {
                    return Some(());
                }
            },
        }

        if contains_useless_backreference(&regex_literal.text()) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "This regular expression contains an unnecessary backreference."
            },
        ))
    }
}

fn contains_useless_backreference(regex: &str) -> bool {
    let mut defined_groups = HashSet::new();
    let mut named_groups = HashMap::new();
    let mut current_group = 0;
    let mut chars = regex.chars().peekable();
    let mut previous_was_backslash = false;
    let mut in_named_group = false;
    let mut named_group_name = String::new();

    while let Some(c) = chars.next() {
        if previous_was_backslash {
            match c {
                'k' if chars.next_if_eq(&'<').is_some() => {
                    in_named_group = true;
                    continue;
                }
                _ if c.is_digit(10) => {
                    let group_ref: usize = c.to_string().parse().unwrap();
                    if !defined_groups.contains(&group_ref) {
                        return true;
                    }
                }
                _ => {}
            }
            previous_was_backslash = false;
        } else {
            match c {
                '(' if chars.peek() == Some(&'?') => {
                    chars.next(); // Consume '?'
                    if chars.peek() == Some(&'<') {
                        chars.next(); // Consume '<'
                        in_named_group = true;
                    } else {
                        current_group += 1;
                        defined_groups.insert(current_group);
                    }
                }
                '\\' => previous_was_backslash = true,
                '>' if in_named_group => {
                    in_named_group = false;
                    named_groups.insert(named_group_name.clone(), current_group);
                    named_group_name.clear();
                    current_group += 1;
                    defined_groups.insert(current_group);
                }
                _ if in_named_group => named_group_name.push(c),
                _ => {}
            }
        }
    }

    false
}
