//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use biome_js_syntax::binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding};
use biome_js_syntax::{
    AnyJsIdentifierUsage, JsLanguage, JsSyntaxKind, JsSyntaxNode, TextRange, TsTypeParameterName,
};
use biome_js_syntax::{AnyJsImportClause, AnyJsNamedImportSpecifier, AnyTsType};
use biome_rowan::{syntax::Preorder, AstNode, SyntaxNodeOptionExt, TokenText};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::mem;
use JsSyntaxKind::*;

/// Events emitted by the [SemanticEventExtractor].
/// These events are later made into the Semantic Model.
#[derive(Debug, Eq, PartialEq)]
pub enum SemanticEvent {
    /// Tracks where a new symbol declaration is found.
    /// Generated for:
    /// - Variable Declarations
    /// - Import bindings
    /// - Functions parameters
    /// - Type parameters
    DeclarationFound {
        range: TextRange,
        scope_id: usize,
        hoisted_scope_id: Option<usize>,
    },

    /// Tracks where a symbol is read, but only if its declaration is before this reference.
    /// Generated for:
    /// - All reference identifiers
    Read {
        range: TextRange,
        declared_at: TextRange,
        scope_id: usize,
    },

    /// Tracks where a symbol is read, but only if its declaration was hoisted.
    /// This means that its declaration is after this reference.
    /// - All reference identifiers
    HoistedRead {
        range: TextRange,
        declared_at: TextRange,
        scope_id: usize,
    },

    /// Tracks where a symbol is written, but only if its declaration is before this reference.
    /// Generated for:
    /// - All identifier assignments
    Write {
        range: TextRange,
        declared_at: TextRange,
        scope_id: usize,
    },

    /// Tracks where a symbol is written, but only if its declaration was hoisted.
    /// This means that its declaration is after this reference.
    /// Generated for:
    /// - All identifier assignments
    HoistedWrite {
        range: TextRange,
        declared_at: TextRange,
        scope_id: usize,
    },

    /// Tracks references that do no have any matching binding
    /// Generated for:
    /// - Unmatched reference identifiers
    UnresolvedReference { is_read: bool, range: TextRange },

    /// Tracks where a new scope starts
    /// Generated for:
    /// - Blocks
    /// - Function body
    ScopeStarted {
        /// Scope range
        range: TextRange,
        scope_id: usize,
        parent_scope_id: Option<usize>,
        is_closure: bool,
    },

    /// Tracks where a scope ends
    /// Generated for:
    /// - Blocks
    /// - Function body
    ScopeEnded {
        /// Scope range
        range: TextRange,
        scope_id: usize,
    },

    /// Tracks where a symbol is exported.
    /// The range points to the binding that is being exported.
    Exported { range: TextRange },
}

impl SemanticEvent {
    pub fn range(&self) -> TextRange {
        match self {
            Self::DeclarationFound { range, .. }
            | Self::ScopeStarted { range, .. }
            | Self::ScopeEnded { range, .. }
            | Self::Read { range, .. }
            | Self::HoistedRead { range, .. }
            | Self::Write { range, .. }
            | Self::HoistedWrite { range, .. }
            | Self::UnresolvedReference { range, .. }
            | Self::Exported { range } => *range,
        }
    }
}

/// Extracts [SemanticEvent] from [JsSyntaxNode].
///
/// The extraction is not entirely pull based, nor entirely push based.
/// This happens because some nodes can generate multiple events.
/// A hoisted variable declaration like `var a`, being the more obvious
/// example. As soon `a` is hoisted, all references of `a` are solved
/// on this node.
///
/// For a simpler way to extract [SemanticEvent] see [semantic_events].
///
/// To use the [SemanticEventExtractor] one must push the current node, following
/// the pre-order of the tree, and must pull events until `pop` returns [None].
///
/// ```rust
/// use biome_js_parser::*;
/// use biome_js_syntax::*;
/// use biome_js_semantic::*;
/// let tree = parse("let a = 1", JsFileSource::js_script(), JsParserOptions::default());
/// let mut extractor = SemanticEventExtractor::new();
/// for e in tree.syntax().preorder() {
///     match e {
///         WalkEvent::Enter(node) => extractor.enter(&node),
///         WalkEvent::Leave(node) => extractor.leave(&node),
///         _ => {}
///     }
///
///     while let Some(e) = extractor.pop() {
///         dbg!(e);
///     }
/// }
/// ```
#[derive(Default, Debug)]
pub struct SemanticEventExtractor {
    /// Event queue
    stash: VecDeque<SemanticEvent>,
    /// Stack of scopes
    scopes: Vec<Scope>,
    /// Number of generated scopes
    /// This allows assigning a unique scope id to every scope.
    scope_count: usize,
    /// At any point this is the set of available bindings and their range in the current scope
    bindings: FxHashMap<BindingName, BindingInfo>,
    /// Type parameters bound in a `infer T` clause.
    infers: Vec<TsTypeParameterName>,
}

/// A binding name is either a type or a value.
///
/// Two bindings (a type and a value bindings) can be associated to the same range.
/// This represents a declaration that is both a type and a value.
/// For example, in TypeScript a class and a enum are both a type and a value.
/// Allocating two bindings allows to for properly detecting type and value shadowing in inner scopes.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum BindingName {
    Type(TokenText),
    Value(TokenText),
}
impl BindingName {
    /// Turn a type into a value and a value into a type.
    fn dual(self) -> Self {
        match self {
            Self::Type(name) => Self::Value(name),
            Self::Value(name) => Self::Type(name),
        }
    }
}

#[derive(Debug, Clone)]
struct BindingInfo {
    /// range of the name
    range: TextRange,
    /// Kind of the declaration,
    /// or in the acse of a bogus declaration, the kind of the name
    declaration_kind: JsSyntaxKind,
}

impl BindingInfo {
    fn new(range: TextRange, declaration_kind: JsSyntaxKind) -> Self {
        Self {
            range,
            declaration_kind,
        }
    }

    fn is_imported(&self) -> bool {
        matches!(
            self.declaration_kind,
            JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION
                | JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER
                | JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER
                | JsSyntaxKind::JS_BOGUS_NAMED_IMPORT_SPECIFIER
                | JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
                | JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER
        )
    }
}

/// This type allows reporting a reference and bind to a binding (if any) later.
/// The range is the range of the referenced binding.
#[derive(Debug, Clone)]
enum Reference {
    /// Read and export a type, a value, or both.
    /// ```js
    /// export { A }
    /// ```
    Export(TextRange),
    /// Read a value or a type.
    /// ```js
    /// f();
    /// a;
    /// ```
    Read(TextRange),
    /// Read a value, or read an imported value as a type (`import type { Value }`).
    /// ```ts
    /// import type Y from ""
    /// typeof Y;
    /// const X = 0;
    /// typeof X;
    ///
    /// type T = {[X]: number, [Y]: number };
    ///
    /// namespace A { type B = number; }
    /// let a: A.B = 1;
    /// ```
    AmbientRead(TextRange),
    /// Assignment
    /// ```js
    /// a = 0;
    /// a += 1;
    /// ```
    Write(TextRange),
}

impl Reference {
    const fn is_write(&self) -> bool {
        matches!(self, Self::Write { .. })
    }

    /// Range of the referenced binding
    const fn range(&self) -> &TextRange {
        match self {
            Self::Export(range)
            | Self::Read(range)
            | Self::AmbientRead(range)
            | Self::Write(range) => range,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ScopeHoisting {
    DontHoistDeclarationsToParent,
    HoistDeclarationsToParent,
}

#[derive(Debug)]
struct Scope {
    scope_id: usize,
    /// All bindings declared inside this scope
    bindings: Vec<BindingName>,
    /// References that still needs to be bound and will be solved at the end of the scope
    references: FxHashMap<BindingName, Vec<Reference>>,
    /// All bindings that where shadowed and will be restored after this scope ends.
    shadowed: Vec<(BindingName, BindingInfo)>,
    /// If this scope allows declarations to be hoisted to parent scope or not
    hoisting: ScopeHoisting,
}

impl SemanticEventExtractor {
    pub fn new() -> Self {
        Self {
            stash: VecDeque::new(),
            scopes: vec![],
            scope_count: 0,
            bindings: FxHashMap::default(),
            infers: vec![],
        }
    }

    /// See [SemanticEvent] for a more detailed description of which events [JsSyntaxNode] generates.
    #[inline]
    pub fn enter(&mut self, node: &JsSyntaxNode) {
        // If you push a scope for a given node type, don't forget to also update `Self::leave`.
        // You should also edit [SemanticModelBuilder::push_node].
        match node.kind() {
            JS_IDENTIFIER_BINDING | TS_IDENTIFIER_BINDING | TS_TYPE_PARAMETER_NAME => {
                self.enter_identifier_binding(&AnyJsIdentifierBinding::unwrap_cast(node.clone()));
            }

            JS_REFERENCE_IDENTIFIER | JSX_REFERENCE_IDENTIFIER | JS_IDENTIFIER_ASSIGNMENT => {
                self.enter_identifier_usage(AnyJsIdentifierUsage::unwrap_cast(node.clone()));
            }

            JS_MODULE | JS_SCRIPT => self.push_scope(
                node.text_range(),
                ScopeHoisting::DontHoistDeclarationsToParent,
                false,
            ),

            JS_FUNCTION_DECLARATION
            | JS_FUNCTION_EXPRESSION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_CONSTRUCTOR_CLASS_MEMBER
            | JS_METHOD_CLASS_MEMBER
            | JS_GETTER_CLASS_MEMBER
            | JS_SETTER_CLASS_MEMBER
            | JS_METHOD_OBJECT_MEMBER
            | JS_GETTER_OBJECT_MEMBER
            | JS_SETTER_OBJECT_MEMBER => {
                self.push_scope(
                    node.text_range(),
                    ScopeHoisting::DontHoistDeclarationsToParent,
                    true,
                );
            }

            JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | JS_CLASS_DECLARATION
            | JS_CLASS_EXPORT_DEFAULT_DECLARATION
            | JS_CLASS_EXPRESSION
            | JS_FUNCTION_BODY
            | JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
            | TS_MODULE_DECLARATION
            | TS_EXTERNAL_MODULE_DECLARATION
            | TS_INTERFACE_DECLARATION
            | TS_ENUM_DECLARATION
            | TS_TYPE_ALIAS_DECLARATION
            | TS_DECLARE_FUNCTION_DECLARATION
            | TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | TS_CALL_SIGNATURE_TYPE_MEMBER
            | TS_METHOD_SIGNATURE_CLASS_MEMBER
            | TS_METHOD_SIGNATURE_TYPE_MEMBER
            | TS_INDEX_SIGNATURE_CLASS_MEMBER
            | TS_INDEX_SIGNATURE_TYPE_MEMBER => {
                self.push_scope(
                    node.text_range(),
                    ScopeHoisting::DontHoistDeclarationsToParent,
                    false,
                );
            }

            JS_BLOCK_STATEMENT | JS_FOR_STATEMENT | JS_FOR_OF_STATEMENT | JS_FOR_IN_STATEMENT
            | JS_SWITCH_STATEMENT | JS_CATCH_CLAUSE => {
                self.push_scope(
                    node.text_range(),
                    ScopeHoisting::HoistDeclarationsToParent,
                    false,
                );
            }

            _ => {
                if let Some(node) = AnyTsType::cast_ref(node) {
                    self.enter_any_type(&node);
                }
            }
        }
    }

    fn enter_any_type(&mut self, node: &AnyTsType) {
        if node.in_conditional_true_type() {
            self.push_scope(
                node.syntax().text_range(),
                ScopeHoisting::DontHoistDeclarationsToParent,
                false,
            );
            self.push_infers_in_scope();
            return;
        }
        let node = node.syntax();
        if matches!(
            node.kind(),
            JsSyntaxKind::TS_FUNCTION_TYPE | JsSyntaxKind::TS_MAPPED_TYPE
        ) {
            self.push_scope(
                node.text_range(),
                ScopeHoisting::DontHoistDeclarationsToParent,
                false,
            );
        }
    }

    fn enter_identifier_binding(&mut self, node: &AnyJsIdentifierBinding) {
        let mut hoisted_scope_id = None;
        let is_exported = if let Ok(name_token) = node.name_token() {
            let name = name_token.token_text_trimmed();
            if let Some(declaration) = node.declaration() {
                let info = BindingInfo::new(name_token.text_range(), declaration.syntax().kind());
                let is_exported = declaration.export().is_some();
                match declaration {
                    AnyJsBindingDeclaration::JsArrayBindingPatternElement(_)
                    | AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(_)
                    | AnyJsBindingDeclaration::JsObjectBindingPatternProperty(_)
                    | AnyJsBindingDeclaration::JsObjectBindingPatternRest(_)
                    | AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(_) => {
                        if let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
                            declaration.parent_binding_pattern_declaration()
                        {
                            if declarator.declaration().map_or(false, |x| x.is_var()) {
                                hoisted_scope_id = self.scope_index_to_hoist_declarations(0)
                            }
                        }
                        self.push_binding(hoisted_scope_id, BindingName::Value(name), info);
                    }
                    AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                        if declarator.declaration().map_or(false, |x| x.is_var()) {
                            hoisted_scope_id = self.scope_index_to_hoist_declarations(0)
                        }
                        self.push_binding(hoisted_scope_id, BindingName::Value(name), info);
                    }
                    AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
                    | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
                    | AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                    | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_) => {
                        hoisted_scope_id = self.scope_index_to_hoist_declarations(1);
                        self.push_binding(hoisted_scope_id, BindingName::Value(name), info);
                    }
                    AnyJsBindingDeclaration::JsClassExpression(_)
                    | AnyJsBindingDeclaration::JsFunctionExpression(_) => {
                        self.push_binding(None, BindingName::Value(name.clone()), info.clone());
                        self.push_binding(None, BindingName::Type(name), info);
                    }
                    AnyJsBindingDeclaration::JsClassDeclaration(_)
                    | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_)
                    | AnyJsBindingDeclaration::TsEnumDeclaration(_) => {
                        // These declarations have their own scope.
                        // Thus we need to hoist the declaration to the parent scope.
                        hoisted_scope_id = self
                            .scopes
                            .get(self.scopes.len() - 2)
                            .map(|scope| scope.scope_id);
                        self.push_binding(
                            hoisted_scope_id,
                            BindingName::Value(name.clone()),
                            info.clone(),
                        );
                        self.push_binding(hoisted_scope_id, BindingName::Type(name), info);
                    }
                    AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
                    | AnyJsBindingDeclaration::TsTypeAliasDeclaration(_) => {
                        // These declarations have their own scope.
                        // Thus we need to hoist the declaration to the parent scope.
                        hoisted_scope_id = self
                            .scopes
                            .get(self.scopes.len() - 2)
                            .map(|scope| scope.scope_id);
                        self.push_binding(hoisted_scope_id, BindingName::Type(name), info);
                    }
                    AnyJsBindingDeclaration::TsModuleDeclaration(_) => {
                        // This declarations has its own scope.
                        // Thus we need to hoist the declaration to the parent scope.
                        hoisted_scope_id = self
                            .scopes
                            .get(self.scopes.len() - 2)
                            .map(|scope| scope.scope_id);
                        self.push_binding(hoisted_scope_id, BindingName::Value(name.clone()), info);
                    }
                    AnyJsBindingDeclaration::TsMappedType(_)
                    | AnyJsBindingDeclaration::TsTypeParameter(_) => {
                        self.push_binding(None, BindingName::Type(name), info);
                    }
                    AnyJsBindingDeclaration::TsImportEqualsDeclaration(declaration) => {
                        if declaration.type_token().is_none() {
                            self.push_binding(None, BindingName::Value(name.clone()), info.clone());
                        }
                        self.push_binding(None, BindingName::Type(name), info);
                    }
                    AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => {
                        let type_token = declaration
                            .parent::<AnyJsImportClause>()
                            .and_then(|clause| clause.type_token());
                        if type_token.is_none() {
                            self.push_binding(None, BindingName::Value(name.clone()), info.clone());
                        }
                        self.push_binding(None, BindingName::Type(name), info);
                    }
                    AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => {
                        let type_token = declaration
                            .parent::<AnyJsImportClause>()
                            .and_then(|clause| clause.type_token());
                        if type_token.is_none() {
                            self.push_binding(None, BindingName::Value(name.clone()), info.clone());
                        } else {
                            self.push_binding(None, BindingName::Type(name), info);
                        }
                    }
                    AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
                    | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
                    | AnyJsBindingDeclaration::JsNamedImportSpecifier(_) => {
                        let specifier =
                            AnyJsNamedImportSpecifier::unwrap_cast(declaration.into_syntax());
                        if !specifier.imports_only_types() {
                            self.push_binding(None, BindingName::Value(name.clone()), info.clone());
                        }
                        self.push_binding(None, BindingName::Type(name), info);
                    }
                    AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
                    | AnyJsBindingDeclaration::JsBogusParameter(_)
                    | AnyJsBindingDeclaration::JsFormalParameter(_)
                    | AnyJsBindingDeclaration::JsRestParameter(_)
                    | AnyJsBindingDeclaration::TsIndexSignatureParameter(_)
                    | AnyJsBindingDeclaration::TsPropertyParameter(_)
                    | AnyJsBindingDeclaration::JsCatchDeclaration(_) => {
                        self.push_binding(None, BindingName::Value(name), info);
                    }
                    AnyJsBindingDeclaration::TsInferType(_) => {
                        // Delay the declaration of parameter types that are inferred.
                        // Their scope corresponds to the true branch of the conditional type.
                        self.infers
                            .push(TsTypeParameterName::unwrap_cast(node.syntax().clone()));
                        return;
                    }
                }
                is_exported
            } else {
                // Handle identifiers in bogus nodes
                let info = BindingInfo::new(name_token.text_range(), node.syntax().kind());
                self.push_binding(None, BindingName::Value(name), info);
                false
            }
        } else {
            // The binding has a bogus name
            false
        };
        let scope_id = self.current_scope_mut().scope_id;
        self.stash.push_back(SemanticEvent::DeclarationFound {
            scope_id,
            hoisted_scope_id,
            range: node.syntax().text_range(),
        });
        if is_exported {
            self.stash.push_back(SemanticEvent::Exported {
                range: node.syntax().text_range(),
            });
        }
    }

    fn enter_identifier_usage(&mut self, node: AnyJsIdentifierUsage) {
        let range = node.syntax().text_range();
        let Ok(name_token) = node.value_token() else {
            return;
        };
        let name = name_token.token_text_trimmed();
        match node {
            AnyJsIdentifierUsage::JsReferenceIdentifier(node) => {
                let Some(parent) = node.syntax().parent() else {
                    self.push_reference(BindingName::Value(name), Reference::Read(range));
                    return;
                };
                match parent.kind() {
                    JS_EXPORT_NAMED_SHORTHAND_SPECIFIER | JS_EXPORT_NAMED_SPECIFIER => {
                        self.push_reference(
                            BindingName::Value(name.clone()),
                            Reference::Export(range),
                        );
                        self.push_reference(BindingName::Type(name), Reference::Export(range));
                    }
                    JS_IDENTIFIER_EXPRESSION => {
                        let Some(grand_parent) = parent.parent() else {
                            self.push_reference(BindingName::Value(name), Reference::Read(range));
                            return;
                        };
                        match grand_parent.kind() {
                            JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE | TS_EXPORT_ASSIGNMENT_CLAUSE => {
                                self.push_reference(
                                    BindingName::Value(name.clone()),
                                    Reference::Export(range),
                                );
                                self.push_reference(
                                    BindingName::Type(name),
                                    Reference::Export(range),
                                );
                            }
                            JS_COMPUTED_MEMBER_NAME => {
                                if matches!(
                                    grand_parent.parent().kind(),
                                    Some(
                                        TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                                            | TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
                                            | TS_PROPERTY_SIGNATURE_TYPE_MEMBER
                                            | TS_METHOD_SIGNATURE_CLASS_MEMBER
                                            | TS_METHOD_SIGNATURE_TYPE_MEMBER
                                            | TS_GETTER_SIGNATURE_CLASS_MEMBER
                                            | TS_GETTER_SIGNATURE_TYPE_MEMBER
                                            | TS_SETTER_SIGNATURE_CLASS_MEMBER
                                            | TS_SETTER_SIGNATURE_TYPE_MEMBER
                                    )
                                ) {
                                    self.push_reference(
                                        BindingName::Value(name.clone()),
                                        Reference::AmbientRead(range),
                                    );
                                } else {
                                    self.push_reference(
                                        BindingName::Value(name.clone()),
                                        Reference::Read(range),
                                    );
                                }
                            }
                            _ => {
                                self.push_reference(
                                    BindingName::Value(name),
                                    Reference::Read(range),
                                );
                            }
                        }
                    }
                    _ => {
                        if name.text() == "this" {
                            // Ignore `this` in typeof position. e.g. `typeof this.prop`.
                            return;
                        }
                        match parent
                            .ancestors()
                            .find(|x| x.kind() != TS_QUALIFIED_NAME)
                            .kind()
                        {
                            Some(TS_REFERENCE_TYPE) => {
                                if matches!(parent.kind(), TS_QUALIFIED_NAME) {
                                    self.push_reference(
                                        BindingName::Value(name),
                                        Reference::AmbientRead(range),
                                    )
                                } else {
                                    self.push_reference(
                                        BindingName::Type(name),
                                        Reference::Read(range),
                                    );
                                }
                            }
                            // ignore binding `<X>` from `import().<X>`
                            Some(TS_IMPORT_TYPE_QUALIFIER) => {}
                            Some(TS_TYPEOF_TYPE) => {
                                // a `typeof` type expression refers a value.
                                // It can also refer to an imported value as a type.
                                // We handle this particular case in `pop_scope` (unresolved reference)
                                self.push_reference(
                                    BindingName::Value(name.clone()),
                                    Reference::AmbientRead(range),
                                );
                            }
                            _ => {
                                self.push_reference(
                                    BindingName::Value(name),
                                    Reference::Read(range),
                                );
                            }
                        };
                    }
                }
            }
            AnyJsIdentifierUsage::JsxReferenceIdentifier(_) => {
                if name.text() == "this" {
                    // Ignore `this` in JSX. e.g. `<this.foo />`.
                    return;
                }
                self.push_reference(BindingName::Value(name), Reference::Read(range));
            }
            AnyJsIdentifierUsage::JsIdentifierAssignment(_) => {
                self.push_reference(BindingName::Value(name), Reference::Write(range));
            }
        }
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    #[inline]
    pub fn leave(&mut self, node: &JsSyntaxNode) {
        match node.kind() {
            JS_MODULE
            | JS_SCRIPT
            | JS_FUNCTION_DECLARATION
            | JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | JS_FUNCTION_EXPRESSION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_CLASS_DECLARATION
            | JS_CLASS_EXPORT_DEFAULT_DECLARATION
            | JS_CLASS_EXPRESSION
            | JS_CONSTRUCTOR_CLASS_MEMBER
            | JS_METHOD_CLASS_MEMBER
            | JS_GETTER_CLASS_MEMBER
            | JS_SETTER_CLASS_MEMBER
            | JS_METHOD_OBJECT_MEMBER
            | JS_GETTER_OBJECT_MEMBER
            | JS_SETTER_OBJECT_MEMBER
            | JS_FUNCTION_BODY
            | JS_BLOCK_STATEMENT
            | JS_FOR_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_SWITCH_STATEMENT
            | JS_CATCH_CLAUSE
            | JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
            | TS_DECLARE_FUNCTION_DECLARATION
            | TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | TS_CALL_SIGNATURE_TYPE_MEMBER
            | TS_METHOD_SIGNATURE_CLASS_MEMBER
            | TS_METHOD_SIGNATURE_TYPE_MEMBER
            | TS_INDEX_SIGNATURE_CLASS_MEMBER
            | TS_INDEX_SIGNATURE_TYPE_MEMBER
            | TS_INTERFACE_DECLARATION
            | TS_ENUM_DECLARATION
            | TS_TYPE_ALIAS_DECLARATION
            | TS_MODULE_DECLARATION
            | TS_EXTERNAL_MODULE_DECLARATION => {
                self.pop_scope(node.text_range());
            }
            _ => {
                if let Some(node) = AnyTsType::cast_ref(node) {
                    self.leave_any_type(&node);
                }
            }
        }
    }

    fn leave_any_type(&mut self, node: &AnyTsType) {
        if node.in_conditional_true_type() {
            self.pop_scope(node.syntax().text_range());
            return;
        }
        let node = node.syntax();
        if matches!(
            node.kind(),
            JsSyntaxKind::TS_FUNCTION_TYPE | JsSyntaxKind::TS_MAPPED_TYPE
        ) {
            self.pop_scope(node.text_range());
        }
        // FALLBACK
        // If the conditional type has a bogus true type,
        // then infer declarations was never pushed into any scope.
        // To ensure that every declaration has a binding,
        // we bind the declaration directly in the scope of the conditional type.
        if matches!(node.kind(), JsSyntaxKind::TS_CONDITIONAL_TYPE) && !self.infers.is_empty() {
            self.push_infers_in_scope()
        }
    }

    /// Return any previous extracted [SemanticEvent].
    #[inline]
    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }

    fn push_infers_in_scope(&mut self) {
        let infers = mem::take(&mut self.infers);
        for infer in infers {
            if let Ok(name_token) = infer.ident_token() {
                let name = name_token.token_text_trimmed();
                let name_range = name_token.text_range();
                let binding_info = BindingInfo::new(name_range, JsSyntaxKind::TS_INFER_TYPE);
                self.push_binding(None, BindingName::Type(name), binding_info);
                let scope_id = self.current_scope_mut().scope_id;
                self.stash.push_back(SemanticEvent::DeclarationFound {
                    scope_id,
                    hoisted_scope_id: None,
                    range: name_range,
                });
            }
        }
    }

    fn push_scope(&mut self, range: TextRange, hoisting: ScopeHoisting, is_closure: bool) {
        let scope_id = self.scope_count;
        self.scope_count += 1;
        self.stash.push_back(SemanticEvent::ScopeStarted {
            range,
            scope_id,
            parent_scope_id: self.scopes.iter().last().map(|x| x.scope_id),
            is_closure,
        });
        self.scopes.push(Scope {
            scope_id,
            bindings: vec![],
            references: FxHashMap::default(),
            shadowed: vec![],
            hoisting,
        });
    }

    /// When a scope dies we do the following:
    /// 1 - Match all references and declarations;
    /// 2 - Unmatched references are promoted to its parent scope or become [UnresolvedReference] events;
    /// 3 - All declarations of this scope are removed;
    /// 4 - All shadowed declarations are restored.
    fn pop_scope(&mut self, scope_range: TextRange) {
        debug_assert!(!self.scopes.is_empty());
        let scope = self.scopes.pop().unwrap();
        let scope_id = scope.scope_id;

        // Bind references to declarations
        for (name, mut references) in scope.references {
            if let Some(&BindingInfo {
                range: declared_at,
                declaration_kind,
            }) = self.bindings.get(&name)
            {
                // We know the declaration of these reference.
                for reference in references {
                    let declaration_before_reference =
                        declared_at.start() < reference.range().start();
                    let event = match reference {
                        Reference::Export(range) => {
                            self.stash
                                .push_back(SemanticEvent::Exported { range: declared_at });
                            if declaration_before_reference {
                                SemanticEvent::Read {
                                    range,
                                    declared_at,
                                    scope_id,
                                }
                            } else {
                                SemanticEvent::HoistedRead {
                                    range,
                                    declared_at,
                                    scope_id,
                                }
                            }
                        }
                        Reference::Read(range) | Reference::AmbientRead(range) => {
                            if declaration_kind == JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER
                                && matches!(name, BindingName::Type(_))
                            {
                                // An import namespace imported as a type can only be
                                // used in a qualified name, e.g `Namespace.Type`.
                                // Thus, the reference is unresolved.
                                // Note that we don't need to forward the reference in a parent scope,
                                // because an import namespace is already in the root scope.
                                self.stash.push_back(SemanticEvent::UnresolvedReference {
                                    is_read: !reference.is_write(),
                                    range: *reference.range(),
                                });
                                continue;
                            }
                            if declaration_before_reference {
                                SemanticEvent::Read {
                                    range,
                                    declared_at,
                                    scope_id,
                                }
                            } else {
                                SemanticEvent::HoistedRead {
                                    range,
                                    declared_at,
                                    scope_id,
                                }
                            }
                        }
                        Reference::Write(range) => {
                            if declaration_before_reference {
                                SemanticEvent::Write {
                                    range,
                                    declared_at,
                                    scope_id,
                                }
                            } else {
                                SemanticEvent::HoistedWrite {
                                    range,
                                    declared_at,
                                    scope_id,
                                }
                            }
                        }
                    };
                    self.stash.push_back(event);
                }
            } else if references.iter().all(|r| matches!(r, Reference::Export(_)))
                && self.bindings.contains_key(&name.clone().dual())
            {
                // Don't report an export that exports at least a binding.
            } else if let Some(parent) = self.scopes.last_mut() {
                // Promote these references to the parent scope
                let parent_references = parent.references.entry(name).or_default();
                parent_references.append(&mut references);
            } else if let Some(info) = self.bindings.get(&name.dual()).cloned() {
                // We are in the global scope.
                // Try to bind some of these references to the dual binding of `name`,
                // otherwise raide `UnresolvedReference`.
                for reference in references {
                    match reference {
                        Reference::Export(_) => {
                            // An export can export both a value and a type.
                            // If a dual binding exists, then it exports to the dual binding.
                        }
                        Reference::AmbientRead(range) if info.is_imported() => {
                            // An ambient read can only read a value,
                            // but also an imported value as a type (with the `type` modifier)
                            let declared_at = info.range;
                            let declaration_before_reference =
                                declared_at.start() < reference.range().start();
                            let event = if declaration_before_reference {
                                SemanticEvent::Read {
                                    range,
                                    declared_at,
                                    scope_id: 0,
                                }
                            } else {
                                SemanticEvent::HoistedRead {
                                    range,
                                    declared_at,
                                    scope_id: 0,
                                }
                            };
                            self.stash.push_back(event);
                        }
                        _ => {
                            self.stash.push_back(SemanticEvent::UnresolvedReference {
                                is_read: !reference.is_write(),
                                range: *reference.range(),
                            });
                        }
                    }
                }
            } else {
                // We are in the global scope. Raise `UnresolvedReference`.
                for reference in references {
                    self.stash.push_back(SemanticEvent::UnresolvedReference {
                        is_read: !reference.is_write(),
                        range: *reference.range(),
                    });
                }
            }
        }

        // Remove all bindings declared in this scope
        for binding in scope.bindings {
            self.bindings.remove(&binding);
        }

        // Restore shadowed bindings
        self.bindings.extend(scope.shadowed);

        self.stash.push_back(SemanticEvent::ScopeEnded {
            range: scope_range,
            scope_id: scope.scope_id,
        });
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        // We should at least have the global scope
        debug_assert!(!self.scopes.is_empty());
        self.scopes.last_mut().unwrap()
    }

    /// Finds the scope where declarations that are hoisted
    /// will be declared at. For example:
    ///
    /// ```js
    /// function f() {
    ///     if (true) {
    ///         var a;
    ///     }
    /// }
    /// ```
    ///
    /// `a` declaration will be hoisted to the scope of
    /// function `f`.
    ///
    /// This method when called inside the `f` scope will return
    /// the `f` scope index.
    fn scope_index_to_hoist_declarations(&mut self, skip: usize) -> Option<usize> {
        debug_assert!(self.scopes.len() > skip);
        // We should at least have the global scope
        // that do not hoist
        debug_assert!(matches!(
            self.scopes[0].hoisting,
            ScopeHoisting::DontHoistDeclarationsToParent
        ));
        self.scopes
            .iter()
            .rev()
            .skip(skip)
            .find(|scope| scope.hoisting == ScopeHoisting::DontHoistDeclarationsToParent)
            .map(|x| x.scope_id)
            .filter(|scope_id| self.current_scope_mut().scope_id != *scope_id)
    }

    /// Push the binding `binding` into the hoisted scope if it exists, or into the current scope.
    fn push_binding(
        &mut self,
        hoisted_scope_id: Option<usize>,
        binding_name: BindingName,
        binding_info: BindingInfo,
    ) {
        let current_scope_id = self.current_scope_mut().scope_id;
        let binding_scope_id = hoisted_scope_id.unwrap_or(current_scope_id);
        let scope = self
            .scopes
            .iter_mut()
            .rev()
            .find(|s| s.scope_id == binding_scope_id);
        // A scope will always be found
        debug_assert!(scope.is_some());
        let scope = scope.unwrap();

        // insert this name into the list of available names
        // and save shadowed names to be used later
        if let Some(shadowed) = self.bindings.insert(binding_name.clone(), binding_info) {
            scope.shadowed.push((binding_name.clone(), shadowed));
        }
        scope.bindings.push(binding_name);
    }

    /// Push the reference `reference` of the binding `binding_name` into the current scope.
    fn push_reference(&mut self, binding_name: BindingName, reference: Reference) {
        self.current_scope_mut()
            .references
            .entry(binding_name)
            .or_default()
            .push(reference);
    }
}

/// Extracts [SemanticEvent] from [SyntaxNode].
/// See [semantic_events] how to create this iterator.
struct SemanticEventIterator {
    iter: Preorder<JsLanguage>,
    extractor: SemanticEventExtractor,
}

impl Iterator for SemanticEventIterator {
    type Item = SemanticEvent;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.extractor.pop() {
                break Some(e);
            } else {
                use biome_js_syntax::WalkEvent::*;
                match self.iter.next() {
                    Some(Enter(node)) => {
                        self.extractor.enter(&node);
                    }
                    Some(Leave(node)) => {
                        self.extractor.leave(&node);
                    }
                    None => {
                        if let Some(e) = self.extractor.pop() {
                            break Some(e);
                        } else {
                            // Check that every scope was pop.
                            debug_assert!(self.extractor.scopes.is_empty());
                            break None;
                        }
                    }
                }
            }
        }
    }
}

/// Extracts [SemanticEvent] from [JsSyntaxNode].
///
/// For a way to extract [SemanticEvent] which gives more control see [SemanticEventExtractor].
///
/// ```rust
/// use biome_js_parser::*;
/// use biome_js_syntax::*;
/// use biome_js_semantic::*;
/// let tree = parse("let a = 1", JsFileSource::js_script(), JsParserOptions::default());
/// for e in semantic_events(tree.syntax()) {
///     dbg!(e);
/// }
/// ```
pub fn semantic_events(root: JsSyntaxNode) -> impl IntoIterator<Item = SemanticEvent> {
    SemanticEventIterator {
        iter: root.preorder(),
        extractor: SemanticEventExtractor::default(),
    }
}
