use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsExpression, AnyJsFunction,
    JsCallExpression, JsExport, JsImport, JsInitializerClause, JsReferenceIdentifier, JsSyntaxKind,
    JsVariableDeclarationClause, JsVariableDeclarator, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::no_convex_old_registered_function_syntax::NoConvexOldRegisteredFunctionSyntaxOptions;
use camino::Utf8Path;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require the object syntax when registering Convex queries, mutations, and actions.
    ///
    /// Passing the handler directly, as in `query(async (ctx) => {})`, leaves no place for an
    /// `args` validator, so Convex can't check the arguments that clients send. Pass an object
    /// with `args` and `handler` properties instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export const list = query(async (ctx) => {
    ///     return await ctx.db.query("messages").collect();
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export const send = mutation(async (ctx, { body }) => {
    ///     await ctx.db.insert("messages", { body });
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export const list = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         return await ctx.db.query("messages").collect();
    ///     },
    /// });
    ///
    /// export const send = mutation({
    ///     args: { body: v.string() },
    ///     handler: async (ctx, { body }) => {
    ///         await ctx.db.insert("messages", { body });
    ///     },
    /// });
    /// ```
    ///
    pub NoConvexOldRegisteredFunctionSyntax {
        version: "next",
        name: "noConvexOldRegisteredFunctionSyntax",
        language: "js",
        sources: &[RuleSource::EslintConvex("no-old-registered-function-syntax").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Convex],
    }
}

impl Rule for NoConvexOldRegisteredFunctionSyntax {
    type Query = Semantic<JsCallExpression>;
    /// The function passed directly to the registration call, without parentheses.
    type State = AnyJsFunction;
    type Signals = Option<Self::State>;
    type Options = NoConvexOldRegisteredFunctionSyntaxOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?;
        let reference = callee.as_js_identifier_expression()?.name().ok()?;
        let name = reference.value_token().ok()?;
        if CONVEX_REGISTRARS
            .binary_search(&name.text_trimmed())
            .is_err()
        {
            return None;
        }

        let arguments = call.arguments().ok()?.args();
        if arguments.len() != 1 {
            return None;
        }
        let AnyJsCallArgument::AnyJsExpression(argument) = arguments.first()?.ok()? else {
            return None;
        };
        let function = match argument.omit_parentheses() {
            AnyJsExpression::JsArrowFunctionExpression(function) => AnyJsFunction::from(function),
            AnyJsExpression::JsFunctionExpression(function) => AnyJsFunction::from(function),
            _ => return None,
        };

        // Convex only registers functions that a module exports.
        call.syntax()
            .ancestors()
            .skip(1)
            .find(|node| node.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
            .and_then(JsInitializerClause::cast)?
            .parent::<JsVariableDeclarator>()?
            .declaration()?
            .parent::<JsVariableDeclarationClause>()?
            .parent::<JsExport>()?;

        if !is_entry_point(ctx.file_path()) || is_imported_from_package(ctx.model(), &reference) {
            return None;
        }

        Some(function)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().callee().ok()?.range(),
                markup! {
                    "This Convex function is registered with a plain function instead of an object."
                },
            )
            .note(markup! {
                "This syntax has no place for an "<Emphasis>"args"</Emphasis>" validator, so Convex can't check the arguments that clients send."
            })
            .note(markup! {
                "Pass an object instead, with the function in its "<Emphasis>"handler"</Emphasis>" property and the expected arguments in its "<Emphasis>"args"</Emphasis>" property."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, function: &Self::State) -> Option<JsRuleAction> {
        let parameter_count = match function.parameters().ok()? {
            AnyJsArrowFunctionParameters::AnyJsBinding(_) => 1,
            AnyJsArrowFunctionParameters::JsParameters(parameters) => parameters.items().len(),
        };

        let mut members = Vec::with_capacity(2);
        // A function with a second parameter reads arguments, so the fix can't know which
        // validator to add. Leaving out `args` lets the user write it.
        if parameter_count < 2 {
            members.push(
                make::js_property_object_member(
                    make::js_literal_member_name(make::ident("args")).into(),
                    make::token(T![:]).with_trailing_space(),
                    make::js_object_expression(
                        make::token(T!['{']),
                        make::js_object_member_list([], []),
                        make::token(T!['}']),
                    )
                    .into(),
                )
                .into(),
            );
        }
        // `replace_node` moves the trivia of the old function to the new object, so the
        // handler must not keep a copy of it.
        let function = AnyJsExpression::cast_ref(function.syntax())?;
        let handler = function
            .clone()
            .with_leading_trivia_pieces([])?
            .with_trailing_trivia_pieces([])?;
        members.push(
            make::js_property_object_member(
                make::js_literal_member_name(make::ident("handler")).into(),
                make::token(T![:]).with_trailing_space(),
                handler,
            )
            .into(),
        );
        let separators = vec![make::token(T![,]).with_trailing_space(); members.len() - 1];
        let object = make::js_object_expression(
            make::token(T!['{']).with_trailing_space(),
            make::js_object_member_list(members, separators),
            make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(function, object.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Move the function into the "<Emphasis>"handler"</Emphasis>" property of an object." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` when Convex would load the file at `path` as a module that defines functions.
///
/// Convex skips files in `_generated` folders, `schema.ts` and `schema.js`, files whose names
/// start with `.` or `#` or contain a space, and files whose names contain more than one dot, such
/// as `messages.test.ts`.
fn is_entry_point(path: &Utf8Path) -> bool {
    if path.parent().is_some_and(|parent| {
        parent
            .components()
            .any(|component| component.as_str() == "_generated")
    }) {
        return false;
    }
    let Some(name) = path.file_name() else {
        return false;
    };
    let Some(extension) = path.extension() else {
        return false;
    };
    ENTRY_POINT_EXTENSIONS.binary_search(&extension).is_ok()
        && !name.starts_with(['.', '#'])
        && !name.contains(' ')
        && name.bytes().filter(|byte| *byte == b'.').count() == 1
        && name != "schema.ts"
        && name != "schema.js"
}

/// Returns `true` when `reference` refers to an import from a package, such as `action` from
/// `mobx`. Imports from relative paths and from paths that contain `_generated`, the folder where
/// Convex generates its registration functions, return `false`.
fn is_imported_from_package(model: &SemanticModel, reference: &JsReferenceIdentifier) -> bool {
    model
        .binding(reference)
        .and_then(|binding| binding.syntax().ancestors().find_map(JsImport::cast))
        .and_then(|import| import.source_text().ok())
        .is_some_and(|source| {
            let source = source.text();
            !source.starts_with(['.', '/']) && !source.contains("_generated")
        })
}

/// The file extensions of files that Convex can load as modules that define functions, sorted for
/// binary search.
const ENTRY_POINT_EXTENSIONS: &[&str] = &["cjs", "cts", "js", "jsx", "mjs", "mts", "ts", "tsx"];

/// The functions that register Convex queries, mutations, and actions, sorted for binary search.
const CONVEX_REGISTRARS: &[&str] = &[
    "action",
    "internalAction",
    "internalMutation",
    "internalQuery",
    "mutation",
    "query",
];

#[cfg(test)]
mod tests {
    use super::{CONVEX_REGISTRARS, ENTRY_POINT_EXTENSIONS, is_entry_point};
    use camino::Utf8Path;

    #[test]
    fn entry_points() {
        for (path, expected) in [
            ("convex/messages.ts", true),
            ("convex/nested/messages.js", true),
            ("convex/_generated/server.js", false),
            ("convex/_generated/nested/api.ts", false),
            ("convex/schema.ts", false),
            ("convex/schema.js", false),
            ("convex/.hidden.ts", false),
            ("convex/#private.ts", false),
            ("convex/my messages.ts", false),
            ("convex/messages.test.ts", false),
            ("convex/types.d.ts", false),
            ("convex/messages.vue", false),
            ("convex/README", false),
        ] {
            assert_eq!(is_entry_point(Utf8Path::new(path)), expected, "{path}");
        }
    }

    #[test]
    fn convex_registrars_are_sorted() {
        for pair in CONVEX_REGISTRARS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "CONVEX_REGISTRARS is not sorted: {:?} must come before {:?}",
                pair[1],
                pair[0]
            );
        }
    }

    #[test]
    fn entry_point_extensions_are_sorted() {
        for pair in ENTRY_POINT_EXTENSIONS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "ENTRY_POINT_EXTENSIONS is not sorted: {:?} must come before {:?}",
                pair[1],
                pair[0]
            );
        }
    }
}
