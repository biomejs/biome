use crate::react::hooks::{is_react_hook_call, is_react_hook_name};
use crate::services::semantic::{SemanticModelBuilderVisitor, SemanticServices};
use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryMatch, Queryable, Rule, RuleDiagnostic, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, Visitor, VisitorContext, VisitorFinishContext,
    context::RuleContext, declare_lint_rule,
};
use biome_analyze::{RuleDomain, RuleSource};
use biome_console::markup;
use biome_js_semantic::{CallsExtensions, SemanticModel};
use biome_js_syntax::{
    AnyFunctionLike, AnyJsBinding, AnyJsClassMemberName, AnyJsExpression, AnyJsFunction,
    AnyJsObjectMemberName, JsArrayAssignmentPatternElement, JsArrayBindingPatternElement,
    JsCallExpression, JsConditionalExpression, JsGetterClassMember, JsGetterObjectMember,
    JsIfStatement, JsLanguage, JsLogicalExpression, JsMethodClassMember, JsMethodObjectMember,
    JsObjectBindingPatternShorthandProperty, JsReturnStatement, JsSetterClassMember,
    JsSetterObjectMember, JsSyntaxKind, JsSyntaxNode, JsTryFinallyStatement, TextRange,
};
use biome_rowan::{AstNode, Language, SyntaxNode, Text, WalkEvent, declare_node_union};
use rustc_hash::FxHashMap;
use std::ops::{Deref, DerefMut};

use crate::react::components::ReactComponentInfo;
use biome_diagnostics::Severity;
use biome_rule_options::use_hook_at_top_level::UseHookAtTopLevelOptions;

declare_lint_rule! {
    /// Enforce that all React hooks are being called from the Top Level component functions.
    ///
    /// _This rule should be used only in **React** projects._
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
    /// ### Valid
    ///
    /// ```js
    /// function Component1() {
    ///     useEffect();
    /// }
    /// ```
    ///
    pub UseHookAtTopLevel {
        version: "1.0.0",
        name: "useHookAtTopLevel",
        language: "jsx",
        sources: &[RuleSource::EslintReactHooks("rules-of-hooks").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::React, RuleDomain::Next],
    }
}

declare_node_union! {
    pub AnyJsFunctionOrMethod = AnyJsFunction | JsMethodClassMember | JsMethodObjectMember
}

impl AnyJsFunctionOrMethod {
    fn is_react_component_or_hook(&self) -> bool {
        if ReactComponentInfo::from_function(self.syntax()).is_some() {
            return true;
        }
        if let Some(name) = self.name() {
            return is_react_hook_name(&name);
        }

        false
    }

    fn name(&self) -> Option<Text> {
        match self {
            Self::AnyJsFunction(function) => function
                .binding()
                .as_ref()
                .map(AnyJsBinding::to_trimmed_text),
            Self::JsMethodClassMember(method) => method
                .name()
                .ok()
                .as_ref()
                .map(AnyJsClassMemberName::to_trimmed_text),
            Self::JsMethodObjectMember(method) => method
                .name()
                .ok()
                .as_ref()
                .map(AnyJsObjectMemberName::to_trimmed_text),
        }
    }
}

pub struct Suggestion {
    hook_name_range: TextRange,
    path: Vec<TextRange>,
    kind: SuggestionKind,
}

pub enum SuggestionKind {
    Regular,
    EarlyReturn(TextRange),
    Nested,
    Recursive,
}

/// Verifies whether the call expression is at the top level of the component,
/// and returns the function node if so.
fn enclosing_function_if_call_is_at_top_level(
    call: &JsCallExpression,
) -> Option<AnyJsFunctionOrMethod> {
    let mut prev_node = None;

    for node in call.syntax().ancestors() {
        match AnyJsFunctionOrMethod::try_cast(node) {
            Ok(enclosing_function) => {
                return Some(enclosing_function);
            }
            Err(node) => {
                if let Some(prev_node) = prev_node
                    && is_conditional_expression(&node, &prev_node)
                {
                    return None;
                }
                prev_node = Some(node);
            }
        }
    }

    None
}

/// Determines whether the given `node` is executed conditionally due to the
/// position it takes within its `parent_node`.
///
/// Returns `true` if and only if the parent node is a node that introduces a
/// condition that makes execution of `node` conditional.
///
/// Generally, this means that for conditional expressions, the "test" is
/// considered unconditional (since it is always evaluated), while the branches
/// are considered conditional.
///
/// For example:
///
/// ```js
///    testNode ? truthyNode : falsyNode
/// // ^^^^^^^^---------------------------- This node is always executed.
/// //            ^^^^^^^^^^---^^^^^^^^^--- These nodes are conditionally executed.
/// ```
fn is_conditional_expression(parent_node: &JsSyntaxNode, node: &JsSyntaxNode) -> bool {
    if let Some(assignment_with_default) = JsArrayAssignmentPatternElement::cast_ref(parent_node) {
        return assignment_with_default
            .init()
            .is_some_and(|default| default.syntax() == node);
    }

    if let Some(binding_pattern_with_default) = JsArrayBindingPatternElement::cast_ref(parent_node)
    {
        return binding_pattern_with_default
            .init()
            .is_some_and(|default| default.syntax() == node);
    }

    if let Some(conditional_expression) = JsConditionalExpression::cast_ref(parent_node) {
        return conditional_expression
            .test()
            .is_ok_and(|test| test.syntax() != node);
    }

    if let Some(if_statement) = JsIfStatement::cast_ref(parent_node) {
        return if_statement.test().is_ok_and(|test| test.syntax() != node);
    }

    if let Some(logical_expression) = JsLogicalExpression::cast_ref(parent_node) {
        return logical_expression
            .right()
            .is_ok_and(|right| right.syntax() == node);
    }

    if let Some(object_binding_pattern_shorthand_property) =
        JsObjectBindingPatternShorthandProperty::cast_ref(parent_node)
    {
        return object_binding_pattern_shorthand_property
            .init()
            .is_some_and(|init| init.syntax() == node);
    }

    if let Some(try_finally_statement) = JsTryFinallyStatement::cast_ref(parent_node) {
        // Code inside `try` statements is considered conditional, because a
        // thrown error is expected at any point, so there's no guarantee
        // whether code will run unconditionally. But we make an exception for
        // the `finally` clause since it does run unconditionally.
        //
        // Note: Of course code outside a `try` block may throw too, but then
        // the exception will bubble up and break the entire component, instead
        // of being merely a violation of the rules of hooks.
        return try_finally_statement
            .finally_clause()
            .is_ok_and(|finally_clause| finally_clause.syntax() != node);
    }

    // The following statement kinds are considered to always make their inner
    // nodes conditional:
    matches!(
        parent_node.kind(),
        JsSyntaxKind::JS_DO_WHILE_STATEMENT
            | JsSyntaxKind::JS_FOR_IN_STATEMENT
            | JsSyntaxKind::JS_FOR_OF_STATEMENT
            | JsSyntaxKind::JS_FOR_STATEMENT
            | JsSyntaxKind::JS_SWITCH_STATEMENT
            | JsSyntaxKind::JS_TRY_STATEMENT
            | JsSyntaxKind::JS_WHILE_STATEMENT
    )
}

fn is_nested_function_inside_component_or_hook(function: &AnyJsFunctionOrMethod) -> bool {
    if function.is_react_component_or_hook() {
        return false;
    }

    let Some(parent) = function.syntax().parent() else {
        return false;
    };

    parent.ancestors().any(|node| {
        AnyJsFunctionOrMethod::cast(node)
            .is_some_and(|enclosing_function| enclosing_function.is_react_component_or_hook())
    })
}

/// Model for tracking which function calls are preceded by an early return.
///
/// The keys in the model are call sites and each value is the text range of an
/// early return that precedes such call site. Call sites without preceding
/// early returns are not included in the model. For call sites that are
/// preceded by multiple early returns, the return statement that we map to is
/// implementation-defined.
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
                if AnyFunctionLike::can_cast(node.kind())
                    || AnyPropertyAccessor::can_cast(node.kind())
                {
                    self.stack
                        .push(EarlyReturnDetectionVisitorStackEntry::default());
                }
            }
            WalkEvent::Leave(node) => {
                if AnyFunctionLike::can_cast(node.kind())
                    || AnyPropertyAccessor::can_cast(node.kind())
                {
                    self.stack.pop();
                    return;
                }

                if let Some(entry) = self.stack.last_mut() {
                    if JsReturnStatement::can_cast(node.kind()) {
                        entry.early_return = Some(node.text_range_with_trivia());
                    } else if let Some(call) = JsCallExpression::cast_ref(node)
                        && let Some(early_return) = entry.early_return
                    {
                        self.early_returns.insert(call.clone(), early_return);
                    }
                }
            }
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        ctx.services.insert_service(self.early_returns);
    }
}

declare_node_union! {
    AnyPropertyAccessor =
        JsGetterObjectMember
        | JsSetterObjectMember
        | JsGetterClassMember
        | JsSetterClassMember
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
        rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let early_returns: &EarlyReturnsModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["EarlyReturnsModel"]))?;
        Ok(Self {
            early_returns: early_returns.clone(),
            semantic_services: SemanticServices::from_services(rule_key, rule_metadata, services)?,
        })
    }
}

impl Phase for FunctionCallServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

#[derive(Clone)]
pub struct FunctionCall(JsCallExpression);

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
    is_enclosed_in_component_or_hook: bool,
    path: Vec<TextRange>,
}

impl Rule for UseHookAtTopLevel {
    type Query = FunctionCall;
    type State = Suggestion;
    type Signals = Option<Self::State>;
    type Options = UseHookAtTopLevelOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let FunctionCall(call) = ctx.query();
        let get_hook_name_range = || match call.callee() {
            Ok(callee) => Some(AnyJsExpression::syntax(&callee).text_trimmed_range()),
            Err(_) => None,
        };

        // Early return for any function call that's not a hook call:
        if !is_react_hook_call(call) {
            return None;
        }

        let model = ctx.semantic_model();
        let early_returns = ctx.early_returns_model();

        let root = CallPath {
            call: call.clone(),
            path: vec![],
            is_enclosed_in_component_or_hook: false,
        };
        let mut calls = vec![root];

        while let Some(CallPath {
            call,
            mut path,
            is_enclosed_in_component_or_hook,
        }) = calls.pop()
        {
            let range = call.syntax().text_range_with_trivia();

            if path.contains(&range) {
                return Some(Suggestion {
                    hook_name_range: get_hook_name_range()?,
                    path,
                    kind: SuggestionKind::Recursive,
                });
            }

            path.push(range);

            if let Some(enclosing_function) = enclosing_function_if_call_is_at_top_level(&call) {
                if is_nested_function_inside_component_or_hook(&enclosing_function) {
                    // We cannot allow nested functions inside hooks and
                    // components, since it would break the requirement for
                    // hooks to be called from the top-level.
                    return Some(Suggestion {
                        hook_name_range: get_hook_name_range()?,
                        path,
                        kind: SuggestionKind::Nested,
                    });
                }

                if let Some(early_return) = early_returns.get(&call) {
                    return Some(Suggestion {
                        hook_name_range: get_hook_name_range()?,
                        path,
                        kind: SuggestionKind::EarlyReturn(*early_return),
                    });
                }

                let enclosed = is_enclosed_in_component_or_hook
                    || enclosing_function.is_react_component_or_hook();

                if let AnyJsFunctionOrMethod::AnyJsFunction(function) = enclosing_function
                    && let Some(calls_iter) = function.all_calls(model)
                {
                    for call in calls_iter {
                        calls.push(CallPath {
                            call: call.tree(),
                            path: path.clone(),
                            is_enclosed_in_component_or_hook: enclosed,
                        });
                    }
                }
            } else {
                // Avoid duplicate diagnostics if this path already passed through
                // a component/hook. We still keep previously enqueued paths to
                // allow recursion detection elsewhere.
                if is_enclosed_in_component_or_hook {
                    continue;
                }
                return Some(Suggestion {
                    hook_name_range: get_hook_name_range()?,
                    path,
                    kind: SuggestionKind::Regular,
                });
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, suggestion: &Self::State) -> Option<RuleDiagnostic> {
        let Suggestion {
            hook_name_range,
            path,
            kind,
        } = suggestion;

        let message = match &kind {
            SuggestionKind::Nested => markup! {
                "This hook is being called from a nested function, but all hooks must be called "
                "unconditionally from the top-level component."
            },
            SuggestionKind::Recursive => markup! { "This hook is being called recursively." },
            _ if path.len() <= 1 => markup! {
                "This hook is being called conditionally, but all hooks must be called in the "
                "exact same order in every component render."
            },
            _ => markup! {
                "This hook is being called indirectly and conditionally, but all hooks must be "
                "called in the exact same order in every component render."
            },
        };

        let mut diag = RuleDiagnostic::new(rule_category!(), hook_name_range, message);
        for (i, range) in path.iter().skip(1).enumerate() {
            let msg = if i == 0 {
                markup! { "This is the call path until the hook." }
            } else {
                markup! {}
            };

            diag = diag.detail(range, msg);
        }

        if let SuggestionKind::EarlyReturn(range) = &kind {
            diag = diag.detail(
                *range,
                markup! { "Hooks should not be called after an early return." },
            )
        }

        let mut diag = diag.note(markup! {
            "For React to preserve state between calls, hooks needs to be called "
            "unconditionally and always in the same order."
        });

        if matches!(kind, SuggestionKind::Recursive) {
            diag = diag.note(markup! {
                "This means recursive calls are not allowed, because they require a condition "
                "in order to terminate."
            });
        }

        let diag = diag.note(markup! {
            "See https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level"
        });
        Some(diag)
    }
}
