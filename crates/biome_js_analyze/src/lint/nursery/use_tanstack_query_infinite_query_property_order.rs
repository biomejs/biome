use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsNamedImportSpecifier, AnyJsObjectMember, JsCallExpression,
    JsIdentifierBinding, JsImport, JsObjectExpression,
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
        version: "2.6.5",
        name: "useTanstackQueryInfiniteQueryPropertyOrder",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("infinite-query-property-order").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::TanstackQuery],
    }
}

/// The properties whose relative order matters, in the order they must be written.
///
/// The derived ordering follows the declaration order of the variants.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum OrderedProperty {
    QueryFn,
    GetPreviousPageParam,
    GetNextPageParam,
}

impl OrderedProperty {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "queryFn" => Some(Self::QueryFn),
            "getPreviousPageParam" => Some(Self::GetPreviousPageParam),
            "getNextPageParam" => Some(Self::GetNextPageParam),
            _ => None,
        }
    }
}

/// Two ordered properties that are written in the wrong order.
///
/// `first` is written before `second` but must come after it.
type Inversion = (AnyJsObjectMember, AnyJsObjectMember);

impl Rule for UseTanstackQueryInfiniteQueryPropertyOrder {
    type Query = Semantic<JsCallExpression>;
    type State = Inversion;
    type Signals = Option<Self::State>;
    type Options = UseTanstackQueryInfiniteQueryPropertyOrderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if !is_infinite_query_function(call, ctx.model()) {
            return None;
        }

        let object = options_object(call)?;

        let ordered: Vec<_> = object
            .members()
            .iter()
            .flatten()
            .filter_map(|member| {
                let property = OrderedProperty::from_name(member.name()?.text())?;
                Some((member, property))
            })
            .collect();

        ordered
            .windows(2)
            .find(|pair| pair[0].1 > pair[1].1)
            .map(|pair| (pair[0].0.clone(), pair[1].0.clone()))
    }

    fn diagnostic(_: &RuleContext<Self>, (first, _): &Self::State) -> Option<RuleDiagnostic> {
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

    fn action(ctx: &RuleContext<Self>, (first, second): &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(
            first.clone(),
            second.clone().trim_comments_and_trivia()?,
        );
        mutation.replace_node_transfer_trivia(
            second.clone(),
            first.clone().trim_comments_and_trivia()?,
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Reorder the properties." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the options object passed as the first argument of `call`.
fn options_object(call: &JsCallExpression) -> Option<JsObjectExpression> {
    let first = call.arguments().ok()?.args().first()?.ok()?;
    let AnyJsCallArgument::AnyJsExpression(first) = first else {
        return None;
    };
    first.omit_parentheses().as_js_object_expression().cloned()
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
