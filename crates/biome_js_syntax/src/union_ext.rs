use crate::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsClass, AnyJsClassMember, AnyJsClassMemberName,
    AnyJsFunction, AnyJsFunctionBody, AnyTsPropertyAnnotation, AnyTsPropertySignatureAnnotation,
    AnyTsVariableAnnotation, JsClassMemberList, JsDecoratorList, JsExtendsClause,
    JsInitializerClause, JsSyntaxToken, JsVariableDeclarator, TsImplementsClause,
    TsReturnTypeAnnotation, TsTypeAnnotation, TsTypeParameters,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxResult};

impl AnyJsClass {
    pub fn decorators(&self) -> JsDecoratorList {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.decorators(),
            Self::JsClassExpression(expression) => expression.decorators(),
            Self::JsClassExportDefaultDeclaration(declaration) => declaration.decorators(),
        }
    }

    pub fn abstract_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.abstract_token(),
            Self::JsClassExpression(_) => None,
            Self::JsClassExportDefaultDeclaration(clause) => clause.abstract_token(),
        }
    }

    pub fn class_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.class_token(),
            Self::JsClassExpression(expression) => expression.class_token(),
            Self::JsClassExportDefaultDeclaration(declaration) => declaration.class_token(),
        }
    }

    pub fn id(&self) -> Option<AnyJsBinding> {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.id().ok(),
            Self::JsClassExpression(expression) => expression.id(),
            Self::JsClassExportDefaultDeclaration(declaration) => declaration.id(),
        }
    }

    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.type_parameters(),
            Self::JsClassExpression(expression) => expression.type_parameters(),
            Self::JsClassExportDefaultDeclaration(clause) => clause.type_parameters(),
        }
    }

    pub fn extends_clause(&self) -> Option<JsExtendsClause> {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.extends_clause(),
            Self::JsClassExpression(expression) => expression.extends_clause(),
            Self::JsClassExportDefaultDeclaration(declaration) => declaration.extends_clause(),
        }
    }

    pub fn implements_clause(&self) -> Option<TsImplementsClause> {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.implements_clause(),
            Self::JsClassExpression(expression) => expression.implements_clause(),
            Self::JsClassExportDefaultDeclaration(declaration) => declaration.implements_clause(),
        }
    }

    pub fn l_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.l_curly_token(),
            Self::JsClassExpression(expression) => expression.l_curly_token(),
            Self::JsClassExportDefaultDeclaration(declaration) => declaration.l_curly_token(),
        }
    }

    pub fn members(&self) -> JsClassMemberList {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.members(),
            Self::JsClassExpression(expression) => expression.members(),
            Self::JsClassExportDefaultDeclaration(declaration) => declaration.members(),
        }
    }

    pub fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsClassDeclaration(declaration) => declaration.r_curly_token(),
            Self::JsClassExpression(expression) => expression.r_curly_token(),
            Self::JsClassExportDefaultDeclaration(declaration) => declaration.r_curly_token(),
        }
    }
}

impl AnyJsClassMember {
    /// Returns the `name` of the member if it has any.
    pub fn name(&self) -> SyntaxResult<Option<AnyJsClassMemberName>> {
        match self {
            Self::JsConstructorClassMember(constructor) => constructor
                .name()
                .map(|name| Some(AnyJsClassMemberName::from(name))),
            Self::JsEmptyClassMember(_) => Ok(None),
            Self::JsGetterClassMember(getter) => getter.name().map(Some),
            Self::JsMethodClassMember(method) => method.name().map(Some),
            Self::JsPropertyClassMember(property) => property.name().map(Some),
            Self::JsSetterClassMember(setter) => setter.name().map(Some),
            Self::JsStaticInitializationBlockClassMember(_) => Ok(None),
            Self::JsBogusMember(_) | Self::JsMetavariable(_) => Ok(None),
            Self::TsConstructorSignatureClassMember(constructor) => constructor
                .name()
                .map(|name| Some(AnyJsClassMemberName::from(name))),
            Self::TsGetterSignatureClassMember(getter) => getter.name().map(Some),
            Self::TsIndexSignatureClassMember(_) => Ok(None),
            Self::TsMethodSignatureClassMember(method) => method.name().map(Some),
            Self::TsPropertySignatureClassMember(property) => property.name().map(Some),
            Self::TsInitializedPropertySignatureClassMember(property) => property.name().map(Some),
            Self::TsSetterSignatureClassMember(setter) => setter.name().map(Some),
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
        matches!(self, Self::JsComputedMemberName(_))
    }
}

impl AnyJsFunction {
    pub fn async_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsArrowFunctionExpression(expr) => expr.async_token(),
            Self::JsFunctionExpression(expr) => expr.async_token(),
            Self::JsFunctionDeclaration(declaration) => declaration.async_token(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.async_token(),
        }
    }

    /// Returns the binding by which the function can be accessed.
    ///
    /// This may be a binding for the function's identifier, or a binding for
    /// the variable to which the function is assigned.
    pub fn binding(&self) -> Option<AnyJsBinding> {
        match self {
            Self::JsFunctionDeclaration(declaration) => declaration.id().ok(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.id(),
            Self::JsArrowFunctionExpression(_) | Self::JsFunctionExpression(_) => {
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
            Self::JsArrowFunctionExpression(_) => Ok(None),
            Self::JsFunctionExpression(expr) => expr.function_token().map(Some),
            Self::JsFunctionDeclaration(declaration) => declaration.function_token().map(Some),
            Self::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.function_token().map(Some)
            }
        }
    }

    pub fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsArrowFunctionExpression(_) => None,
            Self::JsFunctionExpression(expr) => expr.star_token(),
            Self::JsFunctionDeclaration(declaration) => declaration.star_token(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.star_token(),
        }
    }

    pub fn is_generator(&self) -> bool {
        self.star_token().is_some()
    }

    pub fn id(&self) -> SyntaxResult<Option<AnyJsBinding>> {
        match self {
            Self::JsArrowFunctionExpression(_) => Ok(None),
            Self::JsFunctionExpression(expr) => Ok(expr.id()),
            Self::JsFunctionDeclaration(declaration) => declaration.id().map(Some),
            Self::JsFunctionExportDefaultDeclaration(declaration) => Ok(declaration.id()),
        }
    }

    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            Self::JsArrowFunctionExpression(expr) => expr.type_parameters(),
            Self::JsFunctionExpression(expr) => expr.type_parameters(),
            Self::JsFunctionDeclaration(declaration) => declaration.type_parameters(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.type_parameters(),
        }
    }

    pub fn parameters(&self) -> SyntaxResult<AnyJsArrowFunctionParameters> {
        match self {
            Self::JsArrowFunctionExpression(expr) => expr.parameters(),
            Self::JsFunctionExpression(expr) => expr
                .parameters()
                .map(AnyJsArrowFunctionParameters::JsParameters),
            Self::JsFunctionDeclaration(declaration) => declaration
                .parameters()
                .map(AnyJsArrowFunctionParameters::JsParameters),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration
                .parameters()
                .map(AnyJsArrowFunctionParameters::JsParameters),
        }
    }

    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            Self::JsArrowFunctionExpression(expr) => expr.return_type_annotation(),
            Self::JsFunctionExpression(expr) => expr.return_type_annotation(),
            Self::JsFunctionDeclaration(declaration) => declaration.return_type_annotation(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
        }
    }

    pub fn body(&self) -> SyntaxResult<AnyJsFunctionBody> {
        match self {
            Self::JsArrowFunctionExpression(expr) => expr.body(),
            Self::JsFunctionExpression(expr) => expr.body().map(AnyJsFunctionBody::JsFunctionBody),
            Self::JsFunctionDeclaration(declaration) => {
                declaration.body().map(AnyJsFunctionBody::JsFunctionBody)
            }
            Self::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.body().map(AnyJsFunctionBody::JsFunctionBody)
            }
        }
    }
}

impl AnyTsVariableAnnotation {
    pub fn type_annotation(&self) -> SyntaxResult<Option<TsTypeAnnotation>> {
        match self {
            Self::TsDefiniteVariableAnnotation(definite) => definite.type_annotation().map(Some),
            Self::TsTypeAnnotation(type_annotation) => Ok(Some(type_annotation.clone())),
        }
    }
}

impl AnyTsPropertyAnnotation {
    pub fn type_annotation(&self) -> SyntaxResult<Option<TsTypeAnnotation>> {
        match self {
            Self::TsDefinitePropertyAnnotation(definite) => definite.type_annotation().map(Some),
            Self::TsOptionalPropertyAnnotation(optional) => Ok(optional.type_annotation()),
            Self::TsTypeAnnotation(type_annotation) => Ok(Some(type_annotation.clone())),
        }
    }
}

impl AnyTsPropertySignatureAnnotation {
    pub fn type_annotation(&self) -> SyntaxResult<Option<TsTypeAnnotation>> {
        match self {
            Self::TsOptionalPropertyAnnotation(optional) => Ok(optional.type_annotation()),
            Self::TsTypeAnnotation(annotation) => Ok(Some(annotation.clone())),
        }
    }
}

impl AnyJsArrowFunctionParameters {
    pub fn len(&self) -> usize {
        match self {
            Self::AnyJsBinding(_) => 1,
            Self::JsParameters(parameters) => parameters.items().len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
