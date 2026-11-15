//! The syntax nodes rows of the model stand for.
//!
//! The model is shared between the analyzers of the host document and of each
//! embedded snippet, which run on trees of different languages. A row that
//! stands for a node names it with a [`NodeId`] of the node's language, and
//! only an analysis of the tree holding the node can turn the id into the
//! node: an [`AnalyzedSvelteModel`] is built from the [`AnalyzedTree`] an
//! analyzer gives its visitors, which can't be made from a root alone. A rule
//! of the analyzer of an HTML document therefore can't resolve a JavaScript
//! node of a snippet, and the reverse.

use super::model::{Attr, Block, Element, RuneCall, SnippetId, SvelteSemanticModel, Symbol};
use biome_analyze::AnalyzedTree;
use biome_html_syntax::{
    AnyHtmlAttribute, AnySvelteBlock, HtmlElement, HtmlLanguage, HtmlRoot, HtmlSelfClosingElement,
};
use biome_js_syntax::{AnyJsRoot, JsCallExpression, JsLanguage};
use biome_rowan::{AstNode, Language, SyntaxNode, declare_node_union};
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

/// Identifies a syntax node of language `L` that a row of the model stands
/// for.
///
/// Ids are dense over the whole model, so [`AnalyzedSvelteModel`] finds a
/// node in constant time.
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
/// Rows written in another tree have no node here.
#[derive(Debug)]
pub(crate) struct Nodes<L: Language> {
    nodes: Box<[Option<SyntaxNode<L>>]>,
}

impl<L: Language> Nodes<L> {
    fn get(&self, id: NodeId<L>) -> Option<&SyntaxNode<L>> {
        self.nodes.get(id.index())?.as_ref()
    }

    fn node<R: SvelteRow<L>>(&self, row: &R) -> Option<R::Node> {
        R::cast(self.get(row.node_id()?)?.clone())
    }
}

/// The Svelte model as one analysis of a tree of language `L` sees it: the
/// model, and the nodes of that tree its rows stand for.
///
/// It reads as a [`SvelteSemanticModel`], and [`Self::node`] returns the
/// node of a row in constant time. Rows written in another tree have no
/// node.
///
/// Code that holds a root without the [`AnalyzedTree`] of its analysis
/// cannot resolve nodes:
///
/// ```compile_fail
/// # use biome_js_syntax::AnyJsRoot;
/// # use biome_svelte_semantic::SvelteSemanticModel;
/// fn leak(model: &SvelteSemanticModel, root: &AnyJsRoot) {
///     let _ = model.resolve_js_nodes(root, None);
/// }
/// ```
///
/// Nor can a row be resolved in a tree of another language:
///
/// ```compile_fail
/// # use biome_html_syntax::HtmlLanguage;
/// # use biome_svelte_semantic::{AnalyzedSvelteModel, JsNodeId};
/// fn cross(model: &AnalyzedSvelteModel<HtmlLanguage>, id: JsNodeId) {
///     let _ = model.node(&id);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct AnalyzedSvelteModel<L: Language> {
    model: SvelteSemanticModel,
    nodes: Rc<Nodes<L>>,
}

impl<L: Language> AnalyzedSvelteModel<L> {
    /// Returns the node `row` stands for, or `None` when it is not in the
    /// analyzed tree.
    pub fn node<R: SvelteRow<L>>(&self, row: &R) -> Option<R::Node> {
        self.nodes.node(row)
    }
}

impl AnalyzedSvelteModel<JsLanguage> {
    /// Resolves the JavaScript nodes rows of `model` stand for in `tree`.
    ///
    /// `snippet` is the snippet of a component that `tree` is, or `None`
    /// when `tree` is a whole module.
    pub fn new(
        model: SvelteSemanticModel,
        tree: AnalyzedTree<'_, JsLanguage>,
        snippet: Option<SnippetId>,
    ) -> Self {
        let nodes = Rc::new(model.resolve_js_nodes(tree.root(), snippet));
        Self { model, nodes }
    }
}

impl AnalyzedSvelteModel<HtmlLanguage> {
    /// Resolves the nodes rows of `model` stand for in `tree`, the tree of
    /// the host document.
    pub fn new(model: SvelteSemanticModel, tree: AnalyzedTree<'_, HtmlLanguage>) -> Self {
        let nodes = Rc::new(model.resolve_html_nodes(tree.root()));
        Self { model, nodes }
    }
}

impl<L: Language> Deref for AnalyzedSvelteModel<L> {
    type Target = SvelteSemanticModel;

    fn deref(&self) -> &SvelteSemanticModel {
        &self.model
    }
}

/// A row of the model that stands for a syntax node of language `L`.
pub trait SvelteRow<L: Language> {
    type Node;

    /// The id of the node, or `None` when the row stands for no node of `L`.
    fn node_id(&self) -> Option<NodeId<L>>;

    /// Converts the node of the row to [`Self::Node`].
    fn cast(node: SyntaxNode<L>) -> Option<Self::Node>;
}

/// An id the model gives for a node.
impl<L: Language> SvelteRow<L> for NodeId<L> {
    type Node = SyntaxNode<L>;

    fn node_id(&self) -> Option<Self> {
        Some(*self)
    }

    fn cast(node: SyntaxNode<L>) -> Option<Self::Node> {
        Some(node)
    }
}

/// The node of a declaration is the one `Symbol::declaration_range` covers.
impl SvelteRow<JsLanguage> for Symbol {
    type Node = SyntaxNode<JsLanguage>;

    fn node_id(&self) -> Option<NodeId<JsLanguage>> {
        self.js_node()
    }

    fn cast(node: SyntaxNode<JsLanguage>) -> Option<Self::Node> {
        Some(node)
    }
}

impl SvelteRow<JsLanguage> for RuneCall {
    type Node = JsCallExpression;

    fn node_id(&self) -> Option<NodeId<JsLanguage>> {
        Some(self.js_node())
    }

    fn cast(node: SyntaxNode<JsLanguage>) -> Option<Self::Node> {
        JsCallExpression::cast(node)
    }
}

impl SvelteRow<HtmlLanguage> for Element {
    type Node = AnySvelteElementNode;

    fn node_id(&self) -> Option<NodeId<HtmlLanguage>> {
        Some(self.html_node())
    }

    fn cast(node: SyntaxNode<HtmlLanguage>) -> Option<Self::Node> {
        AnySvelteElementNode::cast(node)
    }
}

impl SvelteRow<HtmlLanguage> for Attr {
    type Node = AnyHtmlAttribute;

    fn node_id(&self) -> Option<NodeId<HtmlLanguage>> {
        Some(self.html_node())
    }

    fn cast(node: SyntaxNode<HtmlLanguage>) -> Option<Self::Node> {
        AnyHtmlAttribute::cast(node)
    }
}

impl SvelteRow<HtmlLanguage> for Block {
    type Node = AnySvelteBlock;

    fn node_id(&self) -> Option<NodeId<HtmlLanguage>> {
        Some(self.html_node())
    }

    fn cast(node: SyntaxNode<HtmlLanguage>) -> Option<Self::Node> {
        AnySvelteBlock::cast(node)
    }
}

declare_node_union! {
    /// An element of the host document: `<div></div>` or `<div />`.
    pub AnySvelteElementNode = HtmlElement | HtmlSelfClosingElement
}

impl SvelteSemanticModel {
    /// Resolves the JavaScript nodes of the tree of `snippet`, rooted at
    /// `root`.
    ///
    /// Only the analyzer of `root` may see its nodes, so callers outside this
    /// crate go through [`AnalyzedSvelteModel::new`].
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
        Nodes { nodes }
    }

    /// Resolves the nodes of the host document rooted at `root`.
    ///
    /// Only the analyzer of `root` may see its nodes, so callers outside this
    /// crate go through [`AnalyzedSvelteModel::new`].
    pub(crate) fn resolve_html_nodes(&self, root: &HtmlRoot) -> Nodes<HtmlLanguage> {
        let nodes = self
            .data
            .html_nodes
            .iter()
            .map(|ptr| ptr.try_to_node(root.syntax()))
            .collect();
        Nodes { nodes }
    }
}
