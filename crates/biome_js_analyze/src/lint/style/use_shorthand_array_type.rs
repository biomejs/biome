use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyTsType, JsSyntaxKind, JsSyntaxToken, TsReferenceType, TsTypeArguments, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxNodeOptionExt, TriviaPiece};

use crate::JsRuleAction;

declare_lint_rule! {
    /// When expressing array types, this rule promotes the usage of `T[]` shorthand instead of `Array<T>`.
    ///
    /// ## Examples
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
    /// let invalid: Array<Foo<Bar>>;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalid: Array<[number, number]>;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalid: Array<[number, number]>;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let invalid: ReadonlyArray<string>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let valid: Array<Foo | Bar>;
    /// let valid: Array<keyof Bar>;
    /// let valid: Array<foo | bar>;
    /// ```
    pub UseShorthandArrayType  {
        version: "1.0.0",
        name: "useShorthandArrayType",
        language: "ts",
        recommended: false,
        deprecated: "Use `useConsistentArrayType` instead.",
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug, Copy, Clone)]
enum TsArrayKind {
    /// `Array<T>`
    Simple,
    /// `ReadonlyArray<T>`
    Readonly,
}

impl Rule for UseShorthandArrayType {
    type Query = Ast<TsReferenceType>;
    type State = AnyTsType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let array_kind = get_array_kind_by_reference(node)?;

        // Ignore `Array` in the `extends` and `implements` clauses.
        let parent = node
            .syntax()
            .ancestors()
            .skip(1)
            .find(|ancestor| ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION);
        if parent.kind() == Some(JsSyntaxKind::TS_TYPE_LIST) {
            return None;
        }

        convert_to_array_type(&node.type_arguments()?, array_kind)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        if let Some(kind) = get_array_kind_by_reference(node) {
            return Some(RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                match kind {
                    TsArrayKind::Simple => {
                        markup! {"Use "<Emphasis>"shorthand T[] syntax"</Emphasis>" instead of "<Emphasis>"Array<T> syntax."</Emphasis>}
                    }
                    TsArrayKind::Readonly => {
                        markup! {"Use "<Emphasis>"shorthand readonly T[] syntax"</Emphasis>" instead of "<Emphasis>"ReadonlyArray<T> syntax."</Emphasis>}
                    }
                },
            ));
        };
        None
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        mutation.replace_node(AnyTsType::TsReferenceType(node.clone()), state.clone());

        if let Some(kind) = get_array_kind_by_reference(node) {
            let message = match kind {
                TsArrayKind::Simple => {
                    markup! { "Use "<Emphasis>"shorthand T[] syntax"</Emphasis>" to replace" }
                        .to_owned()
                }
                TsArrayKind::Readonly => {
                    markup! { "Use "<Emphasis>"shorthand readonly T[] syntax"</Emphasis>" to replace" }
                        .to_owned()
                }
            };
            return Some(JsRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                message,
                mutation,
            ));
        };
        None
    }
}

fn get_array_kind_by_reference(ty: &TsReferenceType) -> Option<TsArrayKind> {
    let name = ty.name().ok()?;
    name.as_js_reference_identifier().and_then(|identifier| {
        let name = identifier.value_token().ok()?;
        match name.text_trimmed() {
            "Array" => Some(TsArrayKind::Simple),
            "ReadonlyArray" => Some(TsArrayKind::Readonly),
            _ => None,
        }
    })
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
                    TsArrayKind::Simple => AnyTsType::TsArrayType(make::ts_array_type(
                        element_type,
                        make::token(T!['[']),
                        make::token(T![']']),
                    )),
                    TsArrayKind::Readonly => {
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
                                    return AnyTsType::TsTypeOperatorType(
                                        make::ts_type_operator_type(
                                            readonly_token,
                                            // wrap ArrayType
                                            AnyTsType::TsArrayType(make::ts_array_type(
                                                AnyTsType::TsParenthesizedType(
                                                    make::ts_parenthesized_type(
                                                        make::token(T!['(']),
                                                        element_type,
                                                        make::token(T![')']),
                                                    ),
                                                ),
                                                make::token(T!['[']),
                                                make::token(T![']']),
                                            )),
                                        ),
                                    );
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
                })
            })
            .collect::<Vec<_>>();

        match types_array.len() {
            0 => {}
            1 => {
                // SAFETY: We know that `length` of `array_types` is 1, so unwrap the first element should be safe.
                let first_type = types_array.into_iter().next()?;
                return Some(first_type);
            }
            length => {
                let ts_union_type_builder = make::ts_union_type(make::ts_union_type_variant_list(
                    types_array,
                    (0..length - 1).map(|_| make::token_decorated_with_space(T![|])),
                ));
                return Some(AnyTsType::TsUnionType(ts_union_type_builder.build()));
            }
        }
    }
    None
}
