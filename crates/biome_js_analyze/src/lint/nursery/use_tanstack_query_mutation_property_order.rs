use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
    utils::sorted_separated_list_by,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsNamedImportSpecifier, AnyJsObjectMember, JsCallExpression,
    JsIdentifierBinding, JsImport, JsObjectExpression, JsObjectMemberList, JsSyntaxToken, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange};
use biome_rule_options::use_tanstack_query_mutation_property_order::UseTanstackQueryMutationPropertyOrderOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Enforce that `onMutate` comes before `onError` and `onSettled` in the options of TanStack Query mutations.
    ///
    /// TanStack Query passes the value that `onMutate` returns to `onError` and `onSettled`.
    /// TypeScript works out the type of that value from the properties of the options object,
    /// in the order in which they are written. When `onMutate` comes after `onError` or
    /// `onSettled`, the type of the value in that function can be wrong. The other properties
    /// can be in any position, and `onError` and `onSettled` can be in either order.
    ///
    /// This rule checks the options object passed to `useMutation`, when `useMutation` is
    /// imported from a TanStack Query package such as `@tanstack/react-query`.
    ///
    /// :::note
    /// The rule only checks functions imported by name. It doesn't check calls through a
    /// namespace import such as `rq.useMutation()`.
    /// :::
    ///
    /// The code action moves `onMutate` in front of the first `onError` or `onSettled` property,
    /// and keeps the other properties in the same order. It is unsafe because it changes the
    /// order in which the property values are evaluated. When a spread such as `...sharedOptions`
    /// is written between the two properties, the code action can also change which of the
    /// spread and `onMutate` overrides the other. The code action isn't offered when it would
    /// move a property onto the same line as a comment that comes before it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useMutation } from "@tanstack/react-query";
    ///
    /// useMutation({
    ///     mutationFn: (todo) => saveTodo(todo),
    ///     onError: (error, todo, saved) => restoreTodo(saved.previousTodo),
    ///     onMutate: (todo) => ({ previousTodo: readTodo(todo.id) }),
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useMutation } from "@tanstack/react-query";
    ///
    /// useMutation({
    ///     onSettled: (data, error, todo, saved) => refreshTodo(saved.previousTodo),
    ///     mutationFn: (todo) => saveTodo(todo),
    ///     onMutate: (todo) => ({ previousTodo: readTodo(todo.id) }),
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useMutation } from "@tanstack/react-query";
    ///
    /// useMutation({
    ///     mutationFn: (todo) => saveTodo(todo),
    ///     onMutate: (todo) => ({ previousTodo: readTodo(todo.id) }),
    ///     onError: (error, todo, saved) => restoreTodo(saved.previousTodo),
    ///     onSettled: (data, error, todo, saved) => refreshTodo(saved.previousTodo),
    /// });
    ///
    /// useMutation({
    ///     onMutate: (todo) => ({ previousTodo: readTodo(todo.id) }),
    ///     onSettled: (data, error, todo, saved) => refreshTodo(saved.previousTodo),
    ///     onError: (error, todo, saved) => restoreTodo(saved.previousTodo),
    ///     mutationFn: (todo) => saveTodo(todo),
    /// });
    /// ```
    ///
    pub UseTanstackQueryMutationPropertyOrder {
        version: "2.6.9",
        name: "useTanstackQueryMutationPropertyOrder",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("mutation-property-order").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::TanstackQuery],
    }
}

/// An `onMutate` property that is written after an `onError` or `onSettled` property of the
/// same object.
pub struct MisplacedOnMutate {
    on_mutate: AnyJsObjectMember,
    /// The first `onError` or `onSettled` property of the object.
    first_dependent: AnyJsObjectMember,
}

impl Rule for UseTanstackQueryMutationPropertyOrder {
    type Query = Semantic<JsCallExpression>;
    type State = MisplacedOnMutate;
    type Signals = Option<Self::State>;
    type Options = UseTanstackQueryMutationPropertyOrderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();

        let mut first_dependent = None;
        for member in options_object(call)?.members().iter().flatten() {
            let Some(name) = member.name() else {
                continue;
            };
            match name.text() {
                "onError" | "onSettled" if first_dependent.is_none() => {
                    first_dependent = Some(member);
                }
                "onMutate" => {
                    if let Some(first_dependent) = first_dependent {
                        // Resolving the import is the most expensive check, so it runs last.
                        return is_use_mutation(call, ctx.model()).then_some(MisplacedOnMutate {
                            on_mutate: member,
                            first_dependent,
                        });
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let dependent_name = state.first_dependent.name()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                name_range(&state.on_mutate)?,
                markup! {
                    "This "<Emphasis>"onMutate"</Emphasis>" property comes after "<Emphasis>{dependent_name.text()}</Emphasis>"."
                },
            )
            .note(markup! {
                "TanStack Query passes the value that "<Emphasis>"onMutate"</Emphasis>" returns to "<Emphasis>"onError"</Emphasis>" and "<Emphasis>"onSettled"</Emphasis>". The type of that value is worked out from the properties in the order in which they are written, so it can be wrong in a property that comes before "<Emphasis>"onMutate"</Emphasis>"."
            })
            .note(markup! {
                "Move "<Emphasis>"onMutate"</Emphasis>" before "<Emphasis>"onError"</Emphasis>" and "<Emphasis>"onSettled"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let MisplacedOnMutate {
            on_mutate,
            first_dependent,
        } = state;
        let dependent_name = first_dependent.name()?;
        let members = on_mutate.parent::<JsObjectMemberList>()?;
        let destination = first_dependent.range().start();

        // The properties move together with their comments, which puts three tokens behind a
        // different token: `onMutate` takes the place of `first_dependent`, `first_dependent`
        // takes the place of the token after `onMutate` and its separator, and that token takes
        // the place of `onMutate`.
        let on_mutate_start = on_mutate.syntax().first_token()?;
        let dependent_start = first_dependent.syntax().first_token()?;
        let after_on_mutate = on_mutate
            .syntax()
            .last_token()?
            .next_token()
            .and_then(|token| match token.kind() {
                T![,] => token.next_token(),
                _ => Some(token),
            })?;
        if !(can_replace(&on_mutate_start, &dependent_start)
            && can_replace(&dependent_start, &after_on_mutate)
            && can_replace(&after_on_mutate, &on_mutate_start))
        {
            return None;
        }

        // A separator is only created when `onMutate` was the last property and had no trailing
        // comma. The new separator ends up right in front of `first_dependent`, which needs a
        // space before it unless it starts on a new line.
        let make_separator = if dependent_start.has_leading_newline() {
            || make::token(T![,])
        } else {
            || make::token(T![,]).with_trailing_space()
        };
        let sorted_members = sorted_separated_list_by(
            &members,
            |member| {
                Some(if member == on_mutate {
                    Placement::Moved
                } else if member.range().start() < destination {
                    Placement::Before
                } else {
                    Placement::After
                })
            },
            make_separator,
            Placement::cmp,
        )
        .ok()?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(members, sorted_members);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Move "<Emphasis>"onMutate"</Emphasis>" before "<Emphasis>{dependent_name.text()}</Emphasis>"."
            },
            mutation,
        ))
    }
}

/// Where a property of the options object goes when the misplaced `onMutate` property moves.
///
/// The derived ordering follows the declaration order of the variants. Sorting the properties
/// by their placement keeps the order of the properties that share a placement.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Placement {
    /// The property is written before the first `onError` or `onSettled` property.
    Before,
    /// The property is the misplaced `onMutate` property.
    Moved,
    /// The property is the first `onError` or `onSettled` property, or is written after it.
    After,
}

/// Returns whether `token` can come right after the token that precedes `replaced`.
///
/// A comment after the preceding token can run to the end of its line. `token` must then start
/// on a new line, or it would become part of that comment.
fn can_replace(token: &JsSyntaxToken, replaced: &JsSyntaxToken) -> bool {
    token.has_leading_newline()
        || replaced
            .prev_token()
            .is_none_or(|previous| !previous.has_trailing_comments())
}

/// Returns the options object passed as the first argument of `call`.
fn options_object(call: &JsCallExpression) -> Option<JsObjectExpression> {
    let first = call.arguments().ok()?.args().first()?.ok()?;
    let AnyJsCallArgument::AnyJsExpression(first) = first else {
        return None;
    };
    first.omit_parentheses().as_js_object_expression().cloned()
}

/// Returns the range of the name of `member`, or `None` if `member` has no name, like a spread.
fn name_range(member: &AnyJsObjectMember) -> Option<TextRange> {
    Some(match member {
        AnyJsObjectMember::JsGetterObjectMember(member) => member.name().ok()?.range(),
        AnyJsObjectMember::JsMethodObjectMember(member) => member.name().ok()?.range(),
        AnyJsObjectMember::JsPropertyObjectMember(member) => member.name().ok()?.range(),
        AnyJsObjectMember::JsSetterObjectMember(member) => member.name().ok()?.range(),
        AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => member.name().ok()?.range(),
        AnyJsObjectMember::JsBogusMember(_)
        | AnyJsObjectMember::JsMetavariable(_)
        | AnyJsObjectMember::JsSpread(_) => return None,
    })
}

/// Returns whether `call` calls `useMutation` imported by name from a TanStack Query package.
///
/// The function is identified by its exported name, so a renamed import such as
/// `import { useMutation as useSave }` is recognized too.
fn is_use_mutation(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let Some(reference) = call
        .callee()
        .ok()
        .and_then(|callee| callee.inner_expression())
        .and_then(|callee| callee.as_js_reference_identifier())
    else {
        return false;
    };
    let Some(specifier) = model
        .binding(&reference)
        .and_then(|binding| JsIdentifierBinding::cast(binding.syntax()))
        .and_then(|binding| binding.parent::<AnyJsNamedImportSpecifier>())
    else {
        return false;
    };
    if specifier.imports_only_types()
        || specifier
            .imported_name()
            .is_none_or(|name| name.text_trimmed() != "useMutation")
    {
        return false;
    }

    specifier
        .import_clause()
        .and_then(|clause| clause.parent::<JsImport>())
        .and_then(|import| import.source_text().ok())
        .is_some_and(|source| {
            source
                .text()
                .strip_prefix("@tanstack/")
                .is_some_and(|package| package.ends_with("-query"))
        })
}
