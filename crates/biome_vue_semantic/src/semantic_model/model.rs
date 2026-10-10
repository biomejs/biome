//! The tables of the Vue semantic model and the handles that read them.
//!
//! The model is a set of flat tables. Rows hold text ranges and token text,
//! never syntax nodes, so a model can be compared, cached and shared between
//! the analyzers of different languages. Rows refer to each other with small
//! integer ids.
//!
//! Every row belongs to one syntax tree: the host document, or one embedded
//! JavaScript snippet. A handle's `range()` is expressed in the coordinates of
//! the tree the row belongs to, which is what a lint rule analyzing that tree
//! needs for a diagnostic. `host_range()` is expressed in the coordinates of
//! the host document and is comparable across trees.

use biome_rowan::{TextRange, TextSize, TokenText};
use std::ops::Range;
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
    };
}

define_id!(
    /// Identifies a top-level block of a single-file component.
    BlockId
);
define_id!(
    /// Identifies an element of the host document.
    ElementId
);
define_id!(
    /// Identifies an attribute or directive of an element.
    AttrId
);
define_id!(
    /// Identifies a chain of `v-if`, `v-else-if` and `v-else` siblings.
    ChainId
);
define_id!(
    /// Identifies an embedded JavaScript snippet.
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
define_id!(
    /// Identifies a component.
    ComponentId
);

/// An answer that the source may not allow the model to give.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Tri {
    Yes,
    No,
    /// The source does not say, or says it in a form the model cannot read.
    #[default]
    Unknown,
}

impl From<bool> for Tri {
    fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
    }
}

/// The groups of names a component declares. A name is looked up in exactly
/// one of them.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Namespace {
    /// Variables: script bindings, template variables, and the state a
    /// component instance exposes (props, data, computed, methods).
    Binding,
    /// Events the component can emit.
    Emit,
    /// Slots the component renders.
    Slot,
    /// Components registered for use as tags.
    Component,
    /// Custom directives registered for use as `v-name`.
    Directive,
    /// Names given to elements with a static `ref="name"`.
    TemplateRef,
    /// Declarations nothing refers to by name: watchers, lifecycle hooks.
    Option,
}

impl Namespace {
    pub(crate) const COUNT: usize = 7;

    pub(crate) fn index(self) -> usize {
        self as usize
    }
}

/// Why a component's declarations in a namespace may be incomplete.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OpenReasons(u8);

impl OpenReasons {
    /// The declarations contain a spread element.
    pub const SPREAD: Self = Self(1 << 0);
    /// The component uses `extends` or `mixins`.
    pub const EXTENDS: Self = Self(1 << 1);
    /// A type argument refers to a type the model cannot read.
    pub const UNRESOLVED_TYPE: Self = Self(1 << 2);
    /// An option's value is not written as a literal.
    pub const NON_LITERAL: Self = Self(1 << 3);
    /// A `<script src>` block moves declarations to another file.
    pub const EXTERNAL_SRC: Self = Self(1 << 4);

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub(crate) fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

// ---------------------------------------------------------------------------
// Blocks
// ---------------------------------------------------------------------------

/// The kind of a top-level block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockKind {
    Template,
    Script,
    Style,
    /// Any other top-level element, such as `<i18n>`.
    Custom,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BlockData {
    pub(crate) kind: BlockKind,
    pub(crate) element: ElementId,
    pub(crate) content: TextRange,
    pub(crate) lang: Option<TokenText>,
    pub(crate) setup: bool,
    pub(crate) scoped: bool,
    pub(crate) module: bool,
    pub(crate) has_src: bool,
    pub(crate) is_empty: bool,
    pub(crate) snippet: Option<SnippetId>,
}

// ---------------------------------------------------------------------------
// Elements and attributes
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ElementData {
    pub(crate) range: TextRange,
    pub(crate) start_tag: TextRange,
    pub(crate) parent: Option<ElementId>,
    pub(crate) tag: TokenText,
    pub(crate) is_component: bool,
    pub(crate) tag_ref: Option<ReferenceId>,
    pub(crate) attrs: Range<u32>,
    pub(crate) classes: Range<u32>,
    pub(crate) scope: ScopeId,
    pub(crate) chain: Option<(ChainId, u8)>,
    pub(crate) self_closing: bool,
    pub(crate) class_open: bool,
}

/// The built-in directives, and `Custom` for everything else.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectiveKind {
    If,
    ElseIf,
    Else,
    For,
    Show,
    Bind,
    On,
    Model,
    Slot,
    Html,
    Text,
    Memo,
    Once,
    Pre,
    Cloak,
    Is,
    Custom,
}

impl DirectiveKind {
    pub(crate) fn from_name(name: &str) -> Self {
        match name {
            "v-if" => Self::If,
            "v-else-if" => Self::ElseIf,
            "v-else" => Self::Else,
            "v-for" => Self::For,
            "v-show" => Self::Show,
            "v-bind" => Self::Bind,
            "v-on" => Self::On,
            "v-model" => Self::Model,
            "v-slot" => Self::Slot,
            "v-html" => Self::Html,
            "v-text" => Self::Text,
            "v-memo" => Self::Memo,
            "v-once" => Self::Once,
            "v-pre" => Self::Pre,
            "v-cloak" => Self::Cloak,
            "v-is" => Self::Is,
            _ => Self::Custom,
        }
    }
}

/// The argument of a directive: the `href` of `v-bind:href`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirectiveArg {
    None,
    Static(TokenText),
    /// A dynamic argument, `:[name]`. The text is the expression between the
    /// brackets, which the model does not parse.
    Dynamic(TokenText),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AttrName {
    Plain(TokenText),
    Directive {
        kind: DirectiveKind,
        /// The directive name in its long form, such as `v-bind`.
        name: TokenText,
        arg: DirectiveArg,
        modifiers: Box<[TokenText]>,
        shorthand: bool,
        name_ref: Option<ReferenceId>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AttrValueData {
    Absent,
    Static(TokenText),
    Expr(SnippetId),
    /// A directive value the host parser reads itself (`v-for`), or one that
    /// no snippet was found for.
    Unparsed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AttrData {
    pub(crate) element: ElementId,
    pub(crate) range: TextRange,
    pub(crate) name: AttrName,
    pub(crate) value: AttrValueData,
    pub(crate) value_range: Option<TextRange>,
}

/// The role of an element in a conditional chain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BranchKind {
    If,
    ElseIf,
    Else,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BranchData {
    pub(crate) element: ElementId,
    pub(crate) kind: BranchKind,
    pub(crate) condition: Option<SnippetId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ChainData {
    pub(crate) parent: Option<ElementId>,
    pub(crate) branches: Box<[BranchData]>,
}

/// Whether a class is always applied or depends on a condition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Certainty {
    Always,
    Conditional,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ClassEntryData {
    pub(crate) name: Box<str>,
    pub(crate) range: TextRange,
    pub(crate) certainty: Certainty,
    pub(crate) from_expr: bool,
}

// ---------------------------------------------------------------------------
// Snippets
// ---------------------------------------------------------------------------

/// Where an embedded JavaScript snippet sits in the host document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnippetHost {
    /// The content of a `<script>` block.
    Script(BlockId),
    /// A `{{ }}` interpolation, child of the given element.
    Interpolation(Option<ElementId>),
    /// The value of a directive.
    DirectiveValue(AttrId),
    /// The iterable of a `v-for`.
    VForIterable(AttrId),
    /// A `v-bind()` inside a `<style>` block.
    StyleVBind(BlockId),
    /// A snippet the model could not place.
    Unknown,
}

/// The shape of the outermost expression of a snippet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RootShape {
    Identifier,
    MemberChain,
    Call,
    /// A string literal, or a template literal without substitutions.
    String(Box<str>),
    Bool(bool),
    Null,
    Number,
    Object,
    Array,
    Function {
        params: u8,
    },
    /// More than one statement, or a statement that is not an expression.
    Statements,
    /// `target = $event`, or `param => target = param`. `target` is the
    /// fingerprint of the assigned expression.
    Assignment {
        target: u64,
        from_event: bool,
    },
    /// `!operand`.
    Not,
    /// `a != b` or `a !== b`.
    NotEqual,
    Binary,
    Logical,
    Conditional,
    /// The parameters of a slot, as in `v-slot="{ item }"`.
    SlotParams,
    /// A script block.
    Module,
    Other,
    /// The snippet is empty or failed to parse.
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SnippetData {
    /// The content range in the host document.
    pub(crate) range: TextRange,
    /// The host offset of position zero of the snippet's tree.
    pub(crate) offset: TextSize,
    pub(crate) host: SnippetHost,
    pub(crate) root: RootShape,
    pub(crate) fingerprint: u64,
    pub(crate) operands: Range<u32>,
}

/// One operand of a logical condition. `a || (b && c)` has three operands in
/// two groups: `a` in group 0, `b` and `c` in group 1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Operand {
    pub(crate) range: TextRange,
    pub(crate) fingerprint: u64,
    pub(crate) or_group: u16,
}

impl Operand {
    /// The range of the operand in the host document.
    pub fn host_range(&self) -> TextRange {
        self.range
    }

    /// A hash of the operand's tokens, ignoring whitespace and comments.
    pub fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// The index of the `||` alternative this operand belongs to.
    pub fn or_group(&self) -> u16 {
        self.or_group
    }
}

// ---------------------------------------------------------------------------
// Scopes, symbols and references
// ---------------------------------------------------------------------------

/// What a scope belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeKind {
    /// Variables an element introduces with `v-for` or `v-slot`.
    Element(ElementId),
    /// The top-level bindings of a script block.
    Module(Option<BlockId>),
    /// What a component instance exposes: props, data, computed, methods.
    Instance(ComponentId),
    /// Names Vue provides in every template, such as `$emit`.
    Builtins,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ScopeData {
    pub(crate) kind: ScopeKind,
    pub(crate) parent: Option<ScopeId>,
    pub(crate) symbols: Vec<SymbolId>,
}

/// What kind of declaration a symbol is.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SymbolKind {
    /// A binding introduced by an `import`.
    Import,
    /// A top-level variable, function, class or type of a script block.
    Local,
    /// A variable introduced by `v-for`.
    VForAlias,
    /// A parameter introduced by `v-slot`.
    SlotParam,
    Prop,
    /// A two-way binding declared with `defineModel()`.
    Model,
    Data,
    /// Nuxt's `asyncData` option.
    AsyncData,
    Computed,
    Method,
    /// A member of the object returned by `setup()`.
    SetupReturn,
    Inject,
    Emit,
    Slot,
    ComponentRegistration,
    DirectiveRegistration,
    /// A static `ref="name"` in the template.
    TemplateRef,
    Watcher,
    LifecycleHook,
    /// A name Vue provides, such as `$emit`.
    Builtin,
}

/// What produced the value of a script binding.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Origin {
    #[default]
    None,
    DefineProps,
    DefineEmits,
    DefineModel,
    DefineSlots,
    Ref,
    ShallowRef,
    UseTemplateRef,
    Computed,
    /// The first parameter of `setup()`.
    SetupProps,
    /// The second parameter of `setup()`.
    SetupContext,
    /// `emit` destructured from the second parameter of `setup()`.
    SetupEmit,
    /// A variable destructured from the props object.
    PropsDestructure,
}

/// One runtime type a prop accepts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum PropType {
    Boolean = 1 << 0,
    String = 1 << 1,
    Number = 1 << 2,
    Array = 1 << 3,
    Object = 1 << 4,
    Function = 1 << 5,
    Symbol = 1 << 6,
    Date = 1 << 7,
    BigInt = 1 << 8,
    /// A type the model does not classify, or no type at all.
    Unknown = 1 << 9,
}

/// The set of runtime types a prop accepts.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TypeSet(u16);

impl TypeSet {
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns the set holding `ty` alone.
    pub const fn of(ty: PropType) -> Self {
        Self(ty as u16)
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, ty: PropType) -> bool {
        self.0 & ty as u16 != 0
    }

    /// Returns `true` when `ty` is the one type in the set.
    pub const fn is_only(self, ty: PropType) -> bool {
        self.0 == ty as u16
    }

    pub(crate) fn insert(&mut self, ty: PropType) {
        self.0 |= ty as u16;
    }

    pub(crate) fn union(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// The syntax a prop was declared with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PropForm {
    /// `props: ['name']`
    ArrayString,
    /// `props: { name: String }`
    Constructor,
    /// `props: { name: { type: String } }`
    Options,
    /// `defineProps<{ name: string }>()`
    TypeMember,
}

/// Where a prop's default value is written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefaultSource {
    /// `{ type: Boolean, default: true }`
    Inline,
    /// `withDefaults(defineProps<Props>(), { name: true })`
    WithDefaults,
    /// `const { name = true } = defineProps<Props>()`
    Destructure,
}

/// A default value of a prop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PropDefaultData {
    pub(crate) range: TextRange,
    pub(crate) source: DefaultSource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PropDetail {
    pub(crate) types: TypeSet,
    pub(crate) required: Tri,
    pub(crate) defaults: Box<[PropDefaultData]>,
    pub(crate) validator: Option<TextRange>,
    pub(crate) form: PropForm,
}

/// The syntax an emit was declared with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmitForm {
    /// `emits: ['change']`
    ArrayString,
    /// `emits: { change: null }` or `emits: { change: (id) => true }`
    Object,
    /// `defineEmits<{ (e: 'change', id: number): void }>()`
    CallSignature,
    /// `defineEmits<{ change: [id: number] }>()`
    TypeMember,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Detail {
    None,
    Prop(PropDetail),
    Emit {
        /// The number of payload arguments, when the declaration says.
        payload_params: Option<u8>,
        validator: Option<TextRange>,
        form: EmitForm,
    },
    Method {
        params: u8,
        rest: bool,
    },
    Computed {
        getter: Option<TextRange>,
        setter: Option<TextRange>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SymbolData {
    pub(crate) name: TokenText,
    /// The range of the name.
    pub(crate) range: TextRange,
    /// The range of the whole declaration.
    pub(crate) declaration: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) scope: ScopeId,
    pub(crate) namespace: Namespace,
    pub(crate) kind: SymbolKind,
    pub(crate) origin: Origin,
    pub(crate) parent: Option<SymbolId>,
    pub(crate) component: Option<ComponentId>,
    pub(crate) detail: Detail,
    pub(crate) type_only: bool,
}

/// Where a reference is written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefSite {
    /// An identifier in a JavaScript tree.
    Identifier,
    /// The tag name of an element.
    TagName(ElementId),
    /// The value of an `is` attribute.
    IsValue(AttrId),
    /// The name of a custom directive.
    DirectiveName(AttrId),
    /// The name of a `<slot>` element.
    SlotName(ElementId),
    /// The first argument of an emit call.
    EventName,
    /// The name passed to `useTemplateRef()`, or the member read from `$refs`.
    TemplateRefKey,
}

/// What a reference is reached through.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefBase {
    /// A plain name: `title`.
    Bare,
    /// A member of the component instance: `this.title`.
    This,
    /// A member of another binding: `props.title`, `emit('change')`.
    Via(SymbolId),
    /// A member of a name Vue provides: `$props.title`, `$emit('change')`.
    Builtin,
}

/// What a reference does with the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Access {
    Read,
    /// Assigned to, incremented, or deleted.
    Write,
    /// A method that changes the value in place is called on it.
    Mutate,
    Call,
    /// The value goes somewhere the model does not follow: it is passed as an
    /// argument, returned, spread, or indexed with a computed key.
    Escape,
}

/// What a reference refers to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resolution {
    Symbol(SymbolId),
    /// A JavaScript global the template may use, such as `Math`.
    Global,
    /// Nothing declares the name, and every place that could is readable.
    Unresolved,
    /// Nothing readable declares the name, but something unreadable might.
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
    pub(crate) path: Box<[TokenText]>,
    pub(crate) path_open: bool,
    pub(crate) access: Access,
    pub(crate) call_args: Option<u8>,
    pub(crate) resolution: Resolution,
}

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

/// The syntax a component was defined with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentKind {
    /// A `<script setup>` block.
    Setup,
    /// `export default { ... }` in a single-file component.
    OptionsObject,
    /// `defineComponent(...)`.
    DefineComponent,
    /// `createApp(...)`.
    CreateApp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ComponentData {
    pub(crate) kind: ComponentKind,
    pub(crate) definition: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) has_template: bool,
    pub(crate) instance: ScopeId,
    pub(crate) name: Option<(TokenText, TextRange)>,
    pub(crate) inherit_attrs: Tri,
    pub(crate) decl_open: [OpenReasons; Namespace::COUNT],
    pub(crate) uses_open: [bool; Namespace::COUNT],
    pub(crate) macro_conflict: bool,
    /// For a `<script setup>` component, the component defined by the default
    /// export of the plain `<script>` block of the same file.
    pub(crate) merged: Option<ComponentId>,
    pub(crate) symbols: Vec<SymbolId>,
}

// ---------------------------------------------------------------------------
// The model
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct SemanticModelData {
    pub(crate) blocks: Vec<BlockData>,
    pub(crate) elements: Vec<ElementData>,
    pub(crate) attrs: Vec<AttrData>,
    pub(crate) chains: Vec<ChainData>,
    pub(crate) class_entries: Vec<ClassEntryData>,
    pub(crate) snippets: Vec<SnippetData>,
    pub(crate) operands: Vec<Operand>,
    pub(crate) scopes: Vec<ScopeData>,
    pub(crate) symbols: Vec<SymbolData>,
    pub(crate) references: Vec<ReferenceData>,
    /// For each symbol, the references that resolve to it.
    pub(crate) uses: Vec<Vec<ReferenceId>>,
    pub(crate) components: Vec<ComponentData>,
}

impl SemanticModelData {
    fn snippet_offset(&self, snippet: Option<SnippetId>) -> TextSize {
        snippet.map_or(TextSize::from(0), |id| self.snippets[id.index()].offset)
    }

    /// Converts a host range to the coordinates of the tree it belongs to.
    fn local(&self, range: TextRange, snippet: Option<SnippetId>) -> TextRange {
        let offset = self.snippet_offset(snippet);
        range.checked_sub(offset).unwrap_or(range)
    }
}

/// The semantic model of the Vue components in one file.
///
/// For a `.vue` file the model describes the component, its template, and how
/// names in the template and the scripts relate. For a JavaScript or
/// TypeScript file it describes the components the file defines with
/// `defineComponent()` or `createApp()`, and has no template.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticModel {
    pub(crate) data: Arc<SemanticModelData>,
}

impl Default for SemanticModel {
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
            pub fn id(&self) -> $id {
                self.id
            }

            fn row(&self) -> &$row {
                &self.data.$table[self.id.index()]
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
    /// A top-level block of a single-file component.
    Block,
    BlockId,
    blocks,
    BlockData
);
define_handle!(
    /// An element of the host document.
    Element,
    ElementId,
    elements,
    ElementData
);
define_handle!(
    /// An attribute or directive of an element.
    Attr,
    AttrId,
    attrs,
    AttrData
);
define_handle!(
    /// A chain of `v-if`, `v-else-if` and `v-else` siblings.
    Chain,
    ChainId,
    chains,
    ChainData
);
define_handle!(
    /// An embedded JavaScript snippet.
    Snippet,
    SnippetId,
    snippets,
    SnippetData
);
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
define_handle!(
    /// A component.
    Component,
    ComponentId,
    components,
    ComponentData
);

impl SemanticModel {
    pub(crate) fn new(data: SemanticModelData) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    /// Returns every component the file defines, in source order.
    pub fn components(&self) -> impl Iterator<Item = Component> + '_ {
        (0..self.data.components.len()).map(|index| Component {
            data: self.data.clone(),
            id: ComponentId::new(index),
        })
    }

    /// Returns the component that owns the template of a single-file
    /// component.
    pub fn template_component(&self) -> Option<Component> {
        self.components().find(Component::has_template)
    }

    /// Returns the top-level blocks of a single-file component, in source
    /// order.
    pub fn blocks(&self) -> impl Iterator<Item = Block> + '_ {
        (0..self.data.blocks.len()).map(|index| Block {
            data: self.data.clone(),
            id: BlockId::new(index),
        })
    }

    /// Returns every element of the host document, parents before children.
    pub fn elements(&self) -> impl Iterator<Item = Element> + '_ {
        (0..self.data.elements.len()).map(|index| self.element(ElementId::new(index)))
    }

    /// Returns every embedded JavaScript snippet, in source order.
    pub fn snippets(&self) -> impl Iterator<Item = Snippet> + '_ {
        (0..self.data.snippets.len()).map(|index| self.snippet(SnippetId::new(index)))
    }

    /// Returns every declaration in the model.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        (0..self.data.symbols.len()).map(|index| self.symbol(SymbolId::new(index)))
    }

    /// Returns every reference in the model.
    pub fn references(&self) -> impl Iterator<Item = Reference> + '_ {
        (0..self.data.references.len()).map(|index| self.reference(ReferenceId::new(index)))
    }

    /// Returns every conditional chain in the template.
    pub fn chains(&self) -> impl Iterator<Item = Chain> + '_ {
        (0..self.data.chains.len()).map(|index| Chain {
            data: self.data.clone(),
            id: ChainId::new(index),
        })
    }

    /// Returns the element with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not obtained from this model.
    pub fn element(&self, id: ElementId) -> Element {
        Element {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the snippet with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not obtained from this model.
    pub fn snippet(&self, id: SnippetId) -> Snippet {
        Snippet {
            data: self.data.clone(),
            id,
        }
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

    /// Returns the reference with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not obtained from this model.
    pub fn reference(&self, id: ReferenceId) -> Reference {
        Reference {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the snippet whose tree starts at `offset` in the host document.
    ///
    /// A lint rule analyzing an embedded snippet knows that offset, and uses
    /// this to find which snippet it is running in.
    pub fn snippet_at_offset(&self, offset: TextSize) -> Option<Snippet> {
        self.data
            .snippets
            .iter()
            .position(|snippet| snippet.offset == offset)
            .map(|index| self.snippet(SnippetId::new(index)))
    }

    /// Returns the innermost element whose range in the host document
    /// contains `host_range`.
    pub fn element_at(&self, host_range: TextRange) -> Option<Element> {
        // Elements are stored parents first, so the last match is the innermost.
        self.data
            .elements
            .iter()
            .rposition(|element| element.range.contains_range(host_range))
            .map(|index| self.element(ElementId::new(index)))
    }

    /// Returns the reference written at `host_range`.
    pub fn reference_at(&self, host_range: TextRange) -> Option<Reference> {
        self.data
            .references
            .iter()
            .position(|reference| reference.range == host_range)
            .map(|index| self.reference(ReferenceId::new(index)))
    }

    /// Returns the elements at the root of the template block.
    pub fn roots(&self) -> impl Iterator<Item = Element> + '_ {
        let template = self
            .data
            .blocks
            .iter()
            .find(|block| block.kind == BlockKind::Template)
            .map(|block| block.element);
        self.elements()
            .filter(move |element| template.is_some() && element.row().parent == template)
    }
}

impl Block {
    pub fn kind(&self) -> BlockKind {
        self.row().kind
    }

    /// The element that makes up the block.
    pub fn element(&self) -> Element {
        Element {
            data: self.data.clone(),
            id: self.row().element,
        }
    }

    /// The range of the block's content in the host document.
    pub fn content_range(&self) -> TextRange {
        self.row().content
    }

    /// The value of the `lang` attribute.
    pub fn lang(&self) -> Option<&str> {
        self.row().lang.as_ref().map(TokenText::text)
    }

    /// Whether the block is a `<script setup>`.
    pub fn is_setup(&self) -> bool {
        self.row().setup
    }

    pub fn is_scoped(&self) -> bool {
        self.row().scoped
    }

    pub fn is_module(&self) -> bool {
        self.row().module
    }

    /// Whether the block has a `src` attribute, which moves its content to
    /// another file.
    pub fn has_src(&self) -> bool {
        self.row().has_src
    }

    /// Whether the block has no content other than whitespace.
    pub fn is_empty(&self) -> bool {
        self.row().is_empty
    }

    /// The snippet holding the block's content, for a script block.
    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| Snippet {
            data: self.data.clone(),
            id,
        })
    }
}

impl Element {
    /// The range of the element in the host document.
    pub fn range(&self) -> TextRange {
        self.row().range
    }

    /// The range of the opening tag in the host document.
    pub fn start_tag_range(&self) -> TextRange {
        self.row().start_tag
    }

    pub fn parent(&self) -> Option<Self> {
        self.row().parent.map(|id| Self {
            data: self.data.clone(),
            id,
        })
    }

    /// The tag name as written.
    pub fn tag(&self) -> &str {
        self.row().tag.text()
    }

    /// Whether the tag names a component and not a native element.
    pub fn is_component(&self) -> bool {
        self.row().is_component
    }

    /// The reference from the tag name to a component, for a component tag.
    pub fn tag_reference(&self) -> Option<Reference> {
        self.row().tag_ref.map(|id| Reference {
            data: self.data.clone(),
            id,
        })
    }

    pub fn is_self_closing(&self) -> bool {
        self.row().self_closing
    }

    pub fn attrs(&self) -> impl Iterator<Item = Attr> + '_ {
        self.row().attrs.clone().map(|index| Attr {
            data: self.data.clone(),
            id: AttrId(index),
        })
    }

    /// Returns the first directive of the given kind.
    pub fn directive(&self, kind: DirectiveKind) -> Option<Attr> {
        self.attrs()
            .find(|attr| attr.directive_kind() == Some(kind))
    }

    /// The scope that holds the variables this element introduces.
    pub fn scope(&self) -> Scope {
        Scope {
            data: self.data.clone(),
            id: self.row().scope,
        }
    }

    /// The conditional chain this element is a branch of, and its position in
    /// it.
    pub fn chain(&self) -> Option<(Chain, usize)> {
        self.row().chain.map(|(id, index)| {
            (
                Chain {
                    data: self.data.clone(),
                    id,
                },
                index as usize,
            )
        })
    }

    /// Returns the value of the attribute `name`, whether it is written as a
    /// plain attribute or bound with `v-bind`.
    pub fn effective(&self, name: &str) -> EffectiveValue {
        for attr in self.attrs() {
            match &attr.row().name {
                AttrName::Plain(plain) if plain.text() == name => {
                    return match &attr.row().value {
                        AttrValueData::Static(text) => EffectiveValue::Static(text.text().into()),
                        _ => EffectiveValue::Present,
                    };
                }
                AttrName::Directive {
                    kind: DirectiveKind::Bind,
                    arg: DirectiveArg::Static(arg),
                    ..
                } if arg.text() == name => {
                    return match attr.value() {
                        AttrValue::Expr(snippet) => match snippet.root() {
                            RootShape::String(text) => EffectiveValue::Static(text.clone()),
                            RootShape::Bool(value) => EffectiveValue::Bool(*value),
                            _ => EffectiveValue::Dynamic(snippet),
                        },
                        _ => EffectiveValue::Present,
                    };
                }
                _ => {}
            }
        }
        EffectiveValue::Absent
    }

    /// Returns the classes this element carries, from `class` and `:class`.
    pub fn classes(&self) -> impl Iterator<Item = ClassEntry<'_>> + '_ {
        self.data.class_entries[self.row().classes.start as usize..self.row().classes.end as usize]
            .iter()
            .map(|data| ClassEntry { data })
    }

    /// Whether part of the `:class` expression could not be read, so
    /// [`Self::classes`] may be incomplete.
    pub fn has_open_classes(&self) -> bool {
        self.row().class_open
    }
}

/// The value of an attribute, merged across its plain and bound forms.
#[derive(Clone, Debug, PartialEq)]
pub enum EffectiveValue {
    /// The element has no such attribute.
    Absent,
    /// The attribute is written without a value.
    Present,
    /// The value is a known string.
    Static(Box<str>),
    /// The value is a known boolean, as in `:disabled="true"`.
    Bool(bool),
    /// The value is computed at runtime.
    Dynamic(Snippet),
}

/// A class of an element.
#[derive(Clone, Copy, Debug)]
pub struct ClassEntry<'a> {
    data: &'a ClassEntryData,
}

impl ClassEntry<'_> {
    pub fn name(&self) -> &str {
        &self.data.name
    }

    /// The range of the class name in the host document.
    pub fn host_range(&self) -> TextRange {
        self.data.range
    }

    pub fn certainty(&self) -> Certainty {
        self.data.certainty
    }

    /// Whether the class comes from `:class` and not from `class`.
    pub fn is_from_expression(&self) -> bool {
        self.data.from_expr
    }
}

/// The value of an attribute or directive.
#[derive(Clone, Debug, PartialEq)]
pub enum AttrValue {
    /// No value is written.
    Absent,
    /// A plain string.
    Static(Box<str>),
    /// A JavaScript expression.
    Expr(Snippet),
    /// A value the model does not describe, such as that of `v-for`.
    Unparsed,
}

impl Attr {
    /// The range of the attribute in the host document.
    pub fn range(&self) -> TextRange {
        self.row().range
    }

    /// The range of the value, without its quotes, in the host document.
    pub fn value_range(&self) -> Option<TextRange> {
        self.row().value_range
    }

    pub fn element(&self) -> Element {
        Element {
            data: self.data.clone(),
            id: self.row().element,
        }
    }

    /// The name of a plain attribute.
    pub fn plain_name(&self) -> Option<&str> {
        match &self.row().name {
            AttrName::Plain(name) => Some(name.text()),
            AttrName::Directive { .. } => None,
        }
    }

    pub fn is_directive(&self) -> bool {
        matches!(self.row().name, AttrName::Directive { .. })
    }

    pub fn directive_kind(&self) -> Option<DirectiveKind> {
        match &self.row().name {
            AttrName::Directive { kind, .. } => Some(*kind),
            AttrName::Plain(_) => None,
        }
    }

    /// The directive name in its long form, such as `v-bind` for `:href`.
    pub fn directive_name(&self) -> Option<&str> {
        match &self.row().name {
            AttrName::Directive { name, .. } => Some(name.text()),
            AttrName::Plain(_) => None,
        }
    }

    pub fn directive_arg(&self) -> Option<&DirectiveArg> {
        match &self.row().name {
            AttrName::Directive { arg, .. } => Some(arg),
            AttrName::Plain(_) => None,
        }
    }

    /// The static argument of a directive: `href` for `:href`.
    pub fn static_arg(&self) -> Option<&str> {
        match self.directive_arg()? {
            DirectiveArg::Static(arg) => Some(arg.text()),
            _ => None,
        }
    }

    pub fn modifiers(&self) -> impl Iterator<Item = &str> + '_ {
        let modifiers: &[TokenText] = match &self.row().name {
            AttrName::Directive { modifiers, .. } => modifiers,
            AttrName::Plain(_) => &[],
        };
        modifiers.iter().map(TokenText::text)
    }

    pub fn has_modifier(&self, name: &str) -> bool {
        self.modifiers().any(|modifier| modifier == name)
    }

    /// Whether the directive is written in its short form: `:`, `@` or `#`.
    pub fn is_shorthand(&self) -> bool {
        matches!(
            self.row().name,
            AttrName::Directive {
                shorthand: true,
                ..
            }
        )
    }

    /// The reference from a custom directive's name to its declaration.
    pub fn directive_reference(&self) -> Option<Reference> {
        match &self.row().name {
            AttrName::Directive {
                name_ref: Some(id), ..
            } => Some(Reference {
                data: self.data.clone(),
                id: *id,
            }),
            _ => None,
        }
    }

    pub fn value(&self) -> AttrValue {
        match &self.row().value {
            AttrValueData::Absent => AttrValue::Absent,
            AttrValueData::Static(text) => AttrValue::Static(text.text().into()),
            AttrValueData::Expr(id) => AttrValue::Expr(Snippet {
                data: self.data.clone(),
                id: *id,
            }),
            AttrValueData::Unparsed => AttrValue::Unparsed,
        }
    }

    /// The snippet holding the value of a directive.
    pub fn value_snippet(&self) -> Option<Snippet> {
        match self.value() {
            AttrValue::Expr(snippet) => Some(snippet),
            _ => None,
        }
    }
}

/// A branch of a conditional chain.
#[derive(Clone, Debug)]
pub struct Branch {
    pub element: Element,
    pub kind: BranchKind,
    pub condition: Option<Snippet>,
}

impl Chain {
    /// The element whose children form the chain.
    pub fn parent(&self) -> Option<Element> {
        self.row().parent.map(|id| Element {
            data: self.data.clone(),
            id,
        })
    }

    pub fn branches(&self) -> impl Iterator<Item = Branch> + '_ {
        self.row().branches.iter().map(|branch| Branch {
            element: Element {
                data: self.data.clone(),
                id: branch.element,
            },
            kind: branch.kind,
            condition: branch.condition.map(|id| Snippet {
                data: self.data.clone(),
                id,
            }),
        })
    }
}

impl Snippet {
    /// The range of the snippet's content in the host document.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    /// The position in the host document of the start of the snippet's tree.
    pub fn offset(&self) -> TextSize {
        self.row().offset
    }

    pub fn host(&self) -> SnippetHost {
        self.row().host
    }

    /// The directive whose value or iterable this snippet is.
    pub fn attr(&self) -> Option<Attr> {
        match self.row().host {
            SnippetHost::DirectiveValue(id) | SnippetHost::VForIterable(id) => Some(Attr {
                data: self.data.clone(),
                id,
            }),
            _ => None,
        }
    }

    /// The element this snippet sits in or on.
    pub fn element(&self) -> Option<Element> {
        match self.row().host {
            SnippetHost::Interpolation(Some(id)) => Some(Element {
                data: self.data.clone(),
                id,
            }),
            SnippetHost::DirectiveValue(_) | SnippetHost::VForIterable(_) => {
                self.attr().map(|attr| attr.element())
            }
            _ => None,
        }
    }

    pub fn root(&self) -> &RootShape {
        &self.row().root
    }

    /// A hash of the snippet's tokens, ignoring whitespace and comments.
    ///
    /// Two snippets with equal fingerprints hold the same expression, up to
    /// hash collisions.
    pub fn fingerprint(&self) -> u64 {
        self.row().fingerprint
    }

    /// The operands of the snippet's root, when it is a `||` or `&&`
    /// expression.
    pub fn operands(&self) -> &[Operand] {
        let range = &self.row().operands;
        &self.data.operands[range.start as usize..range.end as usize]
    }

    /// The references written inside this snippet.
    pub fn references(&self) -> impl Iterator<Item = Reference> + '_ {
        let id = self.id;
        self.data
            .references
            .iter()
            .enumerate()
            .filter(move |(_, reference)| reference.snippet == Some(id))
            .map(|(index, _)| Reference {
                data: self.data.clone(),
                id: ReferenceId::new(index),
            })
    }
}

impl Scope {
    pub fn kind(&self) -> ScopeKind {
        self.row().kind
    }

    pub fn parent(&self) -> Option<Self> {
        self.row().parent.map(|id| Self {
            data: self.data.clone(),
            id,
        })
    }

    /// The declarations made directly in this scope.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.row().symbols.iter().map(|id| Symbol {
            data: self.data.clone(),
            id: *id,
        })
    }

    /// This scope followed by each enclosing scope, innermost first.
    pub fn ancestors(&self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(self.clone()), Self::parent)
    }

    /// Looks `name` up in this scope and its enclosing scopes.
    pub fn lookup(&self, namespace: Namespace, name: &str) -> Option<Symbol> {
        self.ancestors().find_map(|scope| {
            scope
                .symbols()
                .find(|symbol| symbol.namespace() == namespace && symbol.name() == name)
        })
    }
}

impl Symbol {
    pub fn name(&self) -> &str {
        self.row().name.text()
    }

    /// The range of the name, in the coordinates of the tree that declares it.
    pub fn range(&self) -> TextRange {
        self.data.local(self.row().range, self.row().snippet)
    }

    /// The range of the name in the host document.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    /// The range of the whole declaration, in the coordinates of the tree
    /// that declares it.
    pub fn declaration_range(&self) -> TextRange {
        self.data.local(self.row().declaration, self.row().snippet)
    }

    /// The snippet that declares the symbol. `None` for a declaration in the
    /// host document, or in a JavaScript file.
    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| Snippet {
            data: self.data.clone(),
            id,
        })
    }

    pub fn kind(&self) -> SymbolKind {
        self.row().kind
    }

    pub fn namespace(&self) -> Namespace {
        self.row().namespace
    }

    pub fn origin(&self) -> Origin {
        self.row().origin
    }

    pub fn scope(&self) -> Scope {
        Scope {
            data: self.data.clone(),
            id: self.row().scope,
        }
    }

    /// The symbol this one is nested in: `user` for `name` in
    /// `data() { return { user: { name: '' } } }`.
    pub fn parent(&self) -> Option<Self> {
        self.row().parent.map(|id| Self {
            data: self.data.clone(),
            id,
        })
    }

    pub fn component(&self) -> Option<Component> {
        self.row().component.map(|id| Component {
            data: self.data.clone(),
            id,
        })
    }

    /// Whether the symbol only exists in the type system.
    pub fn is_type_only(&self) -> bool {
        self.row().type_only
    }

    /// The references that resolve to this symbol.
    pub fn uses(&self) -> impl Iterator<Item = Reference> + '_ {
        self.data.uses[self.id.index()].iter().map(|id| Reference {
            data: self.data.clone(),
            id: *id,
        })
    }

    /// Whether [`Self::uses`] lists every use.
    ///
    /// It does not when the component uses names of this namespace in a way
    /// the model cannot follow, such as `$refs[key]` or `emit(eventName)`.
    pub fn has_complete_uses(&self) -> bool {
        self.row().component.is_none_or(|component| {
            !self.data.components[component.index()].uses_open[self.row().namespace.index()]
        })
    }

    pub(crate) fn detail(&self) -> &Detail {
        &self.row().detail
    }
}

impl Reference {
    pub fn name(&self) -> &str {
        self.row().name.text()
    }

    /// The range of the reference, in the coordinates of the tree it is
    /// written in.
    pub fn range(&self) -> TextRange {
        self.data.local(self.row().range, self.row().snippet)
    }

    /// The range of the reference in the host document.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| Snippet {
            data: self.data.clone(),
            id,
        })
    }

    pub fn site(&self) -> RefSite {
        self.row().site
    }

    pub fn namespace(&self) -> Namespace {
        self.row().namespace
    }

    pub fn base(&self) -> RefBase {
        self.row().base
    }

    /// The static members read after the name: `[b, c]` for `a.b.c`.
    pub fn path(&self) -> impl Iterator<Item = &str> + '_ {
        self.row().path.iter().map(TokenText::text)
    }

    /// Whether the member chain continues after [`Self::path`] with a member
    /// the model cannot name, such as `a.b[key]`.
    pub fn has_open_path(&self) -> bool {
        self.row().path_open
    }

    pub fn access(&self) -> Access {
        self.row().access
    }

    /// The number of arguments, when the reference is called. For an event
    /// name, the number of payload arguments.
    pub fn call_args(&self) -> Option<u8> {
        self.row().call_args
    }

    pub fn resolution(&self) -> Resolution {
        self.row().resolution
    }

    /// The symbol the reference resolves to.
    pub fn symbol(&self) -> Option<Symbol> {
        match self.row().resolution {
            Resolution::Symbol(id) => Some(Symbol {
                data: self.data.clone(),
                id,
            }),
            _ => None,
        }
    }
}

macro_rules! define_declaration {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(pub(crate) Symbol);

        impl std::ops::Deref for $name {
            type Target = Symbol;

            fn deref(&self) -> &Symbol {
                &self.0
            }
        }
    };
}

define_declaration!(
    /// A prop of a component, whatever syntax declared it.
    Prop
);
define_declaration!(
    /// An event a component declares it can emit.
    Emit
);
define_declaration!(
    /// A method of a component.
    Method
);
define_declaration!(
    /// A computed property of a component.
    Computed
);

/// A default value of a prop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PropDefault {
    range: TextRange,
    host_range: TextRange,
    source: DefaultSource,
}

impl PropDefault {
    /// The range of the default value, in the coordinates of the tree that
    /// declares the prop.
    pub fn range(&self) -> TextRange {
        self.range
    }

    /// The range of the default value in the host document.
    pub fn host_range(&self) -> TextRange {
        self.host_range
    }

    pub fn source(&self) -> DefaultSource {
        self.source
    }
}

impl Prop {
    fn prop_detail(&self) -> Option<&PropDetail> {
        match self.0.detail() {
            Detail::Prop(detail) => Some(detail),
            _ => None,
        }
    }

    /// The runtime types the prop accepts.
    ///
    /// The set is empty when the declaration names no type, as in
    /// `props: ['name']`.
    pub fn types(&self) -> TypeSet {
        self.prop_detail()
            .map(|detail| detail.types)
            .unwrap_or_default()
    }

    pub fn required(&self) -> Tri {
        self.prop_detail()
            .map_or(Tri::Unknown, |detail| detail.required)
    }

    /// The default values of the prop. A prop can have more than one when a
    /// default is written in several places.
    pub fn defaults(&self) -> impl Iterator<Item = PropDefault> + '_ {
        let snippet = self.0.row().snippet;
        self.prop_detail()
            .map(|detail| &*detail.defaults)
            .unwrap_or_default()
            .iter()
            .map(move |default| PropDefault {
                range: self.0.data.local(default.range, snippet),
                host_range: default.range,
                source: default.source,
            })
    }

    /// The range of the prop's validator function, in the coordinates of the
    /// tree that declares the prop.
    pub fn validator_range(&self) -> Option<TextRange> {
        let range = self.prop_detail()?.validator?;
        Some(self.0.data.local(range, self.0.row().snippet))
    }

    pub fn form(&self) -> Option<PropForm> {
        self.prop_detail().map(|detail| detail.form)
    }
}

impl Emit {
    /// The number of payload arguments the declaration specifies.
    pub fn payload_params(&self) -> Option<u8> {
        match self.0.detail() {
            Detail::Emit { payload_params, .. } => *payload_params,
            _ => None,
        }
    }

    /// The range of the validator function, in the coordinates of the tree
    /// that declares the emit.
    pub fn validator_range(&self) -> Option<TextRange> {
        match self.0.detail() {
            Detail::Emit {
                validator: Some(range),
                ..
            } => Some(self.0.data.local(*range, self.0.row().snippet)),
            _ => None,
        }
    }

    pub fn form(&self) -> Option<EmitForm> {
        match self.0.detail() {
            Detail::Emit { form, .. } => Some(*form),
            _ => None,
        }
    }
}

impl Method {
    /// The number of parameters the method declares.
    pub fn params(&self) -> Option<u8> {
        match self.0.detail() {
            Detail::Method { params, .. } => Some(*params),
            _ => None,
        }
    }

    pub fn has_rest_param(&self) -> bool {
        matches!(self.0.detail(), Detail::Method { rest: true, .. })
    }
}

impl Computed {
    /// The range of the getter function, in the coordinates of the tree that
    /// declares the computed property.
    pub fn getter_range(&self) -> Option<TextRange> {
        match self.0.detail() {
            Detail::Computed {
                getter: Some(range),
                ..
            } => Some(self.0.data.local(*range, self.0.row().snippet)),
            _ => None,
        }
    }

    pub fn has_setter(&self) -> bool {
        matches!(
            self.0.detail(),
            Detail::Computed {
                setter: Some(_),
                ..
            }
        )
    }
}

impl Component {
    pub fn kind(&self) -> ComponentKind {
        self.row().kind
    }

    /// The range of the definition, in the coordinates of the tree that holds
    /// it: the options object, the call, or the whole `<script setup>` block.
    pub fn range(&self) -> TextRange {
        self.data.local(self.row().definition, self.row().snippet)
    }

    /// The range of the definition in the host document.
    pub fn host_range(&self) -> TextRange {
        self.row().definition
    }

    /// The snippet that defines the component, in a single-file component.
    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| Snippet {
            data: self.data.clone(),
            id,
        })
    }

    /// Whether this component renders the template of the single-file
    /// component.
    pub fn has_template(&self) -> bool {
        self.row().has_template
    }

    /// The name the definition gives the component.
    pub fn name(&self) -> Option<&str> {
        self.row().name.as_ref().map(|(name, _)| name.text())
    }

    /// The range of the name, in the coordinates of the tree that holds it.
    pub fn name_range(&self) -> Option<TextRange> {
        let (_, range) = self.row().name.as_ref()?;
        Some(self.data.local(*range, self.row().snippet))
    }

    /// The value of the `inheritAttrs` option.
    pub fn inherit_attrs(&self) -> Tri {
        self.row().inherit_attrs
    }

    /// Whether the plain `<script>` block declares props or emits that the
    /// `<script setup>` block also declares with a macro.
    pub fn has_macro_conflict(&self) -> bool {
        self.row().macro_conflict
    }

    /// For a `<script setup>` component, the component defined by the default
    /// export of the plain `<script>` block of the same file.
    ///
    /// Vue merges the options of that export into the component, so the
    /// template can also use what it declares.
    pub fn plain_block_component(&self) -> Option<Self> {
        self.row().merged.map(|id| Self {
            data: self.data.clone(),
            id,
        })
    }

    /// Why the declarations in `namespace` may be incomplete.
    ///
    /// When the result is not empty, a name that no declaration matches may
    /// still be declared, and must not be reported as missing.
    pub fn open_reasons(&self, namespace: Namespace) -> OpenReasons {
        self.row().decl_open[namespace.index()]
    }

    /// Whether every declaration in `namespace` is known.
    pub fn is_complete(&self, namespace: Namespace) -> bool {
        self.open_reasons(namespace).is_empty()
    }

    /// The scope holding what the component instance exposes.
    pub fn instance_scope(&self) -> Scope {
        Scope {
            data: self.data.clone(),
            id: self.row().instance,
        }
    }

    /// Every declaration of the component, in source order.
    ///
    /// All of them are written in the tree that defines the component, so
    /// their [`Symbol::range`] can be used by a rule analyzing that tree.
    pub fn declarations(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.row().symbols.iter().map(|id| Symbol {
            data: self.data.clone(),
            id: *id,
        })
    }

    fn of_kind(&self, kind: SymbolKind) -> impl Iterator<Item = Symbol> + '_ {
        self.declarations()
            .filter(move |symbol| symbol.kind() == kind)
    }

    pub fn props(&self) -> impl Iterator<Item = Prop> + '_ {
        self.of_kind(SymbolKind::Prop).map(Prop)
    }

    pub fn models(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::Model)
    }

    pub fn emits(&self) -> impl Iterator<Item = Emit> + '_ {
        self.of_kind(SymbolKind::Emit).map(Emit)
    }

    pub fn data(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::Data)
            .filter(|symbol| symbol.row().parent.is_none())
    }

    pub fn async_data(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::AsyncData)
    }

    /// Computed properties, declared in the `computed` option or with
    /// `computed()` at the top level of `<script setup>`.
    pub fn computed(&self) -> impl Iterator<Item = Computed> + '_ {
        self.declarations()
            .filter(|symbol| {
                symbol.kind() == SymbolKind::Computed || symbol.origin() == Origin::Computed
            })
            .map(Computed)
    }

    pub fn methods(&self) -> impl Iterator<Item = Method> + '_ {
        self.of_kind(SymbolKind::Method).map(Method)
    }

    /// Bindings a setup function exposes to the template: the top-level
    /// bindings of `<script setup>`, or the members of the object `setup()`
    /// returns.
    pub fn setup_bindings(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.declarations().filter(|symbol| {
            matches!(
                symbol.kind(),
                SymbolKind::SetupReturn | SymbolKind::Local | SymbolKind::Import
            )
        })
    }

    pub fn watchers(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::Watcher)
    }

    pub fn hooks(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::LifecycleHook)
    }

    pub fn slots(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::Slot)
    }

    pub fn injects(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::Inject)
    }

    pub fn registered_components(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::ComponentRegistration)
    }

    pub fn registered_directives(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.of_kind(SymbolKind::DirectiveRegistration)
    }
}
