use super::*;
use biome_js_syntax::{AnyJsFunction, AnyJsRoot};

#[derive(Copy, Clone, Debug)]
pub(crate) struct BindingIndex(usize);

impl From<usize> for BindingIndex {
    fn from(v: usize) -> Self {
        BindingIndex(v)
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct ReferenceIndex(usize, usize);

impl ReferenceIndex {
    pub(crate) fn binding(&self) -> BindingIndex {
        BindingIndex(self.0)
    }
}

impl From<(BindingIndex, usize)> for ReferenceIndex {
    fn from((binding_index, index): (BindingIndex, usize)) -> Self {
        ReferenceIndex(binding_index.0, index)
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
    pub(crate) scope_by_range: rust_lapper::Lapper<usize, usize>,
    // Maps the start of a node range to a scope id
    pub(crate) scope_hoisted_to_by_range: FxHashMap<TextSize, usize>,
    // Map to each by its range
    pub(crate) node_by_range: FxHashMap<TextRange, JsSyntaxNode>,
    // Maps any range start in the code to its bindings (usize points to bindings vec)
    pub(crate) declared_at_by_start: FxHashMap<TextSize, usize>,
    // List of all the declarations
    pub(crate) bindings: Vec<SemanticModelBindingData>,
    // Index bindings by range start
    pub(crate) bindings_by_start: FxHashMap<TextSize, usize>,
    // All bindings that were exported
    pub(crate) exported: FxHashSet<TextSize>,
    /// All references that could not be resolved
    pub(crate) unresolved_references: Vec<SemanticModelUnresolvedReference>,
    /// All globals references
    pub(crate) globals: Vec<SemanticModelGlobalBindingData>,
}

impl SemanticModelData {
    pub(crate) fn binding(&self, index: BindingIndex) -> &SemanticModelBindingData {
        &self.bindings[index.0]
    }

    pub(crate) fn reference(&self, index: ReferenceIndex) -> &SemanticModelReference {
        let binding = &self.bindings[index.0];
        &binding.references[index.1]
    }

    pub(crate) fn next_reference(&self, index: ReferenceIndex) -> Option<&SemanticModelReference> {
        let binding = &self.bindings[index.0];
        binding.references.get(index.1 + 1)
    }

    pub(crate) fn scope(&self, range: &TextRange) -> usize {
        let start = range.start().into();
        let end = range.end().into();
        let scopes = self
            .scope_by_range
            .find(start, end)
            .filter(|x| !(start < x.start || end > x.stop));

        // We always want the most tight scope
        match scopes.map(|x| x.val).max() {
            Some(val) => val,
            // We always have at least one scope, the global one.
            None => unreachable!("Expected global scope not present"),
        }
    }

    fn scope_hoisted_to(&self, range: &TextRange) -> Option<usize> {
        self.scope_hoisted_to_by_range.get(&range.start()).copied()
    }

    pub fn is_exported(&self, range: TextRange) -> bool {
        self.exported.contains(&range.start())
    }
}

impl PartialEq for SemanticModelData {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl Eq for SemanticModelData {}

/// The façade for all semantic information.
/// - Scope: [Scope]
/// - Declarations: [declaration]
///
/// See [SemanticModelData] for more information about the internals.
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
            id,
        })
    }

    /// Returns the global scope of the model
    pub fn global_scope(&self) -> Scope {
        Scope {
            data: self.data.clone(),
            id: 0,
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
    ///     .find(|x| x.text() == "arguments")
    ///     .unwrap();
    ///
    /// let block_scope = model.scope(&arguments_reference.syntax());
    /// // or
    /// let block_scope = arguments_reference.scope(&model);
    /// ```
    pub fn scope(&self, node: &JsSyntaxNode) -> Scope {
        let range = node.text_range();
        let id = self.data.scope(&range);
        Scope {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the [Scope] which the specified syntax node was hoisted to, if any.
    /// Can also be called from [AstNode]::scope_hoisted_to extension method.
    pub fn scope_hoisted_to(&self, node: &JsSyntaxNode) -> Option<Scope> {
        let range = node.text_range();
        let id = self.data.scope_hoisted_to(&range)?;
        Some(Scope {
            data: self.data.clone(),
            id,
        })
    }

    pub fn all_bindings(&self) -> impl Iterator<Item = Binding> + '_ {
        self.data.bindings.iter().map(|x| Binding {
            data: self.data.clone(),
            index: x.id,
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
    ///     .find(|x| x.text() == "arguments")
    ///     .unwrap();
    ///
    /// let arguments_binding = model.binding(&arguments_reference);
    /// // or
    /// let arguments_binding = arguments_reference.binding(&model);
    /// ```
    pub fn binding(&self, reference: &impl HasDeclarationAstNode) -> Option<Binding> {
        let reference = reference.node();
        let range = reference.syntax().text_range();
        let id = *self.data.declared_at_by_start.get(&range.start())?;
        Some(Binding {
            data: self.data.clone(),
            index: id.into(),
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
            while global_id < current.data.globals.len() {
                let reference = current
                    .data
                    .globals
                    .get(global_id)
                    .and_then(|global| global.references.get(id))
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
                .get(id)
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
        let range = binding.syntax().text_range();
        let id = &self.data.bindings_by_start[&range.start()];
        Binding {
            data: self.data.clone(),
            index: (*id).into(),
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
