use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsExpression, AnyJsFormalParameter, AnyJsMemberExpression, AnyJsParameter, AnyTsName,
    AnyTsTupleTypeElement, AnyTsType, JsArrowFunctionExpression, JsCallExpression,
    JsFormalParameter, JsIdentifierBinding, JsLanguage, JsParameters, JsRestParameter,
    JsSyntaxToken, T, TsTypeAnnotation,
};
use biome_languages::JsFileSource;
use biome_module_graph::type_inference::TypeInferenceClassification;
use biome_rowan::{AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, declare_node_union};
use biome_rule_options::use_unknown_in_catch_callback_variable::UseUnknownInCatchCallbackVariableOptions;
use smallvec::{SmallVec, smallvec};

use crate::{JsRuleAction, services::typed::Typed};

declare_lint_rule! {
    /// Require the error variable of a Promise rejection callback to be typed as `unknown`.
    ///
    /// TypeScript types the error variable of `.catch()` and `.then()` rejection callbacks as `any`,
    /// but a Promise can be rejected with any value.
    /// Typing it as `unknown` forces a check, such as `error instanceof Error`, before the variable is used.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-no-annotation.ts
    /// Promise.reject(new Error("Request failed")).catch((error) => {
    ///     console.log(error);
    /// });
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-any.ts
    /// Promise.reject(new Error("Request failed")).catch((error: any) => {
    ///     console.log(error);
    /// });
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-error.ts
    /// Promise.reject(new Error("Request failed")).catch((error: Error) => {
    ///     console.log(error.message);
    /// });
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-then.ts
    /// Promise.reject(new Error("Request failed")).then(
    ///     (result) => console.log(result),
    ///     (error) => console.log(error),
    /// );
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// Promise.reject(new Error("Request failed")).catch((error: unknown) => {
    ///     if (error instanceof Error) {
    ///         console.log(error.message);
    ///     }
    /// });
    ///
    /// Promise.reject(new Error("Request failed")).then(
    ///     (result) => console.log(result),
    ///     (error: unknown) => console.log(error),
    /// );
    ///
    /// Promise.reject(new Error("Request failed")).catch(() => {
    ///     console.log("Request failed");
    /// });
    /// ```
    ///
    pub UseUnknownInCatchCallbackVariable {
        version: "next",
        name: "useUnknownInCatchCallbackVariable",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("use-unknown-in-catch-callback-variable").same()],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Types],
    }
}

impl Rule for UseUnknownInCatchCallbackVariable {
    type Query = Typed<JsCallExpression>;
    type State = InvalidErrorVariable;
    type Signals = Box<[Self::State]>;
    type Options = UseUnknownInCatchCallbackVariableOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<JsFileSource>().language().is_typescript() {
            return Box::default();
        }
        find_invalid_error_variables(ctx).unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.parameter.range(),
            markup! {
                "The error variable of this "<Emphasis>{state.method}</Emphasis>" callback isn't typed as "<Emphasis>"unknown"</Emphasis>"."
            },
        )
        .note(markup! {
            "A Promise can be rejected with any value, so the type of this variable can't be known in advance."
        });
        Some(if state.parameter.is_destructuring() {
            diagnostic.note(markup! {
                "Destructuring the error fails at runtime when it is "<Emphasis>"null"</Emphasis>", "<Emphasis>"undefined"</Emphasis>", or doesn't have the expected shape. Type the variable as "<Emphasis>"unknown"</Emphasis>" and check its value before using it."
            })
        } else {
            diagnostic.note(markup! {
                "Type the variable as "<Emphasis>"unknown"</Emphasis>" and check its value, for example with "<Emphasis>"instanceof Error"</Emphasis>", before using it."
            })
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        // `unknown` has no properties to destructure.
        if state.parameter.is_destructuring() {
            return None;
        }
        let mut mutation = ctx.root().begin();
        match &state.parameter {
            AnyErrorParameter::JsIdentifierBinding(binding) => {
                let arrow = binding.parent::<JsArrowFunctionExpression>()?;
                let first_token = binding.syntax().first_token()?;
                let last_token = binding.syntax().last_token()?;
                let parameter = make::js_formal_parameter(
                    make::js_decorator_list([]),
                    AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                        binding
                            .clone()
                            .with_leading_trivia_pieces([])?
                            .with_trailing_trivia_pieces([])?,
                    )),
                )
                .with_type_annotation(make::ts_type_annotation(
                    make::token_with_trailing_space(T![:]),
                    unknown_type(),
                ))
                .build();
                let parameters = make::js_parameters(
                    make::token(T!['('])
                        .with_leading_trivia_pieces(first_token.leading_trivia().pieces()),
                    make::js_parameter_list(
                        [AnyJsParameter::AnyJsFormalParameter(
                            AnyJsFormalParameter::JsFormalParameter(parameter),
                        )],
                        [],
                    ),
                    make::token(T![')'])
                        .with_trailing_trivia_pieces(last_token.trailing_trivia().pieces()),
                );
                mutation.replace_node(
                    arrow.parameters().ok()?,
                    AnyJsArrowFunctionParameters::JsParameters(parameters),
                );
            }
            AnyErrorParameter::JsFormalParameter(parameter) => {
                if let Some(annotation) = parameter.type_annotation() {
                    replace_annotated_type(&mut mutation, &annotation, unknown_type())?;
                } else if let Some(question_mark) = parameter.question_mark_token() {
                    mutation.replace_node(
                        parameter.clone(),
                        parameter
                            .clone()
                            .with_question_mark_token(Some(
                                question_mark.with_trailing_trivia_pieces([]),
                            ))
                            .with_type_annotation(Some(make_annotation(
                                unknown_type(),
                                &question_mark,
                            )?)),
                    );
                } else {
                    let binding = parameter.binding().ok()?;
                    let last_token = binding.syntax().last_token()?;
                    mutation.replace_node(
                        parameter.clone(),
                        parameter
                            .clone()
                            .with_binding(binding.with_trailing_trivia_pieces([])?)
                            .with_type_annotation(Some(make_annotation(
                                unknown_type(),
                                &last_token,
                            )?)),
                    );
                }
            }
            AnyErrorParameter::JsRestParameter(parameter) => {
                if let Some(annotation) = parameter.type_annotation() {
                    replace_annotated_type(&mut mutation, &annotation, unknown_tuple_type())?;
                } else {
                    let binding = parameter.binding().ok()?;
                    let last_token = binding.syntax().last_token()?;
                    mutation.replace_node(
                        parameter.clone(),
                        parameter
                            .clone()
                            .with_binding(binding.with_trailing_trivia_pieces([])?)
                            .with_type_annotation(Some(make_annotation(
                                unknown_tuple_type(),
                                &last_token,
                            )?)),
                    );
                }
            }
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            if matches!(state.parameter, AnyErrorParameter::JsRestParameter(_)) {
                markup! { "Type the rest parameter as "<Emphasis>"[unknown]"</Emphasis>"." }
            } else {
                markup! { "Type the variable as "<Emphasis>"unknown"</Emphasis>"." }
            },
            mutation,
        ))
    }
}

pub struct InvalidErrorVariable {
    parameter: AnyErrorParameter,
    method: RejectionMethod,
}

declare_node_union! {
    /// [JsIdentifierBinding] is the parameter of `error => {}`.
    pub AnyErrorParameter = JsIdentifierBinding | JsFormalParameter | JsRestParameter
}

impl AnyErrorParameter {
    fn is_destructuring(&self) -> bool {
        match self {
            Self::JsFormalParameter(parameter) => !matches!(
                parameter.binding(),
                Ok(AnyJsBindingPattern::AnyJsBinding(_))
            ),
            Self::JsIdentifierBinding(_) | Self::JsRestParameter(_) => false,
        }
    }
}

#[derive(Clone, Copy)]
enum RejectionMethod {
    Catch,
    Then,
}

impl RejectionMethod {
    const fn callback_index(self) -> usize {
        match self {
            Self::Catch => 0,
            Self::Then => 1,
        }
    }
}

impl biome_console::fmt::Display for RejectionMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Catch => "catch",
            Self::Then => "then",
        })
    }
}

/// Returns `None` if the call isn't `.catch()` or `.then()` on a thenable.
fn find_invalid_error_variables(
    ctx: &RuleContext<UseUnknownInCatchCallbackVariable>,
) -> Option<Box<[InvalidErrorVariable]>> {
    let call = ctx.query();
    let callee = call.callee().ok()?.omit_parentheses();
    let member = AnyJsMemberExpression::cast_ref(callee.syntax())?;
    let method = match member.member_name()?.text() {
        "catch" => RejectionMethod::Catch,
        "then" => RejectionMethod::Then,
        _ => return None,
    };
    let arguments = call.arguments().ok()?.args();
    if arguments.len() <= method.callback_index() {
        return None;
    }
    // A spread argument may hide the position of the rejection callback.
    let mut callback = None;
    for argument in arguments.iter().take(method.callback_index() + 1) {
        callback = match argument.ok()? {
            AnyJsCallArgument::AnyJsExpression(expression) => Some(expression),
            AnyJsCallArgument::JsSpread(_) => return None,
        };
    }
    let callback = callback?;
    // Type inference doesn't see through `promise!`.
    let mut object = member.object().ok()?.omit_parentheses();
    while let AnyJsExpression::TsNonNullAssertionExpression(assertion) = object {
        object = assertion.expression().ok()?.omit_parentheses();
    }
    let is_thenable = ctx
        .type_of_expression(&object)
        .and_then(|ty| ty.is_promise_instance())
        == Some(true)
        || ctx.classify_expression_as_thenable(&object) == TypeInferenceClassification::Match;
    if !is_thenable {
        return None;
    }

    let mut invalid = Vec::new();
    let mut pending: SmallVec<[AnyJsExpression; 2]> = smallvec![callback];
    while let Some(expression) = pending.pop() {
        match expression.omit_parentheses() {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                let parameter = match arrow.parameters().ok()? {
                    AnyJsArrowFunctionParameters::AnyJsBinding(
                        AnyJsBinding::JsIdentifierBinding(binding),
                    ) => Some(AnyErrorParameter::from(binding)),
                    AnyJsArrowFunctionParameters::AnyJsBinding(_) => None,
                    AnyJsArrowFunctionParameters::JsParameters(parameters) => {
                        invalid_error_parameter(&parameters)
                    }
                };
                invalid
                    .extend(parameter.map(|parameter| InvalidErrorVariable { parameter, method }));
            }
            AnyJsExpression::JsFunctionExpression(function) => {
                invalid.extend(
                    invalid_error_parameter(&function.parameters().ok()?)
                        .map(|parameter| InvalidErrorVariable { parameter, method }),
                );
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                pending.push(conditional.alternate().ok()?);
                pending.push(conditional.consequent().ok()?);
            }
            AnyJsExpression::JsLogicalExpression(logical) => {
                pending.push(logical.right().ok()?);
                pending.push(logical.left().ok()?);
            }
            AnyJsExpression::JsSequenceExpression(sequence) => {
                pending.push(sequence.right().ok()?);
            }
            _ => {}
        }
    }
    Some(invalid.into_boxed_slice())
}

/// Skips a `this` parameter, which doesn't receive an argument.
fn invalid_error_parameter(parameters: &JsParameters) -> Option<AnyErrorParameter> {
    let parameter = parameters
        .items()
        .iter()
        .find_map(|parameter| match parameter {
            Ok(AnyJsParameter::TsThisParameter(_)) => None,
            parameter => Some(parameter),
        })?
        .ok()?;
    match parameter {
        AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
            parameter,
        )) => {
            let is_valid = match parameter.type_annotation() {
                Some(annotation) => annotation.ty().is_ok_and(is_unknown_type),
                None => false,
            };
            (!is_valid).then_some(AnyErrorParameter::from(parameter))
        }
        AnyJsParameter::JsRestParameter(parameter) => {
            let is_valid = match parameter.type_annotation() {
                Some(annotation) => annotation.ty().is_ok_and(is_unknown_rest_type),
                None => false,
            };
            (!is_valid).then_some(AnyErrorParameter::from(parameter))
        }
        AnyJsParameter::AnyJsFormalParameter(_) | AnyJsParameter::TsThisParameter(_) => None,
    }
}

fn is_unknown_type(mut ty: AnyTsType) -> bool {
    loop {
        match ty {
            AnyTsType::TsUnknownType(_) => return true,
            AnyTsType::TsParenthesizedType(parenthesized) => match parenthesized.ty() {
                Ok(inner) => ty = inner,
                Err(_) => return false,
            },
            _ => return false,
        }
    }
}

/// Accepts an empty tuple `[]`, because then the callback doesn't receive the error.
fn is_unknown_rest_type(mut ty: AnyTsType) -> bool {
    loop {
        let inner = match ty {
            AnyTsType::TsParenthesizedType(parenthesized) => parenthesized.ty(),
            AnyTsType::TsTypeOperatorType(operator) => {
                if !operator
                    .operator_token()
                    .is_ok_and(|token| token.kind() == T![readonly])
                {
                    return false;
                }
                operator.ty()
            }
            AnyTsType::TsArrayType(array) => {
                return array.element_type().is_ok_and(is_unknown_type);
            }
            AnyTsType::TsReferenceType(reference) => {
                let Ok(AnyTsName::JsReferenceIdentifier(name)) = reference.name() else {
                    return false;
                };
                let Ok(name) = name.value_token() else {
                    return false;
                };
                let Some(arguments) = reference.type_arguments() else {
                    return false;
                };
                let arguments = arguments.ts_type_argument_list();
                if arguments.len() != 1 {
                    return false;
                }
                let Some(Ok(argument)) = arguments.first() else {
                    return false;
                };
                match name.text_trimmed() {
                    "Array" | "ReadonlyArray" => return is_unknown_type(argument),
                    "Readonly" => Ok(argument),
                    _ => return false,
                }
            }
            AnyTsType::TsTupleType(tuple) => match tuple.elements().first() {
                None => return true,
                Some(Err(_)) => return false,
                Some(Ok(element)) => match element {
                    AnyTsTupleTypeElement::AnyTsType(element) => {
                        return is_unknown_type(element);
                    }
                    AnyTsTupleTypeElement::TsOptionalTupleTypeElement(element) => {
                        return element.ty().is_ok_and(is_unknown_type);
                    }
                    AnyTsTupleTypeElement::TsNamedTupleTypeElement(element) => {
                        if element.dotdotdot_token().is_none() {
                            return element.ty().is_ok_and(is_unknown_type);
                        }
                        element.ty()
                    }
                    AnyTsTupleTypeElement::TsRestTupleTypeElement(element) => element.ty(),
                },
            },
            _ => return false,
        };
        match inner {
            Ok(inner) => ty = inner,
            Err(_) => return false,
        }
    }
}

/// Keeps the comments around the old type.
fn replace_annotated_type(
    mutation: &mut BatchMutation<JsLanguage>,
    annotation: &TsTypeAnnotation,
    ty: AnyTsType,
) -> Option<()> {
    let old = annotation.ty().ok()?;
    let first_token = old.syntax().first_token()?;
    let last_token = old.syntax().last_token()?;
    let ty = ty
        .prepend_trivia_pieces(first_token.leading_trivia().pieces())?
        .append_trivia_pieces(last_token.trailing_trivia().pieces())?;
    mutation.replace_node(old, ty);
    Some(())
}

/// Moves the trailing comments of `previous_token` after the new annotation.
fn make_annotation(ty: AnyTsType, previous_token: &JsSyntaxToken) -> Option<TsTypeAnnotation> {
    Some(make::ts_type_annotation(
        make::token_with_trailing_space(T![:]),
        ty.append_trivia_pieces(previous_token.trailing_trivia().pieces())?,
    ))
}

fn unknown_type() -> AnyTsType {
    AnyTsType::TsUnknownType(make::ts_unknown_type(make::token(T![unknown])))
}

fn unknown_tuple_type() -> AnyTsType {
    AnyTsType::TsTupleType(make::ts_tuple_type(
        make::token(T!['[']),
        make::ts_tuple_type_element_list([AnyTsTupleTypeElement::AnyTsType(unknown_type())], []),
        make::token(T![']']),
    ))
}
