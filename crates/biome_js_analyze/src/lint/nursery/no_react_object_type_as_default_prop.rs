use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBindingPattern, AnyJsExpression, AnyJsFormalParameter,
    AnyJsLiteralExpression, AnyJsObjectBindingPatternMember, AnyJsParameter, JsParameters,
};
use biome_rowan::AstNode;
use biome_rowan::TextRange;
use biome_rule_options::no_react_object_type_as_default_prop::NoReactObjectTypeAsDefaultPropOptions;
use crate::react::components::{AnyPotentialReactComponentDeclaration, ReactComponentInfo};

declare_lint_rule! {
    /// Disallow array, object, and function values as default props.
    ///
    /// In React, a default prop value like `{ items = [] }` is created every
    /// time the component renders. Arrays, objects, and functions are new values
    /// each time, even when they look the same. React then thinks the prop changed,
    /// so it may re-render the component more than needed, or re-run hooks like
    /// `useEffect` that depends on the prop.
    ///
    /// Numbers, strings, and other primitives are fine, because they stay the same
    /// between renders.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function Component({ items = [] }) {
    ///     return items;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const Component = ({ config = {} }) => config;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const EMPTY_ITEMS = [];
    ///
    /// function Component({ items = EMPTY_ITEMS }) {
    ///     return items;
    /// }
    /// ```
    ///
    /// ```js
    /// function Component({ count = 0, label = "default" }) {
    ///     return count;
    /// }
    /// ```
    ///
    pub NoReactObjectTypeAsDefaultProp {
        version: "next",
        name: "noReactObjectTypeAsDefaultProp",
        language: "js",
        sources: &[RuleSource::EslintReact("no-object-type-as-default-prop").same()],
        recommended: true,
        domains: &[RuleDomain::React],
    }
}

impl Rule for NoReactObjectTypeAsDefaultProp {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = ForbiddenDefault;
    type Signals = Vec<Self::State>;
    type Options = NoReactObjectTypeAsDefaultPropOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if ReactComponentInfo::from_declaration(node.syntax()).is_none() {
            return vec![];
        }
        let Some(parameters) = component_parameters(node) else {
            return vec![];
        };
        collect_forbidden_defaults(&parameters).unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let kind = state.kind.as_str();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "A new "{kind}" is created here on every render."
                },
            )
            .note(markup! {
                "React sees this as a different value each render, so the component may re-render more than needed."
            })
            .note(markup! {
                "Move this value to a constant outside the component and use that as the default."
            }),
        )
    }
}

fn component_parameters(node: &AnyPotentialReactComponentDeclaration) -> Option<JsParameters> {
    match node {
        AnyPotentialReactComponentDeclaration::JsFunctionDeclaration(declaration) => {
            declaration.parameters().ok()
        }
        AnyPotentialReactComponentDeclaration::JsFunctionExportDefaultDeclaration(declaration) => {
            declaration.parameters().ok()
        }
        AnyPotentialReactComponentDeclaration::JsVariableDeclarator(declarator) => {
            let expression = declarator.initializer()?.expression().ok()?;
            parameters_from_expression(&expression)
        }
        _ => None,
    }
}

fn parameters_from_expression(expression: &AnyJsExpression) -> Option<JsParameters> {
    if let Some(arrow) = expression.as_js_arrow_function_expression() {
        return match arrow.parameters().ok()? {
            AnyJsArrowFunctionParameters::JsParameters(parameters) => Some(parameters),
            AnyJsArrowFunctionParameters::AnyJsBinding(_) => None,
        };
    }
    if let Some(function) = expression.as_js_function_expression() {
        return function.parameters().ok();
    }
    // unwrap memo() / forwardRef() wrappers
    if let Some(call) = expression.as_js_call_expression() {
        let first_argument = call.arguments().ok()?.args().into_iter().next()?.ok()?;
        return parameters_from_expression(first_argument.as_any_js_expression()?);
    }
    None
}

#[derive(Clone, Copy)]
enum ForbiddenDefaultKind {
    ObjectLiteral,
    ArrayLiteral,
    ArrowFunction,
    FunctionExpression,
    ClassExpression,
    NewExpression,
    JsxElement,
    RegexLiteral,
    Symbol,
}

impl ForbiddenDefaultKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::ObjectLiteral => "an object literal",
            Self::ArrayLiteral => "an array literal",
            Self::ArrowFunction => "an arrow function",
            Self::FunctionExpression => "a function expression",
            Self::ClassExpression => "a class expression",
            Self::NewExpression => "a newly constructed object",
            Self::JsxElement => "a JSX element",
            Self::RegexLiteral => "a regular expression literal",
            Self::Symbol => "a Symbol",
        }
    }
}

pub struct ForbiddenDefault {
    range: TextRange,
    kind: ForbiddenDefaultKind,
}

fn forbidden_default_kind(expression: &AnyJsExpression) -> Option<ForbiddenDefaultKind> {
    let kind = match expression {
        AnyJsExpression::JsObjectExpression(_) => ForbiddenDefaultKind::ObjectLiteral,
        AnyJsExpression::JsArrayExpression(_) => ForbiddenDefaultKind::ArrayLiteral,
        AnyJsExpression::JsArrowFunctionExpression(_) => ForbiddenDefaultKind::ArrowFunction,
        AnyJsExpression::JsFunctionExpression(_) => ForbiddenDefaultKind::FunctionExpression,
        AnyJsExpression::JsClassExpression(_) => ForbiddenDefaultKind::ClassExpression,
        AnyJsExpression::JsNewExpression(_) => ForbiddenDefaultKind::NewExpression,
        AnyJsExpression::JsxTagExpression(_) => ForbiddenDefaultKind::JsxElement,
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsRegexLiteralExpression(_),
        ) => ForbiddenDefaultKind::RegexLiteral,
        _ => {
            let is_symbol_call = expression
                .as_js_call_expression()
                .and_then(|call| call.callee().ok())
                .and_then(|callee| callee.as_js_reference_identifier())
                .is_some_and(|ident| ident.has_name("Symbol"));
            if is_symbol_call {
                ForbiddenDefaultKind::Symbol
            } else {
                return None;
            }
        }
    };
    Some(kind)
}

fn collect_forbidden_defaults(parameters: &JsParameters) -> Option<Vec<ForbiddenDefault>> {
    let first_param = parameters.items().into_iter().next()?.ok()?;

    let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(param)) =
        first_param
    else {
        return None;
    };

    let AnyJsBindingPattern::JsObjectBindingPattern(object_pattern) = param.binding().ok()? else {
        return None;
    };

    let mut defaults = Vec::new();
    for property in object_pattern.properties() {
        let property = property.ok()?;

        let AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(shorthand) =
            property
        else {
            continue;
        };

        let Some(initializer) = shorthand.init() else {
            continue;
        };
        let default_value = initializer.expression().ok()?;

        let Some(kind) = forbidden_default_kind(&default_value) else {
            continue;
        };

        defaults.push(ForbiddenDefault {
            range: default_value.range(),
            kind,
        });
    }

    Some(defaults)
}
