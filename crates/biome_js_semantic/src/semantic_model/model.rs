use super::*;
use biome_js_syntax::{AnyJsFunction, AnyJsRoot};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct BindingId(u32);

impl BindingId {
    pub(crate) fn new(index: usize) -> Self {
        // SAFETY: We didn't handle files execedding `u32::MAX` bytes.
        // Thus, it isn't possible to execedd `u32::MAX` bindings.
        Self(index as u32)
    }

    pub(crate) fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct ReferenceId(BindingId, u32);

impl ReferenceId {
    pub(crate) fn new(binding_id: BindingId, index: usize) -> Self {
        // SAFETY: We didn't handle files execedding `u32::MAX` bytes.
        // Thus, it isn't possible to execedd `u32::MAX` refernces.
        Self(binding_id, index as u32)
    }

    // Points to [SemanticModel]::bindings vec
    pub(crate) fn binding_id(&self) -> BindingId {
        self.0
    }

    pub(crate) fn index(self) -> usize {
        self.1 as usize
    }
}

// We use `NonZeroU32` to allow niche optimizations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScopeId(std::num::NonZeroU32);

// We don't implement `From<usize> for ScopeId` and `From<ScopeId> for usize`
// to ensure that the API consumers don't create `ScopeId`.
impl ScopeId {
    pub(crate) fn new(index: usize) -> Self {
        // SAFETY: We didn't handle files execedding `u32::MAX` bytes.
        // Thus, it isn't possible to execedd `u32::MAX` scopes.
        //
        // Adding 1 ensurtes that the value is never equal to 0.
        // Instead of adding 1, we could XOR the value with `u32::MAX`.
        // This is what the [nonmax](https://docs.rs/nonmax/latest/nonmax/) crate does.
        // However, this doesn't preserve the order.
        // It is why we opted for adding 1.
        Self(unsafe { std::num::NonZeroU32::new_unchecked(index.unchecked_add(1) as u32) })
    }

    pub(crate) fn index(self) -> usize {
        // SAFETY: The internal representation ensures that the value is never equal to 0.
        // Thus, it is safe to substract 1.
        (unsafe { self.0.get().unchecked_sub(1) }) as usize
    }
}

/// Contains all the data of the [SemanticModel] and only lives behind an [Arc].
///
/// That allows any returned struct (like [Scope], [Binding])
/// to outlive the [SemanticModel], and to not include lifetimes.
#[derive(Debug)]
pub(crate) struct SemanticModelData {
    pub(crate) root: AnyJsRoot,
    // All scopes of this model
    pub(crate) scopes: Vec<SemanticModelScopeData>,
    pub(crate) scope_by_range: rust_lapper::Lapper<u32, ScopeId>,
    // Maps the start of a node range to its scope id
    pub(crate) scope_hoisted_to_by_range: FxHashMap<TextSize, ScopeId>,
    /// Binding and reference nodes indexed by their range start
    pub(crate) binding_node_by_start: FxHashMap<TextSize, JsSyntaxNode>,
    pub(crate) scope_node_by_range: FxHashMap<TextRange, JsSyntaxNode>,
    // Maps any range start in the code to its bindings
    pub(crate) declared_at_by_start: FxHashMap<TextSize, BindingId>,
    // List of all the declarations
    pub(crate) bindings: Vec<SemanticModelBindingData>,
    // Index bindings by range start
    pub(crate) bindings_by_start: FxHashMap<TextSize, BindingId>,
    // All bindings that were exported
    pub(crate) exported: FxHashSet<TextSize>,
    /// All references that could not be resolved
    pub(crate) unresolved_references: Vec<SemanticModelUnresolvedReference>,
    /// All globals references
    pub(crate) globals: Vec<SemanticModelGlobalBindingData>,
}

impl SemanticModelData {
    pub(crate) fn binding(&self, binding_id: BindingId) -> &SemanticModelBindingData {
        &self.bindings[binding_id.index()]
    }

    pub(crate) fn global(&self, global_id: u32) -> &SemanticModelGlobalBindingData {
        &self.globals[global_id as usize]
    }

    pub(crate) fn unresolved_reference(
        &self,
        unresolved_reference_id: u32,
    ) -> &SemanticModelUnresolvedReference {
        &self.unresolved_references[unresolved_reference_id as usize]
    }

    pub(crate) fn reference(&self, reference_id: ReferenceId) -> &SemanticModelReference {
        let binding = &self.binding(reference_id.binding_id());
        &binding.references[reference_id.index()]
    }

    pub(crate) fn next_reference(&self, reference_id: ReferenceId) -> Option<ReferenceId> {
        let binding = &self.binding(reference_id.binding_id());
        let next_index = reference_id.index() + 1;
        if next_index < binding.references.len() {
            Some(ReferenceId::new(reference_id.binding_id(), next_index))
        } else {
            None
        }
    }

    /// Returns the [ScopeId] which the syntax is part of.
    pub(crate) fn scope(&self, range: TextRange) -> ScopeId {
        // Seeking an interval in `self.scope_by_range` require a non-empty interval
        debug_assert!(range.len() > 0.into(), "the range must not be empty.");
        let start = range.start().into();
        let end = range.end().into();
        let scopes = self
            .scope_by_range
            // Find overlapping intervals
            .find(start, end)
            // Only take intersecting intervals
            .filter(|x| !(start < x.start || end > x.stop));
        // We always want the most tight scope
        match scopes.map(|x| x.val).max() {
            Some(val) => val,
            // We always have at least one scope, the global one.
            None => unreachable!("Expected global scope not present"),
        }
    }

    /// Returns the [ScopeId] which the specified syntax node was hoisted to, if any.
    fn scope_hoisted_to(&self, range: TextRange) -> Option<ScopeId> {
        self.scope_hoisted_to_by_range.get(&range.start()).copied()
    }

    pub fn is_exported(&self, range: TextRange) -> bool {
        self.exported.contains(&range.start())
    }

    pub fn has_exports(&self) -> bool {
        !self.exported.is_empty()
    }
}

impl PartialEq for SemanticModelData {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl Eq for SemanticModelData {}

/// The façade for all semantic information.
/// - [Scope]
/// - Declarations
///
/// See `SemanticModelData` for more information about the internals.
#[derive(Clone, Debug)]
pub struct SemanticModel {
    pub(crate) data: Rc<SemanticModelData>,
}

impl SemanticModel {
    pub(crate) fn new(data: SemanticModelData) -> Self {
        Self {
            data: Rc::new(data),
        }
    }

    /// Iterate all scopes
    pub fn scopes(&self) -> impl Iterator<Item = Scope> + '_ {
        self.data.scopes.iter().enumerate().map(|(id, _)| Scope {
            data: self.data.clone(),
            id: ScopeId::new(id),
        })
    }

    /// Returns the global scope of the model
    pub fn global_scope(&self) -> Scope {
        Scope {
            data: self.data.clone(),
            id: ScopeId::new(0),
        }
    }

    /// Returns the [Scope] which the syntax is part of.
    /// Can also be called from [AstNode]::scope extension method.
    ///
    /// ```rust
    /// use biome_js_parser::JsParserOptions;
    /// use biome_rowan::{AstNode, SyntaxNodeCast};
    /// use biome_js_syntax::{JsFileSource, JsReferenceIdentifier};
    /// use biome_js_semantic::{semantic_model, SemanticModelOptions, SemanticScopeExtensions};
    ///
    /// let r = biome_js_parser::parse("function f(){let a = arguments[0]; let b = a + 1;}", JsFileSource::js_module(), JsParserOptions::default());
    /// let model = semantic_model(&r.tree(), SemanticModelOptions::default());
    ///
    /// let arguments_reference = r
    ///     .syntax()
    ///     .descendants()
    ///     .filter_map(|x| x.cast::<JsReferenceIdentifier>())
    ///     .find(|x| x.to_trimmed_string() == "arguments")
    ///     .unwrap();
    ///
    /// let block_scope = model.scope(&arguments_reference.syntax());
    /// // or
    /// let block_scope = arguments_reference.scope(&model);
    /// ```
    pub fn scope(&self, node: &JsSyntaxNode) -> Scope {
        let range = node.text_trimmed_range();
        let id = self.data.scope(range);
        Scope {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the [Scope] which the specified syntax node was hoisted to, if any.
    /// Can also be called from [AstNode]::scope_hoisted_to extension method.
    pub fn scope_hoisted_to(&self, node: &JsSyntaxNode) -> Option<Scope> {
        let range = node.text_trimmed_range();
        let id = self.data.scope_hoisted_to(range)?;
        Some(Scope {
            data: self.data.clone(),
            id,
        })
    }

    pub fn all_bindings(&self) -> impl Iterator<Item = Binding> + '_ {
        self.data
            .bindings
            .iter()
            .enumerate()
            .map(|(index, _)| Binding {
                data: self.data.clone(),
                id: BindingId::new(index),
            })
    }

    pub fn all_exported_bindings(&self) -> impl Iterator<Item = Binding> + '_ {
        self.data
            .exported
            .iter()
            .filter_map(|declared_at| self.data.bindings_by_start.get(declared_at).copied())
            .map(|id| Binding {
                data: self.data.clone(),
                id,
            })
    }

    /// Returns the [Binding] of a reference.
    /// Can also be called from "binding" extension method.
    ///
    /// ```rust
    /// use biome_js_parser::JsParserOptions;
    /// use biome_rowan::{AstNode, SyntaxNodeCast};
    /// use biome_js_syntax::{JsFileSource, JsReferenceIdentifier};
    /// use biome_js_semantic::{semantic_model, BindingExtensions, SemanticModelOptions};
    ///
    /// let r = biome_js_parser::parse("function f(){let a = arguments[0]; let b = a + 1;}", JsFileSource::js_module(), JsParserOptions::default());
    /// let model = semantic_model(&r.tree(), SemanticModelOptions::default());
    ///
    /// let arguments_reference = r
    ///     .syntax()
    ///     .descendants()
    ///     .filter_map(|x| x.cast::<JsReferenceIdentifier>())
    ///     .find(|x| x.to_trimmed_string() == "arguments")
    ///     .unwrap();
    ///
    /// let arguments_binding = model.binding(&arguments_reference);
    /// // or
    /// let arguments_binding = arguments_reference.binding(&model);
    /// ```
    pub fn binding(&self, reference: &impl HasDeclarationAstNode) -> Option<Binding> {
        let reference = reference.node();
        let range = reference.syntax().text_trimmed_range();
        let id = *self.data.declared_at_by_start.get(&range.start())?;
        Some(Binding {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns an iterator of all the globals references in the program
    pub fn all_global_references(
        &self,
    ) -> std::iter::Successors<GlobalReference, fn(&GlobalReference) -> Option<GlobalReference>>
    {
        let first = self
            .data
            .globals
            .first()
            .and_then(|global| global.references.first())
            .map(|_| GlobalReference {
                data: self.data.clone(),
                global_id: 0,
                id: 0,
            });
        fn succ(current: &GlobalReference) -> Option<GlobalReference> {
            let mut global_id = current.global_id;
            let mut id = current.id + 1;
            while (global_id as usize) < current.data.globals.len() {
                let reference = current
                    .data
                    .globals
                    .get(global_id as usize)
                    .and_then(|global| global.references.get(id as usize))
                    .map(|_| GlobalReference {
                        data: current.data.clone(),
                        global_id,
                        id,
                    });

                match reference {
                    Some(reference) => return Some(reference),
                    None => {
                        global_id += 1;
                        id = 0;
                    }
                }
            }

            None
        }
        std::iter::successors(first, succ)
    }

    /// Returns an iterator of all the unresolved references in the program
    pub fn all_unresolved_references(
        &self,
    ) -> std::iter::Successors<
        UnresolvedReference,
        fn(&UnresolvedReference) -> Option<UnresolvedReference>,
    > {
        let first = self
            .data
            .unresolved_references
            .first()
            .map(|_| UnresolvedReference {
                data: self.data.clone(),
                id: 0,
            });
        fn succ(current: &UnresolvedReference) -> Option<UnresolvedReference> {
            let id = current.id + 1;
            current
                .data
                .unresolved_references
                .get(id as usize)
                .map(|_| UnresolvedReference {
                    data: current.data.clone(),
                    id,
                })
        }
        std::iter::successors(first, succ)
    }

    /// Returns if the node is exported or is a reference to a binding
    /// that is exported.
    ///
    /// When a binding is specified this method returns a bool.
    ///
    /// When a reference is specified this method returns `Option<bool>`,
    /// because there is no guarantee that the corresponding declaration exists.
    pub fn is_exported<T>(&self, node: &T) -> T::Result
    where
        T: CanBeImportedExported,
    {
        node.is_exported(self)
    }

    /// Returns `true` if the file contains at least one export.
    pub fn has_exports(&self) -> bool {
        self.data.has_exports()
    }

    /// Returns if the node is imported or is a reference to a binding
    /// that is imported.
    ///
    /// When a binding is specified this method returns a bool.
    ///
    /// When a reference is specified this method returns `Option<bool>`,
    /// because there is no guarantee that the corresponding declaration exists.
    pub fn is_imported<T>(&self, node: &T) -> T::Result
    where
        T: CanBeImportedExported,
    {
        node.is_imported(self)
    }

    /// Returns the [Closure] associated with the node.
    pub fn closure(&self, node: &impl HasClosureAstNode) -> Closure {
        Closure::from_node(self.data.clone(), node)
    }

    /// Returns true or false if the expression is constant, which
    /// means it does not depend on any other variables.
    pub fn is_constant(&self, expr: &AnyJsExpression) -> bool {
        is_constant::is_constant(expr)
    }

    pub fn as_binding(&self, binding: &impl IsBindingAstNode) -> Binding {
        let range = binding.syntax().text_trimmed_range();
        let id = self.data.bindings_by_start[&range.start()];
        Binding {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns all [FunctionCall] of a [AnyJsFunction].
    ///
    /// ```rust
    /// use biome_js_parser::JsParserOptions;
    /// use biome_rowan::{AstNode, SyntaxNodeCast};
    /// use biome_js_syntax::{JsFileSource, AnyJsFunction};
    /// use biome_js_semantic::{semantic_model, CallsExtensions, SemanticModelOptions};
    ///
    /// let r = biome_js_parser::parse("function f(){} f() f()", JsFileSource::js_module(), JsParserOptions::default());
    /// let model = semantic_model(&r.tree(), SemanticModelOptions::default());
    ///
    /// let f_declaration = r
    ///     .syntax()
    ///     .descendants()
    ///     .filter_map(AnyJsFunction::cast)
    ///     .next()
    ///     .unwrap();
    ///
    /// let all_calls_to_f = model.all_calls(&f_declaration);
    /// assert_eq!(2, all_calls_to_f.unwrap().count());
    /// // or
    /// let all_calls_to_f = f_declaration.all_calls(&model);
    /// assert_eq!(2, all_calls_to_f.unwrap().count());
    /// ```
    pub fn all_calls(&self, function: &AnyJsFunction) -> Option<AllCallsIter> {
        Some(AllCallsIter {
            references: function
                .binding()?
                .as_js_identifier_binding()?
                .all_reads(self),
        })
    }
}
