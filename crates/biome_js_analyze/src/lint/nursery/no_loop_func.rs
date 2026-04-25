use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::fmt::{Display, Formatter};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{
    Binding, Capture, CaptureType, ClosureExtensions, JsDeclarationKind, ReferencesExtensions,
    Scope, SemanticModel,
};
use biome_js_syntax::{
    AnyJsForInitializer, AnyJsVariableDeclaration, JsArrowFunctionExpression, JsCallExpression,
    JsDoWhileStatement, JsForInStatement, JsForOfStatement, JsForStatement, JsFunctionDeclaration,
    JsFunctionExpression, JsSyntaxKind, JsVariableKind, JsWhileStatement,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, SyntaxNodeCast, TextSize, TokenText, declare_node_union};
use biome_rule_options::no_loop_func::NoLoopFuncOptions;
use rustc_hash::FxHashSet;

declare_node_union! {
    pub AnyLoopFunction = JsFunctionDeclaration | JsFunctionExpression | JsArrowFunctionExpression
}

declare_lint_rule! {
    /// Disallow functions declared inside loops that capture unsafe outer variables.
    ///
    /// Functions created in loops can easily observe values from a later iteration instead of the
    /// iteration where they were created. This rule reports functions that capture outer bindings
    /// which may be reassigned while the loop continues.
    ///
    /// The rule ignores plain immediately invoked function expressions (IIFEs), but still reports
    /// async, generator, and self-referential IIFEs because they can escape the current iteration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// Using `var` for the iteration variable creates a single binding shared across all iterations, so it's unsafe to capture.
    ///
    /// ```js,expect_diagnostic
    /// for (var i = 0; i < 10; i++) {
    ///     handlers.push(() => i);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let value = 0;
    /// for (let i = 0; i < 10; i++) {
    ///     queue.push(function () {
    ///         return value;
    ///     });
    ///     value += 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// Using `let` or `const` for the iteration variable creates a fresh binding each iteration, so it's safe to capture.
    ///
    /// ```js
    /// for (let i = 0; i < 10; i++) {
    ///     handlers.push(() => i);
    /// }
    /// ```
    ///
    /// ```js
    /// for (var i = 0; i < 10; i++) {
    ///     const current = i;
    ///     queue.push(function() {
    ///         return current;
    ///     });
    /// }
    /// ```
    ///
    pub NoLoopFunc {
        version: "2.4.13",
        name: "noLoopFunc",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::Eslint("no-loop-func").same(),
            RuleSource::EslintTypeScript("no-loop-func").same(),
        ],
    }
}

#[derive(Debug)]
pub struct UnsafeCapture(TokenText);

impl Rule for NoLoopFunc {
    type Query = Semantic<AnyLoopFunction>;
    type State = Box<[UnsafeCapture]>;
    type Signals = Option<Self::State>;
    type Options = NoLoopFuncOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();
        let model = ctx.model();
        let loop_node = get_containing_loop_node(&AnyLoopBoundary::from(function.clone()), model)?;

        if is_skippable_iife(function, model) {
            return None;
        }

        let closure = match function {
            AnyLoopFunction::JsFunctionDeclaration(function) => function.closure(model),
            AnyLoopFunction::JsFunctionExpression(function) => function.closure(model),
            AnyLoopFunction::JsArrowFunctionExpression(function) => function.closure(model),
        };

        let mut seen = FxHashSet::default();
        let mut unsafe_captures = Vec::new();

        for capture in all_captures_in_closure(&closure) {
            if is_safe_capture(&loop_node, &capture, model) {
                continue;
            }

            let name = capture_name(&capture)?;
            if seen.insert(name.clone()) {
                unsafe_captures.push(UnsafeCapture(name));
            }
        }

        if !unsafe_captures.is_empty() {
            Some(unsafe_captures.into_boxed_slice())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let function = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                function.range(),
                markup! {
                    "This function declared in a loop contains unsafe references to outer variables."
                },
            )
                .note(markup! {
                    "Loop-created functions often run after the loop continues, but captured outer bindings like "<Emphasis>"var"</Emphasis>" variables are reused instead of copied per iteration."
                })
                .note(markup! {
                    "The following variables were detected: "<Emphasis>{CaptureList(state)}</Emphasis>
                })
                .note(markup! {
                    "Move the function outside the loop, capture only per-iteration bindings, or avoid mutating the captured variable across iterations."
                }),
        )
    }
}

struct CaptureList<'a>(&'a [UnsafeCapture]);

impl Display for CaptureList<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        for (index, capture) in self.0.iter().enumerate() {
            if index > 0 {
                fmt.write_markup(markup! { ", " })?;
            }
            fmt.write_markup(markup! {{capture.0.text()}})?;
        }
        Ok(())
    }
}

declare_node_union! {
    pub AnySkippableIife = JsFunctionExpression | JsArrowFunctionExpression
}

declare_node_union! {
    pub AnyLoopStatement = JsWhileStatement | JsDoWhileStatement | JsForStatement | JsForInStatement | JsForOfStatement
}

declare_node_union! {
    pub AnyLoopBoundary = AnyLoopFunction | AnyLoopStatement
}

/// Returns all the outer-variable captures that belong to a function.
///
/// A closure is a function that reads values from surrounding scopes. A capture is one of those
/// outside values. `noLoopFunc` tracks captures because they can be shared across loop iterations,
/// even when the read happens inside a nested closure returned by the current function.
///
/// ## Examples
///
/// ```js
/// for (var i = 0; i < 5; i++) {
///     queue.push(() => { // This outer arrow is the function reported by the rule.
///         return () => i; // The nested arrow captures `i` from the loop scope.
///     });
/// }
/// ```
///
/// Both arrows are closures because they read `i` from the loop, and `i` is the capture. This
/// helper keeps that capture attached to the outer arrow even though the actual read happens in the
/// nested arrow.
fn all_captures_in_closure(closure: &biome_js_semantic::Closure) -> impl Iterator<Item = Capture> {
    let range = closure.closure_range();
    closure
        .descendents()
        .flat_map(|closure| closure.all_captures())
        .filter(move |capture| {
            !range.contains(capture.declaration_range().start())
                && range.contains(capture.node().text_trimmed_range().start())
        })
}

/// Returns the loop that should be blamed for a function node.
///
/// This is not a plain ancestor lookup. The rule only treats the parts of a loop that run once per
/// iteration as "inside" the loop. A function in the initializer of `for (;;)`, or in the
/// right-hand side of `for..in` / `for..of`, runs before iteration starts and must not be
/// reported.
fn get_containing_loop_node(
    node: &AnyLoopBoundary,
    model: &SemanticModel,
) -> Option<AnyLoopStatement> {
    let mut current = node.syntax().clone();

    while let Some(parent) = current.parent() {
        if let Some(loop_statement) = AnyLoopStatement::cast(parent.clone()) {
            match loop_statement {
                AnyLoopStatement::JsWhileStatement(_) | AnyLoopStatement::JsDoWhileStatement(_) => {
                    return Some(loop_statement);
                }
                AnyLoopStatement::JsForStatement(for_statement) => {
                    // if we are walking up from the loop body, treat this loop as the containing loop.
                    // but if we are walking up from the initializer, the loop doesn't contain us because the initializer runs before the first iteration starts.
                    if for_statement
                        .initializer()
                        .is_none_or(|initializer: AnyJsForInitializer| {
                            initializer.syntax() != &current
                        })
                    {
                        return Some(AnyLoopStatement::JsForStatement(for_statement));
                    }
                }
                AnyLoopStatement::JsForInStatement(for_statement) => {
                    if for_statement.expression().ok()?.syntax() != &current {
                        return Some(AnyLoopStatement::JsForInStatement(for_statement));
                    }
                }
                AnyLoopStatement::JsForOfStatement(for_statement) => {
                    if for_statement.expression().ok()?.syntax() != &current {
                        return Some(AnyLoopStatement::JsForOfStatement(for_statement));
                    }
                }
            }
        } else {
            match parent.kind() {
                kind if crate::ast_utils::is_function_boundary(kind) => {
                    if let Some(function) = AnySkippableIife::cast(parent.clone())
                        && is_skippable_iife_syntax(&function, model)
                    {
                        current = parent;
                        continue;
                    }
                    return None;
                }
                _ => {}
            }
        }

        current = parent;
    }

    None
}

/// Returns the widest loop region that can still affect a captured binding.
///
/// `noLoopFunc` does not only care about the current loop node. When loops are nested, a write in
/// an outer loop can still change the value seen by the captured binding. The optional
/// `excluded_node` lets callers stop the expansion at a loop-local `let`, because that declaration
/// creates a fresh binding per iteration and should not be treated as shared.
///
/// ## Examples
///
/// ```js
/// for (var batch = 0; batch < batches.length; batch++) { // Writes in this outer loop still matter.
///     for (let i = 0; i < items.length; i++) { // This `let` creates a per-iteration binding.
///         queue.push(() => batch + i); // `batch` is shared, but `i` is not.
///     }
/// }
/// ```
fn get_top_loop_node(
    loop_node: &AnyLoopStatement,
    excluded_node: Option<&AnyJsVariableDeclaration>,
    model: &SemanticModel,
) -> AnyLoopStatement {
    let border = excluded_node.map_or(TextSize::from(0), |node| {
        node.syntax().text_trimmed_range().end()
    });
    let mut top_loop = loop_node.clone();
    let mut current_loop = loop_node.clone();

    while current_loop.syntax().text_trimmed_range().start() >= border {
        top_loop = current_loop.clone();
        let Some(containing_loop) =
            get_containing_loop_node(&AnyLoopBoundary::AnyLoopStatement(current_loop), model)
        else {
            break;
        };
        current_loop = containing_loop;
    }

    top_loop
}

/// Returns `true` when a captured binding cannot produce the stale-value bug this rule targets.
///
/// The business logic of `noLoopFunc` is here: a capture is safe if the function cannot observe a
/// different value by the time it runs. `const` and `using` bindings are safe because their value cannot be reassigned.
/// Loop-local `let` bindings are also safe because each iteration gets a fresh binding. For all
/// other cases, the helper checks whether the binding is ever written after the relevant loop scope
/// starts executing.
///
/// ## Examples
///
/// ```js
/// for (let i = 0; i < 10; i++) { // `let i` creates a fresh binding each iteration.
///     handlers.push(() => i); // Capturing `i` is safe here.
/// }
/// ```
fn is_safe_capture(loop_node: &AnyLoopStatement, capture: &Capture, model: &SemanticModel) -> bool {
    if matches!(capture.ty(), CaptureType::Type) {
        // Type-only captures do not exist at runtime, so they cannot observe stale values.
        return true;
    }

    let binding = capture.binding();
    if !binding.declaration_kind().declares_value() {
        // Non-value bindings cannot produce a runtime value that changes between iterations.
        return true;
    }

    if is_constant_binding(&binding) {
        // Immutable bindings keep the same value even if the function runs in a later iteration.
        return true;
    }

    let declaration = binding_declaration(&binding);
    if declaration.as_ref().is_some_and(|decl| {
        decl.is_let()
            // is the declaration inside the loop body? Outside declarations do not get fresh bindings per iteration, so they are not safe.
            && decl.range().start() > loop_node.syntax().text_trimmed_range().start()
            && decl.range().end() < loop_node.syntax().text_trimmed_range().end()
    }) {
        // A loop-local `let` gets a fresh binding each iteration instead of sharing one binding.
        return true;
    }

    let binding_variable_scope = variable_scope(binding.scope());
    let border = get_top_loop_node(
        loop_node,
        declaration.as_ref().filter(|decl| decl.is_let()),
        model,
    )
    .syntax()
    .text_trimmed_range()
    .start();

    // The capture is safe only if every write happened before the relevant loop region starts, or
    // if the write belongs to a different variable scope and therefore cannot affect this binding.
    binding.all_references().all(|reference| {
        !reference.is_write()
            || (variable_scope(reference.scope()) == binding_variable_scope
                && reference.range_start() < border)
    })
}

/// Returns the variable declaration that created a binding, when the binding comes from a
/// declaration the rule knows how to classify.
fn binding_declaration(binding: &Binding) -> Option<AnyJsVariableDeclaration> {
    let declaration = binding.tree().declaration()?;
    let declaration = declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration);
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return None;
    };

    declarator
        .syntax()
        .ancestors()
        .find_map(AnyJsVariableDeclaration::cast)
}

/// Returns `true` for declarations whose runtime value is stable for the lifetime of the binding. E.g. `const` and `using`.
fn is_constant_binding(binding: &Binding) -> bool {
    match binding.declaration_kind() {
        JsDeclarationKind::Using => true,
        JsDeclarationKind::Value => binding_declaration(binding)
            .is_some_and(|declaration| declaration.variable_kind() == Ok(JsVariableKind::Const)),
        _ => false,
    }
}

/// Returns the name that should be shown to the user for a reported capture.
fn capture_name(capture: &Capture) -> Option<TokenText> {
    capture
        .binding()
        .tree()
        .name_token()
        .ok()
        .map(|token| token.token_text_trimmed())
}

/// Returns the variable scope that determines whether a later write can affect the capture.
///
/// `var` writes are scoped to the nearest function or the global scope, not to the nearest block.
/// This helper normalizes bindings and references to that variable scope so `noLoopFunc` can tell
/// whether two writes target the same shared storage.
///
/// ## Examples
///
/// ```js
/// function outer() {
///     for (var i = 0; i < 10; i++) { // `var i` belongs to `outer`, not to the loop block.
///         queue.push(() => i); // Every iteration captures the same binding.
///     }
/// }
/// ```
///
/// The variable scope of `i` is the body of `outer`, which is why every iteration shares the same
/// binding.
fn variable_scope(scope: Scope) -> Scope {
    scope
        .ancestors()
        .find(|scope| scope.is_global_scope() || scope.closure().is_some())
        .unwrap_or(scope)
}

/// Returns `true` when a function is the kind of IIFE that `noLoopFunc` intentionally ignores.
///
/// Plain IIFEs execute immediately, so they do not usually suffer from the "run in a later
/// iteration" problem. But, we exclude async, generator, and self-referential function
/// expressions from that exemption because they can escape the current iteration or outlive the
/// immediate call.
///
/// ## Examples
///
/// ```js
/// for (var i = 0; i < 10; i++) {
///     (() => i)(); // Immediate call makes this a plain IIFE.
/// }
/// ```
fn is_skippable_iife(function: &AnyLoopFunction, model: &SemanticModel) -> bool {
    let is_iife = match function {
        AnyLoopFunction::JsFunctionExpression(function) => {
            is_iife(&AnySkippableIife::JsFunctionExpression(function.clone()))
        }
        AnyLoopFunction::JsArrowFunctionExpression(function) => is_iife(
            &AnySkippableIife::JsArrowFunctionExpression(function.clone()),
        ),
        AnyLoopFunction::JsFunctionDeclaration(_) => false,
    };

    if is_async_or_generator(function) || !is_iife {
        return false;
    }

    match function {
        AnyLoopFunction::JsFunctionExpression(function) => {
            !is_self_referential_function_expression(function, model)
        }
        AnyLoopFunction::JsArrowFunctionExpression(_) => true,
        AnyLoopFunction::JsFunctionDeclaration(_) => false,
    }
}

/// Returns `true` when an ancestor function node is a skippable IIFE boundary.
///
/// When `get_containing_loop_node` walks outward through parent functions, it normally stops at the
/// first function boundary. This helper encodes the exception for IIFEs that should be ignored so
/// the walk can keep searching for the surrounding loop.
///
/// ## Examples
///
/// ```js
/// for (var i = 0; i < 10; i++) {
///     (() => {
///         return () => i; // The nested arrow still captures the loop variable.
///     })(); // This wrapper IIFE is skipped while walking outward.
/// }
/// ```
///
/// The wrapper IIFE is ignored, allowing the nested arrow to still be associated with the loop.
fn is_skippable_iife_syntax(function: &AnySkippableIife, model: &SemanticModel) -> bool {
    match function {
        AnySkippableIife::JsFunctionExpression(function) => is_skippable_iife(
            &AnyLoopFunction::JsFunctionExpression(function.clone()),
            model,
        ),
        AnySkippableIife::JsArrowFunctionExpression(function) => is_skippable_iife(
            &AnyLoopFunction::JsArrowFunctionExpression(function.clone()),
            model,
        ),
    }
}

/// Returns `true` if a named function expression uses its own name inside the body.
///
/// Self-referential function expressions are not treated like harmless IIFEs. Referencing the
/// function by name can let the function value escape, which means the closure may still observe a
/// later loop value.
///
/// ## Examples
///
/// ```js
/// (function fun() {
///     queue.push(fun); // Referencing `fun` makes the IIFE self-referential.
/// })();
/// ```
///
/// This returns `true`, so the surrounding IIFE is not exempted.
fn is_self_referential_function_expression(
    function: &JsFunctionExpression,
    model: &SemanticModel,
) -> bool {
    function
        .id()
        .and_then(|binding| binding.as_js_identifier_binding().cloned())
        .is_some_and(|binding| binding.all_references(model).next().is_some())
}

/// Returns `true` if a function node is called immediately, even when wrapped in parentheses.
///
/// The IIFE exemption is a semantic question rather than a syntax-kind question. This helper walks
/// out through parentheses until it can see whether the function is the callee of a call
/// expression.
///
/// ## Examples
///
/// ```js
/// (() => value)(); // The call expression makes the arrow an IIFE.
/// ```
fn is_iife(function: &AnySkippableIife) -> bool {
    let mut current = function.syntax().clone();

    while let Some(parent) = current.parent() {
        if parent.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
            current = parent;
            continue;
        }

        if let Some(call) = parent.clone().cast::<JsCallExpression>() {
            return call
                .callee()
                .ok()
                .is_some_and(|callee| callee.syntax() == &current);
        }

        return false;
    }

    false
}

/// Returns `true` for function shapes that are not eligible for the plain-IIFE exemption.
///
/// Async and generator functions can suspend or escape the current turn of execution, so they are
/// treated like ordinary loop-created closures even when called immediately.
///
/// ## Examples
///
/// ```js
/// (async () => value)(); // `async` prevents the plain-IIFE exemption.
/// ```
fn is_async_or_generator(function: &AnyLoopFunction) -> bool {
    match function {
        AnyLoopFunction::JsFunctionDeclaration(function) => {
            function.async_token().is_some() || function.star_token().is_some()
        }
        AnyLoopFunction::JsFunctionExpression(function) => {
            function.async_token().is_some() || function.star_token().is_some()
        }
        AnyLoopFunction::JsArrowFunctionExpression(function) => function.async_token().is_some(),
    }
}
