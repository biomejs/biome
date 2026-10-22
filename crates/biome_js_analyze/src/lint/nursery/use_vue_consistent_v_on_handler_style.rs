use crate::JsRuleAction;
use crate::services::vue::VueSnippet;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsExpression, AnyJsFormalParameter, AnyJsFunctionBody, AnyJsParameter, AnyJsRoot,
    AnyJsStatement, JsArrowFunctionExpression, JsCallExpression, JsComputedMemberExpression,
    JsExpressionTemplateRoot, JsFunctionExpression, JsIdentifierExpression, JsLanguage, JsScript,
    JsStatementList, JsStaticMemberExpression, T,
};
use biome_languages::JsFileSource;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxKindSet, SyntaxResult,
    TextRange, TokenText, declare_node_union,
};
use biome_rule_options::use_vue_consistent_v_on_handler_style::{
    UseVueConsistentVOnHandlerStyleOptions, VueVOnHandlerStyle,
};
use biome_vue_semantic::{ScopeKind, Snippet};

declare_lint_rule! {
    /// Enforce a consistent style for event handlers in Vue `v-on` directives.
    ///
    /// Vue accepts three styles of event handler in `v-on` directives, also written as `@`:
    ///
    /// - A method handler is the name of a function, such as `@click="save"`. Vue calls the
    ///   function with the arguments of the event.
    /// - An inline handler is code that Vue runs when the event happens, such as
    ///   `@click="save()"` or `@click="count++"`. The code can read the first argument of the
    ///   event as `$event`.
    /// - An inline function is a function written in the template, such as
    ///   `@click="() => save()"`. Vue calls it with the arguments of the event.
    ///
    /// Mixing these styles makes templates harder to read. This rule reports event handlers
    /// that don't use the configured style.
    ///
    /// An inline handler or inline function that only calls a function by its name, and passes
    /// it the same arguments it receives, can be replaced by the name of that function. The fix
    /// for this is only offered when the function is declared in the `methods` option of the
    /// component with the same number of parameters, because otherwise Vue could pass the event
    /// to a parameter that the handler left empty.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <button @click="save()">Save</button>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <button @click="() => save()">Save</button>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <button @click="count++">Add</button>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <button @click="save">Save</button>
    ///   <button v-for="item in items" @click="() => remove(item)">Remove</button>
    /// </template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// Default: `"methodOrInlineFunction"`
    ///
    /// The handler style to enforce.
    ///
    /// - `"methodOrInlineFunction"`: use a method handler. Where a method handler can't be used,
    ///   such as when the handler passes a variable of `v-for` to a function, use an inline
    ///   function.
    /// - `"methodOrInline"`: use a method handler. Where a method handler can't be used, use an
    ///   inline handler.
    /// - `"inlineFunction"`: use an inline function.
    /// - `"inline"`: use an inline handler.
    ///
    /// The rule doesn't report handlers that call a method of an object by name, such as
    /// `@click="form.submit"`.
    ///
    /// #### `"methodOrInline"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "style": "methodOrInline"
    ///   }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options,ignore
    /// <template>
    ///   <button v-for="item in items" @click="() => remove(item)">Remove</button>
    /// </template>
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <button @click="save">Save</button>
    ///   <button v-for="item in items" @click="remove(item)">Remove</button>
    /// </template>
    /// ```
    ///
    /// #### `"inlineFunction"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "style": "inlineFunction"
    ///   }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options,ignore
    /// <template>
    ///   <button @click="save">Save</button>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,use_options,ignore
    /// <template>
    ///   <button @click="save()">Save</button>
    /// </template>
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <button @click="() => save()">Save</button>
    ///   <button @click="() => count++">Add</button>
    /// </template>
    /// ```
    ///
    /// #### `"inline"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "style": "inline"
    ///   }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options,ignore
    /// <template>
    ///   <button @click="save">Save</button>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,use_options,ignore
    /// <template>
    ///   <button @click="() => save()">Save</button>
    /// </template>
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```vue,use_options
    /// <template>
    ///   <button @click="save()">Save</button>
    ///   <button @click="save($event)">Save</button>
    ///   <button @click="count++">Add</button>
    /// </template>
    /// ```
    ///
    /// ## See Also
    ///
    /// - To enforce a consistent way of writing the `v-on` directive itself, see [`useVueConsistentVOnStyle`](https://biomejs.dev/linter/rules/use-vue-consistent-v-on-style/).
    ///
    pub UseVueConsistentVOnHandlerStyle {
        version: "next",
        name: "useVueConsistentVOnHandlerStyle",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("v-on-handler-style").same()],
        fix_kind: FixKind::Unsafe,
    }
}

/// An event handler written in a style that the options don't allow.
pub struct HandlerStyleViolation {
    handler: Handler,
    preferred: HandlerKind,
    /// Whether the handler only calls a function by its name and passes it exactly the
    /// arguments it receives, so the function could be the handler.
    ///
    /// Only `true` when the preferred style is a method handler.
    forwards_arguments: bool,
    /// The rewrite that the action applies. `run` decides it, so that the diagnostic can tell
    /// the user what to do when there is none.
    fix: Option<HandlerFix>,
}

impl Rule for UseVueConsistentVOnHandlerStyle {
    type Query = VueSnippet;
    type State = HandlerStyleViolation;
    type Signals = Option<Self::State>;
    type Options = UseVueConsistentVOnHandlerStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<JsFileSource>().is_vue_event_handler() {
            return None;
        }
        let snippet = ctx.query();
        let handler = Handler::from_root(&ctx.root())?;
        let found = handler.kind();
        let allowed = allowed_kinds(ctx.options().style());
        if allowed.first() == Some(&found) {
            return None;
        }

        for &preferred in allowed {
            if preferred == found {
                continue;
            }
            if preferred != HandlerKind::Method {
                let fix = match (&handler, preferred) {
                    (Handler::Inline(script), HandlerKind::InlineFunction) => {
                        can_wrap_in_function(snippet, script).then_some(HandlerFix::WrapInFunction)
                    }
                    (Handler::InlineFunction(function), HandlerKind::Inline) => {
                        inline_handler_body(function).map(HandlerFix::UseBody)
                    }
                    _ => None,
                };
                return Some(HandlerStyleViolation {
                    handler,
                    preferred,
                    forwards_arguments: false,
                    fix,
                });
            }
            let call = handler.identifier_call();
            let forwards_arguments = call
                .as_ref()
                .is_some_and(|call| handler.forwards_arguments(call));
            if !forwards_arguments && uses_element_variable(snippet) {
                continue;
            }
            let fix = call
                .filter(|_| forwards_arguments && !snippet.has_comments() && !handler.is_generator())
                .and_then(|call| method_fix(snippet, &call));
            return Some(HandlerStyleViolation {
                handler,
                preferred,
                forwards_arguments,
                fix,
            });
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let found = state.handler.kind();
        let preferred = state.preferred;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.handler.range(),
            markup! {
                "This event handler is written as "{found}" instead of "{preferred}"."
            },
        )
        .note(markup! {
            "This project writes every event handler in the same style, so that templates are easier to read."
        });
        if state.fix.is_some() {
            return Some(diagnostic);
        }

        let next_step = match (found, preferred) {
            (_, HandlerKind::Method) if !state.forwards_arguments => markup! {
                "Move this code into a function of the component, then use the name of that function as the handler, such as "<Emphasis>"@click=\"save\""</Emphasis>"."
            },
            (_, HandlerKind::Method) => markup! {
                "Use the name of the function as the handler, such as "<Emphasis>"@click=\"save\""</Emphasis>". Vue then passes the arguments of the event to the function, so check that the function expects them."
            },
            (_, HandlerKind::Inline) if state.handler.parameter_count() > 1 => markup! {
                "An inline handler can only read the first argument of the event, as "<Emphasis>"$event"</Emphasis>". Change the event to send a single value, then use the code that the function runs as the handler."
            },
            (_, HandlerKind::Inline) => markup! {
                "Use the code that the function runs as the handler, and read the first argument of the event as "<Emphasis>"$event"</Emphasis>", such as "<Emphasis>"@click=\"save($event)\""</Emphasis>"."
            },
            (HandlerKind::Method, HandlerKind::InlineFunction) => markup! {
                "Wrap the handler in a function that passes on the arguments of the event, such as "<Emphasis>"@click=\"(event) => save(event)\""</Emphasis>"."
            },
            (_, HandlerKind::InlineFunction) => markup! {
                "Wrap the handler in a function, such as "<Emphasis>"@click=\"() => save()\""</Emphasis>". If the handler reads "<Emphasis>"$event"</Emphasis>", read a parameter of the function instead, such as "<Emphasis>"@click=\"(event) => save(event)\""</Emphasis>"."
            },
        };
        Some(diagnostic.note(next_step))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let message = match (state.fix.as_ref()?, &state.handler) {
            (HandlerFix::UseMethod(callee), handler) => {
                let callee = callee.clone().trim_trivia()?;
                let name = callee.name().ok()?.value_token().ok()?.token_text_trimmed();
                match handler {
                    Handler::Inline(script) => {
                        mutation.replace_node(
                            script.statements(),
                            make::js_statement_list([AnyJsStatement::from(
                                make::js_expression_statement(AnyJsExpression::from(callee))
                                    .build(),
                            )]),
                        );
                    }
                    Handler::InlineFunction(function) => {
                        mutation
                            .replace_node(function.to_expression(), AnyJsExpression::from(callee));
                    }
                    Handler::Method(_) => return None,
                }
                markup! { "Use "<Emphasis>{name.text()}</Emphasis>" as the handler." }
                    .to_owned()
            }
            (HandlerFix::WrapInFunction, Handler::Inline(script)) => {
                let arrow = make::js_arrow_function_expression(
                    AnyJsArrowFunctionParameters::from(make::js_parameters(
                        make::token(T!['(']),
                        make::js_parameter_list([], []),
                        make::token(T![')']),
                    )),
                    make::token_decorated_with_space(T![=>]),
                    inline_function_body(&script.statements())?,
                )
                .build();
                mutation.replace_node(
                    script.statements(),
                    make::js_statement_list([AnyJsStatement::from(
                        make::js_expression_statement(AnyJsExpression::from(arrow)).build(),
                    )]),
                );
                markup! { "Wrap the handler in an inline function." }.to_owned()
            }
            (HandlerFix::UseBody(body), Handler::InlineFunction(function)) => {
                mutation.replace_node(function.to_expression(), body.clone());
                markup! { "Use the body of the function as the handler." }.to_owned()
            }
            _ => return None,
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

/// A rewrite of a handler into the preferred style that keeps what it does.
pub enum HandlerFix {
    /// Replace the handler with the name of the function it calls.
    UseMethod(JsIdentifierExpression),
    /// Wrap the inline handler in an arrow function without parameters.
    WrapInFunction,
    /// Replace the inline function with the expression it runs.
    UseBody(AnyJsExpression),
}

/// The style of an event handler.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HandlerKind {
    /// `@click="save"`
    Method,
    /// `@click="save()"`
    Inline,
    /// `@click="() => save()"`
    InlineFunction,
}

impl biome_console::fmt::Display for HandlerKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Method => "a method handler",
            Self::Inline => "an inline handler",
            Self::InlineFunction => "an inline function",
        })
    }
}

/// Returns the handler styles allowed by `style`, the preferred one first.
fn allowed_kinds(style: VueVOnHandlerStyle) -> &'static [HandlerKind] {
    match style {
        VueVOnHandlerStyle::MethodOrInlineFunction => {
            &[HandlerKind::Method, HandlerKind::InlineFunction]
        }
        VueVOnHandlerStyle::MethodOrInline => &[HandlerKind::Method, HandlerKind::Inline],
        VueVOnHandlerStyle::InlineFunction => &[HandlerKind::InlineFunction],
        VueVOnHandlerStyle::Inline => &[HandlerKind::Inline],
    }
}

declare_node_union! {
    pub AnyJsInlineFunction = JsArrowFunctionExpression | JsFunctionExpression
}

/// The value of a `v-on` directive, classified by style.
pub enum Handler {
    Method(JsIdentifierExpression),
    /// The statements of an inline handler.
    Inline(JsScript),
    InlineFunction(AnyJsInlineFunction),
}

impl Handler {
    /// Classifies the tree of a `v-on` value.
    ///
    /// Returns `None` for an empty value, and for a value that names a member of an object,
    /// such as `form.submit`.
    fn from_root(root: &AnyJsRoot) -> Option<Self> {
        match root {
            AnyJsRoot::JsExpressionTemplateRoot(root) => Self::from_expression_root(root),
            AnyJsRoot::JsScript(script) => {
                if script.statements().is_empty() && script.directives().is_empty() {
                    return None;
                }
                Some(Self::Inline(script.clone()))
            }
            _ => None,
        }
    }

    fn from_expression_root(root: &JsExpressionTemplateRoot) -> Option<Self> {
        match root.expression()? {
            AnyJsExpression::JsIdentifierExpression(identifier) => Some(Self::Method(identifier)),
            AnyJsExpression::JsArrowFunctionExpression(function) => {
                Some(Self::InlineFunction(function.into()))
            }
            AnyJsExpression::JsFunctionExpression(function) => {
                Some(Self::InlineFunction(function.into()))
            }
            _ => None,
        }
    }

    fn kind(&self) -> HandlerKind {
        match self {
            Self::Method(_) => HandlerKind::Method,
            Self::Inline(_) => HandlerKind::Inline,
            Self::InlineFunction(_) => HandlerKind::InlineFunction,
        }
    }

    fn range(&self) -> TextRange {
        match self {
            Self::Method(identifier) => identifier.range(),
            Self::Inline(script) => {
                let statements = script.statements().syntax().text_trimmed_range();
                let directives = script.directives();
                if directives.is_empty() {
                    statements
                } else {
                    directives.syntax().text_trimmed_range().cover(statements)
                }
            }
            Self::InlineFunction(function) => function.range(),
        }
    }

    /// Returns the call of a function by its name that makes up the whole handler, ignoring
    /// empty statements and braces.
    fn identifier_call(&self) -> Option<JsCallExpression> {
        let expression = match self {
            Self::Method(_) => return None,
            Self::Inline(script) => {
                if !script.directives().is_empty() {
                    return None;
                }
                single_expression(&script.statements())?
            }
            Self::InlineFunction(function) => match function.body()? {
                AnyJsFunctionBody::AnyJsExpression(expression) => expression,
                AnyJsFunctionBody::JsFunctionBody(body) => single_expression(&body.statements())?,
            },
        };
        let call = expression.omit_parentheses().as_js_call_expression()?.clone();
        if call.optional_chain_token().is_some()
            || !matches!(
                call.callee().ok()?.omit_parentheses(),
                AnyJsExpression::JsIdentifierExpression(_)
            )
        {
            return None;
        }
        Some(call)
    }

    /// Returns `true` when the handler passes `call` the same arguments it receives.
    ///
    /// An inline handler receives no arguments. An inline function forwards its arguments when
    /// each argument of `call` is the parameter at the same position.
    fn forwards_arguments(&self, call: &JsCallExpression) -> bool {
        let Ok(arguments) = call.arguments() else {
            return false;
        };
        match self {
            Self::Method(_) => false,
            Self::Inline(_) => arguments.args().is_empty(),
            Self::InlineFunction(function) => {
                let arguments = arguments.args();
                match function.parameters() {
                    Some(AnyJsArrowFunctionParameters::AnyJsBinding(binding)) => {
                        arguments.len() == 1
                            && arguments
                                .first()
                                .is_some_and(|argument| is_name_of(argument, binding_name(&binding)))
                    }
                    Some(AnyJsArrowFunctionParameters::JsParameters(parameters)) => {
                        let parameters = parameters.items();
                        parameters.len() == arguments.len()
                            && parameters.iter().zip(arguments.iter()).all(
                                |(parameter, argument)| {
                                    is_name_of(
                                        argument,
                                        parameter.ok().and_then(|parameter| plain_name(&parameter)),
                                    )
                                },
                            )
                    }
                    None => false,
                }
            }
        }
    }

    fn parameter_count(&self) -> usize {
        match self {
            Self::InlineFunction(function) => function.parameter_count(),
            _ => 0,
        }
    }

    fn is_generator(&self) -> bool {
        match self {
            Self::InlineFunction(AnyJsInlineFunction::JsFunctionExpression(function)) => {
                function.star_token().is_some()
            }
            _ => false,
        }
    }
}

impl AnyJsInlineFunction {
    fn to_expression(&self) -> AnyJsExpression {
        match self {
            Self::JsArrowFunctionExpression(function) => function.clone().into(),
            Self::JsFunctionExpression(function) => function.clone().into(),
        }
    }

    fn body(&self) -> Option<AnyJsFunctionBody> {
        match self {
            Self::JsArrowFunctionExpression(function) => function.body().ok(),
            Self::JsFunctionExpression(function) => {
                function.body().ok().map(AnyJsFunctionBody::from)
            }
        }
    }

    fn parameters(&self) -> Option<AnyJsArrowFunctionParameters> {
        match self {
            Self::JsArrowFunctionExpression(function) => function.parameters().ok(),
            Self::JsFunctionExpression(function) => function
                .parameters()
                .ok()
                .map(AnyJsArrowFunctionParameters::from),
        }
    }

    fn parameter_count(&self) -> usize {
        match self.parameters() {
            Some(AnyJsArrowFunctionParameters::AnyJsBinding(_)) => 1,
            Some(AnyJsArrowFunctionParameters::JsParameters(parameters)) => {
                parameters.items().len()
            }
            None => 0,
        }
    }
}

/// Returns the name of `parameter` when it is a plain name, without a default value. Returns
/// `None` for a rest parameter or a destructured parameter.
fn plain_name(parameter: &AnyJsParameter) -> Option<TokenText> {
    let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(parameter)) =
        parameter
    else {
        return None;
    };
    if parameter.initializer().is_some() {
        return None;
    }
    let AnyJsBindingPattern::AnyJsBinding(binding) = parameter.binding().ok()? else {
        return None;
    };
    binding_name(&binding)
}

fn binding_name(binding: &AnyJsBinding) -> Option<TokenText> {
    Some(
        binding
            .as_js_identifier_binding()?
            .name_token()
            .ok()?
            .token_text_trimmed(),
    )
}

/// Returns `true` when `argument` is a variable called `name`.
fn is_name_of(argument: SyntaxResult<AnyJsCallArgument>, name: Option<TokenText>) -> bool {
    let Ok(AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(argument))) =
        argument
    else {
        return false;
    };
    name.is_some_and(|name| {
        argument
            .name()
            .and_then(|reference| reference.value_token())
            .is_ok_and(|token| token.text_trimmed() == name.text())
    })
}

/// Returns the expression of the only statement in `statements`, looking inside blocks and
/// ignoring empty statements.
fn single_expression(statements: &JsStatementList) -> Option<AnyJsExpression> {
    let mut statements = statements
        .iter()
        .filter(|statement| !matches!(statement, AnyJsStatement::JsEmptyStatement(_)));
    let statement = statements.next()?;
    if statements.next().is_some() {
        return None;
    }
    match statement {
        AnyJsStatement::JsExpressionStatement(statement) => statement.expression().ok(),
        AnyJsStatement::JsBlockStatement(block) => single_expression(&block.statements()),
        _ => None,
    }
}

/// Returns `true` when the handler uses a variable declared on an element of the template,
/// such as a `v-for` item or a slot parameter.
///
/// A handler that uses such a variable can't be moved into a function of the component.
fn uses_element_variable(snippet: &Snippet) -> bool {
    snippet.references().any(|reference| {
        reference
            .symbol()
            .is_some_and(|symbol| matches!(symbol.scope().kind(), ScopeKind::Element(_)))
    })
}

/// Returns `true` when the handler reads `$event`.
fn reads_event(snippet: &Snippet) -> bool {
    snippet
        .references()
        .any(|reference| reference.name() == "$event")
}

/// Returns `true` when the value of the directive is written between quotes.
fn is_quoted(snippet: &Snippet) -> bool {
    snippet.attr().is_some_and(|attr| {
        attr.value_range()
            .is_some_and(|value| value.end() < attr.range().end())
    })
}

/// Returns the fix that replaces the handler with the function `call` calls, when that
/// function is a method of the component that declares exactly as many parameters as `call`
/// passes, so using it as the handler passes it the same arguments.
///
/// Returns `None` when the number of parameters is unknown, as for a function declared in
/// `<script setup>`, because Vue could then pass the event to a parameter with a default value.
fn method_fix(snippet: &Snippet, call: &JsCallExpression) -> Option<HandlerFix> {
    let callee = call.callee().ok()?.omit_parentheses();
    let callee = callee.as_js_identifier_expression()?;
    let argument_count = call.arguments().ok()?.args().len();
    let range = callee.range();
    let symbol = snippet
        .references()
        .find(|reference| reference.range() == range)?
        .symbol()?;
    let method = symbol
        .component()?
        .methods()
        .find(|method| **method == symbol)?;
    let takes_arguments = !method.has_rest_param()
        && method
            .params()
            .is_some_and(|params| usize::from(params) == argument_count);
    takes_arguments.then(|| HandlerFix::UseMethod(callee.clone()))
}

/// Returns `true` when wrapping the inline handler `script` in an arrow function keeps what it
/// does and keeps the value valid.
///
/// Returns `false` when the handler reads `$event`, which the arrow function doesn't declare,
/// and when the value isn't quoted, because the arrow function adds spaces.
fn can_wrap_in_function(snippet: &Snippet, script: &JsScript) -> bool {
    is_quoted(snippet) && !reads_event(snippet) && script.directives().is_empty()
}

/// Builds the body of an inline function that runs `statements`.
///
/// A single expression without a semicolon becomes the body itself, as in `() => save()`, and
/// is wrapped in parentheses when it is a comma expression. A single block keeps its braces.
/// Anything else is wrapped in braces.
fn inline_function_body(statements: &JsStatementList) -> Option<AnyJsFunctionBody> {
    let statements = statements.clone().trim_trivia()?;
    let mut iter = statements.iter();
    let first = iter.next()?;
    if iter.next().is_none() {
        match first {
            AnyJsStatement::JsExpressionStatement(statement)
                if statement.semicolon_token().is_none() =>
            {
                let expression = statement.expression().ok()?;
                // `() => a(), b()` would run only `a()`.
                let expression = if expression.as_js_sequence_expression().is_some() {
                    AnyJsExpression::from(make::parenthesized(expression))
                } else {
                    expression
                };
                return Some(AnyJsFunctionBody::AnyJsExpression(expression));
            }
            AnyJsStatement::JsBlockStatement(block) => {
                return Some(AnyJsFunctionBody::from(make::js_function_body(
                    block.l_curly_token().ok()?,
                    make::js_directive_list([]),
                    block.statements(),
                    block.r_curly_token().ok()?,
                )));
            }
            _ => {}
        }
    }
    Some(AnyJsFunctionBody::from(make::js_function_body(
        make::token(T!['{']),
        make::js_directive_list([]),
        statements,
        make::token(T!['}']),
    )))
}

/// Returns the code that an inline handler should run in place of `function`.
///
/// Returns `None` when the code can't be kept exactly: the function has parameters, is
/// `async`, isn't an arrow function, has comments that would be removed, or runs more than one
/// expression. Also returns `None` when the code would be a method handler, as `foo` in
/// `() => foo`, because Vue would then call it.
fn inline_handler_body(function: &AnyJsInlineFunction) -> Option<AnyJsExpression> {
    let AnyJsInlineFunction::JsArrowFunctionExpression(arrow) = function else {
        return None;
    };
    if arrow.async_token().is_some() || function.parameter_count() > 0 {
        return None;
    }
    let parameters = arrow.parameters().ok()?;
    let fat_arrow = arrow.fat_arrow_token().ok()?;
    if parameters.syntax().has_comments_descendants()
        || fat_arrow.has_leading_comments()
        || fat_arrow.has_trailing_comments()
    {
        return None;
    }
    let body = match arrow.body().ok()? {
        AnyJsFunctionBody::AnyJsExpression(expression) => expression,
        AnyJsFunctionBody::JsFunctionBody(body) => {
            if body.syntax().has_comments_descendants() || !body.directives().is_empty() {
                return None;
            }
            let mut statements = body.statements().iter();
            let AnyJsStatement::JsExpressionStatement(statement) = statements.next()? else {
                return None;
            };
            if statements.next().is_some() {
                return None;
            }
            statement.expression().ok()?
        }
    };
    if body
        .inner_expression()
        .is_none_or(|inner| METHOD_HANDLER_KINDS.matches(inner.syntax().kind()))
    {
        return None;
    }
    Some(body)
}

/// The kinds of expression that Vue calls as a method handler when one makes up a whole
/// `v-on` value.
const METHOD_HANDLER_KINDS: SyntaxKindSet<JsLanguage> = JsIdentifierExpression::KIND_SET
    .union(JsStaticMemberExpression::KIND_SET)
    .union(JsComputedMemberExpression::KIND_SET)
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsFunctionExpression::KIND_SET);
