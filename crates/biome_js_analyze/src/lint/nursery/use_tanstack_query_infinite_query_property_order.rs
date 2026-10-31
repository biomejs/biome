use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsNamedImportSpecifier, AnyJsObjectMember,
    JsCallExpression, JsIdentifierBinding, JsImport,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_tanstack_query_infinite_query_property_order::UseTanstackQueryInfiniteQueryPropertyOrderOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Enforce a consistent property order in the options of TanStack Query infinite query functions.
    ///
    /// TanStack Query infers the types of an infinite query from the properties of its options
    /// object, in the order in which they are written. `queryFn` must come first, followed by
    /// `getPreviousPageParam` and `getNextPageParam`. A different order can make the inferred
    /// types of the page parameters wrong. The other properties can be in any position.
    ///
    /// This rule checks the options passed to the following functions, when they are imported
    /// from a TanStack Query package such as `@tanstack/react-query`:
    ///
    /// - `useInfiniteQuery`
    /// - `useSuspenseInfiniteQuery`
    /// - `infiniteQueryOptions`
    ///
    /// :::note
    /// The rule only checks functions imported by name. It doesn't check calls through a
    /// namespace import such as `rq.useInfiniteQuery()`.
    /// :::
    ///
    /// The code action moves the three properties to the positions they already occupy, in the
    /// expected order, and leaves the other properties where they are. It is unsafe because it
    /// changes the order in which the property values are evaluated.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useInfiniteQuery } from "@tanstack/react-query";
    ///
    /// useInfiniteQuery({
    ///     queryKey: ["projects"],
    ///     getNextPageParam: (lastPage) => lastPage.nextId,
    ///     queryFn: ({ pageParam }) => fetchProjects(pageParam),
    ///     initialPageParam: 0,
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { infiniteQueryOptions } from "@tanstack/react-query";
    ///
    /// infiniteQueryOptions({
    ///     queryFn: ({ pageParam }) => fetchProjects(pageParam),
    ///     getNextPageParam: (lastPage) => lastPage.nextId,
    ///     getPreviousPageParam: (firstPage) => firstPage.previousId,
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useInfiniteQuery } from "@tanstack/react-query";
    ///
    /// useInfiniteQuery({
    ///     queryKey: ["projects"],
    ///     queryFn: ({ pageParam }) => fetchProjects(pageParam),
    ///     initialPageParam: 0,
    ///     getPreviousPageParam: (firstPage) => firstPage.previousId,
    ///     getNextPageParam: (lastPage) => lastPage.nextId,
    /// });
    /// ```
    ///
    pub UseTanstackQueryInfiniteQueryPropertyOrder {
        version: "next",
        name: "useTanstackQueryInfiniteQueryPropertyOrder",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("infinite-query-property-order").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::TanstackQuery],
    }
}

/// A property that is moved to the position of another property.
type Replacement = (AnyJsObjectMember, AnyJsObjectMember);

impl Rule for UseTanstackQueryInfiniteQueryPropertyOrder {
    type Query = Semantic<JsCallExpression>;
    type State = Box<[Replacement]>;
    type Signals = Option<Self::State>;
    type Options = UseTanstackQueryInfiniteQueryPropertyOrderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if !is_infinite_query_function(call, ctx.model()) {
            return None;
        }

        let arguments = call.arguments().ok()?;
        let first = arguments.args().first()?.ok()?;
        let AnyJsCallArgument::AnyJsExpression(first) = first else {
            return None;
        };
        let AnyJsExpression::JsObjectExpression(object) = first.omit_parentheses() else {
            return None;
        };

        let ordered: Vec<_> = object
            .members()
            .iter()
            .flatten()
            .filter_map(|member| {
                let rank = property_rank(member.name()?.text())?;
                Some((member, rank))
            })
            .collect();
        if ordered.is_sorted_by_key(|(_, rank)| *rank) {
            return None;
        }

        let mut sorted = ordered.clone();
        sorted.sort_by_key(|(_, rank)| *rank);

        let replacements = ordered
            .into_iter()
            .zip(sorted)
            .filter(|((current, _), (expected, _))| current != expected)
            .map(|((current, _), (expected, _))| (current, expected))
            .collect();

        Some(replacements)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (first, _) = state.first()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                first.range(),
                markup! {
                    "The properties of this infinite query are in the wrong order."
                },
            )
            .note(markup! {
                "TanStack Query infers types from the order of the properties. Write "<Emphasis>"queryFn"</Emphasis>", then "<Emphasis>"getPreviousPageParam"</Emphasis>", then "<Emphasis>"getNextPageParam"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        for (current, expected) in state.iter() {
            mutation.replace_node_discard_trivia(current.clone(), expected.clone());
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Reorder the properties." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the position that the property `name` must have relative to the other ordered
/// properties, or `None` if the position of the property is free.
fn property_rank(name: &str) -> Option<u8> {
    match name {
        "queryFn" => Some(0),
        "getPreviousPageParam" => Some(1),
        "getNextPageParam" => Some(2),
        _ => None,
    }
}

/// Returns whether `call` calls an infinite query function imported by name from a TanStack
/// Query package.
///
/// The function is identified by its exported name, so a renamed import such as
/// `import { useInfiniteQuery as useData }` is recognized too.
fn is_infinite_query_function(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let Some(callee) = call.callee().ok().and_then(|c| c.inner_expression()) else {
        return false;
    };
    let Some(reference) = callee.as_js_reference_identifier() else {
        return false;
    };
    let Some(binding) = model.binding(&reference) else {
        return false;
    };
    let Some(specifier) = JsIdentifierBinding::cast(binding.syntax())
        .and_then(|binding| binding.parent::<AnyJsNamedImportSpecifier>())
    else {
        return false;
    };
    if specifier.imports_only_types() {
        return false;
    }
    let Some(imported_name) = specifier.imported_name() else {
        return false;
    };
    if !matches!(
        imported_name.text_trimmed(),
        "infiniteQueryOptions" | "useInfiniteQuery" | "useSuspenseInfiniteQuery"
    ) {
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
