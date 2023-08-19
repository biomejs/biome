use crate::{
    JsArrowFunctionExpression, JsBogusNamedImportSpecifier, JsBogusParameter, JsCatchDeclaration,
    JsClassDeclaration, JsClassExportDefaultDeclaration, JsClassExpression,
    JsConstructorClassMember, JsConstructorParameterList, JsConstructorParameters,
    JsDefaultImportSpecifier, JsFormalParameter, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsIdentifierBinding,
    JsImportDefaultClause, JsImportNamespaceClause, JsMethodClassMember, JsMethodObjectMember,
    JsNamedImportSpecifier, JsNamespaceImportSpecifier, JsParameterList, JsParameters,
    JsRestParameter, JsSetterClassMember, JsSetterObjectMember, JsShorthandNamedImportSpecifier,
    JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsVariableDeclarator, TsCallSignatureTypeMember,
    TsConstructSignatureTypeMember, TsConstructorSignatureClassMember, TsConstructorType,
    TsDeclareFunctionDeclaration, TsDeclareFunctionExportDefaultDeclaration, TsEnumDeclaration,
    TsFunctionType, TsIdentifierBinding, TsImportEqualsDeclaration, TsIndexSignatureClassMember,
    TsIndexSignatureParameter, TsInterfaceDeclaration, TsMethodSignatureClassMember,
    TsMethodSignatureTypeMember, TsModuleDeclaration, TsPropertyParameter,
    TsSetterSignatureClassMember, TsSetterSignatureTypeMember, TsTypeAliasDeclaration,
    TsTypeParameterName,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};

declare_node_union! {
    pub AnyJsBindingDeclaration =
        // variable
            JsVariableDeclarator
        // parameters
            | JsFormalParameter | JsRestParameter | JsBogusParameter
            | TsIndexSignatureParameter | TsPropertyParameter
        // functions
            | JsFunctionDeclaration | JsFunctionExpression
            | TsDeclareFunctionDeclaration
        // classes, objects, interface, type, enum, module
            | JsClassDeclaration | JsClassExpression
            | TsInterfaceDeclaration | TsTypeAliasDeclaration | TsEnumDeclaration | TsModuleDeclaration
        // import
            | JsImportDefaultClause | JsImportNamespaceClause | JsShorthandNamedImportSpecifier
                | JsNamedImportSpecifier | JsBogusNamedImportSpecifier | JsDefaultImportSpecifier
                | JsNamespaceImportSpecifier
            | TsImportEqualsDeclaration
        // export
            | JsClassExportDefaultDeclaration | JsFunctionExportDefaultDeclaration
            | TsDeclareFunctionExportDefaultDeclaration
        // try/catch
            | JsCatchDeclaration
}

impl AnyJsBindingDeclaration {
    /// Returns `true` if `self` and `other` are mergeable declarations.
    ///
    /// See also: <https://www.typescriptlang.org/docs/handbook/declaration-merging.html>
    ///
    /// ## Examples
    ///
    /// A namespace can merge with a class, an enum.
    /// However, an enum cannot merge with a class.
    ///
    /// ```
    /// use rome_js_factory::make;
    /// use rome_js_syntax::{binding_ext::AnyJsBindingDeclaration, T};
    ///
    /// let enum_id = make::js_identifier_binding(make::ident("Order"));
    /// let enum_decl: AnyJsBindingDeclaration = make::ts_enum_declaration(
    ///     make::token(T![enum]),
    ///     enum_id.into(),
    ///     make::token(T!['{']),
    ///     make::ts_enum_member_list(
    ///         [],
    ///         Some(make::token(T![;])),
    ///     ),
    ///     make::token(T!['}']),
    /// ).build().into();
    ///
    /// let namespace_id = make::ts_identifier_binding(make::ident("Order"));
    /// let namespace_decl: AnyJsBindingDeclaration = make::ts_module_declaration(
    ///     make::token(T![namespace]),
    ///     namespace_id.into(),
    ///     make::ts_module_block(
    ///         make::token(T!['{']),
    ///         make::js_module_item_list([]),
    ///         make::token(T!['}']),
    ///     ),
    /// ).into();
    ///
    /// let class_id = make::js_identifier_binding(make::ident("Order"));
    /// let class_decl: AnyJsBindingDeclaration = make::js_class_declaration(
    ///     make::js_decorator_list([]),
    ///     make::token(T![class]),
    ///     class_id.into(),
    ///     make::token(T!['{']),
    ///     make::js_class_member_list([]),
    ///     make::token(T!['}']),
    /// ).build().into();
    ///
    /// assert!(enum_decl.is_mergeable(&namespace_decl));
    /// assert!(namespace_decl.is_mergeable(&enum_decl));
    ///
    /// assert!(class_decl.is_mergeable(&namespace_decl));
    /// assert!(namespace_decl.is_mergeable(&class_decl));
    ///
    /// assert!(!class_decl.is_mergeable(&enum_decl));
    /// assert!(!enum_decl.is_mergeable(&class_decl));
    /// ```
    pub const fn is_mergeable(&self, other: &AnyJsBindingDeclaration) -> bool {
        Self::can_merge(self, other) || Self::can_merge(other, self)
    }

    /// Please use `is_mergeable`.
    /// `can_merge` is sensible to the order of arguments.
    const fn can_merge(a: &AnyJsBindingDeclaration, b: &AnyJsBindingDeclaration) -> bool {
        match (a, b) {
            (
                AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_),
                AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_),
            ) => true,
            (
                AnyJsBindingDeclaration::TsEnumDeclaration(_),
                AnyJsBindingDeclaration::TsEnumDeclaration(_),
            ) => true,
            (
                AnyJsBindingDeclaration::TsTypeAliasDeclaration(_),
                AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                | AnyJsBindingDeclaration::JsVariableDeclarator(_)
                | AnyJsBindingDeclaration::TsModuleDeclaration(_),
            ) => true,
            (
                AnyJsBindingDeclaration::TsInterfaceDeclaration(_),
                AnyJsBindingDeclaration::JsClassDeclaration(_)
                | AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                | AnyJsBindingDeclaration::JsVariableDeclarator(_)
                | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
                | AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
                | AnyJsBindingDeclaration::TsModuleDeclaration(_),
            ) => true,
            (
                AnyJsBindingDeclaration::TsModuleDeclaration(_),
                AnyJsBindingDeclaration::JsClassDeclaration(_)
                | AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                | AnyJsBindingDeclaration::JsVariableDeclarator(_)
                | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
                | AnyJsBindingDeclaration::TsEnumDeclaration(_)
                | AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
                | AnyJsBindingDeclaration::TsModuleDeclaration(_),
            ) => true,
            (_, _) => false,
        }
    }

    /// Returns `true` if `self` is a formal parameter, a rest parameter,
    /// a property parameter, or a bogus parameter.
    pub const fn is_parameter_like(&self) -> bool {
        matches!(
            self,
            AnyJsBindingDeclaration::JsFormalParameter(_)
                | AnyJsBindingDeclaration::JsRestParameter(_)
                | AnyJsBindingDeclaration::JsBogusParameter(_)
                | AnyJsBindingDeclaration::TsPropertyParameter(_)
        )
    }
}

declare_node_union! {
    pub AnyJsIdentifierBinding = JsIdentifierBinding | TsIdentifierBinding | TsTypeParameterName
}

fn declaration(node: &JsSyntaxNode) -> Option<AnyJsBindingDeclaration> {
    use JsSyntaxKind::*;
    let parent = node.parent()?;
    let possible_declarator = parent.ancestors().find(|x| {
        !matches!(
            x.kind(),
            JS_BINDING_PATTERN_WITH_DEFAULT
                | JS_OBJECT_BINDING_PATTERN
                | JS_OBJECT_BINDING_PATTERN_REST
                | JS_OBJECT_BINDING_PATTERN_PROPERTY
                | JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
                | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
                | JS_ARRAY_BINDING_PATTERN
                | JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
                | JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
        )
    })?;

    match AnyJsBindingDeclaration::cast(possible_declarator)? {
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => {
            match parameter.parent::<TsPropertyParameter>() {
                Some(parameter) => Some(AnyJsBindingDeclaration::TsPropertyParameter(parameter)),
                None => Some(AnyJsBindingDeclaration::JsFormalParameter(parameter)),
            }
        }
        declaration => Some(declaration),
    }
}

fn is_under_pattern_binding(node: &JsSyntaxNode) -> Option<bool> {
    use JsSyntaxKind::*;
    Some(matches!(
        node.parent()?.kind(),
        JS_BINDING_PATTERN_WITH_DEFAULT
            | JS_OBJECT_BINDING_PATTERN
            | JS_OBJECT_BINDING_PATTERN_REST
            | JS_OBJECT_BINDING_PATTERN_PROPERTY
            | JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
            | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
            | JS_ARRAY_BINDING_PATTERN
            | JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
            | JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
    ))
}

fn is_under_array_pattern_binding(node: &JsSyntaxNode) -> Option<bool> {
    use JsSyntaxKind::*;
    let parent = node.parent()?;
    match parent.kind() {
        JS_ARRAY_BINDING_PATTERN
        | JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
        | JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => Some(true),
        JS_BINDING_PATTERN_WITH_DEFAULT => is_under_array_pattern_binding(&parent),
        _ => Some(false),
    }
}

fn is_under_object_pattern_binding(node: &JsSyntaxNode) -> Option<bool> {
    use JsSyntaxKind::*;
    let parent = node.parent()?;
    match parent.kind() {
        JS_OBJECT_BINDING_PATTERN
        | JS_OBJECT_BINDING_PATTERN_REST
        | JS_OBJECT_BINDING_PATTERN_PROPERTY
        | JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
        | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => Some(true),
        JS_BINDING_PATTERN_WITH_DEFAULT => is_under_object_pattern_binding(&parent),
        _ => Some(false),
    }
}

impl AnyJsIdentifierBinding {
    pub fn name_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsIdentifierBinding::JsIdentifierBinding(binding) => binding.name_token(),
            AnyJsIdentifierBinding::TsIdentifierBinding(binding) => binding.name_token(),
            AnyJsIdentifierBinding::TsTypeParameterName(binding) => binding.ident_token(),
        }
    }

    pub fn declaration(&self) -> Option<AnyJsBindingDeclaration> {
        let node = match self {
            AnyJsIdentifierBinding::JsIdentifierBinding(binding) => &binding.syntax,
            AnyJsIdentifierBinding::TsIdentifierBinding(binding) => &binding.syntax,
            AnyJsIdentifierBinding::TsTypeParameterName(binding) => &binding.syntax,
        };
        declaration(node)
    }

    pub fn is_under_pattern_binding(&self) -> Option<bool> {
        is_under_pattern_binding(self.syntax())
    }

    pub fn is_under_array_pattern_binding(&self) -> Option<bool> {
        is_under_array_pattern_binding(self.syntax())
    }

    pub fn is_under_object_pattern_binding(&self) -> Option<bool> {
        is_under_object_pattern_binding(self.syntax())
    }

    pub fn with_name_token(self, name_token: JsSyntaxToken) -> AnyJsIdentifierBinding {
        match self {
            AnyJsIdentifierBinding::JsIdentifierBinding(binding) => {
                AnyJsIdentifierBinding::JsIdentifierBinding(binding.with_name_token(name_token))
            }
            AnyJsIdentifierBinding::TsIdentifierBinding(binding) => {
                AnyJsIdentifierBinding::TsIdentifierBinding(binding.with_name_token(name_token))
            }
            AnyJsIdentifierBinding::TsTypeParameterName(binding) => {
                AnyJsIdentifierBinding::TsTypeParameterName(binding.with_ident_token(name_token))
            }
        }
    }
}

impl JsIdentifierBinding {
    /// Navigate upward until the declaration of this binding bypassing all nodes
    /// related to pattern binding.
    pub fn declaration(&self) -> Option<AnyJsBindingDeclaration> {
        declaration(&self.syntax)
    }

    pub fn is_under_pattern_binding(&self) -> Option<bool> {
        is_under_pattern_binding(self.syntax())
    }

    pub fn is_under_array_pattern_binding(&self) -> Option<bool> {
        is_under_array_pattern_binding(self.syntax())
    }

    pub fn is_under_object_pattern_binding(&self) -> Option<bool> {
        is_under_object_pattern_binding(self.syntax())
    }
}

impl TsIdentifierBinding {
    pub fn declaration(&self) -> Option<AnyJsBindingDeclaration> {
        declaration(&self.syntax)
    }

    pub fn is_under_pattern_binding(&self) -> Option<bool> {
        is_under_pattern_binding(self.syntax())
    }

    pub fn is_under_array_pattern_binding(&self) -> Option<bool> {
        is_under_array_pattern_binding(self.syntax())
    }

    pub fn is_under_object_pattern_binding(&self) -> Option<bool> {
        is_under_object_pattern_binding(self.syntax())
    }
}

declare_node_union! {
    pub JsAnyParameterParentFunction =
        JsFunctionDeclaration
        | JsFunctionExpression
        | JsArrowFunctionExpression
        | JsFunctionExportDefaultDeclaration

        | JsConstructorClassMember
        | JsMethodClassMember
        | JsSetterClassMember

        | JsMethodObjectMember
        | JsSetterObjectMember

        | TsFunctionType
        | TsConstructorType

        | TsDeclareFunctionDeclaration
        | TsDeclareFunctionExportDefaultDeclaration

        | TsConstructorSignatureClassMember
        | TsMethodSignatureClassMember
        | TsSetterSignatureClassMember
        | TsIndexSignatureClassMember

        | TsConstructSignatureTypeMember
        | TsMethodSignatureTypeMember
        | TsSetterSignatureTypeMember
        | TsCallSignatureTypeMember
}

fn parent_function(node: &JsSyntaxNode) -> Option<JsAnyParameterParentFunction> {
    let parent = node.parent()?;

    match parent.kind() {
        JsSyntaxKind::JS_PARAMETER_LIST => {
            // SAFETY: kind check above
            let parameters = JsParameterList::unwrap_cast(parent).parent::<JsParameters>()?;
            let parent = parameters.syntax.parent()?;
            JsAnyParameterParentFunction::cast(parent)
        }
        JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST => {
            // SAFETY: kind check above
            let parameters = JsConstructorParameterList::unwrap_cast(parent)
                .parent::<JsConstructorParameters>()?;
            let parent = parameters.syntax().parent()?;
            JsAnyParameterParentFunction::cast(parent)
        }
        _ => JsAnyParameterParentFunction::cast(parent),
    }
}

impl JsFormalParameter {
    pub fn parent_function(&self) -> Option<JsAnyParameterParentFunction> {
        parent_function(&self.syntax)
    }
}

impl JsRestParameter {
    pub fn parent_function(&self) -> Option<JsAnyParameterParentFunction> {
        parent_function(&self.syntax)
    }
}

impl TsPropertyParameter {
    pub fn parent_function(&self) -> Option<JsAnyParameterParentFunction> {
        parent_function(&self.syntax)
    }
}
