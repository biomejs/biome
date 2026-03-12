use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, AnyJsLiteralExpression, AnyJsName,
    AnyTsType, JsVariableDeclaration, JsVariableDeclarator, TsTypeAnnotation,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_destructuring::UseDestructuringOptions;
use std::collections::HashSet;

declare_lint_rule! {
    /// Require destructuring from arrays and/or objects
    ///
    /// With JavaScript ES6, a new syntax was added for creating variables from an array index or object property,
    /// called destructuring. This rule enforces usage of destructuring instead of accessing a property through a member expression.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var foo = array[0];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var bar = foo.bar;
    /// ```
    ///
    ///
    /// ### Valid
    ///
    /// ```js
    /// var [foo] = array;
    /// ```
    ///
    /// ```js
    /// var { bar } = foo;
    /// ```
    ///
    /// ```ts
    /// // Variables with type annotations are ignored
    /// const foo: string = object.foo;
    /// ```
    ///
    pub UseDestructuring {
        version: "2.3.9",
        name: "useDestructuring",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("prefer-destructuring").same()],
    }
}

impl Rule for UseDestructuring {
    type Query = Semantic<JsVariableDeclarator>;
    type State = UseDestructuringState;
    type Signals = Option<Self::State>;
    type Options = UseDestructuringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let initializer = node.initializer()?;
        let declaration = JsVariableDeclaration::cast(node.syntax().parent()?.parent()?)?;
        let has_await_using = declaration.await_token().is_some();
        if declaration.kind().ok()?.text_trimmed() == "using" || has_await_using {
            return None;
        }

        if node.variable_annotation().is_some() {
            return None;
        }

        let left = node.id().ok()?;
        let right = initializer.expression().ok()?;

        if let AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(expr)) = left {
            let ident = expr.name_token().ok()?;
            return should_suggest_destructuring(ident.text_trimmed(), &right, ctx.model());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        match state {
            UseDestructuringState::Array => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        node.range(),
                        markup! {
                            "Use array destructuring instead of accessing array elements by index."
                        },
                    )
                    .note(markup! {
                        "Array destructuring is more readable and expressive than accessing individual elements by index."
                    })
                    .note(markup! {
                        "Replace the array index access with array destructuring syntax."
                    }),
                )
            }
            UseDestructuringState::Object => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        node.range(),
                        markup! {
                            "Use object destructuring instead of accessing object properties."
                        },
                    )
                    .note(markup! {
                        "Object destructuring is more readable and expressive than accessing individual properties."
                    })
                    .note(markup! {
                        "Replace the property access with object destructuring syntax."
                    }),
                )
            }
        }
    }
}

fn should_suggest_destructuring(
    left: &str,
    right: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<UseDestructuringState> {
    match right {
        AnyJsExpression::JsComputedMemberExpression(expr) => {
            if expr.is_optional_chain() {
                return None;
            }

            let member = expr.member().ok()?;
            if let AnyJsExpression::AnyJsLiteralExpression(lit) = member {
                if matches!(lit, AnyJsLiteralExpression::JsNumberLiteralExpression(_)) {
                    let object = expr.object().ok()?;
                    return supports_array_destructuring(&object, model)
                        .then_some(UseDestructuringState::Array);
                }

                let value = lit.value_token().ok()?;

                if left == value.text_trimmed() {
                    return Some(UseDestructuringState::Object);
                }
            }

            None
        }
        AnyJsExpression::JsStaticMemberExpression(expr) => {
            if matches!(expr.member().ok()?, AnyJsName::JsPrivateName(_))
                || matches!(expr.object().ok()?, AnyJsExpression::JsSuperExpression(_))
            {
                return None;
            }

            if expr.is_optional_chain() {
                return None;
            }
            let member = expr.member().ok()?.value_token().ok()?;
            if left == member.text_trimmed() {
                return Some(UseDestructuringState::Object);
            }
            None
        }
        _ => None,
    }
}

pub enum UseDestructuringState {
    Object,
    Array,
}

fn supports_array_destructuring(object: &AnyJsExpression, model: &SemanticModel) -> bool {
    let mut visited = HashSet::<TextRange>::new();
    !matches!(
        array_destructuring_support_for_expression(object, model, &mut visited),
        Some(false)
    )
}

fn array_destructuring_support_for_expression(
    object: &AnyJsExpression,
    model: &SemanticModel,
    visited: &mut HashSet<TextRange>,
) -> Option<bool> {
    match object.clone().omit_parentheses() {
        AnyJsExpression::JsArrayExpression(_) => Some(true),
        AnyJsExpression::JsIdentifierExpression(expr) => {
            let reference = expr.name().ok()?;
            let declaration = model.binding(&reference)?.tree().declaration()?;
            array_destructuring_support_for_declaration(&declaration, model, visited)
        }
        _ => None,
    }
}

fn array_destructuring_support_for_declaration(
    declaration: &AnyJsBindingDeclaration,
    model: &SemanticModel,
    visited: &mut HashSet<TextRange>,
) -> Option<bool> {
    match declaration {
        AnyJsBindingDeclaration::JsVariableDeclarator(node) => {
            if let Some(annotation) = node.variable_annotation() {
                if let Some(annotation) = annotation.as_ts_type_annotation() {
                    return array_destructuring_support_for_type_annotation(
                        annotation, model, visited,
                    );
                }
            }

            let initializer = node.initializer()?.expression().ok()?;
            array_destructuring_support_for_expression(&initializer, model, visited)
        }
        AnyJsBindingDeclaration::JsFormalParameter(node) => {
            let annotation = node.type_annotation()?;
            array_destructuring_support_for_type_annotation(&annotation, model, visited)
        }
        AnyJsBindingDeclaration::TsPropertyParameter(node) => {
            let annotation = node
                .formal_parameter()
                .ok()?
                .as_js_formal_parameter()?
                .type_annotation()?;
            array_destructuring_support_for_type_annotation(&annotation, model, visited)
        }
        AnyJsBindingDeclaration::TsTypeAliasDeclaration(node) => {
            let range = node.range();
            if !visited.insert(range) {
                return Some(false);
            }
            let ty = node.ty().ok()?;
            array_destructuring_support_for_type(&ty, model, visited)
        }
        AnyJsBindingDeclaration::TsInterfaceDeclaration(_) => Some(false),
        _ => None,
    }
}

fn array_destructuring_support_for_type_annotation(
    annotation: &TsTypeAnnotation,
    model: &SemanticModel,
    visited: &mut HashSet<TextRange>,
) -> Option<bool> {
    let ty = annotation.ty().ok()?;
    array_destructuring_support_for_type(&ty, model, visited)
}

fn array_destructuring_support_for_type(
    ty: &AnyTsType,
    model: &SemanticModel,
    visited: &mut HashSet<TextRange>,
) -> Option<bool> {
    match ty {
        AnyTsType::TsArrayType(_)
        | AnyTsType::TsTupleType(_)
        | AnyTsType::TsStringType(_)
        | AnyTsType::TsStringLiteralType(_) => Some(true),
        AnyTsType::TsObjectType(_)
        | AnyTsType::TsBigintType(_)
        | AnyTsType::TsBigintLiteralType(_)
        | AnyTsType::TsBooleanType(_)
        | AnyTsType::TsBooleanLiteralType(_)
        | AnyTsType::TsNullLiteralType(_)
        | AnyTsType::TsNumberType(_)
        | AnyTsType::TsNumberLiteralType(_)
        | AnyTsType::TsSymbolType(_)
        | AnyTsType::TsUndefinedType(_)
        | AnyTsType::TsUnknownType(_)
        | AnyTsType::TsVoidType(_)
        | AnyTsType::TsNeverType(_)
        | AnyTsType::TsNonPrimitiveType(_) => Some(false),
        AnyTsType::TsParenthesizedType(node) => {
            let ty = node.ty().ok()?;
            array_destructuring_support_for_type(&ty, model, visited)
        }
        AnyTsType::TsTypeOperatorType(node) => {
            let ty = node.ty().ok()?;
            array_destructuring_support_for_type(&ty, model, visited)
        }
        AnyTsType::TsReferenceType(node) => {
            let range = node.range();
            if !visited.insert(range) {
                return Some(false);
            }
            let name = node.name().ok()?;
            if let Some(reference) = name.as_js_reference_identifier() {
                let token = reference.value_token().ok()?;
                let name = token.text_trimmed();
                if matches!(
                    name,
                    "Array"
                        | "ReadonlyArray"
                        | "Iterable"
                        | "IterableIterator"
                        | "Iterator"
                        | "Generator"
                        | "String"
                ) {
                    return Some(true);
                }

                let declaration = model.binding(reference)?.tree().declaration()?;
                return array_destructuring_support_for_declaration(&declaration, model, visited);
            }

            None
        }
        _ => None,
    }
}
