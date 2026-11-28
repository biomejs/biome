use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsClass, AnyJsClassMember, AnyJsExpression, AnyJsFunction, AnyJsIdentifierReference,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsComputedMemberName,
    JsConstructorClassMember, JsDecorator, JsExport, JsExportDefaultDeclarationClause,
    JsExportDefaultExpressionClause, JsFunctionDeclaration, JsFunctionExportDefaultDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsObjectExpression, JsObjectMemberList,
    JsParenthesizedExpression, JsPropertyObjectMember, JsSetterClassMember, JsSetterObjectMember,
    JsStaticMemberExpression, JsSuperExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
    JsSyntaxTrivia, JsThisExpression, JsVariableDeclaration, JsVariableDeclarationClause,
    JsVariableDeclarator, JsVariableDeclaratorList, JsVariableStatement,
    declaration_ext::is_in_ambient_context,
};
use biome_rowan::{
    AstNode, AstSeparatedList, SyntaxKindSet, SyntaxTriviaPieceComments, TextRange, WalkEvent,
    declare_node_union,
};
use biome_rule_options::no_outer_scope_in_isolated_functions::NoOuterScopeInIsolatedFunctionsOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow using variables, `this`, and `super` from outside an isolated function.
    ///
    /// Some functions don't run where they're written. A library can convert a function
    /// to text and run that text somewhere else, such as in a worker, in another process,
    /// or inside a browser page. A function that runs this way is called an isolated function.
    /// Variables declared outside an isolated function don't exist where it runs,
    /// so using one throws an error. For the same reason, `this` and `super` don't refer
    /// to the object or class around the function.
    ///
    /// The rule treats these functions as isolated:
    ///
    /// - Functions passed to `makeSynchronous()` or `workerize()`.
    /// - The first function passed to `browser.execute()` or `page.evaluate()`.
    /// - The `func` function passed to `chrome.scripting.executeScript()` or `browser.scripting.executeScript()`.
    /// - Functions, variables holding a function, and object or class members (methods, getters,
    ///   setters, and constructors) that have an `@isolated` comment right before them. The comment can contain only the tag, or the tag followed by
    ///   ` - ` and an explanation, such as `// @isolated - runs in a worker`.
    ///
    /// The rule only checks variables declared in the same file, including imports.
    /// It ignores global variables, such as `Array` or `fetch`, types,
    /// and variables declared with TypeScript's `declare` keyword.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import makeSynchronous from "make-synchronous";
    ///
    /// const url = "https://example.com";
    ///
    /// const getText = makeSynchronous(async () => {
    ///     const response = await fetch(url);
    ///     return response.text();
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const greeting = "hi";
    ///
    /// /** @isolated */
    /// function greet() {
    ///     return greeting.toUpperCase();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const counter = {
    ///     count: 0,
    ///     /** @isolated */
    ///     increment() {
    ///         return this.count + 1;
    ///     },
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import makeSynchronous from "make-synchronous";
    ///
    /// const getText = makeSynchronous(async () => {
    ///     const url = "https://example.com";
    ///     const response = await fetch(url);
    ///     return response.text();
    /// });
    ///
    /// const getTextFrom = makeSynchronous(async (url) => {
    ///     const response = await fetch(url);
    ///     return response.text();
    /// });
    ///
    /// /** @isolated */
    /// function greet(greeting) {
    ///     return greeting.toUpperCase();
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you use Playwright and only want to check the functions passed to `page.evaluate()`, see [`noPlaywrightUnsafeReferences`](https://biomejs.dev/linter/rules/no-playwright-unsafe-references/).
    ///
    pub NoOuterScopeInIsolatedFunctions {
        version: "2.6.10",
        name: "noOuterScopeInIsolatedFunctions",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("isolated-functions").inspired()],
        recommended: false,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub AnyIsolatedFunctionCandidate =
        AnyJsFunction
        | JsMethodObjectMember
        | JsGetterObjectMember
        | JsSetterObjectMember
        | JsMethodClassMember
        | JsGetterClassMember
        | JsSetterClassMember
        | JsConstructorClassMember
}

impl Rule for NoOuterScopeInIsolatedFunctions {
    type Query = Semantic<AnyIsolatedFunctionCandidate>;
    type State = OuterScopeUse;
    type Signals = Box<[Self::State]>;
    type Options = NoOuterScopeInIsolatedFunctionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();
        let Some(reason) = isolation_reason(function) else {
            return Box::default();
        };
        find_outer_scope_uses(ctx.model(), function, reason)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match &state.kind {
            OuterScopeUseKind::Variable {
                reference,
                declaration,
            } => {
                let name = reference.value_token().ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    reference.range(),
                    markup! {
                        "The variable "<Emphasis>{name.text_trimmed()}</Emphasis>" is declared outside of this isolated function."
                    },
                )
                .detail(
                    declaration,
                    markup! { "The variable is declared here." },
                )
            }
            OuterScopeUseKind::Context(expression) => RuleDiagnostic::new(
                rule_category!(),
                expression.range(),
                markup! {
                    <Emphasis>{ContextKeyword(expression)}</Emphasis>" is used inside an isolated function."
                },
            ),
        };
        let diagnostic = match state.reason {
            IsolationReason::Comment => diagnostic.note(markup! {
                "The "<Emphasis>"@isolated"</Emphasis>" comment marks this function as one that runs apart from the code around it."
            }),
            IsolationReason::Api(api) if api.takes_func_option() => diagnostic.note(markup! {
                "The "<Emphasis>"func"</Emphasis>" function passed to "<Emphasis>{api}</Emphasis>" runs apart from the code around it."
            }),
            IsolationReason::Api(api) => diagnostic.note(markup! {
                "Functions passed to "<Emphasis>{api}</Emphasis>" run apart from the code around them."
            }),
        };
        Some(match &state.kind {
            OuterScopeUseKind::Variable { .. } => diagnostic
                .note(markup! {
                    "Variables declared outside of the function don't exist where it runs, so using one throws an error."
                })
                .note(markup! {
                    "Declare the variable inside the function, or pass its value to the function as an argument."
                }),
            OuterScopeUseKind::Context(expression) => diagnostic
                .note(markup! {
                    "Where the function runs, "<Emphasis>{ContextKeyword(expression)}</Emphasis>" doesn't refer to the object or class around it."
                })
                .note(markup! {
                    "Pass the values the function needs as arguments instead."
                }),
        })
    }
}

/// Reports a use, inside an isolated function, of something the function can't access
/// where it runs.
pub struct OuterScopeUse {
    reason: IsolationReason,
    kind: OuterScopeUseKind,
}

pub enum OuterScopeUseKind {
    /// A reference to a variable whose declaration, at `declaration`, is outside the function.
    Variable {
        reference: AnyJsIdentifierReference,
        declaration: TextRange,
    },
    /// A `this` or `super` expression that gets its value from the isolated function.
    Context(AnyJsContextExpression),
}

declare_node_union! {
    pub AnyJsContextExpression = JsThisExpression | JsSuperExpression
}

/// Explains why the rule treats a function as isolated.
#[derive(Clone, Copy)]
enum IsolationReason {
    /// The function follows an `@isolated` comment.
    Comment,
    /// The function is passed to an API that runs it apart from the surrounding code.
    Api(IsolatingApi),
}

#[derive(Clone, Copy)]
enum IsolatingApi {
    MakeSynchronous,
    Workerize,
    BrowserExecute,
    PageEvaluate,
    ChromeScriptingExecuteScript,
    BrowserScriptingExecuteScript,
}

impl IsolatingApi {
    /// Returns `true` when the API receives the function as the `func` property of an object.
    const fn takes_func_option(self) -> bool {
        matches!(
            self,
            Self::ChromeScriptingExecuteScript | Self::BrowserScriptingExecuteScript
        )
    }
}

impl biome_console::fmt::Display for IsolatingApi {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::MakeSynchronous => "makeSynchronous()",
            Self::Workerize => "workerize()",
            Self::BrowserExecute => "browser.execute()",
            Self::PageEvaluate => "page.evaluate()",
            Self::ChromeScriptingExecuteScript => "chrome.scripting.executeScript()",
            Self::BrowserScriptingExecuteScript => "browser.scripting.executeScript()",
        })
    }
}

/// Writes `this` or `super`, depending on the expression.
struct ContextKeyword<'a>(&'a AnyJsContextExpression);

impl biome_console::fmt::Display for ContextKeyword<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self.0 {
            AnyJsContextExpression::JsThisExpression(_) => "this",
            AnyJsContextExpression::JsSuperExpression(_) => "super",
        })
    }
}

/// Returns why `function` is isolated, or `None` when it isn't.
fn isolation_reason(function: &AnyIsolatedFunctionCandidate) -> Option<IsolationReason> {
    if has_isolated_comment(function) {
        return Some(IsolationReason::Comment);
    }
    let api = match function {
        AnyIsolatedFunctionCandidate::AnyJsFunction(
            AnyJsFunction::JsArrowFunctionExpression(_) | AnyJsFunction::JsFunctionExpression(_),
        ) => {
            let expression = AnyJsExpression::cast_ref(function.syntax())?.outer_expression()?;
            if let Some(member) = expression.parent::<JsPropertyObjectMember>() {
                execute_script_api_of_func_member(member.name().ok()?.name()?.text(), &member)
            } else {
                argument_api(&expression)
            }
        }
        AnyIsolatedFunctionCandidate::JsMethodObjectMember(method) => {
            execute_script_api_of_func_member(method.name().ok()?.name()?.text(), method)
        }
        _ => None,
    }?;
    Some(IsolationReason::Api(api))
}

/// Returns the API that receives `argument` and isolates it, if any.
fn argument_api(argument: &AnyJsExpression) -> Option<IsolatingApi> {
    let (call, is_first) = call_of_argument(argument)?;
    match call.callee().ok()?.omit_parentheses() {
        AnyJsExpression::JsIdentifierExpression(callee) => {
            match callee.name().ok()?.value_token().ok()?.text_trimmed() {
                "makeSynchronous" => Some(IsolatingApi::MakeSynchronous),
                "workerize" => Some(IsolatingApi::Workerize),
                _ => None,
            }
        }
        AnyJsExpression::JsStaticMemberExpression(callee) if is_first => {
            let object = static_member_object_name(&callee)?;
            let member = callee.member().ok()?;
            match (
                object.text_trimmed(),
                member.as_js_name()?.value_token().ok()?.text_trimmed(),
            ) {
                ("browser", "execute") => Some(IsolatingApi::BrowserExecute),
                ("page", "evaluate") => Some(IsolatingApi::PageEvaluate),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Returns the `executeScript()` API when `member` is a `func` property of the object
/// passed as the first argument of `chrome.scripting.executeScript()` or
/// `browser.scripting.executeScript()`.
fn execute_script_api_of_func_member(
    name: &str,
    member: &impl AstNode<Language = JsLanguage>,
) -> Option<IsolatingApi> {
    if name != "func" {
        return None;
    }
    let object = member
        .parent::<JsObjectMemberList>()?
        .parent::<JsObjectExpression>()?;
    let object = AnyJsExpression::JsObjectExpression(object).outer_expression()?;
    let (call, true) = call_of_argument(&object)? else {
        return None;
    };
    let callee = call.callee().ok()?.omit_parentheses();
    let callee = callee.as_js_static_member_expression()?;
    if callee
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?
        .text_trimmed()
        != "executeScript"
    {
        return None;
    }
    let scripting = callee.object().ok()?.omit_parentheses();
    let scripting = scripting.as_js_static_member_expression()?;
    if scripting
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?
        .text_trimmed()
        != "scripting"
    {
        return None;
    }
    match static_member_object_name(scripting)?.text_trimmed() {
        "chrome" => Some(IsolatingApi::ChromeScriptingExecuteScript),
        "browser" => Some(IsolatingApi::BrowserScriptingExecuteScript),
        _ => None,
    }
}

/// Returns the call that receives `argument`, and whether `argument` is its first argument.
fn call_of_argument(argument: &AnyJsExpression) -> Option<(JsCallExpression, bool)> {
    let arguments = argument
        .parent::<JsCallArgumentList>()?
        .parent::<JsCallArguments>()?;
    let is_first = arguments
        .args()
        .first()
        .and_then(Result::ok)
        .is_some_and(|first| first.syntax() == argument.syntax());
    Some((arguments.parent::<JsCallExpression>()?, is_first))
}

/// Returns the name of the object in `object.member` when the object is a plain identifier.
fn static_member_object_name(member: &JsStaticMemberExpression) -> Option<JsSyntaxToken> {
    let object = member.object().ok()?.omit_parentheses();
    object
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .value_token()
        .ok()
}

/// Returns `true` when the comment right before `function` is an `@isolated` comment.
///
/// When no comment directly precedes the function, the search continues with the
/// declaration, export, or object property that holds the function, so that comments placed
/// before `const`, `export`, or a property name also apply. The search stops at the first
/// comment found, even if it isn't an `@isolated` comment.
fn has_isolated_comment(function: &AnyIsolatedFunctionCandidate) -> bool {
    for node in function.syntax().ancestors() {
        if let Some(comment) = comment_before(&node) {
            return is_isolated_comment(comment.text());
        }
        let Some(parent) = node.parent() else {
            return false;
        };
        let is_holder = COMMENT_HOLDER_KINDS.matches(parent.kind())
            || (parent.kind() == JsSyntaxKind::JS_INITIALIZER_CLAUSE
                && parent.parent().is_some_and(|declarator| {
                    declarator.kind() == JsSyntaxKind::JS_VARIABLE_DECLARATOR
                }));
        if !is_holder {
            return false;
        }
    }
    false
}

/// Returns the last comment between `node` and the token before it.
fn comment_before(node: &JsSyntaxNode) -> Option<SyntaxTriviaPieceComments<JsLanguage>> {
    let first_token = node.first_token()?;
    let last_comment =
        |trivia: JsSyntaxTrivia| trivia.pieces().rev().find_map(|piece| piece.as_comments());
    last_comment(first_token.leading_trivia()).or_else(|| {
        // A comment on the same line as the previous token, such as `foo(/* @isolated */ () => {})`,
        // belongs to the trailing trivia of that token.
        last_comment(first_token.prev_token()?.trailing_trivia())
    })
}

/// Returns `true` when `comment` contains only the `@isolated` tag, or the tag followed by
/// ` - ` or ` -- ` and an explanation. The tag is case-insensitive.
fn is_isolated_comment(comment: &str) -> bool {
    let content = if let Some(content) = comment.strip_prefix("//") {
        content
    } else if let Some(content) = comment
        .strip_prefix("/*")
        .and_then(|content| content.strip_suffix("*/"))
    {
        content
    } else {
        return false;
    };
    // JSDoc comments start every line with `*`, as in `/**\n * @isolated\n */`.
    let content = content
        .trim_start_matches(|c: char| c == '*' || c.is_whitespace())
        .trim_end();
    let Some(tag) = content.get(..ISOLATED_TAG.len()) else {
        return false;
    };
    if !tag.eq_ignore_ascii_case(ISOLATED_TAG) {
        return false;
    }
    let rest = &content[ISOLATED_TAG.len()..];
    rest.is_empty() || rest.starts_with(" - ") || rest.starts_with(" -- ")
}

/// Collects every use, inside `function`, of a variable declared outside of it, and every
/// `this` or `super` that gets its value from `function`.
fn find_outer_scope_uses(
    model: &SemanticModel,
    function: &AnyIsolatedFunctionCandidate,
    reason: IsolationReason,
) -> Box<[OuterScopeUse]> {
    let function_syntax = function.syntax();
    let function_range = function.range();
    let mut uses = Vec::new();
    let mut preorder = function_syntax.preorder_of_kinds(OUTER_SCOPE_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(reference) = AnyJsIdentifierReference::cast_ref(&node) {
            // Type annotations are removed before the function runs.
            if reference.is_only_type() {
                continue;
            }
            // Unresolved references are global variables, and `declare` describes values
            // that the place where the function runs provides.
            let Some(binding) = model.binding(&reference) else {
                continue;
            };
            let declaration = binding.syntax();
            if function_range.contains_range(declaration.text_trimmed_range())
                || is_in_ambient_context(&declaration)
            {
                continue;
            }
            uses.push(OuterScopeUse {
                reason,
                kind: OuterScopeUseKind::Variable {
                    reference,
                    declaration: declaration.text_trimmed_range(),
                },
            });
        } else if let Some(expression) = AnyJsContextExpression::cast_ref(&node) {
            if gets_context_from(&expression, function) {
                uses.push(OuterScopeUse {
                    reason,
                    kind: OuterScopeUseKind::Context(expression),
                });
            }
        } else if JsDecorator::can_cast(node.kind()) {
            // The decorators of a method and of its parameters run where the method is
            // declared, not where the method runs.
            if node
                .ancestors()
                .find(|ancestor| DECORATED_KINDS.matches(ancestor.kind()))
                .as_ref()
                == Some(function_syntax)
            {
                preorder.skip_subtree();
            }
        } else if node.parent().as_ref() == Some(function_syntax) {
            // The computed name of a method runs where the method is declared, not where
            // the method runs.
            preorder.skip_subtree();
        }
    }
    uses.into_boxed_slice()
}

/// Returns `true` when `expression` gets its `this` or `super` value from `function`.
///
/// Arrow functions use the value of the code around them. Other functions, methods,
/// and class members have their own value, except in computed member names, which
/// run in the code around the member.
fn gets_context_from(
    expression: &AnyJsContextExpression,
    function: &AnyIsolatedFunctionCandidate,
) -> bool {
    let mut is_in_computed_name = false;
    for node in expression.syntax().ancestors().skip(1) {
        if is_in_computed_name {
            // `node` is the member that owns the computed name.
            is_in_computed_name = false;
            if &node == function.syntax() {
                return false;
            }
            continue;
        }
        if &node == function.syntax() {
            return true;
        }
        if node.kind() == JsSyntaxKind::JS_COMPUTED_MEMBER_NAME {
            is_in_computed_name = true;
        } else if CONTEXT_BOUNDARY_KINDS.matches(node.kind()) {
            return false;
        }
    }
    false
}

const ISOLATED_TAG: &str = "@isolated";

/// The nodes that hold a function, and that a comment placed before them applies to.
/// Initializers are handled separately, because only those of variable declarations qualify.
const COMMENT_HOLDER_KINDS: SyntaxKindSet<JsLanguage> = JsParenthesizedExpression::KIND_SET
    .union(JsVariableDeclarator::KIND_SET)
    .union(JsVariableDeclaratorList::KIND_SET)
    .union(JsVariableDeclaration::KIND_SET)
    .union(JsVariableStatement::KIND_SET)
    .union(JsVariableDeclarationClause::KIND_SET)
    .union(JsExportDefaultExpressionClause::KIND_SET)
    .union(JsExportDefaultDeclarationClause::KIND_SET)
    .union(JsExport::KIND_SET)
    .union(JsPropertyObjectMember::KIND_SET);

/// The nodes the search for outer scope uses looks at: references, `this` and `super`,
/// and the parts of a method that run outside of it.
const OUTER_SCOPE_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = AnyJsIdentifierReference::KIND_SET
    .union(AnyJsContextExpression::KIND_SET)
    .union(JsComputedMemberName::KIND_SET)
    .union(JsDecorator::KIND_SET);

/// The nodes that decorators can apply to. Decorators of parameters belong to the class
/// member that declares the parameters.
const DECORATED_KINDS: SyntaxKindSet<JsLanguage> =
    AnyJsClassMember::KIND_SET.union(AnyJsClass::KIND_SET);

/// The nodes that have their own `this` and `super` values.
const CONTEXT_BOUNDARY_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionExpression::KIND_SET
    .union(JsFunctionDeclaration::KIND_SET)
    .union(JsFunctionExportDefaultDeclaration::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET)
    .union(AnyJsClassMember::KIND_SET);
