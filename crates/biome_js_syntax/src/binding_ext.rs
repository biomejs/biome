use crate::{
    AnyJsImportClause, AnyJsNamedImportSpecifier, JsArrowFunctionExpression,
    JsBogusNamedImportSpecifier, JsBogusParameter, JsCatchDeclaration, JsClassDeclaration,
    JsClassExportDefaultDeclaration, JsClassExpression, JsConstructorClassMember,
    JsConstructorParameterList, JsConstructorParameters, JsDefaultImportSpecifier, JsExport,
    JsFormalParameter, JsFunctionDeclaration, JsFunctionExportDefaultDeclaration,
    JsFunctionExpression, JsIdentifierBinding, JsMethodClassMember, JsMethodObjectMember,
    JsNamedImportSpecifier, JsNamespaceImportSpecifier, JsParameterList, JsParameters,
    JsRestParameter, JsSetterClassMember, JsSetterObjectMember, JsShorthandNamedImportSpecifier,
    JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsVariableDeclarator, TsCallSignatureTypeMember,
    TsConstructSignatureTypeMember, TsConstructorSignatureClassMember, TsConstructorType,
    TsDeclareFunctionDeclaration, TsDeclareFunctionExportDefaultDeclaration, TsEnumDeclaration,
    TsFunctionType, TsIdentifierBinding, TsImportEqualsDeclaration, TsIndexSignatureClassMember,
    TsIndexSignatureParameter, TsInferType, TsInterfaceDeclaration, TsMappedType,
    TsMethodSignatureClassMember, TsMethodSignatureTypeMember, TsModuleDeclaration,
    TsPropertyParameter, TsSetterSignatureClassMember, TsSetterSignatureTypeMember,
    TsTypeAliasDeclaration, TsTypeParameter, TsTypeParameterName,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

declare_node_union! {
    pub AnyJsBindingDeclaration =
        // variable
            JsVariableDeclarator
        // parameters
            | JsArrowFunctionExpression | JsFormalParameter | JsRestParameter | JsBogusParameter
            | TsIndexSignatureParameter | TsPropertyParameter
        // type parameter
            | TsInferType | TsMappedType | TsTypeParameter
        // functions
            | JsFunctionDeclaration | JsFunctionExpression
            | TsDeclareFunctionDeclaration
        // classes, objects, interface, type, enum, module
            | JsClassDeclaration | JsClassExpression
            | TsInterfaceDeclaration | TsTypeAliasDeclaration | TsEnumDeclaration | TsModuleDeclaration
        // import
            | JsShorthandNamedImportSpecifier
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
    /// use biome_js_factory::make;
    /// use biome_js_syntax::{binding_ext::AnyJsBindingDeclaration, T};
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
                AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_),
                AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
                | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_),
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
            AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
                | AnyJsBindingDeclaration::JsFormalParameter(_)
                | AnyJsBindingDeclaration::JsRestParameter(_)
                | AnyJsBindingDeclaration::JsBogusParameter(_)
                | AnyJsBindingDeclaration::TsPropertyParameter(_)
        )
    }

    /// Returns the export statement if this declaration is directly exported.
    pub fn export(&self) -> Option<JsExport> {
        let maybe_export = match self {
            Self::JsVariableDeclarator(_) => self.syntax().ancestors().nth(4),
            Self::JsFunctionDeclaration(_)
            | Self::JsClassDeclaration(_)
            | Self::TsTypeAliasDeclaration(_)
            | Self::TsEnumDeclaration(_)
            | Self::TsModuleDeclaration(_) => self.syntax().parent(),
            Self::TsInterfaceDeclaration(_) => {
                // interfaces can be in a default export clause
                // `export default interface I {}`
                self.syntax()
                    .ancestors()
                    .skip(1)
                    .find(|x| x.kind() != JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)
            }
            Self::JsClassExportDefaultDeclaration(_)
            | Self::JsFunctionExportDefaultDeclaration(_)
            | Self::TsDeclareFunctionDeclaration(_)
            | Self::TsDeclareFunctionExportDefaultDeclaration(_) => self.syntax().grand_parent(),
            _ => None,
        };
        maybe_export.and_then(JsExport::cast)
    }
}

declare_node_union! {
    pub AnyJsIdentifierBinding = JsIdentifierBinding | TsIdentifierBinding | TsTypeParameterName
}

fn declaration(node: &JsSyntaxNode) -> Option<AnyJsBindingDeclaration> {
    let possible_declarator = node.ancestors().skip(1).find(|x| {
        !matches!(
            x.kind(),
            JsSyntaxKind::JS_BINDING_PATTERN_WITH_DEFAULT
                | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN
                | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST
                | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY
                | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
                | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
                | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN
                | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
                | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
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
        declaration(self.syntax())
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

    /// Returns true if this binding is only a type and not a runtime value.
    pub fn is_type_only(&self) -> bool {
        match self {
            AnyJsIdentifierBinding::JsIdentifierBinding(binding) => {
                if let Some(specifier) = binding.parent::<AnyJsNamedImportSpecifier>() {
                    return specifier.imports_only_types();
                }
                if let Some(clause) = binding
                    .syntax()
                    .grand_parent()
                    .and_then(AnyJsImportClause::cast)
                {
                    return clause.type_token().is_some();
                }
            }
            AnyJsIdentifierBinding::TsIdentifierBinding(binding) => {
                // ignore TypeScript namespaces
                return binding.parent::<TsModuleDeclaration>().is_none();
            }
            AnyJsIdentifierBinding::TsTypeParameterName(_) => {}
        }
        false
    }

    pub fn with_name_token(self, name_token: JsSyntaxToken) -> Self {
        match self {
            Self::JsIdentifierBinding(binding) => {
                Self::JsIdentifierBinding(binding.with_name_token(name_token))
            }
            Self::TsIdentifierBinding(binding) => {
                Self::TsIdentifierBinding(binding.with_name_token(name_token))
            }
            Self::TsTypeParameterName(binding) => {
                Self::TsTypeParameterName(binding.with_ident_token(name_token))
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
