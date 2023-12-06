use crate::react::hooks::react_hook_configuration;
use crate::semantic_analyzers::correctness::use_exhaustive_dependencies::{
    HooksOptions, ReactExtensiveDependenciesOptions,
};
use crate::semantic_services::{SemanticModelBuilderVisitor, SemanticServices};
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryMatch, Queryable,
    RuleKey, ServiceBag, Visitor, VisitorContext, VisitorFinishContext,
};
use biome_console::markup;
use biome_js_semantic::{CallsExtensions, SemanticModel};
use biome_js_syntax::{
    AnyFunctionLike, AnyJsFunction, JsCallExpression, JsFunctionBody, JsLanguage,
    JsReturnStatement, JsSyntaxKind, TextRange,
};
use biome_rowan::{AstNode, Language, SyntaxNode, WalkEvent};
use rustc_hash::FxHashMap;
use std::ops::{Deref, DerefMut};

declare_rule! {
    /// Enforce that all React hooks are being called from the Top Level component functions.
    ///
    /// To understand why this required see https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function Component1({ a }) {
    ///     if (a == 1) {
    ///         useEffect();
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function Component1({ a }) {
    ///     if (a != 1) {
    ///         return;
    ///     }
    ///
    ///     useEffect();
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// function Component1() {
    ///     useEffect();
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// Allows to specify custom hooks - from libraries or internal projects - that can be considered stable.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "hooks": [
    ///             { "name": "useLocation", "closureIndex": 0, "dependenciesIndex": 1},
    ///             { "name": "useQuery", "closureIndex": 1, "dependenciesIndex": 0}
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// Given the previous example, your hooks be used like this:
    ///
    /// ```js
    /// function Foo() {
    ///     const location = useLocation(() => {}, []);
    ///     const query = useQuery([], () => {});
    /// }
    /// ```
    ///
    pub(crate) UseHookAtTopLevel {
        version: "1.0.0",
        name: "useHookAtTopLevel",
        recommended: false,
    }
}

pub enum Suggestion {
    None {
        hook_name_range: TextRange,
        path: Vec<TextRange>,
        early_return: Option<TextRange>,
    },
}

// Verify if the call expression is at the top level
// of the component
fn enclosing_function_if_call_is_at_top_level(call: &JsCallExpression) -> Option<AnyJsFunction> {
    let next = call.syntax().ancestors().find(|x| {
        !matches!(
            x.kind(),
            JsSyntaxKind::JS_STATEMENT_LIST
                | JsSyntaxKind::JS_BLOCK_STATEMENT
                | JsSyntaxKind::JS_VARIABLE_STATEMENT
                | JsSyntaxKind::JS_EXPRESSION_STATEMENT
                | JsSyntaxKind::JS_RETURN_STATEMENT
                | JsSyntaxKind::JS_CALL_EXPRESSION
                | JsSyntaxKind::JS_CALL_ARGUMENT_LIST
                | JsSyntaxKind::JS_CALL_ARGUMENTS
                | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                | JsSyntaxKind::JS_INITIALIZER_CLAUSE
                | JsSyntaxKind::JS_VARIABLE_DECLARATOR
                | JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST
                | JsSyntaxKind::JS_VARIABLE_DECLARATION
                | JsSyntaxKind::TS_AS_EXPRESSION
                | JsSyntaxKind::TS_SATISFIES_EXPRESSION
        )
    });

    next.and_then(JsFunctionBody::cast)
        .and_then(|body| body.parent::<AnyJsFunction>())
}

/// Model for tracking which function calls are preceeded by an early return.
#[derive(Clone, Default)]
struct EarlyReturnsModel(FxHashMap<JsCallExpression, TextRange>);

impl Deref for EarlyReturnsModel {
    type Target = FxHashMap<JsCallExpression, TextRange>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EarlyReturnsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
struct EarlyReturnDetectionVisitor {
    early_returns: EarlyReturnsModel,
    stack: Vec<EarlyReturnDetectionVisitorStackEntry>,
}

#[derive(Default)]
struct EarlyReturnDetectionVisitorStackEntry {
    early_return: Option<TextRange>,
}

impl Visitor for EarlyReturnDetectionVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        _ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                if AnyFunctionLike::can_cast(node.kind()) {
                    self.stack
                        .push(EarlyReturnDetectionVisitorStackEntry::default());
                }
            }
            WalkEvent::Leave(node) => {
                if AnyFunctionLike::can_cast(node.kind()) {
                    self.stack.pop();
                    return;
                }

                if let Some(entry) = self.stack.last_mut() {
                    if JsReturnStatement::can_cast(node.kind()) {
                        entry.early_return = Some(node.text_range());
                    } else if let Some(call) = JsCallExpression::cast_ref(node) {
                        if let Some(early_return) = entry.early_return {
                            self.early_returns.insert(call.clone(), early_return);
                        }
                    }
                }
            }
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        ctx.services.insert_service(self.early_returns);
    }
}

#[derive(Default)]
struct FunctionCallVisitor;

impl Visitor for FunctionCallVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(_) => {}
            WalkEvent::Leave(node) => {
                if let Some(call) = JsCallExpression::cast_ref(node) {
                    ctx.match_query(FunctionCall(call));
                }
            }
        }
    }
}

pub struct FunctionCallServices {
    early_returns: EarlyReturnsModel,
    semantic_services: SemanticServices,
}

impl FunctionCallServices {
    fn early_returns_model(&self) -> &EarlyReturnsModel {
        &self.early_returns
    }

    fn semantic_model(&self) -> &SemanticModel {
        self.semantic_services.model()
    }
}

impl FromServices for FunctionCallServices {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let early_returns: &EarlyReturnsModel = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["EarlyReturnsModel"])
        })?;
        Ok(Self {
            early_returns: early_returns.clone(),
            semantic_services: SemanticServices::from_services(rule_key, services)?,
        })
    }
}

impl Phase for FunctionCallServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

#[derive(Clone)]
pub(crate) struct FunctionCall(JsCallExpression);

impl QueryMatch for FunctionCall {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for FunctionCall {
    type Input = Self;
    type Language = JsLanguage;
    type Output = Self;
    type Services = FunctionCallServices;

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        root: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Syntax, EarlyReturnDetectionVisitor::default);
        analyzer.add_visitor(Phases::Semantic, FunctionCallVisitor::default);
    }

    fn unwrap_match(_services: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.clone()
    }
}

#[derive(Debug)]
pub struct CallPath {
    call: JsCallExpression,
    path: Vec<TextRange>,
}

impl Rule for UseHookAtTopLevel {
    type Query = FunctionCall;
    type State = Suggestion;
    type Signals = Option<Self::State>;
    type Options = HooksOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let options = ReactExtensiveDependenciesOptions::new(options.clone());

        let FunctionCall(call) = ctx.query();
        let hook_name_range = call.callee().ok()?.syntax().text_trimmed_range();
        if react_hook_configuration(call, &options.hooks_config).is_some() {
            let model = ctx.semantic_model();
            let early_returns = ctx.early_returns_model();

            let root = CallPath {
                call: call.clone(),
                path: vec![],
            };
            let mut calls = vec![root];

            while let Some(CallPath { call, path }) = calls.pop() {
                let range = call.syntax().text_range();

                let mut path = path.clone();
                path.push(range);

                if let Some(enclosing_function) = enclosing_function_if_call_is_at_top_level(&call)
                {
                    if let Some(early_return) = early_returns.get(&call) {
                        return Some(Suggestion::None {
                            hook_name_range,
                            path,
                            early_return: Some(*early_return),
                        });
                    }

                    if let Some(calls_iter) = enclosing_function.all_calls(model) {
                        for call in calls_iter {
                            calls.push(CallPath {
                                call: call.tree(),
                                path: path.clone(),
                            });
                        }
                    }
                } else {
                    return Some(Suggestion::None {
                        hook_name_range,
                        path,
                        early_return: None,
                    });
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, suggestion: &Self::State) -> Option<RuleDiagnostic> {
        match suggestion {
            Suggestion::None {
                hook_name_range,
                path,
                early_return,
            } => {
                let call_deep = path.len() - 1;

                let mut diag = if call_deep == 0 {
                    RuleDiagnostic::new(
                        rule_category!(),
                        hook_name_range,
                        markup! {
                            "This hook is being called conditionally, but all hooks must be called in the exact same order in every component render."
                        },
                    )
                } else {
                    RuleDiagnostic::new(
                        rule_category!(),
                        hook_name_range,
                        markup! {
                            "This hook is being called indirectly and conditionally, but all hooks must be called in the exact same order in every component render."
                        },
                    )
                };

                for (i, range) in path.iter().skip(1).enumerate() {
                    let msg = if i == 0 {
                        markup! {
                            "This is the call path until the hook."
                        }
                    } else {
                        markup! {}
                    };

                    diag = diag.detail(range, msg);
                }

                if let Some(range) = early_return {
                    diag = diag.detail(
                        range,
                        markup! { "Hooks should not be called after an early return." },
                    )
                }

                let diag = diag.note(
                    markup! {
                        "For React to preserve state between calls, hooks needs to be called unconditionally and always in the same order."
                    },
                ).note(
                    markup! {
                        "See https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level"
                    },
                );
                Some(diag)
            }
        }
    }
}
