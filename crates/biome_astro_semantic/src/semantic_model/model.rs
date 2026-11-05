//! The tables of the Astro semantic model and the handles that read them.
//!
//! Stored and analyzer-facing ranges use host-document coordinates. Snippet
//! offsets retain the guest origin needed by integrations that operate on an
//! embedded syntax tree.

use super::table::{Table, TableId};
use biome_languages::JsFileSource;
use biome_rowan::{Text, TextRange, TextSize, TokenText};
use enumflags2::{BitFlags, bitflags};
use std::sync::Arc;

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(pub(crate) u32);

        impl $name {
            pub(crate) fn new(index: usize) -> Self {
                Self(index as u32)
            }

            pub(crate) fn index(self) -> usize {
                self.0 as usize
            }
        }

        impl TableId for $name {
            fn from_index(index: usize) -> Self {
                Self::new(index)
            }

            fn to_index(self) -> usize {
                self.index()
            }
        }
    };
}

define_id!(
    /// Identifies an Astro document block.
    BlockId
);
define_id!(
    /// Identifies an embedded JavaScript or TypeScript snippet.
    SnippetId
);
define_id!(
    /// Identifies a scope.
    ScopeId
);
define_id!(
    /// Identifies a declaration.
    SymbolId
);
define_id!(
    /// Identifies a use of a name.
    ReferenceId
);

/// An answer that the source may not allow the model to give.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Tri {
    /// The condition is known to hold.
    Yes,
    /// The condition is known not to hold.
    No,
    /// The available source does not determine the condition.
    #[default]
    Unknown,
}

impl From<bool> for Tri {
    fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
    }
}

/// A group of names resolved under the same rules.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Namespace {
    /// Runtime values, including component tags.
    Value,
    /// TypeScript type positions.
    Type,
    /// TypeScript namespace qualifiers.
    Namespace,
}

impl Namespace {
    pub(crate) const COUNT: usize = 3;

    pub(crate) fn index(self) -> usize {
        self as usize
    }
}

/// A reason declarations in a namespace may be incomplete.
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenReason {
    /// A spread may introduce declarations that are not visible in the source.
    Spread = 1 << 0,
    /// A computed value prevents the model from enumerating declarations.
    NonLiteral = 1 << 1,
    /// Declarations may come from source outside the document.
    ExternalSource = 1 << 2,
    /// A parse error prevents complete declaration collection.
    ParseError = 1 << 3,
    Computed = 1 << 4,
}

/// The reasons declarations in a namespace may be incomplete.
pub type OpenReasons = BitFlags<OpenReason>;

/// The kind of an Astro document block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockKind {
    /// The server-side frontmatter block.
    Frontmatter,
    /// A `script` element.
    Script,
    /// A `style` element.
    Style,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BlockData {
    pub(crate) kind: BlockKind,
    pub(crate) range: TextRange,
    pub(crate) content: Option<TextRange>,
    pub(crate) supported: bool,
    pub(crate) globally_scoped: bool,
    pub(crate) external: bool,
    pub(crate) scope: Option<ScopeId>,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) define_vars: Vec<DefineVarData>,
    pub(crate) define_vars_open: OpenReasons,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DefineVarData {
    pub(crate) name: TokenText,
    pub(crate) semantic_name: Text,
    pub(crate) range: TextRange,
    pub(crate) declaration: TextRange,
}

/// Where an embedded snippet sits in the host document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnippetHost {
    /// The snippet belongs to the given frontmatter block.
    Frontmatter(BlockId),
    /// The snippet belongs to the given script block.
    Script(BlockId),
    /// The snippet belongs to a `define:vars` directive on the given script block.
    ScriptDefineVars(BlockId),
    /// The snippet belongs to a `define:vars` directive on the given style block.
    StyleDefineVars(BlockId),
    /// The snippet is an expression in the template.
    Expression,
    /// The model could not associate the snippet with a document block.
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SnippetData {
    pub(crate) range: TextRange,
    pub(crate) offset: TextSize,
    pub(crate) source: JsFileSource,
    pub(crate) host: SnippetHost,
    pub(crate) scope: Option<ScopeId>,
}

/// What a scope belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeKind {
    /// A module scope, optionally associated with a document block.
    Module(Option<BlockId>),
    /// An isolated Astro template expression.
    Expression,
    /// Framework-provided names available throughout the document.
    Builtins,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ScopeData {
    pub(crate) kind: ScopeKind,
    pub(crate) parent: Option<ScopeId>,
    pub(crate) symbols: Vec<SymbolId>,
    pub(crate) open: [OpenReasons; Namespace::COUNT],
}

/// What kind of declaration a symbol is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SymbolKind {
    /// A declaration introduced by an import.
    Import,
    /// A declaration introduced by the document.
    Local,
    /// A declaration synthesized from framework semantics.
    Synthetic,
    /// A declaration supplied by Astro.
    Builtin,
}

/// What produced a declaration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Origin {
    /// The declaration has no framework-specific origin.
    #[default]
    None,
    /// Astro supplies the declaration as a built-in name.
    AstroBuiltin,
    /// A `define:vars` directive supplies the declaration.
    DefineVars,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SymbolData {
    pub(crate) name: TokenText,
    pub(crate) semantic_name: Text,
    pub(crate) range: TextRange,
    pub(crate) declaration: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) scope: ScopeId,
    pub(crate) namespace: Namespace,
    pub(crate) kind: SymbolKind,
    pub(crate) origin: Origin,
    pub(crate) declares_value: bool,
    pub(crate) declares_type: bool,
    pub(crate) declares_namespace: bool,
}

impl SymbolData {
    pub(crate) fn declares_in(&self, namespace: Namespace) -> bool {
        match namespace {
            Namespace::Value => self.declares_value,
            Namespace::Type => self.declares_type,
            Namespace::Namespace => self.declares_namespace,
        }
    }
}

/// Where a reference is written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefSite {
    /// A JavaScript or TypeScript identifier.
    Identifier,
    /// An HTML-family tag name.
    TagName,
    /// An HTML-family attribute.
    Attribute,
    /// A framework directive.
    Directive,
    /// A framework-defined use without a source reference.
    Implicit,
}

/// What a reference is reached through.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefBase {
    /// The reference uses a name directly.
    Bare,
    /// The reference is reached through the given symbol.
    Via(SymbolId),
    /// The reference is reached through an Astro built-in.
    Builtin,
}

/// What a reference does with a value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Access {
    /// Reads a value.
    Read,
    /// Assigns a value.
    Write,
    /// Mutates a value without replacing its binding.
    Mutate,
    /// Calls a value.
    Call,
    /// Passes a value to code whose behavior is not modeled.
    Escape,
    /// Uses a name in a type position.
    Type,
}

/// What a reference resolves to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resolution {
    /// The reference resolves to a declaration in this model.
    Symbol(SymbolId),
    /// The reference resolves as a global name.
    Global,
    /// No matching declaration or global is available.
    Unresolved,
    /// Open declarations prevent a definitive resolution.
    Unknowable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReferenceData {
    pub(crate) range: TextRange,
    pub(crate) name: TokenText,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) site: RefSite,
    pub(crate) namespace: Namespace,
    pub(crate) scope: Option<ScopeId>,
    pub(crate) base: RefBase,
    pub(crate) access: Access,
    pub(crate) resolution: Resolution,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct SemanticModelData {
    pub(crate) blocks: Table<BlockId, BlockData>,
    pub(crate) snippets: Table<SnippetId, SnippetData>,
    pub(crate) scopes: Table<ScopeId, ScopeData>,
    pub(crate) symbols: Table<SymbolId, SymbolData>,
    pub(crate) references: Table<ReferenceId, ReferenceData>,
    pub(crate) uses: Table<SymbolId, Vec<ReferenceId>>,
}

/// The semantic model of one Astro document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AstroSemanticModel {
    pub(crate) data: Arc<SemanticModelData>,
}

impl Default for AstroSemanticModel {
    fn default() -> Self {
        Self::new(SemanticModelData::default())
    }
}

macro_rules! define_handle {
    ($(#[$meta:meta])* $name:ident, $id:ident, $table:ident, $row:ident) => {
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $name {
            pub(crate) data: Arc<SemanticModelData>,
            pub(crate) id: $id,
        }

        impl $name {
            /// Returns the identifier of this value within its semantic model.
            pub fn id(&self) -> $id {
                self.id
            }

            fn row(&self) -> &$row {
                &self.data.$table[self.id]
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id && Arc::ptr_eq(&self.data, &other.data)
            }
        }

        impl Eq for $name {}

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.id.0).finish()
            }
        }
    };
}

define_handle!(
    /// An Astro document block.
    Block,
    BlockId,
    blocks,
    BlockData
);
define_handle!(
    /// An embedded JavaScript or TypeScript snippet.
    Snippet,
    SnippetId,
    snippets,
    SnippetData
);

/// A statically known key from a `define:vars` directive.
#[derive(Clone)]
pub struct DefineVar {
    data: Arc<SemanticModelData>,
    block: BlockId,
    index: usize,
}
define_handle!(
    /// A scope.
    Scope,
    ScopeId,
    scopes,
    ScopeData
);
define_handle!(
    /// A declaration.
    Symbol,
    SymbolId,
    symbols,
    SymbolData
);
define_handle!(
    /// A use of a name.
    Reference,
    ReferenceId,
    references,
    ReferenceData
);

impl AstroSemanticModel {
    pub(crate) fn new(data: SemanticModelData) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    /// Returns the frontmatter, script, and style blocks in source order.
    pub fn blocks(&self) -> impl Iterator<Item = Block> + '_ {
        (0..self.data.blocks.len()).map(|index| Block {
            data: self.data.clone(),
            id: BlockId::new(index),
        })
    }

    /// Returns the embedded JavaScript and TypeScript snippets in source order.
    pub fn snippets(&self) -> impl Iterator<Item = Snippet> + '_ {
        (0..self.data.snippets.len()).map(|index| Snippet {
            data: self.data.clone(),
            id: SnippetId::new(index),
        })
    }

    /// Returns all scopes in the model.
    pub fn scopes(&self) -> impl Iterator<Item = Scope> + '_ {
        (0..self.data.scopes.len()).map(|index| Scope {
            data: self.data.clone(),
            id: ScopeId::new(index),
        })
    }

    /// Returns all declarations in the model.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        let mut ids: Vec<_> = (0..self.data.symbols.len()).map(SymbolId::new).collect();
        ids.sort_by(|left, right| {
            let left = &self.data.symbols[*left];
            let right = &self.data.symbols[*right];
            left.range
                .start()
                .cmp(&right.range.start())
                .then_with(|| left.range.end().cmp(&right.range.end()))
                .then_with(|| left.semantic_name.text().cmp(right.semantic_name.text()))
                .then_with(|| (left.kind as u8).cmp(&(right.kind as u8)))
        });
        ids.into_iter().map(|id| Symbol {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns all name references in the model.
    pub fn references(&self) -> impl Iterator<Item = Reference> + '_ {
        (0..self.data.references.len()).map(|index| Reference {
            data: self.data.clone(),
            id: ReferenceId::new(index),
        })
    }

    /// Returns the declaration with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not obtained from this model.
    pub fn symbol(&self, id: SymbolId) -> Symbol {
        Symbol {
            data: self.data.clone(),
            id,
        }
    }
}

impl Block {
    /// Returns whether this is a frontmatter, script, or style block.
    pub fn kind(&self) -> BlockKind {
        self.row().kind
    }

    /// Returns the full block range in host-document coordinates.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    /// Returns the block content range in host-document coordinates.
    pub fn content_range(&self) -> Option<TextRange> {
        self.row().content
    }

    /// Returns whether this block's embedded language is supported.
    pub fn is_supported(&self) -> bool {
        self.row().supported
    }

    /// Returns whether the block's declarations are globally scoped.
    pub fn is_globally_scoped(&self) -> bool {
        self.row().globally_scoped
    }

    /// Returns whether the block loads its content from an external source.
    pub fn has_external_source(&self) -> bool {
        self.row().external
    }

    /// Returns the module scope associated with this block, if any.
    pub fn scope(&self) -> Option<Scope> {
        self.row().scope.map(|id| Scope {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns the embedded snippet associated with this block, if any.
    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| Snippet {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns statically known `define:vars` keys for this block.
    pub fn define_vars(&self) -> impl Iterator<Item = DefineVar> + '_ {
        (0..self.row().define_vars.len()).map(|index| DefineVar {
            data: self.data.clone(),
            block: self.id,
            index,
        })
    }

    /// Returns reasons additional `define:vars` keys may exist.
    pub fn define_vars_open_reasons(&self) -> OpenReasons {
        self.row().define_vars_open
    }
}

impl DefineVar {
    fn row(&self) -> &DefineVarData {
        &self.data.blocks[self.block].define_vars[self.index]
    }

    /// Returns the source spelling of the key.
    pub fn name(&self) -> &str {
        self.row().name.text()
    }

    /// Returns the decoded semantic name of the key.
    pub fn semantic_name(&self) -> &Text {
        &self.row().semantic_name
    }

    /// Returns the key range in host-document coordinates.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    /// Returns the object-member range in host-document coordinates.
    pub fn host_declaration_range(&self) -> TextRange {
        self.row().declaration
    }
}

impl Snippet {
    /// Returns the snippet content range in host-document coordinates.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    pub(crate) fn content_offset(&self) -> TextSize {
        self.row().offset
    }

    /// Returns the language and embedding mode used to parse the snippet.
    pub fn source(&self) -> JsFileSource {
        self.row().source
    }

    /// Returns where the snippet appears in the Astro document.
    pub fn host(&self) -> SnippetHost {
        self.row().host
    }

    /// The Astro scope the snippet resolves free names against.
    pub fn scope(&self) -> Option<Scope> {
        self.row().scope.map(|id| Scope {
            data: self.data.clone(),
            id,
        })
    }
}

impl Scope {
    /// Returns what this scope belongs to.
    pub fn kind(&self) -> ScopeKind {
        self.row().kind
    }

    /// Returns the enclosing scope, if any.
    pub fn parent(&self) -> Option<Self> {
        self.row().parent.map(|id| Self {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns declarations introduced directly in this scope.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.row().symbols.iter().map(|id| Symbol {
            data: self.data.clone(),
            id: *id,
        })
    }

    /// Returns this scope followed by each enclosing scope.
    pub fn ancestors(&self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(self.clone()), Self::parent)
    }

    /// Finds the nearest declaration with the decoded semantic name and namespace.
    pub fn lookup(&self, namespace: Namespace, name: &str) -> Option<Symbol> {
        self.ancestors().find_map(|scope| {
            scope
                .symbols()
                .filter(|symbol| {
                    symbol.row().declares_in(namespace) && symbol.semantic_name() == name
                })
                .min_by_key(|symbol| symbol.kind() == SymbolKind::Synthetic)
        })
    }

    /// Returns why declarations in the namespace may be incomplete.
    pub fn open_reasons(&self, namespace: Namespace) -> OpenReasons {
        self.row().open[namespace.index()]
    }

    /// Returns whether declarations in the namespace may be incomplete.
    pub fn is_open(&self, namespace: Namespace) -> bool {
        !self.open_reasons(namespace).is_empty()
    }
}

impl Symbol {
    /// Returns the source spelling of the declared name.
    pub fn name(&self) -> &str {
        self.row().name.text()
    }

    /// Returns the name after JavaScript identifier or static-key escape decoding.
    pub fn semantic_name(&self) -> &str {
        self.row().semantic_name.text()
    }

    /// Returns the name range in host-document coordinates.
    pub fn range(&self) -> TextRange {
        self.row().range
    }

    /// Returns the name range in host-document coordinates.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    /// Returns the full declaration range in host-document coordinates.
    pub fn declaration_range(&self) -> TextRange {
        self.row().declaration
    }

    /// Returns the full declaration range in host-document coordinates.
    pub fn host_declaration_range(&self) -> TextRange {
        self.row().declaration
    }

    /// Returns the embedded snippet containing the declaration, if any.
    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| Snippet {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns the scope that introduces the declaration.
    pub fn scope(&self) -> Scope {
        Scope {
            data: self.data.clone(),
            id: self.row().scope,
        }
    }

    /// Returns the namespace containing the declaration.
    pub fn namespace(&self) -> Namespace {
        self.row().namespace
    }

    /// Returns the kind of declaration.
    pub fn kind(&self) -> SymbolKind {
        self.row().kind
    }

    /// Returns the framework feature that produced the declaration.
    pub fn origin(&self) -> Origin {
        self.row().origin
    }

    /// Returns whether the declaration introduces a runtime value.
    pub fn declares_value(&self) -> bool {
        self.row().declares_value
    }

    /// Returns whether the declaration introduces a TypeScript type.
    pub fn declares_type(&self) -> bool {
        self.row().declares_type
    }

    /// Returns whether the declaration introduces a TypeScript namespace.
    pub fn declares_namespace(&self) -> bool {
        self.row().declares_namespace
    }

    /// Returns whether the declaration introduces a type but no runtime value.
    pub fn is_type_only(&self) -> bool {
        self.row().declares_type && !self.row().declares_value
    }

    /// Returns references that resolve to this declaration.
    pub fn uses(&self) -> impl Iterator<Item = Reference> + '_ {
        self.data.uses[self.id].iter().map(|id| Reference {
            data: self.data.clone(),
            id: *id,
        })
    }
}

impl Reference {
    /// Returns the referenced name.
    pub fn name(&self) -> &str {
        self.row().name.text()
    }

    /// Returns the reference range in host-document coordinates.
    pub fn range(&self) -> TextRange {
        self.row().range
    }

    /// Returns the reference range in host-document coordinates.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    /// Returns the embedded snippet containing the reference, if any.
    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| Snippet {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns the source construct containing the reference.
    pub fn site(&self) -> RefSite {
        self.row().site
    }

    /// Returns the namespace used to resolve the reference.
    pub fn namespace(&self) -> Namespace {
        self.row().namespace
    }

    /// Returns the Astro scope used to resolve the reference.
    ///
    /// References resolved entirely by the guest JavaScript model have no
    /// Astro scope.
    pub fn scope(&self) -> Option<Scope> {
        self.row().scope.map(|id| Scope {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns how the referenced name is reached.
    pub fn base(&self) -> RefBase {
        self.row().base
    }

    /// Returns how the reference uses its value.
    pub fn access(&self) -> Access {
        self.row().access
    }

    /// Returns the result of resolving the reference.
    pub fn resolution(&self) -> Resolution {
        self.row().resolution
    }
}
