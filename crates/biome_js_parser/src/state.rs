use crate::prelude::*;
use biome_js_syntax::JsFileSource;
use biome_rowan::{TextRange, TextSize};
use enumflags2::{bitflags, make_bitflags, BitFlags};
use indexmap::IndexMap;
use rustc_hash::FxHashSet;
use std::ops::{BitOr, BitOrAssign, Deref, DerefMut, Range, Sub};

type LabelSet = IndexMap<String, LabelledItem>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum LabelledItem {
    Iteration(TextRange),
    Other(TextRange),
}

impl LabelledItem {
    pub(crate) fn range(&self) -> &TextRange {
        match self {
            LabelledItem::Iteration(range) | LabelledItem::Other(range) => range,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ExportDefaultItemKind {
    Expression,
    FunctionOverload,
    FunctionDeclaration,
    Interface,
    ClassDeclaration,
    // Any other declaration
    Declaration,
}
impl ExportDefaultItemKind {
    pub(crate) fn is_interface(&self) -> bool {
        matches!(self, ExportDefaultItemKind::Interface)
    }

    pub const fn is_mergeable(&self, other: &ExportDefaultItemKind) -> bool {
        Self::can_merge(self, other) || Self::can_merge(other, self)
    }
    const fn can_merge(a: &ExportDefaultItemKind, b: &ExportDefaultItemKind) -> bool {
        match (a, b) {
            // test ts decorator_export_default_function_and_function_overload
            // export default function a():void;
            // export default function a(v: number):void;
            // export default function a(v?: any){
            // }
            (
                ExportDefaultItemKind::FunctionOverload,
                ExportDefaultItemKind::FunctionDeclaration
                | ExportDefaultItemKind::FunctionOverload,
            ) => true,
            // test ts decorator_export_default_function_and_interface
            // export default interface A{};
            // export default interface A{};
            // export default function a(){};
            // export default interface A{};
            // export default interface A{};
            //
            // test ts decorator_export_default_class_and_interface
            // export default interface A{};
            // export default interface A{};
            // export default class A{};
            // export default interface A{};
            // export default interface A{};
            (
                ExportDefaultItemKind::Interface,
                ExportDefaultItemKind::ClassDeclaration
                | ExportDefaultItemKind::FunctionDeclaration
                | ExportDefaultItemKind::Interface,
            ) => true,
            (_, _) => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct ExportDefaultItem {
    pub kind: ExportDefaultItemKind,
    pub range: Range<usize>,
}

/// State kept by the parser while parsing.
/// It is required for things such as strict mode or async functions
#[derive(Debug)]
pub(crate) struct JsParserState {
    parsing_context: ParsingContextFlags,
    /// A list of labels for labelled statements used to report undefined label errors
    /// for break and continue, as well as duplicate labels.
    /// Often called label set in the spec.
    label_set: LabelSet,
    /// Whether we are in strict mode code
    strict: Option<StrictMode>,

    /// The exported default item, used for checking duplicate defaults
    pub default_item: Option<ExportDefaultItem>,
    /// If set, the parser reports bindings with identical names. The option stores the name of the
    /// node that disallows duplicate bindings, for example `let`, `const` or `import`.
    pub duplicate_binding_parent: Option<&'static str>,
    pub name_map: IndexMap<String, TextRange>,

    /// Indicates that the parser is speculatively parsing a syntax. Speculative parsing means that the
    /// parser tries to parse a syntax as one kind and determines at the end if the assumption was right
    /// by testing if the parser is at a specific token (or has no errors). For this approach to work,
    /// the parser isn't allowed to skip any tokens while doing error recovery because it may then successfully
    /// skip over all invalid tokens, so that it appears as if it was able to parse the syntax correctly.
    ///
    /// Speculative parsing is useful if a syntax is ambiguous and no amount of lookahead (except parsing the whole syntax)
    /// is sufficient to determine what syntax it is. For example, the syntax `(a, b) ...`
    /// in JavaScript is either a parenthesized expression or an arrow expression if `...` is a `=>`.
    /// The challenge is, that it isn't possible to tell which of the two kinds it is until the parser
    /// processed all of `(a, b)`.
    pub(crate) speculative_parsing: bool,

    /// Stores the token positions of all syntax that looks like an arrow expressions but aren't one.
    /// Optimization to reduce the back-tracking required when parsing parenthesized and arrow function expressions.
    pub(crate) not_parenthesized_arrow: FxHashSet<TextSize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StrictMode {
    Module,
    Explicit(TextRange),
    Class(TextRange),
}

impl JsParserState {
    pub fn new(source_type: &JsFileSource) -> Self {
        let mut state = JsParserState {
            parsing_context: ParsingContextFlags::TOP_LEVEL,
            label_set: IndexMap::new(),
            strict: source_type
                .module_kind()
                .is_module()
                .then_some(StrictMode::Module),
            default_item: None,
            name_map: IndexMap::new(),
            duplicate_binding_parent: None,
            not_parenthesized_arrow: Default::default(),
            speculative_parsing: false,
        };

        if source_type.module_kind().is_module() {
            state.parsing_context |= ParsingContextFlags::IN_ASYNC
        }

        // test d.ts arguments_in_definition_file
        // function a(...arguments: any[]): void;
        if source_type.language().is_definition_file() {
            EnterAmbientContext.apply(&mut state);
        }

        state
    }

    pub fn in_function(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_FUNCTION)
    }

    pub fn in_generator(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_GENERATOR)
    }

    pub fn in_async(&self) -> bool {
        self.parsing_context.contains(ParsingContextFlags::IN_ASYNC)
    }

    pub fn in_ambient_context(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::AMBIENT_CONTEXT)
    }

    pub fn in_constructor(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::IN_CONSTRUCTOR)
    }

    pub fn is_top_level(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::TOP_LEVEL)
    }

    pub fn continue_allowed(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::CONTINUE_ALLOWED)
    }
    pub fn break_allowed(&self) -> bool {
        self.parsing_context
            .contains(ParsingContextFlags::BREAK_ALLOWED)
    }

    pub fn strict(&self) -> Option<&StrictMode> {
        self.strict.as_ref()
    }

    pub fn get_labelled_item(&self, label: &str) -> Option<&LabelledItem> {
        self.label_set.get(label)
    }

    pub(super) fn checkpoint(&self) -> JsParserStateCheckpoint {
        JsParserStateCheckpoint::snapshot(self)
    }

    pub(super) fn restore(&mut self, checkpoint: JsParserStateCheckpoint) {
        checkpoint.rewind(self);
    }
}

/// Stores a checkpoint of the [JsParserState].
/// Allows rewinding the state to its previous state.
///
/// It's important that creating and rewinding a snapshot is cheap. Consider the performance implications
/// before adding new unscoped state.
#[derive(Debug)]
pub(super) struct JsParserStateCheckpoint {
    /// Additional data that we only want to store in debug mode
    #[cfg(debug_assertions)]
    debug_checkpoint: JsDebugParserStateCheckpoint,
}

impl JsParserStateCheckpoint {
    /// Creates a snapshot of the passed in state.
    #[cfg(debug_assertions)]
    fn snapshot(state: &JsParserState) -> Self {
        Self {
            debug_checkpoint: JsDebugParserStateCheckpoint::snapshot(state),
        }
    }

    #[cfg(not(debug_assertions))]
    fn snapshot(_: &JsParserState) -> Self {
        Self {}
    }

    /// Restores the `state values` to the time when this snapshot was created.
    #[cfg(debug_assertions)]
    fn rewind(self, state: &mut JsParserState) {
        self.debug_checkpoint.rewind(state);
    }

    #[cfg(not(debug_assertions))]
    fn rewind(self, _: &JsParserState) {}
}

/// Most of the [JsParserState] is scoped state. It should, therefore, not be necessary to rewind
/// that state because that's already taken care of by `with_state` and `with_scoped_state`.
/// But, you can never no and better be safe than sorry. That's why we use some heuristics
/// to verify that non of the scoped state did change and assert for it when rewinding.
#[derive(Debug, Clone)]
#[cfg(debug_assertions)]
pub(super) struct JsDebugParserStateCheckpoint {
    parsing_context: ParsingContextFlags,
    label_set_len: usize,
    strict: Option<StrictMode>,
    default_item: Option<ExportDefaultItem>,
    duplicate_binding_parent: Option<&'static str>,
    name_map_len: usize,
}

#[cfg(debug_assertions)]
impl JsDebugParserStateCheckpoint {
    fn snapshot(state: &JsParserState) -> Self {
        Self {
            parsing_context: state.parsing_context,
            label_set_len: state.label_set.len(),
            strict: state.strict.clone(),
            default_item: state.default_item.clone(),
            duplicate_binding_parent: state.duplicate_binding_parent,
            name_map_len: state.name_map.len(),
        }
    }

    fn rewind(self, state: &mut JsParserState) {
        assert_eq!(state.parsing_context, self.parsing_context);
        assert_eq!(state.label_set.len(), self.label_set_len);
        assert_eq!(state.strict, self.strict);
        assert_eq!(state.default_item, self.default_item);
        assert_eq!(
            state.duplicate_binding_parent,
            self.duplicate_binding_parent
        );
        assert_eq!(state.name_map.len(), self.name_map_len);
    }
}

/// Reverts state changes to their previous value when it goes out of scope.
/// Can be used like a regular parser.
pub(crate) struct ParserStateGuard<'parser, 't, C>
where
    C: ChangeParserState,
{
    snapshot: C::Snapshot,
    inner: &'parser mut JsParser<'t>,
}

impl<'parser, 't, C: ChangeParserState> ParserStateGuard<'parser, 't, C> {
    pub(super) fn new(parser: &'parser mut JsParser<'t>, snapshot: C::Snapshot) -> Self {
        Self {
            snapshot,
            inner: parser,
        }
    }
}

impl<C: ChangeParserState> Drop for ParserStateGuard<'_, '_, C> {
    fn drop(&mut self) {
        let snapshot = std::mem::take(&mut self.snapshot);

        C::restore(self.inner.state_mut(), snapshot);
    }
}

impl<'t, C: ChangeParserState> Deref for ParserStateGuard<'_, 't, C> {
    type Target = JsParser<'t>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<C: ChangeParserState> DerefMut for ParserStateGuard<'_, '_, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

/// Implements a specific modification to the parser state that can later be reverted.
pub(crate) trait ChangeParserState {
    type Snapshot: Default;

    /// Applies the change to the passed in state and returns snapshot that allows restoring the previous state.
    fn apply(self, state: &mut JsParserState) -> Self::Snapshot;

    /// Restores the state to its previous value
    fn restore(state: &mut JsParserState, value: Self::Snapshot);
}

#[derive(Default, Debug)]
pub struct EnableStrictModeSnapshot(Option<StrictMode>);

/// Enables strict mode
pub(crate) struct EnableStrictMode(pub StrictMode);

impl ChangeParserState for EnableStrictMode {
    type Snapshot = EnableStrictModeSnapshot;

    #[inline]
    fn apply(self, state: &mut JsParserState) -> Self::Snapshot {
        EnableStrictModeSnapshot(std::mem::replace(&mut state.strict, Some(self.0)))
    }

    #[inline]
    fn restore(state: &mut JsParserState, value: Self::Snapshot) {
        state.strict = value.0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum SignatureFlag {
    Async = 1 << 0,
    Generator = 1 << 1,
    Constructor = 1 << 2,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub(crate) struct SignatureFlags(BitFlags<SignatureFlag>);

impl SignatureFlags {
    /// Is the function in an async context
    pub const ASYNC: Self = Self(make_bitflags!(SignatureFlag::{Async}));
    /// Is the function in a generator context
    pub const GENERATOR: Self = Self(make_bitflags!(SignatureFlag::{Generator}));
    /// Is the function a constructor (or constructor context)
    pub const CONSTRUCTOR: Self = Self(make_bitflags!(SignatureFlag::{Constructor}));

    pub const fn empty() -> Self {
        Self(BitFlags::EMPTY)
    }

    pub fn contains(&self, other: impl Into<SignatureFlags>) -> bool {
        self.0.contains(other.into().0)
    }
}

impl From<SignatureFlags> for ParsingContextFlags {
    fn from(flags: SignatureFlags) -> Self {
        let mut parsing_context = ParsingContextFlags::empty();

        if flags.contains(SignatureFlags::ASYNC) {
            parsing_context |= ParsingContextFlags::IN_ASYNC;
        }

        if flags.contains(SignatureFlags::GENERATOR) {
            parsing_context |= ParsingContextFlags::IN_GENERATOR;
        }

        if flags.contains(SignatureFlags::CONSTRUCTOR) {
            parsing_context |= ParsingContextFlags::IN_CONSTRUCTOR;
        }

        parsing_context
    }
}

impl BitOr for SignatureFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        SignatureFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for SignatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum ParsingContextFlag {
    InGenerator = 1 << 0,
    InFunction = 1 << 1,
    InConstructor = 1 << 2,
    InAsync = 1 << 3,
    TopLevel = 1 << 4,
    BreakAllowed = 1 << 5,
    ContinueAllowed = 1 << 6,
    AmbientContext = 1 << 7,
}

/// Flags representing the parsing state.
/// The reasons to use flags instead of individual boolean fields on `ParserState` are:
/// * It's possible to use bit masks to define what state should be inherited. For example,
///   functions inherit whether they're defined inside a parameter but override the `in_async` flag
/// * It's easier to snapshot the previous state. Individual boolean fields would require that a change
///   snapshots each individual boolean field to allow restoring the previous state. With bitflags, all that
///   is needed is to copy away the flags field and restore it after.
#[derive(Debug, Copy, Default, Clone, Eq, PartialEq)]
pub(crate) struct ParsingContextFlags(BitFlags<ParsingContextFlag>);

impl ParsingContextFlags {
    /// Whether the parser is in a generator function like `function* a() {}`
    /// Matches the `Yield` parameter in the ECMA spec
    const IN_GENERATOR: Self = Self(make_bitflags!(ParsingContextFlag::{InGenerator}));
    /// Whether the parser is inside a function
    const IN_FUNCTION: Self = Self(make_bitflags!(ParsingContextFlag::{InFunction}));
    /// Whatever the parser is inside a constructor
    const IN_CONSTRUCTOR: Self = Self(make_bitflags!(ParsingContextFlag::{InConstructor}));

    /// Is async allowed in this context. Either because it's an async function or top level await is supported.
    /// Equivalent to the `Async` generator in the ECMA spec
    const IN_ASYNC: Self = Self(make_bitflags!(ParsingContextFlag::{InAsync}));

    /// Whether the parser is parsing a top-level statement (not inside a class, function, parameter) or not
    const TOP_LEVEL: Self = Self(make_bitflags!(ParsingContextFlag::{TopLevel}));

    /// Whether the parser is in an iteration or switch statement and
    /// `break` is allowed.
    const BREAK_ALLOWED: Self = Self(make_bitflags!(ParsingContextFlag::{BreakAllowed}));

    /// Whether the parser is in an iteration statement and `continue` is allowed.
    const CONTINUE_ALLOWED: Self = Self(make_bitflags!(ParsingContextFlag::{ContinueAllowed}));

    /// Whether the parser is in a TypeScript ambient context
    const AMBIENT_CONTEXT: Self = Self(make_bitflags!(ParsingContextFlag::{AmbientContext}));

    /// Bitmask of all the flags that must be reset (shouldn't be inherited) when the parser enters a function
    const FUNCTION_RESET_MASK: Self = Self(
        make_bitflags!(ParsingContextFlag::{BreakAllowed | ContinueAllowed | InConstructor | InAsync | InGenerator | TopLevel }),
    );

    /// Bitmask of all the flags that must be reset (shouldn't be inherited) when entering parameters.
    const PARAMETER_RESET_MASK: Self = Self(
        make_bitflags!(ParsingContextFlag::{InConstructor | InFunction | TopLevel | InGenerator | InAsync }),
    );

    pub const fn empty() -> Self {
        Self(BitFlags::EMPTY)
    }

    pub fn contains(&self, other: impl Into<ParsingContextFlags>) -> bool {
        self.0.contains(other.into().0)
    }
}

impl BitOr for ParsingContextFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ParsingContextFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for ParsingContextFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Sub for ParsingContextFlags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct ParsingContextFlagsSnapshot(ParsingContextFlags);

pub(crate) trait ChangeParserStateFlags {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags;
}

impl<T: ChangeParserStateFlags> ChangeParserState for T {
    type Snapshot = ParsingContextFlagsSnapshot;

    fn apply(self, state: &mut JsParserState) -> Self::Snapshot {
        let new_flags = self.compute_new_flags(state.parsing_context);
        ParsingContextFlagsSnapshot(std::mem::replace(&mut state.parsing_context, new_flags))
    }

    fn restore(state: &mut JsParserState, value: Self::Snapshot) {
        state.parsing_context = value.0
    }
}

/// Enters the parsing of function/method parameters
pub(crate) struct EnterParameters(
    /// Whether async and yield are reserved keywords
    pub(crate) SignatureFlags,
);

impl ChangeParserStateFlags for EnterParameters {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        (existing - ParsingContextFlags::PARAMETER_RESET_MASK) | ParsingContextFlags::from(self.0)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum BreakableKind {
    // Iteration statement like Do, While, For
    Iteration,

    // Switch statement
    Switch,
}

pub(crate) struct EnterBreakable(pub(crate) BreakableKind);

impl ChangeParserStateFlags for EnterBreakable {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        let mut flags = existing | ParsingContextFlags::BREAK_ALLOWED;

        if self.0 == BreakableKind::Iteration {
            flags |= ParsingContextFlags::CONTINUE_ALLOWED;
        }

        flags
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnterFunctionSnapshot {
    parsing_context: ParsingContextFlags,
    label_set: LabelSet,
}

/// Enters the parsing of a function/method. Resets the relevant parser state and sets the state
/// according to the passed [SignatureFlags]
pub(crate) struct EnterFunction(pub(crate) SignatureFlags);

impl ChangeParserState for EnterFunction {
    type Snapshot = EnterFunctionSnapshot;

    #[inline]
    fn apply(self, state: &mut JsParserState) -> Self::Snapshot {
        let new_flags = (state.parsing_context - ParsingContextFlags::FUNCTION_RESET_MASK)
            | ParsingContextFlags::IN_FUNCTION
            | ParsingContextFlags::from(self.0);

        EnterFunctionSnapshot {
            parsing_context: std::mem::replace(&mut state.parsing_context, new_flags),
            label_set: std::mem::take(&mut state.label_set),
        }
    }

    #[inline]
    fn restore(state: &mut JsParserState, value: Self::Snapshot) {
        state.parsing_context = value.parsing_context;
        state.label_set = value.label_set;
    }
}

pub(crate) struct EnterClassPropertyInitializer;

impl ChangeParserStateFlags for EnterClassPropertyInitializer {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        existing
            - ParsingContextFlags::TOP_LEVEL
            - ParsingContextFlags::IN_ASYNC
            - ParsingContextFlags::IN_GENERATOR
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct EnterClassStaticInitializationBlockSnapshot {
    label_set: LabelSet,
    flags: ParsingContextFlags,
}

pub(crate) struct EnterClassStaticInitializationBlock;

impl ChangeParserState for EnterClassStaticInitializationBlock {
    type Snapshot = EnterClassStaticInitializationBlockSnapshot;

    fn apply(self, state: &mut JsParserState) -> Self::Snapshot {
        let flags = (state.parsing_context
            - ParsingContextFlags::FUNCTION_RESET_MASK
            - ParsingContextFlags::IN_FUNCTION)
            | ParsingContextFlags::IN_ASYNC; // allow async for better error recovery
        EnterClassStaticInitializationBlockSnapshot {
            flags: std::mem::replace(&mut state.parsing_context, flags),
            label_set: std::mem::take(&mut state.label_set),
        }
    }

    fn restore(state: &mut JsParserState, value: Self::Snapshot) {
        state.parsing_context = value.flags;
        state.label_set = value.label_set;
    }
}

#[derive(Debug, Default)]
pub(crate) struct WithLabelSnapshot {
    #[cfg(debug_assertions)]
    label_set_len: usize,
}

/// Adds the labelled item with the given label to the `label_set`.
/// Removes the label when the change is undone.
pub(crate) struct WithLabel(pub String, pub LabelledItem);

impl ChangeParserState for WithLabel {
    type Snapshot = WithLabelSnapshot;

    fn apply(self, state: &mut JsParserState) -> Self::Snapshot {
        #[cfg(debug_assertions)]
        let previous_len = state.label_set.len();
        state.label_set.insert(self.0, self.1);
        WithLabelSnapshot {
            // Capturing the len is sufficient because:
            // * The labels are stored in an index map that uses insertion-order
            // * Labels are scoped and new labels are always appended to the end of the list
            #[cfg(debug_assertions)]
            label_set_len: previous_len,
        }
    }

    #[cfg(not(debug_assertions))]
    fn restore(state: &mut JsParserState, _: Self::Snapshot) {
        state.label_set.pop();
    }

    #[cfg(debug_assertions)]
    fn restore(state: &mut JsParserState, value: Self::Snapshot) {
        assert_eq!(state.label_set.len(), value.label_set_len + 1);
        state.label_set.pop();
    }
}

/// Sets the state changes needed when parsing a TS type declaration (async and await are not reserved identifiers)
pub(crate) struct EnterType;

impl ChangeParserStateFlags for EnterType {
    fn compute_new_flags(&self, existing: ParsingContextFlags) -> ParsingContextFlags {
        existing - ParsingContextFlags::IN_ASYNC - ParsingContextFlags::IN_GENERATOR
    }
}

#[derive(Default)]
pub(crate) struct EnterAmbientContextSnapshot {
    flags: ParsingContextFlags,
    default_item: Option<ExportDefaultItem>,
    strict_mode: Option<StrictMode>,
}

pub(crate) struct EnterAmbientContext;

impl ChangeParserState for EnterAmbientContext {
    type Snapshot = EnterAmbientContextSnapshot;

    fn apply(self, state: &mut JsParserState) -> Self::Snapshot {
        let new_flags = state.parsing_context | ParsingContextFlags::AMBIENT_CONTEXT;
        EnterAmbientContextSnapshot {
            flags: std::mem::replace(&mut state.parsing_context, new_flags),
            default_item: state.default_item.take(),
            strict_mode: state.strict.take(),
        }
    }

    fn restore(state: &mut JsParserState, value: Self::Snapshot) {
        state.parsing_context = value.flags;
        state.default_item = value.default_item;
        state.strict = value.strict_mode;
    }
}
