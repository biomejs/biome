use crate::JsRuleAction;
use crate::services::typed::Typed;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFunction, AnyJsParameter,
    JsArrayBindingPattern, JsArrayBindingPatternElement, JsArrayBindingPatternElementList,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsFormalParameter,
    JsInitializerClause, JsNewExpression, JsObjectBindingPattern, JsObjectBindingPatternProperty,
    JsObjectBindingPatternPropertyList, JsObjectBindingPatternShorthandProperty,
    JsParameterList, JsParameters, JsVariableDeclarator,
};
use biome_js_type_info::InferredType;
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, chain_trivia_pieces, declare_node_union,
    trim_trailing_trivia_pieces,
};
use biome_rule_options::no_useless_default_assignment::NoUselessDefaultAssignmentOptions;

declare_lint_rule! {
    /// Disallow default values that can never be used.
    ///
    /// A default value only applies when the value it replaces is `undefined`.
    /// If the type says the value can't be `undefined`, the default is dead code.
    ///
    /// This rule checks destructured object properties, destructured tuple
    /// elements, and parameters of callbacks passed directly to a call.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-parameter-property.ts
    /// function greet({ name = "friend" }: { name: string }) {
    ///     return `Hello, ${name}`;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-variable-property.ts
    /// const { name = "friend" } = { name: "Ada" };
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-tuple-element.ts
    /// declare const pair: [string, number];
    /// const [label = "none", count] = pair;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-callback-parameter.ts
    /// declare function onMessage(listener: (message: string) => void): void;
    /// onMessage((message = "") => console.log(message));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function greet({ name = "friend" }: { name?: string }) {
    ///     return `Hello, ${name}`;
    /// }
    ///
    /// const { label = "none" } = { label: undefined };
    ///
    /// declare const names: string[];
    /// const [first = "nobody"] = names;
    ///
    /// declare function onMessage(listener: (message?: string) => void): void;
    /// onMessage((message = "") => console.log(message));
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to remove default values that are `undefined`, see [`noUselessUndefined`](https://biomejs.dev/linter/rules/no-useless-undefined/).
    ///
    pub NoUselessDefaultAssignment {
        version: "next",
        name: "noUselessDefaultAssignment",
        language: "ts",
        recommended: false,
        severity: Severity::Information,
        sources: &[RuleSource::EslintTypeScript("no-useless-default-assignment").inspired()],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    pub AnyJsBindingWithDefault =
        JsArrayBindingPatternElement
        | JsFormalParameter
        | JsObjectBindingPatternProperty
        | JsObjectBindingPatternShorthandProperty
}

/// The kind of value that a useless default value replaces.
#[derive(Clone, Copy)]
pub enum DefaultTarget {
    /// An array element in an array destructuring pattern.
    Element,
    /// A parameter of a function passed to a call.
    Parameter,
    /// A property in an object destructuring pattern.
    Property,
}

impl Rule for NoUselessDefaultAssignment {
    type Query = Typed<AnyJsBindingWithDefault>;
    type State = DefaultTarget;
    type Signals = Option<Self::State>;
    type Options = NoUselessDefaultAssignmentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let default_value = node.initializer()?.expression().ok()?;
        if default_value
            .as_js_reference_identifier()
            .is_some_and(|identifier| identifier.is_undefined())
        {
            return None;
        }

        let (target, value_type) = match node {
            AnyJsBindingWithDefault::JsObjectBindingPatternShorthandProperty(property) => {
                let name = property
                    .identifier()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?;
                (
                    DefaultTarget::Property,
                    destructured_property_type(ctx, &property.parent()?, name.text_trimmed())?,
                )
            }
            AnyJsBindingWithDefault::JsObjectBindingPatternProperty(property) => {
                let name = property.member().ok()?.name()?;
                (
                    DefaultTarget::Property,
                    destructured_property_type(ctx, &property.parent()?, name.text())?,
                )
            }
            AnyJsBindingWithDefault::JsArrayBindingPatternElement(element) => {
                let elements = element.parent::<JsArrayBindingPatternElementList>()?;
                let pattern = elements.parent::<JsArrayBindingPattern>()?;
                let index = elements
                    .iter()
                    .position(|item| item.is_ok_and(|item| item.syntax() == element.syntax()))?;
                let source = pattern_source_type(ctx, &pattern.into())?;
                (
                    DefaultTarget::Element,
                    source.required_tuple_element_type(index)?,
                )
            }
            AnyJsBindingWithDefault::JsFormalParameter(parameter) => {
                let parameter_type = expected_callback_parameter_type(ctx, parameter)?;
                if parameter_type.is_generic() {
                    return None;
                }
                (DefaultTarget::Parameter, parameter_type)
            }
        };

        (value_type.has_undefined_variant() == Some(false)).then_some(target)
    }

    fn diagnostic(ctx: &RuleContext<Self>, target: &Self::State) -> Option<RuleDiagnostic> {
        let default_value = ctx.query().initializer()?.expression().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                default_value.range(),
                markup! { "This default value is never used." },
            )
            .note(markup! {
                "A default value only applies when the value is "<Emphasis>"undefined"</Emphasis>", but the type of this "{target}" doesn't include "<Emphasis>"undefined"</Emphasis>"."
            })
            .note(markup! {
                "Remove the default value, or add "<Emphasis>"undefined"</Emphasis>" to the type if the value can be missing."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _target: &Self::State) -> Option<JsRuleAction> {
        let initializer = ctx.query().initializer()?;
        // The whitespace before `=` belongs to the preceding token. It is
        // replaced by the trivia after the default value, so `(value = "")`
        // becomes `(value)` and `{ value = "" }` becomes `{ value }`.
        let previous = initializer.syntax().first_token()?.prev_token()?;
        // Removing the line break before `=` would move the code after the
        // default value into a `//` comment that ends the previous line.
        if previous
            .trailing_trivia()
            .pieces()
            .any(|piece| piece.kind().is_single_line_comment())
        {
            return None;
        }
        let last = initializer.syntax().last_token()?;
        let new_previous = previous.with_trailing_trivia_pieces(chain_trivia_pieces(
            trim_trailing_trivia_pieces(previous.trailing_trivia().pieces()),
            last.trailing_trivia().pieces(),
        ));
        let mut mutation = ctx.root().begin();
        mutation.replace_token_discard_trivia(previous, new_previous);
        mutation.remove_node(initializer);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the default value." }.to_owned(),
            mutation,
        ))
    }
}

impl AnyJsBindingWithDefault {
    fn initializer(&self) -> Option<JsInitializerClause> {
        match self {
            Self::JsArrayBindingPatternElement(element) => element.init(),
            Self::JsFormalParameter(parameter) => parameter.initializer(),
            Self::JsObjectBindingPatternProperty(property) => property.init(),
            Self::JsObjectBindingPatternShorthandProperty(property) => property.init(),
        }
    }
}

impl biome_console::fmt::Display for DefaultTarget {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Element => "array element",
            Self::Parameter => "parameter",
            Self::Property => "property",
        })
    }
}

/// Returns the type of the property `name`, which a member of `properties`
/// destructures, when the destructured object declares it as required.
fn destructured_property_type<'a>(
    ctx: &'a RuleContext<NoUselessDefaultAssignment>,
    properties: &JsObjectBindingPatternPropertyList,
    name: &str,
) -> Option<InferredType<'a>> {
    let pattern = properties.parent::<JsObjectBindingPattern>()?;
    pattern_source_type(ctx, &pattern.into())?.required_property_type(name)
}

/// Returns the type of the value that `pattern` destructures, when `pattern`
/// is the whole target of a variable declaration without a type annotation,
/// or a whole parameter of a function declaration, function expression, or
/// arrow function.
///
/// Nested patterns return `None`.
fn pattern_source_type<'a>(
    ctx: &'a RuleContext<NoUselessDefaultAssignment>,
    pattern: &AnyJsBindingPattern,
) -> Option<InferredType<'a>> {
    let parent = pattern.syntax().parent()?;
    if let Some(declarator) = JsVariableDeclarator::cast_ref(&parent) {
        // The type of the initializer isn't the type of the destructured value
        // when an annotation declares a different one.
        if declarator.variable_annotation().is_some() {
            return None;
        }
        return ctx.type_of_expression(&declarator.initializer()?.expression().ok()?);
    }

    let parameter = JsFormalParameter::cast(parent)?;
    let (function, index) = function_of_parameter(&parameter)?;
    let function_type = match function {
        AnyJsFunction::JsArrowFunctionExpression(function) => {
            ctx.type_of_expression(&function.into())?
        }
        AnyJsFunction::JsFunctionExpression(function) => ctx.type_of_expression(&function.into())?,
        AnyJsFunction::JsFunctionDeclaration(function) => {
            let name = function.id().ok()?;
            let name = name.as_js_identifier_binding()?.name_token().ok()?;
            ctx.type_of_named_value(function.range(), name.text_trimmed())?
        }
        AnyJsFunction::JsFunctionExportDefaultDeclaration(function) => {
            let name = function.id()?;
            let name = name.as_js_identifier_binding()?.name_token().ok()?;
            ctx.type_of_named_value(function.range(), name.text_trimmed())?
        }
    };
    function_type.required_callable_parameter_type(index)
}

/// Returns the type that a call passes to `parameter`, when `parameter`
/// belongs to a function expression or arrow function written directly as an
/// argument of a call or `new` expression.
///
/// Returns `None` when the call can omit the parameter, or when the expected
/// type of the argument can't be determined.
fn expected_callback_parameter_type<'a>(
    ctx: &'a RuleContext<NoUselessDefaultAssignment>,
    parameter: &JsFormalParameter,
) -> Option<InferredType<'a>> {
    let (function, index) = function_of_parameter(parameter)?;
    let function = match function {
        AnyJsFunction::JsArrowFunctionExpression(function) => AnyJsExpression::from(function),
        AnyJsFunction::JsFunctionExpression(function) => AnyJsExpression::from(function),
        AnyJsFunction::JsFunctionDeclaration(_)
        | AnyJsFunction::JsFunctionExportDefaultDeclaration(_) => return None,
    };

    let argument = function.outer_expression()?;
    let arguments = JsCallArgumentList::cast(argument.syntax().parent()?)?;
    let argument_index = arguments
        .iter()
        .position(|item| item.is_ok_and(|item| item.syntax() == argument.syntax()))?;
    let call = arguments.parent::<JsCallArguments>()?.syntax().parent()?;
    let expected = if let Some(call) = JsCallExpression::cast_ref(&call) {
        ctx.expected_argument_type(&call.callee().ok()?, &arguments, argument_index, false)?
    } else {
        let call = JsNewExpression::cast(call)?;
        ctx.expected_argument_type(&call.callee().ok()?, &arguments, argument_index, true)?
    };
    expected.required_callable_parameter_type(index)
}

/// Returns the function that declares `parameter`, and the position of
/// `parameter` in its parameter list, not counting a `this` parameter.
fn function_of_parameter(parameter: &JsFormalParameter) -> Option<(AnyJsFunction, usize)> {
    let parameters = parameter.parent::<JsParameterList>()?;
    let index = parameters
        .iter()
        .filter_map(Result::ok)
        .filter(|item| !matches!(item, AnyJsParameter::TsThisParameter(_)))
        .position(|item| item.syntax() == parameter.syntax())?;
    let function = AnyJsFunction::cast(parameters.parent::<JsParameters>()?.syntax().parent()?)?;
    Some((function, index))
}
