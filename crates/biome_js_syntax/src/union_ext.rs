use crate::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsClass, AnyJsClassMember, AnyJsClassMemberName,
    AnyJsFunction, AnyJsFunctionBody, AnyTsPropertyAnnotation, AnyTsVariableAnnotation,
    JsClassMemberList, JsDecoratorList, JsExtendsClause, JsInitializerClause, JsSyntaxToken,
    JsVariableDeclarator, TsImplementsClause, TsReturnTypeAnnotation, TsTypeAnnotation,
    TsTypeParameters,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxResult};

impl AnyJsClass {
    pub fn decorators(&self) -> JsDecoratorList {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.decorators(),
            AnyJsClass::JsClassExpression(expression) => expression.decorators(),
            AnyJsClass::JsClassExportDefaultDeclaration(declaration) => declaration.decorators(),
        }
    }

    pub fn abstract_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.abstract_token(),
            AnyJsClass::JsClassExpression(_) => None,
            AnyJsClass::JsClassExportDefaultDeclaration(clause) => clause.abstract_token(),
        }
    }

    pub fn class_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.class_token(),
            AnyJsClass::JsClassExpression(expression) => expression.class_token(),
            AnyJsClass::JsClassExportDefaultDeclaration(declaration) => declaration.class_token(),
        }
    }

    pub fn id(&self) -> Option<AnyJsBinding> {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.id().ok(),
            AnyJsClass::JsClassExpression(expression) => expression.id(),
            AnyJsClass::JsClassExportDefaultDeclaration(declaration) => declaration.id(),
        }
    }

    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.type_parameters(),
            AnyJsClass::JsClassExpression(expression) => expression.type_parameters(),
            AnyJsClass::JsClassExportDefaultDeclaration(clause) => clause.type_parameters(),
        }
    }

    pub fn extends_clause(&self) -> Option<JsExtendsClause> {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.extends_clause(),
            AnyJsClass::JsClassExpression(expression) => expression.extends_clause(),
            AnyJsClass::JsClassExportDefaultDeclaration(declaration) => {
                declaration.extends_clause()
            }
        }
    }

    pub fn implements_clause(&self) -> Option<TsImplementsClause> {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.implements_clause(),
            AnyJsClass::JsClassExpression(expression) => expression.implements_clause(),
            AnyJsClass::JsClassExportDefaultDeclaration(declaration) => {
                declaration.implements_clause()
            }
        }
    }

    pub fn l_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.l_curly_token(),
            AnyJsClass::JsClassExpression(expression) => expression.l_curly_token(),
            AnyJsClass::JsClassExportDefaultDeclaration(declaration) => declaration.l_curly_token(),
        }
    }

    pub fn members(&self) -> JsClassMemberList {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.members(),
            AnyJsClass::JsClassExpression(expression) => expression.members(),
            AnyJsClass::JsClassExportDefaultDeclaration(declaration) => declaration.members(),
        }
    }

    pub fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsClass::JsClassDeclaration(declaration) => declaration.r_curly_token(),
            AnyJsClass::JsClassExpression(expression) => expression.r_curly_token(),
            AnyJsClass::JsClassExportDefaultDeclaration(declaration) => declaration.r_curly_token(),
        }
    }
}

impl AnyJsClassMember {
    /// Returns the `name` of the member if it has any.
    pub fn name(&self) -> SyntaxResult<Option<AnyJsClassMemberName>> {
        match self {
            AnyJsClassMember::JsConstructorClassMember(constructor) => constructor
                .name()
                .map(|name| Some(AnyJsClassMemberName::from(name))),
            AnyJsClassMember::JsEmptyClassMember(_) => Ok(None),
            AnyJsClassMember::JsGetterClassMember(getter) => getter.name().map(Some),
            AnyJsClassMember::JsMethodClassMember(method) => method.name().map(Some),
            AnyJsClassMember::JsPropertyClassMember(property) => property.name().map(Some),
            AnyJsClassMember::JsSetterClassMember(setter) => setter.name().map(Some),
            AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => Ok(None),
            AnyJsClassMember::JsBogusMember(_) | AnyJsClassMember::JsMetavariable(_) => Ok(None),
            AnyJsClassMember::TsConstructorSignatureClassMember(constructor) => constructor
                .name()
                .map(|name| Some(AnyJsClassMemberName::from(name))),
            AnyJsClassMember::TsGetterSignatureClassMember(getter) => getter.name().map(Some),
            AnyJsClassMember::TsIndexSignatureClassMember(_) => Ok(None),
            AnyJsClassMember::TsMethodSignatureClassMember(method) => method.name().map(Some),
            AnyJsClassMember::TsPropertySignatureClassMember(property) => property.name().map(Some),
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(property) => {
                property.name().map(Some)
            }
            AnyJsClassMember::TsSetterSignatureClassMember(setter) => setter.name().map(Some),
        }
    }

    /// Tests if the member has a [`JsLiteralMemberName`](crate::JsLiteralMemberName) of `name`.
    pub fn has_name(&self, name: &str) -> SyntaxResult<bool> {
        match self.name()? {
            Some(AnyJsClassMemberName::JsLiteralMemberName(literal)) => {
                Ok(literal.value()?.text_trimmed() == name)
            }
            _ => Ok(false),
        }
    }
}

impl AnyJsClassMemberName {
    pub const fn is_computed(&self) -> bool {
        matches!(self, AnyJsClassMemberName::JsComputedMemberName(_))
    }
}

impl AnyJsFunction {
    pub fn async_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsFunction::JsArrowFunctionExpression(expr) => expr.async_token(),
            AnyJsFunction::JsFunctionExpression(expr) => expr.async_token(),
            AnyJsFunction::JsFunctionDeclaration(declaration) => declaration.async_token(),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.async_token()
            }
        }
    }

    /// Returns the binding by which the function can be accessed.
    ///
    /// This may be a binding for the function's identifier, or a binding for
    /// the variable to which the function is assigned.
    pub fn binding(&self) -> Option<AnyJsBinding> {
        match self {
            AnyJsFunction::JsFunctionDeclaration(declaration) => declaration.id().ok(),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => declaration.id(),
            AnyJsFunction::JsArrowFunctionExpression(_)
            | AnyJsFunction::JsFunctionExpression(_) => {
                let parent = self
                    .parent::<JsInitializerClause>()?
                    .parent::<JsVariableDeclarator>()?;
                parent.id().ok()?.as_any_js_binding().cloned()
            }
        }
    }

    pub fn is_async(&self) -> bool {
        self.async_token().is_some()
    }

    pub fn function_token(&self) -> SyntaxResult<Option<JsSyntaxToken>> {
        match self {
            AnyJsFunction::JsArrowFunctionExpression(_) => Ok(None),
            AnyJsFunction::JsFunctionExpression(expr) => expr.function_token().map(Some),
            AnyJsFunction::JsFunctionDeclaration(declaration) => {
                declaration.function_token().map(Some)
            }
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.function_token().map(Some)
            }
        }
    }

    pub fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsFunction::JsArrowFunctionExpression(_) => None,
            AnyJsFunction::JsFunctionExpression(expr) => expr.star_token(),
            AnyJsFunction::JsFunctionDeclaration(declaration) => declaration.star_token(),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.star_token()
            }
        }
    }

    pub fn is_generator(&self) -> bool {
        self.star_token().is_some()
    }

    pub fn id(&self) -> SyntaxResult<Option<AnyJsBinding>> {
        match self {
            AnyJsFunction::JsArrowFunctionExpression(_) => Ok(None),
            AnyJsFunction::JsFunctionExpression(expr) => Ok(expr.id()),
            AnyJsFunction::JsFunctionDeclaration(declaration) => declaration.id().map(Some),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => Ok(declaration.id()),
        }
    }

    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            AnyJsFunction::JsArrowFunctionExpression(expr) => expr.type_parameters(),
            AnyJsFunction::JsFunctionExpression(expr) => expr.type_parameters(),
            AnyJsFunction::JsFunctionDeclaration(declaration) => declaration.type_parameters(),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.type_parameters()
            }
        }
    }

    pub fn parameters(&self) -> SyntaxResult<AnyJsArrowFunctionParameters> {
        match self {
            AnyJsFunction::JsArrowFunctionExpression(expr) => expr.parameters(),
            AnyJsFunction::JsFunctionExpression(expr) => expr
                .parameters()
                .map(AnyJsArrowFunctionParameters::JsParameters),
            AnyJsFunction::JsFunctionDeclaration(declaration) => declaration
                .parameters()
                .map(AnyJsArrowFunctionParameters::JsParameters),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => declaration
                .parameters()
                .map(AnyJsArrowFunctionParameters::JsParameters),
        }
    }

    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            AnyJsFunction::JsArrowFunctionExpression(expr) => expr.return_type_annotation(),
            AnyJsFunction::JsFunctionExpression(expr) => expr.return_type_annotation(),
            AnyJsFunction::JsFunctionDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
        }
    }

    pub fn body(&self) -> SyntaxResult<AnyJsFunctionBody> {
        match self {
            AnyJsFunction::JsArrowFunctionExpression(expr) => expr.body(),
            AnyJsFunction::JsFunctionExpression(expr) => {
                expr.body().map(AnyJsFunctionBody::JsFunctionBody)
            }
            AnyJsFunction::JsFunctionDeclaration(declaration) => {
                declaration.body().map(AnyJsFunctionBody::JsFunctionBody)
            }
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.body().map(AnyJsFunctionBody::JsFunctionBody)
            }
        }
    }
}

impl AnyTsVariableAnnotation {
    pub fn type_annotation(&self) -> SyntaxResult<Option<TsTypeAnnotation>> {
        match self {
            AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(definite) => {
                definite.type_annotation().map(Some)
            }
            AnyTsVariableAnnotation::TsTypeAnnotation(type_annotation) => {
                Ok(Some(type_annotation.clone()))
            }
        }
    }
}

impl AnyTsPropertyAnnotation {
    pub fn type_annotation(&self) -> SyntaxResult<Option<TsTypeAnnotation>> {
        match self {
            AnyTsPropertyAnnotation::TsDefinitePropertyAnnotation(definite) => {
                definite.type_annotation().map(Some)
            }
            AnyTsPropertyAnnotation::TsOptionalPropertyAnnotation(optional) => {
                Ok(optional.type_annotation())
            }
            AnyTsPropertyAnnotation::TsTypeAnnotation(type_annotation) => {
                Ok(Some(type_annotation.clone()))
            }
        }
    }
}

impl AnyJsArrowFunctionParameters {
    pub fn len(&self) -> usize {
        match self {
            AnyJsArrowFunctionParameters::AnyJsBinding(_) => 1,
            AnyJsArrowFunctionParameters::JsParameters(parameters) => parameters.items().len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
