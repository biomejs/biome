use biome_analyze::{
    Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsImportClause, AnyJsMemberExpression, JsCallExpression,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::AstNode;
use biome_rule_options::no_array_for_each::NoArrayForEachOptions;

use crate::{
    services::semantic::Semantic, utils::indexed_collection::is_known_non_indexed_collection,
};

declare_lint_rule! {
    /// Disallow the `forEach()` method in favor of `for...of` loops.
    ///
    /// A `for...of` loop runs the same code for each item of an array, and it has several
    /// advantages over passing a function to `forEach()`:
    /// - `break` stops the loop early, and `continue` skips to the next item.
    /// - `return` and `await` apply to the surrounding function, so you can leave that function
    ///   or wait for a promise from inside the loop.
    /// - In TypeScript, a type checked before the loop is still known inside the loop,
    ///   because the loop body is not a separate function.
    ///
    /// Use `array.entries()` when you also need the index of each item:
    /// `for (const [index, item] of array.entries())`.
    ///
    /// This rule doesn't report calls when the value is known not to be an array,
    /// such as a `Map` or a `Set`, either because it is created with `new Map()`
    /// or because its type annotation says so.
    /// It also ignores `forEach()` from the following libraries,
    /// because they take the list as an argument instead of looping over the value they're called on:
    /// `React.Children`, `Children`, `R` (Ramda), `pIteration`, `Effect`,
    /// and namespace imports of `strict-callbag-basics`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// array.forEach((element) => {
    ///     bar(element);
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array?.forEach((element) => {
    ///     bar(element);
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.forEach((element, index) => {
    ///     bar(element, index);
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// for (const element of array) {
    ///     bar(element);
    /// }
    ///
    /// for (const [index, element] of array.entries()) {
    ///     bar(element, index);
    /// }
    ///
    /// const map = new Map();
    /// map.forEach((value, key) => bar(key, value));
    ///
    /// function log(set: Set<string>) {
    ///     set.forEach((value) => console.log(value));
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - [`noForEach`](https://biomejs.dev/linter/rules/no-for-each/) also reports `forEach()` calls,
    ///   but allows callbacks that use the index of each item.
    pub NoArrayForEach {
        version: "2.6.5",
        name: "noArrayForEach",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-for-each").inspired()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoArrayForEach {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoArrayForEachOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let callee = for_each_callee(ctx.query())?;
        let receiver = callee.object().ok()?;
        let model = ctx.model();
        if is_library_receiver(&receiver)
            || is_strict_callbag_basics_namespace(&receiver, model)
            || is_known_non_indexed_collection(&receiver, model)
        {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match for_each_callee(ctx.query())? {
            AnyJsMemberExpression::JsStaticMemberExpression(member) => member.member().ok()?.range(),
            AnyJsMemberExpression::JsComputedMemberExpression(member) => {
                member.member().ok()?.range()
            }
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"forEach()"</Emphasis>" call can be written as a "<Emphasis>"for...of"</Emphasis>" loop."
                },
            )
            .note(markup! {
                "A "<Emphasis>"for...of"</Emphasis>" loop can use "<Emphasis>"break"</Emphasis>" and "<Emphasis>"continue"</Emphasis>", and "<Emphasis>"return"</Emphasis>" and "<Emphasis>"await"</Emphasis>" inside it apply to the surrounding function."
            })
            .note(markup! {
                "Replace this call with a "<Emphasis>"for...of"</Emphasis>" loop. Loop over "<Emphasis>"entries()"</Emphasis>" if you also need the index."
            }),
        )
    }
}

const STRICT_CALLBAG_BASICS: &str = "strict-callbag-basics";

/// Returns the callee of `call` if it reads a property named `forEach`, as in `value.forEach()`,
/// `value?.forEach()`, or `value["forEach"]()`.
fn for_each_callee(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    let callee =
        AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
    (callee.member_name()?.text() == "forEach").then_some(callee)
}

/// Returns `true` if `receiver` is an object from a library whose `forEach()` takes the list to
/// loop over as an argument: `React.Children`, `Children`, `R` (Ramda), `pIteration`, or `Effect`.
fn is_library_receiver(receiver: &AnyJsExpression) -> bool {
    match receiver {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .and_then(|name| name.value_token())
            .is_ok_and(|name| {
                matches!(
                    name.text_trimmed(),
                    "Children" | "Effect" | "R" | "pIteration"
                )
            }),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            !member.is_optional()
                && member
                    .member()
                    .ok()
                    .and_then(|name| name.as_js_name()?.value_token().ok())
                    .is_some_and(|name| name.text_trimmed() == "Children")
                && member
                    .object()
                    .ok()
                    .and_then(|object| object.as_js_reference_identifier())
                    .and_then(|object| object.value_token().ok())
                    .is_some_and(|object| object.text_trimmed() == "React")
        }
        _ => false,
    }
}

/// Returns `true` if `receiver` is a variable created by `import * as name from "strict-callbag-basics"`.
fn is_strict_callbag_basics_namespace(receiver: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(AnyJsBindingDeclaration::JsNamespaceImportSpecifier(specifier)) = receiver
        .as_js_reference_identifier()
        .and_then(|reference| model.binding(&reference))
        .and_then(|binding| binding.tree().declaration())
    else {
        return false;
    };
    specifier
        .syntax()
        .parent()
        .and_then(AnyJsImportClause::cast)
        .and_then(|clause| clause.source().ok()?.inner_string_text().ok())
        .is_some_and(|source| source.text() == STRICT_CALLBAG_BASICS)
}
