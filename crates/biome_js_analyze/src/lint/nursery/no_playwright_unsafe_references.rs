use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::{Applicability, Severity};
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsArrayElement, AnyJsArrowFunctionParameters, AnyJsBinding,
    AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsFormalParameter,
    AnyJsIdentifierReference, AnyJsMemberExpression, AnyJsParameter, JsArrayBindingPattern,
    JsArrayBindingPatternElementList, JsArrayElementList, JsArrayExpression, JsCallExpression,
    JsInitializerClause, JsParameterList, T, declaration_ext::is_in_ambient_context,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, TokenText};
use biome_rule_options::no_playwright_unsafe_references::NoPlaywrightUnsafeReferencesOptions;

use crate::JsRuleAction;
use crate::frameworks::playwright::get_page_or_frame_name;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow using variables from outside the function passed to `page.evaluate()` and `page.addInitScript()`.
    ///
    /// Playwright doesn't run the function passed to `page.evaluate()` or `page.addInitScript()`
    /// in your test. It converts the function to text, sends it to the browser page, and runs it there.
    /// Variables declared in your test file don't exist in the browser page,
    /// so using them inside the function throws an error when the test runs.
    ///
    /// To use a value from your test inside the function, pass it as the second argument
    /// of `page.evaluate()` or `page.addInitScript()`, and read it from the function's parameter.
    /// Playwright copies the value to the browser page for you.
    ///
    /// The rule only checks variables declared in the same file.
    /// It ignores global variables, such as `document` or `Promise`, and variables
    /// declared with TypeScript's `declare` keyword.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const x = 7;
    /// const y = 8;
    /// await page.evaluate(() => x * y);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const theme = "dark";
    /// await page.addInitScript(() => {
    ///     localStorage.setItem("theme", theme);
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const x = 7;
    /// const y = 8;
    /// await page.evaluate(([x, y]) => x * y, [x, y]);
    ///
    /// const theme = "dark";
    /// await page.addInitScript((theme) => {
    ///     localStorage.setItem("theme", theme);
    /// }, theme);
    ///
    /// await page.evaluate(() => document.title);
    /// ```
    ///
    pub NoPlaywrightUnsafeReferences {
        version: "next",
        name: "noPlaywrightUnsafeReferences",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-unsafe-references").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Playwright],
    }
}

impl Rule for NoPlaywrightUnsafeReferences {
    type Query = Semantic<JsCallExpression>;
    /// Every use, inside the callback, of a variable declared outside of it.
    type State = Box<[AnyJsIdentifierReference]>;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightUnsafeReferencesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let member = callee_member(call)?;
        let method = member.member_name()?;
        if method.text() != "evaluate" && method.text() != "addInitScript" {
            return None;
        }
        get_page_or_frame_name(&member.object().ok()?)?;

        let callback = callback(call)?;
        let callback_range = callback.range();
        let model = ctx.model();
        let references: Box<[_]> = callback
            .syntax()
            .descendants()
            .filter_map(AnyJsIdentifierReference::cast)
            // Type annotations are removed before the function is sent to the browser.
            .filter(|reference| !reference.is_only_type())
            // Unresolved references and ambient declarations (`declare const x`) describe
            // globals that have no value in the test, so the browser page may provide them.
            .filter(|reference| {
                model.binding(reference).is_some_and(|binding| {
                    !callback_range.contains_range(binding.range())
                        && !is_in_ambient_context(&binding.syntax())
                })
            })
            .collect();

        (!references.is_empty()).then_some(references)
    }

    fn diagnostic(ctx: &RuleContext<Self>, references: &Self::State) -> Option<RuleDiagnostic> {
        let method = callee_member(ctx.query())?.member_name()?;
        let method = method.text();
        let (first, rest) = references.split_first()?;
        let first_name = first.value_token().ok()?;
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first.range(),
            markup! {
                "The variable "<Emphasis>{first_name.text_trimmed()}</Emphasis>" is declared outside the function passed to "<Emphasis>{method}"()"</Emphasis>"."
            },
        );
        // Only the first use of each variable is shown, to keep the diagnostic short.
        for (index, reference) in rest.iter().enumerate() {
            let name = reference.value_token().ok()?;
            let is_repeated = references[..=index].iter().any(|previous| {
                previous
                    .value_token()
                    .is_ok_and(|previous| previous.text_trimmed() == name.text_trimmed())
            });
            if !is_repeated {
                diagnostic = diagnostic.detail(
                    reference.range(),
                    markup! {
                        "The variable "<Emphasis>{name.text_trimmed()}</Emphasis>" is also declared outside the function."
                    },
                );
            }
        }
        Some(
            diagnostic
                .note(markup! {
                    "Playwright runs this function inside the browser page, where variables from your test don't exist. Using them throws an error when the test runs."
                })
                .note(markup! {
                    "Pass the values as the second argument of "<Emphasis>{method}"()"</Emphasis>", and read them from the function's parameter."
                }),
        )
    }

    fn action(ctx: &RuleContext<Self>, references: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let arguments = call.arguments().ok()?.args();
        if arguments.len() > 2 {
            return None;
        }
        // Only a bare function is rewritten. `callback_parameter()` rejects anything else,
        // including a function wrapped in parentheses.
        let callback = arguments.first()?.ok()?.as_any_js_expression()?.clone();
        let value = match arguments.iter().nth(1).transpose().ok()? {
            None => None,
            Some(AnyJsCallArgument::AnyJsExpression(value)) => Some(value),
            Some(AnyJsCallArgument::JsSpread(_)) => return None,
        };

        let mut names: Vec<TokenText> = Vec::with_capacity(references.len());
        for reference in references {
            let name = reference.value_token().ok()?.token_text_trimmed();
            if !names.contains(&name) {
                names.push(name);
            }
        }

        let (new_parameter, new_value) = match (callback_parameter(&callback)?, value) {
            (CallbackParameter::None, None) => (
                names_pattern(&names),
                make::js_array_expression(
                    make::token(T!['[']),
                    array_element_list(Vec::new(), &names),
                    make::token(T![']']),
                ),
            ),
            (CallbackParameter::None, Some(AnyJsExpression::JsArrayExpression(array)))
                if array.elements().is_empty() =>
            {
                (
                    names_pattern(&names),
                    array.with_elements(array_element_list(Vec::new(), &names)),
                )
            }
            // The function ignores the current value, so there's no parameter to extend.
            (CallbackParameter::None, Some(_)) | (CallbackParameter::Single(..), None) => {
                return None;
            }
            (
                CallbackParameter::Single(
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern),
                    None,
                ),
                Some(AnyJsExpression::JsArrayExpression(array)),
            ) if can_append_elements(&pattern, &array) => {
                let pattern_elements = pattern
                    .elements()
                    .iter()
                    .collect::<Result<Vec<_>, _>>()
                    .ok()?;
                let array_elements = array
                    .elements()
                    .iter()
                    .collect::<Result<Vec<_>, _>>()
                    .ok()?;
                (
                    pattern.with_elements(binding_element_list(pattern_elements, &names)),
                    array.with_elements(array_element_list(array_elements, &names)),
                )
            }
            (CallbackParameter::Single(binding, initializer), Some(value)) => {
                // In `([a = x, x])`, the default value would read `x` before it's assigned.
                if let Some(initializer) = &initializer {
                    let initializer_range = initializer.range();
                    if references
                        .iter()
                        .any(|reference| initializer_range.contains_range(reference.range()))
                    {
                        return None;
                    }
                }
                wrap_in_arrays(binding, initializer, value, &names)?
            }
        };

        let new_callback = with_parameter(callback.clone(), new_parameter)?;
        let new_value = AnyJsExpression::JsArrayExpression(new_value);
        // With a single argument, any existing separator is a trailing comma,
        // which is dropped in favor of a separator followed by a space.
        let separators = if arguments.len() == 2 {
            arguments.separators().collect::<Result<Vec<_>, _>>().ok()?
        } else {
            vec![make::token_with_trailing_space(T![,])]
        };
        let new_arguments = make::js_call_argument_list(
            [
                AnyJsCallArgument::AnyJsExpression(new_callback),
                AnyJsCallArgument::AnyJsExpression(new_value),
            ],
            separators,
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(arguments, new_arguments);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::MaybeIncorrect,
            markup! { "Pass the variables as an argument." }.to_owned(),
            mutation,
        ))
    }
}

/// The single parameter of a callback, when its shape lets the fix extend it.
enum CallbackParameter {
    None,
    Single(AnyJsBindingPattern, Option<JsInitializerClause>),
}

fn callee_member(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())
}

/// Returns the first argument of `call` when it is an arrow function or a function expression.
fn callback(call: &JsCallExpression) -> Option<AnyJsExpression> {
    let callback = call
        .arguments()
        .ok()?
        .args()
        .first()?
        .ok()?
        .as_any_js_expression()?
        .clone()
        .omit_parentheses();
    matches!(
        callback,
        AnyJsExpression::JsArrowFunctionExpression(_) | AnyJsExpression::JsFunctionExpression(_)
    )
    .then_some(callback)
}

/// Returns the parameter of `callback`, or `None` when the fix can't safely rewrite the parameters.
///
/// The fix gives up on rest parameters, `this` parameters, parameters with a type annotation,
/// optional or decorated parameters, and functions with more than one parameter.
fn callback_parameter(callback: &AnyJsExpression) -> Option<CallbackParameter> {
    let parameters = match callback {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => match arrow.parameters().ok()? {
            AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                return Some(CallbackParameter::Single(
                    AnyJsBindingPattern::AnyJsBinding(binding),
                    None,
                ));
            }
            AnyJsArrowFunctionParameters::JsParameters(parameters) => parameters,
        },
        AnyJsExpression::JsFunctionExpression(function) => function.parameters().ok()?,
        _ => return None,
    };
    let items = parameters.items();
    match items.len() {
        0 => Some(CallbackParameter::None),
        1 => {
            let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                parameter,
            )) = items.first()?.ok()?
            else {
                return None;
            };
            if !parameter.decorators().is_empty()
                || parameter.question_mark_token().is_some()
                || parameter.type_annotation().is_some()
            {
                return None;
            }
            Some(CallbackParameter::Single(
                parameter.binding().ok()?,
                parameter.initializer(),
            ))
        }
        _ => None,
    }
}

/// Returns `true` when the elements of `pattern` match the elements of `array` one to one,
/// so new names can be appended to both.
fn can_append_elements(pattern: &JsArrayBindingPattern, array: &JsArrayExpression) -> bool {
    let pattern_elements = pattern.elements();
    let array_elements = array.elements();
    pattern_elements.len() == array_elements.len()
        && pattern_elements.iter().all(|element| {
            !matches!(
                element,
                Ok(AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(_))
            )
        })
        && array_elements
            .iter()
            .all(|element| !matches!(element, Ok(AnyJsArrayElement::JsSpread(_))))
}

/// Turns the parameter `binding = initializer` into `[binding = initializer, a, b]`
/// and the argument `value` into `[value, a, b]`, where `a` and `b` stand for `names`.
fn wrap_in_arrays(
    binding: AnyJsBindingPattern,
    initializer: Option<JsInitializerClause>,
    value: AnyJsExpression,
    names: &[TokenText],
) -> Option<(JsArrayBindingPattern, JsArrayExpression)> {
    let leading_trivia = binding.syntax().first_leading_trivia()?;
    let (binding, initializer, trailing_trivia) = match initializer {
        Some(initializer) => {
            let trailing_trivia = initializer.syntax().last_trailing_trivia()?;
            (
                binding.trim_leading_trivia()?,
                Some(initializer.trim_trailing_trivia()?),
                trailing_trivia,
            )
        }
        None => {
            let trailing_trivia = binding.syntax().last_trailing_trivia()?;
            (binding.trim_trivia()?, None, trailing_trivia)
        }
    };
    let mut element = make::js_array_binding_pattern_element(binding);
    if let Some(initializer) = initializer {
        element = element.with_init(initializer);
    }
    let element = AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element.build());
    let pattern = make::js_array_binding_pattern(
        make::token(T!['[']).with_leading_trivia_pieces(leading_trivia.pieces()),
        binding_element_list(vec![element], names),
        make::token(T![']']).with_trailing_trivia_pieces(trailing_trivia.pieces()),
    );

    let value_leading_trivia = value.syntax().first_leading_trivia()?;
    let value_trailing_trivia = value.syntax().last_trailing_trivia()?;
    let value = AnyJsArrayElement::AnyJsExpression(value.trim_trivia()?);
    let array = make::js_array_expression(
        make::token(T!['[']).with_leading_trivia_pieces(value_leading_trivia.pieces()),
        array_element_list(vec![value], names),
        make::token(T![']']).with_trailing_trivia_pieces(value_trailing_trivia.pieces()),
    );
    Some((pattern, array))
}

/// Replaces the parameters of `callback` with the single parameter `pattern`.
fn with_parameter(
    callback: AnyJsExpression,
    pattern: JsArrayBindingPattern,
) -> Option<AnyJsExpression> {
    match callback {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => {
            let parameters = match arrow.parameters().ok()? {
                AnyJsArrowFunctionParameters::JsParameters(parameters) => {
                    parameters.with_items(parameter_list(pattern))
                }
                // `x => {}` has no parentheses. They take over the whitespace and
                // comments that surrounded `x`, which `pattern` carries.
                AnyJsArrowFunctionParameters::AnyJsBinding(_) => {
                    let leading_trivia = pattern.syntax().first_leading_trivia()?;
                    let trailing_trivia = pattern.syntax().last_trailing_trivia()?;
                    make::js_parameters(
                        make::token(T!['(']).with_leading_trivia_pieces(leading_trivia.pieces()),
                        parameter_list(pattern.trim_trivia()?),
                        make::token(T![')']).with_trailing_trivia_pieces(trailing_trivia.pieces()),
                    )
                }
            };
            Some(AnyJsExpression::JsArrowFunctionExpression(
                arrow.with_parameters(AnyJsArrowFunctionParameters::JsParameters(parameters)),
            ))
        }
        AnyJsExpression::JsFunctionExpression(function) => {
            let parameters = function
                .parameters()
                .ok()?
                .with_items(parameter_list(pattern));
            Some(AnyJsExpression::JsFunctionExpression(
                function.with_parameters(parameters),
            ))
        }
        _ => None,
    }
}

fn parameter_list(pattern: JsArrayBindingPattern) -> JsParameterList {
    let parameter = AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
        make::js_formal_parameter(
            make::js_decorator_list([]),
            AnyJsBindingPattern::JsArrayBindingPattern(pattern),
        )
        .build(),
    ));
    make::js_parameter_list([parameter], [])
}

/// Builds an array pattern with one variable per name, such as `[a, b]`.
fn names_pattern(names: &[TokenText]) -> JsArrayBindingPattern {
    make::js_array_binding_pattern(
        make::token(T!['[']),
        binding_element_list(Vec::new(), names),
        make::token(T![']']),
    )
}

/// Builds the elements of an array pattern from `elements` followed by one variable per name.
fn binding_element_list(
    mut elements: Vec<AnyJsArrayBindingPatternElement>,
    names: &[TokenText],
) -> JsArrayBindingPatternElementList {
    elements.extend(names.iter().map(|name| {
        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
            make::js_array_binding_pattern_element(AnyJsBindingPattern::AnyJsBinding(
                AnyJsBinding::JsIdentifierBinding(make::js_identifier_binding(make::ident(
                    name.text(),
                ))),
            ))
            .build(),
        )
    }));
    let separators = (1..elements.len()).map(|_| make::token_with_trailing_space(T![,]));
    make::js_array_binding_pattern_element_list(elements, separators)
}

/// Builds the elements of an array from `elements` followed by one variable reference per name.
fn array_element_list(
    mut elements: Vec<AnyJsArrayElement>,
    names: &[TokenText],
) -> JsArrayElementList {
    elements.extend(names.iter().map(|name| {
        AnyJsArrayElement::AnyJsExpression(AnyJsExpression::JsIdentifierExpression(
            make::js_identifier_expression(make::js_reference_identifier(make::ident(name.text()))),
        ))
    }));
    let separators = (1..elements.len()).map(|_| make::token_with_trailing_space(T![,]));
    make::js_array_element_list(elements, separators)
}
