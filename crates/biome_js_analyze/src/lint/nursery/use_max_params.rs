use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, JsArrowFunctionExpression, JsConstructorClassMember,
    JsConstructorParameters, JsFunctionDeclaration, JsFunctionExpression, JsMethodClassMember,
    JsMethodObjectMember, JsParameters,
};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::use_max_params::UseMaxParamsOptions;

declare_lint_rule! {
    /// Enforce a maximum number of parameters in function definitions.
    ///
    /// Functions that take numerous parameters can be difficult to read and write
    /// because it requires the memorization of what each parameter is, its type,
    /// and the order they should appear in.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo(a, b, c, d, e, f, g, h) {
    ///     // too many parameters
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const bar = (a, b, c, d, e, f, g, h) => {
    ///     // too many parameters
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Baz {
    ///     method(a, b, c, d, e, f, g, h) {
    ///         // too many parameters
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo(a, b, c) {
    ///     // within limit
    /// }
    /// ```
    ///
    /// ```js
    /// const bar = (a, b, c) => {
    ///     // within limit
    /// }
    /// ```
    ///
    /// ```js
    /// class Baz {
    ///     method(a, b, c) {
    ///         // within limit
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### max
    ///
    /// The maximum number of parameters allowed (default: 7).
    ///
    pub UseMaxParams {
        version: "next",
        name: "useMaxParams",
        language: "js",
        severity: Severity::Warning,
        recommended: false,
    }
}

declare_node_union! {
    pub AnyFunctionLike = JsFunctionDeclaration | JsFunctionExpression | JsArrowFunctionExpression | JsMethodClassMember | JsMethodObjectMember | JsConstructorClassMember
}

#[derive(Debug, Clone)]
pub struct UseMaxParamsState {
    pub parameter_count: usize,
}

impl Rule for UseMaxParams {
    type Query = Ast<AnyFunctionLike>;
    type State = UseMaxParamsState;
    type Signals = Option<Self::State>;
    type Options = UseMaxParamsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();

        let parameters = match node {
            AnyFunctionLike::JsFunctionDeclaration(func) => {
                func.parameters().ok().map(FunctionParameters::JsParameters)
            }
            AnyFunctionLike::JsFunctionExpression(func) => {
                func.parameters().ok().map(FunctionParameters::JsParameters)
            }
            AnyFunctionLike::JsArrowFunctionExpression(func) => {
                func.parameters().ok().map(|p| match p {
                    AnyJsArrowFunctionParameters::JsParameters(params) => {
                        FunctionParameters::JsParameters(params)
                    }
                    AnyJsArrowFunctionParameters::AnyJsBinding(_) => {
                        FunctionParameters::SingleParameter
                    }
                })
            }
            AnyFunctionLike::JsMethodClassMember(method) => method
                .parameters()
                .ok()
                .map(FunctionParameters::JsParameters),
            AnyFunctionLike::JsMethodObjectMember(method) => method
                .parameters()
                .ok()
                .map(FunctionParameters::JsParameters),
            AnyFunctionLike::JsConstructorClassMember(constructor) => constructor
                .parameters()
                .ok()
                .map(FunctionParameters::JsConstructorParameters),
        };

        let parameters = parameters?;
        let parameter_count = count_parameters(&parameters);

        (parameter_count > options.max as usize).then_some(UseMaxParamsState { parameter_count })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let options = ctx.options();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Function has "{state.parameter_count}" parameters, but only "{options.max}" are allowed."
                },
            )
            .note(markup! {
                "Functions with many parameters are hard to read and maintain."
            })
            .note(markup! {
                "Consider using an options object, splitting into smaller functions, or grouping related parameters."
            })
        )
    }
}

enum FunctionParameters {
    JsParameters(JsParameters),
    JsConstructorParameters(JsConstructorParameters),
    SingleParameter,
}

fn count_parameters(parameters: &FunctionParameters) -> usize {
    match parameters {
        FunctionParameters::SingleParameter => 1,
        FunctionParameters::JsParameters(params) => count_js_parameters(params),
        FunctionParameters::JsConstructorParameters(params) => count_constructor_parameters(params),
    }
}

fn count_js_parameters(params: &JsParameters) -> usize {
    params.items().into_iter().filter_map(Result::ok).count()
}

fn count_constructor_parameters(params: &JsConstructorParameters) -> usize {
    params
        .parameters()
        .into_iter()
        .filter_map(Result::ok)
        .count()
}
