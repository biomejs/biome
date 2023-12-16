use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsRegexLiteralExpression;
use biome_rowan::AstNode;

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
        version: "1.0.0",
        name: "noUselessBackrefInRegex",
        recommended: true,
    }
}

impl Rule for NoUselessBackrefInRegex {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = (); // You might need a more complex state to track capturing groups and backreferences
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex_literal = ctx.query();
        // Implement logic here to parse the regex and check for useless backreferences
        // This is a complex task and might require a regex parsing library or custom implementation

        // Placeholder logic
        // Check if the regex contains a backreference that might be unnecessary
        if is_useless_backref(&regex_literal.text()) {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().syntax().text_range(),
            markup! {
                "This regular expression contains an unnecessary backreference."
            },
        ))
    }
}

// Placeholder function for detecting useless backreferences
// You need to replace this with actual logic based on regex parsing
fn is_useless_backref(regex_pattern: &str) -> bool {
    // Logic to parse regex and check for useless backreferences
    false
}

// Additional logic and helper functions for parsing and analyzing the regex pattern
