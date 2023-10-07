//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use biome_js_syntax::binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding};
use biome_js_syntax::{
    AnyJsExportNamedSpecifier, AnyJsNamedImportSpecifier, AnyTsType, JsImportNamedClause,
};
use biome_js_syntax::{
    AnyJsIdentifierUsage, JsLanguage, JsReferenceIdentifier, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken, TextRange, TextSize, TsTypeParameterName,
};
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
        scope_started_at: TextSize,
        scope_id: usize,
        hoisted_scope_id: Option<usize>,
        name: TokenText,
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
    pub fn range(&self) -> &TextRange {
        match self {
            Self::DeclarationFound { range, .. }
            | Self::ScopeStarted { range, .. }
            | Self::ScopeEnded { range, .. }
            | Self::Read { range, .. }
            | Self::HoistedRead { range, .. }
            | Self::Write { range, .. }
            | Self::HoistedWrite { range, .. }
            | Self::UnresolvedReference { range, .. }
            | Self::Exported { range } => range,
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
/// For a simpler way to extract [SemanticEvent] see [semantic_events] or [SemanticEventIterator].
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
    bindings: FxHashMap<BindingName, TextRange>,
    /// Type parameters bound in a `infer T` clause.
    infers: Vec<TsTypeParameterName>,
}

/// A declaration is either a type, a value, or both.
///
/// For example a class is botha  type and a value in TypeScript.
/// A variable declaration is always a value, and a type alias is always a type.
#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum DeclarationKind {
    Type,
    Value,
    Both,
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
    /// Returns a binding with the same name, but the other kind.
    fn dual(self) -> Self {
        match self {
            Self::Type(name) => Self::Value(name),
            Self::Value(name) => Self::Type(name),
        }
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
    /// Read and export only a type
    /// ```ts
    /// export { type T1 }
    /// export type { T2, T3 }
    /// ```
    ExportType(TextRange),
    /// All reads that are not an export
    /// ```js
    /// f();
    /// a;
    /// ```
    Read(TextRange),
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
            | Self::ExportType(range)
            | Self::Read(range)
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
    started_at: TextSize,
    /// All bindings declared inside this scope
    bindings: Vec<BindingName>,
    /// References that still needs to be bound and will be solved at the end of the scope
    references: FxHashMap<BindingName, Vec<Reference>>,
    /// All bindings that where shadowed and will be restored after this scope ends.
    shadowed: Vec<(BindingName, TextRange)>,
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

    /// See [SemanticEvent] for a more detailed description of which events [SyntaxNode] generates.
    #[inline]
    pub fn enter(&mut self, node: &JsSyntaxNode) {
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
            | TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
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
            self.push_conditional_true_scope(node);
        } else if let Some(node) = node.as_ts_function_type() {
            self.push_scope(
                node.syntax().text_range(),
                ScopeHoisting::DontHoistDeclarationsToParent,
                false,
            );
        }
    }

    fn enter_identifier_binding(&mut self, node: &AnyJsIdentifierBinding) -> Option<()> {
        let name_token = node.name_token().ok()?;
        let Some(declaration) = node.declaration() else {
            // Handle identifiers in bogus statements,
            // and arrow function with a single parameter without parentheses.
            // TODO: The AST should be modified.
            // In `a => ...` the binding `a` should be wrapped in a JSFormalParameter.
            self.push_binding_into_scope(None, &name_token, DeclarationKind::Value);
            return Some(());
        };
        let is_exported = declaration.export().is_some();
        match declaration {
            AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                let declaration = declarator.declaration()?;
                let hoisted_scope_id = if declaration.is_var() {
                    self.scope_index_to_hoist_declarations(0)
                } else {
                    None
                };
                self.push_binding_into_scope(hoisted_scope_id, &name_token, DeclarationKind::Value);
            }
            AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
            | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::JsFunctionDeclaration(_)
            | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_) => {
                let hoisted_scope_id = self.scope_index_to_hoist_declarations(1);
                self.push_binding_into_scope(hoisted_scope_id, &name_token, DeclarationKind::Value);
            }
            AnyJsBindingDeclaration::JsClassExpression(_)
            | AnyJsBindingDeclaration::JsFunctionExpression(_) => {
                self.push_binding_into_scope(None, &name_token, DeclarationKind::Both);
            }
            AnyJsBindingDeclaration::JsClassDeclaration(_)
            | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_)
            | AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
            | AnyJsBindingDeclaration::TsTypeAliasDeclaration(_)
            | AnyJsBindingDeclaration::TsEnumDeclaration(_)
            | AnyJsBindingDeclaration::TsModuleDeclaration(_) => {
                let binding_kind = match declaration.syntax().kind() {
                    JS_CLASS_DECLARATION
                    | JS_CLASS_EXPORT_DEFAULT_DECLARATION
                    | TS_ENUM_DECLARATION => DeclarationKind::Both,
                    TS_INTERFACE_DECLARATION | TS_TYPE_ALIAS_DECLARATION => DeclarationKind::Type,
                    _ => DeclarationKind::Value,
                };
                // These declarations have their own scope.
                // Thus we need to hoist the declaration to the parent scope.
                let parent_scope = self.scopes.get(self.scopes.len() - 2);
                let parent_scope = parent_scope.map(|scope| scope.scope_id);
                self.push_binding_into_scope(parent_scope, &name_token, binding_kind);
            }
            AnyJsBindingDeclaration::TsInferType(_) => {
                self.infers
                    .push(TsTypeParameterName::unwrap_cast(node.syntax().clone()));
            }
            AnyJsBindingDeclaration::TsMappedType(_)
            | AnyJsBindingDeclaration::TsTypeParameter(_) => {
                self.push_binding_into_scope(None, &name_token, DeclarationKind::Type);
            }
            AnyJsBindingDeclaration::JsImportDefaultClause(clause) => {
                let binding_kind = if clause.type_token().is_some() {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, binding_kind);
            }
            AnyJsBindingDeclaration::JsImportNamespaceClause(clause) => {
                let binding_kind = if clause.type_token().is_some() {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, binding_kind);
            }
            AnyJsBindingDeclaration::TsImportEqualsDeclaration(declaration) => {
                let binding_kind = if declaration.type_token().is_some() {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, binding_kind);
            }
            AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => {
                let clause = declaration.parent::<JsImportNamedClause>()?;
                let binding_kind = if clause.type_token().is_some() {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, binding_kind);
            }
            AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_) => {
                let specifier = AnyJsNamedImportSpecifier::unwrap_cast(declaration.into_syntax());
                let binding_kind = if specifier.is_type_only() {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, binding_kind);
            }
            AnyJsBindingDeclaration::JsBogusParameter(_)
            | AnyJsBindingDeclaration::JsFormalParameter(_)
            | AnyJsBindingDeclaration::JsRestParameter(_)
            | AnyJsBindingDeclaration::TsIndexSignatureParameter(_)
            | AnyJsBindingDeclaration::TsPropertyParameter(_)
            | AnyJsBindingDeclaration::JsCatchDeclaration(_) => {
                self.push_binding_into_scope(None, &name_token, DeclarationKind::Value);
            }
        }
        if is_exported {
            self.stash.push_back(SemanticEvent::Exported {
                range: node.syntax().text_range(),
            });
        }
        Some(())
    }

    fn enter_identifier_usage(&mut self, node: AnyJsIdentifierUsage) {
        let range = node.syntax().text_range();
        let Ok(name_token) = node.value_token() else {
            return;
        };
        let name = name_token.token_text_trimmed();
        let current_scope = self.current_scope_mut();
        let (binding_name, reference) = match node {
            AnyJsIdentifierUsage::JsReferenceIdentifier(node) => {
                match Self::is_js_reference_identifier_exported(&node) {
                    Some(reference @ Reference::ExportType(_)) => {
                        (BindingName::Type(name), reference)
                    }
                    Some(reference @ Reference::Export(_)) => {
                        // A regular `export` can export both a type binding and a value binding.
                        // ```js
                        // type X = number;
                        // const X = "";
                        // export { X }
                        // ```
                        // Thus, we add two references
                        let references = current_scope
                            .references
                            .entry(BindingName::Value(name.clone()))
                            .or_default();
                        references.push(reference.clone());
                        (BindingName::Type(name), reference)
                    }
                    _ => {
                        if name.text() == "this" {
                            // Ignore `this` in typeof position. e.g. `typeof this.prop`.
                            return;
                        }
                        let binding_name = match node
                            .syntax()
                            .ancestors()
                            .skip(1)
                            .find(|x| x.kind() != TS_QUALIFIED_NAME)
                            .kind()
                        {
                            Some(TS_REFERENCE_TYPE | TS_NAME_WITH_TYPE_ARGUMENTS) => {
                                BindingName::Type(name)
                            }
                            // ignore binding `<X>` from `import().<X>`
                            Some(TS_IMPORT_TYPE_QUALIFIER) => return,
                            _ => BindingName::Value(name),
                        };
                        (binding_name, Reference::Read(range))
                    }
                }
            }
            AnyJsIdentifierUsage::JsxReferenceIdentifier(_) => {
                (BindingName::Value(name), Reference::Read(range))
            }
            AnyJsIdentifierUsage::JsIdentifierAssignment(_) => {
                (BindingName::Value(name), Reference::Write(range))
            }
        };
        let references = current_scope.references.entry(binding_name).or_default();
        references.push(reference);
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    #[inline]
    pub fn leave(&mut self, node: &JsSyntaxNode) {
        match node.kind() {
            JS_MODULE | JS_SCRIPT => self.pop_scope(node.text_range()),
            JS_FUNCTION_DECLARATION
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
        } else if let Some(node) = node.as_ts_function_type() {
            self.pop_scope(node.syntax().text_range());
        }
    }

    /// Return any previous extracted [SemanticEvent].
    #[inline]
    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }

    fn push_conditional_true_scope(&mut self, node: &AnyTsType) {
        self.push_scope(
            node.syntax().text_range(),
            ScopeHoisting::DontHoistDeclarationsToParent,
            false,
        );

        let infers = mem::take(&mut self.infers);
        for infer in infers {
            if let Ok(name_token) = infer.ident_token() {
                self.push_binding_into_scope(None, &name_token, DeclarationKind::Type);
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
            started_at: range.start(),
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
    fn pop_scope(&mut self, range: TextRange) {
        debug_assert!(!self.scopes.is_empty());
        let scope = self.scopes.pop().unwrap();
        let scope_id = scope.scope_id;

        // Match references and declarations
        for (name, mut references) in scope.references {
            if let Some(&declared_at) = self.bindings.get(&name) {
                // If we know the declaration of these reference push the correct events...
                for reference in references {
                    let declaration_before_reference =
                        declared_at.start() < reference.range().start();
                    let event = match reference {
                        Reference::Export(range) | Reference::ExportType(range) => {
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
                        Reference::Read(range) => {
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
            } else if let Some(parent) = self.scopes.last_mut() {
                // ... if not, promote these references to the parent scope ...
                let parent_references = parent.references.entry(name).or_default();
                parent_references.append(&mut references);
            } else {
                let has_dual_binding = self.bindings.get(&name.dual()).is_some();
                // ... or raise UnresolvedReference if this is the global scope.
                for reference in references {
                    if has_dual_binding && matches!(reference, Reference::Export { .. }) {
                        // An export can export both a value and a type.
                        // If a dual binding exists, then it exports the dual binding.
                        continue;
                    }
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
            range,
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

    fn push_binding_into_scope(
        &mut self,
        hoisted_scope_id: Option<usize>,
        name_token: &JsSyntaxToken,
        declaration_kind: DeclarationKind,
    ) {
        let name = name_token.token_text_trimmed();
        let name_range = name_token.text_range();
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

        let binding_name = if declaration_kind == DeclarationKind::Type {
            BindingName::Type(name.clone())
        } else {
            BindingName::Value(name.clone())
        };
        // insert this name into the list of available names
        // and save shadowed names to be used later
        if let Some(shadowed) = self.bindings.insert(binding_name.clone(), name_range) {
            scope.shadowed.push((binding_name.clone(), shadowed));
        }
        scope.bindings.push(binding_name);

        if declaration_kind == DeclarationKind::Both {
            let binding_name = BindingName::Type(name.clone());
            if let Some(shadowed) = self.bindings.insert(binding_name.clone(), name_range) {
                scope.shadowed.push((binding_name.clone(), shadowed));
            }
            scope.bindings.push(binding_name);
        }

        self.stash.push_back(SemanticEvent::DeclarationFound {
            range: name_range,
            scope_started_at: scope.started_at,
            scope_id: current_scope_id,
            hoisted_scope_id,
            name,
        });
    }

    // Returns an export reference if `reference` is exported.
    fn is_js_reference_identifier_exported(reference: &JsReferenceIdentifier) -> Option<Reference> {
        let range = reference.syntax().text_range();
        if let Some(specifier) = AnyJsExportNamedSpecifier::cast(reference.syntax().parent()?) {
            if specifier.is_type_only() {
                Some(Reference::ExportType(range))
            } else {
                Some(Reference::Export(range))
            }
        } else if matches!(
            reference.syntax().grand_parent()?.kind(),
            JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE | TS_EXPORT_ASSIGNMENT_CLAUSE
        ) {
            Some(Reference::Export(range))
        } else {
            None
        }
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
