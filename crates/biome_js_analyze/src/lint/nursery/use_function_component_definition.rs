use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::use_function_component_definition::{
    ComponentDefinitionStyle, UseFunctionComponentDefinitionOptions,
};

use crate::react::components::{
    AnyPotentialReactComponentDeclaration, ReactComponentInfo, ReactComponentKind,
};

declare_lint_rule! {
    /// Enforce a specific function type for React function components.
    ///
    /// This rule keeps function component definitions consistent. By default, named
    /// components must be written as function declarations.
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
    /// ## Options
    ///
    /// Use `namedComponents` to set the style for named function components.
    ///
    /// ```json
    /// {
    ///   "options": {
    ///     "namedComponents": "arrowFunction"
    ///   }
    /// }
    /// ```
    ///
    pub UseFunctionComponentDefinition {
        version: "next",
        name: "useFunctionComponentDefinition",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReact("function-component-definition").same()],
    }
}

#[derive(Debug)]
pub struct UseFunctionComponentDefinitionState {
    component_name: JsSyntaxToken,
    actual_style: ComponentDefinitionStyle,
    preferred_style: ComponentDefinitionStyle,
    range: TextRange,
}

impl Rule for UseFunctionComponentDefinition {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = UseFunctionComponentDefinitionState;
    type Signals = Option<Self::State>;
    type Options = UseFunctionComponentDefinitionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let info = ReactComponentInfo::from_declaration(node.syntax())?;
        let ReactComponentKind::Function(_) = info.kind else {
            return None;
        };

        let component_name = info.name.clone()?;
        let range = info.declaration_highlight_range();
        let actual_style = component_definition_style(node)?;
        let preferred_style = ctx
            .options()
            .named_components
            .unwrap_or(ComponentDefinitionStyle::FunctionDeclaration);

        if actual_style == preferred_style {
            return None;
        }

        Some(UseFunctionComponentDefinitionState {
            component_name,
            actual_style,
            preferred_style,
            range,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Use " {state.preferred_style.label()} " for the React component " {state.component_name.text_trimmed()} "."
                },
            )
            .note(markup! {
                "This component is currently defined as " {state.actual_style.label()} ". Mixing component definition styles makes component declarations harder to scan."
            })
            .note(markup! {
                "Rewrite this component as " {state.preferred_style.label()} " or configure `namedComponents` to allow " {state.actual_style.label()} "."
            }),
        )
    }
}

fn component_definition_style(
    node: &AnyPotentialReactComponentDeclaration,
) -> Option<ComponentDefinitionStyle> {
    match node {
        AnyPotentialReactComponentDeclaration::JsFunctionDeclaration(_)
        | AnyPotentialReactComponentDeclaration::JsFunctionExportDefaultDeclaration(_) => {
            Some(ComponentDefinitionStyle::FunctionDeclaration)
        }
        AnyPotentialReactComponentDeclaration::JsVariableDeclarator(declarator) => {
            expression_definition_style(&declarator.initializer()?.expression().ok()?)
        }
        AnyPotentialReactComponentDeclaration::JsAssignmentExpression(assignment) => {
            expression_definition_style(&assignment.right().ok()?)
        }
        _ => None,
    }
}

fn expression_definition_style(expression: &AnyJsExpression) -> Option<ComponentDefinitionStyle> {
    match expression {
        AnyJsExpression::JsFunctionExpression(_) => Some(ComponentDefinitionStyle::FunctionExpression),
        AnyJsExpression::JsArrowFunctionExpression(_) => Some(ComponentDefinitionStyle::ArrowFunction),
        AnyJsExpression::JsCallExpression(call) => {
            let callee_name = call.callee().ok()?.get_callee_member_name()?;
            let callee_member_name = callee_name.text_trimmed();
            if callee_member_name != "memo" && callee_member_name != "forwardRef" {
                return None;
            }

            let args = call.arguments().ok()?;
            if args.args().len() != 1 {
                return None;
            }

            let first_arg = args.args().into_iter().next()?.ok()?;
            expression_definition_style(first_arg.as_any_js_expression()?)
        }
        _ => None,
    }
}
