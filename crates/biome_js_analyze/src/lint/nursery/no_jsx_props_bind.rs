use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_js_semantic::Binding;
use biome_js_syntax::{AnyJsExpression, JsxAttribute, binding_ext::AnyJsBindingDeclaration};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_jsx_props_bind::NoJsxPropsBindOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow .bind(), arrow functions, or function expressions in JSX props
    ///
    /// Using `.bind()` or creating a function inline in props creates a new function
    /// on every render, changing identity and defeating memoisation,
    /// which may cause unnecessary rerenders.
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <Foo onClick={this._handleClick.bind(this)}></Foo>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <Foo onClick={() => console.log('Hello!')}></Foo>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <Foo onClick={function () { console.log('Hello!'); }}></Foo>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Foo onClick={this._handleClick}></Foo>
    /// ```

    pub NoJsxPropsBind {
        version: "2.3.11",
        name: "noJsxPropsBind",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-no-bind").inspired()],
        domains: &[RuleDomain::React],
    }
}

enum InvalidKind {
    ArrowFunction,
    Function,
    Bind,
}

pub struct NoJsxPropsBindState {
    invalid_kind: InvalidKind,
    attribute_range: TextRange,
}

fn declaration_is_global(declaration: &AnyJsBindingDeclaration) -> bool {
    // TODO: This needs some work
    !declaration
        .syntax()
        .ancestors()
        .skip(1)
        .any(|anc| anc.kind() == biome_js_syntax::JsSyntaxKind::JS_FUNCTION_DECLARATION)
}

impl Rule for NoJsxPropsBind {
    type Query = Semantic<JsxAttribute>;
    type State = NoJsxPropsBindState;
    type Signals = Option<Self::State>;
    type Options = NoJsxPropsBindOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression: AnyJsExpression = ctx
            .query()
            .initializer()?
            .value()
            .ok()?
            .as_jsx_expression_attribute_value()?
            .expression()
            .ok()?;

        match &expression {
            AnyJsExpression::JsArrowFunctionExpression(_) => Some(NoJsxPropsBindState {
                invalid_kind: InvalidKind::ArrowFunction,
                attribute_range: expression.range(),
            }),

            AnyJsExpression::JsFunctionExpression(_) => Some(NoJsxPropsBindState {
                invalid_kind: InvalidKind::Function,
                attribute_range: expression.range(),
            }),
            AnyJsExpression::JsCallExpression(call) => {
                // TODO: This will still throw a false positive on e.g. window.bind()
                let is_bind = call
                    .callee()
                    .ok()
                    .and_then(|c| c.as_js_static_member_expression().cloned())
                    .and_then(|m| m.member().ok())
                    .and_then(|n| n.value_token().ok())
                    .is_some_and(|t| t.text() == "bind");
                if is_bind {
                    Some(NoJsxPropsBindState {
                        invalid_kind: InvalidKind::Bind,
                        attribute_range: expression.range(),
                    })
                } else {
                    None
                }
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let model = ctx.model();
                let binding: Binding = model.binding(&identifier.name().ok()?)?;

                let declaration = binding.tree().declaration()?;

                match &declaration {
                    AnyJsBindingDeclaration::JsFunctionDeclaration(_) => {
                        if declaration_is_global(&declaration) {
                            return None;
                        }
                        Some(NoJsxPropsBindState {
                            invalid_kind: InvalidKind::Function,
                            attribute_range: expression.range(),
                        })
                    }
                    AnyJsBindingDeclaration::JsVariableDeclarator(variable_declarator) => {
                        match variable_declarator.initializer()?.expression().ok()? {
                            AnyJsExpression::JsFunctionExpression(_)
                            | AnyJsExpression::JsArrowFunctionExpression(_) => {
                                if declaration_is_global(&declaration) {
                                    return None;
                                }
                                Some(NoJsxPropsBindState {
                                    invalid_kind: InvalidKind::Function,
                                    attribute_range: expression.range(),
                                })
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let note = match state.invalid_kind {
            InvalidKind::ArrowFunction => "JSX props should not use arrow functions",
            InvalidKind::Bind => "JSX props should not use .bind()",
            InvalidKind::Function => "JSX props should not use function expressions",
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.attribute_range,
                "This function will be recreated on every render. Pass stable function references as props to avoid unnecessary rerenders.",
            )
            .note(note)
            .note("Consider extracting the function or wrapping it in useCallback"),
        )
    }
}
