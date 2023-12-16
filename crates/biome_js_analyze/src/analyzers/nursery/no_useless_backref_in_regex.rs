use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsRegexLiteralExpression, JsSyntaxNode};
use biome_rowan::AstNode;
use regex_syntax::ast::GroupKind;
use regex_syntax::hir::{Hir, HirKind};
use regex_syntax::Parser;
use std::collections::HashSet;

use crate::semantic_services::Semantic;

declare_rule! {
    /// Disallow useless backreferences in regular expressions.
    ///
    /// This rule reports regular expressions that use backreferences (\k<name>, \number)
    /// that refer to a non-existent capturing group.
    ///
    /// These references will always fail to match, which may not be the intended behavior.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /(?:\d)-\1/;  // \1 refers to a non-existent capturing group
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// /(\d)-\1/;  // \1 correctly refers to the capturing group
    /// ```
    ///
    pub(crate) NoUselessBackrefInRegex {
        version: "nightly",
        name: "noUselessBackrefInRegex",
        recommended: true,
    }
}

pub struct NoUselessBackrefInRegexState {
    node_ptr: JsSyntaxNode,
    backref: String,
}

impl Rule for NoUselessBackrefInRegex {
    type Query = Semantic<JsRegexLiteralExpression>;
    type State = NoUselessBackrefInRegexState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex_node = ctx.query();
        let regex_token = regex_node.value_token().ok()?;
        let regex_text = regex_token.text();

        let analyzer = RegexAnalyzer::new();
        if let Some(backref) = analyzer.find_useless_backref(regex_text) {
            return Some(NoUselessBackrefInRegexState {
                node_ptr: JsSyntaxNode::new(regex_node.syntax()),
                backref,
            });
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let regex_node = state.node_ptr.to_node(ctx.root());
        Some(RuleDiagnostic::new(
            rule_category!(),
            regex_node.text_range(),
            markup! {
                "Useless backreference "<Emphasis>{state.backref.clone()}</Emphasis>" in regular expression."
            },
        ).note(markup! {
            "This backreference refers to a non-existent capturing group."
        }))
    }
}

pub struct RegexAnalyzer {
    capturing_groups: HashSet<String>,
}

impl RegexAnalyzer {
    pub fn new() -> Self {
        RegexAnalyzer {
            capturing_groups: HashSet::new(),
        }
    }

    fn parse_capturing_groups(&mut self, regex_text: &str) {
        let hir = Parser::new().parse(regex_text).unwrap();
        self.visit_hir(&hir, 1);
    }

    fn visit_hir(&mut self, hir: &Hir, mut group_index: usize) {
        match hir.kind() {
            HirKind::Group(group) => {
                if let GroupKind::CaptureIndex(_) = group.kind {
                    self.capturing_groups.insert(group_index.to_string());
                    group_index += 1;
                }
                self.visit_hir(group.hir(), group_index);
            }
            HirKind::Concat(exprs) | HirKind::Alternation(exprs) => {
                for expr in exprs {
                    self.visit_hir(expr, group_index);
                }
            }
            _ => {}
        }
    }

    fn find_invalid_backrefs(&self, regex_text: &str) -> Option<String> {
        let hir = Parser::new().parse(regex_text).unwrap();
        self.find_backrefs_in_hir(&hir)
    }

    fn find_backrefs_in_hir(&self, hir: &Hir) -> Option<String> {
        match hir.kind() {
            HirKind::Backref(backref) => {
                let backref_str = backref.index().to_string();
                if !self.capturing_groups.contains(&backref_str) {
                    return Some(backref_str);
                }
            }
            HirKind::Group(group) => {
                if let Some(found) = self.find_backrefs_in_hir(group.hir()) {
                    return Some(found);
                }
            }
            HirKind::Concat(exprs) | HirKind::Alternation(exprs) => {
                for expr in exprs {
                    if let Some(found) = self.find_backrefs_in_hir(expr) {
                        return Some(found);
                    }
                }
            }
            _ => {}
        }
        None
    }
}
