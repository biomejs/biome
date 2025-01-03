use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::{markup, Markup, MarkupBuf};
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyTsName, AnyTsType, JsSyntaxKind, JsSyntaxToken, TriviaPieceKind, TsReferenceType,
    TsTypeArguments, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxNodeOptionExt, TriviaPiece};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

declare_lint_rule! {
    /// Require consistently using either `T[]` or `Array<T>`
    ///
    /// _TypeScript_ provides two equivalent ways to define an array type: `T[]` and `Array<T>`.
    /// The two styles are functionally equivalent.
    /// Using the same style consistently across your codebase makes it easier for developers to read and understand array types.
    ///
    /// ## Example
    ///
    /// ### Invalid
    /// ```ts,expect_diagnostic
    /// let invalid: Array<foo>;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalid: Promise<Array<string>>;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalid3: Array<Foo<Bar>>;
    /// ```
    ///
    /// ### Valid
    /// ```ts
    /// const valid: Array<string | number> = ['a', 'b'];
    /// const valid: Array<{ prop: string }> = [{ prop: 'a' }];
    /// const valid: Array<() => void> = [() => {}];
    /// const valid: MyType[] = ['a', 'b'];
    /// const valid: string[] = ['a', 'b'];
    /// const valid: readonly string[] = ['a', 'b'];
    /// ```
    ///
    /// ## Options
    ///
    /// Use the options to specify the syntax of array declarations to use.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "syntax": "shorthand"
    ///     }
    /// }
    /// ```
    ///
    /// ### syntax
    ///
    /// The syntax to use:
    /// - `generic`: array declarations will be converted to `Array<T>` or `ReadonlyArray<T>`
    /// - `shorthand`: array declarations will be converted to `T[]` or `readonly T[]`
    ///
    /// Default: `shorthand`
    ///
    pub UseConsistentArrayType {
        version: "1.5.0",
        name: "useConsistentArrayType",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("array-type")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug, Copy, Clone)]
enum TsArrayKind {
    /// `T[]`
    Shorthand,
    /// `Array<T>`
    GenericArray,
    /// `readonly T[]`
    Readonly,
    /// `ReadonlyArray<T>`
    ReadonlyGenericArray,
}

impl Rule for UseConsistentArrayType {
    type Query = Ast<AnyTsType>;
    type State = AnyTsType;
    type Signals = Option<Self::State>;
    type Options = ConsistentArrayTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query = ctx.query();
        let options = ctx.options();

        match query {
            AnyTsType::TsTypeOperatorType(_) | AnyTsType::TsArrayType(_)
                if query.syntax().parent().kind() != Some(JsSyntaxKind::TS_TYPE_OPERATOR_TYPE) =>
            {
                if options.syntax == ConsistentArrayType::Shorthand {
                    return None;
                }
                let array_kind = get_array_kind_by_any_type(query)?;
                transform_array_type(query.to_owned(), array_kind)
            }
            AnyTsType::TsReferenceType(ty) => {
                if options.syntax == ConsistentArrayType::Generic {
                    return None;
                }

                // Ignore `Array` in the `extends` and `implements` clauses.
                let parent =
                    ty.syntax().ancestors().skip(1).find(|ancestor| {
                        ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
                    });
                if parent.kind() == Some(JsSyntaxKind::TS_TYPE_LIST) {
                    return None;
                }

                let array_kind = get_array_kind_by_reference(ty)?;
                convert_to_array_type(&ty.type_arguments()?, array_kind)
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let query = ctx.query();
        let options = ctx.options();

        match query {
            AnyTsType::TsTypeOperatorType(_) | AnyTsType::TsArrayType(_)
                if query.syntax().parent().kind() != Some(JsSyntaxKind::TS_TYPE_OPERATOR_TYPE) =>
            {
                if options.syntax == ConsistentArrayType::Shorthand {
                    return None;
                }

                let is_readonly = matches!(query, AnyTsType::TsTypeOperatorType(_));
                let title = if is_readonly {
                    get_diagnostic_title(TsArrayKind::Readonly)
                } else {
                    get_diagnostic_title(TsArrayKind::Shorthand)
                };

                Some(RuleDiagnostic::new(rule_category!(), query.range(), title))
            }
            AnyTsType::TsReferenceType(ty) => {
                if options.syntax == ConsistentArrayType::Generic {
                    return None;
                }

                if let Some(kind) = get_array_kind_by_reference(ty) {
                    return Some(RuleDiagnostic::new(
                        rule_category!(),
                        ty.range(),
                        get_diagnostic_title(kind),
                    ));
                }

                None
            }
            _ => None,
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let query = ctx.query();
        let mut mutation = ctx.root().begin();

        match query {
            AnyTsType::TsReferenceType(ty) => {
                mutation.replace_node(AnyTsType::TsReferenceType(ty.clone()), state.clone());
                if let Some(kind) = get_array_kind_by_reference(ty) {
                    return Some(JsRuleAction::new(
                        ctx.metadata().action_category(ctx.category(), ctx.group()),
                        ctx.metadata().applicability(),
                        get_action_message(kind),
                        mutation,
                    ));
                }

                None
            }
            AnyTsType::TsTypeOperatorType(ty) => {
                mutation.replace_node(AnyTsType::TsTypeOperatorType(ty.clone()), state.clone());
                let ty = ty.ty().ok()?;

                if let Some(kind) = get_array_kind_by_any_type(&ty) {
                    return Some(JsRuleAction::new(
                        ctx.metadata().action_category(ctx.category(), ctx.group()),
                        ctx.metadata().applicability(),
                        get_action_message(kind),
                        mutation,
                    ));
                }

                None
            }
            AnyTsType::TsArrayType(ty)
                if query.syntax().parent().kind() != Some(JsSyntaxKind::TS_TYPE_OPERATOR_TYPE) =>
            {
                mutation.replace_node(AnyTsType::TsArrayType(ty.clone()), state.clone());
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    get_action_message(TsArrayKind::Shorthand),
                    mutation,
                ))
            }
            _ => None,
        }
    }
}

fn get_array_kind_by_reference(ty: &TsReferenceType) -> Option<TsArrayKind> {
    let name = ty.name().ok()?;
    name.as_js_reference_identifier().and_then(|ident| {
        let name = ident.value_token().ok()?;
        match name.text_trimmed() {
            "Array" => Some(TsArrayKind::GenericArray),
            "ReadonlyArray" => Some(TsArrayKind::ReadonlyGenericArray),
            _ => None,
        }
    })
}

fn get_array_kind_by_any_type(ty: &AnyTsType) -> Option<TsArrayKind> {
    if let AnyTsType::TsTypeOperatorType(ty) = ty {
        let operator_token = ty.operator_token().ok()?;
        let is_readonly = operator_token.text_trimmed() == "readonly";

        return (matches!(ty.ty(), Ok(AnyTsType::TsArrayType(_))) && is_readonly)
            .then_some(TsArrayKind::Readonly);
    } else if let AnyTsType::TsArrayType(_) = ty {
        return Some(TsArrayKind::Shorthand);
    }

    None
}

fn transform_array_type(ty: AnyTsType, array_kind: TsArrayKind) -> Option<AnyTsType> {
    if let AnyTsType::TsArrayType(_) = ty {
        let array_types = transform_array_element_type(ty, array_kind)
            .into_iter()
            .collect::<Vec<_>>();

        return get_array_type(array_types);
    } else if let AnyTsType::TsTypeOperatorType(opt_ty) = ty {
        let ty = opt_ty.ty().ok()?;
        return transform_array_type(ty, array_kind);
    }

    None
}

fn convert_to_array_type(
    type_arguments: &TsTypeArguments,
    array_kind: TsArrayKind,
) -> Option<AnyTsType> {
    if type_arguments.ts_type_argument_list().len() > 0 {
        let types_array = type_arguments
            .ts_type_argument_list()
            .into_iter()
            .filter_map(|param| {
                let param = param.ok()?;
                transform_array_element_type(param, array_kind)
            })
            .collect::<Vec<_>>();

        return get_array_type(types_array);
    }
    None
}

fn get_array_type(array_types: Vec<AnyTsType>) -> Option<AnyTsType> {
    match array_types.len() {
        0 => None,
        1 => {
            // SAFETY: We know that `length` of `array_types` is 1, so unwrap the first element should be safe.
            let first_type = array_types.into_iter().next()?;
            Some(first_type)
        }
        length => {
            let ts_union_type_builder = make::ts_union_type(make::ts_union_type_variant_list(
                array_types,
                (0..length - 1).map(|_| {
                    make::token(T![|])
                        .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
                }),
            ));
            Some(AnyTsType::TsUnionType(ts_union_type_builder.build()))
        }
    }
}

fn transform_array_element_type(param: AnyTsType, array_kind: TsArrayKind) -> Option<AnyTsType> {
    let element_type = match &param {
        // Intersection or higher types
        AnyTsType::TsUnionType(_)
        | AnyTsType::TsIntersectionType(_)
        | AnyTsType::TsFunctionType(_)
        | AnyTsType::TsConstructorType(_)
        | AnyTsType::TsConditionalType(_)
        | AnyTsType::TsTypeOperatorType(_)
        | AnyTsType::TsInferType(_)
        | AnyTsType::TsObjectType(_)
        | AnyTsType::TsMappedType(_) => None,

        AnyTsType::TsReferenceType(ty) => match get_array_kind_by_reference(ty) {
            Some(array_kind) => {
                if let Some(type_arguments) = ty.type_arguments() {
                    convert_to_array_type(&type_arguments, array_kind)
                } else {
                    Some(param)
                }
            }
            None => Some(param),
        },

        _ => Some(param),
    };

    element_type.map(|element_type| match array_kind {
        TsArrayKind::GenericArray => AnyTsType::TsArrayType(make::ts_array_type(
            element_type,
            make::token(T!['[']),
            make::token(T![']']),
        )),
        TsArrayKind::ReadonlyGenericArray => {
            let readonly_token = JsSyntaxToken::new_detached(
                JsSyntaxKind::TS_READONLY_MODIFIER,
                "readonly ",
                [],
                [TriviaPiece::whitespace(1)],
            );

            // Modify `ReadonlyArray<ReadonlyArray<T>>` to `readonly (readonly T[])[]`
            if let AnyTsType::TsTypeOperatorType(op) = &element_type {
                if let Ok(op) = op.operator_token() {
                    if op.text_trimmed() == "readonly" {
                        return AnyTsType::TsTypeOperatorType(make::ts_type_operator_type(
                            readonly_token,
                            // wrap ArrayType
                            AnyTsType::TsArrayType(make::ts_array_type(
                                AnyTsType::TsParenthesizedType(make::ts_parenthesized_type(
                                    make::token(T!['(']),
                                    element_type,
                                    make::token(T![')']),
                                )),
                                make::token(T!['[']),
                                make::token(T![']']),
                            )),
                        ));
                    }
                }
            }

            AnyTsType::TsTypeOperatorType(make::ts_type_operator_type(
                readonly_token,
                AnyTsType::TsArrayType(make::ts_array_type(
                    element_type,
                    make::token(T!['[']),
                    make::token(T![']']),
                )),
            ))
        }

        TsArrayKind::Shorthand => {
            let element_type = if let AnyTsType::TsArrayType(array_type) = element_type {
                array_type.element_type().ok()
            } else {
                Some(element_type)
            };
            generate_array_type(element_type, false)
        }
        TsArrayKind::Readonly => {
            let element_type = if let AnyTsType::TsArrayType(array_type) = &element_type {
                if let Ok(element_type) = array_type.element_type() {
                    element_type
                } else {
                    element_type
                }
            } else {
                element_type
            };

            let element_type = if let AnyTsType::TsParenthesizedType(paren_type) = element_type {
                match paren_type.ty() {
                    Ok(AnyTsType::TsTypeOperatorType(opt_ty)) => {
                        let ele_type = if let Ok(AnyTsType::TsArrayType(array_type)) = opt_ty.ty() {
                            array_type.element_type().ok()
                        } else {
                            None
                        };
                        Some(generate_array_type(ele_type, true))
                    }
                    Ok(AnyTsType::TsArrayType(array_type)) => {
                        let ele_type = array_type.element_type().ok();
                        Some(generate_array_type(ele_type, false))
                    }
                    _ => None,
                }
            } else {
                Some(element_type)
            };

            generate_array_type(element_type, true)
        }
    })
}

fn get_diagnostic_title<'a>(array_kind: TsArrayKind) -> Markup<'a> {
    match array_kind {
        TsArrayKind::GenericArray => {
            markup! {"Use "<Emphasis>"shorthand T[] syntax"</Emphasis>" instead of "<Emphasis>"Array<T> syntax."</Emphasis>}
        }
        TsArrayKind::ReadonlyGenericArray => {
            markup! {"Use "<Emphasis>"shorthand readonly T[] syntax"</Emphasis>" instead of "<Emphasis>"ReadonlyArray<T> syntax."</Emphasis>}
        }
        TsArrayKind::Shorthand => {
            markup! {"Use "<Emphasis>"Array<T> syntax"</Emphasis>" instead of "<Emphasis>"shorthand T[] syntax."</Emphasis>}
        }
        TsArrayKind::Readonly => {
            markup! {"Use "<Emphasis>"ReadonlyArray<T> syntax"</Emphasis>" instead of "<Emphasis>"shorthand readonly T[] syntax."</Emphasis>}
        }
    }
}

fn get_action_message(array_kind: TsArrayKind) -> MarkupBuf {
    match array_kind {
        TsArrayKind::GenericArray => {
            markup! { "Use "<Emphasis>"shorthand T[] syntax"</Emphasis>" to replace" }.to_owned()
        }
        TsArrayKind::ReadonlyGenericArray => {
            markup! { "Use "<Emphasis>"shorthand readonly T[] syntax"</Emphasis>" to replace" }
                .to_owned()
        }
        TsArrayKind::Shorthand => {
            markup! { "Use "<Emphasis>"Array<T> syntax"</Emphasis>" to replace"}.to_owned()
        }
        TsArrayKind::Readonly => {
            markup! { "Use "<Emphasis>"ReadonlyArray<T> syntax"</Emphasis>" to replace"}.to_owned()
        }
    }
}

fn generate_array_type<I>(element_type: I, is_readonly: bool) -> AnyTsType
where
    I: IntoIterator<Item = AnyTsType>,
    I::IntoIter: ExactSizeIterator,
{
    let ident = if is_readonly {
        make::ident("ReadonlyArray")
    } else {
        make::ident("Array")
    };

    AnyTsType::TsReferenceType(
        make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
            make::js_reference_identifier(ident),
        ))
        .with_type_arguments(make::ts_type_arguments(
            make::token(T![<]),
            make::ts_type_argument_list(element_type, None),
            make::token(T![>]),
        ))
        .build(),
    )
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct ConsistentArrayTypeOptions {
    pub syntax: ConsistentArrayType,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ConsistentArrayType {
    /// `ItemType[]`
    #[default]
    Shorthand,
    /// `Array<ItemType>`
    Generic,
}

impl FromStr for ConsistentArrayType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shorthand" => Ok(Self::Shorthand),
            "generic" => Ok(Self::Generic),
            _ => Err("Value not supported for array type syntax"),
        }
    }
}
