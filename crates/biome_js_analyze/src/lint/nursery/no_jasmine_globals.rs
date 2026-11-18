use crate::{JsRuleAction, services::semantic::Semantic};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsAssignmentExpression,
    JsCallExpression, JsExpressionStatement, JsImport, JsReferenceIdentifier, T,
    assign_ext::AnyJsMemberAssignment, global_identifier, static_value::StaticValue,
};
use biome_rowan::{AstNode, BatchMutationExt, declare_node_union};
use biome_rule_options::no_jasmine_globals::NoJasmineGlobalsOptions;

declare_lint_rule! {
    /// Disallow Jasmine globals in Jest tests.
    ///
    /// Jest can run tests with a runner called `jest-jasmine2`, which is built on the Jasmine
    /// test framework. That runner makes the `jasmine` object and the functions `spyOn()`,
    /// `spyOnProperty()`, `fail()`, and `pending()` available in every test file.
    ///
    /// Jest doesn't document these globals, and `jest-circus`, the default runner since
    /// Jest 27, doesn't provide them. Tests that use them fail with the default runner.
    ///
    /// This rule reports:
    ///
    /// - calls to `spyOn()`, `spyOnProperty()`, `fail()`, and `pending()`;
    /// - every access to a property of the `jasmine` object, such as `jasmine.createSpy()`
    ///   or `jasmine["createSpy"]()`.
    ///
    /// The rule ignores a name that the file declares or imports itself, such as a parameter
    /// called `fail`.
    ///
    /// Jest has its own replacement for most Jasmine globals:
    ///
    /// | Jasmine global | Replacement in Jest |
    /// | --- | --- |
    /// | `spyOn()` and `spyOnProperty()` | `jest.spyOn()` |
    /// | `fail()` | Throw an error |
    /// | `pending()` | `test.skip()` |
    /// | `jasmine.any()`, `jasmine.anything()`, `jasmine.arrayContaining()`, `jasmine.objectContaining()`, and `jasmine.stringMatching()` | The function with the same name on `expect`, such as `expect.any()` |
    /// | `jasmine.addMatchers()` | `expect.extend()` |
    /// | `jasmine.createSpy()` | `jest.fn()` |
    /// | `jasmine.DEFAULT_TIMEOUT_INTERVAL = value` | `jest.setTimeout(value)` |
    ///
    /// Biome only offers a fix for the `jasmine` functions that have the same name on
    /// `expect`, and for assignments to `jasmine.DEFAULT_TIMEOUT_INTERVAL`. The fix is unsafe
    /// because Biome can't check that the file runs in Jest, where `expect` and `jest` exist.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test("saves the user", () => {
    ///     spyOn(database, "save");
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test("rejects invalid input", () => {
    ///     fail();
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test("supports dark mode", () => {
    ///     pending();
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test("returns a number", () => {
    ///     expect(getTotal()).toEqual(jasmine.any(Number));
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const onSave = jasmine.createSpy();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// jest.setTimeout(5000);
    ///
    /// test("saves the user", () => {
    ///     jest.spyOn(database, "save");
    /// });
    ///
    /// test("rejects invalid input", () => {
    ///     throw new Error("The input was accepted");
    /// });
    ///
    /// test.skip("supports dark mode", () => {});
    ///
    /// test("returns a number", () => {
    ///     expect(getTotal()).toEqual(expect.any(Number));
    /// });
    ///
    /// const onSave = jest.fn();
    ///
    /// function onError(fail) {
    ///     fail();
    /// }
    /// ```
    ///
    pub NoJasmineGlobals {
        version: "next",
        name: "noJasmineGlobals",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintJest("no-jasmine-globals").same()],
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyNoJasmineGlobalsQuery = JsCallExpression | AnyJsMemberExpression | AnyJsMemberAssignment
}

impl Rule for NoJasmineGlobals {
    type Query = Semantic<AnyNoJasmineGlobalsQuery>;
    type State = JasmineGlobal;
    type Signals = Option<Self::State>;
    type Options = NoJasmineGlobalsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let (reference, global) = match ctx.query() {
            AnyNoJasmineGlobalsQuery::JsCallExpression(call) => {
                let callee = call.callee().ok()?.omit_type_wrappers()?;
                let (reference, name) =
                    global_identifier(&callee.as_any_global_identifier_expression()?)?;
                (reference, JasmineGlobal::from_function_name(name.text())?)
            }
            AnyNoJasmineGlobalsQuery::AnyJsMemberExpression(member) => (
                jasmine_reference(member.object().ok()?)?,
                JasmineGlobal::from_read_property(member.member_name()),
            ),
            AnyNoJasmineGlobalsQuery::AnyJsMemberAssignment(member) => (
                jasmine_reference(member.object().ok()?)?,
                JasmineGlobal::from_assigned_property(member.member_name()),
            ),
        };

        ctx.model().binding(&reference).is_none().then_some(global)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match ctx.query() {
            AnyNoJasmineGlobalsQuery::JsCallExpression(call) => call.callee().ok()?.range(),
            member => member.range(),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Unexpected use of a Jasmine global."
                },
            )
            .note(markup! {
                "Jest only provides Jasmine globals when it runs tests with the "<Emphasis>"jest-jasmine2"</Emphasis>" runner. Its default runner doesn't provide them, so this code fails there."
            })
            .note(markup! {
                {state}
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let JasmineGlobal::Replaceable(function) = state else {
            return None;
        };
        let model = ctx.model();
        let mut mutation = ctx.root().begin();

        match ctx.query() {
            AnyNoJasmineGlobalsQuery::AnyJsMemberExpression(member)
                if function.is_expect_matcher() =>
            {
                let object = member.object().ok()?;
                // Replacing the object discards the tokens inside it, such as the
                // parentheses in `(jasmine).any()`, along with any comments attached to them.
                if object.syntax().has_inner_comments() || is_shadowed(model, ctx.query(), "expect")
                {
                    return None;
                }
                mutation.replace_node(object, identifier_expression("expect"));
            }
            AnyNoJasmineGlobalsQuery::AnyJsMemberAssignment(member)
                if matches!(function, JestFunction::SetTimeout) =>
            {
                let assignment = member.parent::<JsAssignmentExpression>()?;
                // `jest.setTimeout()` returns the `jest` object and not the timeout, so the
                // assignment can only be replaced where its value is unused.
                assignment.parent::<JsExpressionStatement>()?;
                // Rebuilding the statement discards the tokens around `=`, along with any
                // comments attached to them.
                if assignment.operator_token().ok()?.kind() != T![=]
                    || assignment.syntax().has_inner_comments()
                    || is_shadowed(model, ctx.query(), "jest")
                {
                    return None;
                }

                // `replace_node` moves the trivia around the assignment to the new call.
                let timeout = assignment
                    .right()
                    .ok()?
                    .with_leading_trivia_pieces([])?
                    .with_trailing_trivia_pieces([])?;
                let callee = make::js_static_member_expression(
                    identifier_expression("jest"),
                    make::token(T![.]),
                    make::js_name(make::ident("setTimeout")).into(),
                );
                let arguments = make::js_call_arguments(
                    make::token(T!['(']),
                    make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(timeout)], []),
                    make::token(T![')']),
                );
                mutation.replace_node(
                    AnyJsExpression::from(assignment),
                    make::js_call_expression(callee.into(), arguments)
                        .build()
                        .into(),
                );
            }
            _ => return None,
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{function}</Emphasis>" instead." },
            mutation,
        ))
    }
}

/// A use of a Jasmine global, grouped by what replaces it in Jest.
#[derive(Clone, Copy)]
pub enum JasmineGlobal {
    /// A call to `fail()`, which Jest replaces with a thrown error.
    Fail,
    /// A global that Jest replaces with one of its own functions.
    Replaceable(JestFunction),
    /// A property of the `jasmine` object that has no direct replacement in Jest.
    Other,
}

impl JasmineGlobal {
    /// Returns the Jasmine global that a call to the global function `name` uses, or `None`
    /// if Jasmine has no global function with that name.
    fn from_function_name(name: &str) -> Option<Self> {
        match name {
            "fail" => Some(Self::Fail),
            "pending" => Some(Self::Replaceable(JestFunction::TestSkip)),
            "spyOn" | "spyOnProperty" => Some(Self::Replaceable(JestFunction::SpyOn)),
            _ => None,
        }
    }

    /// Returns the Jasmine global that reading the property `name` of the `jasmine` object
    /// uses.
    ///
    /// `name` is `None` when the property is only known when the code runs, as in
    /// `jasmine[key]`.
    fn from_read_property(name: Option<StaticValue>) -> Self {
        name.and_then(|name| {
            JASMINE_FUNCTIONS
                .binary_search_by(|(function, _)| (*function).cmp(name.text()))
                .ok()
        })
        .map_or(Self::Other, |index| {
            Self::Replaceable(JASMINE_FUNCTIONS[index].1)
        })
    }

    /// Returns the Jasmine global that assigning to the property `name` of the `jasmine`
    /// object uses.
    ///
    /// `name` is `None` when the property is only known when the code runs, as in
    /// `jasmine[key] = value`.
    fn from_assigned_property(name: Option<StaticValue>) -> Self {
        if name.is_some_and(|name| name.text() == "DEFAULT_TIMEOUT_INTERVAL") {
            Self::Replaceable(JestFunction::SetTimeout)
        } else {
            Self::Other
        }
    }
}

/// Writes the advice on what to use in place of the Jasmine global.
impl biome_console::fmt::Display for JasmineGlobal {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Fail => fmt.write_markup(markup! {
                "Throw an error to fail the test instead."
            }),
            Self::Replaceable(function) => fmt.write_markup(markup! {
                "Use "<Emphasis>{function}</Emphasis>" instead."
            }),
            Self::Other => fmt.write_markup(markup! {
                "Use Jest's documented API instead."
            }),
        }
    }
}

/// A Jest function that replaces a Jasmine global.
#[derive(Clone, Copy)]
pub enum JestFunction {
    ExpectAny,
    ExpectAnything,
    ExpectArrayContaining,
    ExpectExtend,
    ExpectObjectContaining,
    ExpectStringMatching,
    Fn,
    SetTimeout,
    SpyOn,
    TestSkip,
}

impl JestFunction {
    /// Whether the `jasmine` object has a function with the same name and behavior, so that
    /// `jasmine.any()` can become `expect.any()` by replacing only the object.
    const fn is_expect_matcher(self) -> bool {
        matches!(
            self,
            Self::ExpectAny
                | Self::ExpectAnything
                | Self::ExpectArrayContaining
                | Self::ExpectObjectContaining
                | Self::ExpectStringMatching
        )
    }
}

impl biome_console::fmt::Display for JestFunction {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::ExpectAny => "expect.any()",
            Self::ExpectAnything => "expect.anything()",
            Self::ExpectArrayContaining => "expect.arrayContaining()",
            Self::ExpectExtend => "expect.extend()",
            Self::ExpectObjectContaining => "expect.objectContaining()",
            Self::ExpectStringMatching => "expect.stringMatching()",
            Self::Fn => "jest.fn()",
            Self::SetTimeout => "jest.setTimeout()",
            Self::SpyOn => "jest.spyOn()",
            Self::TestSkip => "test.skip()",
        })
    }
}

/// The functions of the `jasmine` object that a Jest function replaces.
///
/// Sorted by name, because [`JasmineGlobal::from_read_property`] uses a binary search.
const JASMINE_FUNCTIONS: &[(&str, JestFunction)] = &[
    ("addMatchers", JestFunction::ExpectExtend),
    ("any", JestFunction::ExpectAny),
    ("anything", JestFunction::ExpectAnything),
    ("arrayContaining", JestFunction::ExpectArrayContaining),
    ("createSpy", JestFunction::Fn),
    ("objectContaining", JestFunction::ExpectObjectContaining),
    ("stringMatching", JestFunction::ExpectStringMatching),
];

/// Returns the identifier to look up in the semantic model if `object` names the `jasmine`
/// global, either directly or through the global object, as in `globalThis.jasmine`.
fn jasmine_reference(object: AnyJsExpression) -> Option<JsReferenceIdentifier> {
    let object = object.omit_type_wrappers()?;
    let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;
    (name.text() == "jasmine").then_some(reference)
}

/// Returns whether `name` refers to something other than Jest's global at `node`.
///
/// Any declaration in the file hides the global, except for an import from `@jest/globals`,
/// which exports Jest's globals.
fn is_shadowed(model: &SemanticModel, node: &AnyNoJasmineGlobalsQuery, name: &str) -> bool {
    model
        .scope(node.syntax())
        .ancestors()
        .find_map(|scope| scope.get_binding(name))
        .is_some_and(|binding| {
            !binding
                .syntax()
                .ancestors()
                .find_map(JsImport::cast)
                .and_then(|import| import.source_text().ok())
                .is_some_and(|source| source.text() == "@jest/globals")
        })
}

fn identifier_expression(name: &str) -> AnyJsExpression {
    make::js_identifier_expression(make::js_reference_identifier(make::ident(name))).into()
}

#[cfg(test)]
mod tests {
    use super::JASMINE_FUNCTIONS;

    #[test]
    fn jasmine_functions_are_sorted() {
        for pair in JASMINE_FUNCTIONS.windows(2) {
            assert!(
                pair[0].0.cmp(pair[1].0).is_lt(),
                "{} must come before {}",
                pair[0].0,
                pair[1].0
            );
        }
    }
}
