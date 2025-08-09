use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsCallArguments, JsCallExpression,
    JsNewExpression, JsPropertyObjectMember, T, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TokenText, TriviaPieceKind};
use biome_rule_options::use_static_response_methods::UseStaticResponseMethodsOptions;
use biome_string_case::StrLikeExtension;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Use static `Response` methods instead of `new Response()` constructor when possible.
    ///
    /// `new Response(JSON.stringify({ value: 1 }))` can be simplified to [Response.json()](https://developer.mozilla.org/en-US/docs/Web/API/Response/json).
    /// `new Response(null, { status: 301, headers: { location: 'https://example.com' } })` can be simplified to [Response.redirect()](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static).
    ///
    /// These methods are more concise and emphasize the intent of the code better,
    /// however they are not a direct replacement when additional options such as extra headers are needed.
    ///
    /// In case of `Response.redirect()`, the `location` header must also be a full URL, because server runtimes (Node, Deno, etc.) will throw an error for relative URLs.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// new Response(JSON.stringify({ value: 1 }));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Response(JSON.stringify({ value: 0 }), {
    ///     headers: {
    ///         'Content-Type': 'application/json',
    ///     }
    /// })
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Response(null, {
    ///    headers: {
    ///        location: 'https://example.com',
    ///    },
    ///    status: 302,
    /// })
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // JSON.stringify() with a replacer function
    /// new Response(JSON.stringify({ value: 0 }, () => {}))
    /// ```
    ///
    /// ```js
    /// new Response(null, {
    ///    headers: {
    ///        location: 'https://example.com',
    ///        'x-foo': 'extra-header',
    ///    },
    ///    status: 302,
    /// })
    /// ```
    ///
    /// ```js
    /// new Response(null, {
    ///    headers: {
    ///        location: '/relative-url',
    ///    },
    ///    status: 302,
    /// })
    /// ```
    ///
    pub UseStaticResponseMethods {
        version: "2.0.0",
        name: "useStaticResponseMethods",
        language: "js",
        fix_kind: FixKind::Unsafe,
        recommended: false,
    }
}

impl Rule for UseStaticResponseMethods {
    type Query = Semantic<JsNewExpression>;
    type State = ResponseSimplification;
    type Signals = Option<Self::State>;
    type Options = UseStaticResponseMethodsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let (reference, name) = global_identifier(&callee.omit_parentheses())?;

        if name.text() != "Response" {
            return None;
        }

        let model = ctx.model();
        if model.binding(&reference).is_some() {
            return None;
        }

        let args = node.arguments()?;
        if args.args().len() > 2 {
            return None;
        }

        check_for_json_simplification(model, &args)
            .or_else(|| check_for_redirect_simplification(&args))
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let (suggestion, instead_of) = match state {
            ResponseSimplification::Json(_) => {
                ("Response.json()", "new Response(JSON.stringify())")
            }
            ResponseSimplification::Redirect(_) => ("Response.redirect()", "new Response()"),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use "<Emphasis>{{suggestion}}</Emphasis>" instead of "<Emphasis>{instead_of}</Emphasis>"."
                },
            )
            .note(markup! {
                <Emphasis>{suggestion}</Emphasis>" is more concise and emphasizes the intent of the code better."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let response_json_expr = make::js_static_member_expression(
            make::js_identifier_expression(make::js_reference_identifier(make::ident("Response")))
                .into(),
            make::token(T![.]),
            make::js_name(make::ident("json")).into(),
        );

        let args_values = match state {
            ResponseSimplification::Json(arg) => make::js_call_argument_list([arg.clone()], []),
            ResponseSimplification::Redirect((status, location)) => {
                let status_literal = AnyJsLiteralExpression::JsNumberLiteralExpression(
                    make::js_number_literal_expression(make::js_number_literal(
                        status.to_string().as_str(),
                    )),
                );
                let status_arg = AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::AnyJsLiteralExpression(status_literal),
                );

                let location_literal = AnyJsLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(make::js_string_literal(location)),
                );
                let location_arg = AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::AnyJsLiteralExpression(location_literal),
                );

                make::js_call_argument_list(
                    [location_arg, status_arg],
                    Some(
                        make::token(T![,])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    ),
                )
            }
        };

        let args = make::js_call_arguments(make::token(T!['(']), args_values, make::token(T![')']));

        let new_call_expr = make::js_call_expression(response_json_expr.into(), args).build();
        mutation.replace_element(node.clone().into(), new_call_expr.into());

        let suggestion = match state {
            ResponseSimplification::Json(_) => "Response.json()",
            ResponseSimplification::Redirect(_) => "Response.redirect()",
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Replace with "<Emphasis>{{suggestion}}</Emphasis>"."
            }
            .to_owned(),
            mutation,
        ))
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static#status
const REDIRECT_STATUS_CODES: [u16; 5] = [301, 302, 303, 307, 308];

pub enum ResponseSimplification {
    Json(AnyJsCallArgument),
    Redirect((u16, Box<str>)),
}

fn check_for_json_simplification(
    model: &SemanticModel,
    args: &JsCallArguments,
) -> Option<ResponseSimplification> {
    let [Some(first_arg), second_arg] = args.get_arguments_by_index([0, 1]) else {
        return None;
    };

    let first_arg_call_expr = first_arg.as_any_js_expression()?.as_js_call_expression()?;
    let json_arg = extract_json_stringify_arg(model, first_arg_call_expr)?;

    let second_arg = second_arg.and_then(|arg| AnyJsExpression::cast_ref(arg.syntax()));

    if let Some(second_arg) = second_arg {
        for_each_object_member(&second_arg, &mut |name, property| {
            if name != "headers" {
                return None;
            }

            for_each_object_member(&property.value().ok()?, &mut |name, property| {
                if name != "content-type" {
                    return None;
                }

                let value = get_property_string_literal_value(property)?;
                if value != "application/json" {
                    return None;
                }

                Some(())
            })
        })?;
    }

    Some(ResponseSimplification::Json(json_arg))
}

fn check_for_redirect_simplification(args: &JsCallArguments) -> Option<ResponseSimplification> {
    let [Some(first_arg), Some(second_arg)] = args.get_arguments_by_index([0, 1]) else {
        return None;
    };

    let first_arg_any = first_arg.as_any_js_expression()?;
    let is_first_arg_empty = match first_arg_any {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNullLiteralExpression(_),
        ) => true,
        AnyJsExpression::JsIdentifierExpression(expr) => {
            expr.name().map(|ident| ident.is_undefined()).ok()?
        }
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(expr),
        ) => expr.inner_string_text().ok()?.text().is_empty(),
        _ => return None,
    };

    if !is_first_arg_empty {
        return None;
    }

    let mut status: Option<u16> = None;
    let mut location: Option<Box<str>> = None;

    for_each_object_member(
        second_arg.as_any_js_expression()?,
        &mut |name, property| match name {
            "status" => {
                let value = property.value().ok()?;
                let number = value
                    .as_any_js_literal_expression()?
                    .as_js_number_literal_expression()?
                    .as_number()? as u16;

                if REDIRECT_STATUS_CODES.contains(&number) {
                    status = Some(number);
                }

                Some(())
            }
            "headers" => {
                let object = property.value().ok()?;

                for_each_object_member(&object, &mut |name, property| {
                    if name != "location" {
                        return None;
                    }

                    let value = get_property_string_literal_value(property).take_if(|v| {
                        // In server runtimes (Node, Deno, etc.) relative URLs will throw an error.
                        // https://github.com/denoland/deno/issues/20674
                        v.starts_with("http")
                    })?;

                    location = Some(value.text().into());

                    Some(())
                })
            }
            _ => None,
        },
    )?;

    Some(ResponseSimplification::Redirect((status?, location?)))
}

/// Extracts the argument from a `JSON.stringify` call.
fn extract_json_stringify_arg(
    model: &SemanticModel,
    call_expr: &JsCallExpression,
) -> Option<AnyJsCallArgument> {
    let callee = call_expr.callee().ok()?;
    let callee = callee.as_js_static_member_expression()?;
    let object = callee.object().ok()?;
    let (reference, object_name) = global_identifier(&object.omit_parentheses())?;

    if object_name.text() != "JSON" {
        return None;
    }

    if model.binding(&reference).is_some() {
        return None;
    }

    let args = call_expr.arguments().ok()?.args();

    // If arguments are empty or contain more than one argument
    // we can't use shorthand.
    if args.len() != 1 {
        return None;
    }

    args.first()?.ok()
}

fn for_each_object_member(
    expr: &AnyJsExpression,
    each_member: &mut impl FnMut(&str, &JsPropertyObjectMember) -> Option<()>,
) -> Option<()> {
    let object_expr = expr.as_js_object_expression()?;

    for member in object_expr.members() {
        let member = member.ok()?;
        let property_member = member.as_js_property_object_member()?;
        let name = property_member.name().ok()?.name()?;
        let text = name.to_ascii_lowercase_cow();

        each_member(&text, property_member)?;
    }

    Some(())
}

fn get_property_string_literal_value(property: &JsPropertyObjectMember) -> Option<TokenText> {
    let value = property.value().ok()?;
    let string_literal = value
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?;

    string_literal.inner_string_text().ok()
}
