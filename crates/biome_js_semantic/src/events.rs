//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use rustc_hash::FxHashMap;
use std::collections::{HashMap, VecDeque};

use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, JsAssignmentExpression,
    JsCallExpression, JsForVariableDeclaration, JsIdentifierAssignment, JsIdentifierBinding,
    JsLanguage, JsParenthesizedExpression, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode,
    JsSyntaxToken, JsVariableDeclaration, JsVariableDeclarator, JsVariableDeclaratorList,
    JsxReferenceIdentifier, TextRange, TextSize, TsIdentifierBinding, TsTypeParameterName,
};
use biome_rowan::{syntax::Preorder, AstNode, SyntaxNodeCast, SyntaxNodeOptionExt, TokenText};

/// Events emitted by the [SemanticEventExtractor]. These events are later
/// made into the Semantic Model.
#[derive(Debug)]
pub enum SemanticEvent {
    /// Tracks where a new symbol declaration is found.
    /// Generated for:
    /// - Variable Declarations
    /// - Import bindings
    /// - Functions parameters
    DeclarationFound {
        range: TextRange,
        scope_started_at: TextSize,
        scope_id: usize,
        hoisted_scope_id: Option<usize>,
        name: TokenText,
    },

    /// Tracks where a symbol is read, but only if its declaration
    /// is before this reference.
    /// Generated for:
    /// - All reference identifiers
    Read {
        range: TextRange,
        declared_at: TextRange,
        scope_id: usize,
    },

    /// Tracks where a symbol is read, but only if its declaration
    /// was hoisted. This means that its declaration is after this reference.
    /// - All reference identifiers
    HoistedRead {
        range: TextRange,
        declared_at: TextRange,
        scope_id: usize,
    },

    /// Tracks where a symbol is written, but only if its declaration
    /// is before this reference.
    /// Generated for:
    /// - All identifier assignments
    Write {
        range: TextRange,
        declared_at: TextRange,
        scope_id: usize,
    },

    /// Tracks where a symbol is written, but only if its declaration
    /// was hoisted. This means that its declaration is after this reference.
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
    /// The range points to the binding that
    /// is being exported
    Exported { range: TextRange },
}

impl SemanticEvent {
    pub fn range(&self) -> &TextRange {
        match self {
            SemanticEvent::DeclarationFound { range, .. }
            | SemanticEvent::ScopeStarted { range, .. }
            | SemanticEvent::ScopeEnded { range, .. }
            | SemanticEvent::Read { range, .. }
            | SemanticEvent::HoistedRead { range, .. }
            | SemanticEvent::Write { range, .. }
            | SemanticEvent::HoistedWrite { range, .. }
            | SemanticEvent::UnresolvedReference { range, .. }
            | SemanticEvent::Exported { range } => range,
        }
    }

    pub fn str<'a>(&self, code: &'a str) -> &'a str {
        let range = self.range();
        let start: u32 = range.start().into();
        let end: u32 = range.end().into();
        &code[start as usize..end as usize]
    }
}

/// Extracts [SemanticEvent] from [JsSyntaxNode].
///
/// The extraction is not entirely pull based, nor entirely push based.
/// This happens because some nodes can generate multiple events.
/// A hoisted variable declaration like ```var a```, being the more obvious
/// example. As soon ```a``` is hoisted, all references of ```a``` are solved
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
    stash: VecDeque<SemanticEvent>,
    scopes: Vec<Scope>,
    next_scope_id: usize,
    /// At any point this is the set of available bindings and
    /// the range of its declaration
    bindings: FxHashMap<TokenText, BindingInfo>,
}

/// Holds the text range of the token when it is bound,
/// along with the kind of declaration
#[derive(Debug)]
struct BindingInfo {
    text_range: TextRange,
    /// For export determination, it is necessary to provide
    /// information on how a specific token is bound
    declaration_kind: JsSyntaxKind,
}

#[derive(Debug)]
struct Binding {
    name: TokenText,
}

#[derive(Debug)]
enum Reference {
    Read { range: TextRange, is_exported: bool },
    Write { range: TextRange },
}

impl Reference {
    fn is_read(&self) -> bool {
        matches!(self, Reference::Read { .. })
    }

    pub fn range(&self) -> TextRange {
        match self {
            Reference::Read { range, .. } | Reference::Write { range } => *range,
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
    bindings: Vec<Binding>,
    /// References that still needs to be bound and
    /// will be solved at the end of the scope
    references: HashMap<TokenText, Vec<Reference>>,
    /// All bindings that where shadowed and will be
    /// restored after this scope ends.
    shadowed: Vec<(TokenText, BindingInfo)>,
    /// If this scope allows declarations to be hoisted
    /// to parent scope or not
    hoisting: ScopeHoisting,
}

/// Returns the node that defines the result of the expression
fn result_of(expr: &JsParenthesizedExpression) -> Option<AnyJsExpression> {
    let mut expr = Some(AnyJsExpression::JsParenthesizedExpression(expr.clone()));
    loop {
        match expr {
            Some(AnyJsExpression::JsParenthesizedExpression(e)) => {
                expr = e.expression().ok();
            }
            Some(AnyJsExpression::JsSequenceExpression(e)) => {
                expr = e.right().ok();
            }
            Some(AnyJsExpression::JsAssignmentExpression(e)) => {
                expr = e.right().ok();
            }
            Some(expr) => return Some(expr),
            None => return None,
        }
    }
}

impl SemanticEventExtractor {
    pub fn new() -> Self {
        Self {
            stash: VecDeque::new(),
            scopes: vec![],
            next_scope_id: 0,
            bindings: FxHashMap::default(),
        }
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    #[inline]
    pub fn enter(&mut self, node: &JsSyntaxNode) {
        use biome_js_syntax::JsSyntaxKind::*;

        match node.kind() {
            JS_IDENTIFIER_BINDING | TS_IDENTIFIER_BINDING | TS_TYPE_PARAMETER_NAME => {
                self.enter_identifier_binding(node);
            }
            JS_REFERENCE_IDENTIFIER | JSX_REFERENCE_IDENTIFIER => {
                self.enter_reference_identifier(node);
            }
            JS_IDENTIFIER_ASSIGNMENT => {
                self.enter_js_identifier_assignment(node);
            }
            JS_CALL_EXPRESSION => {
                self.enter_js_call_expression(node);
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
            | TS_FUNCTION_TYPE
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

            _ => {}
        }
    }

    fn is_var(binding: &impl AstNode<Language = JsLanguage>) -> Option<bool> {
        let declarator = binding.parent::<JsVariableDeclarator>()?;

        use JsSyntaxKind::*;
        let is_var = match declarator.syntax().parent().kind() {
            Some(JS_VARIABLE_DECLARATOR_LIST) => declarator
                .parent::<JsVariableDeclaratorList>()?
                .parent::<JsVariableDeclaration>()?
                .is_var(),
            Some(JS_FOR_VARIABLE_DECLARATION) => {
                declarator
                    .parent::<JsForVariableDeclaration>()?
                    .kind_token()
                    .ok()?
                    .kind()
                    == VAR_KW
            }
            _ => false,
        };
        Some(is_var)
    }

    fn enter_identifier_binding(&mut self, node: &JsSyntaxNode) -> Option<()> {
        use JsSyntaxKind::*;
        debug_assert!(matches!(
            node.kind(),
            JS_IDENTIFIER_BINDING | TS_IDENTIFIER_BINDING | TS_TYPE_PARAMETER_NAME
        ), "specified node is not a identifier binding (JS_IDENTIFIER_BINDING, TS_IDENTIFIER_BINDING, TS_TYPE_PARAMETER)");

        let (name_token, is_var) = match node.kind() {
            JS_IDENTIFIER_BINDING => {
                let binding = node.clone().cast::<JsIdentifierBinding>()?;
                let name_token = binding.name_token().ok()?;
                let is_var = Self::is_var(&binding);
                (name_token, is_var)
            }
            TS_IDENTIFIER_BINDING => {
                let binding = node.clone().cast::<TsIdentifierBinding>()?;
                let name_token = binding.name_token().ok()?;
                let is_var = Self::is_var(&binding);
                (name_token, is_var)
            }
            TS_TYPE_PARAMETER_NAME => {
                let token = node.clone().cast::<TsTypeParameterName>()?;
                let name_token = token.ident_token().ok()?;
                let is_var = Some(false);
                (name_token, is_var)
            }
            _ => return None,
        };

        let parent = node.parent()?;
        let parent_kind = parent.kind();
        match parent_kind {
            JS_VARIABLE_DECLARATOR => {
                if let Some(true) = is_var {
                    let hoisted_scope_id = self.scope_index_to_hoist_declarations(0);
                    self.push_binding_into_scope(hoisted_scope_id, &name_token, &parent_kind);
                } else {
                    self.push_binding_into_scope(None, &name_token, &parent_kind);
                };
                self.export_variable_declarator(node, &parent);
            }
            JS_FUNCTION_DECLARATION | JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                let hoisted_scope_id = self.scope_index_to_hoist_declarations(1);
                self.push_binding_into_scope(hoisted_scope_id, &name_token, &parent_kind);
                self.export_function_declaration(node, &parent);
            }
            JS_CLASS_EXPRESSION | JS_FUNCTION_EXPRESSION => {
                self.push_binding_into_scope(None, &name_token, &parent_kind);
                self.export_declaration_expression(node, &parent);
            }
            JS_CLASS_DECLARATION
            | JS_CLASS_EXPORT_DEFAULT_DECLARATION
            | TS_ENUM_DECLARATION
            | TS_INTERFACE_DECLARATION
            | TS_MODULE_DECLARATION
            | TS_TYPE_ALIAS_DECLARATION => {
                let parent_scope = self.scopes.get(self.scopes.len() - 2);
                let parent_scope = parent_scope.map(|scope| scope.scope_id);
                self.push_binding_into_scope(parent_scope, &name_token, &parent_kind);
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
                self.push_binding_into_scope(None, &name_token, &parent_kind);

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

                if let JS_VARIABLE_DECLARATOR = possible_declarator.kind() {
                    self.export_variable_declarator(node, &possible_declarator);
                }
            }
            _ => {
                self.push_binding_into_scope(None, &name_token, &parent_kind);
            }
        }

        Some(())
    }

    fn enter_reference_identifier(&mut self, node: &JsSyntaxNode) -> Option<()> {
        debug_assert!(
            matches!(
                node.kind(),
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER | JsSyntaxKind::JSX_REFERENCE_IDENTIFIER
            ),
            "specified node is not a reference identifier (JS_REFERENCE_IDENTIFIER, JSX_REFERENCE_IDENTIFIER)"
        );

        let (name, is_exported) = match node.kind() {
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                // SAFETY: kind check above
                let reference = JsReferenceIdentifier::unwrap_cast(node.clone());
                let name_token = reference.value_token().ok()?;
                // skip `this` reference representing the class instance
                if name_token.token_text_trimmed() == "this" {
                    return None;
                }
                (
                    name_token.token_text_trimmed(),
                    self.is_js_reference_identifier_exported(node),
                )
            }
            JsSyntaxKind::JSX_REFERENCE_IDENTIFIER => {
                // SAFETY: kind check above
                let reference = JsxReferenceIdentifier::unwrap_cast(node.clone());
                let name_token = reference.value_token().ok()?;
                (name_token.token_text_trimmed(), false)
            }
            _ => return None,
        };

        let current_scope = self.current_scope_mut();
        let references = current_scope.references.entry(name).or_default();
        references.push(Reference::Read {
            range: node.text_range(),
            is_exported,
        });

        Some(())
    }

    fn enter_js_identifier_assignment(&mut self, node: &JsSyntaxNode) -> Option<()> {
        debug_assert!(
            matches!(node.kind(), JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT),
            "specified node is not a identifier assignment (JS_IDENTIFIER_ASSIGNMENT)"
        );

        let reference = node.clone().cast::<JsIdentifierAssignment>()?;
        let name_token = reference.name_token().ok()?;
        let name = name_token.token_text_trimmed();

        let current_scope = self.current_scope_mut();
        let references = current_scope.references.entry(name).or_default();
        references.push(Reference::Write {
            range: node.text_range(),
        });

        Some(())
    }

    fn enter_js_call_expression(&mut self, node: &JsSyntaxNode) -> Option<()> {
        debug_assert!(matches!(node.kind(), JsSyntaxKind::JS_CALL_EXPRESSION));

        let call = node.clone().cast::<JsCallExpression>()?;
        let callee = call.callee().ok()?;

        if let JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION = callee.syntax().kind() {
            let expr = callee.as_js_parenthesized_expression()?;
            let range = expr.syntax().text_range();
            if let Some(AnyJsExpression::JsFunctionExpression(expr)) = result_of(expr) {
                let id = expr.id()?;
                self.stash.push_back(SemanticEvent::Read {
                    range,
                    declared_at: id.syntax().text_range(),
                    scope_id: self.scopes.last().unwrap().scope_id,
                });
            }
        }

        Some(())
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    #[inline]
    pub fn leave(&mut self, node: &JsSyntaxNode) {
        use biome_js_syntax::JsSyntaxKind::*;

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
            | TS_FUNCTION_TYPE
            | TS_INTERFACE_DECLARATION
            | TS_ENUM_DECLARATION
            | TS_TYPE_ALIAS_DECLARATION
            | TS_MODULE_DECLARATION => {
                self.pop_scope(node.text_range());
            }
            _ => {}
        }
    }

    /// Return any previous extracted [SemanticEvent].
    #[inline]
    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }

    fn push_scope(&mut self, range: TextRange, hoisting: ScopeHoisting, is_closure: bool) {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;

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

        if let Some(scope) = self.scopes.pop() {
            // Match references and declarations
            for (name, mut references) in scope.references {
                // If we know the declaration of these reference push the correct events...
                if let Some(declared_binding) = self.bindings.get(&name) {
                    let declaration_at = declared_binding.text_range;
                    let declaration_syntax_kind = declared_binding.declaration_kind;

                    for reference in &references {
                        let declaration_before_reference =
                            declaration_at.start() < reference.range().start();
                        let e = match (declaration_before_reference, &reference) {
                            (true, Reference::Read { range, .. }) => SemanticEvent::Read {
                                range: *range,
                                declared_at: declaration_at,
                                scope_id: scope.scope_id,
                            },
                            (false, Reference::Read { range, .. }) => SemanticEvent::HoistedRead {
                                range: *range,
                                declared_at: declaration_at,
                                scope_id: scope.scope_id,
                            },
                            (true, Reference::Write { range }) => SemanticEvent::Write {
                                range: *range,
                                declared_at: declaration_at,
                                scope_id: scope.scope_id,
                            },
                            (false, Reference::Write { range }) => SemanticEvent::HoistedWrite {
                                range: *range,
                                declared_at: declaration_at,
                                scope_id: scope.scope_id,
                            },
                        };
                        self.stash.push_back(e);

                        match reference {
                            Reference::Read { is_exported, .. } if *is_exported => {
                                // Check shadowed bindings to find an exportable binding
                                let find_exportable_binding = scope.shadowed.iter().find_map(
                                    |(shadowed_ident_name, shadowed_binding_info)| {
                                        if shadowed_ident_name != &name {
                                            return None;
                                        }

                                        // The order of interface and other bindings is valid in either order
                                        match (
                                            declaration_syntax_kind,
                                            shadowed_binding_info.declaration_kind,
                                        ) {
                                            (
                                                JsSyntaxKind::JS_VARIABLE_DECLARATOR,
                                                JsSyntaxKind::TS_INTERFACE_DECLARATION,
                                            )
                                            | (
                                                JsSyntaxKind::TS_INTERFACE_DECLARATION,
                                                JsSyntaxKind::JS_VARIABLE_DECLARATOR,
                                            )
                                            | (
                                                JsSyntaxKind::JS_CLASS_DECLARATION,
                                                JsSyntaxKind::TS_INTERFACE_DECLARATION,
                                            )
                                            | (
                                                JsSyntaxKind::TS_INTERFACE_DECLARATION,
                                                JsSyntaxKind::JS_CLASS_DECLARATION,
                                            ) => Some(shadowed_binding_info),
                                            _ => None,
                                        }
                                    },
                                );
                                if let Some(binding_info) = find_exportable_binding {
                                    self.stash.push_back(SemanticEvent::Exported {
                                        range: binding_info.text_range,
                                    });
                                }

                                self.stash.push_back(SemanticEvent::Exported {
                                    range: declaration_at,
                                });
                            }
                            _ => {}
                        }
                    }
                } else if let Some(parent) = self.scopes.last_mut() {
                    // ... if not, promote these references to the parent scope ...
                    let parent_references = parent.references.entry(name).or_default();
                    parent_references.append(&mut references);
                } else {
                    // ... or raise UnresolvedReference if this is the global scope.
                    for reference in references {
                        self.stash.push_back(SemanticEvent::UnresolvedReference {
                            is_read: reference.is_read(),
                            range: reference.range(),
                        });
                    }
                }
            }

            // Remove all bindings declared in this scope
            for binding in scope.bindings {
                self.bindings.remove(&binding.name);
            }

            // Restore shadowed bindings
            self.bindings.extend(scope.shadowed);

            self.stash.push_back(SemanticEvent::ScopeEnded {
                range,
                started_at: scope.started_at,
                scope_id: scope.scope_id,
            });
        }
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
        declaration_kind: &JsSyntaxKind,
    ) {
        let name = name_token.token_text_trimmed();
        let declaration_range = name_token.text_range();

        // insert this name into the list of available names
        // and save shadowed names to be used later
        let shadowed = self
            .bindings
            .insert(
                name.clone(),
                BindingInfo {
                    text_range: declaration_range,
                    declaration_kind: *declaration_kind,
                },
            )
            .map(|shadowed_range| (name.clone(), shadowed_range));

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

        scope.bindings.push(Binding { name: name.clone() });
        scope.shadowed.extend(shadowed);

        let scope_started_at = scope.started_at;

        self.stash.push_back(SemanticEvent::DeclarationFound {
            range: declaration_range,
            scope_started_at,
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
        use JsSyntaxKind::*;
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
        use JsSyntaxKind::*;
        debug_assert!(matches!(
            declaration_expression.kind(),
            JS_FUNCTION_EXPRESSION | JS_CLASS_EXPRESSION
        ));
        let is_module_exports = declaration_expression
            .parent()
            .map(|x| self.is_assignment_left_side_module_exports(&x))
            .unwrap_or(false);
        if is_module_exports {
            self.stash.push_back(SemanticEvent::Exported {
                range: binding.text_range(),
            });
        }
    }

    // Check if a class, type alias, enum, interface, module is exported and raise the [Exported] event.
    fn export_declaration(&mut self, binding: &JsSyntaxNode, declaration: &JsSyntaxNode) {
        use JsSyntaxKind::*;
        debug_assert!(matches!(
            declaration.kind(),
            JS_CLASS_DECLARATION
                | JS_CLASS_EXPORT_DEFAULT_DECLARATION
                | TS_TYPE_ALIAS_DECLARATION
                | TS_ENUM_DECLARATION
                | TS_INTERFACE_DECLARATION
                | TS_MODULE_DECLARATION
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
    fn export_variable_declarator(
        &mut self,
        binding: &JsSyntaxNode,
        variable_declarator: &JsSyntaxNode,
    ) {
        use JsSyntaxKind::*;
        debug_assert!(matches!(variable_declarator.kind(), JS_VARIABLE_DECLARATOR));
        let is_exported = matches!(
            variable_declarator
                .parent()
                .and_then(|list| list.parent())
                .and_then(|declaration| declaration.parent())
                .and_then(|declaration_clause| declaration_clause.parent())
                .map(|x| x.kind()),
            Some(JS_EXPORT) | Some(TS_EXPORT_DECLARE_CLAUSE)
        );

        if is_exported {
            self.stash.push_back(SemanticEvent::Exported {
                range: binding.text_range(),
            });
        }
    }

    // Check if a reference is exported and raise the [Exported] event.
    fn is_js_reference_identifier_exported(&mut self, reference: &JsSyntaxNode) -> bool {
        use JsSyntaxKind::*;
        debug_assert!(matches!(reference.kind(), JS_REFERENCE_IDENTIFIER));
        let reference_parent = reference.parent();
        let reference_greatparent = reference_parent.as_ref().and_then(|p| p.parent());

        // check export keyword
        matches!(
            reference_parent.kind(),
            Some(JS_EXPORT_NAMED_SHORTHAND_SPECIFIER | JS_EXPORT_NAMED_SPECIFIER)
        ) | {
            // check "export default" keyword
            matches!(
                reference_greatparent.kind(),
                Some(JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE)
            )
        } | {
            // check module.exports
            match reference_parent.map(|x| x.kind()) {
                Some(JS_IDENTIFIER_EXPRESSION) => reference_greatparent
                    .map(|x| self.is_assignment_left_side_module_exports(&x))
                    .unwrap_or(false),
                Some(JS_SHORTHAND_PROPERTY_OBJECT_MEMBER) => reference_greatparent
                    .and_then(|g| g.grand_parent())
                    .map(|x| self.is_assignment_left_side_module_exports(&x))
                    .unwrap_or(false),
                _ => false,
            }
        }
    }

    fn is_assignment_left_side_module_exports(&self, node: &JsSyntaxNode) -> bool {
        match node.kind() {
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let expr = node.clone().cast::<JsAssignmentExpression>();
                match expr.and_then(|x| x.left().ok()) {
                    Some(AnyJsAssignmentPattern::AnyJsAssignment(
                        AnyJsAssignment::JsStaticMemberAssignment(a),
                    )) => {
                        let first = a
                            .object()
                            .ok()
                            .and_then(|x| x.as_js_identifier_expression().cloned())
                            .and_then(|x| x.name().ok())
                            .and_then(|x| x.value_token().ok())
                            .map(|x| x.token_text_trimmed());

                        match first {
                            Some(first) if first == "module" => {
                                let second = a
                                    .member()
                                    .ok()
                                    .and_then(|x| x.as_js_name().cloned())
                                    .and_then(|x| x.value_token().ok())
                                    .map(|x| x.token_text_trimmed());
                                // module.exports = ..
                                matches!(second, Some(second) if second == "exports")
                            }
                            // exports.<anything> = ..
                            Some(first) if first == "exports" => true,
                            _ => false,
                        }
                    }
                    // exports = ...
                    Some(AnyJsAssignmentPattern::AnyJsAssignment(
                        AnyJsAssignment::JsIdentifierAssignment(ident),
                    )) => ident.syntax().text_trimmed() == "exports",
                    _ => false,
                }
            }
            _ => false,
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
