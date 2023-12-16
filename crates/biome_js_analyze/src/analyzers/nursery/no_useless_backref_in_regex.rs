use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsRegexLiteralExpression;
use biome_rowan::AstNode;
use regex::Regex;
use std::collections::HashSet;

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

        // Check is the regex valid
        if Regex::new(&regex_literal.text()).is_err() {
            return None;
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
    let mut current_group = 0;
    let mut chars = regex.chars().peekable();
    let mut previous_was_backslash = false;

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                current_group += 1;
                defined_groups.insert(current_group);
            }
            '\\' => {
                if previous_was_backslash {
                    // This is an escaped backslash, reset flag
                    previous_was_backslash = false;
                } else {
                    // This is a backslash, set flag and handle next char
                    previous_was_backslash = true;
                    continue;
                }
            }
            _ if previous_was_backslash && c.is_digit(10) => {
                let group_ref: usize = c.to_string().parse().unwrap();
                if !defined_groups.contains(&group_ref) {
                    return true; // Useless backreference found
                }
                previous_was_backslash = false;
            }
            _ => {
                previous_was_backslash = false;
            }
        }
    }

    false
}
