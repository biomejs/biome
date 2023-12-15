use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{JsRegexLiteral, JsRegexPattern};
use biome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Identify and suggest fixes for unnecessary backreferences in regular expressions.
    ///
    /// A backreference in a regular expression refers to a previous part of the matched pattern.
    /// In some cases, these backreferences are redundant and can be removed without changing
    /// the behavior of the regex.
    ///
    /// This rule detects such scenarios and suggests a simplified version of the regex.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let regex = /(\d+)\1/; // Unnecessary backreference
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let regex = /(\d+)/; // No backreferences
    /// ```
    ///
    /// ```ts
    /// let regex = /(\d+)\1/; // Necessary backreference
    /// ```
    ///
    pub(crate) NoUselessBackrefInRegex {
        version: "nightly",
        name: "noUselessBackrefInRegex",
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessBackrefInRegex {
    type Query = Ast<JsRegexLiteral>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex_literal = ctx.query();

        if let Some(JsRegexPattern(pattern)) = regex_literal.pattern() {
            if pattern_contains_useless_backreference(&pattern) {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(rule_category!(), ctx.query().range(), markup! {
            "This regex contains an unnecessary backreference."
        }).note(markup! { "Consider removing or replacing the unnecessary backreference to simplify the regex." }))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let regex_literal = ctx.query();
        let mut mutation = ctx.root().begin();

        if let Some(JsRegexPattern(pattern)) = regex_literal.pattern() {
            let simplified_pattern = remove_useless_backreferences(&pattern);

            mutation.replace_node(
                regex_literal.clone(),
                make::js_regex_literal(simplified_pattern).build(),
            );

            return Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::Always,
                message: markup! { "Remove unnecessary backreference in the regex." }.to_owned(),
                mutation,
            });
        }

        None
    }
}

/// Check if the pattern contains any useless backreferences.
fn pattern_contains_useless_backreference(pattern: &str) -> bool {
    // Parse the regex pattern
    // Track capturing groups and their positions
    // Analyze each backreference found:
    //   - Check if it refers to a non-existent group
    //   - Check if it's a forward reference
    //   - Check if it's used within an alternative where it can't match
    // Return true if any such backreference is found
    false // Placeholder
}

/// Remove unnecessary backreferences from the pattern.
fn remove_useless_backreferences(pattern: &str) -> String {
    // Parse the regex pattern
    // Track capturing groups and their positions
    // Identify and remove useless backreferences
    // Reconstruct the pattern without these backreferences
    pattern.to_string() // Placeholder
}
