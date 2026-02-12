use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyTsType, AnyTsVariableAnnotation, JsVariableDeclarator};
use biome_rowan::AstNode;
use biome_rule_options::no_unsafe_assignment::NoUnsafeAssignmentOptions;

use crate::services::typed::Typed;

declare_lint_rule! {
    /// Disallow assigning a value typed as `any` to a variable.
    ///
    /// The `any` type in TypeScript is a dangerous "escape hatch" from the
    /// type system. Using `any` disables many type checking rules and is
    /// generally best used only as a last resort or when prototyping code.
    ///
    /// Despite your best intentions, the `any` type can sometimes leak into
    /// your codebase. Assigning an `any` typed value to a variable can be
    /// hard to pick up on, particularly if it leaks in from an external
    /// library.
    ///
    /// This rule disallows assigning `any` to a variable.
    ///
    /// Unlike the original typescript-eslint rule, this rule does not yet
    /// check array destructuring with `any[]`, or compare generic type
    /// arguments (e.g. assigning `Set<any>` to `Set<string>`).
    ///
    /// Assigning to a variable explicitly annotated as `any` or `unknown` is
    /// allowed, since the developer has intentionally opted into those types.
    ///
    /// :::caution
    /// This rule relies on Biome's type inference, which is still under active
    /// development. As a result, it may miss some cases.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=returns-any.ts
    /// declare function getPayload(): any;
    /// const payload = getPayload();
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=annotated-target.ts
    /// declare function getPayload(): any;
    /// const payload: string = getPayload();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid-examples.ts
    /// declare function getPayload(): any;
    /// declare function getString(): string;
    ///
    /// const a = getString();
    ///
    /// const b: unknown = getPayload();
    ///
    /// const c: any = getPayload();
    /// ```
    ///
    pub NoUnsafeAssignment {
        version: "next",
        name: "noUnsafeAssignment",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-unsafe-assignment").inspired()],
        domains: &[RuleDomain::Project],
    }
}

impl Rule for NoUnsafeAssignment {
    type Query = Typed<JsVariableDeclarator>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUnsafeAssignmentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declarator = ctx.query();

        // Only check declarators that have an initializer.
        let initializer = declarator.initializer()?;
        let expression = initializer.expression().ok()?;

        let ty = ctx.type_of_expression(&expression);

        // Only flag when the type system actually resolved the type to `any`.
        // `is_inferred()` returns false when type inference failed (Unknown),
        // which avoids false positives for unresolved types.
        if !ty.is_inferred() || !ty.is_any_keyword() {
            return None;
        }

        // Allow assignment if the variable is explicitly annotated as `any` or
        // `unknown`, since the developer has intentionally opted into those types.
        if has_any_or_unknown_annotation(declarator) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let declarator = ctx.query();
        let initializer = declarator.initializer()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                initializer.range(),
                markup! {
                    "The assigned expression has the type "<Emphasis>"any"</Emphasis>", which undermines type safety."
                },
            )
            .note(markup! {
                "This often happens when using untyped third-party libraries or "
                "APIs. Consider annotating the variable with a more specific type "
                "or using "<Emphasis>"unknown"</Emphasis>" instead."
            }),
        )
    }
}

/// Checks whether the given variable declarator has an explicit `any` or
/// `unknown` type annotation.
fn has_any_or_unknown_annotation(declarator: &JsVariableDeclarator) -> bool {
    declarator
        .variable_annotation()
        .and_then(|annotation| match annotation {
            AnyTsVariableAnnotation::TsTypeAnnotation(ts_annotation) => {
                let ty = ts_annotation.ty().ok()?;
                Some(matches!(
                    ty,
                    AnyTsType::TsAnyType(_) | AnyTsType::TsUnknownType(_)
                ))
            }
            AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(_) => Some(false),
        })
        .unwrap_or(false)
}
