use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::AnyJsExpression;
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_react_function_component_definition::{
    ComponentStyle, UseReactFunctionComponentDefinitionOptions,
};

use crate::react::components::{
    AnyPotentialReactComponentDeclaration, ReactComponentInfo, ReactComponentKind,
};

declare_lint_rule! {
    /// Enforce a consistent style for React function component definitions.
    ///
    /// This rule allows you to choose between function declarations and arrow functions for your components.
    /// Standardizing the style makes the codebase more consistent and easier to read.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// const MyComponent = (props) => {
    ///   return <div>{props.name}</div>;
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function MyComponent(props) {
    ///   return <div>{props.name}</div>;
    /// }
    /// ```
    ///
    pub UseReactFunctionComponentDefinition {
        version: "next",
        name: "useReactFunctionComponentDefinition",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReact("function-component-definition").same()],
    }
}


impl Rule for UseReactFunctionComponentDefinition {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = ComponentStyle;
    type Signals = Option<Self::State>;
    type Options = UseReactFunctionComponentDefinitionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let info = ReactComponentInfo::from_declaration(node.syntax())?;

        // We only care about function components
        let ReactComponentKind::Function(_) = &info.kind else {
            return None;
        };

        let style = get_component_style(node)?;
        let is_named = info.name.is_some();

        let preferred_style = if is_named {
            &options.named_components
        } else {
            &options.unnamed_components
        };

        if &style != preferred_style {
            return Some(preferred_style.clone());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let preferred_style = match state {
            ComponentStyle::FunctionDeclaration => "function declaration",
            ComponentStyle::FunctionExpression => "function expression",
            ComponentStyle::ArrowFunction => "arrow function",
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This React component should be defined using a "<Emphasis>{preferred_style}</Emphasis>"."
                },
            )
            .note(markup! {
                "Consistent component styles improve readability and maintainability."
            }),
        )
    }
}

fn get_component_style(node: &AnyPotentialReactComponentDeclaration) -> Option<ComponentStyle> {
    use AnyPotentialReactComponentDeclaration::*;
    match node {
        JsFunctionDeclaration(_) | JsFunctionExportDefaultDeclaration(_) => {
            Some(ComponentStyle::FunctionDeclaration)
        }
        JsVariableDeclarator(decl) => {
            let expression = decl.initializer()?.expression().ok()?;
            match expression {
                AnyJsExpression::JsArrowFunctionExpression(_) => Some(ComponentStyle::ArrowFunction),
                AnyJsExpression::JsFunctionExpression(_) => Some(ComponentStyle::FunctionExpression),
                AnyJsExpression::JsCallExpression(call) => {
                    // Handle wrapped components like memo(() => {})
                    let inner = get_inner_expression(AnyJsExpression::JsCallExpression(call))?;
                    match inner {
                        AnyJsExpression::JsArrowFunctionExpression(_) => {
                            Some(ComponentStyle::ArrowFunction)
                        }
                        AnyJsExpression::JsFunctionExpression(_) => {
                            Some(ComponentStyle::FunctionExpression)
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        JsExportDefaultExpressionClause(clause) => {
            let expression = clause.expression().ok()?;
            let inner = get_inner_expression(expression)?;
            match inner {
                AnyJsExpression::JsArrowFunctionExpression(_) => Some(ComponentStyle::ArrowFunction),
                AnyJsExpression::JsFunctionExpression(_) => Some(ComponentStyle::FunctionExpression),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Helper to unwrapped memo() or forwardRef()
fn get_inner_expression(expr: AnyJsExpression) -> Option<AnyJsExpression> {
    let mut current = expr;
    while let AnyJsExpression::JsCallExpression(call) = &current {
        let callee = call.callee().ok()?;
        let member_name = callee.get_callee_member_name()?;
        if member_name.text_trimmed() == "memo" || member_name.text_trimmed() == "forwardRef" {
            let args = call.arguments().ok()?.args();
            let first_arg = args.first()?.ok()?;
            current = first_arg.as_any_js_expression()?.clone();
        } else {
            break;
        }
    }
    Some(current)
}

