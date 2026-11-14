use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::assign_ext::AnyJsMemberAssignment;
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsCallArgument, AnyJsExpression,
    AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsMemberExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsCallExpression, T,
};
use biome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_spy_on::UseSpyOnOptions;

declare_lint_rule! {
    /// Enforce using `jest.spyOn()` or `vi.spyOn()` instead of overwriting a property with a mock function.
    ///
    /// When a property is replaced with `jest.fn()` or `vi.fn()`, the original value is lost.
    /// It has to be saved and put back by hand after the test, otherwise the mock leaks into
    /// other tests. A mock created with `jest.spyOn()` or `vi.spyOn()` remembers the original
    /// value, so it can be restored with `mockRestore()`, `restoreAllMocks()`, or the
    /// `restoreMocks` setting of the test runner.
    ///
    /// A spy calls the original function until you give it another implementation, for example
    /// with `mockImplementation()` or `mockReturnValue()`.
    ///
    /// The fix is unsafe because `spyOn()` throws an error when the property doesn't exist or
    /// isn't a function, while the assignment always succeeds.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Date.now = jest.fn();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Date.now = vi.fn(() => 10);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window.fetch = jest.fn().mockResolvedValue(response);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// jest.spyOn(Date, "now");
    /// jest.spyOn(Date, "now").mockImplementation(() => 10);
    /// vi.spyOn(window, "fetch").mockResolvedValue(response);
    /// const mock = jest.fn();
    /// const mocks = { now: vi.fn() };
    /// ```
    ///
    pub UseSpyOn {
        version: "next",
        name: "useSpyOn",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-spy-on").same(),
            RuleSource::EslintVitest("prefer-spy-on").same(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseSpyOn {
    type Query = Ast<JsAssignmentExpression>;
    type State = UseSpyOnState;
    type Signals = Option<Self::State>;
    type Options = UseSpyOnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assignment = ctx.query();
        if assignment.operator_token().ok()?.kind() != T![=] {
            return None;
        }
        let AnyJsAssignmentPattern::AnyJsAssignment(target) = assignment.left().ok()? else {
            return None;
        };
        let target = AnyJsMemberAssignment::cast(target.omit_target_parentheses().into_syntax())?;
        // `spyOn()` can't access private class members.
        if let AnyJsMemberAssignment::JsStaticMemberAssignment(target) = &target
            && target.member().ok()?.as_js_name().is_none()
        {
            return None;
        }

        // Walks down the chain of calls and member accesses on the right side, such as
        // `jest.fn().mockReturnValue(1)`, until it reaches the call at its start.
        let mut expression = assignment.right().ok()?.omit_parentheses();
        loop {
            let call = match expression {
                AnyJsExpression::JsStaticMemberExpression(member) => {
                    expression = member.object().ok()?.omit_parentheses();
                    continue;
                }
                AnyJsExpression::JsComputedMemberExpression(member) => {
                    expression = member.object().ok()?.omit_parentheses();
                    continue;
                }
                AnyJsExpression::JsCallExpression(call) => call,
                _ => return None,
            };
            let callee =
                AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
            let object = callee.object().ok()?.omit_parentheses();
            let Some(identifier) = object.as_js_identifier_expression() else {
                expression = object;
                continue;
            };
            let framework = match identifier.name().ok()?.value_token().ok()?.text_trimmed() {
                "jest" => TestFramework::Jest,
                "vi" => TestFramework::Vitest,
                _ => return None,
            };
            if callee.member_name()?.text() != "fn" {
                return None;
            }
            return Some(UseSpyOnState {
                target,
                mock_call: call,
                framework,
            });
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let framework = state.framework;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This assignment replaces a property with a mock function."
                },
            )
            .note(markup! {
                "The original value of the property is lost, so it has to be put back by hand after the test."
            })
            .note(markup! {
                "Use "<Emphasis>{framework}".spyOn()"</Emphasis>" instead, so the original value can be restored with "<Emphasis>"mockRestore()"</Emphasis>" or "<Emphasis>"restoreAllMocks()"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let assignment = ctx.query();
        // Rebuilding the assignment discards the tokens between the target and the mock call,
        // along with any comments attached to them.
        if assignment.syntax().has_inner_comments() {
            return None;
        }
        let target = &state.target;
        // The comments before the assignment are attached to the object. They're stripped here
        // because `replace_node()` moves them in front of the whole replacement.
        let object = target.object().ok()?.trim_comments_and_trivia()?;
        if object.as_js_super_expression().is_some() {
            return None;
        }
        let key = match target {
            AnyJsMemberAssignment::JsStaticMemberAssignment(target) => {
                let name = target.member().ok()?.as_js_name()?.value_token().ok()?;
                let name = name.text_trimmed();
                AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
                    make::js_string_literal_expression(if ctx.preferred_quote().is_double() {
                        make::js_string_literal(name)
                    } else {
                        make::js_string_literal_single_quotes(name)
                    }),
                ))
            }
            AnyJsMemberAssignment::JsComputedMemberAssignment(target) => {
                let key = target.member().ok()?.trim_trivia()?;
                // A comma expression would otherwise be read as several arguments.
                if key.as_js_sequence_expression().is_some() {
                    make::parenthesized(key).into()
                } else {
                    key
                }
            }
        };

        let framework = AnyJsExpression::from(make::js_identifier_expression(
            make::js_reference_identifier(make::ident(state.framework.namespace())),
        ));
        let spy_on = make::js_call_expression(
            make::js_static_member_expression(
                framework,
                make::token(T![.]),
                make::js_name(make::ident("spyOn")).into(),
            )
            .into(),
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list(
                    [
                        AnyJsCallArgument::AnyJsExpression(object),
                        AnyJsCallArgument::AnyJsExpression(key),
                    ],
                    [make::token_with_trailing_space(T![,])],
                ),
                make::token(T![')']),
            ),
        )
        .build();

        // `jest.fn(implementation)` becomes `.mockImplementation(implementation)`, unless the
        // mock call is already followed by `.mockImplementation()`.
        let has_mock_implementation = state
            .mock_call
            .syntax()
            .parent()
            .and_then(AnyJsMemberExpression::cast)
            .and_then(|member| member.member_name())
            .is_some_and(|name| name.text() == "mockImplementation");
        let replacement = if has_mock_implementation {
            spy_on
        } else {
            let implementation = match state.mock_call.arguments().ok()?.args().first() {
                Some(argument) => Some(argument.ok()?),
                // Vitest calls the original function when `mockImplementation()` gets no
                // argument, while `vi.fn()` without an argument returns `undefined`.
                None if matches!(state.framework, TestFramework::Vitest) => {
                    Some(AnyJsCallArgument::AnyJsExpression(return_undefined().into()))
                }
                None => None,
            };
            make::js_call_expression(
                make::js_static_member_expression(
                    spy_on.into(),
                    make::token(T![.]),
                    make::js_name(make::ident("mockImplementation")).into(),
                )
                .into(),
                make::js_call_arguments(
                    make::token(T!['(']),
                    make::js_call_argument_list(implementation, []),
                    make::token(T![')']),
                ),
            )
            .build()
        };

        let right = assignment.right().ok()?;
        let replacement = if right.syntax() == state.mock_call.syntax() {
            AnyJsExpression::from(replacement)
        } else {
            right.replace_node(
                AnyJsExpression::from(state.mock_call.clone()),
                AnyJsExpression::from(replacement),
            )?
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node(AnyJsExpression::from(assignment.clone()), replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{state.framework}".spyOn()"</Emphasis>" instead." }
                .to_owned(),
            mutation,
        ))
    }
}

pub struct UseSpyOnState {
    /// The member that is assigned, without the parentheses around it.
    target: AnyJsMemberAssignment,
    /// The `jest.fn()` or `vi.fn()` call at the start of the assigned value.
    mock_call: JsCallExpression,
    /// The test framework whose mock function is assigned.
    framework: TestFramework,
}

#[derive(Clone, Copy)]
pub enum TestFramework {
    Jest,
    Vitest,
}

impl TestFramework {
    /// Returns the name of the global object that holds the framework's mock functions.
    const fn namespace(self) -> &'static str {
        match self {
            Self::Jest => "jest",
            Self::Vitest => "vi",
        }
    }
}

impl biome_console::fmt::Display for TestFramework {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(self.namespace())
    }
}

/// Builds the arrow function `() => undefined`.
fn return_undefined() -> JsArrowFunctionExpression {
    make::js_arrow_function_expression(
        make::js_parameters(
            make::token(T!['(']),
            make::js_parameter_list([], []),
            make::token(T![')']),
        )
        .into(),
        make::token_decorated_with_space(T![=>]),
        AnyJsFunctionBody::AnyJsExpression(
            make::js_identifier_expression(make::js_reference_identifier(make::ident(
                "undefined",
            )))
            .into(),
        ),
    )
    .build()
}
