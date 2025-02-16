use biome_analyze::context::RuleContext;
use biome_analyze::{
    declare_lint_rule, AddVisitor, FixKind, Phases, QueryMatch, Queryable, Rule, RuleDiagnostic,
    RuleSource, ServiceBag, Visitor, VisitorContext,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsStatement, JsBreakStatement, JsContinueStatement, JsFileSource, JsLabeledStatement,
    JsLanguage, TextRange, WalkEvent,
};

use biome_rowan::{AstNode, BatchMutationExt, Language, SyntaxNode, SyntaxResult, TokenText};
use rustc_hash::FxHashSet;

use crate::services::control_flow::AnyJsControlFlowRoot;
use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unused labels.
    ///
    /// Labels that are declared and never used are most likely an error due to incomplete refactoring.
    ///
    /// The rule ignores reactive Svelte statements in Svelte components.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
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
    /// ```js
    /// LOOP: for (const x of xs) {
    ///     if (x > 0) {
    ///         break LOOP;
    ///     }
    ///     f(x);
    /// }
    /// ```
    ///
    /// ```js
    /// function nonNegative(n) {
    ///     DEV: assert(n >= 0);
    ///     return n;
    /// }
    /// ```
    ///
    /// ```svelte
    /// <script>
    /// $: { /* reactive block */ }
    /// </script>
    /// ```
    pub NoUnusedLabels {
        version: "1.0.0",
        name: "noUnusedLabels",
        language: "js",
        sources: &[RuleSource::Eslint("no-unused-labels")],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

#[derive(Default)]
struct UnusedLabelVisitor {
    root_id: u32,
    // Key = (root_id, label)
    labels: FxHashSet<(u32, TokenText)>,
}

impl UnusedLabelVisitor {
    fn insert(&mut self, label: TokenText) {
        self.labels.insert((self.root_id, label));
    }

    fn remove(&mut self, label: TokenText) -> bool {
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
                if AnyJsControlFlowRoot::can_cast(node.kind()) {
                    self.root_id += 1;
                } else if let Some(label_stmt) = JsLabeledStatement::cast_ref(node) {
                    // Ignore unbreakable statements.
                    // It is sometimes use to mark debug-only statements.
                    if is_breakable_labeled_statement(&label_stmt.body()) {
                        if let Ok(label_tok) = label_stmt.label_token() {
                            self.insert(label_tok.token_text_trimmed());
                        }
                    }
                } else if let Some(break_stmt) = JsBreakStatement::cast_ref(node) {
                    if let Some(label_tok) = break_stmt.label_token() {
                        self.remove(label_tok.token_text_trimmed());
                    }
                } else if let Some(continue_stmt) = JsContinueStatement::cast_ref(node) {
                    if let Some(label_tok) = continue_stmt.label_token() {
                        self.remove(label_tok.token_text_trimmed());
                    }
                }
            }
            WalkEvent::Leave(node) => {
                if AnyJsControlFlowRoot::can_cast(node.kind()) {
                    self.root_id -= 1;
                } else if let Some(label_stmt) = JsLabeledStatement::cast_ref(node) {
                    if let Ok(label_tok) = label_stmt.label_token() {
                        if self.remove(label_tok.token_text_trimmed()) {
                            ctx.match_query(UnusedLabel(label_stmt));
                        }
                    }
                }
            }
        }
    }
}

pub struct UnusedLabel(JsLabeledStatement);

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

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let label = ctx.query().label_token().ok()?;
        let label = label.text_trimmed();
        if label == "$"
            && ctx
                .source_type::<JsFileSource>()
                .as_embedding_kind()
                .is_svelte()
        {
            return None;
        }
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
        ).note(markup!{
            "The label is not used by any "<Emphasis>"break"</Emphasis>" statement and continue statement."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let unused_label = ctx.query();
        let body = unused_label.body().ok()?;
        let mut mutation = ctx.root().begin();
        mutation.replace_node(unused_label.clone().into(), body);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {"Remove the unused "<Emphasis>"label"</Emphasis>"."}.to_owned(),
            mutation,
        ))
    }
}

fn is_breakable_labeled_statement(stmt: &SyntaxResult<AnyJsStatement>) -> bool {
    matches!(
        stmt,
        Ok(AnyJsStatement::JsBlockStatement(_)
            | AnyJsStatement::JsDoWhileStatement(_)
            | AnyJsStatement::JsForInStatement(_)
            | AnyJsStatement::JsForOfStatement(_)
            | AnyJsStatement::JsForStatement(_)
            | AnyJsStatement::JsIfStatement(_)
            | AnyJsStatement::JsSwitchStatement(_)
            | AnyJsStatement::JsTryFinallyStatement(_)
            | AnyJsStatement::JsTryStatement(_)
            | AnyJsStatement::JsWhileStatement(_)
            | AnyJsStatement::JsWithStatement(_))
    )
}
