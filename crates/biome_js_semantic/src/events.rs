//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsIdentifierUsage, AnyJsVariableDeclaration,
    JsAssignmentExpression, JsExportNamedClause, JsLanguage, JsReferenceIdentifier, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken, JsVariableDeclarator, TextRange, TextSize,
    TsImportEqualsDeclaration, TsTypeParameterName,
};
use biome_js_syntax::{
    AnyJsExportNamedSpecifier, AnyJsNamedImportSpecifier, AnyTsType, JsImportDefaultClause,
    JsImportNamedClause,
};
use biome_rowan::{syntax::Preorder, AstNode, SyntaxNodeOptionExt, TokenText};
use rustc_hash::FxHashMap;
use std::collections::{HashMap, VecDeque};
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
        range: TextRange,
        started_at: TextSize,
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
    /// At any point this is the set of available bindings in the current scope
    bindings: FxHashMap<BindingName, BindingInfo>,
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

/// Holds the [TextRange] of the token along with the kind of declaration
#[derive(Debug)]
struct BindingInfo {
    /// Range of the identifier binding.
    name_range: TextRange,
    /// Syntax kind of the declaration associated to the binding
    declaration_syntax_kind: JsSyntaxKind,
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

    const fn is_any_export(&self) -> bool {
        matches!(self, Self::Export { .. } | Self::ExportType { .. })
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

#[derive(Debug)]
pub enum ScopeHoisting {
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
    references: HashMap<BindingName, Vec<Reference>>,
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
            | TS_DECLARE_FUNCTION_DECLARATION => {
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
        let name_token = match node {
            AnyJsIdentifierBinding::JsIdentifierBinding(binding) => binding.name_token(),
            AnyJsIdentifierBinding::TsIdentifierBinding(binding) => binding.name_token(),
            AnyJsIdentifierBinding::TsTypeParameterName(binding) => {
                if binding.in_infer_type() {
                    self.infers.push(binding.clone());
                    return None;
                }
                binding.ident_token()
            }
        }
        .ok()?;

        let node = node.syntax();
        let parent = node.parent()?;
        let parent_kind = parent.kind();
        match parent_kind {
            JS_VARIABLE_DECLARATOR => {
                let declarator = JsVariableDeclarator::unwrap_cast(parent);
                let declaration = declarator.declaration()?;
                let hoisted_scope_id = if declaration.is_var() {
                    self.scope_index_to_hoist_declarations(0)
                } else {
                    None
                };
                self.push_binding_into_scope(
                    hoisted_scope_id,
                    &name_token,
                    parent_kind,
                    DeclarationKind::Value,
                );
                self.export_variable_declaration(node, &declaration);
            }
            JS_FUNCTION_DECLARATION | JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                let hoisted_scope_id = self.scope_index_to_hoist_declarations(1);
                self.push_binding_into_scope(
                    hoisted_scope_id,
                    &name_token,
                    parent_kind,
                    DeclarationKind::Value,
                );
                self.export_function_declaration(node, &parent);
            }
            JS_CLASS_EXPRESSION | JS_FUNCTION_EXPRESSION => {
                self.push_binding_into_scope(None, &name_token, parent_kind, DeclarationKind::Both);
                self.export_declaration_expression(node, &parent);
            }
            JS_CLASS_DECLARATION
            | JS_CLASS_EXPORT_DEFAULT_DECLARATION
            | TS_ENUM_DECLARATION
            | TS_INTERFACE_DECLARATION
            | TS_TYPE_ALIAS_DECLARATION
            | TS_MODULE_DECLARATION => {
                let binding_kind = match parent_kind {
                    JS_CLASS_DECLARATION
                    | JS_CLASS_EXPORT_DEFAULT_DECLARATION
                    | TS_ENUM_DECLARATION => DeclarationKind::Both,
                    TS_INTERFACE_DECLARATION | TS_TYPE_ALIAS_DECLARATION => DeclarationKind::Type,
                    _ => DeclarationKind::Value,
                };
                let parent_scope = self.scopes.get(self.scopes.len() - 2);
                let parent_scope = parent_scope.map(|scope| scope.scope_id);
                self.push_binding_into_scope(parent_scope, &name_token, parent_kind, binding_kind);
                self.export_declaration(node, &parent);
            }
            JS_BINDING_PATTERN_WITH_DEFAULT
            | JS_OBJECT_BINDING_PATTERN
            | JS_OBJECT_BINDING_PATTERN_REST
            | JS_OBJECT_BINDING_PATTERN_PROPERTY
            | JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
            | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
            | JS_ARRAY_BINDING_PATTERN
            | JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
            | JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
                self.push_binding_into_scope(
                    None,
                    &name_token,
                    parent_kind,
                    DeclarationKind::Value,
                );

                let possible_declaration = parent.ancestors().find(|x| {
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
                            | JS_VARIABLE_DECLARATOR
                            | JS_VARIABLE_DECLARATOR_LIST
                    )
                })?;
                self.export_variable_declaration(
                    node,
                    &AnyJsVariableDeclaration::cast(possible_declaration)?,
                );
            }
            TS_INFER_TYPE | TS_MAPPED_TYPE | TS_TYPE_PARAMETER => {
                self.push_binding_into_scope(None, &name_token, parent_kind, DeclarationKind::Type);
            }
            JS_IMPORT_DEFAULT_CLAUSE => {
                let clause = JsImportDefaultClause::unwrap_cast(parent);
                let binding_kind = if clause.type_token().is_some() {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, parent_kind, binding_kind);
            }
            TS_IMPORT_EQUALS_DECLARATION => {
                let declaration = TsImportEqualsDeclaration::unwrap_cast(parent);
                let binding_kind = if declaration.type_token().is_some() {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, parent_kind, binding_kind);
            }
            JS_DEFAULT_IMPORT_SPECIFIER => {
                let clause = JsImportNamedClause::cast(parent.parent()?)?;
                let binding_kind = if clause.type_token().is_some() {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, parent_kind, binding_kind);
            }
            JS_NAMED_IMPORT_SPECIFIER | JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                let specifier = AnyJsNamedImportSpecifier::unwrap_cast(parent);
                let binding_kind = if specifier.type_token().is_some()
                    || JsImportNamedClause::cast(specifier.syntax().ancestors().nth(3)?)?
                        .type_token()
                        .is_some()
                {
                    DeclarationKind::Type
                } else {
                    DeclarationKind::Both
                };
                self.push_binding_into_scope(None, &name_token, parent_kind, binding_kind);
            }
            _ => {
                self.push_binding_into_scope(
                    None,
                    &name_token,
                    parent_kind,
                    DeclarationKind::Value,
                );
            }
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
            let name_token = infer.ident_token().ok();
            let parent_kind = infer.syntax().parent().map(|parent| parent.kind());

            if let (Some(name_token), Some(parent_kind)) = (name_token, parent_kind) {
                self.push_binding_into_scope(None, &name_token, parent_kind, DeclarationKind::Type);
            }
        }
    }

    fn push_scope(&mut self, range: TextRange, hoisting: ScopeHoisting, is_closure: bool) {
        let scope_id = self.scope_count;
        self.scope_count += 1;

        let parent_scope_id = self.scopes.iter().last().map(|x| x.scope_id);

        self.stash.push_back(SemanticEvent::ScopeStarted {
            range,
            scope_id,
            parent_scope_id,
            is_closure,
        });

        self.scopes.push(Scope {
            scope_id,
            started_at: range.start(),
            bindings: vec![],
            references: HashMap::new(),
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

        // Match references and declarations
        for (name, mut references) in scope.references {
            // If we know the declaration of these reference push the correct events...
            if let Some(declared_binding) = self.bindings.get(&name) {
                let declared_at = declared_binding.name_range;
                let declaration_syntax_kind = declared_binding.declaration_syntax_kind;

                for reference in &references {
                    let declaration_before_reference =
                        declared_at.start() < reference.range().start();
                    let event = match &reference {
                        Reference::Export(range)
                        | Reference::ExportType(range)
                        | Reference::Read(range) => {
                            if declaration_before_reference {
                                SemanticEvent::Read {
                                    range: *range,
                                    declared_at,
                                    scope_id: scope.scope_id,
                                }
                            } else {
                                SemanticEvent::HoistedRead {
                                    range: *range,
                                    declared_at,
                                    scope_id: scope.scope_id,
                                }
                            }
                        }
                        Reference::Write(range) => {
                            if declaration_before_reference {
                                SemanticEvent::Write {
                                    range: *range,
                                    declared_at,
                                    scope_id: scope.scope_id,
                                }
                            } else {
                                SemanticEvent::HoistedWrite {
                                    range: *range,
                                    declared_at,
                                    scope_id: scope.scope_id,
                                }
                            }
                        }
                    };
                    self.stash.push_back(event);

                    if reference.is_any_export() {
                        // Check shadowed bindings to find an exportable binding
                        let find_exportable_binding = scope.shadowed.iter().find_map(
                            |(shadowed_ident_name, shadowed_binding_info)| {
                                if shadowed_ident_name != &name {
                                    return None;
                                }

                                // The order of interface and other bindings is valid in either order
                                match (
                                    declaration_syntax_kind,
                                    shadowed_binding_info.declaration_syntax_kind,
                                ) {
                                    (
                                        JS_VARIABLE_DECLARATOR | JS_CLASS_DECLARATION,
                                        TS_INTERFACE_DECLARATION,
                                    )
                                    | (
                                        TS_INTERFACE_DECLARATION,
                                        JS_VARIABLE_DECLARATOR | JS_CLASS_DECLARATION,
                                    ) => Some(shadowed_binding_info),
                                    _ => None,
                                }
                            },
                        );
                        if let Some(binding_info) = find_exportable_binding {
                            self.stash.push_back(SemanticEvent::Exported {
                                range: binding_info.name_range,
                            });
                        }

                        self.stash
                            .push_back(SemanticEvent::Exported { range: declared_at });
                    }
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
            started_at: scope.started_at,
            scope_id: scope.scope_id,
        });
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        // We should at least have the global scope
        debug_assert!(!self.scopes.is_empty());

        match self.scopes.last_mut() {
            Some(scope) => scope,
            None => unreachable!(),
        }
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
        // We should at least have the global scope
        // that do not hoist
        debug_assert!(matches!(
            self.scopes[0].hoisting,
            ScopeHoisting::DontHoistDeclarationsToParent
        ));
        debug_assert!(self.scopes.len() > skip);

        let scope_id_hoisted_to = self
            .scopes
            .iter()
            .rev()
            .skip(skip)
            .find(|scope| matches!(scope.hoisting, ScopeHoisting::DontHoistDeclarationsToParent))
            .map(|x| x.scope_id);

        let current_scope_id = self.current_scope_mut().scope_id;
        match scope_id_hoisted_to {
            Some(scope_id) => {
                if scope_id == current_scope_id {
                    None
                } else {
                    Some(scope_id)
                }
            }
            // Worst case this will fallback to the global scope
            // which will be idx = 0
            None => unreachable!("We must have a least of scope."),
        }
    }

    fn push_binding_into_scope(
        &mut self,
        hoisted_scope_id: Option<usize>,
        name_token: &JsSyntaxToken,
        declaration_syntax_kind: JsSyntaxKind,
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
        let binding_info = BindingInfo {
            name_range,
            declaration_syntax_kind,
        };
        // insert this name into the list of available names
        // and save shadowed names to be used later
        if let Some(shadowed) = self.bindings.insert(binding_name.clone(), binding_info) {
            scope.shadowed.push((binding_name.clone(), shadowed));
        }
        scope.bindings.push(binding_name);

        if declaration_kind == DeclarationKind::Both {
            let binding_name = BindingName::Type(name.clone());
            let binding_info = BindingInfo {
                name_range,
                declaration_syntax_kind,
            };
            if let Some(shadowed) = self.bindings.insert(binding_name.clone(), binding_info) {
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

    // Check if a function is exported and raise the [Exported] event.
    fn export_function_declaration(
        &mut self,
        binding: &JsSyntaxNode,
        function_declaration: &JsSyntaxNode,
    ) {
        debug_assert!(matches!(
            function_declaration.kind(),
            JS_FUNCTION_DECLARATION | JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
        ));
        let is_exported = matches!(
            function_declaration.parent().kind(),
            Some(JS_EXPORT | JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)
        );
        if is_exported {
            self.stash.push_back(SemanticEvent::Exported {
                range: binding.text_range(),
            });
        }
    }

    // Check if a function or class expression is exported and raise the [Exported] event.
    fn export_declaration_expression(
        &mut self,
        binding: &JsSyntaxNode,
        declaration_expression: &JsSyntaxNode,
    ) {
        debug_assert!(matches!(
            declaration_expression.kind(),
            JS_FUNCTION_EXPRESSION | JS_CLASS_EXPRESSION
        ));
        let is_module_exports = declaration_expression
            .parent()
            .map(Self::is_assignment_left_side_module_exports)
            .unwrap_or(false);
        if is_module_exports {
            self.stash.push_back(SemanticEvent::Exported {
                range: binding.text_range(),
            });
        }
    }

    // Check if a class, type alias, enum, interface, module is exported and raise the [Exported] event.
    fn export_declaration(&mut self, binding: &JsSyntaxNode, declaration: &JsSyntaxNode) {
        debug_assert!(matches!(
            declaration.kind(),
            JS_CLASS_DECLARATION
                | JS_CLASS_EXPORT_DEFAULT_DECLARATION
                | TS_TYPE_ALIAS_DECLARATION
                | TS_ENUM_DECLARATION
                | TS_INTERFACE_DECLARATION
                | TS_MODULE_DECLARATION
                | TS_EXTERNAL_MODULE_DECLARATION
        ));
        let is_exported = matches!(
            declaration.parent().kind(),
            Some(JS_EXPORT | JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)
        );

        if is_exported {
            self.stash.push_back(SemanticEvent::Exported {
                range: binding.text_range(),
            });
        }
    }

    // Check if a variable is exported and raise the [Exported] event.
    fn export_variable_declaration(
        &mut self,
        binding: &JsSyntaxNode,
        variable_declaration: &AnyJsVariableDeclaration,
    ) {
        let is_exported = variable_declaration
            .syntax()
            .grand_parent()
            .is_some_and(|x| matches!(x.kind(), JS_EXPORT | TS_EXPORT_DECLARE_CLAUSE));
        if is_exported {
            self.stash.push_back(SemanticEvent::Exported {
                range: binding.text_range(),
            });
        }
    }

    // Returns an export reference if `reference` is exported.
    fn is_js_reference_identifier_exported(reference: &JsReferenceIdentifier) -> Option<Reference> {
        let range = reference.syntax().text_range();
        let parent = reference.syntax().parent()?;
        match parent.kind() {
            JS_EXPORT_NAMED_SPECIFIER | JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
                let specifier = AnyJsExportNamedSpecifier::unwrap_cast(parent);
                if specifier.type_token().is_some()
                    || JsExportNamedClause::cast(specifier.syntax().parent()?.parent()?)?
                        .type_token()
                        .is_some()
                {
                    Some(Reference::ExportType(range))
                } else {
                    Some(Reference::Export(range))
                }
            }
            JS_IDENTIFIER_EXPRESSION => {
                let grandparent = parent.parent()?;
                // check "export default" keyword
                if matches!(
                    grandparent.kind(),
                    JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE | TS_EXPORT_ASSIGNMENT_CLAUSE
                ) {
                    return Some(Reference::Export(range));
                }
                // check module.exports = X
                Self::is_assignment_left_side_module_exports(grandparent)
                    .then_some(Reference::Export(range))
            }
            JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                let maybe_assignment = parent.ancestors().nth(3)?;
                // check module.exports = { X }
                Self::is_assignment_left_side_module_exports(maybe_assignment)
                    .then_some(Reference::Export(range))
            }
            _ => None,
        }
    }

    fn is_assignment_left_side_module_exports(node: JsSyntaxNode) -> bool {
        if let Some(expr) = JsAssignmentExpression::cast(node) {
            return match expr.left() {
                Ok(AnyJsAssignmentPattern::AnyJsAssignment(
                    AnyJsAssignment::JsStaticMemberAssignment(member),
                )) => {
                    let Some(first) = member
                        .object()
                        .ok()
                        .and_then(|x| x.as_js_reference_identifier()?.value_token().ok())
                    else {
                        return false;
                    };
                    match first.text_trimmed() {
                        // module.exports = ..
                        "module" => {
                            let Some(second) = member
                                .member()
                                .ok()
                                .and_then(|x| x.as_js_name()?.value_token().ok())
                            else {
                                return false;
                            };
                            second.text_trimmed() == "exports"
                        }
                        // exports.<anything> = ..
                        "exports" => true,
                        _ => false,
                    }
                }
                // exports = ...
                Ok(AnyJsAssignmentPattern::AnyJsAssignment(
                    AnyJsAssignment::JsIdentifierAssignment(ident),
                )) => ident.syntax().text_trimmed() == "exports",
                _ => false,
            };
        }
        false
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
