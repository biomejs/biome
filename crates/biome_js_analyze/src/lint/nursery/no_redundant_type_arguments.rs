use crate::{JsRuleAction, services::typed::Typed, utils::is_node_equal};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::{Binding, BindingExtensions, SemanticModel};
use biome_js_syntax::{
    AnyJsClass, AnyJsConstructorParameter, AnyJsExpression, AnyJsFunction, AnyJsParameter,
    AnyTsName, AnyTsType, JsCallExpression, JsExtendsClause, JsLanguage, JsNewExpression,
    JsTemplateExpression, JsxOpeningElement, JsxSelfClosingElement, TsDeclareFunctionDeclaration,
    TsDeclareFunctionExportDefaultDeclaration, TsInterfaceDeclaration, TsReferenceType,
    TsTypeAliasDeclaration, TsTypeArguments, TsTypeParameter, TsTypeParameters, inner_string_text,
};
use biome_js_type_info::{
    GenericTypeParameter, Literal, Path, Type, TypeData, TypeMemberKind, TypeReference,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutation, BatchMutationExt, SyntaxResult,
    TextRange, declare_node_union,
};
use biome_rule_options::no_redundant_type_arguments::NoRedundantTypeArgumentsOptions;

declare_lint_rule! {
    /// Disallow explicit type arguments that match a declaration's default type or can be inferred.
    ///
    /// Explicit type arguments make a call or type reference harder to read when they repeat what
    /// TypeScript already knows. This rule reports explicit type arguments when the trailing type
    /// argument is identical to the corresponding default type parameter, or when a direct function
    /// or constructor parameter already provides enough information to infer it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-default-call.ts
    /// function f<T = number>() {}
    /// f<number>();
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-inferred-call.ts
    /// function f<T>(x: T) {}
    /// f<number>(10);
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-reference-type.ts
    /// type Box<T = string> = T;
    /// type Value = Box<string>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid-default-call.ts
    /// function f<T = number>() {}
    /// f<string>();
    /// ```
    ///
    /// ```ts,file=valid-inferred-call.ts
    /// function f<T>(x: T) {}
    /// declare const value: any;
    /// f<string>(value);
    /// ```
    ///
    /// ```ts,file=valid-reference-type.ts
    /// type Box<T = string> = T;
    /// type Value = Box<number>;
    /// ```
    pub NoRedundantTypeArguments {
        version: "next",
        name: "noRedundantTypeArguments",
        language: "js",
        recommended: true,
        fix_kind: FixKind::Unsafe,
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-type-arguments").same()],
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    pub AnyNoRedundantTypeArgumentsQuery =
        JsCallExpression
        | JsNewExpression
        | JsTemplateExpression
        | JsExtendsClause
        | TsReferenceType
        | JsxOpeningElement
        | JsxSelfClosingElement
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RedundantTypeArgumentReason {
    CanBeInferred,
    MatchesDefault,
}

#[derive(Clone, Copy, Debug)]
pub struct NoRedundantTypeArgumentsState {
    range: TextRange,
    reason: RedundantTypeArgumentReason,
}

impl Rule for NoRedundantTypeArguments {
    type Query = Typed<AnyNoRedundantTypeArgumentsQuery>;
    type State = NoRedundantTypeArgumentsState;
    type Signals = Option<Self::State>;
    type Options = NoRedundantTypeArgumentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().run(ctx)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = match state.reason {
            RedundantTypeArgumentReason::CanBeInferred => markup! {
                "This explicit type argument can be inferred from the corresponding argument."
            },
            RedundantTypeArgumentReason::MatchesDefault => markup! {
                "This explicit type argument matches the corresponding default type parameter."
            },
        };

        let note = match state.reason {
            RedundantTypeArgumentReason::CanBeInferred => markup! {
                "It's often preferable to let TypeScript infer the type argument to improve code readability and prevent extra churn when refactoring surrounding code."
            },
            RedundantTypeArgumentReason::MatchesDefault => markup! {
                "Since the explicit type argument is the same as the default, it can be safely removed without changing the behavior of the code. Leaving it in is redundant and harms readability."
            },
        };

        Some(RuleDiagnostic::new(rule_category!(), state.range, message).note(note))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        ctx.query().action(&mut mutation)?;

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the redundant explicit type argument." }.to_owned(),
            mutation,
        ))
    }
}

impl AnyNoRedundantTypeArgumentsQuery {
    fn run(
        &self,
        ctx: &RuleContext<NoRedundantTypeArguments>,
    ) -> Option<NoRedundantTypeArgumentsState> {
        match self {
            Self::JsCallExpression(node) => run_call_expression(ctx, node),
            Self::JsNewExpression(node) => run_new_expression(ctx, node),
            Self::JsTemplateExpression(node) => run_template_expression(ctx, node),
            Self::JsExtendsClause(node) => run_extends_clause(ctx, node),
            Self::TsReferenceType(node) => run_reference_type(ctx, node),
            Self::JsxOpeningElement(node) => run_jsx_opening_element(ctx, node),
            Self::JsxSelfClosingElement(node) => run_jsx_self_closing_element(ctx, node),
        }
    }

    fn action(&self, mutation: &mut BatchMutation<JsLanguage>) -> Option<()> {
        match self {
            Self::JsCallExpression(node) => {
                mutation.replace_node(node.clone(), remove_last_type_argument_from_call(node)?)
            }
            Self::JsNewExpression(node) => {
                mutation.replace_node(node.clone(), remove_last_type_argument_from_new(node)?)
            }
            Self::JsTemplateExpression(node) => {
                mutation.replace_node(node.clone(), remove_last_type_argument_from_template(node)?)
            }
            Self::JsExtendsClause(node) => {
                mutation.replace_node(node.clone(), remove_last_type_argument_from_extends(node)?)
            }
            Self::TsReferenceType(node) => mutation.replace_node(
                node.clone(),
                remove_last_type_argument_from_reference(node)?,
            ),
            Self::JsxOpeningElement(node) => mutation.replace_node(
                node.clone(),
                remove_last_type_argument_from_jsx_opening(node)?,
            ),
            Self::JsxSelfClosingElement(node) => mutation.replace_node(
                node.clone(),
                remove_last_type_argument_from_jsx_self_closing(node)?,
            ),
        }
        Some(())
    }
}

/// Checks call expressions such as `f<number>(10)`.
///
/// ```ts
/// function f<T>(value: T) {}
/// f<number>(10);
/// ```
fn run_call_expression(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    node: &JsCallExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let type_arguments = node.type_arguments()?;
    let callee = node.callee().ok()?;
    let callee_type = ctx.type_of_expression(&callee);

    default_for_function_type(&callee_type, &type_arguments)
        .or_else(|| inferred_from_call_arguments(ctx, &callee_type, &type_arguments, node))
        .or_else(|| inferred_from_local_call_syntax(ctx, &type_arguments, node))
        .or_else(|| inferred_from_local_call_binding(ctx, &type_arguments, node))
}

/// Checks constructor calls such as `new Box<number>(10)`.
///
/// ```ts
/// class Box<T> { constructor(value: T) {} }
/// new Box<number>(10);
/// ```
fn run_new_expression(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    node: &JsNewExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let type_arguments = node.type_arguments()?;
    let callee = node.callee().ok()?;
    let callee_type = ctx.type_of_expression(&callee);

    default_for_class_type(&callee_type, &type_arguments)
        .or_else(|| inferred_from_new_arguments(ctx, &callee_type, &type_arguments, node))
        .or_else(|| inferred_from_local_new_syntax(ctx, &type_arguments, node))
        .or_else(|| inferred_from_local_new_binding(ctx, &type_arguments, node))
}

/// Checks tagged template calls such as `tag<number>`${1}``.
///
/// ```ts
/// function tag<T = number>(template: TemplateStringsArray, value: T) {}
/// tag<number>`${1}`;
/// ```
fn run_template_expression(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    node: &JsTemplateExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let type_arguments = node.type_arguments()?;
    let tag = node.tag()?;
    let tag_type = ctx.type_of_expression(&tag);

    default_for_function_type(&tag_type, &type_arguments)
        .or_else(|| inferred_from_template_arguments(ctx, &tag_type, &type_arguments, node))
        .or_else(|| inferred_from_local_template_syntax(ctx, &type_arguments, node))
        .or_else(|| inferred_from_local_template_binding(ctx, &type_arguments, node))
}

/// Checks `extends` clauses such as `class Derived extends Base<string> {}`.
///
/// ```ts
/// class Base<T = string> {}
/// class Derived extends Base<string> {}
/// ```
fn run_extends_clause(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    node: &JsExtendsClause,
) -> Option<NoRedundantTypeArgumentsState> {
    let name = node
        .super_class()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?;
    let type_arguments = node.type_arguments()?;

    if let Some(model) = ctx.get_service::<SemanticModel>()
        && let Some(binding) = name.binding(model)
    {
        default_from_binding_type_parameters(&binding, &type_arguments)
    } else {
        default_from_local_named_type_parameters(
            ctx,
            name.syntax(),
            name.value_token().ok()?.text_trimmed(),
            &type_arguments,
        )
    }
}

/// Checks type references such as `type Value = Box<string>`.
///
/// ```ts
/// type Box<T = string> = T;
/// type Value = Box<string>;
/// ```
fn run_reference_type(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    node: &TsReferenceType,
) -> Option<NoRedundantTypeArgumentsState> {
    let AnyTsName::JsReferenceIdentifier(reference) = node.name().ok()? else {
        return None;
    };

    let type_arguments = node.type_arguments()?;
    if let Some(model) = ctx.get_service::<SemanticModel>()
        && let Some(binding) = reference.binding(model)
    {
        default_from_binding_type_parameters(&binding, &type_arguments)
    } else {
        default_from_local_named_type_parameters(
            ctx,
            reference.syntax(),
            reference.value_token().ok()?.text_trimmed(),
            &type_arguments,
        )
    }
}

/// Checks JSX opening tags such as `<Button<string>>`.
///
/// ```tsx
/// function Button<T = string>() { return <div />; }
/// const value = <Button<string>></Button>;
/// ```
fn run_jsx_opening_element(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    node: &JsxOpeningElement,
) -> Option<NoRedundantTypeArgumentsState> {
    let element_name = node.name().ok()?;
    let name = element_name.as_jsx_reference_identifier()?;
    let type_arguments = node.type_arguments()?;

    if let Some(model) = ctx.get_service::<SemanticModel>()
        && let Some(binding) = name.binding(model)
    {
        default_from_binding_type_parameters(&binding, &type_arguments)
    } else {
        default_from_local_named_type_parameters(
            ctx,
            name.syntax(),
            name.value_token().ok()?.text_trimmed(),
            &type_arguments,
        )
    }
}

/// Checks self-closing JSX tags such as `<Button<string> />`.
///
/// ```tsx
/// function Button<T = string>() { return <div />; }
/// const value = <Button<string> />;
/// ```
fn run_jsx_self_closing_element(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    node: &JsxSelfClosingElement,
) -> Option<NoRedundantTypeArgumentsState> {
    let element_name = node.name().ok()?;
    let name = element_name.as_jsx_reference_identifier()?;
    let type_arguments = node.type_arguments()?;

    if let Some(model) = ctx.get_service::<SemanticModel>()
        && let Some(binding) = name.binding(model)
    {
        default_from_binding_type_parameters(&binding, &type_arguments)
    } else {
        default_from_local_named_type_parameters(
            ctx,
            name.syntax(),
            name.value_token().ok()?.text_trimmed(),
            &type_arguments,
        )
    }
}

/// Reports a trailing type argument when it matches a function type parameter default.
///
/// Example: `f<number>()` when `function f<T = number>() {}`.
fn default_for_function_type(
    function_type: &Type,
    type_arguments: &TsTypeArguments,
) -> Option<NoRedundantTypeArgumentsState> {
    let function = function_type.as_function()?;
    default_from_generic_type_parameters(function_type, &function.type_parameters, type_arguments)
}

/// Reports a trailing type argument when it matches a class type parameter default.
///
/// Example: `new Box<number>()` when `class Box<T = number> {}`.
fn default_for_class_type(
    class_type: &Type,
    type_arguments: &TsTypeArguments,
) -> Option<NoRedundantTypeArgumentsState> {
    let class = class_type.as_class()?;
    default_from_generic_type_parameters(class_type, &class.type_parameters, type_arguments)
}

/// Compares the last explicit type argument against resolved generic defaults from type info.
///
/// This handles cases where the declaration is read from typed resolution rather than syntax,
/// such as `f<number>()` or `new Box<number>()`.
///
/// ```ts
/// function f<T = number>() {}
/// f<number>();
/// ```
fn default_from_generic_type_parameters(
    owner_type: &Type,
    generic_parameters: &[biome_js_type_info::TypeReference],
    type_arguments: &TsTypeArguments,
) -> Option<NoRedundantTypeArgumentsState> {
    let (index, explicit_argument) = last_type_argument(type_arguments)?;
    let generic = resolved_generic_parameter(owner_type, generic_parameters, index)?;

    if generic.default.is_unknown() {
        return None;
    }

    let default_type = owner_type.resolve(&generic.default)?;
    if !semantic_type_matches_explicit_argument(&default_type, &explicit_argument) {
        return None;
    }

    Some(NoRedundantTypeArgumentsState {
        range: explicit_argument.range(),
        reason: RedundantTypeArgumentReason::MatchesDefault,
    })
}

/// Compares the last explicit type argument against syntactic type parameter defaults.
///
/// This is used for syntax-only contexts such as `extends Base<string>` and `type Value = Box<string>`.
///
/// ```ts
/// type Box<T = string> = T;
/// type Value = Box<string>;
/// ```
fn default_from_binding_type_parameters(
    binding: &Binding,
    type_arguments: &TsTypeArguments,
) -> Option<NoRedundantTypeArgumentsState> {
    let type_parameters = type_parameters_from_binding(binding)?;
    let (index, explicit_argument) = last_type_argument(type_arguments)?;
    let parameter = type_parameter_at(&type_parameters, index)?;
    let default_type = parameter.default()?.ty().ok()?;

    if !is_node_equal(
        explicit_argument.clone().omit_parentheses().syntax(),
        default_type.clone().omit_parentheses().syntax(),
    ) {
        return None;
    }

    Some(NoRedundantTypeArgumentsState {
        range: explicit_argument.range(),
        reason: RedundantTypeArgumentReason::MatchesDefault,
    })
}

/// Detects inference cases from resolved function parameter types.
///
/// ```ts
/// function f<T>(value: T) {}
/// f<number>(10);
/// ```
fn inferred_from_call_arguments(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    function_type: &Type,
    type_arguments: &TsTypeArguments,
    node: &JsCallExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let function = function_type.as_function()?;
    let (index, explicit_argument) = last_type_argument(type_arguments)?;
    let generic = resolved_generic_parameter(function_type, &function.type_parameters, index)?;
    let parameter_index =
        parameter_index_for_generic(function_type, &function.parameters, &generic)?;

    let argument = node
        .arguments()
        .ok()?
        .args()
        .iter()
        .nth(parameter_index)?
        .ok()?;
    let argument = argument.as_any_js_expression()?;
    let argument_type = ctx.type_of_expression(argument);

    inferred_type_matches_explicit_argument(&argument_type, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Detects inference cases from resolved constructor parameter types.
///
/// ```ts
/// class Box<T> { constructor(value: T) {} }
/// new Box<number>(10);
/// ```
fn inferred_from_new_arguments(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    class_type: &Type,
    type_arguments: &TsTypeArguments,
    node: &JsNewExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let class = class_type.as_class()?;
    let constructor = class
        .members
        .iter()
        .find(|member| member.kind == TypeMemberKind::Constructor)?;
    let constructor_type = class_type.resolve(&constructor.ty)?;
    let constructor_type_data = constructor_type.as_function()?;

    let (index, explicit_argument) = last_type_argument(type_arguments)?;
    let generic = resolved_generic_parameter(class_type, &class.type_parameters, index)?;
    let parameter_index = parameter_index_for_generic(
        &constructor_type,
        &constructor_type_data.parameters,
        &generic,
    )?;

    let arguments = node.arguments()?;
    let argument = arguments.args().iter().nth(parameter_index)?.ok()?;
    let argument = argument.as_any_js_expression()?;
    let argument_type = ctx.type_of_expression(argument);

    inferred_type_matches_explicit_argument(&argument_type, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Detects inference cases for tagged templates from resolved parameter types.
///
/// ```ts
/// function tag<T>(template: TemplateStringsArray, value: T) {}
/// tag<number>`${1}`;
/// ```
fn inferred_from_template_arguments(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    function_type: &Type,
    type_arguments: &TsTypeArguments,
    node: &JsTemplateExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let function = function_type.as_function()?;
    let (index, explicit_argument) = last_type_argument(type_arguments)?;
    let generic = resolved_generic_parameter(function_type, &function.type_parameters, index)?;
    let parameter_index =
        parameter_index_for_generic(function_type, &function.parameters, &generic)?;
    if parameter_index == 0 {
        return None;
    }

    let argument = node
        .elements()
        .iter()
        .nth(parameter_index - 1)?
        .as_js_template_element()?
        .expression()
        .ok()?;
    let argument_type = ctx.type_of_expression(&argument);

    inferred_type_matches_explicit_argument(&argument_type, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Falls back to local syntax when typed resolution cannot directly connect a call argument to a generic.
///
/// Example: `f<number>(10)` with a locally declared `function f<T>(value: T) {}`.
fn inferred_from_local_call_binding(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    type_arguments: &TsTypeArguments,
    node: &JsCallExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let model = ctx.get_service::<SemanticModel>()?;
    let binding = node
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .binding(model)?;
    let parameter_index =
        local_parameter_index_for_explicit_type_argument(&binding, type_arguments)?;
    let explicit_argument = last_type_argument(type_arguments)?.1;
    let argument = node
        .arguments()
        .ok()?
        .args()
        .iter()
        .nth(parameter_index)?
        .ok()?;
    let argument = argument.as_any_js_expression()?;
    let argument_type = ctx.type_of_expression(argument);

    inferred_type_matches_explicit_argument(&argument_type, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Falls back to same-file syntax lookup when typed inference cannot resolve the callee symbol.
///
/// Example: `f<number>(10)` with `function f<T>(value: T) {}` declared in the same file.
fn inferred_from_local_call_syntax(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    type_arguments: &TsTypeArguments,
    node: &JsCallExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let name = node
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?;
    let function =
        local_function_by_name(ctx, name.syntax(), name.value_token().ok()?.text_trimmed())?;
    let parameter_index =
        parameter_index_for_local_function_type_argument(&function, type_arguments)?;
    let explicit_argument = last_type_argument(type_arguments)?.1;
    let argument = node
        .arguments()
        .ok()?
        .args()
        .iter()
        .nth(parameter_index)?
        .ok()?;
    let argument = argument.as_any_js_expression()?;

    expression_matches_explicit_argument(ctx, argument, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Falls back to local class syntax to infer constructor type arguments from constructor parameters.
///
/// Example: `new Box<number>(10)` with `constructor(value: T) {}`.
fn inferred_from_local_new_binding(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    type_arguments: &TsTypeArguments,
    node: &JsNewExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let model = ctx.get_service::<SemanticModel>()?;
    let binding = node
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .binding(model)?;
    let parameter_index =
        local_constructor_parameter_index_for_explicit_type_argument(&binding, type_arguments)?;
    let explicit_argument = last_type_argument(type_arguments)?.1;
    let arguments = node.arguments()?;
    let argument = arguments.args().iter().nth(parameter_index)?.ok()?;
    let argument = argument.as_any_js_expression()?;
    let argument_type = ctx.type_of_expression(argument);

    inferred_type_matches_explicit_argument(&argument_type, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Falls back to same-file class syntax when typed inference cannot resolve the constructor symbol.
///
/// Example: `new Box<number>(10)` with `class Box<T> { constructor(value: T) {} }`.
fn inferred_from_local_new_syntax(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    type_arguments: &TsTypeArguments,
    node: &JsNewExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let name = node
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?;
    let class = local_class_by_name(ctx, name.syntax(), name.value_token().ok()?.text_trimmed())?;
    let parameter_index = parameter_index_for_local_class_type_argument(&class, type_arguments)?;
    let explicit_argument = last_type_argument(type_arguments)?.1;
    let argument = node.arguments()?.args().iter().nth(parameter_index)?.ok()?;
    let argument = argument.as_any_js_expression()?;

    expression_matches_explicit_argument(ctx, argument, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Falls back to local tagged-template syntax to infer generic arguments from template expressions.
///
/// Example: `tag<number>`${1}`` with `function tag<T>(template, value: T) {}`.
fn inferred_from_local_template_binding(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    type_arguments: &TsTypeArguments,
    node: &JsTemplateExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let model = ctx.get_service::<SemanticModel>()?;
    let binding = node
        .tag()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .binding(model)?;
    let parameter_index =
        local_parameter_index_for_explicit_type_argument(&binding, type_arguments)?;
    if parameter_index == 0 {
        return None;
    }

    let explicit_argument = last_type_argument(type_arguments)?.1;
    let argument = node
        .elements()
        .iter()
        .nth(parameter_index - 1)?
        .as_js_template_element()?
        .expression()
        .ok()?;
    let argument_type = ctx.type_of_expression(&argument);

    inferred_type_matches_explicit_argument(&argument_type, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Falls back to same-file tagged-template syntax when typed inference cannot resolve the tag symbol.
///
/// Example: `tag<number>`${1}`` with `function tag<T>(template, value: T) {}`.
fn inferred_from_local_template_syntax(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    type_arguments: &TsTypeArguments,
    node: &JsTemplateExpression,
) -> Option<NoRedundantTypeArgumentsState> {
    let name = node.tag()?.as_js_identifier_expression()?.name().ok()?;
    let function =
        local_function_by_name(ctx, name.syntax(), name.value_token().ok()?.text_trimmed())?;
    let parameter_index =
        parameter_index_for_local_function_type_argument(&function, type_arguments)?;
    if parameter_index == 0 {
        return None;
    }

    let explicit_argument = last_type_argument(type_arguments)?.1;
    let argument = node
        .elements()
        .iter()
        .nth(parameter_index - 1)?
        .as_js_template_element()?
        .expression()
        .ok()?;

    expression_matches_explicit_argument(ctx, &argument, &explicit_argument).then_some(
        NoRedundantTypeArgumentsState {
            range: explicit_argument.range(),
            reason: RedundantTypeArgumentReason::CanBeInferred,
        },
    )
}

/// Resolves the generic type parameter at `index` into owned type metadata.
///
/// This turns the trailing argument in `f<number>(10)` into the corresponding `T` metadata.
fn resolved_generic_parameter(
    owner_type: &Type,
    generic_parameters: &[biome_js_type_info::TypeReference],
    index: usize,
) -> Option<GenericTypeParameter> {
    let generic = owner_type.resolve(generic_parameters.get(index)?)?;
    match &*generic {
        TypeData::Generic(generic) => Some(generic.as_ref().clone()),
        _ => None,
    }
}

/// Finds the resolved parameter position whose type is the same generic parameter.
///
/// For `function f<T, U>(first: T, second: U) {}`, `U` maps to index `1`.
fn parameter_index_for_generic(
    owner_type: &Type,
    parameters: &[biome_js_type_info::FunctionParameter],
    generic: &GenericTypeParameter,
) -> Option<usize> {
    parameters.iter().position(|parameter| {
        owner_type
            .resolve(parameter.ty())
            .and_then(|resolved| match &*resolved {
                TypeData::Generic(resolved_generic) => Some(resolved_generic.name == generic.name),
                _ => None,
            })
            .unwrap_or(false)
    })
}

/// Returns the trailing explicit type argument, since the rule only removes one trailing argument per pass.
///
/// Example: in `f<number, string>()`, this returns the `string` argument at index `1`.
fn last_type_argument(type_arguments: &TsTypeArguments) -> Option<(usize, AnyTsType)> {
    let arguments = type_arguments.ts_type_argument_list();
    let index = arguments.len().checked_sub(1)?;
    let argument = arguments.iter().nth(index)?.ok()?;
    Some((index, argument))
}

/// Walks ancestors from a semantic binding to find the declaration's type parameter list.
///
/// This covers declarations such as `function f<T>() {}`, `class Box<T> {}`, and `type Box<T> = T`.
fn type_parameters_from_binding(binding: &Binding) -> Option<TsTypeParameters> {
    let syntax = binding.syntax();

    for ancestor in syntax.ancestors() {
        if let Some(function) = AnyJsFunction::cast_ref(&ancestor) {
            return function.type_parameters();
        }
        if let Some(class) = AnyJsClass::cast_ref(&ancestor) {
            return class.type_parameters();
        }
        if let Some(declaration) = TsDeclareFunctionDeclaration::cast_ref(&ancestor) {
            return declaration.type_parameters();
        }
        if let Some(declaration) = TsDeclareFunctionExportDefaultDeclaration::cast_ref(&ancestor) {
            return declaration.type_parameters();
        }
        if let Some(interface) = TsInterfaceDeclaration::cast_ref(&ancestor) {
            return interface.type_parameters();
        }
        if let Some(alias) = TsTypeAliasDeclaration::cast_ref(&ancestor) {
            return alias.type_parameters();
        }
    }

    None
}

/// Returns the type parameter at a given index.
///
/// In `<T, U = string>`, index `1` returns `U = string`.
fn type_parameter_at(type_parameters: &TsTypeParameters, index: usize) -> Option<TsTypeParameter> {
    type_parameters.items().iter().nth(index)?.ok()
}

/// Finds the local function parameter position that corresponds to the trailing explicit type argument.
///
/// For `function f<T, U>(first: T, second: U) {}`, the trailing argument in `f<number, string>()`
/// maps to the `second` parameter.
fn local_parameter_index_for_explicit_type_argument(
    binding: &Binding,
    type_arguments: &TsTypeArguments,
) -> Option<usize> {
    let type_parameters = type_parameters_from_binding(binding)?;
    let (index, _) = last_type_argument(type_arguments)?;
    let type_parameter_name = type_parameter_at(&type_parameters, index)?
        .name()
        .ok()?
        .ident_token()
        .ok()?;
    let type_parameter_name = type_parameter_name.text_trimmed();

    let syntax = binding.syntax();
    syntax.ancestors().find_map(|ancestor| {
        let function = AnyJsFunction::cast(ancestor)?;
        parameter_index_in_function(&function, type_parameter_name)
    })
}

/// Finds the local constructor parameter position that corresponds to the trailing explicit type argument.
///
/// For `class Box<T> { constructor(value: T) {} }`, the trailing argument in `new Box<number>()`
/// maps to `value`.
fn local_constructor_parameter_index_for_explicit_type_argument(
    binding: &Binding,
    type_arguments: &TsTypeArguments,
) -> Option<usize> {
    let type_parameters = type_parameters_from_binding(binding)?;
    let (index, _) = last_type_argument(type_arguments)?;
    let type_parameter_name = type_parameter_at(&type_parameters, index)?
        .name()
        .ok()?
        .ident_token()
        .ok()?;
    let type_parameter_name = type_parameter_name.text_trimmed();

    let syntax = binding.syntax();
    let class = syntax.ancestors().find_map(AnyJsClass::cast)?;
    class.members().iter().find_map(|member| {
        let constructor = member.as_js_constructor_class_member()?;
        parameter_index_in_constructor(constructor, type_parameter_name)
    })
}

/// Resolves local same-file declarations by name and compares their default type parameter.
///
/// Example: `type Box<T = string> = T; type Value = Box<string>;`.
fn default_from_local_named_type_parameters(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    syntax: &biome_js_syntax::JsSyntaxNode,
    name: &str,
    type_arguments: &TsTypeArguments,
) -> Option<NoRedundantTypeArgumentsState> {
    let type_parameters = local_type_parameters_by_name(ctx, syntax, name)?;
    let (index, explicit_argument) = last_type_argument(type_arguments)?;
    let parameter = type_parameter_at(&type_parameters, index)?;
    let default_type = parameter.default()?.ty().ok()?;

    if !{
        let left: &AnyTsType = &explicit_argument;
        let right: &AnyTsType = &default_type;
        let left = left.clone().omit_parentheses();
        let right = right.clone().omit_parentheses();

        is_node_equal(left.syntax(), right.syntax())
    } {
        return None;
    }

    Some(NoRedundantTypeArgumentsState {
        range: explicit_argument.range(),
        reason: RedundantTypeArgumentReason::MatchesDefault,
    })
}

/// Looks up a same-file function declaration by identifier text using the semantic model.
///
/// ```ts
/// function f<T>(value: T) {}
/// f<number>(10);
/// ```
fn local_function_by_name(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    syntax: &biome_js_syntax::JsSyntaxNode,
    name: &str,
) -> Option<AnyJsFunction> {
    let binding = local_binding_by_name(ctx, syntax, name)?;
    binding.syntax().ancestors().find_map(AnyJsFunction::cast)
}

/// Looks up a same-file class declaration by identifier text using the semantic model.
///
/// ```ts
/// class Box<T> {
///     constructor(value: T) {}
/// }
/// new Box<number>(10);
/// ```
fn local_class_by_name(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    syntax: &biome_js_syntax::JsSyntaxNode,
    name: &str,
) -> Option<AnyJsClass> {
    let binding = local_binding_by_name(ctx, syntax, name)?;
    binding.syntax().ancestors().find_map(AnyJsClass::cast)
}

/// Looks up same-file declarations with type parameters by identifier text using the semantic model.
///
/// ```ts
/// interface Box<T = string> {}
/// type Value = Box<string>;
/// ```
fn local_type_parameters_by_name(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    syntax: &biome_js_syntax::JsSyntaxNode,
    name: &str,
) -> Option<TsTypeParameters> {
    let binding = local_binding_by_name(ctx, syntax, name)?;
    type_parameters_from_binding(&binding)
}

/// Finds the parameter index for the trailing explicit type argument in a same-file function.
///
/// ```ts
/// function f<T, U>(first: T, second: U) {}
/// f<number, string>(1, "x");
/// ```
fn parameter_index_for_local_function_type_argument(
    function: &AnyJsFunction,
    type_arguments: &TsTypeArguments,
) -> Option<usize> {
    let type_parameters = function.type_parameters()?;
    let (index, _) = last_type_argument(type_arguments)?;
    let type_parameter_name = type_parameter_at(&type_parameters, index)?
        .name()
        .ok()?
        .ident_token()
        .ok()?;
    parameter_index_in_function(function, type_parameter_name.text_trimmed())
}

/// Finds the constructor parameter index for the trailing explicit type argument in a same-file class.
///
/// ```ts
/// class Box<T> {
///     constructor(value: T) {}
/// }
/// new Box<number>(10);
/// ```
fn parameter_index_for_local_class_type_argument(
    class: &AnyJsClass,
    type_arguments: &TsTypeArguments,
) -> Option<usize> {
    let type_parameters = class.type_parameters()?;
    let (index, _) = last_type_argument(type_arguments)?;
    let type_parameter_name = type_parameter_at(&type_parameters, index)?
        .name()
        .ok()?
        .ident_token()
        .ok()?;

    class.members().iter().find_map(|member| {
        let constructor = member.as_js_constructor_class_member()?;
        parameter_index_in_constructor(constructor, type_parameter_name.text_trimmed())
    })
}

/// Compares an argument expression against an explicit type argument using type info.
///
/// ```ts
/// function f<T>(value: T) {}
/// f<number>(10);
/// ```
fn expression_matches_explicit_argument(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    expression: &AnyJsExpression,
    explicit_argument: &AnyTsType,
) -> bool {
    let argument_type = ctx.type_of_expression(expression);
    inferred_type_matches_explicit_argument(&argument_type, explicit_argument)
        || literal_expression_matches_keyword_type(expression, explicit_argument)
}

/// Looks up the nearest visible binding for a name by walking lexical scopes.
///
/// ```ts
/// function outer() {
///     function inner<T>(value: T) {}
///     inner<number>(10);
/// }
/// ```
fn local_binding_by_name(
    ctx: &RuleContext<NoRedundantTypeArguments>,
    syntax: &biome_js_syntax::JsSyntaxNode,
    name: &str,
) -> Option<Binding> {
    let model = ctx.get_service::<SemanticModel>()?;
    model
        .scope(syntax)
        .ancestors()
        .find_map(|scope| scope.get_binding(name))
}

/// Returns `true` when a literal expression matches a primitive keyword type syntactically.
///
/// ```ts
/// function f<T>(value: T) {}
/// f<number>(10);
/// ```
fn literal_expression_matches_keyword_type(
    expression: &AnyJsExpression,
    explicit_argument: &AnyTsType,
) -> bool {
    let Some(literal) = expression.as_any_js_literal_expression() else {
        return false;
    };

    (explicit_argument.as_ts_number_type().is_some()
        && literal.as_js_number_literal_expression().is_some())
        || (explicit_argument.as_ts_string_type().is_some()
            && literal.as_js_string_literal_expression().is_some())
        || (explicit_argument.as_ts_boolean_type().is_some()
            && literal.as_js_boolean_literal_expression().is_some())
        || (explicit_argument.as_ts_bigint_type().is_some()
            && literal.as_js_bigint_literal_expression().is_some())
}

/// Searches a function signature for the first parameter annotated with the target type parameter name.
///
/// Example: in `function f<T>(value: T) {}`, this returns the index of `value`.
fn parameter_index_in_function(
    function: &AnyJsFunction,
    type_parameter_name: &str,
) -> Option<usize> {
    let parameters = function.parameters().ok()?;
    let parameters = parameters.as_js_parameters()?;
    parameters
        .items()
        .iter()
        .enumerate()
        .find_map(|(index, parameter)| {
            let parameter = parameter.ok()?;
            parameter_uses_type_parameter(&parameter, type_parameter_name)?.then_some(index)
        })
}

/// Searches a constructor signature for the first parameter annotated with the target type parameter name.
///
/// Example: in `constructor(value: T) {}`, this returns the index of `value`.
fn parameter_index_in_constructor(
    constructor: &biome_js_syntax::JsConstructorClassMember,
    type_parameter_name: &str,
) -> Option<usize> {
    constructor
        .parameters()
        .ok()?
        .parameters()
        .iter()
        .enumerate()
        .find_map(|(index, parameter)| {
            let parameter = parameter.ok()?;
            constructor_parameter_uses_type_parameter(&parameter, type_parameter_name)?
                .then_some(index)
        })
}

/// Returns `true` when a function parameter annotation directly references the requested type parameter.
///
/// Example: `value: T` matches `T`, while `value: number` does not.
fn parameter_uses_type_parameter(
    parameter: &AnyJsParameter,
    type_parameter_name: &str,
) -> Option<bool> {
    let type_annotation = match parameter {
        AnyJsParameter::AnyJsFormalParameter(formal_parameter) => formal_parameter
            .as_js_formal_parameter()
            .and_then(|formal_parameter| formal_parameter.type_annotation())
            .and_then(|type_annotation| type_annotation.ty().ok()),
        AnyJsParameter::JsRestParameter(rest_parameter) => rest_parameter
            .type_annotation()
            .and_then(|type_annotation| type_annotation.ty().ok()),
        AnyJsParameter::TsThisParameter(_) => None,
    }?;

    let reference_type = type_annotation.as_ts_reference_type()?;
    let AnyTsName::JsReferenceIdentifier(reference_identifier) = reference_type.name().ok()? else {
        return None;
    };

    Some(
        reference_identifier
            .value_token()
            .is_ok_and(|token| token.text_trimmed() == type_parameter_name),
    )
}

/// Returns `true` when a constructor parameter annotation directly references the requested type parameter.
///
/// This supports plain, rest, and property parameters such as `constructor(public value: T) {}`.
///
/// ```ts
/// class Box<T> {
///     constructor(public value: T) {}
/// }
/// ```
fn constructor_parameter_uses_type_parameter(
    parameter: &AnyJsConstructorParameter,
    type_parameter_name: &str,
) -> Option<bool> {
    let type_annotation = match parameter {
        AnyJsConstructorParameter::AnyJsFormalParameter(formal_parameter) => formal_parameter
            .as_js_formal_parameter()
            .and_then(|formal_parameter| formal_parameter.type_annotation())
            .and_then(|type_annotation| type_annotation.ty().ok()),
        AnyJsConstructorParameter::JsRestParameter(rest_parameter) => rest_parameter
            .type_annotation()
            .and_then(|type_annotation| type_annotation.ty().ok()),
        AnyJsConstructorParameter::TsPropertyParameter(property_parameter) => property_parameter
            .formal_parameter()
            .ok()
            .and_then(|formal_parameter| formal_parameter.type_annotation())
            .and_then(|type_annotation| type_annotation.ty().ok()),
    }?;

    let reference_type = type_annotation.as_ts_reference_type()?;
    let AnyTsName::JsReferenceIdentifier(reference_identifier) = reference_type.name().ok()? else {
        return None;
    };

    Some(
        reference_identifier
            .value_token()
            .is_ok_and(|token| token.text_trimmed() == type_parameter_name),
    )
}

/// Compares an inferred runtime type against an explicit type argument.
///
/// This accepts exact text matches and a few literal-to-primitive cases such as `10` matching `number`
/// and `"x"` matching `string`.
fn inferred_type_matches_explicit_argument(
    argument_type: &Type,
    explicit_argument: &AnyTsType,
) -> bool {
    if semantic_type_matches_explicit_argument(argument_type, explicit_argument) {
        return true;
    }

    let explicit_argument = explicit_argument.clone().omit_parentheses();

    if explicit_argument.as_ts_number_type().is_some() {
        return argument_type.is_number_or_number_literal();
    }

    if explicit_argument.as_ts_string_type().is_some() {
        return argument_type.is_string_or_string_literal();
    }

    if explicit_argument.as_ts_boolean_type().is_some() {
        return {
            matches!(&**argument_type, TypeData::Boolean)
                || matches!(&**argument_type, TypeData::Literal(literal) if matches!(literal.as_ref(), Literal::Boolean(_)))
        };
    }

    if explicit_argument.as_ts_bigint_type().is_some() {
        return {
            matches!(&**argument_type, TypeData::BigInt)
                || matches!(&**argument_type, TypeData::Literal(literal) if matches!(literal.as_ref(), Literal::BigInt(_)))
        };
    }

    false
}

/// Checks whether an explicit trailing type argument matches the semantic type selected by the
/// rule.
///
/// This is the final comparison used after the rule has determined which defaulted or inferred
/// type the trailing argument should match.
fn semantic_type_matches_explicit_argument(
    semantic_type: &Type,
    explicit_argument: &AnyTsType,
) -> bool {
    let explicit_argument = explicit_argument.clone().omit_parentheses();

    match &**semantic_type {
        TypeData::AnyKeyword => explicit_argument.as_ts_any_type().is_some(),
        TypeData::BigInt => explicit_argument.as_ts_bigint_type().is_some(),
        TypeData::Boolean => explicit_argument.as_ts_boolean_type().is_some(),
        TypeData::NeverKeyword => explicit_argument.as_ts_never_type().is_some(),
        TypeData::Null => explicit_argument.as_ts_null_literal_type().is_some(),
        TypeData::Number => explicit_argument.as_ts_number_type().is_some(),
        TypeData::ObjectKeyword => explicit_argument.as_ts_non_primitive_type().is_some(),
        TypeData::String => explicit_argument.as_ts_string_type().is_some(),
        TypeData::Symbol => explicit_argument.as_ts_symbol_type().is_some(),
        TypeData::ThisKeyword => explicit_argument.as_ts_this_type().is_some(),
        TypeData::Undefined => explicit_argument.as_ts_undefined_type().is_some(),
        TypeData::Unknown | TypeData::UnknownKeyword => {
            explicit_argument.as_ts_unknown_type().is_some()
        }
        TypeData::VoidKeyword => explicit_argument.as_ts_void_type().is_some(),
        TypeData::Literal(literal) => {
            literal_type_matches_explicit_argument(literal, &explicit_argument).unwrap_or(false)
        }
        TypeData::Generic(generic) => reference_name_matches_explicit_argument(
            explicit_argument.as_ts_reference_type(),
            [generic.name.text()],
        )
        .unwrap_or(false),
        TypeData::Reference(reference) => {
            type_reference_matches_explicit_argument(semantic_type, reference, &explicit_argument)
        }
        TypeData::Class(class) => reference_name_matches_explicit_argument(
            explicit_argument.as_ts_reference_type(),
            class.name.iter().map(|name| name.text()),
        )
        .unwrap_or(false),
        TypeData::Interface(interface) => reference_name_matches_explicit_argument(
            explicit_argument.as_ts_reference_type(),
            [interface.name.text()],
        )
        .unwrap_or(false),
        TypeData::Union(union) => type_reference_list_matches_explicit_arguments(
            semantic_type,
            union.types(),
            explicit_argument
                .as_ts_union_type()
                .map(|union| union.types().iter()),
        )
        .unwrap_or(false),
        TypeData::Intersection(intersection) => type_reference_list_matches_explicit_arguments(
            semantic_type,
            intersection.types(),
            explicit_argument
                .as_ts_intersection_type()
                .map(|intersection| intersection.types().iter()),
        )
        .unwrap_or(false),
        _ => false,
    }
}

/// Matches literal semantic types against explicit literal syntax such as `"x"`, `10`, or `true`.
///
/// The rule uses this to treat explicit literal type arguments as redundant when the inferred or
/// defaulted semantic type is the same literal value.
fn literal_type_matches_explicit_argument(
    literal: &Literal,
    explicit_argument: &AnyTsType,
) -> Option<bool> {
    Some(match literal {
        Literal::Boolean(value) => explicit_argument
            .as_ts_boolean_literal_type()
            .and_then(|literal| literal.literal().ok())
            .map(|token| token.text_trimmed() == if value.as_bool() { "true" } else { "false" })?,
        Literal::BigInt(value) => explicit_argument
            .as_ts_bigint_literal_type()
            .and_then(|literal| literal.literal_token().ok().map(|token| (literal, token)))
            .map(|(literal, token)| {
                let expected = value.text();
                if literal.minus_token().is_some() {
                    expected
                        .strip_prefix('-')
                        .is_some_and(|expected| token.text_trimmed() == expected)
                } else {
                    token.text_trimmed() == expected
                }
            })?,
        Literal::Number(value) => {
            let value = value.to_f64()?;

            explicit_argument
                .as_ts_number_literal_type()
                .and_then(number_literal_value)
                .map(|literal| literal == value)?
        }
        Literal::String(value) => explicit_argument
            .as_ts_string_literal_type()
            .and_then(|literal| literal.literal_token().ok())
            .map(|token| inner_string_text(&token) == value.as_str())?,
        _ => false,
    })
}

/// Checks whether a semantic reference type like `Promise<string>` matches an explicit reference
/// type argument with the same name and nested type arguments.
///
/// The rule uses this when the defaulted or inferred type is itself a named generic reference.
fn type_reference_matches_explicit_argument(
    semantic_type: &Type,
    type_reference: &TypeReference,
    explicit_argument: &AnyTsType,
) -> bool {
    let Some(reference_type) = explicit_argument.as_ts_reference_type() else {
        return false;
    };

    match type_reference {
        TypeReference::Qualifier(qualifier) => {
            let Ok(name) = reference_type.name() else {
                return false;
            };

            ts_name_matches_path(&name, &qualifier.path)
                && type_argument_lists_match(
                    semantic_type,
                    &qualifier.type_parameters,
                    reference_type.type_arguments(),
                )
        }
        TypeReference::Import(_) | TypeReference::Resolved(_) => false,
    }
}

/// Checks whether every member of a compound semantic type, such as a union or intersection,
/// matches the corresponding explicit type member.
///
/// The rule uses this when the trailing type argument is redundant only if the full compound type
/// matches branch-for-branch.
fn type_reference_list_matches_explicit_arguments<I>(
    semantic_type: &Type,
    expected_types: &[TypeReference],
    explicit_arguments: Option<I>,
) -> Option<bool>
where
    I: IntoIterator<Item = biome_rowan::SyntaxResult<AnyTsType>>,
{
    let explicit_arguments = explicit_arguments?
        .into_iter()
        .collect::<SyntaxResult<Vec<_>>>()
        .ok()?;

    Some(
        expected_types.len() == explicit_arguments.len()
            && expected_types
                .iter()
                .zip(explicit_arguments.iter())
                .all(|(expected, explicit)| {
                    semantic_type.resolve(expected).is_some_and(|resolved| {
                        semantic_type_matches_explicit_argument(&resolved, explicit)
                    })
                }),
    )
}

/// Matches the type arguments nested inside a semantic reference, for example the `<string>` in
/// `Promise<string>`.
///
/// This ensures the rule only reports a redundant explicit argument when both the outer reference
/// and each nested generic argument already match what type resolution produced.
fn type_argument_lists_match(
    semantic_type: &Type,
    expected_arguments: &[TypeReference],
    explicit_arguments: Option<TsTypeArguments>,
) -> bool {
    match explicit_arguments {
        Some(explicit_arguments) => {
            let explicit_arguments = explicit_arguments.ts_type_argument_list();
            expected_arguments.len() == explicit_arguments.len()
                && expected_arguments
                    .iter()
                    .zip(explicit_arguments.iter())
                    .all(|(expected, explicit)| {
                        let Ok(explicit) = explicit else {
                            return false;
                        };

                        semantic_type.resolve(expected).is_some_and(|resolved| {
                            semantic_type_matches_explicit_argument(&resolved, &explicit)
                        })
                    })
        }
        None => expected_arguments.is_empty(),
    }
}

/// Checks whether an explicit reference type names the expected symbol and has no nested type
/// arguments.
///
/// The rule uses this for cases where matching the referenced type name is enough to prove that a
/// trailing type argument repeats an existing generic, class, or interface type.
fn reference_name_matches_explicit_argument<'a>(
    explicit_argument: Option<&TsReferenceType>,
    expected_path: impl IntoIterator<Item = &'a str>,
) -> Option<bool> {
    let explicit_argument = explicit_argument?;
    let name = explicit_argument.name().ok()?;

    let mut expected_path = expected_path.into_iter();

    Some(
        explicit_argument.type_arguments().is_none()
            && ts_name_matches_parts(&name, &mut expected_path)?
            && expected_path.next().is_none(),
    )
}

/// Checks whether a type-name syntax node refers to the same symbol path as a semantic reference.
///
/// The rule uses this for qualified names such as `Namespace.Box` when comparing explicit type
/// arguments against named defaults or inferred reference types.
fn ts_name_matches_path(name: &AnyTsName, path: &Path) -> bool {
    let mut expected_parts = path.iter().map(|part| part.text());

    ts_name_matches_parts(name, &mut expected_parts).unwrap_or(false)
        && expected_parts.next().is_none()
}

/// Walks a type name from left to right and checks that each identifier matches the expected path.
///
/// The rule uses this to compare qualified names one segment at a time when deciding whether an
/// explicit type argument names the same type.
fn ts_name_matches_parts<'a, I>(name: &AnyTsName, expected_parts: &mut I) -> Option<bool>
where
    I: Iterator<Item = &'a str>,
{
    let part_count = ts_name_part_count(name)?;

    for index in 0..part_count {
        let expected_part = expected_parts.next()?;

        if !ts_name_part_matches(name, part_count, index, expected_part) {
            return Some(false);
        }
    }

    Some(true)
}

/// Counts how many identifiers participate in a type name such as `Namespace.Box.Value`.
///
/// The rule uses this to compare each segment of a qualified name in order.
fn ts_name_part_count(name: &AnyTsName) -> Option<usize> {
    let mut count = 0;
    let mut current = name.clone();

    loop {
        match &current {
            AnyTsName::JsReferenceIdentifier(_) => return Some(count + 1),
            AnyTsName::TsQualifiedName(qualified_name) => {
                count += 1;
                current = qualified_name.left().ok()?;
            }
        }
    }
}

/// Checks whether the identifier at `index` within a type name matches the expected text.
///
/// The rule uses this to compare qualified names segment-by-segment while keeping the traversal
/// order aligned with the semantic path.
fn ts_name_part_matches(name: &AnyTsName, part_count: usize, index: usize, expected: &str) -> bool {
    let mut remaining_from_right = part_count.checked_sub(index + 1);
    let mut current = name.clone();

    loop {
        match &current {
            AnyTsName::JsReferenceIdentifier(identifier) => {
                return remaining_from_right == Some(0)
                    && identifier
                        .value_token()
                        .is_ok_and(|token| token.text_trimmed() == expected);
            }
            AnyTsName::TsQualifiedName(qualified_name) => {
                if remaining_from_right == Some(0) {
                    return qualified_name
                        .right()
                        .ok()
                        .and_then(|right| right.value_token().ok())
                        .is_some_and(|token| token.text_trimmed() == expected);
                }

                let Some(remaining) = remaining_from_right else {
                    return false;
                };

                remaining_from_right = Some(remaining - 1);
                current = match qualified_name.left() {
                    Ok(left) => left,
                    Err(_) => return false,
                };
            }
        }
    }
}

/// Parses a numeric literal type node into a comparable numeric value.
///
/// The rule uses this to decide whether an explicit numeric literal type argument matches the
/// numeric literal type it inferred or resolved.
fn number_literal_value(number_literal: &biome_js_syntax::TsNumberLiteralType) -> Option<f64> {
    let literal = number_literal.literal_token().ok()?;
    let value = literal.text_trimmed().parse::<f64>().ok()?;

    Some(if number_literal.minus_token().is_some() {
        -value
    } else {
        value
    })
}

/// Builds a new type-argument list without the trailing explicit type argument.
///
/// For `f<number, string>()`, this produces the equivalent of `f<number>()`.
fn remove_last_type_argument(type_arguments: TsTypeArguments) -> Option<TsTypeArguments> {
    let arguments = type_arguments.ts_type_argument_list();
    if arguments.len() <= 1 {
        return None;
    }

    let kept_arguments = arguments
        .iter()
        .take(arguments.len() - 1)
        .collect::<SyntaxResult<Vec<_>>>()
        .ok()?;
    let kept_separators = arguments
        .separators()
        .take(arguments.len() - 2)
        .collect::<SyntaxResult<Vec<_>>>()
        .ok()?;

    Some(
        type_arguments.with_ts_type_argument_list(biome_js_factory::make::ts_type_argument_list(
            kept_arguments,
            kept_separators,
        )),
    )
}

/// Applies the trailing-type-argument removal fix to a call expression.
///
/// Example: `f<number>()` becomes `f()`.
fn remove_last_type_argument_from_call(node: &JsCallExpression) -> Option<JsCallExpression> {
    let type_arguments = node.type_arguments()?;
    Some(match type_arguments.ts_type_argument_list().len() {
        0 => return None,
        1 => node.clone().with_type_arguments(None),
        _ => node
            .clone()
            .with_type_arguments(Some(remove_last_type_argument(type_arguments)?)),
    })
}

/// Applies the trailing-type-argument removal fix to a `new` expression.
///
/// Example: `new Box<number>()` becomes `new Box()`.
fn remove_last_type_argument_from_new(node: &JsNewExpression) -> Option<JsNewExpression> {
    let type_arguments = node.type_arguments()?;
    Some(match type_arguments.ts_type_argument_list().len() {
        0 => return None,
        1 => node.clone().with_type_arguments(None),
        _ => node
            .clone()
            .with_type_arguments(Some(remove_last_type_argument(type_arguments)?)),
    })
}

/// Applies the trailing-type-argument removal fix to a tagged template.
///
/// Example: `tag<number>`${1}`` becomes `tag`${1}``.
fn remove_last_type_argument_from_template(
    node: &JsTemplateExpression,
) -> Option<JsTemplateExpression> {
    let type_arguments = node.type_arguments()?;
    Some(match type_arguments.ts_type_argument_list().len() {
        0 => return None,
        1 => node.clone().with_type_arguments(None),
        _ => node
            .clone()
            .with_type_arguments(Some(remove_last_type_argument(type_arguments)?)),
    })
}

/// Applies the trailing-type-argument removal fix to an `extends` clause.
///
/// Example: `class Derived extends Base<string> {}` becomes `class Derived extends Base {}`.
fn remove_last_type_argument_from_extends(node: &JsExtendsClause) -> Option<JsExtendsClause> {
    let type_arguments = node.type_arguments()?;
    Some(match type_arguments.ts_type_argument_list().len() {
        0 => return None,
        1 => node.clone().with_type_arguments(None),
        _ => node
            .clone()
            .with_type_arguments(Some(remove_last_type_argument(type_arguments)?)),
    })
}

/// Applies the trailing-type-argument removal fix to a type reference.
///
/// Example: `type Value = Box<string>` becomes `type Value = Box`.
fn remove_last_type_argument_from_reference(node: &TsReferenceType) -> Option<TsReferenceType> {
    let type_arguments = node.type_arguments()?;
    Some(match type_arguments.ts_type_argument_list().len() {
        0 => return None,
        1 => node.clone().with_type_arguments(None),
        _ => node
            .clone()
            .with_type_arguments(Some(remove_last_type_argument(type_arguments)?)),
    })
}

/// Applies the trailing-type-argument removal fix to an opening JSX tag.
///
/// Example: `<Button<string>>` becomes `<Button>`.
fn remove_last_type_argument_from_jsx_opening(
    node: &JsxOpeningElement,
) -> Option<JsxOpeningElement> {
    let type_arguments = node.type_arguments()?;
    Some(match type_arguments.ts_type_argument_list().len() {
        0 => return None,
        1 => node.clone().with_type_arguments(None),
        _ => node
            .clone()
            .with_type_arguments(Some(remove_last_type_argument(type_arguments)?)),
    })
}

/// Applies the trailing-type-argument removal fix to a self-closing JSX tag.
///
/// Example: `<Button<string> />` becomes `<Button />`.
fn remove_last_type_argument_from_jsx_self_closing(
    node: &JsxSelfClosingElement,
) -> Option<JsxSelfClosingElement> {
    let type_arguments = node.type_arguments()?;
    Some(match type_arguments.ts_type_argument_list().len() {
        0 => return None,
        1 => node.clone().with_type_arguments(None),
        _ => node
            .clone()
            .with_type_arguments(Some(remove_last_type_argument(type_arguments)?)),
    })
}
