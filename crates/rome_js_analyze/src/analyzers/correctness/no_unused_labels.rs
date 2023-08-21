use rome_analyze::context::RuleContext;
use rome_analyze::{
    declare_rule, ActionCategory, AddVisitor, Phases, QueryMatch, Queryable, Rule, RuleDiagnostic,
    ServiceBag, Visitor, VisitorContext,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    JsBreakStatement, JsContinueStatement, JsLabeledStatement, JsLanguage, TextRange, WalkEvent,
};

use rome_rowan::{AstNode, BatchMutationExt, Language, NodeOrToken, SyntaxNode};
use rustc_hash::FxHashMap;

use crate::control_flow::AnyJsControlFlowRoot;
use crate::JsRuleAction;

declare_rule! {
    /// Disallow unused labels.
    ///
    /// Labels that are declared and never used are most likely an error due to incomplete refactoring.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-unused-labels
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```cjs,expect_diagnostic
    /// LOOP: for (const x of xs) {
    ///     if (x > 0) {
    ///         break;
    ///     }
    ///     f(x);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```cjs
    /// LOOP: for (const x of xs) {
    ///     if (x > 0) {
    ///         break LOOP;
    ///     }
    ///     f(x);
    /// }
    /// ```
    ///
    pub(crate) NoUnusedLabels {
        version: "1.0.0",
        name: "noUnusedLabels",
        recommended: true,
    }
}

#[derive(Default)]
struct UnusedLabelVisitor {
    root_id: usize,
    // Key = (root_id, label)
    labels: FxHashMap<(usize, String), JsLabeledStatement>,
}

impl UnusedLabelVisitor {
    fn insert(&mut self, label: String, label_stmt: JsLabeledStatement) {
        self.labels.insert((self.root_id, label), label_stmt);
    }

    fn remove(&mut self, label: String) -> Option<JsLabeledStatement> {
        self.labels.remove(&(self.root_id, label))
    }
}

impl Visitor for UnusedLabelVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                if AnyJsControlFlowRoot::cast_ref(node).is_some() {
                    self.root_id += 1;
                } else if let Some(label_stmt) = JsLabeledStatement::cast_ref(node) {
                    if let Ok(label_tok) = label_stmt.label_token() {
                        self.insert(label_tok.text_trimmed().to_owned(), label_stmt);
                    }
                } else if let Some(break_stmt) = JsBreakStatement::cast_ref(node) {
                    if let Some(label_tok) = break_stmt.label_token() {
                        self.remove(label_tok.text_trimmed().to_owned());
                    }
                } else if let Some(continue_stmt) = JsContinueStatement::cast_ref(node) {
                    if let Some(label_tok) = continue_stmt.label_token() {
                        self.remove(label_tok.text_trimmed().to_owned());
                    }
                }
            }
            WalkEvent::Leave(node) => {
                if AnyJsControlFlowRoot::cast_ref(node).is_some() {
                    self.root_id -= 1;
                } else if let Some(stmt_label) = JsLabeledStatement::cast_ref(node) {
                    if let Ok(label_tok) = stmt_label.label_token() {
                        let result = self.remove(label_tok.text_trimmed().to_owned());
                        if let Some(label_stmt) = result {
                            ctx.match_query(UnusedLabel(label_stmt));
                        }
                    }
                }
            }
        }
    }
}

pub(crate) struct UnusedLabel(JsLabeledStatement);

impl QueryMatch for UnusedLabel {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for UnusedLabel {
    type Input = Self;
    type Language = JsLanguage;
    type Output = JsLabeledStatement;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, UnusedLabelVisitor::default);
    }

    // Extract the output object from the input type
    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.0.clone()
    }
}

impl Rule for NoUnusedLabels {
    type Query = UnusedLabel;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Option<Self::State> {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let unused_label = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            unused_label.label_token().ok()?.text_trimmed_range(),
            markup! {
                "Unused "<Emphasis>"label"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let unused_label = ctx.query();
        let body = unused_label.body().ok()?;
        let mut mutation = ctx.root().begin();
        mutation.replace_element(
            NodeOrToken::Node(unused_label.syntax().to_owned()),
            NodeOrToken::Node(body.syntax().to_owned()),
        );
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! {"Remove the unused "<Emphasis>"label"</Emphasis>"."}.to_owned(),
            mutation,
        })
    }
}
