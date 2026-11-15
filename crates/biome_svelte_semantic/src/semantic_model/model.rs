//! The tables of the Svelte semantic model and the handles that read them.
//!
//! The model is a set of flat tables. Rows hold text ranges and token text,
//! never syntax nodes, so a model can be compared, cached and shared between
//! the analyzers of different languages. Rows refer to each other with small
//! integer ids.
//!
//! A row that stands for a syntax node also holds its id: a [`JsNodeId`] for
//! a JavaScript node, an [`HtmlNodeId`] for a node of the host document. The
//! id names a pointer to the node, which only the analyzer of the tree holding
//! the node can resolve. See [`AnalyzedSvelteModel`](super::nodes::AnalyzedSvelteModel).
//!
//! Every row belongs to one syntax tree: the host document, or one embedded
//! JavaScript snippet. A handle's `range()` is expressed in the coordinates of
//! the tree the row belongs to, which is what a lint rule analyzing that tree
//! needs for a diagnostic. `host_range()` is expressed in the coordinates of
//! the host document and is comparable across trees.

use super::nodes::{HtmlNodeId, JsNodeId};
use biome_html_syntax::HtmlLanguage;
use biome_js_syntax::JsLanguage;
use biome_rowan::{SyntaxNodePtr, TextRange, TextSize, TokenText};
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
    /// Identifies a top-level `<script>` element.
    ScriptId
);
define_id!(
    /// Identifies a top-level `<style>` element.
    StyleId
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
    /// Identifies a logic block: `{#if}`, `{#each}`, `{#await}`, `{#key}` or
    /// `{#snippet}`.
    BlockId
);
define_id!(
    /// Identifies a branch of a logic block.
    BranchId
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
    /// Identifies a call of a rune.
    RuneCallId
);
define_id!(
    /// Identifies a prop the component declares.
    PropId
);
define_id!(
    /// Identifies a name a script exports.
    ExportId
);

/// What kind of file the model describes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileKind {
    /// A `.svelte` component.
    #[default]
    Component,
    /// A `.svelte.js` or `.svelte.ts` module.
    Module,
}

/// A file SvelteKit gives a meaning to by its name, under `src/routes`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteFile {
    /// `+page.svelte`, including the layout-reset form `+page@.svelte`.
    PageComponent,
    /// `+page.js` or `+page.ts`.
    PageModule,
    /// `+page.server.js` or `+page.server.ts`.
    PageServerModule,
    /// `+layout.svelte`, including the layout-reset form `+layout@.svelte`.
    LayoutComponent,
    /// `+layout.js` or `+layout.ts`.
    LayoutModule,
    /// `+layout.server.js` or `+layout.server.ts`.
    LayoutServerModule,
    /// `+error.svelte`.
    ErrorComponent,
    /// `+server.js` or `+server.ts`.
    Server,
}

/// Which `<script>` of a component an element is.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScriptKind {
    /// The instance script, run once per component instance.
    Instance,
    /// `<script module>`, or the legacy `<script context="module">`, run once
    /// when the module is first evaluated.
    Module,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ScriptData {
    pub(crate) kind: ScriptKind,
    pub(crate) element: ElementId,
    pub(crate) content: TextRange,
    pub(crate) lang: Option<TokenText>,
    pub(crate) snippet: Option<SnippetId>,
    /// The snippet holding the value of the `generics` attribute.
    pub(crate) generics: Option<SnippetId>,
    pub(crate) has_generics: bool,
    /// The scope of the script's top-level bindings.
    pub(crate) scope: ScopeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StyleData {
    pub(crate) element: ElementId,
    pub(crate) content: TextRange,
    pub(crate) lang: Option<TokenText>,
}

/// What an element stands for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ElementKind {
    /// An HTML, SVG or MathML element, such as `<div>`.
    Dom,
    /// A component: a capitalized tag such as `<Button>`, or a dotted one such
    /// as `<ui.Button>`.
    Component,
    /// `<svelte:element this={tag}>`, a DOM element whose tag is chosen at
    /// runtime.
    DynamicElement,
    /// Any other `svelte:` element, such as `<svelte:window>` or
    /// `<svelte:boundary>`.
    Special,
}

/// How many times an element can be rendered by one component instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Occurrence {
    /// Exactly once.
    Once,
    /// Once or not at all, as in a branch of `{#if}`.
    AtMostOnce,
    /// Any number of times, as in the body of `{#each}` or `{#snippet}`.
    Any,
}

impl Occurrence {
    /// The occurrence of an element whose parent has occurrence `self`, when
    /// the element appears `inner` times in the parent.
    pub(crate) fn then(self, inner: Self) -> Self {
        match (self, inner) {
            (Self::Once, other) | (other, Self::Once) => other,
            (Self::AtMostOnce, Self::AtMostOnce) => Self::AtMostOnce,
            _ => Self::Any,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ElementData {
    pub(crate) range: TextRange,
    pub(crate) start_tag: TextRange,
    /// The range between the opening and the closing tag.
    pub(crate) content: TextRange,
    pub(crate) parent: Option<ElementId>,
    pub(crate) tag: TokenText,
    pub(crate) kind: ElementKind,
    pub(crate) tag_ref: Option<ReferenceId>,
    pub(crate) attrs: Range<u32>,
    /// The scope the element is written in.
    pub(crate) scope: ScopeId,
    pub(crate) occurrence: Occurrence,
    pub(crate) self_closing: bool,
    pub(crate) node: HtmlNodeId,
}

/// The kind of a directive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectiveKind {
    /// `bind:`
    Bind,
    /// `class:`
    Class,
    /// `style:`
    Style,
    /// `use:`
    Use,
    /// `transition:`
    Transition,
    /// `in:`
    In,
    /// `out:`
    Out,
    /// `animate:`
    Animate,
}

impl DirectiveKind {
    /// The prefix the directive is written with, without the colon.
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Bind => "bind",
            Self::Class => "class",
            Self::Style => "style",
            Self::Use => "use",
            Self::Transition => "transition",
            Self::In => "in",
            Self::Out => "out",
            Self::Animate => "animate",
        }
    }

    /// Whether the name after the colon refers to a function in scope: the
    /// action of `use:`, the transition of `transition:`, `in:` and `out:`,
    /// and the animation of `animate:`.
    pub fn names_a_function(self) -> bool {
        matches!(
            self,
            Self::Use | Self::Transition | Self::In | Self::Out | Self::Animate
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AttrName {
    /// `name`, `name="value"` or `name={value}`.
    Plain(TokenText),
    /// `{name}`, which passes the variable `name` as the attribute `name`.
    Shorthand(TokenText),
    /// `{...value}`.
    Spread,
    /// `{@attach value}`.
    Attach,
    Directive {
        kind: DirectiveKind,
        /// The name after the colon, as written: `value`, `fade`, `a.b`.
        name: TokenText,
        modifiers: Box<[TokenText]>,
        /// The reference from the name to a function in scope, for the
        /// directives that name one.
        name_ref: Option<ReferenceId>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ValuePartData {
    Text(TokenText),
    Expr(Option<SnippetId>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AttrValueData {
    /// No value is written: `disabled`, or `bind:value` with its name as the
    /// value.
    Absent,
    /// A quoted string without expressions.
    Static(TokenText),
    /// A single expression: `name={value}`, `{name}`, `{...value}`.
    Expr(Option<SnippetId>),
    /// A quoted string with expressions: `class="a {b}"`.
    Interpolated(Box<[ValuePartData]>),
    /// The getter and the setter of a function binding:
    /// `bind:value={get, set}`.
    FunctionBinding {
        get: Option<SnippetId>,
        set: Option<SnippetId>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AttrData {
    pub(crate) element: ElementId,
    pub(crate) range: TextRange,
    pub(crate) name: AttrName,
    pub(crate) value: AttrValueData,
    /// The reference a directive without a value makes through its name, as
    /// `value` in `bind:value`.
    pub(crate) implicit_ref: Option<ReferenceId>,
    pub(crate) node: HtmlNodeId,
}

/// The kind of a logic block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockKind {
    If,
    Each,
    Await,
    Key,
    Snippet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BlockData {
    pub(crate) kind: BlockKind,
    pub(crate) range: TextRange,
    /// The element the block is written in.
    pub(crate) parent: Option<ElementId>,
    /// The expression the block opens with: the condition of `{#if}`, the
    /// list of `{#each}`, the promise of `{#await}`, the expression of
    /// `{#key}`, the signature of `{#snippet}`.
    pub(crate) expression: Option<SnippetId>,
    /// The key of `{#each}`.
    pub(crate) key: Option<SnippetId>,
    pub(crate) branches: Range<u32>,
    pub(crate) node: HtmlNodeId,
}

/// The role of a branch in its block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BranchKind {
    /// The first branch of `{#if}`.
    If,
    /// `{:else if}`.
    ElseIf,
    /// `{:else}` of `{#if}`.
    Else,
    /// The body of `{#each}`, rendered once per item.
    EachBody,
    /// `{:else}` of `{#each}`, rendered when the list is empty.
    EachElse,
    /// The content of `{#await}` shown while the promise is pending.
    Pending,
    /// The content shown once the promise resolves.
    Then,
    /// The content shown once the promise rejects.
    Catch,
    /// The body of `{#key}`.
    KeyBody,
    /// The body of `{#snippet}`, rendered once per `{@render}`.
    SnippetBody,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BranchData {
    pub(crate) block: BlockId,
    pub(crate) kind: BranchKind,
    /// The range of the branch's content.
    pub(crate) range: TextRange,
    /// The condition of `{#if}` and `{:else if}`.
    pub(crate) condition: Option<SnippetId>,
    pub(crate) scope: ScopeId,
}

/// Where an embedded JavaScript snippet sits in the host document.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnippetHost {
    /// The content of a `<script>`.
    Script(ScriptId),
    /// The `generics` attribute of a `<script>`.
    Generics(ScriptId),
    /// A `{value}` in the content of an element, whose value is rendered as
    /// text. `None` at the root of the template.
    Text(Option<ElementId>),
    /// The whole value of an attribute or directive, a shorthand attribute, a
    /// spread or an attachment.
    AttrValue(AttrId),
    /// An expression inside a quoted attribute value, as `b` in
    /// `class="a {b}"`, with its index among the parts of the value.
    AttrPart(AttrId, u16),
    /// The getter of a function binding.
    BindGetter(AttrId),
    /// The setter of a function binding.
    BindSetter(AttrId),
    /// The condition of a branch of `{#if}`.
    Condition(BranchId),
    /// The list of `{#each}`.
    EachList(BlockId),
    /// The key of `{#each}`.
    EachKey(BlockId),
    /// The promise of `{#await}`.
    AwaitPromise(BlockId),
    /// The expression of `{#key}`.
    KeyExpression(BlockId),
    /// The name and parameters of `{#snippet}`.
    SnippetSignature(BlockId),
    /// The declaration of a `{@const}` tag.
    ConstTag,
    /// The declaration of a `{let}` or `{const}` tag.
    DeclarationTag,
    /// The expression of `{@render}`.
    RenderTag,
    /// The expression of `{@html}`.
    HtmlTag,
    /// A name listed by `{@debug}`.
    DebugTag,
    /// A snippet the model could not place.
    Unknown,
}

/// The shape of the outermost expression of a snippet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RootShape {
    Identifier,
    MemberChain,
    Call,
    /// A string literal, or a template literal without substitutions. The
    /// text is written as in the source, between the quotes, with its escapes
    /// kept.
    String(Box<str>),
    Bool(bool),
    Null,
    Number,
    Object,
    Array,
    Function {
        params: u8,
    },
    Class,
    /// A template literal with substitutions.
    Template,
    /// An assignment, such as `count = 0` or `count += 1`. `target` is the
    /// fingerprint of the assigned expression.
    Assignment {
        target: u64,
    },
    /// `!operand`.
    Not,
    Binary,
    Logical,
    Conditional,
    /// A script.
    Module,
    /// The signature of `{#snippet}`.
    SnippetSignature,
    /// The declaration of a `{@const}`, `{let}` or `{const}` tag.
    Declaration,
    Other,
    /// The snippet is empty or failed to parse.
    Invalid,
}

/// Whether the outermost expression of a snippet can be assigned to, as the
/// value of `bind:` must be.
///
/// Parentheses and TypeScript wrappers such as `name as string` and `name!`
/// are looked through.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Assignability {
    /// A variable or a property: `name`, `user.name`, `list[0]`.
    Assignable,
    /// A property read through `?.`: `user?.name`.
    OptionalChain,
    /// Anything else.
    NotAssignable,
}

/// A value the model computed from the source, without running it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StaticValue {
    /// A string, with its escapes resolved.
    String(Box<str>),
    /// A number, as written in the source.
    Number(Box<str>),
    Bool(bool),
    Null,
    Undefined,
}

/// What the model knows about the value of an expression.
///
/// Identifiers are followed to the initializer of a `const` declared once,
/// across the scripts and the template, but not into the argument of a rune:
/// nothing is known of `x` in `const x = $state('a')`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValueFacts {
    pub(crate) value: Option<StaticValue>,
    pub(crate) strings: Option<Box<[Box<str>]>>,
    pub(crate) prefix: Option<Box<str>>,
    pub(crate) suffix: Option<Box<str>>,
}

impl ValueFacts {
    /// The value of the expression, when it is always the same.
    pub fn value(&self) -> Option<&StaticValue> {
        self.value.as_ref()
    }

    /// Every string the expression can evaluate to, when it can only
    /// evaluate to strings and the model can list them, as for
    /// `active ? 'on' : 'off'`.
    pub fn possible_strings(&self) -> Option<&[Box<str>]> {
        self.strings.as_deref()
    }

    /// The text every string value of the expression starts with, as `/a/`
    /// for `` `/a/${id}` ``.
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// The text every string value of the expression ends with.
    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SnippetData {
    /// The content range in the host document.
    pub(crate) range: TextRange,
    /// The host offset of position zero of the snippet's tree.
    pub(crate) offset: TextSize,
    pub(crate) host: SnippetHost,
    pub(crate) root: RootShape,
    pub(crate) assignability: Assignability,
    pub(crate) fingerprint: u64,
    pub(crate) operands: Range<u32>,
    pub(crate) has_comments: bool,
    /// Where the free names of the snippet are looked up. For a script, its
    /// own scope.
    pub(crate) scope: Option<ScopeId>,
    /// Whether the snippet is written inside a `{#snippet}` block.
    pub(crate) in_snippet_block: bool,
    pub(crate) facts: ValueFacts,
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

/// What a scope belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScopeKind {
    /// The top-level bindings of `<script module>`, or of a `.svelte.js`
    /// module.
    Module,
    /// The top-level bindings of the instance script.
    Instance,
    /// The root of the template.
    Template,
    /// A branch of a logic block.
    Branch(BranchId),
    /// The children of an element, which see the snippets declared among
    /// them and the `{@const}` tags of a component's children.
    Children(ElementId),
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
    /// A top-level variable, function, class or type of a script.
    Local,
    /// A variable `{#each}` declares for the current item.
    EachItem,
    /// The index variable of `{#each}`.
    EachIndex,
    /// The value of a resolved promise, in `{:then value}`.
    AwaitValue,
    /// The error of a rejected promise, in `{:catch error}`.
    AwaitError,
    /// The name of a `{#snippet}`.
    Snippet,
    /// A parameter of a `{#snippet}`.
    SnippetParam,
    /// A variable declared by a `{@const}` tag.
    ConstTag,
    /// A variable declared by a `{let}` or `{const}` tag.
    DeclarationTag,
}

/// How a script declares a binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclarationKind {
    Const,
    Let,
    Var,
    Function,
    Class,
    Import,
    /// A parameter of a `{#snippet}`.
    Parameter,
    /// A variable the template declares for the content of a block, such as
    /// the item of `{#each}` or the value of `{:then}`. Svelte does not let
    /// it be reassigned.
    BlockVariable,
    /// A type, an interface, or a type-only import.
    Type,
    /// An enum or a namespace.
    Other,
}

/// What an imported binding stands for in the module it comes from.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Imported {
    /// `import name from 'source'`
    Default,
    /// `import * as name from 'source'`
    Namespace,
    /// `import { name } from 'source'` or `import { name as alias } from
    /// 'source'`, with the name the source exports.
    Named(TokenText),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ImportData {
    pub(crate) source: TokenText,
    pub(crate) imported: Imported,
}

/// A function `svelte/store` provides to create a store.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreCreator {
    Writable,
    Readable,
    Derived,
}

impl StoreCreator {
    pub(crate) fn from_name(name: &str) -> Option<Self> {
        match name {
            "writable" => Some(Self::Writable),
            "readable" => Some(Self::Readable),
            "derived" => Some(Self::Derived),
            _ => None,
        }
    }
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
    pub(crate) kind: SymbolKind,
    pub(crate) declaration_kind: DeclarationKind,
    pub(crate) import: Option<ImportData>,
    /// The rune whose call initializes the binding: `$state` for
    /// `let count = $state(0)`, `$props` for `let { title } = $props()`.
    pub(crate) rune: Option<RuneCallId>,
    pub(crate) store: Option<StoreCreator>,
    /// For a variable of `{#each}`, the binding that holds the list.
    pub(crate) each_source: Option<SymbolId>,
    pub(crate) exported: bool,
    pub(crate) type_only: bool,
    /// What is known of the initializer of a `const`.
    pub(crate) facts: ValueFacts,
    /// The node of the whole declaration, for a declaration written in
    /// JavaScript.
    pub(crate) node: Option<JsNodeId>,
}

/// A rune.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Rune {
    /// `$state`
    State,
    /// `$state.raw`
    StateRaw,
    /// `$state.snapshot`
    StateSnapshot,
    /// `$state.eager`
    StateEager,
    /// `$derived`
    Derived,
    /// `$derived.by`
    DerivedBy,
    /// `$effect`
    Effect,
    /// `$effect.pre`
    EffectPre,
    /// `$effect.root`
    EffectRoot,
    /// `$effect.tracking`
    EffectTracking,
    /// `$effect.pending`
    EffectPending,
    /// `$props`
    Props,
    /// `$props.id`
    PropsId,
    /// `$bindable`
    Bindable,
    /// `$inspect`
    Inspect,
    /// `$inspect.trace`
    InspectTrace,
    /// `$host`
    Host,
}

impl Rune {
    /// The names of the runes, sorted.
    pub(crate) const BASE_NAMES: &[&str] = &[
        "$bindable",
        "$derived",
        "$effect",
        "$host",
        "$inspect",
        "$props",
        "$state",
    ];

    /// Whether `name` is the name of a rune, without the member of a rune
    /// such as `.by`.
    pub fn is_base_name(name: &str) -> bool {
        Self::BASE_NAMES.binary_search(&name).is_ok()
    }

    /// Returns the rune `base` names, read through `member` when it is
    /// `Some`: `$derived` and `by` give [`Self::DerivedBy`].
    pub fn from_name(base: &str, member: Option<&str>) -> Option<Self> {
        Some(match (base, member) {
            ("$state", None) => Self::State,
            ("$state", Some("raw")) => Self::StateRaw,
            ("$state", Some("snapshot")) => Self::StateSnapshot,
            ("$state", Some("eager")) => Self::StateEager,
            ("$derived", None) => Self::Derived,
            ("$derived", Some("by")) => Self::DerivedBy,
            ("$effect", None) => Self::Effect,
            ("$effect", Some("pre")) => Self::EffectPre,
            ("$effect", Some("root")) => Self::EffectRoot,
            ("$effect", Some("tracking")) => Self::EffectTracking,
            ("$effect", Some("pending")) => Self::EffectPending,
            ("$props", None) => Self::Props,
            ("$props", Some("id")) => Self::PropsId,
            ("$bindable", None) => Self::Bindable,
            ("$inspect", None) => Self::Inspect,
            ("$inspect", Some("trace")) => Self::InspectTrace,
            ("$host", None) => Self::Host,
            _ => return None,
        })
    }

    /// The name of the rune as written, such as `$derived.by`.
    pub fn name(self) -> &'static str {
        match self {
            Self::State => "$state",
            Self::StateRaw => "$state.raw",
            Self::StateSnapshot => "$state.snapshot",
            Self::StateEager => "$state.eager",
            Self::Derived => "$derived",
            Self::DerivedBy => "$derived.by",
            Self::Effect => "$effect",
            Self::EffectPre => "$effect.pre",
            Self::EffectRoot => "$effect.root",
            Self::EffectTracking => "$effect.tracking",
            Self::EffectPending => "$effect.pending",
            Self::Props => "$props",
            Self::PropsId => "$props.id",
            Self::Bindable => "$bindable",
            Self::Inspect => "$inspect",
            Self::InspectTrace => "$inspect.trace",
            Self::Host => "$host",
        }
    }

    /// Whether the rune declares reactive state: `$state` or `$state.raw`.
    pub fn is_state(self) -> bool {
        matches!(self, Self::State | Self::StateRaw)
    }

    /// Whether the rune declares a derived value: `$derived` or
    /// `$derived.by`.
    pub fn is_derived(self) -> bool {
        matches!(self, Self::Derived | Self::DerivedBy)
    }

    /// Whether the rune runs an effect: `$effect` or `$effect.pre`.
    pub fn is_effect(self) -> bool {
        matches!(self, Self::Effect | Self::EffectPre)
    }
}

/// Where the value a rune call returns is stored.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuneTarget {
    /// A variable: `let count = $state(0)`, or each variable of
    /// `let { a, b } = $props()`.
    Variable,
    /// A class field: `count = $state(0)` in a class body.
    ClassField(Box<str>),
    /// A field the constructor assigns: `this.count = $state(0)`.
    ConstructorField(Box<str>),
    /// Anything else, such as an argument or an expression statement.
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RuneCallData {
    pub(crate) rune: Rune,
    /// The range of the call.
    pub(crate) range: TextRange,
    /// The range of the callee, such as `$derived.by`.
    pub(crate) callee: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) target: RuneTarget,
    /// The symbols the call initializes, for a [`RuneTarget::Variable`]
    /// declared at the top level or in the template.
    pub(crate) symbols: Box<[SymbolId]>,
    pub(crate) args: u8,
    pub(crate) node: JsNodeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PropData {
    pub(crate) name: TokenText,
    /// The range of the name, where the destructuring pattern or the type
    /// declares it.
    pub(crate) range: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    /// The variable the destructuring pattern binds the prop to.
    pub(crate) local: Option<SymbolId>,
    /// The range of the default value.
    pub(crate) default: Option<TextRange>,
    /// Whether the default value is a `$bindable()` call.
    pub(crate) bindable: bool,
    /// Whether the type of the props declares the prop.
    pub(crate) in_type: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PropsData {
    pub(crate) call: RuneCallId,
    /// The variable holding all the props, for `let props = $props()`.
    pub(crate) object: Option<SymbolId>,
    /// The variable holding the props the pattern doesn't name, for
    /// `let { a, ...rest } = $props()`.
    pub(crate) rest: Option<SymbolId>,
    /// Whether the pattern has members the model cannot name.
    pub(crate) pattern_open: bool,
    /// Whether the declaration has a type annotation.
    pub(crate) has_type: bool,
    /// Whether the type of the props has members the model cannot list.
    pub(crate) type_open: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExportData {
    /// The name the export is visible as.
    pub(crate) name: TokenText,
    pub(crate) range: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) local: Option<SymbolId>,
    /// Whether the export moves a binding of another module, as in
    /// `export { a } from './a'`.
    pub(crate) reexport: bool,
    pub(crate) type_only: bool,
}

/// Where a reference is written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefSite {
    /// An identifier in a JavaScript tree.
    Identifier,
    /// The tag name of a component.
    TagName(ElementId),
    /// The name of a directive that names a function, as `fade` in
    /// `transition:fade`.
    DirectiveName(AttrId),
    /// The name of a directive without a value, as `value` in `bind:value`.
    DirectiveShorthand(AttrId),
    /// A use of the list of `{#each}` made by assigning to one of its
    /// variables, as `items` for `bind:value={item.name}`.
    EachItem,
}

/// What a reference does with the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Access {
    Read,
    /// Assigned to, incremented, or deleted. A write with a non-empty
    /// [`Reference::path`] changes a property, not the variable.
    Write,
    /// A method that changes the value in place is called on it.
    Mutate,
    Call,
    /// The value goes somewhere the model does not follow: it is passed as an
    /// argument, returned, spread, or assigned to another variable.
    Escape,
}

/// What a reference refers to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resolution {
    Symbol(SymbolId),
    /// A rune.
    Rune,
    /// Nothing in the component declares the name: it is a global, or it is
    /// not declared at all.
    Global,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReferenceData {
    pub(crate) range: TextRange,
    pub(crate) name: TokenText,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) site: RefSite,
    pub(crate) scope: Option<ScopeId>,
    /// For a use of the list of `{#each}`, the variable of the block the use
    /// is made through.
    pub(crate) via: Option<SymbolId>,
    pub(crate) path: Box<[TokenText]>,
    pub(crate) path_open: bool,
    pub(crate) access: Access,
    pub(crate) call_args: Option<u8>,
    pub(crate) resolution: Resolution,
    pub(crate) store_subscription: bool,
    pub(crate) in_function: bool,
    pub(crate) in_snippet_block: bool,
    pub(crate) type_only: bool,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct SemanticModelData {
    pub(crate) file_kind: FileKind,
    pub(crate) route_file: Option<RouteFile>,
    pub(crate) runes: bool,
    /// The value of `<svelte:options runes>`.
    pub(crate) runes_option: Option<bool>,
    pub(crate) scripts: Vec<ScriptData>,
    pub(crate) styles: Vec<StyleData>,
    pub(crate) elements: Vec<ElementData>,
    pub(crate) attrs: Vec<AttrData>,
    pub(crate) blocks: Vec<BlockData>,
    pub(crate) branches: Vec<BranchData>,
    pub(crate) snippets: Vec<SnippetData>,
    pub(crate) operands: Vec<Operand>,
    pub(crate) scopes: Vec<ScopeData>,
    pub(crate) symbols: Vec<SymbolData>,
    pub(crate) references: Vec<ReferenceData>,
    /// For each symbol, the references that resolve to it.
    pub(crate) uses: Vec<Vec<ReferenceId>>,
    /// The references written outside every snippet, then those of each
    /// snippet: index `0` holds the former, index `i + 1` those of snippet `i`.
    pub(crate) references_by_snippet: Vec<Vec<ReferenceId>>,
    pub(crate) rune_calls: Vec<RuneCallData>,
    pub(crate) props: Vec<PropData>,
    pub(crate) props_declaration: Option<PropsData>,
    pub(crate) exports: Vec<ExportData>,
    pub(crate) module_scope: Option<ScopeId>,
    pub(crate) instance_scope: Option<ScopeId>,
    pub(crate) template_scope: Option<ScopeId>,
    pub(crate) js_nodes: Vec<JsNodeData>,
    /// The nodes of the host document that rows of the model point to.
    pub(crate) html_nodes: Vec<SyntaxNodePtr<HtmlLanguage>>,
}

/// A JavaScript syntax node that rows of the model point to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct JsNodeData {
    /// The snippet whose tree holds the node. `None` for a JavaScript file.
    pub(crate) snippet: Option<SnippetId>,
    /// The node, in the coordinates of its tree.
    pub(crate) ptr: SyntaxNodePtr<JsLanguage>,
}

impl SemanticModelData {
    fn snippet_offset(&self, snippet: Option<SnippetId>) -> TextSize {
        snippet.map_or(TextSize::from(0), |id| self.snippets[id.index()].offset)
    }

    /// Converts a host range to the coordinates of the tree it belongs to.
    pub(crate) fn local(&self, range: TextRange, snippet: Option<SnippetId>) -> TextRange {
        let offset = self.snippet_offset(snippet);
        range.checked_sub(offset).unwrap_or(range)
    }
}

/// The semantic model of a Svelte component or of a `.svelte.js` module.
///
/// For a component, the model describes the scripts, the template, and how
/// names in the template and the scripts relate. For a module, it describes
/// the module's bindings and the runes it calls, and has no template.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SvelteSemanticModel {
    pub(crate) data: Arc<SemanticModelData>,
}

impl Default for SvelteSemanticModel {
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
    /// A top-level `<script>` element.
    Script,
    ScriptId,
    scripts,
    ScriptData
);
define_handle!(
    /// A top-level `<style>` element.
    Style,
    StyleId,
    styles,
    StyleData
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
    /// A logic block: `{#if}`, `{#each}`, `{#await}`, `{#key}` or
    /// `{#snippet}`.
    Block,
    BlockId,
    blocks,
    BlockData
);
define_handle!(
    /// A branch of a logic block.
    Branch,
    BranchId,
    branches,
    BranchData
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
    /// A call of a rune.
    RuneCall,
    RuneCallId,
    rune_calls,
    RuneCallData
);
define_handle!(
    /// A prop the component declares.
    Prop,
    PropId,
    props,
    PropData
);
define_handle!(
    /// A name a script exports.
    Export,
    ExportId,
    exports,
    ExportData
);

/// Creates the handle `$name` for `$id` in the table of `$self`.
macro_rules! handle {
    ($self:expr, $name:ident, $id:expr) => {
        $name {
            data: $self.data.clone(),
            id: $id,
        }
    };
}

/// Iterates over the handles of every row of a table.
macro_rules! all_rows {
    ($self:expr, $name:ident, $id:ident, $table:ident) => {
        (0..$self.data.$table.len()).map(|index| handle!($self, $name, $id::new(index)))
    };
}

impl SvelteSemanticModel {
    pub(crate) fn new(data: SemanticModelData) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    pub fn file_kind(&self) -> FileKind {
        self.data.file_kind
    }

    /// The meaning SvelteKit gives the file by its name, when the file is
    /// under `src/routes`.
    ///
    /// Only a component or a `.svelte.js` module has a model, so this is
    /// one of the component kinds. Use [`route_file`](super::route_file) to
    /// classify the path of any other route file, such as `+page.server.ts`.
    pub fn route_file(&self) -> Option<RouteFile> {
        self.data.route_file
    }

    /// Whether Svelte compiles the file in runes mode.
    ///
    /// A module is always in runes mode. A component is when
    /// `<svelte:options runes>` says so, or else when it refers to a rune or
    /// uses `await` at the top level of its instance script or of its
    /// template. The model does not read the compiler options of
    /// `svelte.config.js`.
    pub fn is_runes_mode(&self) -> bool {
        self.data.runes
    }

    /// The value of `<svelte:options runes>`, when it is written.
    pub fn runes_option(&self) -> Option<bool> {
        self.data.runes_option
    }

    /// Returns the top-level `<script>` elements, in source order.
    pub fn scripts(&self) -> impl Iterator<Item = Script> + '_ {
        all_rows!(self, Script, ScriptId, scripts)
    }

    /// Returns the `<script>` of the given kind.
    pub fn script(&self, kind: ScriptKind) -> Option<Script> {
        self.scripts().find(|script| script.kind() == kind)
    }

    /// Returns the top-level `<style>` elements, in source order.
    pub fn styles(&self) -> impl Iterator<Item = Style> + '_ {
        all_rows!(self, Style, StyleId, styles)
    }

    /// Returns every element of the host document, parents before children.
    pub fn elements(&self) -> impl Iterator<Item = Element> + '_ {
        all_rows!(self, Element, ElementId, elements)
    }

    /// Returns every logic block, in source order.
    pub fn blocks(&self) -> impl Iterator<Item = Block> + '_ {
        all_rows!(self, Block, BlockId, blocks)
    }

    /// Returns every embedded JavaScript snippet, in source order.
    pub fn snippets(&self) -> impl Iterator<Item = Snippet> + '_ {
        all_rows!(self, Snippet, SnippetId, snippets)
    }

    /// Returns every declaration in the model.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        all_rows!(self, Symbol, SymbolId, symbols)
    }

    /// Returns every reference in the model.
    pub fn references(&self) -> impl Iterator<Item = Reference> + '_ {
        all_rows!(self, Reference, ReferenceId, references)
    }

    /// Returns every call of a rune, in the order the trees were read.
    pub fn rune_calls(&self) -> impl Iterator<Item = RuneCall> + '_ {
        all_rows!(self, RuneCall, RuneCallId, rune_calls)
    }

    /// Returns every name the scripts export.
    pub fn exports(&self) -> impl Iterator<Item = Export> + '_ {
        all_rows!(self, Export, ExportId, exports)
    }

    /// Returns the props the component declares: the members of the
    /// destructuring pattern of `$props()`, and the members of its type.
    pub fn props(&self) -> impl Iterator<Item = Prop> + '_ {
        all_rows!(self, Prop, PropId, props)
    }

    /// Returns the declaration of the props, the `$props()` call of the
    /// instance script.
    pub fn props_declaration(&self) -> Option<PropsDeclaration<'_>> {
        self.data
            .props_declaration
            .as_ref()
            .map(|data| PropsDeclaration { model: self, data })
    }

    /// Returns the references written in `snippet`, or, for `None`, those
    /// written in the host document or in a module.
    pub fn references_in(
        &self,
        snippet: Option<SnippetId>,
    ) -> impl Iterator<Item = Reference> + '_ {
        self.data
            .references_by_snippet
            .get(snippet.map_or(0, |id| id.index() + 1))
            .into_iter()
            .flatten()
            .map(|id| handle!(self, Reference, *id))
    }

    /// The scope of the top-level bindings of `<script module>`, or of a
    /// module.
    pub fn module_scope(&self) -> Option<Scope> {
        self.data.module_scope.map(|id| handle!(self, Scope, id))
    }

    /// The scope of the top-level bindings of the instance script.
    pub fn instance_scope(&self) -> Option<Scope> {
        self.data.instance_scope.map(|id| handle!(self, Scope, id))
    }

    /// The scope of the root of the template.
    pub fn template_scope(&self) -> Option<Scope> {
        self.data.template_scope.map(|id| handle!(self, Scope, id))
    }

    /// Returns the element with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not obtained from this model.
    pub fn element(&self, id: ElementId) -> Element {
        handle!(self, Element, id)
    }

    /// Returns the snippet with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not obtained from this model.
    pub fn snippet(&self, id: SnippetId) -> Snippet {
        handle!(self, Snippet, id)
    }

    /// Returns the declaration with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not obtained from this model.
    pub fn symbol(&self, id: SymbolId) -> Symbol {
        handle!(self, Symbol, id)
    }

    /// Returns the reference with the given id.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not obtained from this model.
    pub fn reference(&self, id: ReferenceId) -> Reference {
        handle!(self, Reference, id)
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
            .map(|index| handle!(self, Snippet, SnippetId::new(index)))
    }

    /// Returns the innermost element whose range in the host document
    /// contains `host_range`.
    pub fn element_at(&self, host_range: TextRange) -> Option<Element> {
        // Elements are stored parents first, so the last match is the innermost.
        self.data
            .elements
            .iter()
            .rposition(|element| element.range.contains_range(host_range))
            .map(|index| handle!(self, Element, ElementId::new(index)))
    }

    /// Returns the attribute or directive written at `host_range`.
    ///
    /// `host_range` must be the whole attribute without its surrounding
    /// whitespace.
    pub fn attr_at(&self, host_range: TextRange) -> Option<Attr> {
        // Attributes are stored in source order and do not overlap.
        let index = self
            .data
            .attrs
            .partition_point(|attr| attr.range.start() < host_range.start());
        let attr = self.data.attrs.get(index)?;
        (attr.range == host_range).then(|| handle!(self, Attr, AttrId::new(index)))
    }

    /// Returns the reference written at `host_range`.
    pub fn reference_at(&self, host_range: TextRange) -> Option<Reference> {
        self.data
            .references
            .iter()
            .position(|reference| reference.range == host_range && reference.via.is_none())
            .map(|index| handle!(self, Reference, ReferenceId::new(index)))
    }

    /// Returns the symbol whose name is written at `host_range`.
    pub fn symbol_at(&self, host_range: TextRange) -> Option<Symbol> {
        self.data
            .symbols
            .iter()
            .position(|symbol| symbol.range == host_range)
            .map(|index| handle!(self, Symbol, SymbolId::new(index)))
    }

    /// Returns the rune call written at `host_range`.
    pub fn rune_call_at(&self, host_range: TextRange) -> Option<RuneCall> {
        self.data
            .rune_calls
            .iter()
            .position(|call| call.range == host_range)
            .map(|index| handle!(self, RuneCall, RuneCallId::new(index)))
    }

    /// Returns the branches whose content contains `host_range`, innermost
    /// first.
    ///
    /// A rule uses it to know whether an element or an expression is only
    /// rendered under a condition, as inside `{#if browser}`.
    pub fn branches_containing(&self, host_range: TextRange) -> impl Iterator<Item = Branch> + '_ {
        // Branches are stored in source order, so an enclosing branch comes
        // before the branches it contains.
        let mut found: Vec<_> = self
            .data
            .branches
            .iter()
            .enumerate()
            .filter(|(_, branch)| branch.range.contains_range(host_range))
            .map(|(index, _)| BranchId::new(index))
            .collect();
        found.reverse();
        found.into_iter().map(|id| handle!(self, Branch, id))
    }

    /// Returns the elements at the root of the template, other than the
    /// top-level `<script>` and `<style>` elements.
    pub fn roots(&self) -> impl Iterator<Item = Element> + '_ {
        let excluded: Vec<_> = self
            .data
            .scripts
            .iter()
            .map(|script| script.element)
            .chain(self.data.styles.iter().map(|style| style.element))
            .collect();
        self.elements().filter(move |element| {
            element.row().parent.is_none() && !excluded.contains(&element.id)
        })
    }
}

impl Script {
    pub fn kind(&self) -> ScriptKind {
        self.row().kind
    }

    pub fn element(&self) -> Element {
        handle!(self, Element, self.row().element)
    }

    /// The range of the content in the host document.
    pub fn content_range(&self) -> TextRange {
        self.row().content
    }

    /// The value of the `lang` attribute.
    pub fn lang(&self) -> Option<&str> {
        self.row().lang.as_ref().map(TokenText::text)
    }

    /// The snippet holding the content.
    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| handle!(self, Snippet, id))
    }

    /// Whether the element has a `generics` attribute.
    pub fn has_generics(&self) -> bool {
        self.row().has_generics
    }

    /// The snippet holding the value of the `generics` attribute.
    pub fn generics(&self) -> Option<Snippet> {
        self.row().generics.map(|id| handle!(self, Snippet, id))
    }

    /// The scope of the script's top-level bindings.
    pub fn scope(&self) -> Scope {
        handle!(self, Scope, self.row().scope)
    }
}

impl Style {
    pub fn element(&self) -> Element {
        handle!(self, Element, self.row().element)
    }

    /// The range of the content in the host document.
    pub fn content_range(&self) -> TextRange {
        self.row().content
    }

    /// The value of the `lang` attribute.
    pub fn lang(&self) -> Option<&str> {
        self.row().lang.as_ref().map(TokenText::text)
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

    /// The range between the opening and the closing tag in the host
    /// document. Empty for a self-closing element.
    pub fn content_range(&self) -> TextRange {
        self.row().content
    }

    /// The node of the element, the one [`Self::range`] covers.
    pub fn html_node(&self) -> HtmlNodeId {
        self.row().node
    }

    pub fn parent(&self) -> Option<Self> {
        self.row().parent.map(|id| handle!(self, Element, id))
    }

    /// The tag name as written.
    pub fn tag(&self) -> &str {
        self.row().tag.text()
    }

    pub fn kind(&self) -> ElementKind {
        self.row().kind
    }

    pub fn is_component(&self) -> bool {
        self.row().kind == ElementKind::Component
    }

    /// Whether the element renders a DOM element: a native tag or
    /// `<svelte:element>`.
    pub fn is_dom_element(&self) -> bool {
        matches!(
            self.row().kind,
            ElementKind::Dom | ElementKind::DynamicElement
        )
    }

    /// The reference from the tag name to the component, for a component
    /// tag. For `<ui.Button>`, the reference is to `ui`, with the path
    /// `[Button]`.
    pub fn tag_reference(&self) -> Option<Reference> {
        self.row().tag_ref.map(|id| handle!(self, Reference, id))
    }

    pub fn is_self_closing(&self) -> bool {
        self.row().self_closing
    }

    pub fn attrs(&self) -> impl Iterator<Item = Attr> + '_ {
        self.row()
            .attrs
            .clone()
            .map(|index| handle!(self, Attr, AttrId(index)))
    }

    /// Returns the plain or shorthand attribute named `name`.
    pub fn attr(&self, name: &str) -> Option<Attr> {
        self.attrs().find(|attr| attr.name() == Some(name))
    }

    /// Returns the first directive of the given kind and name.
    pub fn directive(&self, kind: DirectiveKind, name: &str) -> Option<Attr> {
        self.attrs()
            .find(|attr| attr.directive_kind() == Some(kind) && attr.directive_name() == Some(name))
    }

    /// Whether a spread attribute may set attributes the model cannot see.
    pub fn has_spread(&self) -> bool {
        self.attrs()
            .any(|attr| matches!(attr.row().name, AttrName::Spread))
    }

    /// The scope the element is written in.
    pub fn scope(&self) -> Scope {
        handle!(self, Scope, self.row().scope)
    }

    /// How many times one component instance can render the element.
    pub fn occurrence(&self) -> Occurrence {
        self.row().occurrence
    }
}

/// The value of an attribute or directive.
#[derive(Clone, Debug, PartialEq)]
pub enum AttrValue {
    /// No value is written.
    Absent,
    /// A quoted string without expressions.
    Static(Box<str>),
    /// A single expression. `None` when no snippet was found for it.
    Expr(Option<Snippet>),
    /// A quoted string with expressions, as `class="a {b}"`.
    Interpolated(Vec<ValuePart>),
    /// The getter and the setter of `bind:value={get, set}`.
    FunctionBinding {
        get: Option<Snippet>,
        set: Option<Snippet>,
    },
}

/// A part of a quoted attribute value with expressions.
#[derive(Clone, Debug, PartialEq)]
pub enum ValuePart {
    Text(Box<str>),
    Expr(Option<Snippet>),
}

impl Attr {
    /// The range of the attribute in the host document.
    pub fn range(&self) -> TextRange {
        self.row().range
    }

    /// The node of the attribute or directive, the one [`Self::range`]
    /// covers.
    pub fn html_node(&self) -> HtmlNodeId {
        self.row().node
    }

    pub fn element(&self) -> Element {
        handle!(self, Element, self.row().element)
    }

    /// The name of a plain or shorthand attribute.
    pub fn name(&self) -> Option<&str> {
        match &self.row().name {
            AttrName::Plain(name) | AttrName::Shorthand(name) => Some(name.text()),
            _ => None,
        }
    }

    /// Whether the attribute is written `{name}`.
    pub fn is_shorthand(&self) -> bool {
        matches!(self.row().name, AttrName::Shorthand(_))
    }

    pub fn is_spread(&self) -> bool {
        matches!(self.row().name, AttrName::Spread)
    }

    pub fn is_attachment(&self) -> bool {
        matches!(self.row().name, AttrName::Attach)
    }

    pub fn is_directive(&self) -> bool {
        matches!(self.row().name, AttrName::Directive { .. })
    }

    pub fn directive_kind(&self) -> Option<DirectiveKind> {
        match &self.row().name {
            AttrName::Directive { kind, .. } => Some(*kind),
            _ => None,
        }
    }

    /// The name after the colon of a directive, as `value` in `bind:value`.
    pub fn directive_name(&self) -> Option<&str> {
        match &self.row().name {
            AttrName::Directive { name, .. } => Some(name.text()),
            _ => None,
        }
    }

    pub fn modifiers(&self) -> impl Iterator<Item = &str> + '_ {
        let modifiers: &[TokenText] = match &self.row().name {
            AttrName::Directive { modifiers, .. } => modifiers,
            _ => &[],
        };
        modifiers.iter().map(TokenText::text)
    }

    /// Whether the attribute is an event handler: an attribute whose name
    /// starts with `on` and whose value is an expression, as `onclick={go}` or
    /// `{onclick}`, on an element that is not a component. On a component,
    /// such an attribute is a prop. The legacy `on:click` directive is not
    /// one.
    pub fn is_event_handler(&self) -> bool {
        self.name()
            .is_some_and(|name| name.starts_with("on") && !name.contains(':'))
            && matches!(self.row().value, AttrValueData::Expr(_))
            && !self.element().is_component()
    }

    /// The reference from the name of a directive to a function in scope,
    /// as `fade` in `transition:fade`.
    pub fn directive_reference(&self) -> Option<Reference> {
        match &self.row().name {
            AttrName::Directive {
                name_ref: Some(id), ..
            } => Some(handle!(self, Reference, *id)),
            _ => None,
        }
    }

    /// The reference a directive without a value makes through its name, as
    /// `value` in `bind:value`.
    pub fn implicit_reference(&self) -> Option<Reference> {
        self.row()
            .implicit_ref
            .map(|id| handle!(self, Reference, id))
    }

    pub fn value(&self) -> AttrValue {
        let snippet = |id: &Option<SnippetId>| id.map(|id| handle!(self, Snippet, id));
        match &self.row().value {
            AttrValueData::Absent => AttrValue::Absent,
            AttrValueData::Static(text) => AttrValue::Static(text.text().into()),
            AttrValueData::Expr(id) => AttrValue::Expr(snippet(id)),
            AttrValueData::Interpolated(parts) => AttrValue::Interpolated(
                parts
                    .iter()
                    .map(|part| match part {
                        ValuePartData::Text(text) => ValuePart::Text(text.text().into()),
                        ValuePartData::Expr(id) => ValuePart::Expr(snippet(id)),
                    })
                    .collect(),
            ),
            AttrValueData::FunctionBinding { get, set } => AttrValue::FunctionBinding {
                get: snippet(get),
                set: snippet(set),
            },
        }
    }

    /// The snippet holding the value, when the value is a single expression.
    pub fn value_snippet(&self) -> Option<Snippet> {
        match self.value() {
            AttrValue::Expr(snippet) => snippet,
            _ => None,
        }
    }

    /// The value, when it is a string the model knows: a quoted string
    /// without expressions, or an expression whose value is a string.
    pub fn static_string(&self) -> Option<Box<str>> {
        match self.value() {
            AttrValue::Static(text) => Some(text),
            AttrValue::Expr(Some(snippet)) => match snippet.facts().value() {
                Some(StaticValue::String(text)) => Some(text.clone()),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Block {
    pub fn kind(&self) -> BlockKind {
        self.row().kind
    }

    /// The range of the block in the host document.
    pub fn range(&self) -> TextRange {
        self.row().range
    }

    /// The node of the block, the one [`Self::range`] covers.
    pub fn html_node(&self) -> HtmlNodeId {
        self.row().node
    }

    /// The element the block is written in.
    pub fn parent(&self) -> Option<Element> {
        self.row().parent.map(|id| handle!(self, Element, id))
    }

    /// The expression the block opens with: the condition of `{#if}`, the
    /// list of `{#each}`, the promise of `{#await}`, the expression of
    /// `{#key}`, or the signature of `{#snippet}`.
    pub fn expression(&self) -> Option<Snippet> {
        self.row().expression.map(|id| handle!(self, Snippet, id))
    }

    /// The key of `{#each}`.
    pub fn key(&self) -> Option<Snippet> {
        self.row().key.map(|id| handle!(self, Snippet, id))
    }

    pub fn branches(&self) -> impl Iterator<Item = Branch> + '_ {
        self.row()
            .branches
            .clone()
            .map(|index| handle!(self, Branch, BranchId(index)))
    }

    /// The variables the block declares for its content: the item and index
    /// of `{#each}`, the value and error of `{#await}`, the parameters of
    /// `{#snippet}`.
    pub fn declarations(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.branches()
            .flat_map(|branch| branch.scope().symbols().collect::<Vec<_>>())
            .filter(|symbol| {
                matches!(
                    symbol.kind(),
                    SymbolKind::EachItem
                        | SymbolKind::EachIndex
                        | SymbolKind::AwaitValue
                        | SymbolKind::AwaitError
                        | SymbolKind::SnippetParam
                )
            })
    }
}

impl Branch {
    pub fn kind(&self) -> BranchKind {
        self.row().kind
    }

    pub fn block(&self) -> Block {
        handle!(self, Block, self.row().block)
    }

    /// The range of the branch's content in the host document.
    pub fn content_range(&self) -> TextRange {
        self.row().range
    }

    /// The condition of `{#if}` and `{:else if}`.
    pub fn condition(&self) -> Option<Snippet> {
        self.row().condition.map(|id| handle!(self, Snippet, id))
    }

    /// The scope of the branch's content.
    pub fn scope(&self) -> Scope {
        handle!(self, Scope, self.row().scope)
    }
}

impl Snippet {
    /// The range of the snippet's content in the host document.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    /// The range of the snippet's content, in the coordinates of its tree.
    pub fn range(&self) -> TextRange {
        self.data.local(self.row().range, Some(self.id))
    }

    /// The position in the host document of the start of the snippet's tree.
    pub fn offset(&self) -> TextSize {
        self.row().offset
    }

    pub fn host(&self) -> SnippetHost {
        self.row().host
    }

    /// The attribute or directive whose value this snippet is, or is part of.
    pub fn attr(&self) -> Option<Attr> {
        match self.row().host {
            SnippetHost::AttrValue(id)
            | SnippetHost::AttrPart(id, _)
            | SnippetHost::BindGetter(id)
            | SnippetHost::BindSetter(id) => Some(handle!(self, Attr, id)),
            _ => None,
        }
    }

    /// The script whose content this snippet is.
    pub fn script(&self) -> Option<Script> {
        match self.row().host {
            SnippetHost::Script(id) => Some(handle!(self, Script, id)),
            _ => None,
        }
    }

    pub fn root(&self) -> &RootShape {
        &self.row().root
    }

    pub fn assignability(&self) -> Assignability {
        self.row().assignability
    }

    /// A hash of the snippet's tokens, ignoring whitespace and comments.
    ///
    /// Two snippets with equal fingerprints hold the same expression, up to
    /// hash collisions.
    pub fn fingerprint(&self) -> u64 {
        self.row().fingerprint
    }

    /// Whether the snippet contains a comment.
    pub fn has_comments(&self) -> bool {
        self.row().has_comments
    }

    /// The operands of the snippet's root, when it is a `||` or `&&`
    /// expression.
    pub fn operands(&self) -> &[Operand] {
        let range = &self.row().operands;
        &self.data.operands[range.start as usize..range.end as usize]
    }

    /// What the model knows about the value of the snippet's expression.
    pub fn facts(&self) -> &ValueFacts {
        &self.row().facts
    }

    /// Whether the snippet is written inside a `{#snippet}` block.
    pub fn is_in_snippet_block(&self) -> bool {
        self.row().in_snippet_block
    }

    /// The scope the free names of the snippet are looked up in.
    pub fn scope(&self) -> Option<Scope> {
        self.row().scope.map(|id| handle!(self, Scope, id))
    }

    /// The references written inside this snippet.
    pub fn references(&self) -> impl Iterator<Item = Reference> + '_ {
        self.data
            .references_by_snippet
            .get(self.id.index() + 1)
            .into_iter()
            .flatten()
            .map(|id| handle!(self, Reference, *id))
    }
}

impl Scope {
    pub fn kind(&self) -> ScopeKind {
        self.row().kind
    }

    pub fn parent(&self) -> Option<Self> {
        self.row().parent.map(|id| handle!(self, Scope, id))
    }

    /// The declarations made directly in this scope.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.row()
            .symbols
            .iter()
            .map(|id| handle!(self, Symbol, *id))
    }

    /// This scope followed by each enclosing scope, innermost first.
    pub fn ancestors(&self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(self.clone()), Self::parent)
    }

    /// Looks `name` up in this scope and its enclosing scopes.
    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        self.ancestors()
            .find_map(|scope| scope.symbols().find(|symbol| symbol.name() == name))
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
    /// host document, such as a variable of `{#each}`, or in a module.
    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| handle!(self, Snippet, id))
    }

    pub fn kind(&self) -> SymbolKind {
        self.row().kind
    }

    pub fn declaration_kind(&self) -> DeclarationKind {
        self.row().declaration_kind
    }

    pub fn scope(&self) -> Scope {
        handle!(self, Scope, self.row().scope)
    }

    /// Whether the symbol only exists in the type system.
    pub fn is_type_only(&self) -> bool {
        self.row().type_only
    }

    /// Whether a script exports the symbol.
    pub fn is_exported(&self) -> bool {
        self.row().exported
    }

    /// The module an imported symbol comes from, such as `svelte/store`.
    pub fn import_source(&self) -> Option<&str> {
        self.row()
            .import
            .as_ref()
            .map(|import| import.source.text())
    }

    /// What an imported symbol stands for in the module it comes from.
    pub fn imported(&self) -> Option<&Imported> {
        self.row().import.as_ref().map(|import| &import.imported)
    }

    /// Whether the symbol is the export `name` of the module `source`,
    /// imported under any local name.
    pub fn is_import_of(&self, source: &str, name: &str) -> bool {
        self.row().import.as_ref().is_some_and(|import| {
            import.source.text() == source
                && matches!(&import.imported, Imported::Named(imported) if imported.text() == name)
        })
    }

    /// The call of the rune that initializes the symbol, as `$state(0)` for
    /// `count` in `let count = $state(0)`.
    pub fn rune_call(&self) -> Option<RuneCall> {
        self.row().rune.map(|id| handle!(self, RuneCall, id))
    }

    /// The rune that initializes the symbol.
    pub fn rune(&self) -> Option<Rune> {
        self.rune_call().map(|call| call.rune())
    }

    /// The function of `svelte/store` that creates the store the symbol is
    /// initialized with, as for `const count = writable(0)`.
    pub fn store_creator(&self) -> Option<StoreCreator> {
        self.row().store
    }

    /// For a variable of `{#each}`, the variable holding the list.
    pub fn each_source(&self) -> Option<Self> {
        self.row().each_source.map(|id| handle!(self, Symbol, id))
    }

    /// What is known of the value of a `const` from its initializer.
    pub fn facts(&self) -> &ValueFacts {
        &self.row().facts
    }

    /// The node of the whole declaration, the one [`Self::declaration_range`]
    /// covers, for a declaration written in JavaScript.
    ///
    /// The node is in the tree [`Self::snippet`] names.
    pub fn js_node(&self) -> Option<JsNodeId> {
        self.row().node
    }

    /// The references that resolve to this symbol, in the scripts and in the
    /// template.
    pub fn uses(&self) -> impl Iterator<Item = Reference> + '_ {
        self.data.uses[self.id.index()]
            .iter()
            .map(|id| handle!(self, Reference, *id))
    }

    /// Whether any use assigns a new value to the variable, as opposed to
    /// changing one of its properties or setting the value of the store it
    /// holds.
    pub fn is_reassigned(&self) -> bool {
        self.uses().any(|reference| reference.is_reassignment())
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
        self.row().snippet.map(|id| handle!(self, Snippet, id))
    }

    pub fn site(&self) -> RefSite {
        self.row().site
    }

    /// The scope the name was looked up in, for a reference written in the
    /// template.
    pub fn scope(&self) -> Option<Scope> {
        self.row().scope.map(|id| handle!(self, Scope, id))
    }

    /// For a use of the list of `{#each}` made through one of its variables,
    /// that variable.
    pub fn via(&self) -> Option<Symbol> {
        self.row().via.map(|id| handle!(self, Symbol, id))
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

    /// Whether the reference assigns a new value to the variable itself.
    pub fn is_reassignment(&self) -> bool {
        let row = self.row();
        row.access == Access::Write
            && row.path.is_empty()
            && !row.path_open
            && !row.store_subscription
    }

    /// The number of arguments, when the reference is called.
    pub fn call_args(&self) -> Option<u8> {
        self.row().call_args
    }

    pub fn resolution(&self) -> Resolution {
        self.row().resolution
    }

    /// The symbol the reference resolves to.
    pub fn symbol(&self) -> Option<Symbol> {
        match self.row().resolution {
            Resolution::Symbol(id) => Some(handle!(self, Symbol, id)),
            _ => None,
        }
    }

    /// Whether the reference reads or sets the value of a store, as `$count`
    /// does for the store `count`.
    pub fn is_store_subscription(&self) -> bool {
        self.row().store_subscription
    }

    /// Whether the reference is written inside a function, in its tree.
    pub fn is_in_function(&self) -> bool {
        self.row().in_function
    }

    /// Whether the reference is written inside a `{#snippet}` block.
    pub fn is_in_snippet_block(&self) -> bool {
        self.row().in_snippet_block
    }

    /// Whether the reference runs when the component or the module is set
    /// up: it is neither inside a function nor inside a `{#snippet}` block.
    pub fn is_top_level(&self) -> bool {
        !self.row().in_function && !self.row().in_snippet_block
    }

    /// Whether the reference is written in a type position.
    pub fn is_type_only(&self) -> bool {
        self.row().type_only
    }

    /// The module and the export the reference reads: `svelte/store` and
    /// `writable` for `writable` imported from `svelte/store`, and for
    /// `store.writable` with `store` imported as a namespace.
    pub fn imported_member(&self) -> Option<(&str, &str)> {
        let symbol = &self.data.symbols[match self.row().resolution {
            Resolution::Symbol(id) => id.index(),
            _ => return None,
        }];
        let import = symbol.import.as_ref()?;
        match &import.imported {
            Imported::Named(name) => Some((import.source.text(), name.text())),
            Imported::Namespace => Some((import.source.text(), self.row().path.first()?.text())),
            Imported::Default => None,
        }
    }
}

impl RuneCall {
    pub fn rune(&self) -> Rune {
        self.row().rune
    }

    /// The range of the call, in the coordinates of its tree.
    pub fn range(&self) -> TextRange {
        self.data.local(self.row().range, self.row().snippet)
    }

    /// The range of the call in the host document.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    /// The range of the callee, such as `$derived.by`, in the coordinates of
    /// its tree.
    pub fn callee_range(&self) -> TextRange {
        self.data.local(self.row().callee, self.row().snippet)
    }

    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| handle!(self, Snippet, id))
    }

    pub fn target(&self) -> &RuneTarget {
        &self.row().target
    }

    /// The symbols the call initializes.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.row()
            .symbols
            .iter()
            .map(|id| handle!(self, Symbol, *id))
    }

    /// The number of arguments.
    pub fn args(&self) -> u8 {
        self.row().args
    }

    /// The node of the call. It casts to
    /// [`JsCallExpression`](biome_js_syntax::JsCallExpression).
    pub fn js_node(&self) -> JsNodeId {
        self.row().node
    }
}

impl Prop {
    /// The name of the prop.
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

    /// The variable the destructuring pattern of `$props()` binds the prop
    /// to.
    pub fn local(&self) -> Option<Symbol> {
        self.row().local.map(|id| handle!(self, Symbol, id))
    }

    /// The range of the default value, in the coordinates of the tree that
    /// declares the prop.
    pub fn default_range(&self) -> Option<TextRange> {
        let range = self.row().default?;
        Some(self.data.local(range, self.row().snippet))
    }

    /// Whether the default value is a `$bindable()` call.
    pub fn is_bindable(&self) -> bool {
        self.row().bindable
    }

    /// Whether the type of the props declares the prop.
    pub fn is_in_type(&self) -> bool {
        self.row().in_type
    }

    /// The uses of the prop: the uses of its variable, and the members named
    /// after it of the props object or of the rest of the props, as
    /// `props.title` for `let props = $props()`.
    pub fn uses(&self) -> impl Iterator<Item = Reference> + '_ {
        let objects = self
            .data
            .props_declaration
            .as_ref()
            .map_or_default(|props| [props.object, props.rest]);
        let local = self.row().local;
        let name = self.row().name.clone();
        local
            .into_iter()
            .chain(objects.into_iter().flatten())
            .flat_map(|id| self.data.uses[id.index()].clone())
            .map(|id| handle!(self, Reference, id))
            .filter(move |reference| {
                let symbol = reference.symbol().map(|symbol| symbol.id);
                (local.is_some() && symbol == local) || reference.path().next() == Some(name.text())
            })
    }
}

/// The `$props()` declaration of a component.
#[derive(Clone, Copy, Debug)]
pub struct PropsDeclaration<'a> {
    model: &'a SvelteSemanticModel,
    data: &'a PropsData,
}

impl PropsDeclaration<'_> {
    pub fn call(&self) -> RuneCall {
        handle!(self.model, RuneCall, self.data.call)
    }

    /// The variable holding all the props, for `let props = $props()`.
    pub fn object(&self) -> Option<Symbol> {
        self.data.object.map(|id| handle!(self.model, Symbol, id))
    }

    /// The variable holding the props the pattern doesn't name, for
    /// `let { a, ...rest } = $props()`.
    pub fn rest(&self) -> Option<Symbol> {
        self.data.rest.map(|id| handle!(self.model, Symbol, id))
    }

    /// Whether the declaration has a type annotation.
    pub fn has_type(&self) -> bool {
        self.data.has_type
    }

    /// Whether [`SvelteSemanticModel::props`] lists every prop the component
    /// accepts.
    ///
    /// It does not when the destructuring pattern has a member the model
    /// cannot name, or when the type of the props refers to a type the model
    /// cannot read, such as an imported one, or has an index signature.
    /// Without a type, it does not when the props are read as a whole, as in
    /// `let props = $props()` or `let { a, ...rest } = $props()`, which
    /// accept any prop.
    pub fn is_complete(&self) -> bool {
        if self.data.pattern_open {
            return false;
        }
        if self.data.has_type {
            !self.data.type_open
        } else {
            self.data.object.is_none() && self.data.rest.is_none()
        }
    }
}

impl Export {
    /// The name the export is visible as.
    pub fn name(&self) -> &str {
        self.row().name.text()
    }

    /// The range of the export, in the coordinates of its tree.
    pub fn range(&self) -> TextRange {
        self.data.local(self.row().range, self.row().snippet)
    }

    /// The range of the export in the host document.
    pub fn host_range(&self) -> TextRange {
        self.row().range
    }

    pub fn snippet(&self) -> Option<Snippet> {
        self.row().snippet.map(|id| handle!(self, Snippet, id))
    }

    /// The symbol the export makes visible, when it is declared in the same
    /// script.
    pub fn local(&self) -> Option<Symbol> {
        self.row().local.map(|id| handle!(self, Symbol, id))
    }

    /// Whether the export makes a binding of another module visible, as in
    /// `export { a } from './a'`.
    pub fn is_reexport(&self) -> bool {
        self.row().reexport
    }

    /// Whether the export only exists in the type system.
    pub fn is_type_only(&self) -> bool {
        self.row().type_only
    }
}
