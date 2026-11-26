use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsFormalParameter, AnyJsObjectMember, AnyJsObjectMemberName, AnyJsParameter,
    JsArrowFunctionExpression, JsCallExpression, JsExportDefaultExpressionClause,
    JsFunctionExpression, JsMethodObjectMember, JsObjectExpression, JsSyntaxToken,
    JsVariableDeclarator, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, declare_node_union};
use biome_rule_options::use_convex_args_validator::UseConvexArgsValidatorOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Require an `args` validator in Convex queries, mutations, and actions.
    ///
    /// Convex checks the arguments of every call against the `args` validator before the
    /// handler runs. Without it, callers can pass any value, because TypeScript types aren't
    /// checked at runtime. A function without arguments should use `args: {}`.
    ///
    /// This rule checks exported `query`, `mutation`, `action`, `internalQuery`,
    /// `internalMutation`, and `internalAction` functions, including the older syntax
    /// `query(async (ctx) => {})`, which can't define `args`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// export const get = query({
    ///     handler: async (ctx, { id }: { id: Id<"tasks"> }) => {
    ///         return await ctx.db.get(id);
    ///     },
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export const list = query({
    ///     handler: async (ctx) => {
    ///         return await ctx.db.query("tasks").collect();
    ///     },
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export const list = query(async (ctx) => {
    ///     return await ctx.db.query("tasks").collect();
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export const get = query({
    ///     args: { id: v.id("tasks") },
    ///     handler: async (ctx, { id }) => {
    ///         return await ctx.db.get(id);
    ///     },
    /// });
    ///
    /// export const list = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         return await ctx.db.query("tasks").collect();
    ///     },
    /// });
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreUnusedArguments`
    ///
    /// Type: `boolean`
    ///
    /// Default: `false`
    ///
    /// When set to `true`, the rule allows a function without `args` if its handler doesn't
    /// take an arguments parameter. Functions whose handler takes arguments are still
    /// reported.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreUnusedArguments": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// export const get = query({
    ///     handler: async (ctx, { id }) => {
    ///         return await ctx.db.get(id);
    ///     },
    /// });
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// export const list = query({
    ///     handler: async (ctx) => {
    ///         return await ctx.db.query("tasks").collect();
    ///     },
    /// });
    /// ```
    ///
    pub UseConvexArgsValidator {
        version: "next",
        name: "useConvexArgsValidator",
        language: "js",
        sources: &[RuleSource::EslintConvex("require-args-validator").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Convex],
    }
}

declare_node_union! {
    /// The exports that can register a Convex function: `export const name = query(...)` and
    /// `export default query(...)`.
    pub AnyConvexFunctionExport = JsVariableDeclarator | JsExportDefaultExpressionClause
}

impl Rule for UseConvexArgsValidator {
    type Query = Ast<AnyConvexFunctionExport>;
    type State = MissingArgsValidator;
    type Signals = Option<Self::State>;
    type Options = UseConvexArgsValidatorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = match ctx.query() {
            AnyConvexFunctionExport::JsVariableDeclarator(declarator) => {
                AnyJsBindingDeclaration::from(declarator.clone()).export()?;
                declarator.initializer()?.expression().ok()?
            }
            AnyConvexFunctionExport::JsExportDefaultExpressionClause(clause) => {
                clause.expression().ok()?
            }
        };
        let AnyJsExpression::JsCallExpression(call) = expression.omit_type_wrappers()? else {
            return None;
        };
        let callee = call.callee().ok()?;
        let callee_name = callee
            .as_js_identifier_expression()?
            .name()
            .ok()?
            .value_token()
            .ok()?;
        CONVEX_FUNCTION_BUILDERS
            .binary_search(&callee_name.text_trimmed())
            .ok()?;
        let arguments = call.arguments().ok()?.args();
        if arguments.len() != 1 {
            return None;
        }
        let Some(Ok(AnyJsCallArgument::AnyJsExpression(argument))) = arguments.first() else {
            return None;
        };

        let (definition, handler_arguments) = match argument.omit_type_wrappers()? {
            AnyJsExpression::JsObjectExpression(object) => {
                // A spread or a computed name might add `args`.
                if object
                    .members()
                    .iter()
                    .flatten()
                    .any(|member| member.name().is_none_or(|name| name.text() == "args"))
                {
                    return None;
                }
                let handler_arguments = object
                    .find_member("handler")
                    .and_then(AnyConvexHandler::from_member)
                    .map_or(HandlerArguments::Unknown, |handler| handler.arguments());
                (ConvexFunctionDefinition::Object(object), handler_arguments)
            }
            function @ (AnyJsExpression::JsArrowFunctionExpression(_)
            | AnyJsExpression::JsFunctionExpression(_)) => {
                let handler_arguments = AnyConvexHandler::cast_ref(function.syntax())?.arguments();
                (
                    ConvexFunctionDefinition::Handler(function),
                    handler_arguments,
                )
            }
            _ => return None,
        };

        if handler_arguments != HandlerArguments::Used && ctx.options().ignore_unused_arguments() {
            return None;
        }

        Some(MissingArgsValidator {
            call,
            definition,
            handler_arguments,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.call.callee().ok()?.range(),
                markup! {
                    "This Convex function doesn't have an "<Emphasis>"args"</Emphasis>" validator."
                },
            )
            .note(markup! {
                "Without an "<Emphasis>"args"</Emphasis>" validator, Convex doesn't check the arguments of a call before the handler runs, so the handler can receive values of any type."
            })
            .note(match (&state.definition, state.handler_arguments) {
                (ConvexFunctionDefinition::Handler(_), HandlerArguments::Used) => markup! {
                    "Replace the handler with an object that has an "<Emphasis>"args"</Emphasis>" property describing each argument, for example "<Emphasis>"args: { id: v.id(\"tasks\") }"</Emphasis>", and a "<Emphasis>"handler"</Emphasis>" property with the handler."
                },
                (ConvexFunctionDefinition::Handler(_), _) => markup! {
                    "Replace the handler with an object that has the properties "<Emphasis>"args: {}"</Emphasis>" and "<Emphasis>"handler"</Emphasis>", so that Convex rejects calls that pass arguments."
                },
                (ConvexFunctionDefinition::Object(_), HandlerArguments::Used) => markup! {
                    "Add an "<Emphasis>"args"</Emphasis>" property that describes each argument, for example "<Emphasis>"args: { id: v.id(\"tasks\") }"</Emphasis>"."
                },
                (ConvexFunctionDefinition::Object(_), HandlerArguments::Unused) => markup! {
                    "Add "<Emphasis>"args: {}"</Emphasis>" so that Convex rejects calls that pass arguments."
                },
                (ConvexFunctionDefinition::Object(_), HandlerArguments::Unknown) => markup! {
                    "Add an "<Emphasis>"args"</Emphasis>" property that describes each argument, or "<Emphasis>"args: {}"</Emphasis>" if the function doesn't take arguments."
                },
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if state.handler_arguments != HandlerArguments::Unused {
            return None;
        }
        let mut mutation = ctx.root().begin();
        match &state.definition {
            ConvexFunctionDefinition::Object(object) => {
                let members = object.members();
                // Put `args` on its own line when the existing members are on separate lines.
                let leading_trivia = members.syntax().first_token()?.leading_trivia();
                let pieces: Vec<_> = leading_trivia.pieces().collect();
                let (name, comma) = match pieces.iter().rposition(|piece| piece.is_newline()) {
                    Some(newline) => (
                        make::ident("args").with_leading_trivia_pieces(pieces[newline..].to_vec()),
                        make::token(T![,]),
                    ),
                    None => (
                        make::ident("args"),
                        make::token(T![,]).with_trailing_space(),
                    ),
                };
                let mut items = vec![empty_args_member(name)];
                let mut separators = vec![comma];
                for element in members.elements() {
                    items.push(element.node().ok()?.clone());
                    separators.extend(element.trailing_separator().ok()?.cloned());
                }
                // The first existing member keeps its own leading trivia, so copying it onto
                // `args` as well would duplicate comments.
                mutation.replace_node_discard_trivia(
                    members,
                    make::js_object_member_list(items, separators),
                );
            }
            ConvexFunctionDefinition::Handler(handler) => {
                let handler_member = make::js_property_object_member(
                    AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(
                        make::ident("handler"),
                    )),
                    make::token(T![:]).with_trailing_space(),
                    // `replace_node` moves the handler's outer trivia onto the new object.
                    handler.clone().trim_comments_and_trivia()?,
                );
                let object = make::js_object_expression(
                    make::token(T!['{']).with_trailing_space(),
                    make::js_object_member_list(
                        [
                            empty_args_member(make::ident("args")),
                            AnyJsObjectMember::JsPropertyObjectMember(handler_member),
                        ],
                        [make::token(T![,]).with_trailing_space()],
                    ),
                    make::token(T!['}']).with_leading_space(),
                );
                mutation.replace_node(handler.clone(), AnyJsExpression::JsObjectExpression(object));
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add an empty "<Emphasis>"args"</Emphasis>" validator." },
            mutation,
        ))
    }
}

/// A Convex function registration that doesn't define `args`.
pub struct MissingArgsValidator {
    /// The call to `query`, `mutation`, or another function that registers a Convex function.
    call: JsCallExpression,
    definition: ConvexFunctionDefinition,
    handler_arguments: HandlerArguments,
}

/// The argument passed to the function that registers a Convex function.
enum ConvexFunctionDefinition {
    /// An object with the function's properties, as in `query({ handler })`.
    Object(JsObjectExpression),
    /// The handler itself, as in `query(async (ctx) => {})`. This older syntax can't define
    /// `args`.
    Handler(AnyJsExpression),
}

/// Whether the handler of a Convex function takes an arguments parameter.
#[derive(Clone, Copy, Eq, PartialEq)]
enum HandlerArguments {
    /// The handler has a second parameter, as in `(ctx, { id }) => {}`, or a rest parameter
    /// that receives the arguments, as in `(...params) => {}`.
    Used,
    /// The handler has no second parameter, or the second parameter is an empty object
    /// pattern, as in `(ctx, {}) => {}`.
    Unused,
    /// The handler's parameters aren't visible, as in `query({ handler })` or `query({})`.
    Unknown,
}

declare_node_union! {
    /// A handler function written directly in a Convex function registration.
    AnyConvexHandler = JsArrowFunctionExpression | JsFunctionExpression | JsMethodObjectMember
}

impl AnyConvexHandler {
    /// Returns the handler function defined by the `handler` member of a Convex function
    /// object, or `None` if the member's value isn't a function expression.
    fn from_member(member: AnyJsObjectMember) -> Option<Self> {
        match member {
            AnyJsObjectMember::JsPropertyObjectMember(property) => {
                match property.value().ok()?.omit_type_wrappers()? {
                    AnyJsExpression::JsArrowFunctionExpression(function) => Some(function.into()),
                    AnyJsExpression::JsFunctionExpression(function) => Some(function.into()),
                    _ => None,
                }
            }
            AnyJsObjectMember::JsMethodObjectMember(method) => Some(method.into()),
            _ => None,
        }
    }

    /// Returns whether the handler takes an arguments parameter, its second parameter after
    /// the Convex context. A TypeScript `this` parameter doesn't count, and a rest parameter
    /// in the first or second position, as in `(...params) => {}`, receives the arguments.
    fn arguments(&self) -> HandlerArguments {
        let parameters = match self {
            Self::JsArrowFunctionExpression(function) => match function.parameters() {
                Ok(AnyJsArrowFunctionParameters::JsParameters(parameters)) => Ok(parameters),
                Ok(AnyJsArrowFunctionParameters::AnyJsBinding(_)) => {
                    return HandlerArguments::Unused;
                }
                Err(error) => Err(error),
            },
            Self::JsFunctionExpression(function) => function.parameters(),
            Self::JsMethodObjectMember(method) => method.parameters(),
        };
        let Ok(parameters) = parameters else {
            return HandlerArguments::Unknown;
        };
        let mut parameters = parameters
            .items()
            .iter()
            .flatten()
            .filter(|parameter| parameter.as_ts_this_parameter().is_none());
        if parameters
            .next()
            .is_some_and(|parameter| parameter.as_js_rest_parameter().is_some())
        {
            return HandlerArguments::Used;
        }
        match parameters.next() {
            None => HandlerArguments::Unused,
            Some(AnyJsParameter::AnyJsFormalParameter(
                AnyJsFormalParameter::JsFormalParameter(parameter),
            )) if matches!(
                parameter.binding(),
                Ok(AnyJsBindingPattern::JsObjectBindingPattern(pattern))
                    if pattern.properties().is_empty()
            ) =>
            {
                HandlerArguments::Unused
            }
            Some(_) => HandlerArguments::Used,
        }
    }
}

/// Creates the property `args: {}` with `name` as its name token.
fn empty_args_member(name: JsSyntaxToken) -> AnyJsObjectMember {
    AnyJsObjectMember::JsPropertyObjectMember(make::js_property_object_member(
        AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(name)),
        make::token(T![:]).with_trailing_space(),
        AnyJsExpression::JsObjectExpression(make::js_object_expression(
            make::token(T!['{']),
            make::js_object_member_list([], []),
            make::token(T!['}']),
        )),
    ))
}

/// The functions that register Convex queries, mutations, and actions, sorted for binary
/// search.
const CONVEX_FUNCTION_BUILDERS: &[&str] = &[
    "action",
    "internalAction",
    "internalMutation",
    "internalQuery",
    "mutation",
    "query",
];

#[cfg(test)]
mod tests {
    use super::CONVEX_FUNCTION_BUILDERS;

    #[test]
    fn convex_function_builders_are_sorted() {
        for pair in CONVEX_FUNCTION_BUILDERS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must come before {}",
                pair[1],
                pair[0]
            );
        }
    }
}
