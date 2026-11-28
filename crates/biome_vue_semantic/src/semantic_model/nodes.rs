//! The syntax nodes rows of the model stand for.
//!
//! The model is shared between the analyzers of the host document and of each
//! embedded snippet, which run on trees of different languages. A row that
//! stands for a node names it with a [`NodeId`] of the node's language, and
//! only an analysis of the tree holding the node can turn the id into the
//! node: an [`AnalyzedVueModel`] is built from the [`AnalyzedTree`] an
//! analyzer gives its visitors, which can't be made from a root alone. A rule of the analyzer of
//! an HTML document therefore can't resolve a JavaScript node of a snippet,
//! and the reverse.
//!
//! The same analysis finds the row a node stands for: it keeps the id of each
//! node it resolved, keyed by the node.

use super::model::{
    Attr, Component, Computed, Element, Emit, HtmlRow, Method, Prop, PropDefault, Reference,
    SnippetId, Symbol, VueSemanticModel,
};
use biome_analyze::AnalyzedTree;
use biome_html_syntax::{
    AnyHtmlAttribute, HtmlElement, HtmlLanguage, HtmlRoot, HtmlSelfClosingElement,
};
use biome_js_syntax::{
    AnyJsRoot, JsLanguage, JsPropertyObjectMember, JsShorthandPropertyObjectMember,
    JsStringLiteralExpression, TsMethodSignatureTypeMember, TsPropertySignatureTypeMember,
};
use biome_rowan::{AstNode, Language, SyntaxNode, declare_node_union};
use rustc_hash::FxHashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

/// Identifies a syntax node of language `L` that a row of the model stands
/// for.
///
/// Ids are dense over the whole model, so [`AnalyzedVueModel`] finds a node
/// in constant time.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NodeId<L: Language> {
    index: u32,
    language: PhantomData<L>,
}

impl<L: Language> NodeId<L> {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index: index as u32,
            language: PhantomData,
        }
    }

    pub(crate) fn index(self) -> usize {
        self.index as usize
    }
}

/// Identifies a JavaScript syntax node that a row of the model stands for.
pub type JsNodeId = NodeId<JsLanguage>;

/// Identifies a syntax node of the host document that a row of the model
/// stands for.
pub type HtmlNodeId = NodeId<HtmlLanguage>;

/// The nodes of one analyzed tree that rows of the model stand for.
///
/// Rows written in another tree, such as another script block of the same
/// single-file component, have no node here.
#[derive(Debug)]
pub(crate) struct Nodes<L: Language> {
    nodes: Box<[Option<SyntaxNode<L>>]>,
    /// The id of each node of `nodes`.
    ids: FxHashMap<SyntaxNode<L>, NodeId<L>>,
}

impl<L: Language> Nodes<L> {
    /// Creates the nodes of one tree from the node each id names, or `None`
    /// for an id whose node is in another tree.
    fn new(nodes: Box<[Option<SyntaxNode<L>>]>) -> Self {
        let ids = nodes
            .iter()
            .enumerate()
            .filter_map(|(index, node)| Some((node.clone()?, NodeId::new(index))))
            .collect();
        Self { nodes, ids }
    }

    /// Returns the node `id` names, or `None` when it is not in the tree
    /// these nodes were resolved in.
    pub(crate) fn get(&self, id: NodeId<L>) -> Option<&SyntaxNode<L>> {
        self.nodes.get(id.index())?.as_ref()
    }

    /// Returns the id that names `node`, or `None` when no row of the model
    /// stands for it.
    pub(crate) fn id(&self, node: &SyntaxNode<L>) -> Option<NodeId<L>> {
        self.ids.get(node).copied()
    }

    /// Returns the node `row` stands for, or `None` when it is not in the
    /// tree these nodes were resolved in.
    pub(crate) fn node<R: VueRow<L>>(&self, row: &R) -> Option<R::Node> {
        R::cast(self.get(row.node_id()?)?.clone())
    }
}

/// The Vue model as one analysis of a tree of language `L` sees it: the
/// model, and the nodes of that tree its rows stand for.
///
/// It reads as a [`VueSemanticModel`], and [`Self::node`] returns the node of
/// a row in constant time. Rows written in another tree, such as another
/// script block of the same single-file component, have no node.
///
/// The reverse lookups, such as [`Self::attr_of`], return the row a node
/// stands for in constant time.
///
/// Code that holds a root without the [`AnalyzedTree`] of its analysis
/// cannot resolve nodes:
///
/// ```compile_fail
/// # use biome_js_syntax::AnyJsRoot;
/// # use biome_vue_semantic::VueSemanticModel;
/// fn leak(model: &VueSemanticModel, root: &AnyJsRoot) {
///     let _ = model.resolve_js_nodes(root, None);
/// }
/// ```
///
/// Nor can a row be resolved in a tree of another language:
///
/// ```compile_fail
/// # use biome_html_syntax::HtmlLanguage;
/// # use biome_vue_semantic::{AnalyzedVueModel, JsNodeId};
/// fn cross(model: &AnalyzedVueModel<HtmlLanguage>, id: JsNodeId) {
///     let _ = model.node(&id);
/// }
/// ```
///
/// ```compile_fail
/// # use biome_analyze::AnalyzedTree;
/// # use biome_html_syntax::HtmlLanguage;
/// # use biome_js_syntax::JsLanguage;
/// # use biome_vue_semantic::{AnalyzedVueModel, VueSemanticModel};
/// fn cross(model: VueSemanticModel, tree: AnalyzedTree<'_, HtmlLanguage>) {
///     let _ = AnalyzedVueModel::<JsLanguage>::new(model, tree, None);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct AnalyzedVueModel<L: Language> {
    model: VueSemanticModel,
    nodes: Rc<Nodes<L>>,
}

impl<L: Language> AnalyzedVueModel<L> {
    /// Returns the node `row` stands for, or `None` when it is not in the
    /// analyzed tree.
    pub fn node<R: VueRow<L>>(&self, row: &R) -> Option<R::Node> {
        self.nodes.node(row)
    }
}

impl AnalyzedVueModel<JsLanguage> {
    /// Resolves the JavaScript nodes rows of `model` stand for in `tree`.
    ///
    /// `snippet` is the snippet of a single-file component that `tree` is,
    /// or `None` when `tree` is a whole JavaScript file.
    pub fn new(
        model: VueSemanticModel,
        tree: AnalyzedTree<'_, JsLanguage>,
        snippet: Option<SnippetId>,
    ) -> Self {
        let nodes = Rc::new(model.resolve_js_nodes(tree.root(), snippet));
        Self { model, nodes }
    }

    /// Returns the reference of the model that is written at `node`.
    ///
    /// `node` must be the node that holds the name, the one
    /// [`Reference::range`] covers:
    ///
    /// - For `title`, the `JsReferenceIdentifier`, or the
    ///   `JsIdentifierAssignment` in `title = 1`. The `JsIdentifierExpression`
    ///   around a reference is not that node.
    /// - For `this.title`, the `JsName` of the member.
    /// - For `emit('change')`, the `JsStringLiteralExpression`.
    ///
    /// Returns `None` for any other node, and for a name the model records no
    /// reference for.
    pub fn reference_of(&self, node: &impl AstNode<Language = JsLanguage>) -> Option<Reference> {
        let id = self.nodes.id(node.syntax())?;
        let reference = self.model.data.js_nodes[id.index()].reference?;
        Some(self.model.reference(reference))
    }
}

impl AnalyzedVueModel<HtmlLanguage> {
    /// Resolves the nodes rows of `model` stand for in `tree`, the tree of
    /// the host document.
    pub fn new(model: VueSemanticModel, tree: AnalyzedTree<'_, HtmlLanguage>) -> Self {
        let nodes = Rc::new(model.resolve_html_nodes(tree.root()));
        Self { model, nodes }
    }

    /// Returns the element of the model that `node` is.
    ///
    /// `node` must be the whole element, an [`HtmlElement`] or an
    /// [`HtmlSelfClosingElement`]. Returns `None` for any other node, such as
    /// the opening tag of an element, and for every node when the model does
    /// not describe the analyzed tree.
    pub fn element_of(&self, node: &impl AstNode<Language = HtmlLanguage>) -> Option<Element> {
        match self.row_of(node)? {
            HtmlRow::Element(id) => Some(self.model.element(id)),
            HtmlRow::Attr(_) => None,
        }
    }

    /// Returns the attribute or directive of the model that `node` is.
    ///
    /// Returns `None` for any other node, for an attribute the model does not
    /// describe, such as a directive the parser could not read, and for every
    /// node when the model does not describe the analyzed tree.
    pub fn attr_of(&self, node: &impl AstNode<Language = HtmlLanguage>) -> Option<Attr> {
        match self.row_of(node)? {
            HtmlRow::Attr(id) => Some(Attr {
                data: self.model.data.clone(),
                id,
            }),
            HtmlRow::Element(_) => None,
        }
    }

    fn row_of(&self, node: &impl AstNode<Language = HtmlLanguage>) -> Option<HtmlRow> {
        let id = self.nodes.id(node.syntax())?;
        Some(self.model.data.html_nodes[id.index()].row)
    }
}

impl<L: Language> Deref for AnalyzedVueModel<L> {
    type Target = VueSemanticModel;

    fn deref(&self) -> &VueSemanticModel {
        &self.model
    }
}

/// A row of the model that stands for a syntax node of language `L`.
pub trait VueRow<L: Language> {
    type Node;

    /// The id of the node, or `None` when the row stands for no node of `L`.
    fn node_id(&self) -> Option<NodeId<L>>;

    /// Converts the node of the row to [`Self::Node`].
    fn cast(node: SyntaxNode<L>) -> Option<Self::Node>;
}

/// An id the model gives for a node, such as [`Computed::getter_node`].
impl<L: Language> VueRow<L> for NodeId<L> {
    type Node = SyntaxNode<L>;

    fn node_id(&self) -> Option<Self> {
        Some(*self)
    }

    fn cast(node: SyntaxNode<L>) -> Option<Self::Node> {
        Some(node)
    }
}

/// Implements [`VueRow`] for rows whose node has no type of its own, with
/// `$node_id` giving the id of the node of `$row`.
macro_rules! impl_vue_row {
    ($language:ty, |$row:ident| $node_id:expr; $($ty:ty),+) => {
        $(
            impl VueRow<$language> for $ty {
                type Node = SyntaxNode<$language>;

                fn node_id(&self) -> Option<NodeId<$language>> {
                    let $row = self;
                    $node_id
                }

                fn cast(node: SyntaxNode<$language>) -> Option<Self::Node> {
                    Some(node)
                }
            }
        )+
    };
}

// The node of a declaration is the one `Symbol::declaration_range` covers.
impl_vue_row!(JsLanguage, |row| Symbol::js_node(row); Symbol, Emit, Method, Computed);

// The node of a component is the one `Component::range` covers, and the node
// of a default value the one `PropDefault::range` covers.
impl_vue_row!(JsLanguage, |row| Some(row.js_node()); Component, PropDefault);

// The node of a reference is the one `Reference::range` covers.
impl_vue_row!(JsLanguage, |row| row.js_node(); Reference);

impl VueRow<JsLanguage> for Prop {
    type Node = AnyVuePropNode;

    fn node_id(&self) -> Option<NodeId<JsLanguage>> {
        Symbol::js_node(self)
    }

    fn cast(node: SyntaxNode<JsLanguage>) -> Option<Self::Node> {
        AnyVuePropNode::cast(node)
    }
}

impl VueRow<HtmlLanguage> for Element {
    type Node = AnyVueElementNode;

    fn node_id(&self) -> Option<NodeId<HtmlLanguage>> {
        Some(self.html_node())
    }

    fn cast(node: SyntaxNode<HtmlLanguage>) -> Option<Self::Node> {
        AnyVueElementNode::cast(node)
    }
}

impl VueRow<HtmlLanguage> for Attr {
    type Node = AnyHtmlAttribute;

    fn node_id(&self) -> Option<NodeId<HtmlLanguage>> {
        Some(self.html_node())
    }

    fn cast(node: SyntaxNode<HtmlLanguage>) -> Option<Self::Node> {
        AnyHtmlAttribute::cast(node)
    }
}

declare_node_union! {
    /// The declaration of a prop, in each syntax Vue accepts:
    ///
    /// - `props: ['name']` declares it with a string.
    /// - `props: { name: String }` declares it with a property.
    /// - `props: { name }` declares it with a shorthand property.
    /// - `defineProps<{ name: string }>()` declares it with a property or
    ///   method signature.
    pub AnyVuePropNode =
        JsStringLiteralExpression
        | JsPropertyObjectMember
        | JsShorthandPropertyObjectMember
        | TsPropertySignatureTypeMember
        | TsMethodSignatureTypeMember
}

declare_node_union! {
    /// An element of the host document: `<div></div>` or `<div />`.
    pub AnyVueElementNode = HtmlElement | HtmlSelfClosingElement
}

impl VueSemanticModel {
    /// Resolves the JavaScript nodes of the tree of `snippet`, rooted at
    /// `root`.
    ///
    /// Only the analyzer of `root` may see its nodes, so callers outside this
    /// crate go through [`AnalyzedVueModel::new`].
    pub(crate) fn resolve_js_nodes(
        &self,
        root: &AnyJsRoot,
        snippet: Option<SnippetId>,
    ) -> Nodes<JsLanguage> {
        let nodes = self
            .data
            .js_nodes
            .iter()
            .map(|node| {
                (node.snippet == snippet)
                    .then(|| node.ptr.try_to_node(root.syntax()))
                    .flatten()
            })
            .collect();
        Nodes::new(nodes)
    }

    /// Resolves the nodes of the host document rooted at `root`.
    ///
    /// Only the analyzer of `root` may see its nodes, so callers outside this
    /// crate go through [`AnalyzedVueModel::new`].
    pub(crate) fn resolve_html_nodes(&self, root: &HtmlRoot) -> Nodes<HtmlLanguage> {
        let nodes = self
            .data
            .html_nodes
            .iter()
            .map(|node| node.ptr.try_to_node(root.syntax()))
            .collect();
        Nodes::new(nodes)
    }
}
