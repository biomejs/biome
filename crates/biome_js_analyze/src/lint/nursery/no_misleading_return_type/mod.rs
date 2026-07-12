mod analysis;

use std::io;

use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::{
    fmt::{Display, Formatter},
    markup,
};
use biome_js_syntax::{
    AnyJsFunction, JsGetterClassMember, JsGetterObjectMember, JsMethodClassMember,
    JsMethodObjectMember,
};
use biome_rowan::{TextRange, declare_node_union};
use biome_rule_options::no_misleading_return_type::NoMisleadingReturnTypeOptions;

use crate::services::typed::Typed;

declare_lint_rule! {
    /// Detect return type annotations that are misleadingly wider than what
    /// the implementation actually returns.
    ///
    /// Reports when a function's explicit return type annotation is wider than
    /// what TypeScript would infer from the implementation, hiding precise types
    /// from callers.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid.ts
    /// function getStatus(b: boolean): string { if (b) return "loading"; return "idle"; }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid2.ts
    /// function getCode(ok: boolean): number { if (ok) return 200; return 404; }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid3.ts
    /// class Foo { getStatus(b: boolean): string { if (b) return "loading"; return "idle"; } }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid4.ts
    /// const obj = { getMode(b: boolean): string { if (b) return "dark"; return "light"; } };
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid5.ts
    /// function makeData(): object { return { retry: true }; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function getStatus() { return "loading"; }
    /// ```
    ///
    /// ```ts
    /// function run(): void { return; }
    /// ```
    ///
    /// ```ts
    /// class Foo { greet(): string { return "hello"; } }
    /// ```
    ///
    /// ## Known limitations
    ///
    /// - Suggested replacement types are only shown when their textual
    ///   representation is up to 80 characters long. Longer unions fall back to
    ///   a generic note without the specific suggestion.
    /// - When a return uses a type assertion such as `as T`, the rule does
    ///   not flag the return unless it can prove that `T` is narrower than
    ///   `object`. Trusted cases include `unknown`, `any`, `typeof` queries,
    ///   conditional types, generic type parameters, and types the rule
    ///   cannot resolve. Intersections (`A & B`) are trusted when every
    ///   member is or when any member is `any`; unions (`A | B`) when at
    ///   least one is.
    pub NoMisleadingReturnType {
        version: "2.4.11",
        name: "noMisleadingReturnType",
        language: "ts",
        recommended: false,
        domains: &[RuleDomain::Types],
        issue_number: Some("9810"),
    }
}

declare_node_union! {
    pub AnyFunctionLikeWithReturnType =
        AnyJsFunction
        | JsMethodClassMember
        | JsMethodObjectMember
        | JsGetterClassMember
        | JsGetterObjectMember
}

pub struct RuleState {
    pub(super) annotation_range: TextRange,
    pub(super) suggestion: Option<String>,
}

impl Display for RuleState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> io::Result<()> {
        if let Some(suggestion) = &self.suggestion {
            formatter.write_str(suggestion)
        } else {
            Ok(())
        }
    }
}

impl Rule for NoMisleadingReturnType {
    type Query = Typed<AnyFunctionLikeWithReturnType>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoMisleadingReturnTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        analysis::run(ctx)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            state.annotation_range,
            markup! {
                "The return type annotation is wider than what the function actually returns."
            },
        )
        .note(markup! {
            "A wider return type hides the precise types that callers could rely on."
        });

        let diag = if state.suggestion.is_some() {
            diag.note(markup! {
                "Consider using "{state}" as the return type."
            })
        } else {
            diag.note(markup! {
                "Narrow the return type to match what the function actually returns."
            })
        };

        Some(diag)
    }
}
