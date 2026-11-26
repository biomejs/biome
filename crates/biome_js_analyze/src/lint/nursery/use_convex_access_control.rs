use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{CanBeImportedExported, SemanticModel};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody, AnyJsNamedImportSpecifier,
    AnyJsObjectMember, AnyJsStatement,
    JsCallExpression, JsExportDefaultExpressionClause, JsVariableDeclarator,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::use_convex_access_control::UseConvexAccessControlOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require public Convex functions to call an access control function.
    ///
    /// Anyone on the internet can call an exported `query`, `mutation` or `action`, so a missing
    /// access check lets any user read or change data they shouldn't have access to. The rule
    /// checks these functions when they're imported from `_generated/server`. Internal functions,
    /// such as `internalMutation`, are ignored.
    ///
    /// Access control functions are recognized by name. By default, these match: `requireUser`,
    /// `assertAdmin`, `checkAccess`, `ensureOwner`, `canEditNote`, `hasRole` and `authz.require`.
    /// See the `prefixes` option for the exact rule. A plain `require(...)` call never counts.
    ///
    /// The call must be at the top level of the handler: on its own, in a variable declaration,
    /// in a `return` statement, or in the condition of an `if` statement. Calls inside an `if`
    /// body, a `try` block or another function aren't detected. The rule can't tell whether the
    /// check is the right one for the function.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { mutation } from "./_generated/server";
    ///
    /// export const send = mutation({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         await ctx.db.insert("messages", { body: "hello" });
    ///     },
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { internalMutation, mutation, query } from "./_generated/server";
    ///
    /// export const list = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         const user = await requireUser(ctx);
    ///     },
    /// });
    ///
    /// export const update = mutation({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         if (!(await canEditNote(ctx))) {
    ///             throw new Error("Unauthorized");
    ///         }
    ///     },
    /// });
    ///
    /// export const remove = mutation({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         await authz.require(ctx, "documents:delete");
    ///     },
    /// });
    ///
    /// export const cleanup = internalMutation({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         await ctx.db.delete("messages", messageId);
    ///     },
    /// });
    /// ```
    ///
    /// ## Options
    ///
    /// ### `functions`
    ///
    /// Type: `string[]`
    ///
    /// Default: `[]`
    ///
    /// Names of functions that count as access control functions, in addition to the names
    /// matched by `prefixes`. The whole name must match.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "functions": ["getCurrentUserOrThrow"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { query } from "./_generated/server";
    ///
    /// export const list = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         await getCurrentUser(ctx);
    ///     },
    /// });
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { query } from "./_generated/server";
    ///
    /// export const list = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         await getCurrentUserOrThrow(ctx);
    ///     },
    /// });
    /// ```
    ///
    /// ### `prefixes`
    ///
    /// Type: `string[]`
    ///
    /// Default: `["assert", "can", "check", "ensure", "has", "require"]`
    ///
    /// Words that start the name of an access control function. A name matches a prefix when it's
    /// the prefix on its own, or the prefix followed by an uppercase letter or `_`. A plain
    /// `has(...)` call never counts, because it's usually a `Map` or `Set` lookup. Setting this
    /// option replaces the default prefixes.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "prefixes": ["authorize"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import { query } from "./_generated/server";
    ///
    /// export const list = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         await requireUser(ctx);
    ///     },
    /// });
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// import { query } from "./_generated/server";
    ///
    /// export const list = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         await authorizeUser(ctx);
    ///     },
    /// });
    /// ```
    ///
    pub UseConvexAccessControl {
        version: "next",
        name: "useConvexAccessControl",
        language: "js",
        sources: &[RuleSource::EslintConvex("require-access-control").inspired()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex],
    }
}

declare_node_union! {
    /// A place where an exported value can be defined: `const name = ...` or
    /// `export default ...`.
    pub AnyConvexFunctionDefinition = JsVariableDeclarator | JsExportDefaultExpressionClause
}

impl Rule for UseConvexAccessControl {
    type Query = Semantic<AnyConvexFunctionDefinition>;
    type State = ConvexFunction;
    type Signals = Option<Self::State>;
    type Options = UseConvexAccessControlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let expression = match ctx.query() {
            AnyConvexFunctionDefinition::JsVariableDeclarator(declarator) => {
                let binding = declarator.id().ok()?;
                let binding = binding.as_any_js_binding()?.as_js_identifier_binding()?;
                if !binding.is_exported(model) {
                    return None;
                }
                declarator.initializer()?.expression().ok()?
            }
            AnyConvexFunctionDefinition::JsExportDefaultExpressionClause(clause) => {
                clause.expression().ok()?
            }
        };
        let call = expression
            .omit_type_wrappers()?
            .as_js_call_expression()?
            .clone();
        let (function, handler) = public_convex_function(&call, model)?;
        let options = ctx.options();
        if has_access_control(&handler, options) {
            return None;
        }
        Some(function)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let kind = state.kind;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.callee_range,
            markup! {
                "This public Convex "{kind}" doesn't call an access control function."
            },
        )
        .note(markup! {
            "Anyone on the internet can call this function, so without an access control check, any user can read or change data they shouldn't have access to."
        });
        Some(if ctx.options().prefixes.is_some() {
            diagnostic.note(markup! {
                "Call one of your access control functions at the top level of the handler. If this function shouldn't be callable from outside, use "<Emphasis>{kind.internal_registrar()}</Emphasis>" instead."
            })
        } else {
            diagnostic.note(markup! {
                "Call an access control function, such as "<Emphasis>"requireUser(ctx)"</Emphasis>" or "<Emphasis>"canEditNote(ctx)"</Emphasis>", at the top level of the handler. If this function shouldn't be callable from outside, use "<Emphasis>{kind.internal_registrar()}</Emphasis>" instead."
            })
        })
    }
}

/// An exported public Convex function whose handler doesn't call an access control function.
pub struct ConvexFunction {
    kind: ConvexFunctionKind,
    /// The range of the `query`, `mutation` or `action` reference that creates the function.
    callee_range: TextRange,
}

#[derive(Clone, Copy)]
enum ConvexFunctionKind {
    Action,
    Mutation,
    Query,
}

impl ConvexFunctionKind {
    fn from_registrar(name: &str) -> Option<Self> {
        match name {
            "action" => Some(Self::Action),
            "mutation" => Some(Self::Mutation),
            "query" => Some(Self::Query),
            _ => None,
        }
    }

    const fn internal_registrar(self) -> &'static str {
        match self {
            Self::Action => "internalAction",
            Self::Mutation => "internalMutation",
            Self::Query => "internalQuery",
        }
    }
}

impl biome_console::fmt::Display for ConvexFunctionKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Action => "action",
            Self::Mutation => "mutation",
            Self::Query => "query",
        })
    }
}

/// Returns the function and its handler body when `call` creates a public Convex function, such
/// as `mutation({ handler: async (ctx) => {} })` or `mutation(async (ctx) => {})`.
///
/// The callee must be `query`, `mutation` or `action` imported from a `_generated/server` file.
/// Returns `None` when the handler isn't written inline, because the rule can't check it.
fn public_convex_function(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> Option<(ConvexFunction, AnyJsFunctionBody)> {
    let callee = call.callee().ok()?.omit_parentheses();
    let reference = callee.as_js_identifier_expression()?.name().ok()?;
    let specifier = match model.binding(&reference)?.tree().declaration()? {
        AnyJsBindingDeclaration::JsNamedImportSpecifier(specifier) => {
            AnyJsNamedImportSpecifier::from(specifier)
        }
        AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(specifier) => specifier.into(),
        _ => return None,
    };
    let source = specifier
        .import_clause()?
        .source()
        .ok()?
        .inner_string_text()
        .ok()?;
    if !is_convex_server_module(source.text()) {
        return None;
    }
    let kind = ConvexFunctionKind::from_registrar(specifier.imported_name()?.text_trimmed())?;

    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }
    let AnyJsCallArgument::AnyJsExpression(argument) = arguments.first()?.ok()? else {
        return None;
    };
    let argument = argument.omit_type_wrappers()?;
    let handler = match &argument {
        AnyJsExpression::JsObjectExpression(object) => match object.find_member("handler")? {
            AnyJsObjectMember::JsPropertyObjectMember(property) => {
                function_body(&property.value().ok()?.omit_type_wrappers()?)?
            }
            AnyJsObjectMember::JsMethodObjectMember(method) => method.body().ok()?.into(),
            _ => return None,
        },
        argument => function_body(argument)?,
    };
    Some((
        ConvexFunction {
            kind,
            callee_range: reference.range(),
        },
        handler,
    ))
}

fn function_body(expression: &AnyJsExpression) -> Option<AnyJsFunctionBody> {
    match expression {
        AnyJsExpression::JsArrowFunctionExpression(function) => function.body().ok(),
        AnyJsExpression::JsFunctionExpression(function) => Some(function.body().ok()?.into()),
        _ => None,
    }
}

/// Returns `true` when a top-level statement of `body` calls an access control function.
fn has_access_control(body: &AnyJsFunctionBody, options: &UseConvexAccessControlOptions) -> bool {
    let body = match body {
        AnyJsFunctionBody::AnyJsExpression(expression) => {
            return is_access_control_call(expression, options);
        }
        AnyJsFunctionBody::JsFunctionBody(body) => body,
    };
    body.statements().iter().any(|statement| match statement {
        AnyJsStatement::JsExpressionStatement(statement) => statement
            .expression()
            .is_ok_and(|expression| is_access_control_call(&expression, options)),
        AnyJsStatement::JsVariableStatement(statement) => {
            statement.declaration().is_ok_and(|declaration| {
                declaration.declarators().iter().flatten().any(|declarator| {
                    declarator
                        .initializer()
                        .and_then(|initializer| initializer.expression().ok())
                        .is_some_and(|expression| is_access_control_call(&expression, options))
                })
            })
        }
        AnyJsStatement::JsReturnStatement(statement) => statement
            .argument()
            .is_some_and(|expression| is_access_control_call(&expression, options)),
        // A boolean check acts as access control when it decides whether the handler goes on,
        // such as `if (!(await canEdit(ctx)) || isBanned) throw ...`.
        AnyJsStatement::JsIfStatement(statement) => statement
            .test()
            .is_ok_and(|test| contains_access_control_call(test, options)),
        _ => false,
    })
}

/// Returns `true` when `expression` contains an access control call that is only wrapped in
/// `await`, `!`, logical operators or binary operators.
fn contains_access_control_call(
    expression: AnyJsExpression,
    options: &UseConvexAccessControlOptions,
) -> bool {
    let mut stack = vec![expression];
    while let Some(expression) = stack.pop() {
        if is_access_control_call(&expression, options) {
            return true;
        }
        match expression.omit_type_wrappers() {
            Some(AnyJsExpression::JsAwaitExpression(expression)) => {
                stack.extend(expression.argument().ok());
            }
            Some(AnyJsExpression::JsUnaryExpression(expression)) => {
                stack.extend(expression.argument().ok());
            }
            Some(AnyJsExpression::JsLogicalExpression(expression)) => {
                stack.extend(expression.left().ok());
                stack.extend(expression.right().ok());
            }
            Some(AnyJsExpression::JsBinaryExpression(expression)) => {
                stack.extend(expression.left().ok());
                stack.extend(expression.right().ok());
            }
            _ => {}
        }
    }
    false
}

/// Returns `true` when `expression` is a call to an access control function, optionally
/// awaited, such as `await requireUser(ctx)` or `authz.can(ctx)`.
fn is_access_control_call(
    expression: &AnyJsExpression,
    options: &UseConvexAccessControlOptions,
) -> bool {
    let mut expression = expression.clone();
    let call = loop {
        match expression.omit_type_wrappers() {
            Some(AnyJsExpression::JsAwaitExpression(await_expression)) => {
                let Ok(argument) = await_expression.argument() else {
                    return false;
                };
                expression = argument;
            }
            Some(AnyJsExpression::JsCallExpression(call)) => break call,
            _ => return false,
        }
    };
    let Ok(callee) = call.callee() else {
        return false;
    };
    let name = match callee.omit_parentheses() {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let Ok(name) = identifier.name().and_then(|name| name.value_token()) else {
                return false;
            };
            // A bare `require(...)` loads a CommonJS module.
            if name.text_trimmed() == "require" {
                return false;
            }
            name
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let Some(name) = member
                .member()
                .ok()
                .and_then(|member| member.as_js_name()?.value_token().ok())
            else {
                return false;
            };
            name
        }
        _ => return false,
    };
    let name = name.text_trimmed();
    if options
        .functions()
        .iter()
        .any(|function| function.as_ref() == name)
    {
        return true;
    }
    // A plain `has(...)` call is usually a `Map` or `Set` lookup.
    if name == "has" {
        return false;
    }
    match options.prefixes.as_deref() {
        Some(prefixes) => prefixes
            .iter()
            .any(|prefix| starts_with_word(name, prefix)),
        None => {
            // The default prefixes are lowercase, so the prefix of `name` ends at the first
            // uppercase letter or `_`.
            let prefix_end = name
                .bytes()
                .position(|byte| byte.is_ascii_uppercase() || byte == b'_')
                .unwrap_or(name.len());
            UseConvexAccessControlOptions::DEFAULT_PREFIXES
                .binary_search(&&name[..prefix_end])
                .is_ok()
        }
    }
}

/// Returns `true` when `name` is `prefix`, or `prefix` followed by an uppercase letter or `_`.
fn starts_with_word(name: &str, prefix: &str) -> bool {
    name.strip_prefix(prefix).is_some_and(|rest| {
        rest.bytes()
            .next()
            .is_none_or(|byte| byte.is_ascii_uppercase() || byte == b'_')
    })
}

/// Returns `true` when `source` imports the file that Convex generates to export `query`,
/// `mutation` and `action`, such as `./_generated/server` or `../convex/_generated/server.js`.
fn is_convex_server_module(source: &str) -> bool {
    source
        .strip_suffix(".js")
        .unwrap_or(source)
        .ends_with("/_generated/server")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_prefixes_are_sorted() {
        for pair in UseConvexAccessControlOptions::DEFAULT_PREFIXES.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must come before {}",
                pair[1],
                pair[0]
            );
        }
    }
}
