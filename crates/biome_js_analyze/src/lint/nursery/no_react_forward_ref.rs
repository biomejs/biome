use std::sync::Arc;

use camino::Utf8PathBuf;

use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsExpression, AnyJsFormalParameter, AnyJsObjectBindingPatternMember, AnyJsParameter,
    AnyTsType, AnyTsTypeMember, JsCallExpression, JsIdentifierBinding, JsParameters, T,
    TsPropertySignatureTypeMember, TsReferenceType,
};
use biome_package::PackageJson;
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::no_react_forward_ref::NoReactForwardRefOptions;

use crate::JsRuleAction;
use crate::react::{ReactLibrary, is_global_react_import, is_react_call_api};
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Replaces usages of `forwardRef` with passing `ref` as a prop.
    ///
    /// In React 19, `forwardRef` is no longer necessary. Pass `ref` as a prop instead.
    /// This rule detects the usage of the `forwardRef` API, and it suggests using the prop `ref`
    /// instead.
    /// See [the official blog post](https://react.dev/blog/2024/12/05/react-19#ref-as-a-prop) for details.
    ///
    /// This rule should be disabled if you are working with React 18 or earlier.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import { forwardRef } from "react";
    ///
    /// const MyInput = forwardRef(function MyInput(props, ref) {
    ///   return <input ref={ref} {...props} />;
    /// });
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// import { forwardRef } from "react";
    ///
    /// const MyInput = forwardRef((props, ref) => {
    ///   return <input ref={ref} {...props} />;
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function MyInput({ ref, ...props }) {
    ///   return <input ref={ref} {...props} />;
    /// }
    /// ```
    ///
    /// ```jsx
    /// const MyInput = ({ ref, ...props }) => {
    ///   return <input ref={ref} {...props} />;
    /// }
    /// ```
    ///
    pub NoReactForwardRef {
        version: "2.2.5",
        name: "noReactForwardRef",
        language: "js",
        severity: Severity::Warning,
        domains: &[RuleDomain::React],
        sources: &[
            RuleSource::EslintReactX("no-forward-ref").same(),
            RuleSource::EslintReactXyz("no-forward-ref").same(),
        ],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoReactForwardRef {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoReactForwardRefOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let callee = node.callee().ok()?;

        let is_react_19 = ctx
            .get_service::<Option<(Utf8PathBuf, Arc<PackageJson>)>>()
            .and_then(|manifest| {
                manifest
                    .as_ref()
                    .map(|(_, package_json)| package_json.matches_dependency("react", ">=19.0.0"))
            });

        if is_react_19 == Some(false) {
            return None;
        }

        is_react_call_api(&callee, model, ReactLibrary::React, "forwardRef").then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use of "<Emphasis>"forwardRef"</Emphasis>" is detected, which is deprecated."
                }
            )
            .note(markup! {
                "In React 19, 'forwardRef' is no longer necessary. Pass 'ref' as a prop instead."
            })
            .note(markup! {
                "Replace the use of "<Emphasis>"forwardRef"</Emphasis>" with passing "<Emphasis>"ref"</Emphasis>" as a prop."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let model = ctx.model();
        let mut mutation = ctx.root().begin();

        let AnyJsCallArgument::AnyJsExpression(function) =
            node.arguments().ok()?.args().first()?.ok()?
        else {
            return None;
        };

        let type_args = match node.type_arguments() {
            Some(p) => {
                let mut iter = p.ts_type_argument_list().into_iter();
                let ref_type = iter.next().and_then(|node| node.ok());
                let props_type = iter.next().and_then(|node| node.ok());

                (ref_type, props_type)
            }
            _ => (None, None),
        };

        let global_react_import = model
            .all_bindings()
            .filter_map(|binding| JsIdentifierBinding::cast_ref(binding.syntax()))
            .find(|binding| is_global_react_import(binding, ReactLibrary::React));

        let new_function: AnyJsExpression = match function.clone() {
            AnyJsExpression::JsArrowFunctionExpression(f) => {
                let AnyJsArrowFunctionParameters::JsParameters(params) = f.parameters().ok()?
                else {
                    return None;
                };

                f.with_parameters(fix_parameters(params, type_args, global_react_import)?.into())
                    .into()
            }
            AnyJsExpression::JsFunctionExpression(f) => {
                let params = f.parameters().ok()?;

                f.with_parameters(fix_parameters(params, type_args, global_react_import)?)
                    .into()
            }
            _ => return None,
        };

        mutation.replace_node(node.clone().into(), new_function);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"forwardRef()"</Emphasis>" call and receive the "<Emphasis>"ref"</Emphasis>" as a prop." },
            mutation,
        ))
    }
}

fn fix_parameters(
    params: JsParameters,
    type_args: (Option<AnyTsType>, Option<AnyTsType>),
    global_react_import: Option<JsIdentifierBinding>,
) -> Option<JsParameters> {
    let mut iter = params.items().into_iter();
    let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(props)) =
        iter.next()?.ok()?
    else {
        return None;
    };

    let mut new_props = props.clone();

    // If the forwarded ref is used in the function, add it into the first parameter
    if let Some(AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
        r#ref,
    ))) = iter.next().and_then(|node| node.ok())
    {
        let mut new_ref = match r#ref.binding().ok()? {
            // (props, ref) => ({ ref, ...props })
            AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(b))
                if b.name_token().ok()?.text_trimmed() == "ref" =>
            {
                make::js_object_binding_pattern_shorthand_property(b.into())
                    .build()
                    .into()
            }
            // (props, myRef) => ({ ref: myRef, ...props })
            // (props, { current }) => ({ ref: { current }, ...props })
            _ => make::js_object_binding_pattern_property(
                make::js_literal_member_name(make::ident("ref")).into(),
                make::token_with_trailing_space(T![:]),
                r#ref.binding().ok()?,
            )
            .build()
            .into(),
        };

        match props.binding().ok()? {
            // (props, ref) => ({ ref, ...props })
            AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
                let properties = make::js_object_binding_pattern_property_list(
                    [
                        new_ref,
                        make::js_object_binding_pattern_rest(make::token(T![...]), binding.into())
                            .into(),
                    ],
                    [make::token_with_trailing_space(T![,])],
                );

                new_props = new_props.with_binding(
                    make::js_object_binding_pattern(
                        make::token_with_trailing_space(T!['{']),
                        properties,
                        make::token_with_leading_space(T!['}']),
                    )
                    .into(),
                );
            }

            // ({ foo, bar }, ref) => ({ foo, bar, ref })
            AnyJsBindingPattern::JsObjectBindingPattern(binding) => {
                let rest =
                    binding
                        .properties()
                        .into_iter()
                        .find_map(|member| match member.ok()? {
                            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                                Some(rest)
                            }
                            _ => None,
                        });

                let mut properties = binding
                    .properties()
                    .into_iter()
                    .filter_map(|member| member.ok())
                    .filter(|member| {
                        !matches!(
                            member,
                            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(_)
                        )
                    })
                    .collect::<Vec<_>>();

                // Transfer the trailing trivia of the last property to the new `ref` property.
                if rest.is_none()
                    && let Some(p) = properties.last_mut()
                    && let Some(trivia) = p.syntax().last_trailing_trivia()
                {
                    new_ref = new_ref.with_trailing_trivia_pieces(trivia.pieces())?;
                    *p = p.clone().with_trailing_trivia_pieces([])?;
                }

                // Add the `ref` property just before the rest property.
                properties.push(new_ref);

                if let Some(rest) = rest {
                    properties.push(rest.into());
                }

                let mut separators = binding
                    .properties()
                    .separators()
                    .filter_map(|sep| sep.ok())
                    .collect::<Vec<_>>();

                separators.push(
                    separators
                        .last()
                        .cloned()
                        .unwrap_or_else(|| make::token_with_trailing_space(T![,])),
                );

                new_props = new_props.with_binding(
                    binding
                        .with_properties(make::js_object_binding_pattern_property_list(
                            properties, separators,
                        ))
                        .into(),
                )
            }

            // Not a valid function component? Nothing to do.
            _ => return None,
        }
    }

    // Add a type annotation to the function based on the type arguments provided for forwardRef.
    if let (Some(ref_type), Some(props_type)) = type_args
        && props.type_annotation().is_none()
    {
        let new_props_type: AnyTsType = match props_type {
            // If the props is annotated with an object type, try to add a new property for the ref.
            AnyTsType::TsObjectType(ty) => {
                let mut members = ty.members().into_iter().collect::<Vec<_>>();
                let mut trailing_trivia = None;

                if let Some(AnyTsTypeMember::TsPropertySignatureTypeMember(mut member)) =
                    members.pop()
                {
                    // Detach the trailing trivia of the last property to be moved later.
                    trailing_trivia = member.syntax().last_trailing_trivia();
                    member = member.with_trailing_trivia_pieces([])?;

                    if member.separator_token().is_none() {
                        member = member.with_separator_token_token(Some(
                            make::token_with_trailing_space(T![,]),
                        ));
                    }

                    members.push(member.into());
                }

                let mut new_member = make_ref_property_member(ref_type, global_react_import)?;
                if let Some(trivia) = trailing_trivia {
                    // Attach the removed trailing trivia above.
                    new_member = new_member.with_trailing_trivia_pieces(trivia.pieces())?;
                }

                members.push(new_member.into());

                ty.with_members(make::ts_type_member_list(members)).into()
            }

            // Otherwise, create an intersection type with the props type and the ref type.
            ty => make::ts_intersection_type(make::ts_intersection_type_element_list(
                [
                    ty,
                    make::ts_object_type(
                        make::token_with_trailing_space(T!['{']),
                        make::ts_type_member_list([make_ref_property_member(
                            ref_type,
                            global_react_import,
                        )?
                        .into()]),
                        make::token_with_leading_space(T!['}']),
                    )
                    .into(),
                ],
                [make::token_decorated_with_space(T![&])],
            ))
            .build()
            .into(),
        };

        new_props = new_props.with_type_annotation(Some(make::ts_type_annotation(
            make::token_with_trailing_space(T![:]),
            new_props_type,
        )));
    }

    Some(params.with_items(make::js_parameter_list(
        [AnyJsParameter::AnyJsFormalParameter(new_props.into())],
        [],
    )))
}

/// Make a property type member for the `ref` property with a type annotation.
fn make_ref_property_member(
    ty: AnyTsType,
    global_react_import: Option<JsIdentifierBinding>,
) -> Option<TsPropertySignatureTypeMember> {
    Some(
        make::ts_property_signature_type_member(
            make::js_literal_member_name(make::ident("ref")).into(),
        )
        .with_optional_token(make::token(T![?]))
        .with_type_annotation(make::ts_type_annotation(
            make::token_with_trailing_space(T![:]),
            make_ref_object_type(ty, global_react_import)?.into(),
        ))
        .build(),
    )
}

/// Make a `React.RefObject<T | null>` type where `T` is the specified type.
fn make_ref_object_type(
    ty: AnyTsType,
    global_react_import: Option<JsIdentifierBinding>,
) -> Option<TsReferenceType> {
    // If `React` is imported globally, prefer using it.
    let reference_type = if let Some(binding) = global_react_import {
        let react = binding
            .name_token()
            .ok()?
            .with_leading_trivia_pieces([])
            .with_trailing_trivia_pieces([]);

        make::ts_qualified_name(
            make::js_reference_identifier(react).into(),
            make::token(T![.]),
            make::js_name(make::ident("RefObject")),
        )
        .into()
    } else {
        // TODO: Automatically adding the symbol to the import would be nice
        make::js_reference_identifier(make::ident("RefObject")).into()
    };

    Some(
        make::ts_reference_type(reference_type)
            .with_type_arguments(make::ts_type_arguments(
                make::token(T![<]),
                make::ts_type_argument_list(
                    [make::ts_union_type(make::ts_union_type_variant_list(
                        [ty, make::ts_null_literal_type(make::token(T![null])).into()],
                        [make::token_decorated_with_space(T![|])],
                    ))
                    .build()
                    .into()],
                    [],
                ),
                make::token(T![>]),
            ))
            .build(),
    )
}
