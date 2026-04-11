use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsRoot, JsDirective,
    JsDirectiveList, JsExport, JsSyntaxToken, T, export_ext::AnyJsExported,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TokenText, TriviaPieceKind};
use biome_rule_options::use_react_async_server_function::UseReactAsyncServerFunctionOptions;

declare_lint_rule! {
    /// Require functions with the "use server" directive to be async.
    ///
    /// Require Server Functions (functions in a file with a top-level `"use server"` directive or functions with their own `"use server"` directive) to be async.
    ///
    /// See the [React documentation](https://react.dev/reference/rsc/use-server) for more details.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <form
    ///   action={() => {
    ///     'use server';
    ///     // ...
    ///   }}
    /// >
    ///   // ...
    /// </form>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function serverFunction() {
    ///   'use server';
    ///   // ...
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// 'use server';
    /// export function serverFunction() {
    ///   // ...
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <form
    ///   action={async () => {
    ///     'use server';
    ///     // ...
    ///   }}
    /// >
    ///   // ...
    /// </form>
    /// ```
    ///
    /// ```js
    /// async function serverFunction() {
    ///   'use server';
    ///   // ...
    /// }
    /// ```
    ///
    /// ```js
    /// 'use server';
    /// export async function serverFunction() {
    ///   // ...
    /// }
    /// ```
    ///
    pub UseReactAsyncServerFunction {
        version: "next",
        name: "useReactAsyncServerFunction",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReact("async-server-action").same(), RuleSource::EslintReactXyz("rsc-function-definition").same(), RuleSource::EslintReactRsc("function-definition").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseReactAsyncServerFunction {
    type Query = Ast<AnyFunctionLike>;
    type State = ServerFunctionKind;
    type Signals = Option<Self::State>;
    type Options = UseReactAsyncServerFunctionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Generator functions are ignored, matching the upstream behavior.
        if node.is_async()
            || node.is_generator()
            || matches!(node, AnyFunctionLike::JsConstructorClassMember(_))
        {
            return None;
        }

        if has_local_use_server_directive(node) {
            return Some(ServerFunctionKind::Local);
        }

        let root = ctx.root();
        if !has_file_level_use_server_directive(&root) {
            return None;
        }

        match node {
            AnyFunctionLike::AnyJsFunction(function) if is_exported_function(function, &root) => {
                Some(ServerFunctionKind::File)
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "This server function is missing the "<Emphasis>"async"</Emphasis>" keyword." },
            )
            .note(match state {
                ServerFunctionKind::File =>
                    (markup! { "Functions exported from files with the \"use server\" directive are React Server Functions and therefore must be async." }).to_owned(),
                ServerFunctionKind::Local =>
                    (markup! {"Functions with the \"use server\" directive are React Server Functions and therefore must be async." }).to_owned(),
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match node {
            AnyFunctionLike::AnyJsFunction(AnyJsFunction::JsArrowFunctionExpression(function)) => {
                mutation.replace_node(
                    function.clone(),
                    function
                        .clone()
                        .with_async_token(Some(make_async_token_with_space())),
                );
            }
            AnyFunctionLike::AnyJsFunction(AnyJsFunction::JsFunctionExpression(function)) => {
                let function_token = function.function_token().ok()?;
                let fixed_function_token = function_token.with_leading_trivia_pieces([]);
                mutation.replace_node(
                    function.clone(),
                    function
                        .clone()
                        .with_async_token(Some(make_async_token_with_space_from_anchor(
                            &function_token,
                        )))
                        .with_function_token(fixed_function_token),
                );
            }
            AnyFunctionLike::AnyJsFunction(AnyJsFunction::JsFunctionDeclaration(function)) => {
                let function_token = function.function_token().ok()?;
                let fixed_function_token = function_token.with_leading_trivia_pieces([]);
                mutation.replace_node(
                    function.clone(),
                    function
                        .clone()
                        .with_async_token(Some(make_async_token_with_space_from_anchor(
                            &function_token,
                        )))
                        .with_function_token(fixed_function_token),
                );
            }
            AnyFunctionLike::AnyJsFunction(AnyJsFunction::JsFunctionExportDefaultDeclaration(
                function,
            )) => {
                let function_token = function.function_token().ok()?;
                let fixed_function_token = function_token.with_leading_trivia_pieces([]);
                mutation.replace_node(
                    function.clone(),
                    function
                        .clone()
                        .with_async_token(Some(make_async_token_with_space_from_anchor(
                            &function_token,
                        )))
                        .with_function_token(fixed_function_token),
                );
            }
            AnyFunctionLike::JsMethodObjectMember(function) => {
                let name = function.name().ok()?;
                let anchor = name.syntax().first_token()?;
                let stripped_name = name.with_leading_trivia_pieces([])?;
                mutation.replace_node(
                    function.clone(),
                    function
                        .clone()
                        .with_async_token(Some(make_async_token_with_space_from_anchor(&anchor)))
                        .with_name(stripped_name),
                );
            }
            AnyFunctionLike::JsMethodClassMember(function) => {
                if function.modifiers().is_empty() {
                    let name = function.name().ok()?;
                    let anchor = name.syntax().first_token()?;
                    let stripped_name = name.with_leading_trivia_pieces([])?;
                    mutation.replace_node(
                        function.clone(),
                        function
                            .clone()
                            .with_async_token(Some(make_async_token_with_space_from_anchor(
                                &anchor,
                            )))
                            .with_name(stripped_name),
                    );
                } else {
                    mutation.replace_node(
                        function.clone(),
                        function
                            .clone()
                            .with_async_token(Some(make_async_token_with_space())),
                    );
                }
            }
            AnyFunctionLike::JsConstructorClassMember(_) => return None,
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add "<Emphasis>"async"</Emphasis>" keyword." }.to_owned(),
            mutation,
        ))
    }
}

pub enum ServerFunctionKind {
    File,
    Local,
}

fn make_async_token_with_space() -> biome_js_syntax::JsSyntaxToken {
    make::token(T![async]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
}

fn make_async_token_with_space_from_anchor(
    anchor: &JsSyntaxToken,
) -> biome_js_syntax::JsSyntaxToken {
    make::token(T![async])
        .with_leading_trivia_pieces(anchor.leading_trivia().pieces())
        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
}

fn has_local_use_server_directive(function: &AnyFunctionLike) -> bool {
    let Ok(body) = function.body() else {
        return false;
    };

    match body {
        AnyJsFunctionBody::JsFunctionBody(function_body) => {
            is_first_use_server_directive(&function_body.directives())
        }
        _ => false,
    }
}

fn has_file_level_use_server_directive(root: &AnyJsRoot) -> bool {
    match root {
        AnyJsRoot::JsModule(module) => is_first_use_server_directive(&module.directives()),
        AnyJsRoot::JsScript(script) => is_first_use_server_directive(&script.directives()),
        _ => false,
    }
}

fn is_first_use_server_directive(directives: &JsDirectiveList) -> bool {
    directives
        .into_iter()
        .next()
        .is_some_and(|directive| is_use_server(&directive))
}

fn is_use_server(directive: &JsDirective) -> bool {
    directive
        .inner_string_text()
        .is_ok_and(|text| text.text() == "use server")
}

fn is_exported_function(function: &AnyJsFunction, root: &AnyJsRoot) -> bool {
    if matches!(
        function,
        AnyJsFunction::JsFunctionExportDefaultDeclaration(_)
    ) {
        return true;
    }

    let binding_name = function_binding_name(function);

    root.syntax()
        .descendants()
        .filter_map(JsExport::cast)
        .flat_map(|export| export.get_exported_items())
        .any(|item| {
            item.exported.as_ref().is_some_and(|exported| {
                is_matching_exported_syntax(function, exported)
                    || is_matching_exported_identifier(binding_name.as_ref(), exported)
            })
        })
}

fn is_matching_exported_syntax(function: &AnyJsFunction, exported: &AnyJsExported) -> bool {
    match (function, exported) {
        (
            AnyJsFunction::JsFunctionDeclaration(function),
            AnyJsExported::JsFunctionDeclaration(exported),
        ) => function.syntax() == exported.syntax(),
        (
            AnyJsFunction::JsFunctionExpression(function),
            AnyJsExported::AnyJsExpression(AnyJsExpression::JsFunctionExpression(exported)),
        ) => function.syntax() == exported.syntax(),
        (
            AnyJsFunction::JsArrowFunctionExpression(function),
            AnyJsExported::AnyJsExpression(AnyJsExpression::JsArrowFunctionExpression(exported)),
        ) => function.syntax() == exported.syntax(),
        _ => false,
    }
}

fn is_matching_exported_identifier(
    binding_name: Option<&TokenText>,
    exported: &AnyJsExported,
) -> bool {
    let Some(binding_name) = binding_name else {
        return false;
    };

    if let AnyJsExported::AnyIdentifier(identifier) = exported {
        return identifier
            .name_token()
            .is_some_and(|name_token| name_token.token_text_trimmed() == *binding_name);
    }

    false
}

fn function_binding_name(function: &AnyJsFunction) -> Option<TokenText> {
    function
        .binding()?
        .as_js_identifier_binding()?
        .name_token()
        .ok()
        .map(|token| token.token_text_trimmed())
}
