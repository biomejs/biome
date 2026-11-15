use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, binding_ext::AnyJsBindingDeclaration,
    JsCallExpression, JsImport, JsReferenceIdentifier, JsSyntaxToken,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_describe_function_title::UseDescribeFunctionTitleOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Require passing an imported function as a test suite title instead of a string with its name.
    ///
    /// Vitest and Jest accept a function as the title of a test suite (a `describe` or `suite` block),
    /// and use the name of the function as the title.
    /// When a test suite is about a single function, passing the function itself keeps the title correct
    /// even if the function is renamed later, because the import has to be renamed too.
    /// A string with the old name would silently stay out of date.
    ///
    /// This rule reports a test suite, including variants such as `describe.only(...)` and `describe.each(table)(...)`, when its title is:
    /// - a string that matches the name of an imported value, such as `describe("myFunction", ...)`;
    /// - the `name` property of an imported value, such as `describe(myFunction.name, ...)`.
    ///
    /// A `describe` or `suite` function that you define yourself or import from a module other than
    /// `vitest` or `@jest/globals`, such as `node:test`, is ignored, because it may only accept a string title.
    /// Mocha's global `describe` also only accepts a string title, and can't be told apart from the one of Jest or Vitest,
    /// so this rule shouldn't be enabled in projects that use Mocha.
    ///
    /// Type-only imports are ignored, because they don't exist when the tests run.
    /// Namespace imports (`import * as utils from "./utils"`) are also ignored,
    /// because they are objects without a name, not functions.
    ///
    /// The fix is unsafe because the title can change.
    /// The name of a function doesn't always match the name it was imported with,
    /// for example with `import { parse as parseJson } from "./parse"` or with a default import.
    /// Also, the rule doesn't check that the imported value is a function.
    /// Test runners use titles to name stored snapshots and to select which tests to run,
    /// so a changed title can leave a stored snapshot unused.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { myFunction } from "./myFunction";
    ///
    /// describe("myFunction", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { myFunction } from "./myFunction";
    ///
    /// describe(myFunction.name, () => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { myFunction } from "./myFunction";
    ///
    /// describe(myFunction, () => {});
    /// describe("other things", () => {});
    /// test("myFunction", () => {});
    /// ```
    ///
    pub UseDescribeFunctionTitle {
        version: "next",
        name: "useDescribeFunctionTitle",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintVitest("prefer-describe-function-title").inspired()],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseDescribeFunctionTitle {
    type Query = Semantic<JsCallExpression>;
    /// The name token of the imported value to pass as the title.
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = UseDescribeFunctionTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        if !is_test_runner_suite(model, &suite_function(call)?) {
            return None;
        }

        let arguments = call.arguments().ok()?.args();
        // A call without a callback, such as `describe("myFunction")`, is left alone.
        if arguments.len() < 2 {
            return None;
        }
        let title = arguments.first()?.ok()?;
        let binding = match title.as_any_js_expression()?.clone().omit_parentheses() {
            // describe("myFunction", ...)
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(string),
            ) => {
                let name = string.inner_string_text().ok()?;
                model
                    .scope(call.syntax())
                    .ancestors()
                    .find_map(|scope| scope.get_binding(name.text()))?
            }
            // describe(myFunction.name, ...)
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let property = member.member().ok()?;
                if property.as_js_name()?.value_token().ok()?.text_trimmed() != "name" {
                    return None;
                }
                let object = member.object().ok()?;
                let reference = object.omit_parentheses();
                model.binding(&reference.as_js_identifier_expression()?.name().ok()?)?
            }
            _ => return None,
        };

        imported_value_name(&binding, call)
    }

    fn diagnostic(ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        let title = ctx.query().arguments().ok()?.args().first()?.ok()?;
        let name = name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                title.range(),
                markup! {
                    "This test suite title repeats the name of the imported "<Emphasis>{name}</Emphasis>"."
                },
            )
            .note(markup! {
                "Passing "<Emphasis>{name}</Emphasis>" itself as the title keeps the title correct when the function is renamed."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, name: &Self::State) -> Option<JsRuleAction> {
        let title = ctx.query().arguments().ok()?.args().first()?.ok()?;
        let title = title.as_any_js_expression()?.clone();
        let name = name.text_trimmed();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            title,
            make::js_identifier_expression(make::js_reference_identifier(make::ident(name))).into(),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{name}</Emphasis>" as the title." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the `describe` or `suite` identifier of a call that defines a test
/// suite, such as `describe(...)`, `describe.only(...)`, `suite.skip(...)`, or
/// the curried forms `describe.each(table)(...)` and
/// `describe.skipIf(condition)(...)`.
///
/// Playwright's `test.describe(...)` is excluded because it only accepts a
/// string title.
fn suite_function(call: &JsCallExpression) -> Option<JsReferenceIdentifier> {
    match call.callee().ok()?.omit_parentheses() {
        AnyJsExpression::JsCallExpression(inner_call) => {
            suite_callee_root(inner_call.callee().ok()?, true)
        }
        AnyJsExpression::JsTemplateExpression(template) => suite_callee_root(template.tag()?, true),
        callee => suite_callee_root(callee, false),
    }
}

/// Returns the root identifier of `callee` if it is `describe`, `suite`, or
/// one of their focused or skipped aliases, followed by any number of
/// [SUITE_MODIFIERS].
///
/// When `is_curried` is `true`, the last member must instead be one that
/// returns the suite function, such as `each` in `describe.each(table)`.
fn suite_callee_root(callee: AnyJsExpression, is_curried: bool) -> Option<JsReferenceIdentifier> {
    let mut callee = callee.omit_parentheses();
    let mut is_last = true;
    loop {
        match callee {
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let name = member.member().ok()?.as_js_name()?.value_token().ok()?;
                let name = name.text_trimmed();
                let is_allowed = if is_last && is_curried {
                    matches!(name, "each" | "for" | "runIf" | "skipIf")
                } else {
                    SUITE_MODIFIERS.binary_search(&name).is_ok()
                };
                if !is_allowed {
                    return None;
                }
                is_last = false;
                callee = member.object().ok()?.omit_parentheses();
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                if is_last && is_curried {
                    return None;
                }
                let reference = identifier.name().ok()?;
                let is_suite = matches!(
                    reference.value_token().ok()?.text_trimmed(),
                    "describe" | "fdescribe" | "suite" | "xdescribe"
                );
                return is_suite.then_some(reference);
            }
            _ => return None,
        }
    }
}

/// Returns `true` if `reference` is a global, like the `describe` that Jest
/// and Vitest provide, or is imported from `vitest` or `@jest/globals`.
///
/// Other suite functions, such as the `describe` of `node:test` or a local
/// function, don't necessarily accept a function as the title.
fn is_test_runner_suite(model: &SemanticModel, reference: &JsReferenceIdentifier) -> bool {
    let Some(binding) = model.binding(reference) else {
        return true;
    };
    binding.is_imported()
        && binding
            .syntax_near(reference.syntax())
            .ancestors()
            .find_map(JsImport::cast)
            .and_then(|import| import.source_text().ok())
            .is_some_and(|source| matches!(source.text(), "vitest" | "@jest/globals"))
}

/// Returns the name token of `binding` if it was introduced by a default or
/// named import that can be used as a value.
fn imported_value_name(binding: &Binding, call: &JsCallExpression) -> Option<JsSyntaxToken> {
    let identifier = binding.tree_near(call.syntax());
    let is_value_import = matches!(
        identifier.declaration()?,
        AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
    ) && !identifier.is_type_only();
    if !is_value_import {
        return None;
    }
    identifier.name_token().ok()
}

/// The members that can follow `describe` or `suite` and still define a test
/// suite, such as `only` in `describe.only(...)`. Sorted for binary search.
const SUITE_MODIFIERS: &[&str] = &["concurrent", "only", "sequential", "shuffle", "skip", "todo"];

#[cfg(test)]
mod tests {
    use super::SUITE_MODIFIERS;

    #[test]
    fn suite_modifiers_are_sorted() {
        for pair in SUITE_MODIFIERS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "SUITE_MODIFIERS is not sorted: {:?} must come before {:?}",
                pair[1],
                pair[0]
            );
        }
    }
}
