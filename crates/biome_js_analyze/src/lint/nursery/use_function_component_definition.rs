use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsSyntaxToken, JsVariableDeclaration, JsVariableStatement, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind};
use biome_rule_options::use_function_component_definition::{
    ComponentDefinitionStyle, UseFunctionComponentDefinitionOptions,
};

use crate::{
    JsRuleAction,
    react::components::{
        AnyPotentialReactComponentDeclaration, ReactComponentInfo, ReactComponentKind,
    },
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
        sources: &[RuleSource::EslintReact("function-component-definition").inspired()],
        fix_kind: FixKind::Unsafe,
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
                    "The React component " {state.component_name.text_trimmed()} " is defined as " {state.actual_style.label()} "."
                },
            )
            .note(markup! {
                "Mixing component definition styles makes component declarations harder to scan."
            })
            .note(markup! {
                "Rewrite this component as " {state.preferred_style.label()} " or configure `namedComponents` to allow " {state.actual_style.label()} "."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if state.preferred_style != ComponentDefinitionStyle::FunctionDeclaration {
            return None;
        }

        let node = ctx.query();
        let AnyPotentialReactComponentDeclaration::JsVariableDeclarator(declarator) = node else {
            return None;
        };
        if declarator.variable_annotation().is_some() {
            return None;
        }

        let declaration = declarator.parent::<JsVariableDeclaration>()?;
        if declaration.declarators().len() != 1 {
            return None;
        }

        let statement = declaration.parent::<JsVariableStatement>()?;
        let function = declarator
            .initializer()?
            .expression()
            .ok()?
            .as_js_function_expression()?
            .clone();

        let id = declarator.id().ok()?.as_any_js_binding()?.clone();
        let id_name = id.as_js_identifier_binding()?.name_token().ok()?;
        if let Some(function_id) = function.id()
            && function_id
                .as_js_identifier_binding()?
                .name_token()
                .ok()?
                .text_trimmed()
                != id_name.text_trimmed()
        {
            return None;
        }

        let mut function_declaration = make::js_function_declaration(
            make::token(T![function]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            id,
            function.parameters().ok()?,
            function.body().ok()?,
        );

        if let Some(async_token) = function.async_token() {
            function_declaration = function_declaration.with_async_token(async_token);
        }
        if let Some(star_token) = function.star_token() {
            function_declaration = function_declaration.with_star_token(star_token);
        }
        if let Some(type_parameters) = function.type_parameters() {
            function_declaration = function_declaration.with_type_parameters(type_parameters);
        }
        if let Some(return_type_annotation) = function.return_type_annotation() {
            function_declaration =
                function_declaration.with_return_type_annotation(return_type_annotation);
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsStatement::JsVariableStatement(statement),
            AnyJsStatement::JsFunctionDeclaration(function_declaration.build()),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a function declaration for this component." }.to_owned(),
            mutation,
        ))
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
            let is_memo = callee_member_name == "memo";
            let is_forward_ref = callee_member_name == "forwardRef";
            if !is_memo && !is_forward_ref {
                return None;
            }

            let args = call.arguments().ok()?;
            let args_len = args.args().len();
            if (is_memo && !(1..=2).contains(&args_len)) || (is_forward_ref && args_len != 1) {
                return None;
            }

            let first_arg = args.args().into_iter().next()?.ok()?;
            expression_definition_style(first_arg.as_any_js_expression()?)
        }
        _ => None,
    }
}
