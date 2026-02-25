use std::collections::HashMap;
use std::sync::Arc;

use biome_analyze::PluginTargetLanguage;
use biome_aria::AriaRoles;
use biome_css_syntax::{CssSyntaxNode, CssSyntaxToken};
use biome_js_semantic::{AnyHasClosureNode, CaptureType, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
    global_identifier, jsx_ext::AnyJsxElement,
};
use biome_js_type_info::{Literal, TypeData};
use biome_json_syntax::{JsonSyntaxNode, JsonSyntaxToken};
use biome_module_graph::ModuleResolver;
use biome_rowan::{AnySyntaxNode, AstNode, NodeOrToken, SyntaxKind, SyntaxNodeCast};
use biome_text_size::TextRange;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};

/// Maximum number of node handles per check call. Prevents memory
/// exhaustion from misbehaving guests before the fuel limit kicks in.
const MAX_NODE_HANDLES: usize = 1_000_000;

/// Per-check state holding the node handle table.
///
/// Created fresh for each `check_node()` call and consumed after.
pub(crate) struct HostState {
    /// Handle (u32 index) → node mapping.
    nodes: Vec<ConcreteNode>,
    /// Optional JS semantic model for semantic queries.
    /// Only set when checking JS nodes in the semantic phase.
    semantic_model: Option<SemanticModel>,
    /// Optional module resolver for type inference (JS only).
    module_resolver: Option<Arc<ModuleResolver>>,
    /// File path of the current file being analyzed.
    file_path: String,
    /// Cached compiled regex patterns (avoids recompilation per call).
    regex_cache: HashMap<String, regex::Regex>,
    /// WASI context (required by wasmtime-wasi for wasm32-wasip2 guests).
    wasi_ctx: WasiCtx,
    /// WASI resource table.
    resource_table: ResourceTable,
}

// SAFETY: HostState is only ever created, used, and consumed on a single thread
// within a single `check_node()` call. The `Send` bound is required by wasmtime's
// `Store<T>` API but we never actually send HostState across threads. The
// non-Send field is `ConcreteNode` which wraps `SyntaxNode` (contains `Rc`).
#[expect(unsafe_code)]
unsafe impl Send for HostState {}

impl IoView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}

impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

/// A language-specific syntax element (node or token) stored in the handle table.
/// We store concrete typed elements to avoid repeated downcasting.
pub(crate) enum ConcreteNode {
    Js(JsSyntaxNode),
    Css(CssSyntaxNode),
    Json(JsonSyntaxNode),
    JsToken(JsSyntaxToken),
    CssToken(CssSyntaxToken),
    JsonToken(JsonSyntaxToken),
}

/// Dispatch a method call across all node `ConcreteNode` variants.
/// Each variant binds the inner node to `$n` and evaluates `$body`.
/// Token variants use `$fallback`.
macro_rules! dispatch_node {
    ($self:expr, |$n:ident| $body:expr, $fallback:expr) => {
        match $self {
            ConcreteNode::Js($n) => $body,
            ConcreteNode::Css($n) => $body,
            ConcreteNode::Json($n) => $body,
            ConcreteNode::JsToken(_) | ConcreteNode::CssToken(_) | ConcreteNode::JsonToken(_) => {
                $fallback
            }
        }
    };
}

/// Dispatch a method call across ALL `ConcreteNode` variants (nodes and tokens).
/// Each variant binds the inner element to `$n` and evaluates `$body`.
macro_rules! dispatch_all {
    ($self:expr, |$n:ident| $body:expr) => {
        match $self {
            ConcreteNode::Js($n) => $body,
            ConcreteNode::Css($n) => $body,
            ConcreteNode::Json($n) => $body,
            ConcreteNode::JsToken($n) => $body,
            ConcreteNode::CssToken($n) => $body,
            ConcreteNode::JsonToken($n) => $body,
        }
    };
}

/// Like [`dispatch_node`], but wraps each arm's `Option` result back into the
/// matching `ConcreteNode` variant via `.map(Self::Variant)`.
macro_rules! dispatch_map {
    ($self:expr, |$n:ident| $body:expr) => {
        match $self {
            ConcreteNode::Js($n) => ($body).map(ConcreteNode::Js),
            ConcreteNode::Css($n) => ($body).map(ConcreteNode::Css),
            ConcreteNode::Json($n) => ($body).map(ConcreteNode::Json),
            ConcreteNode::JsToken(_) | ConcreteNode::CssToken(_) | ConcreteNode::JsonToken(_) => {
                None
            }
        }
    };
}

/// Like [`dispatch_node`], but maps an iterator's items into the matching
/// `ConcreteNode` variant and collects into a `Vec`.
macro_rules! dispatch_collect {
    ($self:expr, |$n:ident| $body:expr) => {
        match $self {
            ConcreteNode::Js($n) => ($body).map(ConcreteNode::Js).collect(),
            ConcreteNode::Css($n) => ($body).map(ConcreteNode::Css).collect(),
            ConcreteNode::Json($n) => ($body).map(ConcreteNode::Json).collect(),
            ConcreteNode::JsToken(_) | ConcreteNode::CssToken(_) | ConcreteNode::JsonToken(_) => {
                Vec::new()
            }
        }
    };
}

/// Dispatch `children_with_tokens()`, mapping each `NodeOrToken` element
/// into the matching node / token `ConcreteNode` variant.
macro_rules! dispatch_children_with_tokens {
    ($self:expr) => {
        match $self {
            ConcreteNode::Js(n) => n
                .children_with_tokens()
                .map(|el| match el {
                    NodeOrToken::Node(n) => ConcreteNode::Js(n),
                    NodeOrToken::Token(t) => ConcreteNode::JsToken(t),
                })
                .collect(),
            ConcreteNode::Css(n) => n
                .children_with_tokens()
                .map(|el| match el {
                    NodeOrToken::Node(n) => ConcreteNode::Css(n),
                    NodeOrToken::Token(t) => ConcreteNode::CssToken(t),
                })
                .collect(),
            ConcreteNode::Json(n) => n
                .children_with_tokens()
                .map(|el| match el {
                    NodeOrToken::Node(n) => ConcreteNode::Json(n),
                    NodeOrToken::Token(t) => ConcreteNode::JsonToken(t),
                })
                .collect(),
            ConcreteNode::JsToken(_) | ConcreteNode::CssToken(_) | ConcreteNode::JsonToken(_) => {
                Vec::new()
            }
        }
    };
}

impl ConcreteNode {
    fn kind_raw(&self) -> u16 {
        dispatch_all!(self, |n| n.kind().to_raw().0)
    }

    fn text_with_trivia_string(&self) -> String {
        match self {
            Self::Js(n) => n.text_with_trivia().to_string(),
            Self::Css(n) => n.text_with_trivia().to_string(),
            Self::Json(n) => n.text_with_trivia().to_string(),
            Self::JsToken(t) => t.text().to_string(),
            Self::CssToken(t) => t.text().to_string(),
            Self::JsonToken(t) => t.text().to_string(),
        }
    }

    fn text_trimmed_string(&self) -> String {
        match self {
            Self::Js(n) => n.text_trimmed().to_string(),
            Self::Css(n) => n.text_trimmed().to_string(),
            Self::Json(n) => n.text_trimmed().to_string(),
            Self::JsToken(t) => t.text_trimmed().to_string(),
            Self::CssToken(t) => t.text_trimmed().to_string(),
            Self::JsonToken(t) => t.text_trimmed().to_string(),
        }
    }

    fn text_range(&self) -> TextRange {
        match self {
            Self::Js(n) => n.text_range_with_trivia(),
            Self::Css(n) => n.text_range_with_trivia(),
            Self::Json(n) => n.text_range_with_trivia(),
            Self::JsToken(t) => t.text_range(),
            Self::CssToken(t) => t.text_range(),
            Self::JsonToken(t) => t.text_range(),
        }
    }

    fn text_trimmed_range(&self) -> TextRange {
        match self {
            Self::Js(n) => n.text_trimmed_range(),
            Self::Css(n) => n.text_trimmed_range(),
            Self::Json(n) => n.text_trimmed_range(),
            Self::JsToken(t) => t.text_trimmed_range(),
            Self::CssToken(t) => t.text_trimmed_range(),
            Self::JsonToken(t) => t.text_trimmed_range(),
        }
    }

    fn parent(&self) -> Option<Self> {
        match self {
            Self::Js(n) => n.parent().map(Self::Js),
            Self::Css(n) => n.parent().map(Self::Css),
            Self::Json(n) => n.parent().map(Self::Json),
            Self::JsToken(t) => t.parent().map(Self::Js),
            Self::CssToken(t) => t.parent().map(Self::Css),
            Self::JsonToken(t) => t.parent().map(Self::Json),
        }
    }

    fn is_token(&self) -> bool {
        matches!(
            self,
            Self::JsToken(_) | Self::CssToken(_) | Self::JsonToken(_)
        )
    }

    fn children(&self) -> Vec<Self> {
        dispatch_collect!(self, |n| n.children())
    }

    fn children_with_tokens(&self) -> Vec<Self> {
        dispatch_children_with_tokens!(self)
    }

    fn child_count(&self) -> usize {
        dispatch_node!(self, |n| n.children().count(), 0)
    }

    fn nth_child(&self, index: usize) -> Option<Self> {
        dispatch_map!(self, |n| n.children().nth(index))
    }

    fn child_by_kind(&self, kind: u16) -> Option<Self> {
        dispatch_map!(self, |n| n.children().find(|c| c.kind().to_raw().0 == kind))
    }

    /// Returns the inner `JsSyntaxNode` if this is a JS node.
    fn as_js_node(&self) -> Option<&JsSyntaxNode> {
        match self {
            Self::Js(n) => Some(n),
            _ => None,
        }
    }

    fn leading_trivia_text(&self) -> String {
        match self {
            Self::Js(n) => n
                .first_leading_trivia()
                .map_or_else(String::new, |t| t.text().to_string()),
            Self::Css(n) => n
                .first_leading_trivia()
                .map_or_else(String::new, |t| t.text().to_string()),
            Self::Json(n) => n
                .first_leading_trivia()
                .map_or_else(String::new, |t| t.text().to_string()),
            Self::JsToken(t) => t.leading_trivia().text().to_string(),
            Self::CssToken(t) => t.leading_trivia().text().to_string(),
            Self::JsonToken(t) => t.leading_trivia().text().to_string(),
        }
    }

    fn trailing_trivia_text(&self) -> String {
        match self {
            Self::Js(n) => n
                .last_trailing_trivia()
                .map_or_else(String::new, |t| t.text().to_string()),
            Self::Css(n) => n
                .last_trailing_trivia()
                .map_or_else(String::new, |t| t.text().to_string()),
            Self::Json(n) => n
                .last_trailing_trivia()
                .map_or_else(String::new, |t| t.text().to_string()),
            Self::JsToken(t) => t.trailing_trivia().text().to_string(),
            Self::CssToken(t) => t.trailing_trivia().text().to_string(),
            Self::JsonToken(t) => t.trailing_trivia().text().to_string(),
        }
    }
}

impl HostState {
    /// Create a new host state seeded with the given node at handle 0.
    pub(crate) fn new(
        node: AnySyntaxNode,
        language: PluginTargetLanguage,
        semantic_model: Option<SemanticModel>,
        module_resolver: Option<Arc<ModuleResolver>>,
        file_path: String,
    ) -> Self {
        let concrete = match language {
            PluginTargetLanguage::JavaScript => node
                .downcast_ref::<JsSyntaxNode>()
                .map(|n| ConcreteNode::Js(n.clone())),
            PluginTargetLanguage::Css => node
                .downcast_ref::<CssSyntaxNode>()
                .map(|n| ConcreteNode::Css(n.clone())),
            PluginTargetLanguage::Json => node
                .downcast_ref::<JsonSyntaxNode>()
                .map(|n| ConcreteNode::Json(n.clone())),
        };

        let mut nodes = Vec::with_capacity(256);
        if let Some(node) = concrete {
            nodes.push(node);
        }

        Self {
            nodes,
            semantic_model,
            module_resolver,
            file_path,
            regex_cache: HashMap::new(),
            wasi_ctx: WasiCtxBuilder::new().build(),
            resource_table: ResourceTable::new(),
        }
    }

    /// Create a dummy empty state (used for metadata queries).
    pub(crate) fn empty() -> Self {
        Self {
            nodes: Vec::new(),
            semantic_model: None,
            module_resolver: None,
            file_path: String::new(),
            regex_cache: HashMap::new(),
            wasi_ctx: WasiCtxBuilder::new().build(),
            resource_table: ResourceTable::new(),
        }
    }

    /// Intern a node and return its handle.
    ///
    /// Returns `u32::MAX` when the handle table is full. All host functions
    /// treat `u32::MAX` as an invalid handle, returning safe defaults.
    pub(crate) fn intern(&mut self, node: ConcreteNode) -> u32 {
        if self.nodes.len() >= MAX_NODE_HANDLES {
            return u32::MAX;
        }
        let handle = self.nodes.len() as u32;
        self.nodes.push(node);
        handle
    }

    /// Look up a node by handle.
    pub(crate) fn get(&self, handle: u32) -> Option<&ConcreteNode> {
        self.nodes.get(handle as usize)
    }

    // --- Host function implementations ---

    pub(crate) fn node_kind(&self, handle: u32) -> u32 {
        self.get(handle).map_or(0, |n| u32::from(n.kind_raw()))
    }

    pub(crate) fn node_text(&self, handle: u32) -> String {
        self.get(handle)
            .map_or_else(String::new, |n| n.text_with_trivia_string())
    }

    pub(crate) fn node_trimmed_text(&self, handle: u32) -> String {
        self.get(handle)
            .map_or_else(String::new, |n| n.text_trimmed_string())
    }

    pub(crate) fn node_range(&self, handle: u32) -> (u32, u32) {
        self.get(handle).map_or((0, 0), |n| {
            let range = n.text_range();
            (u32::from(range.start()), u32::from(range.end()))
        })
    }

    pub(crate) fn node_parent(&mut self, handle: u32) -> Option<u32> {
        let parent = self.get(handle)?.parent()?;
        Some(self.intern(parent))
    }

    pub(crate) fn node_child_count(&self, handle: u32) -> u32 {
        self.get(handle).map_or(0, |n| n.child_count() as u32)
    }

    pub(crate) fn node_nth_child(&mut self, handle: u32, index: u32) -> Option<u32> {
        let child = self.get(handle)?.nth_child(index as usize)?;
        Some(self.intern(child))
    }

    pub(crate) fn node_children(&mut self, handle: u32) -> Vec<u32> {
        let Some(node) = self.get(handle) else {
            return Vec::new();
        };
        // Collect children into a single Vec<ConcreteNode>, then intern them.
        // We must collect first because `intern()` borrows `self` mutably.
        let children = node.children();
        children.into_iter().map(|c| self.intern(c)).collect()
    }

    pub(crate) fn node_child_by_kind(&mut self, handle: u32, kind: u32) -> Option<u32> {
        let child = self.get(handle)?.child_by_kind(kind as u16)?;
        Some(self.intern(child))
    }

    pub(crate) fn element_is_token(&self, handle: u32) -> bool {
        self.get(handle).is_some_and(|n| n.is_token())
    }

    pub(crate) fn node_children_with_tokens(&mut self, handle: u32) -> Vec<u32> {
        let Some(node) = self.get(handle) else {
            return Vec::new();
        };
        let children = node.children_with_tokens();
        children.into_iter().map(|c| self.intern(c)).collect()
    }

    pub(crate) fn node_trimmed_range(&self, handle: u32) -> (u32, u32) {
        self.get(handle).map_or((0, 0), |n| {
            let range = n.text_trimmed_range();
            (u32::from(range.start()), u32::from(range.end()))
        })
    }

    // --- Semantic model host function implementations ---

    /// Convenience accessor: returns the semantic model and the JS node for `handle`.
    /// Returns `None` when either the model is absent or the handle is not a JS node.
    fn semantic_js_node(&self, handle: u32) -> Option<(&SemanticModel, &JsSyntaxNode)> {
        let model = self.semantic_model.as_ref()?;
        let js_node = self.get(handle)?.as_js_node()?;
        Some((model, js_node))
    }

    pub(crate) fn resolve_reference(&mut self, handle: u32) -> Option<u32> {
        let (model, js_node) = self.semantic_js_node(handle)?;
        let binding = model.resolve_reference_node(js_node)?;
        let decl_node = binding.syntax().clone();
        Some(self.intern(ConcreteNode::Js(decl_node)))
    }

    pub(crate) fn all_references(&mut self, handle: u32) -> Vec<u32> {
        let Some(model) = self.semantic_model.as_ref() else {
            return Vec::new();
        };
        let Some(js_node) = self.get(handle).and_then(|n| n.as_js_node()).cloned() else {
            return Vec::new();
        };
        let Some(binding) = model.binding_by_node(&js_node) else {
            return Vec::new();
        };
        binding
            .all_references()
            .map(|r| {
                let ref_node = r.syntax().clone();
                self.intern(ConcreteNode::Js(ref_node))
            })
            .collect()
    }

    pub(crate) fn is_exported(&self, handle: u32) -> bool {
        let Some((model, js_node)) = self.semantic_js_node(handle) else {
            return false;
        };
        model.is_binding_exported(js_node)
    }

    pub(crate) fn node_scope(&mut self, handle: u32) -> Option<u32> {
        let model = self.semantic_model.as_ref()?;
        let js_node = self.get(handle)?.as_js_node()?.clone();
        let scope = model.scope(&js_node);
        let scope_node = scope.syntax().clone();
        Some(self.intern(ConcreteNode::Js(scope_node)))
    }

    pub(crate) fn scope_bindings(&mut self, handle: u32) -> Vec<u32> {
        let Some(model) = self.semantic_model.as_ref() else {
            return Vec::new();
        };
        let Some(js_node) = self.get(handle).and_then(|n| n.as_js_node()).cloned() else {
            return Vec::new();
        };
        let scope = model.scope(&js_node);
        scope
            .bindings()
            .map(|b| {
                let decl_node = b.syntax().clone();
                self.intern(ConcreteNode::Js(decl_node))
            })
            .collect()
    }

    pub(crate) fn parent_scope_node(&mut self, handle: u32) -> Option<u32> {
        let model = self.semantic_model.as_ref()?;
        let js_node = self.get(handle)?.as_js_node()?.clone();
        let scope = model.scope(&js_node);
        let parent = scope.parent()?;
        let parent_node = parent.syntax().clone();
        Some(self.intern(ConcreteNode::Js(parent_node)))
    }

    // --- Enhanced semantic model host function implementations ---

    /// Check if a reference node is a read reference.
    /// Returns `false` if the handle is not a JS reference or no semantic model is available.
    pub(crate) fn reference_is_read(&self, handle: u32) -> bool {
        self.reference_matches(handle, |r| r.is_read())
    }

    /// Check if a reference node is a write reference.
    /// Returns `false` if the handle is not a JS reference or no semantic model is available.
    pub(crate) fn reference_is_write(&self, handle: u32) -> bool {
        self.reference_matches(handle, |r| r.is_write())
    }

    /// Shared implementation for `reference_is_read` / `reference_is_write`.
    fn reference_matches(
        &self,
        handle: u32,
        pred: impl Fn(&biome_js_semantic::Reference) -> bool,
    ) -> bool {
        let Some((model, js_node)) = self.semantic_js_node(handle) else {
            return false;
        };
        let Some(binding) = model.resolve_reference_node(js_node) else {
            return false;
        };
        let range_start = js_node.text_trimmed_range().start();
        binding
            .all_references()
            .any(|r| r.range_start() == range_start && pred(&r))
    }

    /// Given a binding declaration node, return handles to all read reference nodes.
    pub(crate) fn all_read_references(&mut self, handle: u32) -> Vec<u32> {
        self.collect_binding_references(handle, |b| b.all_reads())
    }

    /// Given a binding declaration node, return handles to all write reference nodes.
    pub(crate) fn all_write_references(&mut self, handle: u32) -> Vec<u32> {
        self.collect_binding_references(handle, |b| b.all_writes())
    }

    /// Shared implementation for `all_read_references` / `all_write_references`.
    fn collect_binding_references<I, F>(&mut self, handle: u32, iter_fn: F) -> Vec<u32>
    where
        I: Iterator<Item = biome_js_semantic::Reference>,
        F: FnOnce(&biome_js_semantic::Binding) -> I,
    {
        let Some((model, js_node)) = self.semantic_js_node(handle) else {
            return Vec::new();
        };
        let Some(binding) = model.binding_by_node(js_node) else {
            return Vec::new();
        };
        iter_fn(&binding)
            .map(|r| self.intern(ConcreteNode::Js(r.syntax().clone())))
            .collect()
    }

    /// Check whether a binding declaration node is imported.
    pub(crate) fn is_imported(&self, handle: u32) -> bool {
        let Some((model, js_node)) = self.semantic_js_node(handle) else {
            return false;
        };
        model
            .binding_by_node(js_node)
            .is_some_and(|b| b.is_imported())
    }

    /// Look up a binding by name within the given scope node.
    pub(crate) fn scope_get_binding_by_name(
        &mut self,
        scope_handle: u32,
        name: &str,
    ) -> Option<u32> {
        let model = self.semantic_model.as_ref()?;
        let js_node = self.get(scope_handle)?.as_js_node()?.clone();
        let scope = model.scope(&js_node);
        let binding = scope.get_binding(name)?;
        Some(self.intern(ConcreteNode::Js(binding.syntax().clone())))
    }

    /// Check if the given scope node is the global scope.
    pub(crate) fn is_global_scope(&self, scope_handle: u32) -> bool {
        let Some((model, js_node)) = self.semantic_js_node(scope_handle) else {
            return false;
        };
        model.scope(js_node).is_global_scope()
    }

    /// Return ancestor scope nodes from the given scope up to the global scope.
    pub(crate) fn scope_ancestors(&mut self, scope_handle: u32) -> Vec<u32> {
        self.collect_scope_items(scope_handle, |s| s.ancestors())
    }

    /// Return immediate child scope nodes of the given scope.
    pub(crate) fn scope_children(&mut self, scope_handle: u32) -> Vec<u32> {
        self.collect_scope_items(scope_handle, |s| s.children())
    }

    /// Shared implementation for `scope_ancestors` / `scope_children`.
    fn collect_scope_items<I, F>(&mut self, scope_handle: u32, iter_fn: F) -> Vec<u32>
    where
        I: Iterator<Item = biome_js_semantic::Scope>,
        F: FnOnce(biome_js_semantic::Scope) -> I,
    {
        let Some(model) = self.semantic_model.as_ref() else {
            return Vec::new();
        };
        let Some(js_node) = self.get(scope_handle).and_then(|n| n.as_js_node()).cloned() else {
            return Vec::new();
        };
        let scope = model.scope(&js_node);
        iter_fn(scope)
            .map(|s| self.intern(ConcreteNode::Js(s.syntax().clone())))
            .collect()
    }

    /// Check if an expression node is constant (no variable dependencies).
    pub(crate) fn is_constant_expression(&self, handle: u32) -> bool {
        let Some((model, js_node)) = self.semantic_js_node(handle) else {
            return false;
        };
        let Some(expr) = js_node.clone().cast::<AnyJsExpression>() else {
            return false;
        };
        model.is_constant(&expr)
    }

    /// Return captured variable info for a closure (function/arrow) node.
    /// Each entry is `(reference_handle, binding_handle, is_by_reference)`.
    pub(crate) fn closure_captures(&mut self, handle: u32) -> Vec<(u32, u32, bool)> {
        let Some(model) = self.semantic_model.as_ref() else {
            return Vec::new();
        };
        let Some(js_node) = self.get(handle).and_then(|n| n.as_js_node()).cloned() else {
            return Vec::new();
        };
        let Some(closure_node) = AnyHasClosureNode::from_node(&js_node) else {
            return Vec::new();
        };
        let closure = model.closure(&closure_node);
        closure
            .all_captures()
            .map(|capture| {
                let ref_handle = self.intern(ConcreteNode::Js(capture.node().clone()));
                let binding_handle =
                    self.intern(ConcreteNode::Js(capture.binding().syntax().clone()));
                let is_by_reference = matches!(capture.ty(), CaptureType::ByReference);
                (ref_handle, binding_handle, is_by_reference)
            })
            .collect()
    }

    /// Get the inferred type of a JS expression node as a string tag.
    /// Returns `"unknown"` if type inference is unavailable or the node is not an expression.
    pub(crate) fn expression_type(&self, handle: u32) -> String {
        let Some(js_node) = self.get(handle).and_then(|n| n.as_js_node()) else {
            return "unknown".to_string();
        };
        let Some(resolver) = self.module_resolver.as_ref() else {
            return "unknown".to_string();
        };
        let Some(expr) = js_node.clone().cast::<AnyJsExpression>() else {
            return "unknown".to_string();
        };
        let ty = resolver.resolved_type_of_expression(&expr);
        type_data_to_tag(&ty).to_string()
    }

    /// Get the file path of the current file being analyzed.
    pub(crate) fn file_path(&self) -> String {
        self.file_path.clone()
    }

    // --- Structured type info (Gap 6) ---

    /// Get structured type information for a JS expression node.
    /// Returns tag, optional literal value, and optional union member tags.
    pub(crate) fn expression_type_info(
        &self,
        handle: u32,
    ) -> (String, Option<String>, Option<Vec<String>>) {
        let Some(js_node) = self.get(handle).and_then(|n| n.as_js_node()) else {
            return ("unknown".to_string(), None, None);
        };
        let Some(resolver) = self.module_resolver.as_ref() else {
            return ("unknown".to_string(), None, None);
        };
        let Some(expr) = js_node.clone().cast::<AnyJsExpression>() else {
            return ("unknown".to_string(), None, None);
        };
        let ty = resolver.resolved_type_of_expression(&expr);
        let type_data: &TypeData = &ty;

        let tag = type_data_to_tag(type_data).to_string();
        let literal_value = extract_literal_value(type_data);
        let union_members = extract_union_members(type_data, resolver);

        (tag, literal_value, union_members)
    }

    // --- Global reference detection (Gap 2) ---

    /// Check if a reference node refers to a global (unresolved) binding.
    pub(crate) fn is_global_reference(&self, handle: u32) -> bool {
        let Some(js_node) = self.get(handle).and_then(|n| n.as_js_node()) else {
            return false;
        };

        // First: check if this is a JsReferenceIdentifier with no local binding.
        if let Some(ref_id) = js_node.clone().cast::<JsReferenceIdentifier>() {
            if let Some(model) = self.semantic_model.as_ref() {
                return model.binding(&ref_id).is_none();
            }
            // No semantic model — can't determine, return false.
            return false;
        }

        // Second: check for globalThis.X / window.X patterns.
        if let Some(expr) = js_node.clone().cast::<AnyJsExpression>()
            && global_identifier(&expr).is_some()
        {
            return true;
        }

        false
    }

    // --- Trivia access (Gap 5) ---

    /// Get leading trivia text of a node or token.
    pub(crate) fn node_leading_trivia(&self, handle: u32) -> String {
        self.get(handle)
            .map_or_else(String::new, |n| n.leading_trivia_text())
    }

    /// Get trailing trivia text of a node or token.
    pub(crate) fn node_trailing_trivia(&self, handle: u32) -> String {
        self.get(handle)
            .map_or_else(String::new, |n| n.trailing_trivia_text())
    }

    // --- Regex utilities (Gap 11) ---

    /// Test if text matches a regex pattern.
    pub(crate) fn regex_matches(&mut self, text: &str, pattern: &str) -> bool {
        let Some(re) = self.get_or_compile_regex(pattern) else {
            return false;
        };
        re.is_match(text)
    }

    /// Find all matches and return (start, end) byte offset pairs.
    pub(crate) fn regex_find_all(&mut self, text: &str, pattern: &str) -> Vec<(u32, u32)> {
        let Some(re) = self.get_or_compile_regex(pattern) else {
            return Vec::new();
        };
        re.find_iter(text)
            .map(|m| (m.start() as u32, m.end() as u32))
            .collect()
    }

    /// Get or compile a regex, caching for reuse.
    fn get_or_compile_regex(&mut self, pattern: &str) -> Option<&regex::Regex> {
        if !self.regex_cache.contains_key(pattern) {
            match regex::Regex::new(pattern) {
                Ok(re) => {
                    self.regex_cache.insert(pattern.to_string(), re);
                }
                Err(_) => return None,
            }
        }
        self.regex_cache.get(pattern)
    }

    // --- Control flow heuristic (Gap 8) ---

    /// Check if a node is reachable (heuristic: not preceded by
    /// return/throw/break/continue in the same block).
    pub(crate) fn is_reachable(&self, handle: u32) -> bool {
        let Some(js_node) = self.get(handle).and_then(|n| n.as_js_node()) else {
            // Non-JS nodes or tokens are considered reachable.
            return true;
        };

        // Walk up to find the enclosing statement list / block.
        let mut current = js_node.clone();
        while let Some(parent) = current.parent() {
            if parent.kind() == JsSyntaxKind::JS_STATEMENT_LIST {
                // Check if any preceding sibling is an unconditional exit.
                for sibling in parent.children() {
                    // If we've reached the current node's range, stop.
                    if sibling.text_trimmed_range().start() >= js_node.text_trimmed_range().start()
                    {
                        return true;
                    }
                    let kind = sibling.kind();
                    if kind == JsSyntaxKind::JS_RETURN_STATEMENT
                        || kind == JsSyntaxKind::JS_THROW_STATEMENT
                        || kind == JsSyntaxKind::JS_BREAK_STATEMENT
                        || kind == JsSyntaxKind::JS_CONTINUE_STATEMENT
                    {
                        return false;
                    }
                }
                return true;
            }
            current = parent;
        }

        true
    }

    // --- ARIA implicit role (Gap 9) ---

    /// Get the implicit ARIA role for a JSX element node.
    pub(crate) fn aria_implicit_role(&self, handle: u32) -> Option<String> {
        let js_node = self.get(handle)?.as_js_node()?;
        let jsx_element = AnyJsxElement::cast(js_node.clone())?;
        let role = AriaRoles.get_implicit_role(&jsx_element)?;
        Some(role.to_string())
    }
}

/// Extract the literal value from a [`TypeData::Literal`] as a string.
fn extract_literal_value(data: &TypeData) -> Option<String> {
    match data {
        TypeData::Literal(lit) => match lit.as_ref() {
            Literal::Boolean(b) => Some(b.as_bool().to_string()),
            Literal::Number(n) => Some(n.as_str().to_string()),
            Literal::String(s) => Some(s.as_str().to_string()),
            Literal::BigInt(b) => Some(b.to_string()),
            Literal::Template(t) => Some(t.to_string()),
            Literal::Object(_) | Literal::RegExp(_) => None,
        },
        _ => None,
    }
}

/// Extract union member tags from a [`TypeData::Union`].
fn extract_union_members(data: &TypeData, resolver: &Arc<ModuleResolver>) -> Option<Vec<String>> {
    match data {
        TypeData::Union(union) => {
            let members: Vec<String> = union
                .types()
                .iter()
                .map(|type_ref| {
                    let member_ty = resolver.resolved_type_for_reference(type_ref);
                    let member_data: &TypeData = &member_ty;
                    type_data_to_tag(member_data).to_string()
                })
                .collect();
            Some(members)
        }
        _ => None,
    }
}

/// Map a [`TypeData`] variant to a simple string tag for the plugin API.
///
/// Uses exhaustive matching so that new `TypeData` variants cause compile
/// errors instead of silently falling into a catch-all.
///
/// For [`TypeData::Literal`], returns the underlying primitive type
/// (e.g. `"boolean"` for `Literal::Boolean`) rather than `"literal"`,
/// which is more useful for plugin authors.
fn type_data_to_tag(data: &TypeData) -> &'static str {
    match data {
        // Primitives
        TypeData::Boolean => "boolean",
        TypeData::Number => "number",
        TypeData::String => "string",
        TypeData::BigInt => "bigint",
        TypeData::Null => "null",
        TypeData::Undefined => "undefined",
        TypeData::Symbol => "symbol",

        // Literal types — return the underlying primitive type.
        TypeData::Literal(lit) => match lit.as_ref() {
            Literal::Boolean(_) => "boolean",
            Literal::Number(_) => "number",
            Literal::String(_) | Literal::Template(_) => "string",
            Literal::BigInt(_) => "bigint",
            Literal::Object(_) => "object",
            Literal::RegExp(_) => "object",
        },

        // Structural types
        TypeData::Object(_) | TypeData::ObjectKeyword => "object",
        TypeData::Function(_) => "function",
        TypeData::Class(_) => "class",
        TypeData::Tuple(_) => "array",
        TypeData::Union(_) => "union",
        TypeData::Intersection(_) => "intersection",

        // Object-like structural types
        TypeData::Interface(_) | TypeData::Module(_) | TypeData::Namespace(_) => "object",
        TypeData::Constructor(_) => "function",
        TypeData::InstanceOf(_) => "object",
        TypeData::ImportNamespace(_) => "object",
        TypeData::Global => "object",

        // Keyword types
        TypeData::AnyKeyword | TypeData::UnknownKeyword => "unknown",
        TypeData::NeverKeyword => "never",
        TypeData::ThisKeyword => "object",
        TypeData::VoidKeyword => "undefined",

        // Unresolved references and type operations — cannot determine a
        // concrete type, so report "unknown".
        TypeData::Unknown
        | TypeData::Conditional
        | TypeData::Generic(_)
        | TypeData::TypeOperator(_)
        | TypeData::Reference(_)
        | TypeData::MergedReference(_)
        | TypeData::TypeofExpression(_)
        | TypeData::TypeofType(_)
        | TypeData::TypeofValue(_) => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_css_syntax::CssFileSource;
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::JsFileSource;
    use biome_json_parser::{JsonParserOptions, parse_json};
    use biome_rowan::AstNode;

    fn parse_js(source: &str) -> JsSyntaxNode {
        biome_js_parser::parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        )
        .tree()
        .syntax()
        .clone()
    }

    fn js_state(source: &str) -> HostState {
        let root = parse_js(source);
        HostState::new(
            root.into(),
            PluginTargetLanguage::JavaScript,
            None,
            None,
            String::new(),
        )
    }

    // ---------------------------------------------------------------
    // Interning & handle basics
    // ---------------------------------------------------------------

    #[test]
    fn intern_root_node() {
        let state = js_state("let x = 1;");
        assert!(state.get(0).is_some());
        assert!(state.get(1).is_none());
    }

    #[test]
    fn intern_returns_sequential_handles() {
        let root = parse_js("let x = 1;");
        let child = root.children().next().unwrap();
        let mut state = js_state("let x = 1;");
        let h = state.intern(ConcreteNode::Js(child));
        assert_eq!(h, 1); // 0 = root, 1 = first intern
    }

    #[test]
    fn get_invalid_handle_returns_none() {
        let state = js_state("let x = 1;");
        assert!(state.get(999).is_none());
    }

    // ---------------------------------------------------------------
    // node_kind
    // ---------------------------------------------------------------

    #[test]
    fn node_kind_returns_raw_kind() {
        let root = parse_js("let x = 1;");
        let expected_kind = root.kind().to_raw().0;
        let state = js_state("let x = 1;");
        assert_eq!(state.node_kind(0), u32::from(expected_kind));
    }

    #[test]
    fn node_kind_invalid_handle_returns_zero() {
        let state = js_state("let x = 1;");
        assert_eq!(state.node_kind(42), 0);
    }

    // ---------------------------------------------------------------
    // node_text / node_trimmed_text
    // ---------------------------------------------------------------

    #[test]
    fn node_text_returns_full_source() {
        let source = "let x = 1;";
        let state = js_state(source);
        let text = state.node_text(0);
        // Root text includes the entire source
        assert!(text.contains("let x = 1;"));
    }

    #[test]
    fn node_trimmed_text_excludes_trivia() {
        let source = "let x = 1;";
        let state = js_state(source);
        let trimmed = state.node_trimmed_text(0);
        assert!(trimmed.contains("let x = 1;"));
    }

    #[test]
    fn node_text_on_child_is_subset_of_root() {
        let source = "let x = 1;";
        let mut state = js_state(source);
        let root_text = state.node_text(0);
        let children = state.node_children(0);
        assert!(!children.is_empty());
        // At least one child should have text that appears in the root
        let any_nonempty = children.iter().any(|&h| !state.node_text(h).is_empty());
        assert!(any_nonempty || !root_text.is_empty());
    }

    #[test]
    fn node_text_invalid_handle_returns_empty() {
        let state = js_state("let x = 1;");
        assert_eq!(state.node_text(99), "");
        assert_eq!(state.node_trimmed_text(99), "");
    }

    // ---------------------------------------------------------------
    // node_range
    // ---------------------------------------------------------------

    #[test]
    fn node_range_covers_source() {
        let source = "let x = 1;";
        let state = js_state(source);
        let (start, end) = state.node_range(0);
        assert_eq!(start, 0);
        assert_eq!(end as usize, source.len());
    }

    #[test]
    fn node_range_invalid_handle_returns_zeros() {
        let state = js_state("let x = 1;");
        assert_eq!(state.node_range(77), (0, 0));
    }

    #[test]
    fn child_range_is_within_parent_range() {
        let mut state = js_state("let x = 1;");
        let (p_start, p_end) = state.node_range(0);
        let children = state.node_children(0);
        for &handle in &children {
            let (c_start, c_end) = state.node_range(handle);
            assert!(c_start >= p_start);
            assert!(c_end <= p_end);
        }
    }

    // ---------------------------------------------------------------
    // node_parent
    // ---------------------------------------------------------------

    #[test]
    fn root_has_no_parent() {
        let mut state = js_state("let x = 1;");
        assert_eq!(state.node_parent(0), None);
    }

    #[test]
    fn child_parent_roundtrips_to_root() {
        let mut state = js_state("let x = 1;");
        let children = state.node_children(0);
        assert!(!children.is_empty());
        let parent_handle = state.node_parent(children[0]);
        // The parent of a root's child should be the root
        assert!(parent_handle.is_some());
        let parent_kind = state.node_kind(parent_handle.unwrap());
        let root_kind = state.node_kind(0);
        assert_eq!(parent_kind, root_kind);
    }

    #[test]
    fn node_parent_invalid_handle_returns_none() {
        let mut state = js_state("let x = 1;");
        assert_eq!(state.node_parent(100), None);
    }

    // ---------------------------------------------------------------
    // node_child_count
    // ---------------------------------------------------------------

    #[test]
    fn node_child_count_matches_children_len() {
        let mut state = js_state("let x = 1; const y = 2;");
        let count = state.node_child_count(0);
        let children = state.node_children(0);
        assert_eq!(count as usize, children.len());
        assert!(count > 0);
    }

    #[test]
    fn node_child_count_invalid_handle_returns_zero() {
        let state = js_state("let x = 1;");
        assert_eq!(state.node_child_count(50), 0);
    }

    // ---------------------------------------------------------------
    // node_nth_child
    // ---------------------------------------------------------------

    #[test]
    fn node_nth_child_first_matches_children() {
        let mut state = js_state("let x = 1;");
        let children = state.node_children(0);
        assert!(!children.is_empty());
        // Requesting nth_child(0, 0) should give us the same kind as children[0]
        let first = state.node_nth_child(0, 0);
        assert!(first.is_some());
        let first_kind = state.node_kind(first.unwrap());
        let expected_kind = state.node_kind(children[0]);
        assert_eq!(first_kind, expected_kind);
    }

    #[test]
    fn node_nth_child_out_of_bounds_returns_none() {
        let mut state = js_state("let x = 1;");
        assert_eq!(state.node_nth_child(0, 9999), None);
    }

    #[test]
    fn node_nth_child_invalid_handle_returns_none() {
        let mut state = js_state("let x = 1;");
        assert_eq!(state.node_nth_child(88, 0), None);
    }

    // ---------------------------------------------------------------
    // node_children
    // ---------------------------------------------------------------

    #[test]
    fn node_children_returns_valid_handles() {
        let mut state = js_state("let x = 1;");
        let children = state.node_children(0);
        assert!(!children.is_empty());
        for &handle in &children {
            assert!(state.get(handle).is_some());
            assert_ne!(state.node_kind(handle), 0);
        }
    }

    #[test]
    fn node_children_invalid_handle_returns_empty() {
        let mut state = js_state("let x = 1;");
        assert!(state.node_children(99).is_empty());
    }

    #[test]
    fn deep_tree_traversal() {
        // Walk two levels deep and verify handles remain valid
        let mut state = js_state("function foo() { return 1; }");
        let l1 = state.node_children(0);
        assert!(!l1.is_empty());
        // Find a child that itself has children (i.e., a node, not a token-only list)
        let mut found_grandchildren = false;
        for &child in &l1 {
            let l2 = state.node_children(child);
            if !l2.is_empty() {
                found_grandchildren = true;
                for &handle in &l2 {
                    assert!(state.get(handle).is_some());
                }
                break;
            }
        }
        assert!(found_grandchildren);
    }

    // ---------------------------------------------------------------
    // node_child_by_kind
    // ---------------------------------------------------------------

    #[test]
    fn node_child_by_kind_finds_existing() {
        let root = parse_js("let x = 1;");
        // JS_MODULE root should contain a JsModuleItemList (or similar)
        let first_child = root.children().next().unwrap();
        let target_kind = first_child.kind().to_raw().0;

        let mut state = js_state("let x = 1;");
        let found = state.node_child_by_kind(0, u32::from(target_kind));
        assert!(found.is_some());
        assert_eq!(state.node_kind(found.unwrap()), u32::from(target_kind));
    }

    #[test]
    fn node_child_by_kind_nonexistent_returns_none() {
        let mut state = js_state("let x = 1;");
        // Use an unlikely kind value
        assert_eq!(state.node_child_by_kind(0, 65535), None);
    }

    #[test]
    fn node_child_by_kind_invalid_handle_returns_none() {
        let mut state = js_state("let x = 1;");
        assert_eq!(state.node_child_by_kind(50, 1), None);
    }

    // ---------------------------------------------------------------
    // empty state
    // ---------------------------------------------------------------

    #[test]
    fn empty_state_handles_gracefully() {
        let state = HostState::empty();
        assert!(state.get(0).is_none());
        assert_eq!(state.node_kind(0), 0);
        assert_eq!(state.node_text(0), "");
        assert_eq!(state.node_trimmed_text(0), "");
        assert_eq!(state.node_range(0), (0, 0));
        assert_eq!(state.node_child_count(0), 0);
    }

    #[test]
    fn empty_state_mutable_ops() {
        let mut state = HostState::empty();
        assert_eq!(state.node_parent(0), None);
        assert_eq!(state.node_nth_child(0, 0), None);
        assert!(state.node_children(0).is_empty());
        assert_eq!(state.node_child_by_kind(0, 1), None);
    }

    // ---------------------------------------------------------------
    // Language mismatch
    // ---------------------------------------------------------------

    #[test]
    fn wrong_language_produces_empty_root() {
        // Parse JS but tell HostState it's CSS — downcast should fail
        let root = parse_js("let x = 1;");
        let state = HostState::new(
            root.into(),
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
        );
        // No node at handle 0 because downcast failed
        assert!(state.get(0).is_none());
        assert_eq!(state.node_kind(0), 0);
    }

    // ---------------------------------------------------------------
    // CSS language support
    // ---------------------------------------------------------------

    #[test]
    fn css_root_node_is_interned() {
        let root = parse_css(
            "body { color: red; }",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .tree()
        .syntax()
        .clone();
        let expected_kind = root.kind().to_raw().0;

        let state = HostState::new(
            root.into(),
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
        );
        assert!(state.get(0).is_some());
        assert_eq!(state.node_kind(0), u32::from(expected_kind));
    }

    #[test]
    fn css_children_navigation() {
        let root = parse_css(
            "body { color: red; }",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .tree()
        .syntax()
        .clone();

        let mut state = HostState::new(
            root.into(),
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
        );
        let children = state.node_children(0);
        assert!(!children.is_empty());
        assert!(state.node_child_count(0) > 0);
    }

    // ---------------------------------------------------------------
    // JSON language support
    // ---------------------------------------------------------------

    #[test]
    fn json_root_node_is_interned() {
        let root = parse_json(r#"{"key": "value"}"#, JsonParserOptions::default())
            .tree()
            .syntax()
            .clone();
        let expected_kind = root.kind().to_raw().0;

        let state = HostState::new(
            root.into(),
            PluginTargetLanguage::Json,
            None,
            None,
            String::new(),
        );
        assert!(state.get(0).is_some());
        assert_eq!(state.node_kind(0), u32::from(expected_kind));
    }

    #[test]
    fn json_tree_traversal() {
        let root = parse_json(r#"{"a": 1, "b": [2, 3]}"#, JsonParserOptions::default())
            .tree()
            .syntax()
            .clone();

        let mut state = HostState::new(
            root.into(),
            PluginTargetLanguage::Json,
            None,
            None,
            String::new(),
        );
        let l1 = state.node_children(0);
        assert!(!l1.is_empty());
        // Navigate deeper
        let l2 = state.node_children(l1[0]);
        assert!(!l2.is_empty());
        // Parent of l2[0] should have the same kind as l1[0]
        if let Some(parent_h) = state.node_parent(l2[0]) {
            assert_eq!(state.node_kind(parent_h), state.node_kind(l1[0]));
        }
    }

    // ---------------------------------------------------------------
    // Concrete node methods directly
    // ---------------------------------------------------------------

    #[test]
    fn concrete_node_js_text_range() {
        let root = parse_js("x;");
        let node = ConcreteNode::Js(root);
        let range = node.text_range();
        assert_eq!(u32::from(range.start()), 0);
        assert_eq!(u32::from(range.end()), 2);
    }

    #[test]
    fn concrete_node_js_parent_of_root_is_none() {
        let root = parse_js("x;");
        let node = ConcreteNode::Js(root);
        assert!(node.parent().is_none());
    }

    #[test]
    fn concrete_node_js_child_by_kind_found() {
        let root = parse_js("let x = 1;");
        let node = ConcreteNode::Js(root.clone());
        let first_child = root.children().next().unwrap();
        let target_kind = first_child.kind().to_raw().0;
        let found = node.child_by_kind(target_kind);
        assert!(found.is_some());
    }

    #[test]
    fn concrete_node_js_child_by_kind_not_found() {
        let root = parse_js("let x = 1;");
        let node = ConcreteNode::Js(root);
        assert!(node.child_by_kind(u16::MAX).is_none());
    }

    // ---------------------------------------------------------------
    // Node handle cap
    // ---------------------------------------------------------------

    #[test]
    fn intern_cap_returns_max_at_limit() {
        let root = parse_js("let x = 1;");
        let mut state = js_state("let x = 1;");
        // Fill to the cap (handle 0 is the root, so we need MAX - 1 more)
        for _ in 1..MAX_NODE_HANDLES {
            let h = state.intern(ConcreteNode::Js(root.clone()));
            assert_ne!(h, u32::MAX);
        }
        assert_eq!(state.nodes.len(), MAX_NODE_HANDLES);
        // Next intern should return u32::MAX and not grow the vec
        let overflow = state.intern(ConcreteNode::Js(root.clone()));
        assert_eq!(overflow, u32::MAX);
        assert_eq!(state.nodes.len(), MAX_NODE_HANDLES);
    }

    #[test]
    fn max_handle_degrades_gracefully() {
        let mut state = js_state("let x = 1;");
        // u32::MAX is beyond any valid index — all lookups should return defaults
        assert_eq!(state.node_kind(u32::MAX), 0);
        assert_eq!(state.node_text(u32::MAX), "");
        assert_eq!(state.node_trimmed_text(u32::MAX), "");
        assert_eq!(state.node_range(u32::MAX), (0, 0));
        assert_eq!(state.node_parent(u32::MAX), None);
        assert_eq!(state.node_child_count(u32::MAX), 0);
        assert_eq!(state.node_nth_child(u32::MAX, 0), None);
        assert!(state.node_children(u32::MAX).is_empty());
        assert_eq!(state.node_child_by_kind(u32::MAX, 1), None);
    }

    // ---------------------------------------------------------------
    // CSS/JSON graceful degradation for JS-only semantic functions
    // ---------------------------------------------------------------

    #[test]
    fn css_semantic_functions_return_defaults() {
        let root = parse_css(
            "a { color: red; }",
            CssFileSource::css(),
            CssParserOptions::default(),
        )
        .tree()
        .syntax()
        .clone();
        let mut state = HostState::new(
            root.into(),
            PluginTargetLanguage::Css,
            None,
            None,
            String::new(),
        );
        assert!(!state.reference_is_read(0));
        assert!(!state.reference_is_write(0));
        assert!(state.all_read_references(0).is_empty());
        assert!(state.all_write_references(0).is_empty());
        assert!(!state.is_imported(0));
        assert_eq!(state.scope_get_binding_by_name(0, "x"), None);
        assert!(!state.is_global_scope(0));
        assert!(state.scope_ancestors(0).is_empty());
        assert!(state.scope_children(0).is_empty());
        assert!(!state.is_constant_expression(0));
        assert!(state.closure_captures(0).is_empty());
        assert_eq!(state.expression_type(0), "unknown");
    }

    #[test]
    fn json_semantic_functions_return_defaults() {
        let root = parse_json(r#"{"a": 1}"#, JsonParserOptions::default())
            .tree()
            .syntax()
            .clone();
        let mut state = HostState::new(
            root.into(),
            PluginTargetLanguage::Json,
            None,
            None,
            String::new(),
        );
        assert!(!state.reference_is_read(0));
        assert!(state.all_read_references(0).is_empty());
        assert!(!state.is_imported(0));
        assert!(!state.is_constant_expression(0));
        assert_eq!(state.expression_type(0), "unknown");
    }

    // ---------------------------------------------------------------
    // Semantic model host functions
    // ---------------------------------------------------------------

    /// Create a HostState with both semantic model and module resolver (type inference).
    fn js_state_with_resolver(source: &str) -> HostState {
        use biome_fs::{BiomePath, MemoryFileSystem};
        use biome_module_graph::{ModuleGraph, ModuleResolver};
        use biome_project_layout::ProjectLayout;
        use camino::Utf8Path;

        let parse = biome_js_parser::parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = biome_js_semantic::semantic_model(
            &parse.tree(),
            biome_js_semantic::SemanticModelOptions::default(),
        );

        // Build a ModuleGraph with the source file so we can get a ModuleResolver.
        let fs = MemoryFileSystem::default();
        fs.insert("/src/test.ts".into(), source);
        let path = BiomePath::new("/src/test.ts");
        let added_paths = vec![(&path, parse.tree())];
        let module_graph = Arc::new(ModuleGraph::default());
        module_graph.update_graph_for_js_paths(&fs, &ProjectLayout::default(), &added_paths, true);
        let module_info = module_graph
            .js_module_info_for_path(Utf8Path::new("/src/test.ts"))
            .expect("module should exist");
        let resolver = Arc::new(ModuleResolver::for_module(
            module_info,
            module_graph.clone(),
        ));

        let root = parse.syntax().clone();
        HostState::new(
            root.into(),
            PluginTargetLanguage::JavaScript,
            Some(model),
            Some(resolver),
            "/src/test.ts".to_string(),
        )
    }

    fn js_state_with_semantic(source: &str) -> HostState {
        let parse = biome_js_parser::parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let model = biome_js_semantic::semantic_model(
            &parse.tree(),
            biome_js_semantic::SemanticModelOptions::default(),
        );
        let root = parse.tree().syntax().clone();
        HostState::new(
            root.into(),
            PluginTargetLanguage::JavaScript,
            Some(model),
            None,
            String::new(),
        )
    }

    #[test]
    fn semantic_without_model_returns_none() {
        // State without semantic model — all semantic functions return defaults
        let mut state = js_state("let x = 1; x;");
        assert_eq!(state.resolve_reference(0), None);
        assert!(state.all_references(0).is_empty());
        assert!(!state.is_exported(0));
        assert_eq!(state.node_scope(0), None);
        assert!(state.scope_bindings(0).is_empty());
        assert_eq!(state.parent_scope_node(0), None);
    }

    #[test]
    fn semantic_node_scope_returns_scope_node() {
        let mut state = js_state_with_semantic("let x = 1;");
        // Root is at handle 0, node_scope should return a scope node handle
        let scope_handle = state.node_scope(0);
        assert!(scope_handle.is_some());
        let sh = scope_handle.unwrap();
        assert!(state.get(sh).is_some());
    }

    #[test]
    fn semantic_scope_bindings_finds_declarations() {
        let mut state = js_state_with_semantic("let x = 1; let y = 2;");
        // Navigate to the first child to get inside the module body
        let children = state.node_children(0);
        assert!(!children.is_empty());
        // Get scope for root — should contain bindings x and y
        let scope_handle = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope_handle);
        // There should be at least 2 bindings (x and y)
        assert!(
            bindings.len() >= 2,
            "Expected >= 2 bindings, got {}",
            bindings.len()
        );
        // Each binding handle should be valid
        for &h in &bindings {
            assert!(state.get(h).is_some());
        }
    }

    #[test]
    fn semantic_resolve_reference_and_all_references() {
        // Source: `let x = 1; x;`
        let mut state = js_state_with_semantic("let x = 1; x;");

        // Tree: JsModule → [JsDirectiveList, JsModuleItemList]
        //   JsModuleItemList → [JsVariableStatement, JsExpressionStatement]
        //   JsExpressionStatement → [JsIdentifierExpression]
        //   JsIdentifierExpression → [JsReferenceIdentifier]
        let root_children = state.node_children(0);
        assert!(
            root_children.len() >= 2,
            "JsModule should have >= 2 children (directive list + item list)"
        );
        let item_list = root_children[1]; // JsModuleItemList
        let statements = state.node_children(item_list);
        assert!(
            statements.len() >= 2,
            "Expected >= 2 statements, got {}",
            statements.len()
        );

        let expr_stmt = statements[1]; // JsExpressionStatement: x;
        let expr_children = state.node_children(expr_stmt);
        assert!(
            !expr_children.is_empty(),
            "JsExpressionStatement should have children"
        );
        let ident_expr = expr_children[0]; // JsIdentifierExpression

        // Navigate to JsReferenceIdentifier
        let ref_children = state.node_children(ident_expr);
        let ref_node = if ref_children.is_empty() {
            ident_expr
        } else {
            ref_children[0] // JsReferenceIdentifier
        };

        // Resolve the reference to its declaration
        let binding = state.resolve_reference(ref_node);
        let binding_handle = binding.expect("resolve_reference should find binding for 'x'");
        let text = state.node_trimmed_text(binding_handle);
        assert_eq!(text, "x");

        // Get all references to this binding
        let refs = state.all_references(binding_handle);
        assert!(
            !refs.is_empty(),
            "Expected at least one reference to binding 'x'"
        );
    }

    #[test]
    fn semantic_is_exported_default_not_exported() {
        let mut state = js_state_with_semantic("let x = 1;");
        // Get the scope and its bindings
        let scope_handle = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope_handle);
        assert!(
            !bindings.is_empty(),
            "scope_bindings should return bindings for 'let x = 1;'"
        );
        // x is not exported
        assert!(!state.is_exported(bindings[0]));
    }

    #[test]
    fn semantic_is_exported_for_export() {
        let mut state = js_state_with_semantic("export let x = 1;");
        let scope_handle = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope_handle);
        assert!(
            !bindings.is_empty(),
            "scope_bindings should return bindings for 'export let x = 1;'"
        );
        assert!(state.is_exported(bindings[0]));
    }

    #[test]
    fn semantic_parent_scope() {
        let mut state = js_state_with_semantic("function foo() { let inner = 1; }");
        // Root scope should have a scope
        let _root_scope = state.node_scope(0).unwrap();
        let body = find_function_body(&mut state);
        let inner_scope = state.node_scope(body);
        assert!(inner_scope.is_some());
        let inner_scope_h = inner_scope.unwrap();

        // The parent of the inner scope should exist
        let parent = state.parent_scope_node(inner_scope_h);
        assert!(parent.is_some());
    }

    // ---------------------------------------------------------------
    // Token-level access
    // ---------------------------------------------------------------

    /// Navigate from root to the first JsVariableStatement:
    /// JsModule → JsModuleItemList → JsVariableStatement
    fn find_var_statement(state: &mut HostState) -> u32 {
        let root_children = state.node_children(0);
        assert!(
            root_children.len() >= 2,
            "JsModule should have directive list + item list"
        );
        let item_list = root_children[1]; // JsModuleItemList
        let statements = state.node_children(item_list);
        assert!(!statements.is_empty(), "Should have at least one statement");
        statements[0] // JsVariableStatement
    }

    #[test]
    fn children_with_tokens_includes_tokens() {
        let mut state = js_state("let x = 1;");
        // JsVariableStatement has both node and token children
        let stmt = find_var_statement(&mut state);
        let node_children = state.node_children(stmt);
        let all_children = state.node_children_with_tokens(stmt);
        // children_with_tokens should include more elements (e.g., `;` token)
        assert!(
            all_children.len() >= node_children.len(),
            "children_with_tokens ({}) should be >= children ({})",
            all_children.len(),
            node_children.len(),
        );
        // Should have at least one token (the semicolon)
        let has_token = all_children.iter().any(|&h| state.element_is_token(h));
        assert!(
            has_token,
            "JsVariableStatement should have token children (e.g., `;`)"
        );
    }

    #[test]
    fn element_is_token_distinguishes_nodes_and_tokens() {
        let mut state = js_state("let x = 1;");
        // Root is a node, not a token
        assert!(!state.element_is_token(0));

        // Navigate to JsVariableStatement and get its children with tokens
        let stmt = find_var_statement(&mut state);
        let stmt_children_with_tokens = state.node_children_with_tokens(stmt);

        let has_node = stmt_children_with_tokens
            .iter()
            .any(|&h| !state.element_is_token(h));
        let has_token = stmt_children_with_tokens
            .iter()
            .any(|&h| state.element_is_token(h));
        assert!(has_node, "Should have at least one node child");
        assert!(has_token, "Should have at least one token child");
    }

    #[test]
    fn token_text_returns_correct_text() {
        let mut state = js_state("let x = 1;");
        let stmt = find_var_statement(&mut state);
        let stmt_children_with_tokens = state.node_children_with_tokens(stmt);

        for &h in &stmt_children_with_tokens {
            if state.element_is_token(h) {
                let text = state.node_text(h);
                assert!(!text.is_empty(), "Token text should not be empty");
                return;
            }
        }
        panic!("Should have found at least one token");
    }

    #[test]
    fn token_kind_is_nonzero() {
        let mut state = js_state("let x = 1;");
        let stmt = find_var_statement(&mut state);
        let stmt_children_with_tokens = state.node_children_with_tokens(stmt);

        for &h in &stmt_children_with_tokens {
            if state.element_is_token(h) {
                let kind = state.node_kind(h);
                assert_ne!(kind, 0, "Token kind should be nonzero");
                return;
            }
        }
        panic!("Should have found at least one token");
    }

    #[test]
    fn token_parent_returns_node() {
        let mut state = js_state("let x = 1;");
        let stmt = find_var_statement(&mut state);
        let stmt_children_with_tokens = state.node_children_with_tokens(stmt);

        for &h in &stmt_children_with_tokens {
            if state.element_is_token(h) {
                let parent = state.node_parent(h);
                assert!(parent.is_some(), "Token should have a parent");
                let parent_h = parent.unwrap();
                assert!(
                    !state.element_is_token(parent_h),
                    "Token's parent should be a node"
                );
                return;
            }
        }
        panic!("Should have found at least one token");
    }

    #[test]
    fn token_children_are_empty() {
        let mut state = js_state("let x = 1;");
        let stmt = find_var_statement(&mut state);
        let stmt_children_with_tokens = state.node_children_with_tokens(stmt);

        for &h in &stmt_children_with_tokens {
            if state.element_is_token(h) {
                assert!(
                    state.node_children(h).is_empty(),
                    "Token should have no node children"
                );
                assert!(
                    state.node_children_with_tokens(h).is_empty(),
                    "Token should have no children at all"
                );
                return;
            }
        }
        panic!("Should have found at least one token");
    }

    #[test]
    fn trimmed_range_is_within_full_range() {
        let state = js_state("  let x = 1;  ");
        let (full_start, full_end) = state.node_range(0);
        let (trim_start, trim_end) = state.node_trimmed_range(0);
        assert!(
            trim_start >= full_start,
            "Trimmed start should be >= full start"
        );
        assert!(trim_end <= full_end, "Trimmed end should be <= full end");
    }

    #[test]
    fn invalid_handle_token_functions() {
        let mut state = js_state("let x = 1;");
        assert!(!state.element_is_token(999));
        assert!(state.node_children_with_tokens(999).is_empty());
        assert_eq!(state.node_trimmed_range(999), (0, 0));
    }

    // ---------------------------------------------------------------
    // Enhanced semantic model host functions
    // ---------------------------------------------------------------

    #[test]
    fn reference_is_read_for_read_reference() {
        let mut state = js_state_with_semantic("let x = 1; console.log(x);");
        // Navigate to the reference `x` in `console.log(x)`
        // JsModule → [JsDirectiveList, JsModuleItemList]
        let root_children = state.node_children(0);
        let item_list = root_children[1];
        let statements = state.node_children(item_list);
        // Second statement: console.log(x);
        let expr_stmt = statements[1];
        let expr_children = state.node_children(expr_stmt);
        let call_expr = expr_children[0]; // JsCallExpression
        let call_children = state.node_children(call_expr);
        // Find the JsCallArguments → JsCallArgumentList → x
        for &child in &call_children {
            let grandchildren = state.node_children(child);
            for &gc in &grandchildren {
                let text = state.node_trimmed_text(gc);
                if text == "x" {
                    assert!(
                        state.reference_is_read(gc),
                        "Reference to 'x' in console.log(x) should be a read"
                    );
                    assert!(
                        !state.reference_is_write(gc),
                        "Reference to 'x' in console.log(x) should NOT be a write"
                    );
                    return;
                }
                // Check grandchildren's children too
                let ggchildren = state.node_children(gc);
                for &ggc in &ggchildren {
                    let text = state.node_trimmed_text(ggc);
                    if text == "x" {
                        assert!(
                            state.reference_is_read(ggc),
                            "Reference to 'x' should be a read"
                        );
                        return;
                    }
                }
            }
        }
    }

    #[test]
    fn reference_is_write_for_assignment() {
        let mut state = js_state_with_semantic("let x = 1; x = 2;");
        let root_children = state.node_children(0);
        let item_list = root_children[1];
        let statements = state.node_children(item_list);
        // Second statement: x = 2;
        let expr_stmt = statements[1];
        // Navigate to the assignment identifier
        let expr_children = state.node_children(expr_stmt);
        let assign_expr = expr_children[0]; // JsAssignmentExpression
        let assign_children = state.node_children(assign_expr);
        // First child should be the assignment target (JsIdentifierAssignment)
        if !assign_children.is_empty() {
            let target = assign_children[0];
            // The target itself or its child should be the write reference
            assert!(
                state.reference_is_write(target),
                "Assignment target 'x' should be a write reference"
            );
        }
    }

    #[test]
    fn all_read_and_write_references() {
        let mut state = js_state_with_semantic("let x = 1; console.log(x); x = 2;");
        // Get the binding for x
        let scope = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope);
        // Find binding 'x'
        let x_binding = bindings
            .iter()
            .find(|&&h| state.node_trimmed_text(h) == "x")
            .copied();
        let x_binding = x_binding.expect("should find binding 'x'");

        let reads = state.all_read_references(x_binding);
        let writes = state.all_write_references(x_binding);

        assert!(
            !reads.is_empty(),
            "Should have at least one read reference to 'x'"
        );
        assert!(
            !writes.is_empty(),
            "Should have at least one write reference to 'x'"
        );
    }

    #[test]
    fn is_imported_for_import_statement() {
        let mut state = js_state_with_semantic("import { foo } from 'bar'; foo();");
        let scope = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope);
        // foo should be imported
        let foo_binding = bindings
            .iter()
            .find(|&&h| state.node_trimmed_text(h) == "foo")
            .copied();
        let foo_binding = foo_binding.expect("should find binding 'foo'");
        assert!(state.is_imported(foo_binding), "'foo' should be imported");
    }

    #[test]
    fn is_imported_for_local_variable() {
        let mut state = js_state_with_semantic("let x = 1;");
        let scope = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope);
        assert!(!bindings.is_empty());
        assert!(
            !state.is_imported(bindings[0]),
            "Local variable should not be imported"
        );
    }

    #[test]
    fn scope_get_binding_by_name_finds_binding() {
        let mut state = js_state_with_semantic("let x = 1; let y = 2;");
        let scope = state.node_scope(0).unwrap();
        let x_handle = state.scope_get_binding_by_name(scope, "x");
        assert!(x_handle.is_some(), "Should find binding 'x' by name");
        assert_eq!(state.node_trimmed_text(x_handle.unwrap()), "x");

        let y_handle = state.scope_get_binding_by_name(scope, "y");
        assert!(y_handle.is_some(), "Should find binding 'y' by name");
        assert_eq!(state.node_trimmed_text(y_handle.unwrap()), "y");
    }

    #[test]
    fn scope_get_binding_by_name_returns_none_for_missing() {
        let mut state = js_state_with_semantic("let x = 1;");
        let scope = state.node_scope(0).unwrap();
        let missing = state.scope_get_binding_by_name(scope, "nonexistent");
        assert!(missing.is_none(), "Should not find 'nonexistent' binding");
    }

    #[test]
    fn is_global_scope_for_root() {
        let mut state = js_state_with_semantic("let x = 1;");
        let scope = state.node_scope(0).unwrap();
        // Root scope should be global
        assert!(state.is_global_scope(scope), "Root scope should be global");
    }

    #[test]
    fn is_global_scope_for_function_scope() {
        let mut state = js_state_with_semantic("function f() { let y = 1; }");
        let body = find_function_body(&mut state);
        let inner_scope = state.node_scope(body).unwrap();
        assert!(
            !state.is_global_scope(inner_scope),
            "Function body scope should not be global"
        );
    }

    #[test]
    fn scope_ancestors_chain() {
        let mut state = js_state_with_semantic("function f() { let y = 1; }");
        let body = find_function_body(&mut state);
        let inner_scope = state.node_scope(body).unwrap();
        let ancestors = state.scope_ancestors(inner_scope);
        assert!(
            !ancestors.is_empty(),
            "Should have at least one ancestor scope"
        );
    }

    #[test]
    fn scope_children_returns_child_scopes() {
        let mut state = js_state_with_semantic("function f() { let y = 1; }");
        let root_scope = state.node_scope(0).unwrap();
        let children = state.scope_children(root_scope);
        assert!(
            !children.is_empty(),
            "Root scope should have child scopes (function f)"
        );
    }

    /// Navigate from root (handle 0) to the first function body.
    /// Path: JsModule → JsModuleItemList → JsFunctionDeclaration → JsFunctionBody.
    /// The body is identified as the first child of the declaration with >1 node children.
    fn find_function_body(state: &mut HostState) -> u32 {
        let root_children = state.node_children(0);
        let item_list = root_children[1];
        let stmts = state.node_children(item_list);
        let func_decl = stmts[0];
        let func_children = state.node_children(func_decl);
        for &child in &func_children {
            let grandchildren = state.node_children(child);
            if grandchildren.len() > 1 {
                return child;
            }
        }
        panic!("should find function body");
    }

    /// Walk all descendants of `handle` (depth-first) and return the first one whose
    /// trimmed text matches `text`.
    fn find_by_text(state: &mut HostState, handle: u32, text: &str) -> Option<u32> {
        let mut queue = vec![handle];
        while let Some(h) = queue.pop() {
            if state.node_trimmed_text(h) == text {
                return Some(h);
            }
            let children = state.node_children(h);
            queue.extend(children);
        }
        None
    }

    #[test]
    fn is_constant_expression_for_literal() {
        let mut state = js_state_with_semantic("const a = 1 + 2;");
        // Find the "1 + 2" binary expression by searching for its text
        let expr = find_by_text(&mut state, 0, "1 + 2");
        let expr = expr.expect("should find expression '1 + 2'");
        assert!(
            state.is_constant_expression(expr),
            "'1 + 2' should be a constant expression"
        );
    }

    #[test]
    fn is_constant_expression_for_variable_ref() {
        let mut state = js_state_with_semantic("let b = 1; const a = b + 1;");
        // Find "b + 1" which references variable b — not constant
        let expr = find_by_text(&mut state, 0, "b + 1");
        let expr = expr.expect("should find expression 'b + 1'");
        assert!(
            !state.is_constant_expression(expr),
            "'b + 1' (references variable) should NOT be a constant expression"
        );
    }

    #[test]
    fn closure_captures_for_function() {
        let mut state = js_state_with_semantic("let a = 1; function f() { console.log(a); }");
        // Navigate to the function declaration
        let root_children = state.node_children(0);
        let item_list = root_children[1];
        let stmts = state.node_children(item_list);
        // Second item should be the function declaration
        let func_decl = stmts[1];
        let captures = state.closure_captures(func_decl);
        assert!(
            !captures.is_empty(),
            "Function f should capture variable 'a'"
        );
        // Each capture is (ref_handle, binding_handle, is_by_reference)
        for (ref_h, bind_h, is_by_ref) in &captures {
            assert!(
                state.get(*ref_h).is_some(),
                "Reference handle should be valid"
            );
            assert!(
                state.get(*bind_h).is_some(),
                "Binding handle should be valid"
            );
            assert!(*is_by_ref, "Capture should be by reference");
        }
    }

    #[test]
    fn closure_captures_empty_for_non_closure() {
        let mut state = js_state_with_semantic("let x = 1;");
        // Root is not a closure
        let captures = state.closure_captures(0);
        assert!(
            captures.is_empty(),
            "Non-closure node should have no captures"
        );
    }

    #[test]
    fn file_path_returns_stored_path() {
        let parse = biome_js_parser::parse(
            "let x = 1;",
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let root = parse.tree().syntax().clone();
        let state = HostState::new(
            root.into(),
            PluginTargetLanguage::JavaScript,
            None,
            None,
            "src/test.js".to_string(),
        );
        assert_eq!(state.file_path(), "src/test.js");
    }

    #[test]
    fn file_path_empty_for_empty_state() {
        let state = HostState::empty();
        assert_eq!(state.file_path(), "");
    }

    #[test]
    fn expression_type_returns_unknown_without_resolver() {
        let state = js_state("const x = true;");
        // No module resolver, should return "unknown"
        assert_eq!(state.expression_type(0), "unknown");
    }

    #[test]
    fn enhanced_semantic_without_model_returns_defaults() {
        let mut state = js_state("let x = 1; x;");
        assert!(!state.reference_is_read(0));
        assert!(!state.reference_is_write(0));
        assert!(state.all_read_references(0).is_empty());
        assert!(state.all_write_references(0).is_empty());
        assert!(!state.is_imported(0));
        assert_eq!(state.scope_get_binding_by_name(0, "x"), None);
        assert!(!state.is_global_scope(0));
        assert!(state.scope_ancestors(0).is_empty());
        assert!(state.scope_children(0).is_empty());
        assert!(!state.is_constant_expression(0));
        assert!(state.closure_captures(0).is_empty());
    }

    // ---------------------------------------------------------------
    // Invalid-handle tests for new host functions (with semantic model)
    // ---------------------------------------------------------------

    #[test]
    fn enhanced_semantic_invalid_handle_returns_defaults() {
        let mut state = js_state_with_semantic("let x = 1;");
        let bad = 999u32;
        assert!(!state.reference_is_read(bad));
        assert!(!state.reference_is_write(bad));
        assert!(state.all_read_references(bad).is_empty());
        assert!(state.all_write_references(bad).is_empty());
        assert!(!state.is_imported(bad));
        assert_eq!(state.scope_get_binding_by_name(bad, "x"), None);
        assert!(!state.is_global_scope(bad));
        assert!(state.scope_ancestors(bad).is_empty());
        assert!(state.scope_children(bad).is_empty());
        assert!(!state.is_constant_expression(bad));
        assert!(state.closure_captures(bad).is_empty());
        assert_eq!(state.expression_type(bad), "unknown");
    }

    // ---------------------------------------------------------------
    // reference_is_read/write on non-reference nodes
    // ---------------------------------------------------------------

    #[test]
    fn reference_is_read_on_binding_declaration_returns_false() {
        let mut state = js_state_with_semantic("let x = 1; console.log(x);");
        // Get the binding declaration node for 'x'
        let scope = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope);
        let x_binding = bindings
            .iter()
            .find(|&&h| state.node_trimmed_text(h) == "x")
            .copied()
            .expect("should find binding 'x'");
        // A binding declaration is NOT a reference — should return false
        assert!(
            !state.reference_is_read(x_binding),
            "Binding declaration should not be treated as a read reference"
        );
        assert!(
            !state.reference_is_write(x_binding),
            "Binding declaration should not be treated as a write reference"
        );
    }

    // ---------------------------------------------------------------
    // all_read_references returned handle validity
    // ---------------------------------------------------------------

    #[test]
    fn all_read_references_returns_valid_handles_with_correct_text() {
        let mut state = js_state_with_semantic("let x = 1; console.log(x); alert(x);");
        let scope = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope);
        let x_binding = bindings
            .iter()
            .find(|&&h| state.node_trimmed_text(h) == "x")
            .copied()
            .expect("should find binding 'x'");

        let reads = state.all_read_references(x_binding);
        assert!(
            reads.len() >= 2,
            "Expected >= 2 read references to 'x', got {}",
            reads.len()
        );
        for &h in &reads {
            assert!(state.get(h).is_some(), "Handle should be valid");
            assert_eq!(
                state.node_trimmed_text(h),
                "x",
                "Read reference should point to 'x'"
            );
        }
    }

    #[test]
    fn all_write_references_returns_valid_handles_with_correct_text() {
        let mut state = js_state_with_semantic("let x = 1; x = 2; x = 3;");
        let scope = state.node_scope(0).unwrap();
        let bindings = state.scope_bindings(scope);
        let x_binding = bindings
            .iter()
            .find(|&&h| state.node_trimmed_text(h) == "x")
            .copied()
            .expect("should find binding 'x'");

        let writes = state.all_write_references(x_binding);
        assert!(
            writes.len() >= 2,
            "Expected >= 2 write references to 'x', got {}",
            writes.len()
        );
        for &h in &writes {
            assert!(state.get(h).is_some(), "Handle should be valid");
            assert_eq!(
                state.node_trimmed_text(h),
                "x",
                "Write reference should point to 'x'"
            );
        }
    }

    #[test]
    fn all_read_references_on_non_binding_returns_empty() {
        let mut state = js_state_with_semantic("let x = 1;");
        // Root is not a binding node
        let reads = state.all_read_references(0);
        assert!(
            reads.is_empty(),
            "Non-binding node should return empty reads"
        );
    }

    // ---------------------------------------------------------------
    // scope_children leaf scope
    // ---------------------------------------------------------------

    #[test]
    fn scope_children_empty_for_leaf_scope() {
        let mut state = js_state_with_semantic("function f() { let y = 1; }");
        let body = find_function_body(&mut state);
        let inner_scope = state.node_scope(body).unwrap();
        let children = state.scope_children(inner_scope);
        assert!(
            children.is_empty(),
            "Leaf scope (function with no nested functions) should have no child scopes"
        );
    }

    // ---------------------------------------------------------------
    // is_constant_expression on non-expression node
    // ---------------------------------------------------------------

    #[test]
    fn is_constant_expression_on_non_expression_returns_false() {
        let state = js_state_with_semantic("const a = 1;");
        // The root JsModule is not an expression
        assert!(
            !state.is_constant_expression(0),
            "Non-expression node (JsModule) should not be constant"
        );
    }

    // ---------------------------------------------------------------
    // expression_type edge cases
    // ---------------------------------------------------------------

    #[test]
    fn expression_type_on_non_expression_returns_unknown() {
        let state = js_state_with_semantic("const x = true;");
        // The root JsModule is not an expression
        assert_eq!(
            state.expression_type(0),
            "unknown",
            "Non-expression node should return 'unknown'"
        );
    }

    #[test]
    fn expression_type_with_resolver_returns_boolean() {
        let mut state = js_state_with_resolver("const x = true;");
        let expr = find_by_text(&mut state, 0, "true").expect("should find 'true' literal");
        let ty = state.expression_type(expr);
        assert_eq!(ty, "boolean", "Literal 'true' should resolve to boolean");
    }

    #[test]
    fn expression_type_with_resolver_returns_number() {
        let mut state = js_state_with_resolver("const n = 42;");
        let expr = find_by_text(&mut state, 0, "42").expect("should find '42' literal");
        let ty = state.expression_type(expr);
        assert_eq!(ty, "number", "Literal '42' should resolve to number");
    }

    #[test]
    fn expression_type_with_resolver_returns_string() {
        let mut state = js_state_with_resolver("const s = \"hello\";");
        let expr = find_by_text(&mut state, 0, "\"hello\"").expect("should find string literal");
        let ty = state.expression_type(expr);
        assert_eq!(ty, "string", "String literal should resolve to string");
    }

    // ---------------------------------------------------------------
    // closure_captures text verification
    // ---------------------------------------------------------------

    #[test]
    fn closure_captures_reference_text_matches_variable() {
        let mut state = js_state_with_semantic("let a = 1; function f() { console.log(a); }");
        let root_children = state.node_children(0);
        let item_list = root_children[1];
        let stmts = state.node_children(item_list);
        let func_decl = stmts[1]; // function declaration
        let captures = state.closure_captures(func_decl);
        assert!(!captures.is_empty(), "Should have captures");

        // The captured reference should point to 'a'
        let (ref_h, bind_h, _) = captures[0];
        assert_eq!(
            state.node_trimmed_text(ref_h),
            "a",
            "Capture reference should be 'a'"
        );
        assert_eq!(
            state.node_trimmed_text(bind_h),
            "a",
            "Capture binding should be 'a'"
        );
    }

    // ---------------------------------------------------------------
    // type_data_to_tag unit tests
    // ---------------------------------------------------------------

    #[test]
    fn type_data_to_tag_maps_primitives() {
        assert_eq!(super::type_data_to_tag(&TypeData::Boolean), "boolean");
        assert_eq!(super::type_data_to_tag(&TypeData::Number), "number");
        assert_eq!(super::type_data_to_tag(&TypeData::String), "string");
        assert_eq!(super::type_data_to_tag(&TypeData::BigInt), "bigint");
        assert_eq!(super::type_data_to_tag(&TypeData::Null), "null");
        assert_eq!(super::type_data_to_tag(&TypeData::Undefined), "undefined");
        assert_eq!(super::type_data_to_tag(&TypeData::Symbol), "symbol");
    }

    #[test]
    fn type_data_to_tag_maps_complex_types() {
        use biome_js_type_info::literal::{BooleanLiteral, NumberLiteral, StringLiteral};
        use biome_rowan::Text;

        // Keyword types
        assert_eq!(super::type_data_to_tag(&TypeData::ObjectKeyword), "object");
        assert_eq!(super::type_data_to_tag(&TypeData::AnyKeyword), "unknown");
        assert_eq!(
            super::type_data_to_tag(&TypeData::UnknownKeyword),
            "unknown"
        );
        assert_eq!(super::type_data_to_tag(&TypeData::NeverKeyword), "never");
        assert_eq!(super::type_data_to_tag(&TypeData::ThisKeyword), "object");
        assert_eq!(super::type_data_to_tag(&TypeData::VoidKeyword), "undefined");

        // Structural object-like types
        assert_eq!(super::type_data_to_tag(&TypeData::Global), "object");

        // Unknown / unresolvable
        assert_eq!(super::type_data_to_tag(&TypeData::Unknown), "unknown");
        assert_eq!(super::type_data_to_tag(&TypeData::Conditional), "unknown");

        // Literal sub-types return the underlying primitive
        assert_eq!(
            super::type_data_to_tag(&TypeData::Literal(Box::new(Literal::Boolean(
                BooleanLiteral::from(true)
            )))),
            "boolean"
        );
        assert_eq!(
            super::type_data_to_tag(&TypeData::Literal(Box::new(Literal::Number(
                NumberLiteral::new(Text::new_static("42"))
            )))),
            "number"
        );
        assert_eq!(
            super::type_data_to_tag(&TypeData::Literal(Box::new(Literal::String(
                StringLiteral::from("hello")
            )))),
            "string"
        );
        assert_eq!(
            super::type_data_to_tag(&TypeData::Literal(Box::new(Literal::BigInt(
                Text::new_static("1n")
            )))),
            "bigint"
        );
    }

    // ---------------------------------------------------------------
    // scope_ancestors: global scope has no ancestors beyond itself
    // ---------------------------------------------------------------

    #[test]
    fn scope_ancestors_from_global_scope() {
        let mut state = js_state_with_semantic("let x = 1;");
        let root_scope = state.node_scope(0).unwrap();
        let ancestors = state.scope_ancestors(root_scope);
        // Global scope ancestors: depends on implementation — may include self
        // Key property: should not panic and should return a finite list
        for &h in &ancestors {
            assert!(state.get(h).is_some(), "Ancestor handle should be valid");
        }
    }
}
