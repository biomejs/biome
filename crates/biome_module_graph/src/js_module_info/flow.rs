//! Syntax-only flow facts that let type inference narrow reads of variables
//! and their property paths.
//!
//! Collection records, for each execution root that tests or writes a
//! variable, a graph of possible paths whose conditions are already
//! decomposed into guards on subjects and whose writes record the values they
//! assign. It also records the incoming path of every read such a guard or
//! write can reach. Type inference later applies the guards
//! to resolved types without inspecting syntax again.
//!
//! A subject is what a runtime test checks and a read narrows: a variable, or
//! a property path that starts at a variable or at `this`, such as `o.v`,
//! `a.b.c`, `t[0]`, or `this.#cache`. Guards and reads refer to a subject by
//! its id in the root's subject table, so a test and a read of the same subject
//! match without comparing syntax. A destructuring pattern such as
//! `{ kind, v }` is a subject too. Tests of its elements narrow the pattern,
//! and each element's declared type at a read is its property on the narrowed
//! pattern.
//!
//! An execution root is a body analyzed independently, such as a module,
//! function, or method. A variable narrows in each root that reads it: the
//! root that declares it, and a root nested in that one, such as a function
//! that reads a module's variable or one of its imports. A root starts from
//! the variable's declared type, except in a closure that continues the flow
//! where it is created, as TypeScript's `checkIdentifier` decides: there the
//! variable starts from its type at that point, its capture point, which
//! [`FlowCapture`] records. Property paths and `this` never continue. Each
//! root has its own `this`, and paths on it narrow only where Biome types
//! `this` as TypeScript does, as [`subjects::this_can_narrow`] describes. A
//! property path narrows only when its variable or `this` does.
//!
//! A write to a subject in the root's own flow is a write point of the graph,
//! where the subject takes the type of the assigned value, reduced against
//! its declared type as in TypeScript, and the paths that extend it take
//! their declared types. A write that runs with the root but outside that
//! flow, as in a function called where it is defined or a class static
//! block, has no write point, so the subject it writes doesn't narrow
//! anywhere in the root. Calls never invalidate facts, as in TypeScript.
//! Writes in other roots don't either, because they run before the root
//! runs or while a call inside it runs.

mod builder;
mod expressions;
mod guards;
mod scanner;
mod subjects;
mod writes;

pub(crate) use builder::{AnyMemberDeclaration, CalleeReturn, declaration_return, member_return};
pub(crate) use expressions::AnyFlowExpression;
pub(crate) use guards::{FlowGuard, FlowTest, UnitValue, binding_unit_value, guard_signature};
pub(crate) use scanner::FlowRootScanner;

use biome_js_control_flow::{AnyJsControlFlowRoot, is_truthy_literal};
use biome_js_semantic::{
    Binding, HasDeclarationAstNode, JsDeclarationKind, Scope, ScopeId, SemanticModel,
};
use biome_js_syntax::{
    AnyJsClass, AnyJsClassMember, AnyJsExpression, AnyJsFunction, AnyJsLiteralExpression,
    AnyJsVariableDeclaration, AnyTsType, AnyTsVariableAnnotation, JsArrayBindingPattern,
    JsArrayBindingPatternElement, JsArrayBindingPatternElementList,
    JsArrayBindingPatternRestElement, JsBreakStatement, JsClassDeclaration,
    JsClassExportDefaultDeclaration, JsComputedMemberName, JsConstructorClassMember,
    JsContinueStatement, JsDecorator, JsDoWhileStatement, JsExpressionStatement, JsForInStatement,
    JsForOfStatement, JsForStatement, JsGetterClassMember, JsGetterObjectMember,
    JsIdentifierAssignment, JsIdentifierBinding, JsIdentifierExpression, JsIfStatement, JsLanguage,
    JsMethodClassMember, JsMethodObjectMember, JsObjectBindingPattern,
    JsObjectBindingPatternProperty, JsObjectBindingPatternPropertyList, JsObjectBindingPatternRest,
    JsObjectBindingPatternShorthandProperty, JsReturnStatement, JsSetterClassMember,
    JsSetterObjectMember, JsSwitchStatement, JsSyntaxKind, JsSyntaxNode, JsThisExpression,
    JsThrowStatement, JsTryFinallyStatement, JsTryStatement, JsVariableDeclarationClause,
    JsVariableKind, JsVariableStatement, JsWhileStatement, JsWithStatement, TsDeclareStatement,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_js_type_info::{
    DestructureField, RawTypeData, RawTypeId, TypeReference, TypeofExpression, TypeofKind,
    is_raw_narrowing_invariant, raw_may_be_object_union, raw_may_be_union, raw_may_have_literal,
    raw_may_narrow_by_instanceof,
};
use biome_rowan::{
    AstNode, AstPtr, AstSeparatedList, SyntaxKindSet, Text, TextRange, TextSize, WalkEvent,
    declare_node_union,
};
use builder::{AnyContinuingClosure, BuiltCapture, BuiltRoot, BuiltSwitch, build_root};
use guards::GuardFunctions;
use rustc_hash::{FxHashMap, FxHashSet};
use writes::{WrittenPaths, WrittenTargets};

/// Index of a node in [`RootFlow::nodes`].
///
/// Construction gives up on a root before its node count exceeds this type.
pub(crate) type FlowNodeId = u32;

/// Index of a subject in [`RootFlow::subjects`].
pub(crate) type FlowSubjectId = u32;

/// What a runtime test checks and a read narrows within one execution root.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum FlowSubject {
    /// The variable declared at this range.
    Binding(TextRange),
    /// The `this` of the execution root. A table stores it only for a root
    /// where paths on `this` can narrow, which are listed on
    /// [`subjects::this_can_narrow`].
    This,
    /// The property `key` of the subject `object`, as in `o.v` or `o["v"]`.
    ///
    /// A table stores `object` before its members, so `object` is always a
    /// smaller id than the member.
    Member {
        object: FlowSubjectId,
        key: PropertyKey,
    },
    /// The destructuring pattern at this range, such as `{ kind, v }` in
    /// `const { kind, v } = s`, which stands for the value it destructures.
    ///
    /// A runtime test of one of its elements narrows it as a test of the
    /// element's property narrows an object, and a read of another element
    /// takes that element's property on the narrowed pattern, as in
    /// TypeScript. Only [`FlowGuard::Property`] guards narrow it. No
    /// expression reads it, so [`RootFlow::pattern_elements`] maps the reads
    /// of its elements to it.
    Pattern(TextRange),
}

/// The property that a step of a subject's path reads.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum PropertyKey {
    /// A property named by an identifier with its escapes decoded, or by an
    /// element access key that spells the property's name. `o.v`, `o.\u0076`,
    /// and `o["v"]` read the same property, and so do `t[0]` and `t["0"]`.
    Name(Text),
    /// A private name, such as `#v` in `o.#v`, without its `#` and with its
    /// escapes decoded. It never matches a [`Self::Name`]. So `o["#v"]` reads
    /// the property named `#v`, a different subject from `o.#v`.
    Private(Text),
}

/// Returns the subject that the path of `subject` starts at, a
/// [`FlowSubject::Binding`], [`FlowSubject::This`], or
/// [`FlowSubject::Pattern`], given the `subjects` of its root, each at the
/// index of its id.
///
/// Returns `None` when `subjects` has no such subject.
fn subject_root(subjects: &[FlowSubject], mut subject: FlowSubjectId) -> Option<&FlowSubject> {
    loop {
        match subjects.get(subject as usize)? {
            root @ (FlowSubject::Binding(_) | FlowSubject::This | FlowSubject::Pattern(_)) => {
                return Some(root);
            }
            FlowSubject::Member { object, .. } => {
                if *object >= subject {
                    return None;
                }
                subject = *object;
            }
        }
    }
}

/// Classes and object literal methods and accessors. Biome types a `this`
/// inside them as an instance of the class or as the object, even in the
/// decorators and `extends` clause of a class and in the computed key of a
/// method or accessor, where `this` is the one of the code around them.
const OTHER_THIS_KINDS: SyntaxKindSet<JsLanguage> = AnyJsClass::KIND_SET
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// The subjects of one execution root under construction, each stored once.
///
/// The builder records reads and the condition decomposer records runtime
/// tests through the same table, so a read and a test of the same subject get
/// the same id. A table made with [`Default`] stores no [`FlowSubject::This`].
#[derive(Default)]
struct FlowSubjectTable {
    subjects: Vec<FlowSubject>,
    ids: FxHashMap<FlowSubject, FlowSubjectId>,
    /// The execution root whose `this` can be a subject, or `None` when paths
    /// on the root's `this` cannot narrow.
    this_root: Option<JsSyntaxNode>,
    /// Maps the range of each element variable of a [`FlowSubject::Pattern`]
    /// in the table to the pattern and the key of the property that the
    /// element reads. It holds only the elements that the narrowing of the
    /// pattern reaches.
    pattern_elements: FxHashMap<TextRange, (FlowSubjectId, PropertyKey)>,
    /// The variables found to be no element of such a pattern.
    non_elements: FxHashSet<TextRange>,
}

impl FlowSubjectTable {
    /// Returns an empty table for the execution root `root`. The table stores
    /// the root's `this` only when `this_can_narrow` is true.
    fn new(root: &AnyJsControlFlowRoot, this_can_narrow: bool) -> Self {
        Self {
            this_root: this_can_narrow.then(|| root.syntax().clone()),
            ..Self::default()
        }
    }

    /// Returns the id of `subject`, or `None` when the table does not store it.
    fn get(&self, subject: &FlowSubject) -> Option<FlowSubjectId> {
        self.ids.get(subject).copied()
    }

    /// Returns the subjects, each at the index of its id.
    fn as_slice(&self) -> &[FlowSubject] {
        &self.subjects
    }

    /// Returns the id of the variable that `reference`, a read or an
    /// assignment target, refers to, adding the variable to the table on
    /// first use.
    ///
    /// Returns `None` when `reference` refers to no declaration, as a global
    /// does, or when the table has used every [`FlowSubjectId`].
    fn reference(
        &mut self,
        model: &SemanticModel,
        reference: &impl HasDeclarationAstNode,
    ) -> Option<FlowSubjectId> {
        let binding = model.binding(reference)?;
        self.intern(FlowSubject::Binding(binding.range()))
    }

    /// Returns the id of the `this` that `expression` reads, adding it to the
    /// table on first use.
    ///
    /// Returns `None` when the table's root cannot narrow its `this`, when the
    /// table has used every [`FlowSubjectId`], and for a `this` in the
    /// decorators or `extends` clause of a class inside the root, or in the
    /// computed key of an object literal method or accessor inside it. That
    /// `this` is the root's own, but Biome types it as an instance of the
    /// class or as the object.
    ///
    /// Also returns `None` for a `this` outside the root, because it can be
    /// the `this` of another function. The initializer of an aliased
    /// condition that an enclosing function declares can contain such a
    /// `this`.
    fn this(&mut self, expression: &JsThisExpression) -> Option<FlowSubjectId> {
        let root = self.this_root.as_ref()?;
        for node in expression.syntax().ancestors() {
            if node == *root {
                return self.intern(FlowSubject::This);
            }
            if OTHER_THIS_KINDS.matches(node.kind()) {
                return None;
            }
        }
        None
    }

    /// Returns the destructuring pattern that a runtime test of the variable
    /// declared at `range` narrows, with the key of the property that the
    /// variable reads from it. Adds the pattern to the table on first use.
    /// See [`subjects::narrowing_pattern`] for the patterns that a test
    /// narrows.
    ///
    /// Returns `None` when no such pattern holds the variable, and when the
    /// table has used every [`FlowSubjectId`].
    fn pattern(
        &mut self,
        range: TextRange,
        model: &SemanticModel,
    ) -> Option<(FlowSubjectId, PropertyKey)> {
        if let Some(element) = self.pattern_elements.get(&range) {
            return Some(element.clone());
        }
        if self.non_elements.contains(&range) {
            return None;
        }
        let Some((pattern, elements)) = model
            .as_binding_by_range(range)
            .and_then(|binding| subjects::narrowing_pattern(&binding, model))
        else {
            self.non_elements.insert(range);
            return None;
        };
        let pattern = self.intern(FlowSubject::Pattern(pattern.range()))?;
        for (element, key) in elements {
            self.pattern_elements.insert(element, (pattern, key));
        }
        self.pattern_elements.get(&range).cloned()
    }

    /// Returns the pattern and the key of the property that the variable
    /// declared at `range` reads, when a pattern already in the table holds
    /// the variable. Unlike [`Self::pattern`], it never adds a pattern.
    fn pattern_element(&self, range: TextRange) -> Option<&(FlowSubjectId, PropertyKey)> {
        self.pattern_elements.get(&range)
    }

    fn intern(&mut self, subject: FlowSubject) -> Option<FlowSubjectId> {
        if let Some(id) = self.ids.get(&subject) {
            return Some(*id);
        }
        let id = FlowSubjectId::try_from(self.subjects.len()).ok()?;
        self.subjects.push(subject.clone());
        self.ids.insert(subject, id);
        Some(id)
    }

    /// Returns the subjects, each at the index of its id.
    fn into_subjects(self) -> Vec<FlowSubject> {
        self.subjects
    }
}

/// Limits the graph steps spent deciding which reads a test can reach in one
/// execution root. Like other construction limits, exceeding it leaves the
/// root without flow, so its reads keep their ordinary types.
const MAX_RELEVANCE_STEPS: usize = 1_048_576;

/// A predecessor relation, not an instruction to execute.
///
/// Joins retain every reachable predecessor. Loop backedges can make the graph
/// cyclic, so consumers must bound their traversal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FlowNode {
    Start,
    Join(Vec<FlowNodeId>),
    Condition {
        antecedent: FlowNodeId,
        /// Index of the condition's guard in [`RootFlow::guards`].
        guard: usize,
    },
    /// The point right after a write to a subject.
    Write {
        antecedent: FlowNodeId,
        /// Index of the write in [`RootFlow::writes`].
        write: u32,
    },
}

/// A write to a subject, which a [`FlowNode::Write`] stands for.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FlowWrite {
    pub(crate) subject: FlowSubjectId,
    pub(crate) effect: FlowWriteEffect,
}

/// How a write changes the type of its subject, matching TypeScript.
///
/// An index refers to [`RootFlow::assigned`], the value that the write
/// assigns. A write whose value Biome can't type as TypeScript does has no
/// index, so the subject gets its declared type there.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum FlowWriteEffect {
    /// An assignment of a value, as `x = v`, `x ??= v` where the right
    /// operand runs, a destructuring target, or an initializer: the declared
    /// type reduced by the assigned type, as [`narrow_by_assignment`] gives
    /// it.
    ///
    /// [`narrow_by_assignment`]: biome_js_type_info::narrow_by_assignment
    Assign(Option<u32>),
    /// An assignment whose right operand is a binary expression with a shift
    /// or arithmetic operator, as `x = x + 1`: the primitive type of the
    /// literal types of the declared type reduced by the assigned type, as in
    /// TypeScript.
    CompoundLike(Option<u32>),
    /// A compound assignment, as `x += 1`, or `++` and `--`: the primitive
    /// type of the literal types of the type before the write, which can be
    /// wider than the declared type, as `number` for `x: 1 | 2` after `x++`.
    Compound,
    /// `delete o.p`, which assigns `undefined`.
    Undefined,
    /// A write that Biome doesn't reduce, as `(x as T) = v` or `delete o["p"]`:
    /// the declared type.
    Reset,
    /// A write through a key that is not a literal, as `o[k] = v` or
    /// `delete o[k]`, which can write any property of the subject: every path
    /// below the subject gets its declared type, and the subject itself keeps
    /// its type.
    Wildcard,
}

/// The value that a write assigns, before inference resolves its type in
/// the reading context.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FlowAssigned {
    pub(crate) value: FlowAssignedValue,
    /// The fields that destructuring reads from the value to reach the
    /// target, outermost first, each with the default that supplies it when
    /// it is `undefined`.
    pub(crate) steps: Box<[FlowDestructureStep]>,
    /// The callees of the calls whose results the value and the defaults can
    /// be. A write reduces by the value only after the reading context
    /// confirms that the return types of these callees come from annotations.
    pub(crate) callees: Box<[FlowCallee]>,
}

/// The callee of a call whose result a value can be, when collection can't
/// tell whether its return type comes from an annotation. The reading
/// context decides it; see [`CalleeReturn::Resolved`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum FlowCallee {
    /// The binding at this range, which an import declares.
    Import(TextRange),
    /// The member that the static member expression at this range reads, as
    /// `a.m`. The reading context takes its object and name from the type
    /// that local inference collected for the expression.
    Member(TextRange),
}

/// Where the value of a write comes from.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum FlowAssignedValue {
    /// The value of the expression at this range, as the right operand of an
    /// assignment or an initializer.
    Expression(TextRange),
    /// An element of the iterable at this range, which the head of a
    /// `for...of` loop assigns.
    IterableValue(TextRange),
    /// A key, which the head of a `for...in` loop assigns: a `string`.
    Key,
}

/// One field that destructuring reads, as `a` in `({ a: x } = o)` or `0` in
/// `[x] = t`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FlowDestructureStep {
    pub(crate) field: DestructureField,
    /// The range of the default that supplies the field when it is
    /// `undefined`, as `1` in `[x = 1] = t`.
    pub(crate) default: Option<TextRange>,
}

/// The paths through one execution root and the guards its conditions apply.
#[derive(Debug)]
pub(crate) struct RootFlow {
    pub(crate) nodes: Box<[FlowNode]>,
    pub(crate) guards: Box<[FlowGuard]>,
    /// The subjects that guards and reads refer to, each at the index of its
    /// id.
    pub(crate) subjects: Box<[FlowSubject]>,
    /// The points of each subject's reads, sorted and without duplicates.
    pub(crate) read_points: FxHashMap<FlowSubjectId, Box<[FlowNodeId]>>,
    /// The subjects with reads that conditions narrow only through tests of
    /// their shape:
    ///
    /// - [`FlowGuard::Property`], such as `s` when the only condition that
    ///   mentions it is `s.kind === "c"`.
    /// - `in`, which [`FlowTest::In`] applies to `x` in `"a" in x`.
    /// - `instanceof`, which [`FlowTest::Instanceof`] applies to `x` in
    ///   `x instanceof Date`.
    /// - Calls to type-guard functions whose targets have only object types,
    ///   as `isFoo(x)` with `x is Foo`.
    ///
    /// A test of a property or `in` narrows only a union with two object
    /// types, and the others narrow only a type with an object type or
    /// `unknown`. So these subjects need no solve when their declared type is
    /// not one. Every [`FlowSubject::Pattern`] with reads is in this set.
    pub(crate) property_only: FxHashSet<FlowSubjectId>,
    /// The subjects in [`Self::property_only`] that `instanceof` or a
    /// type-guard function whose target has only object types narrows. Their
    /// declared type need not be a union.
    pub(crate) instance_tested: FxHashSet<FlowSubjectId>,
    /// The [`FlowSubject::Pattern`] of each element whose reads a test of the
    /// pattern can reach, keyed by the element's subject. Each such read is
    /// also a read point of the pattern, so that the declared type of the
    /// element at the read is its property on the narrowed pattern.
    pub(crate) pattern_elements: FxHashMap<FlowSubjectId, FlowSubjectId>,
    /// The subjects with reads that a [`FlowGuard::ReadonlyPath`] guard
    /// tests, or narrows as the object of a property guard. Inference decides
    /// at a read of such a subject whether its path is readonly.
    pub(crate) readonly_tested: FxHashSet<FlowSubjectId>,
    /// The subjects in [`Self::readonly_tested`] that no other guard tests or
    /// narrows, which need no solve when their path is not readonly.
    pub(crate) only_readonly_tested: FxHashSet<FlowSubjectId>,
    /// The `switch` statements without `default` that inference may find
    /// exhaustive, at most [`MAX_SWITCHES`] of them. The statement at index
    /// `i` is the one whose [`FlowGuard::Bypass`] has `switch` `i`. Empty when
    /// no such guard reaches a read, and when finding the reads they reach
    /// exceeds the relevance work limit.
    pub(crate) switches: Box<[FlowSwitch]>,
    /// The read points that the [`FlowGuard::Bypass`] of a statement in
    /// [`Self::switches`] reaches, sorted.
    pub(crate) bypassed: Box<[BypassedPoint]>,
    /// The writes that [`FlowNode::Write`] nodes stand for.
    pub(crate) writes: Box<[FlowWrite]>,
    /// The values that the writes assign.
    pub(crate) assigned: Box<[FlowAssigned]>,
    /// For each subject with reads that a write with a value reduces, the
    /// indices of those values in [`Self::assigned`], sorted. Inference
    /// resolves their types in this order and passes them to the solve.
    pub(crate) subject_writes: FxHashMap<FlowSubjectId, Box<[u32]>>,
    /// The subjects with reads that only writes mention and no condition
    /// tests. A write narrows only a union or a type with a literal member,
    /// so their reads need no solve when their declared type is neither.
    pub(crate) assignment_only: FxHashSet<FlowSubjectId>,
    /// The subjects with reads that may be read before they are assigned: a
    /// `let` declared without an initializer, a `var`, and in a constructor,
    /// a property of `this` whose field has no initializer. Narrowing starts
    /// them with `undefined` added to their declared type, as TypeScript
    /// does. See [`RootCollector::uninitialized_subjects`] for the exact
    /// conditions.
    ///
    /// A subject with a capture is never here: its type at the capture point
    /// already starts where the variable is declared.
    pub(crate) uninitialized: FxHashSet<FlowSubjectId>,
    /// For each variable that the root reads and that starts, when the root
    /// runs, with its type where an enclosing root creates the closure that
    /// holds the root, that capture point, as [`FlowCapture`] describes.
    pub(crate) captures: FxHashMap<FlowSubjectId, FlowCapture>,
}

/// Where a variable that a closure reads continues the flow of an enclosing
/// root, as TypeScript's `getFlowTypeOfReference` continues from the point
/// where a function expression is created.
///
/// A root that the closure holds, directly or through other closures, starts
/// the variable with its type at [`Self::point`] instead of its declared
/// type. That type can itself come from a capture of the enclosing root, so
/// inference resolves the chain outward.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FlowCapture {
    /// Index of the enclosing root in the module flow.
    pub(crate) root: usize,
    /// The flow point of the enclosing root where it creates the outermost
    /// closure that holds the reading root, a read point of
    /// [`Self::subject`] there.
    pub(crate) point: FlowNodeId,
    /// The variable in the enclosing root's subject table.
    pub(crate) subject: FlowSubjectId,
    /// Where that closure starts, which decides the `switch` statements whose
    /// cases come before the point.
    pub(crate) start: TextSize,
    /// Whether some code assigns the variable. TypeScript's `checkIdentifier`
    /// then assumes a variable that a closure reads assigned where it is
    /// declared, so a `let` without an initializer starts there without
    /// `undefined`, while a `let` that nothing assigns keeps it.
    pub(crate) assumes_initialized: bool,
}

/// A read point with the `switch` statements whose [`FlowGuard::Bypass`]
/// reaches it: bit `i` stands for [`RootFlow::switches`]`[i]`.
pub(crate) type BypassedPoint = (FlowNodeId, u64);

/// Limits how many `switch` statements per execution root can be found
/// exhaustive, because the solve takes them as the bits of a `u64` mask.
pub(crate) const MAX_SWITCHES: usize = 64;

/// A `switch` statement whose [`FlowGuard::Bypass`] gives `never` when the
/// statement is exhaustive.
///
/// Collection records only statements that TypeScript can find exhaustive:
/// statements without `default`, with a discriminant that is a subject or
/// `typeof` of one, and with cases that all test a unit value or a kind of
/// value. A unit value is a value such as `"a"`, `-1`, `null`, `undefined`, or
/// a `const` that holds one, and a kind of value is a string such as
/// `"string"`. A case that tests an imported variable is recorded too, because
/// only inference can read its value. Inference counts the statement as
/// exhaustive only when that variable holds a unit value.
#[derive(Debug)]
pub(crate) struct FlowSwitch {
    /// The range of the expression whose type at the statement decides
    /// whether it is exhaustive: the discriminant, or the operand of
    /// `typeof`, without parentheses.
    pub(crate) discriminant: TextRange,
    /// The subject of that expression.
    pub(crate) subject: FlowSubjectId,
    /// The index of the guard that tests that no case matches, the guard of
    /// the statement's [`FlowGuard::Bypass`].
    pub(crate) guard: usize,
    /// Where the statement's cases start. A read before this offset never
    /// asks whether the statement is exhaustive. The discriminant's own read
    /// lies before it, so deciding whether a statement is exhaustive never
    /// asks about the statement itself.
    pub(crate) start: TextSize,
    /// The kinds of value that the cases of `switch (typeof x)` test, one bit
    /// per [`TypeofKind`], or `None` for any other discriminant.
    /// [`typeof_kind_bit`] maps each kind to its bit. TypeScript finds such a
    /// statement exhaustive for `x: unknown` or `x: any` when its cases test
    /// every kind.
    pub(crate) typeof_kinds: Option<u8>,
}

/// Returns the bit of `kind` in [`FlowSwitch::typeof_kinds`].
pub(crate) const fn typeof_kind_bit(kind: TypeofKind) -> u8 {
    1 << match kind {
        TypeofKind::Undefined => 0,
        TypeofKind::Object => 1,
        TypeofKind::Boolean => 2,
        TypeofKind::Number => 3,
        TypeofKind::BigInt => 4,
        TypeofKind::String => 5,
        TypeofKind::Symbol => 6,
        TypeofKind::Function => 7,
    }
}

impl RootFlow {
    /// Returns the statements in [`Self::switches`] whose bypass reaches
    /// `point`, one bit each, or zero when no bypass reaches it.
    pub(crate) fn bypassed_switches(&self, point: FlowNodeId) -> u64 {
        self.bypassed
            .binary_search_by_key(&point, |(point, _)| *point)
            .map_or(0, |index| self.bypassed[index].1)
    }

    /// Returns the points where the root reads `subject`, sorted, or an empty
    /// slice when it keeps no read of it.
    pub(crate) fn read_points(&self, subject: FlowSubjectId) -> &[FlowNodeId] {
        self.read_points.get(&subject).map_or(&[], |points| points)
    }

    /// Returns whether the root reads `subject` at `point` where a test of
    /// `subject` can reach the read.
    pub(crate) fn reads_at(&self, subject: FlowSubjectId, point: FlowNodeId) -> bool {
        self.read_points(subject).binary_search(&point).is_ok()
    }

    /// Returns whether conditions narrow `subject` only through tests of its
    /// shape, as [`Self::property_only`] describes.
    pub(crate) fn is_property_only(&self, subject: FlowSubjectId) -> bool {
        self.property_only.contains(&subject)
    }

    /// Returns whether `instanceof` or a type-guard function whose target has
    /// only object types narrows `subject`, a subject that
    /// [`Self::is_property_only`] accepts.
    pub(crate) fn is_instance_tested(&self, subject: FlowSubjectId) -> bool {
        self.instance_tested.contains(&subject)
    }

    /// Returns whether a [`FlowGuard::ReadonlyPath`] guard may narrow
    /// `subject`, as [`Self::readonly_tested`] describes.
    pub(crate) fn is_readonly_tested(&self, subject: FlowSubjectId) -> bool {
        self.readonly_tested.contains(&subject)
    }

    /// Returns whether only [`FlowGuard::ReadonlyPath`] guards may narrow
    /// `subject`, as [`Self::only_readonly_tested`] describes.
    pub(crate) fn is_only_readonly_tested(&self, subject: FlowSubjectId) -> bool {
        self.only_readonly_tested.contains(&subject)
    }

    /// Returns the [`FlowSubject::Pattern`] that holds the element `subject`
    /// when a test of the pattern can reach a read of the element. At such a
    /// read, the element's declared type is its property on the narrowed
    /// pattern.
    pub(crate) fn element_pattern(&self, subject: FlowSubjectId) -> Option<FlowSubjectId> {
        self.pattern_elements.get(&subject).copied()
    }

    pub(crate) fn subject(&self, subject: FlowSubjectId) -> Option<&FlowSubject> {
        self.subjects.get(subject as usize)
    }

    /// Returns the range of the variable that the path of `subject` starts
    /// at, which is the variable itself for a [`FlowSubject::Binding`].
    ///
    /// Returns `None` for `this`, a path on `this`, and a pattern, and when
    /// the root has no such subject.
    pub(crate) fn subject_binding(&self, subject: FlowSubjectId) -> Option<TextRange> {
        match subject_root(&self.subjects, subject)? {
            FlowSubject::Binding(range) => Some(*range),
            FlowSubject::This | FlowSubject::Member { .. } | FlowSubject::Pattern(_) => None,
        }
    }

    /// Returns the indices in [`Self::assigned`] of the values that the
    /// writes of `subject` assign, sorted, or an empty slice when no write
    /// with a value reduces it.
    pub(crate) fn subject_writes(&self, subject: FlowSubjectId) -> &[u32] {
        self.subject_writes
            .get(&subject)
            .map_or(&[], |writes| writes)
    }

    /// Returns whether only writes narrow `subject`, as
    /// [`Self::assignment_only`] describes.
    pub(crate) fn is_assignment_only(&self, subject: FlowSubjectId) -> bool {
        self.assignment_only.contains(&subject)
    }

    /// Returns whether `subject` may be read before it is assigned, as
    /// [`Self::uninitialized`] describes.
    pub(crate) fn is_uninitialized(&self, subject: FlowSubjectId) -> bool {
        self.uninitialized.contains(&subject)
    }

    /// Returns where `subject`, a variable, continues the flow of an
    /// enclosing root, as [`Self::captures`] describes, or `None` when it
    /// starts from its declared type.
    pub(crate) fn capture(&self, subject: FlowSubjectId) -> Option<FlowCapture> {
        self.captures.get(&subject).copied()
    }

    /// Returns whether the subject `prefix` is a proper prefix of the path
    /// of `subject`, as `o` and `o.v` are of `o.v.w`.
    pub(crate) fn is_proper_prefix(
        &self,
        prefix: FlowSubjectId,
        mut subject: FlowSubjectId,
    ) -> bool {
        while let Some(FlowSubject::Member { object, .. }) = self.subjects.get(subject as usize) {
            // A table stores objects before their members.
            if *object >= subject {
                return false;
            }
            if *object == prefix {
                return true;
            }
            subject = *object;
        }
        false
    }
}

/// A read whose type a test in its execution root may refine.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FlowRead {
    /// Index of the read's execution root in the module flow.
    pub(crate) root: usize,
    /// The flow that reaches the read, before evaluating it.
    pub(crate) point: FlowNodeId,
    /// The subject the read refers to, in its root's subject table.
    pub(crate) subject: FlowSubjectId,
    /// Whether the read lies in a closure that the root creates, of a
    /// variable that some code assigns, which TypeScript then assumes
    /// assigned where the variable is declared, as [`FlowCapture`]
    /// describes. False for a read in the root's own code.
    pub(crate) assumes_initialized: bool,
}

/// Flow facts for one module.
#[derive(Debug, Default)]
pub(crate) struct ModuleFlow {
    roots: Vec<RootFlow>,
    reads: FxHashMap<TextRange, FlowRead>,
    /// Points to each candidate expression, keyed by its trimmed range.
    candidates: FxHashMap<TextRange, AstPtr<AnyFlowExpression>>,
    /// The declarations of the bindings in [`RootFlow::property_only`] of
    /// every root that keeps reads of them. A variable is here only when it
    /// is property-only in each of those roots, because narrowing starts
    /// from the same type in all of them.
    property_only_bindings: FxHashSet<TextRange>,
    /// The declarations of the bindings in [`Self::property_only_bindings`]
    /// that [`RootFlow::instance_tested`] lists in some root.
    instance_tested_bindings: FxHashSet<TextRange>,
    /// The declarations of the bindings in [`RootFlow::assignment_only`] of
    /// every root that keeps reads of them.
    assignment_only_bindings: FxHashSet<TextRange>,
    /// The candidates that are candidates only because they are, or contain,
    /// reads of one binding in [`Self::property_only_bindings`], reads of one
    /// binding that only `instanceof` or type-guard functions whose targets
    /// have only object types reach, or reads of elements of one pattern in
    /// [`Self::pattern_types`] that no test of the elements themselves
    /// reaches, keyed by range, with the range of that binding's declaration
    /// or of that pattern. Such a candidate keeps its type when the binding
    /// or the pattern has no type that narrowing starts from.
    property_only_candidates: FxHashMap<TextRange, TextRange>,
    /// Maps the range of each [`FlowSubject::Pattern`] with reads to its
    /// declared type, which is the collected type of the value that the
    /// pattern destructures.
    pattern_types: FxHashMap<TextRange, TypeReference>,
}

impl ModuleFlow {
    /// Collects flow for the execution roots that a [`FlowRootScanner`]
    /// selected.
    ///
    /// Reads are kept only when their binding has a narrowable collected type,
    /// as decided by [`is_raw_narrowing_invariant`], and a test or a write of
    /// that binding can reach them. Execution roots that exceed a construction
    /// or relevance work limit keep no reads.
    pub(crate) fn collect(
        roots: FlowRootScanner,
        model: &SemanticModel,
        raw_types: &[RawTypeData],
        raw_binding_types: &FxHashMap<TextRange, TypeReference>,
    ) -> Self {
        Self::collect_with_relevance_budget(
            roots,
            model,
            raw_types,
            raw_binding_types,
            MAX_RELEVANCE_STEPS,
        )
    }

    fn collect_with_relevance_budget(
        roots: FlowRootScanner,
        model: &SemanticModel,
        raw_types: &[RawTypeData],
        raw_binding_types: &FxHashMap<TextRange, TypeReference>,
        relevance_budget: usize,
    ) -> Self {
        let mut flow = Self::default();
        let mut collector = RootCollector {
            model,
            raw_types,
            raw_binding_types,
            relevance_budget,
            functions: GuardFunctions::new(raw_types, raw_binding_types),
            enclosing: FxHashMap::default(),
            redeclared: FxHashMap::default(),
            binding_reads: FxHashMap::default(),
            built: FxHashSet::default(),
            evaluated: FxHashSet::default(),
            exits: FxHashMap::default(),
            capture_rules: FxHashMap::default(),
        };
        for selected in roots.selected() {
            collector.add_root(
                &mut flow,
                &selected.root,
                roots.bindings(selected),
                selected.has_this,
            );
        }
        for (range, reads) in collector.binding_reads {
            match reads {
                BindingReads::PropertyOnly { instance_tested } => {
                    flow.property_only_bindings.insert(range);
                    if instance_tested {
                        flow.instance_tested_bindings.insert(range);
                    }
                }
                BindingReads::AssignmentOnly => {
                    flow.assignment_only_bindings.insert(range);
                }
                BindingReads::Tested => {}
            }
        }
        flow
    }

    /// Returns whether the expression at `range` may have a flow-sensitive type.
    pub(crate) fn is_candidate(&self, range: TextRange) -> bool {
        self.candidates.contains_key(&range)
    }

    /// Returns a pointer to the candidate expression at `range`, which
    /// resolves in the module's syntax tree without searching by range.
    pub(crate) fn candidate(&self, range: TextRange) -> Option<&AstPtr<AnyFlowExpression>> {
        self.candidates.get(&range)
    }

    /// Returns whether conditions narrow the binding declared at `range` only
    /// through tests of its shape, as [`RootFlow::property_only`] describes.
    pub(crate) fn is_property_only_binding(&self, range: TextRange) -> bool {
        self.property_only_bindings.contains(&range)
    }

    /// Returns whether conditions narrow the binding declared at `range` only
    /// through tests of its shape and at least one is `instanceof` or a
    /// type-guard function whose target has only object types.
    ///
    /// This returns `false` when a non-shape test also narrows the binding,
    /// even if only `instanceof` or such type-guard tests reach a particular
    /// read.
    pub(crate) fn is_instance_tested_binding(&self, range: TextRange) -> bool {
        self.instance_tested_bindings.contains(&range)
    }

    /// Returns whether only writes narrow the binding declared at `range`, as
    /// [`RootFlow::assignment_only`] describes.
    pub(crate) fn is_assignment_only_binding(&self, range: TextRange) -> bool {
        self.assignment_only_bindings.contains(&range)
    }

    /// Returns the declaration range of the single binding or pattern whose
    /// reads alone make the expression at `range` a candidate.
    ///
    /// A binding qualifies when conditions narrow it only through tests of
    /// its shape, or when only `instanceof` or type-guard functions whose
    /// targets have only object types reach its reads in this expression. The
    /// latter can apply even when other tests narrow the binding at other
    /// reads. A pattern qualifies through reads of elements that no test of
    /// the elements themselves reaches.
    pub(crate) fn property_only_candidate(&self, range: TextRange) -> Option<TextRange> {
        self.property_only_candidates.get(&range).copied()
    }

    /// Returns the declared type of the [`FlowSubject::Pattern`] at `range`,
    /// which is the collected type of the value that the pattern
    /// destructures, or `None` when `range` is not a pattern with reads.
    pub(crate) fn pattern_type(&self, range: TextRange) -> Option<&TypeReference> {
        self.pattern_types.get(&range)
    }

    /// Records `expression` as a candidate. `binding` is the declaration of
    /// the binding in [`Self::property_only_bindings`], or the range of the
    /// pattern in [`Self::pattern_types`], whose read alone makes
    /// `expression` a candidate. It is `None` for a read of another subject.
    fn insert_candidate(&mut self, expression: &AnyFlowExpression, binding: Option<TextRange>) {
        let range = expression.range();
        let is_new = !self.candidates.contains_key(&range);
        if is_new {
            self.candidates.insert(range, AstPtr::new(expression));
        }
        match binding {
            Some(binding) if is_new => {
                self.property_only_candidates.insert(range, binding);
            }
            Some(binding) if self.property_only_candidates.get(&range) == Some(&binding) => {}
            _ => {
                self.property_only_candidates.remove(&range);
            }
        }
    }

    /// Returns the flow of the read at `range`, an identifier or a member
    /// expression.
    pub(crate) fn read(&self, range: TextRange) -> Option<FlowRead> {
        self.reads.get(&range).copied()
    }

    pub(crate) fn root(&self, index: usize) -> Option<&RootFlow> {
        self.roots.get(index)
    }

    /// Returns the outermost root of the capture chain that starts at the
    /// variable `subject` of the root at index `root`, with the variable in
    /// its subject table: the root itself when the variable has no capture
    /// there.
    ///
    /// Each capture points to a root collected later, so the chain ends.
    fn capture_holder(
        &self,
        mut root: usize,
        mut subject: FlowSubjectId,
    ) -> (usize, FlowSubjectId) {
        while let Some(capture) = self.roots.get(root).and_then(|flow| flow.capture(subject)) {
            if capture.root <= root {
                break;
            }
            root = capture.root;
            subject = capture.subject;
        }
        (root, subject)
    }
}

struct RootCollector<'a> {
    model: &'a SemanticModel,
    raw_types: &'a [RawTypeData],
    raw_binding_types: &'a FxHashMap<TextRange, TypeReference>,
    relevance_budget: usize,
    functions: GuardFunctions<'a>,
    /// Caches, by declaration, whether a variable read in a root nested in
    /// the one that declares it can narrow there. See
    /// [`Self::encloses_eligible`].
    enclosing: FxHashMap<TextRange, bool>,
    /// Caches, for each scope, the declarations that share their name with
    /// another declaration of the scope, or of a function body with a
    /// parameter. See [`Self::redeclared_in`].
    redeclared: FxHashMap<ScopeId, FxHashSet<TextRange>>,
    /// How the kept reads of each variable narrow, merged over the roots
    /// that keep them, keyed by its declaration.
    binding_reads: FxHashMap<TextRange, BindingReads>,
    /// The ranges of the roots built so far.
    built: FxHashSet<TextRange>,
    /// The closures that the roots built so far create in their reachable
    /// code, and their reachable reads of variables that they don't declare,
    /// which closures created in them can capture.
    evaluated: FxHashSet<TextRange>,
    /// Where the first statement that ends every path ends in the own code
    /// of each closure that no root built, as [`Self::first_exit`] finds it.
    exits: FxHashMap<TextRange, TextSize>,
    /// Whether a closure that reads each variable continues the flow where
    /// it is created, as [`Self::capture_rule`] decides, keyed by its
    /// declaration.
    capture_rules: FxHashMap<TextRange, CaptureRule>,
}

/// Whether a read of a variable inside a closure continues the flow where the
/// closure is created, as TypeScript's `checkIdentifier` decides with
/// `isConstantVariable`, `isParameterOrMutableLocalVariable`, and
/// `isPastLastAssignment`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CaptureRule {
    /// Every read does: the variable is a `const`, or a parameter, a `catch`
    /// variable, or a `let` that nothing assigns.
    Always,
    /// A read does when it starts after this offset, the end of the last
    /// assignment of a parameter, a `catch` variable, or a `let`, as
    /// [`assignment_position`] extends it.
    After(TextSize),
    /// No read does: the variable is a `var`, an import, a function, or a
    /// class, a `let` that is exported or declared at the top level of a
    /// file, or a variable that another function assigns.
    Never,
}

impl CaptureRule {
    /// Returns whether a read that starts at `read` continues the flow.
    fn continues_at(self, read: TextSize) -> bool {
        match self {
            Self::Always => true,
            Self::After(position) => position < read,
            Self::Never => false,
        }
    }
}

/// A read inside a closure that a root nested in the root under collection
/// keeps, where that nested root's start continues the collected root's flow.
struct PendingCapture {
    /// The root that keeps the read or that the read's root continues from,
    /// with the variable in its subject table, which gets the capture.
    holder: (usize, FlowSubjectId),
    /// The capture point of the outermost closure that holds the read.
    point: FlowNodeId,
    /// The variable in the collected root's subject table.
    subject: FlowSubjectId,
    /// Where that closure starts.
    start: TextSize,
    /// Whether some code assigns the variable, as
    /// [`FlowCapture::assumes_initialized`] describes.
    assumes_initialized: bool,
}

/// Classifies how the conditions and writes of every root that keeps reads
/// of a variable narrow those reads. [`ModuleFlow::property_only_bindings`]
/// and [`ModuleFlow::assignment_only_bindings`] record the result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BindingReads {
    /// Only tests of the variable's shape narrow the reads: tests of its
    /// properties, `in`, `instanceof`, and type-guard functions whose
    /// targets have only object types. `instance_tested` is true when
    /// `instanceof` or such a type-guard function narrows them in some root.
    /// See [`RootFlow::property_only`].
    PropertyOnly { instance_tested: bool },
    /// Only writes narrow the reads, and no condition tests the variable.
    /// See [`RootFlow::assignment_only`].
    AssignmentOnly,
    /// Any other test, in at least one root.
    Tested,
}

impl BindingReads {
    /// Returns how the reads of a variable narrow when some roots narrow them
    /// as `self` does and others as `other` does.
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (
                Self::PropertyOnly { instance_tested },
                Self::PropertyOnly {
                    instance_tested: other,
                },
            ) => Self::PropertyOnly {
                instance_tested: instance_tested || other,
            },
            (Self::AssignmentOnly, Self::AssignmentOnly) => Self::AssignmentOnly,
            (
                Self::PropertyOnly { .. } | Self::AssignmentOnly | Self::Tested,
                Self::PropertyOnly { .. } | Self::AssignmentOnly | Self::Tested,
            ) => Self::Tested,
        }
    }
}

impl RootCollector<'_> {
    /// Collects the flow of `root`, which declares `bindings` in its own code
    /// and uses `this` there when `has_this` is true.
    ///
    /// A variable that an enclosing root declares, or an import read in a
    /// function, narrows in `root` too, from its declared type, by the tests
    /// and writes of `root` alone. TypeScript uses the flow at a closure's
    /// creation to narrow a read when the variable is a `const`, or when
    /// nothing assigns the variable after the closure is created. Biome
    /// starts the captured variable from its declared type instead. So `root`
    /// gets a graph even when it declares no variable that can narrow, and
    /// relevance keeps no read when no subject can narrow.
    ///
    /// A read inside a closure that `root` creates, directly or through other
    /// closures, continues the flow of `root` where it creates the outermost
    /// one, as [`Self::closure_reads`] describes, so a test in `root` can
    /// narrow it.
    fn add_root(
        &mut self,
        flow: &mut ModuleFlow,
        root: &AnyJsControlFlowRoot,
        bindings: &[JsIdentifierBinding],
        has_this: bool,
    ) {
        let mut eligible: FxHashSet<TextRange> = bindings
            .iter()
            .filter_map(|binding| self.eligible_range(binding))
            .collect();
        let this_can_narrow = has_this && subjects::this_can_narrow(root);
        let Some(BuiltRoot {
            nodes,
            mut guards,
            mut subjects,
            mut mentions,
            mut property_only,
            mut instance_tested,
            instance_mentions,
            mut reads,
            switches,
            writes,
            assigned,
            written,
            mut captures,
        }) = build_root(root, self.model, &mut self.functions, this_can_narrow)
        else {
            return;
        };
        let root_range = root.syntax().text_trimmed_range();
        // Closures created in this root's code, and their capture of the
        // variables read here that the root doesn't declare, are reachable
        // only where the builder evaluated them.
        self.built.insert(root_range);
        for read in &reads {
            if let Some(FlowSubject::Binding(range)) =
                subjects.as_slice().get(read.subject as usize)
                && !root_range.contains_range(*range)
            {
                self.evaluated.insert(read.expression.range());
            }
        }
        self.evaluated
            .extend(captures.iter().map(|capture| capture.closure.range()));
        captures.sort_unstable_by_key(|capture| capture.closure.range().start());
        // The variables that the root reads but doesn't declare: those of an
        // enclosing root and the module's imports.
        for subject in subjects.as_slice() {
            if let FlowSubject::Binding(range) = subject
                && !root_range.contains_range(*range)
                && self.encloses_eligible(*range)
            {
                eligible.insert(*range);
            }
        }
        // A variable narrows only where the graph has a write point for each
        // write that can run in the root's flow. Other writes, as in a
        // function called where it is defined or a class static block, run at
        // points that the graph doesn't model.
        eligible.retain(|range| self.writes_are_modeled(*range, root, &written));
        let uninitialized = self.uninitialized_subjects(root, &subjects, &eligible);
        // A write mentions its subject where it can change the subject's
        // type, so the reads after it are kept, and the subject narrows as a
        // tested one, not only through tests of its shape.
        let condition_mentioned: FxHashSet<FlowSubjectId> =
            mentions.iter().map(|(_, subject)| *subject).collect();
        let mut write_mentioned: FxHashSet<FlowSubjectId> = FxHashSet::default();
        for (node, write) in (0..).zip(&nodes).filter_map(|(id, node)| match node {
            FlowNode::Write { write, .. } => Some((id, *write)),
            FlowNode::Start | FlowNode::Join(_) | FlowNode::Condition { .. } => None,
        }) {
            if let Some(write) = writes.get(write as usize)
                && self.write_may_change(write, subjects.as_slice())
            {
                mentions.push((node, write.subject));
                write_mentioned.insert(write.subject);
            }
        }
        // A subject that may be read before it is assigned, such as a `var`,
        // can be assignment-only too. A read that only writes reach has the
        // declared type unless a write narrows it, and the `undefined` of the
        // subject's start gives the declared type too.
        let mut assignment_only: FxHashSet<FlowSubjectId> = write_mentioned
            .iter()
            .copied()
            .filter(|subject| !condition_mentioned.contains(subject))
            .collect();
        if !write_mentioned.is_empty() {
            property_only.retain(|subject| !write_mentioned.contains(subject));
            instance_tested.retain(|subject| !write_mentioned.contains(subject));
        }
        // A pattern's declared type is the collected type of the value it
        // destructures. The collected type of each element destructures that
        // value, so any element gives it.
        let mut pattern_types: FxHashMap<FlowSubjectId, TypeReference> = FxHashMap::default();
        for (element, (pattern, _)) in &subjects.pattern_elements {
            if !pattern_types.contains_key(pattern)
                && let Some(reference) = self.destructured_type(*element)
            {
                pattern_types.insert(*pattern, reference.clone());
            }
        }
        // A test of a property narrows its object only when the object's
        // declared type is a union with at least two object types.
        // `instanceof` and a type-guard function whose target has only object
        // types narrow only a type with an object type or `unknown`. So a
        // variable or a pattern that only such tests mention keeps its
        // declared type when its collected type shows, without resolving it,
        // that its tests can't narrow it. Dropping it here spares its reads
        // the flow queries.
        let unnarrowable: FxHashSet<FlowSubjectId> = property_only
            .iter()
            .copied()
            .filter(|subject| {
                let reference = match subjects.as_slice().get(*subject as usize) {
                    Some(FlowSubject::Binding(binding)) => self.raw_binding_types.get(binding),
                    Some(FlowSubject::Pattern(_)) => pattern_types.get(subject),
                    Some(FlowSubject::This | FlowSubject::Member { .. }) | None => return false,
                };
                let may_narrow = if instance_tested.contains(subject) {
                    raw_may_narrow_by_instanceof
                } else {
                    raw_may_be_object_union
                };
                reference.is_none_or(|reference| !may_narrow(reference, self.raw_types))
            })
            .collect();
        if !unnarrowable.is_empty() {
            property_only.retain(|subject| !unnarrowable.contains(subject));
            instance_tested.retain(|subject| !unnarrowable.contains(subject));
            mentions.retain(|(_, subject)| !unnarrowable.contains(subject));
        }
        let tested: FxHashSet<FlowSubjectId> =
            mentions.iter().map(|(_, subject)| *subject).collect();
        let is_path = |subject: FlowSubjectId| {
            matches!(
                subjects.as_slice().get(subject as usize),
                Some(FlowSubject::Member { .. })
            )
        };
        // A pattern narrows in the root that declares its elements, which all
        // have the same declaration.
        let local_patterns: FxHashSet<FlowSubjectId> = subjects
            .pattern_elements
            .iter()
            .filter(|(element, _)| eligible.contains(*element))
            .map(|(_, (pattern, _))| *pattern)
            .collect();
        // The table stores `this` only when it can narrow.
        let root_can_narrow = |subject: FlowSubjectId| {
            subject_root(subjects.as_slice(), subject).is_some_and(|root| match root {
                FlowSubject::Binding(binding) => eligible.contains(binding),
                FlowSubject::This => true,
                // A pattern has no paths, so it is `subject` itself.
                FlowSubject::Pattern(_) => local_patterns.contains(&subject),
                FlowSubject::Member { .. } => false,
            })
        };
        let has_aliased_property = guards.iter().any(|guard| {
            matches!(
                guard,
                FlowGuard::Property { object, aliased: true, .. }
                    if tested.contains(object) && root_can_narrow(*object)
            )
        });
        // Scanning for writes visits the whole root, so it runs only when a
        // path could narrow or an aliased property guard could narrow its
        // object.
        let written = (has_aliased_property
            || reads
                .iter()
                .any(|read| tested.contains(&read.subject) && is_path(read.subject)))
        .then(|| WrittenPaths::new(root, self.model, &subjects, &written));
        // A `const` keeps the value that the property had at its declaration,
        // and a write to the property before the test can make the object
        // differ from that value, as in
        // `const k = s.kind; s.kind = "s"; if (k === "c")`. An aliased property
        // guard therefore narrows nothing when a write in the root may change
        // its property.
        if has_aliased_property && let Some(written) = &written {
            for guard in &mut guards {
                let FlowGuard::Property {
                    object,
                    key,
                    aliased: true,
                    ..
                } = guard
                else {
                    continue;
                };
                let property = subjects.get(&FlowSubject::Member {
                    object: *object,
                    key: key.clone(),
                });
                // A write of the object itself, as `s = t` after
                // `const k = s.kind`, changes the object that the `const`
                // read the property of.
                if property
                    .is_none_or(|property| written.cancels(subjects.as_slice(), property, true))
                {
                    *guard = FlowGuard::Keep;
                }
            }
        }
        let (mut readonly_tested, mut only_readonly_tested) =
            readonly_tested_subjects(&mut guards, subjects.as_slice());
        // A write narrows a path whether or not it is readonly.
        only_readonly_tested.retain(|subject| !write_mentioned.contains(subject));
        let narrowable: FxHashSet<FlowSubjectId> = tested
            .iter()
            .copied()
            .filter(|subject| {
                root_can_narrow(*subject)
                    && written.as_ref().is_none_or(|written| {
                        !written.cancels(subjects.as_slice(), *subject, false)
                    })
            })
            .collect();
        let (captured, assumed_initialized) = if captures.is_empty() {
            (Vec::new(), FxHashSet::default())
        } else {
            self.closure_reads(
                flow,
                root_range,
                &captures,
                &mut subjects,
                &narrowable,
                &mut eligible,
                &mut reads,
            )
        };
        // The pattern that narrows the element `subject`, which a test of the
        // element itself need not mention.
        let element_pattern = |subject: FlowSubjectId| {
            let Some(FlowSubject::Binding(binding)) = subjects.as_slice().get(subject as usize)
            else {
                return None;
            };
            let (pattern, _) = subjects.pattern_element(*binding)?;
            (eligible.contains(binding) && narrowable.contains(pattern)).then_some(*pattern)
        };
        let reads: Vec<_> = reads
            .into_iter()
            .filter(|read| {
                narrowable.contains(&read.subject) || element_pattern(read.subject).is_some()
            })
            .collect();
        if reads.is_empty() && captured.is_empty() {
            return;
        }

        let walked: Vec<FlowSubjectId> = reads
            .iter()
            .map(|read| read.subject)
            .chain(captured.iter().map(|capture| capture.subject))
            .flat_map(|subject| {
                [
                    narrowable.contains(&subject).then_some(subject),
                    element_pattern(subject),
                ]
            })
            .flatten()
            .collect();
        // Variables that other tests also mention can have reads that only
        // `instanceof` or a type-guard function whose target has only object
        // types may reach. Such a read narrows only when the variable's
        // declared type can, so its candidate records the variable as a read
        // of a variable in `property_only` does.
        let instance_reached: Vec<FlowSubjectId> = walked
            .iter()
            .copied()
            .filter(|subject| {
                !property_only.contains(subject)
                    && matches!(
                        subjects.as_slice().get(*subject as usize),
                        Some(FlowSubject::Binding(_))
                    )
                    && instance_mentions
                        .iter()
                        .any(|(_, mentioned)| mentioned == subject)
            })
            .collect();
        let Some(reachable) = self.reachable_from_tests(
            &nodes,
            subjects.as_slice(),
            &mentions,
            &property_only,
            walked,
        ) else {
            return;
        };
        // `reachable` counts the `instanceof` mentions too, so the reads that
        // other tests reach need a walk without them.
        let reached_by_other_tests = if instance_reached.is_empty() {
            None
        } else {
            let other_mentions: Vec<_> = mentions
                .iter()
                .filter(|mention| !instance_mentions.contains(mention))
                .copied()
                .collect();
            self.reachable_from_tests(
                &nodes,
                subjects.as_slice(),
                &other_mentions,
                &property_only,
                instance_reached,
            )
        };
        let index = flow.roots.len();
        let mut read_points: FxHashMap<_, Vec<_>> = FxHashMap::default();
        let mut pattern_elements = FxHashMap::default();
        for read in reads {
            let own = narrowable.contains(&read.subject)
                && reachable.contains(&(read.subject, read.point));
            let pattern = element_pattern(read.subject)
                .filter(|pattern| reachable.contains(&(*pattern, read.point)));
            if !own && pattern.is_none() {
                continue;
            }
            let mut binding = None;
            if own {
                read_points
                    .entry(read.subject)
                    .or_default()
                    .push(read.point);
                if let Some(FlowSubject::Binding(range)) =
                    subjects.as_slice().get(read.subject as usize)
                {
                    let reads = if property_only.contains(&read.subject) {
                        binding = Some(*range);
                        BindingReads::PropertyOnly {
                            instance_tested: instance_tested.contains(&read.subject),
                        }
                    } else if assignment_only.contains(&read.subject) {
                        binding = Some(*range);
                        BindingReads::AssignmentOnly
                    } else {
                        if reached_by_other_tests.as_ref().is_some_and(|reached| {
                            instance_mentions
                                .iter()
                                .any(|(_, mentioned)| *mentioned == read.subject)
                                && !reached.contains(&(read.subject, read.point))
                        }) {
                            binding = Some(*range);
                        }
                        BindingReads::Tested
                    };
                    self.binding_reads
                        .entry(*range)
                        .and_modify(|merged| *merged = merged.merge(reads))
                        .or_insert(reads);
                }
            }
            if let Some(pattern) = pattern {
                read_points.entry(pattern).or_default().push(read.point);
                pattern_elements.insert(read.subject, pattern);
                if let Some(FlowSubject::Pattern(range)) = subjects.as_slice().get(pattern as usize)
                {
                    if let Some(reference) = pattern_types.get(&pattern) {
                        flow.pattern_types.insert(*range, reference.clone());
                    }
                    // A read of an element that is also tested itself is a
                    // candidate for two reasons, so it is never skipped.
                    binding = (!own).then_some(*range);
                }
            }
            let expression = read.expression;
            flow.insert_candidate(&expression, binding);
            for ancestor in expression.evaluated_ancestors() {
                flow.insert_candidate(&ancestor, binding);
            }
            flow.reads.insert(
                expression.range(),
                FlowRead {
                    root: index,
                    point: read.point,
                    subject: read.subject,
                    assumes_initialized: assumed_initialized.contains(&expression.range()),
                },
            );
        }
        // A capture point is a read point of the variable, and of the pattern
        // that holds it, for the nested root that continues from it.
        let mut kept_captures = Vec::new();
        for capture in captured {
            let own = narrowable.contains(&capture.subject)
                && reachable.contains(&(capture.subject, capture.point));
            let pattern = element_pattern(capture.subject)
                .filter(|pattern| reachable.contains(&(*pattern, capture.point)));
            if !own && pattern.is_none() {
                continue;
            }
            if own {
                read_points
                    .entry(capture.subject)
                    .or_default()
                    .push(capture.point);
                if let Some(FlowSubject::Binding(range)) =
                    subjects.as_slice().get(capture.subject as usize)
                {
                    let reads = if property_only.contains(&capture.subject) {
                        BindingReads::PropertyOnly {
                            instance_tested: instance_tested.contains(&capture.subject),
                        }
                    } else if assignment_only.contains(&capture.subject) {
                        BindingReads::AssignmentOnly
                    } else {
                        BindingReads::Tested
                    };
                    self.binding_reads
                        .entry(*range)
                        .and_modify(|merged| *merged = merged.merge(reads))
                        .or_insert(reads);
                }
            }
            if let Some(pattern) = pattern {
                read_points.entry(pattern).or_default().push(capture.point);
                pattern_elements.insert(capture.subject, pattern);
                if let Some(FlowSubject::Pattern(range)) = subjects.as_slice().get(pattern as usize)
                    && let Some(reference) = pattern_types.get(&pattern)
                {
                    flow.pattern_types.insert(*range, reference.clone());
                }
            }
            kept_captures.push(capture);
        }
        if !read_points.is_empty() {
            let property_only: FxHashSet<_> = property_only
                .into_iter()
                .filter(|subject| read_points.contains_key(subject))
                .collect();
            instance_tested.retain(|subject| property_only.contains(subject));
            readonly_tested.retain(|subject| read_points.contains_key(subject));
            only_readonly_tested.retain(|subject| readonly_tested.contains(subject));
            let read_points: FxHashMap<_, _> = read_points
                .into_iter()
                .map(|(subject, mut points)| {
                    points.sort_unstable();
                    points.dedup();
                    (subject, points.into_boxed_slice())
                })
                .collect();
            let (switches, bypassed) = self.bypassed_points(&nodes, switches, &read_points);
            // The values that the writes of each read subject assign, which
            // inference resolves for its solve.
            let mut subject_writes: FxHashMap<FlowSubjectId, Vec<u32>> = FxHashMap::default();
            for write in &writes {
                if let FlowWriteEffect::Assign(Some(value))
                | FlowWriteEffect::CompoundLike(Some(value)) = write.effect
                    && read_points.contains_key(&write.subject)
                {
                    subject_writes.entry(write.subject).or_default().push(value);
                }
            }
            let subject_writes = subject_writes
                .into_iter()
                .map(|(subject, mut values)| {
                    values.sort_unstable();
                    values.dedup();
                    (subject, values.into_boxed_slice())
                })
                .collect();
            let uninitialized = uninitialized
                .into_iter()
                .filter(|subject| read_points.contains_key(subject))
                .collect();
            assignment_only.retain(|subject| read_points.contains_key(subject));
            flow.roots.push(RootFlow {
                nodes: nodes.into_boxed_slice(),
                guards: guards.into_boxed_slice(),
                subjects: subjects.into_subjects().into_boxed_slice(),
                property_only,
                instance_tested,
                pattern_elements,
                readonly_tested,
                only_readonly_tested,
                read_points,
                switches,
                bypassed,
                writes: writes.into_boxed_slice(),
                assigned: assigned.into_boxed_slice(),
                subject_writes,
                uninitialized,
                assignment_only,
                captures: FxHashMap::default(),
            });
            for capture in kept_captures {
                let (holder, subject) = capture.holder;
                if let Some(holder) = flow.roots.get_mut(holder) {
                    holder.captures.entry(subject).or_insert(FlowCapture {
                        root: index,
                        point: capture.point,
                        subject: capture.subject,
                        start: capture.start,
                        assumes_initialized: capture.assumes_initialized,
                    });
                    // The type at the capture point starts where the
                    // variable is declared, `undefined` included.
                    holder.uninitialized.remove(&subject);
                }
            }
        }
    }

    /// Returns every statement of `switches`, in order, and the points of
    /// `read_points` that their bypasses reach, sorted, each with one bit per
    /// statement whose bypass reaches it. Keeping every statement keeps bit
    /// `i` for the [`FlowGuard::Bypass`] with `switch` `i`.
    ///
    /// Returns no statement when no bypass reaches a read, and when the walk
    /// spends the relevance work limit, so no statement counts as exhaustive
    /// in the root.
    fn bypassed_points(
        &self,
        nodes: &[FlowNode],
        switches: Vec<BuiltSwitch>,
        read_points: &FxHashMap<FlowSubjectId, Box<[FlowNodeId]>>,
    ) -> (Box<[FlowSwitch]>, Box<[BypassedPoint]>) {
        if switches.is_empty() {
            return (Box::default(), Box::default());
        }
        let successors = successors(nodes);
        let mut reached = vec![0u64; nodes.len()];
        let mut remaining = self.relevance_budget;
        for (bit, switch) in switches.iter().enumerate() {
            let mask = 1u64 << bit;
            let mut pending = vec![switch.bypass];
            while let Some(node) = pending.pop() {
                let Some(left) = remaining.checked_sub(1) else {
                    return (Box::default(), Box::default());
                };
                remaining = left;
                let Some(bits) = reached.get_mut(node as usize) else {
                    return (Box::default(), Box::default());
                };
                if *bits & mask != 0 {
                    continue;
                }
                *bits |= mask;
                pending.extend(&successors[node as usize]);
            }
        }
        let mut bypassed: Vec<_> = read_points
            .values()
            .flatten()
            .filter_map(|point| {
                let bits = reached[*point as usize];
                (bits != 0).then_some((*point, bits))
            })
            .collect();
        if bypassed.is_empty() {
            return (Box::default(), Box::default());
        }
        bypassed.sort_unstable_by_key(|(point, _)| *point);
        bypassed.dedup_by_key(|(point, _)| *point);
        (
            switches.into_iter().map(|switch| switch.switch).collect(),
            bypassed.into_boxed_slice(),
        )
    }

    /// Returns the collected type of the value that the variable declared at
    /// `element` destructures, as `s` in `const { v } = s`, or `None` when its
    /// collected type destructures no value.
    fn destructured_type(&self, element: TextRange) -> Option<&TypeReference> {
        let TypeReference::Resolved(RawTypeId::Local(id)) = self.raw_binding_types.get(&element)?
        else {
            return None;
        };
        let RawTypeData::TypeofExpression(expression) = self.raw_types.get(id.index())? else {
            return None;
        };
        match expression.as_ref() {
            TypeofExpression::Destructure(destructure) => Some(&destructure.ty),
            _ => None,
        }
    }

    /// Returns the range of `binding` when its reads can narrow, in the root
    /// that declares it and in the roots nested in that one. Each root still
    /// checks the writes of the variable with [`Self::writes_are_modeled`].
    ///
    /// An import narrows too, with the declared type of what it imports. A
    /// `var` narrows unless another declaration redeclares its name, as
    /// [`Self::is_redeclared`] lists. A read can reach a `var` before its
    /// declarator. See [`Self::uninitialized_subjects`].
    fn eligible_range(&mut self, binding: &JsIdentifierBinding) -> Option<TextRange> {
        let range = binding.range();
        // The declared type of a destructured variable ignores defaults, so it
        // can lack the type of a default that supplies the value.
        if has_destructuring_default(binding) {
            return None;
        }
        let declared = self.model.as_binding_by_range(range)?;
        // The semantic index matches the start offset, not the entire range.
        if declared.range() != range {
            return None;
        }
        // TypeScript gives such a variable a type that evolves with its
        // writes, which its declared type in Biome doesn't follow.
        if declared.all_writes().next().is_some() && is_evolving_declaration(&declared) {
            return None;
        }
        // Untyped and `any` bindings never narrow, so their reads need no flow.
        if self
            .raw_binding_types
            .get(&range)
            .is_none_or(|reference| is_raw_narrowing_invariant(reference, self.raw_types))
        {
            return None;
        }
        if declared.declaration_kind() == JsDeclarationKind::HoistedValue
            && self.is_redeclared(&declared, binding)
        {
            return None;
        }
        Some(range)
    }

    /// Returns whether the variable declared at `range` can narrow in a root
    /// nested in the one that declares it, by the rules of
    /// [`Self::eligible_range`]. Computes the result once per variable.
    fn encloses_eligible(&mut self, range: TextRange) -> bool {
        if let Some(eligible) = self.enclosing.get(&range) {
            return *eligible;
        }
        let eligible = self
            .model
            .as_binding_by_range(range)
            .and_then(|binding| JsIdentifierBinding::cast(binding.syntax()))
            .and_then(|binding| self.eligible_range(&binding))
            .is_some();
        self.enclosing.insert(range, eligible);
        eligible
    }

    /// Finds the reads, in the closures that the root at `root_range`
    /// creates, of the variables that it narrows, and gives each read that
    /// continues the root's flow, as TypeScript's `checkIdentifier` lets a
    /// read in a closure do, the capture point of the outermost of those
    /// closures, from `captures`, sorted by start.
    ///
    /// The variables are those of `narrowable`, the root's tested subjects,
    /// and the elements of the patterns among them, which `subjects` and
    /// `eligible` gain. A read continues when [`Self::capture_rule`] lets it,
    /// and every function between it and the root is a closure that
    /// continues the flow where it is created and is created in reachable
    /// code, as [`Self::continues_to`] decides.
    ///
    /// A read that no nested root keeps is added to `reads` at the capture
    /// point, so the root narrows it there, and its range is returned among
    /// those whose variable some code assigns, as
    /// [`FlowRead::assumes_initialized`] describes. For a read that a nested
    /// root keeps, the returned [`PendingCapture`] names the outermost root
    /// that keeps it or continues from it, whose start the capture point
    /// gives.
    #[expect(
        clippy::too_many_arguments,
        reason = "The root's collection state is split across locals of `add_root`."
    )]
    fn closure_reads(
        &mut self,
        flow: &ModuleFlow,
        root_range: TextRange,
        captures: &[BuiltCapture],
        subjects: &mut FlowSubjectTable,
        narrowable: &FxHashSet<FlowSubjectId>,
        eligible: &mut FxHashSet<TextRange>,
        reads: &mut Vec<builder::BuiltRead>,
    ) -> (Vec<PendingCapture>, FxHashSet<TextRange>) {
        let mut variables: Vec<TextRange> = narrowable
            .iter()
            .filter_map(|subject| match subjects.as_slice().get(*subject as usize) {
                Some(FlowSubject::Binding(range)) => Some(*range),
                Some(FlowSubject::This | FlowSubject::Member { .. } | FlowSubject::Pattern(_))
                | None => None,
            })
            .chain(
                subjects
                    .pattern_elements
                    .iter()
                    .filter(|(_, (pattern, _))| narrowable.contains(pattern))
                    .map(|(element, _)| *element),
            )
            .collect();
        variables.sort_unstable_by_key(|range| (range.start(), range.end()));
        variables.dedup();
        let mut pending = Vec::new();
        let mut assumed_initialized = FxHashSet::default();
        for range in variables {
            // An element of a pattern of an enclosing root narrows here only
            // when it can narrow at all.
            if !eligible.contains(&range) {
                if root_range.contains_range(range) || !self.encloses_eligible(range) {
                    continue;
                }
                eligible.insert(range);
            }
            let rule = self.capture_rule(range);
            if rule == CaptureRule::Never {
                continue;
            }
            let assumes_initialized = matches!(rule, CaptureRule::After(_));
            let Some(binding) = self.model.as_binding_by_range(range) else {
                continue;
            };
            let Some(subject) = subjects.intern(FlowSubject::Binding(range)) else {
                continue;
            };
            for reference in binding.all_reads() {
                let start = reference.range_start();
                if !rule.continues_at(start) {
                    continue;
                }
                let index =
                    captures.partition_point(|capture| capture.closure.range().start() <= start);
                let Some(capture) = index.checked_sub(1).and_then(|index| captures.get(index))
                else {
                    continue;
                };
                let closure = capture.closure.range();
                if !closure.contains(start) {
                    continue;
                }
                let Some(identifier) = reference
                    .syntax_near(capture.closure.syntax())
                    .parent()
                    .and_then(JsIdentifierExpression::cast)
                else {
                    continue;
                };
                if !self.continues_to(&identifier, &capture.closure) {
                    continue;
                }
                match flow.read(identifier.range()) {
                    Some(kept) => pending.push(PendingCapture {
                        holder: flow.capture_holder(kept.root, kept.subject),
                        point: capture.point,
                        subject,
                        start: closure.start(),
                        assumes_initialized,
                    }),
                    None => {
                        if assumes_initialized {
                            assumed_initialized.insert(identifier.range());
                        }
                        reads.push(builder::BuiltRead {
                            expression: AnyFlowExpression::JsIdentifierExpression(identifier),
                            point: capture.point,
                            subject,
                        });
                    }
                }
            }
        }
        (pending, assumed_initialized)
    }

    /// Returns whether a read of the variable declared at `range` inside a
    /// closure continues the flow where the closure is created, as
    /// [`CaptureRule`] describes, once per variable.
    ///
    /// As TypeScript's `isPastLastAssignment` decides, a parameter, a `catch`
    /// variable, or a `let` continues past its last assignment in the
    /// function that declares it, as [`assignment_position`] extends each
    /// one, and never when another function assigns it.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Only parameters, catch variables, and `const` and `let` declarations continue."
    )]
    fn capture_rule(&mut self, range: TextRange) -> CaptureRule {
        if let Some(rule) = self.capture_rules.get(&range) {
            return *rule;
        }
        let rule = self
            .model
            .as_binding_by_range(range)
            .map_or(CaptureRule::Never, |binding| {
                if binding.is_imported() {
                    return CaptureRule::Never;
                }
                let Some(declaration) = binding.tree().declaration() else {
                    return CaptureRule::Never;
                };
                match declaration
                    .parent_binding_pattern_declaration()
                    .unwrap_or(declaration)
                {
                    AnyJsBindingDeclaration::JsFormalParameter(_)
                    | AnyJsBindingDeclaration::JsRestParameter(_)
                    | AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
                    | AnyJsBindingDeclaration::TsPropertyParameter(_)
                    | AnyJsBindingDeclaration::JsCatchDeclaration(_) => {}
                    AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                        let Some(declaration) = declarator.declaration() else {
                            return CaptureRule::Never;
                        };
                        match declaration.variable_kind() {
                            Ok(JsVariableKind::Const | JsVariableKind::Using) => {
                                return CaptureRule::Always;
                            }
                            // Other modules or scripts can assign a `let`
                            // that a module exports or a script declares
                            // at its top level.
                            Ok(JsVariableKind::Let)
                                if !binding.is_exported()
                                    && !guards::is_top_level_statement(&declaration) => {}
                            Ok(JsVariableKind::Let | JsVariableKind::Var) | Err(_) => {
                                return CaptureRule::Never;
                            }
                        }
                    }
                    _ => return CaptureRule::Never,
                }
                let declared_at = range.start();
                let declaration = binding.tree();
                let declaring = enclosing_function(&declaration);
                let mut last = None;
                for write in binding.all_writes() {
                    let Some(target) =
                        JsIdentifierAssignment::cast(write.syntax_near(declaration.syntax()))
                    else {
                        return CaptureRule::Never;
                    };
                    if enclosing_function(&target) != declaring {
                        return CaptureRule::Never;
                    }
                    let position = assignment_position(&target, declared_at);
                    last = last.max(Some(position));
                }
                last.map_or(CaptureRule::Always, CaptureRule::After)
            });
        self.capture_rules.insert(range, rule);
        rule
    }

    /// Returns whether the read `identifier` lies in code that runs where a
    /// closure continues the flow of `outermost`, which the root under
    /// collection creates: every function between them must be a closure
    /// that [`AnyContinuingClosure::continues_outer_flow`] accepts, created
    /// in the reachable code of the next one out, and the read must be in
    /// the reachable code of its own. Code that its function doesn't run in
    /// its flow, as a decorator, a computed key of a member, a field
    /// initializer, a type, or a `declare` statement, and the body of a
    /// `with` statement stop it.
    fn continues_to(
        &mut self,
        identifier: &JsIdentifierExpression,
        outermost: &AnyContinuingClosure,
    ) -> bool {
        let mut inner = identifier.range();
        for ancestor in identifier.syntax().ancestors().skip(1) {
            let kind = ancestor.kind();
            if AnyJsControlFlowRoot::can_cast(kind) {
                let Some(closure) = AnyContinuingClosure::cast(ancestor)
                    .filter(AnyContinuingClosure::continues_outer_flow)
                else {
                    return false;
                };
                if !self.reaches(&closure, inner) {
                    return false;
                }
                if closure.syntax() == outermost.syntax() {
                    return true;
                }
                inner = closure.range();
            } else if CLOSURE_BARRIER_KINDS.matches(kind)
                || (JsComputedMemberName::can_cast(kind)
                    && ancestor
                        .parent()
                        .is_some_and(|member| MEMBER_KINDS.matches(member.kind())))
            {
                return false;
            }
        }
        false
    }

    /// Returns whether `closure` reaches the read or the closure at `inner`
    /// in its own code: as its builder found, when a root built it, and
    /// otherwise when no statement that ends every path, as
    /// [`Self::first_exit`] finds it, ends before `inner` starts.
    fn reaches(&mut self, closure: &AnyContinuingClosure, inner: TextRange) -> bool {
        if self.built.contains(&closure.range()) {
            return self.evaluated.contains(&inner);
        }
        self.first_exit(closure) > inner.start()
    }

    /// Returns where the first statement of the own code of `closure` that
    /// ends every path through it ends, or the largest offset when it has
    /// none: `return`, `throw`, `break`, `continue`, and a loop whose test is
    /// missing or a truthy literal, once per closure.
    ///
    /// Code after it may be unreachable, where TypeScript gives a read its
    /// declared type. Code after a statement that only some paths end, such
    /// as `if (c) return;`, counts too, which is wider.
    fn first_exit(&mut self, closure: &AnyContinuingClosure) -> TextSize {
        let range = closure.range();
        if let Some(exit) = self.exits.get(&range) {
            return *exit;
        }
        let mut first = TextSize::from(u32::MAX);
        let mut events = closure.syntax().preorder_of_kinds(EXIT_SCAN_KINDS);
        while let Some(event) = events.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if node == *closure.syntax() {
                continue;
            }
            if AnyJsControlFlowRoot::can_cast(node.kind()) {
                events.skip_subtree();
                continue;
            }
            let ends_paths = match AnyPathEnd::cast_ref(&node) {
                Some(
                    AnyPathEnd::JsReturnStatement(_)
                    | AnyPathEnd::JsThrowStatement(_)
                    | AnyPathEnd::JsBreakStatement(_)
                    | AnyPathEnd::JsContinueStatement(_),
                ) => true,
                Some(AnyPathEnd::JsWhileStatement(statement)) => {
                    statement.test().is_ok_and(|test| is_truthy_literal(&test))
                }
                Some(AnyPathEnd::JsDoWhileStatement(statement)) => {
                    statement.test().is_ok_and(|test| is_truthy_literal(&test))
                }
                Some(AnyPathEnd::JsForStatement(statement)) => {
                    statement.test().is_none_or(|test| is_truthy_literal(&test))
                }
                None => false,
            };
            if ends_paths {
                first = first.min(node.text_trimmed_range().end());
            }
        }
        self.exits.insert(range, first);
        first
    }

    /// Returns whether another declaration redeclares the name of the `var`
    /// that `binding` declares and `declared` stands for in the semantic
    /// model. The other declaration belongs either to the scope that the
    /// `var` is hoisted to, such as a second `var`, a function, or a
    /// parameter of the function, or to a scope between them, such as a catch
    /// variable.
    ///
    /// Such a `var` can't narrow, because the semantic model gives each
    /// declaration its own binding and resolves every reference and write to
    /// the last one, while each declarator's initializer writes the variable
    /// that they all declare.
    fn is_redeclared(&mut self, declared: &Binding, binding: &JsIdentifierBinding) -> bool {
        let Ok(name) = binding.name_token() else {
            return true;
        };
        let name = name.text_trimmed();
        let scope = declared.scope();
        let hoisted = self
            .model
            .scope_hoisted_to(binding.syntax())
            .unwrap_or_else(|| scope.clone());
        if scope
            .ancestors()
            .take_while(|scope| scope.id() != hoisted.id())
            .any(|scope| {
                scope
                    .get_binding(name)
                    .is_some_and(|other| other != *declared)
            })
        {
            return true;
        }
        self.redeclared_in(&hoisted).contains(&declared.range())
    }

    /// Returns the declarations of `scope` whose name another declaration
    /// of `scope` declares, and, when `scope` is the body of a function,
    /// those whose name a parameter declares, once per scope.
    fn redeclared_in(&mut self, scope: &Scope) -> &FxHashSet<TextRange> {
        self.redeclared.entry(scope.id()).or_insert_with(|| {
            let mut redeclared = FxHashSet::default();
            // `get_binding` returns the last declaration of a name, so each
            // earlier declaration of the name marks itself and the last one.
            for binding in scope.bindings() {
                if !binding.declaration_kind().declares_value() {
                    continue;
                }
                let Ok(name) = binding.tree().name_token() else {
                    continue;
                };
                if let Some(last) = scope.get_binding(name.text_trimmed())
                    && last != binding
                {
                    redeclared.insert(binding.range());
                    redeclared.insert(last.range());
                }
            }
            if scope.syntax().kind() == JsSyntaxKind::JS_FUNCTION_BODY
                && let Some(parameters) = scope.parent()
            {
                for parameter in parameters.bindings() {
                    let Ok(name) = parameter.tree().name_token() else {
                        continue;
                    };
                    if let Some(local) = scope.get_binding(name.text_trimmed()) {
                        redeclared.insert(local.range());
                    }
                }
            }
            redeclared
        })
    }

    /// Returns whether the graph of `root` has a write point for every write
    /// of the variable declared at `range` that can run in the root's flow,
    /// given the targets that have one, `written`.
    ///
    /// A write in code that runs only when called, as the body of a callback,
    /// never cancels narrowing, as in TypeScript. Neither does a write outside
    /// `root` to a variable that an enclosing root declares, because it runs
    /// in another function, or in the enclosing root before or after the
    /// code of `root` runs. Any other write without a write point, as in a
    /// function called where it is defined, a class static block, a field
    /// initializer, or unreachable code, makes the variable keep its
    /// declared type in the whole root.
    fn writes_are_modeled(
        &self,
        range: TextRange,
        root: &AnyJsControlFlowRoot,
        written: &WrittenTargets,
    ) -> bool {
        let Some(binding) = self.model.as_binding_by_range(range) else {
            return false;
        };
        let root_range = root.syntax().text_trimmed_range();
        binding.all_writes().all(|write| {
            let start = write.range_start();
            !root_range.contains(start)
                || written.variables.contains(&start)
                || writes::runs_only_when_called(&write.syntax_near(root.syntax()), root)
        })
    }

    /// Returns whether `write` can change the type of its subject at a read,
    /// as far as the collected types tell. An assignment or `delete` changes
    /// only a union, and a compound assignment changes only a type with a
    /// literal member. The declared type of a property path is not collected,
    /// so a write of a path can always change it.
    fn write_may_change(&self, write: &FlowWrite, subjects: &[FlowSubject]) -> bool {
        let declared = match subjects.get(write.subject as usize) {
            Some(FlowSubject::Binding(range)) => self.raw_binding_types.get(range),
            Some(FlowSubject::Member { .. } | FlowSubject::This) => None,
            Some(FlowSubject::Pattern(_)) | None => return false,
        };
        let may_be_union =
            || declared.is_none_or(|reference| raw_may_be_union(reference, self.raw_types));
        let may_have_literal =
            || declared.is_none_or(|reference| raw_may_have_literal(reference, self.raw_types));
        match write.effect {
            FlowWriteEffect::Assign(Some(_)) | FlowWriteEffect::Undefined => may_be_union(),
            FlowWriteEffect::CompoundLike(Some(_)) => may_be_union() || may_have_literal(),
            FlowWriteEffect::CompoundLike(None) | FlowWriteEffect::Compound => may_have_literal(),
            FlowWriteEffect::Assign(None) | FlowWriteEffect::Reset | FlowWriteEffect::Wildcard => {
                false
            }
        }
    }

    /// Returns the subjects of `subjects`, the table of `root`, that may be
    /// read before they are assigned, among the variables of `eligible` and
    /// the paths on `this`, for [`RootFlow::uninitialized`].
    ///
    /// In the root that declares it, a `let` counts when it is declared
    /// without an initializer, outside the head of a `for...in` or `for...of`
    /// loop, and a `var` always does, because its declaration is hoisted
    /// above every declarator. Neither counts with a definite assignment
    /// assertion, as `let x!: T`, or in a `declare` statement. In a root
    /// nested in the one that declares it, a `let` or a `var` counts only
    /// when it has no initializer and no write at all. TypeScript treats such
    /// a `let` the same way but assumes such a `var` initialized there. A
    /// path on `this` counts in a constructor, and in an arrow function that
    /// it calls where it is defined, when its first key names a field of the
    /// constructor's class that is not `static`, has no initializer and no
    /// `!`, and is not declared with `declare` or `accessor`, as in
    /// TypeScript.
    fn uninitialized_subjects(
        &self,
        root: &AnyJsControlFlowRoot,
        subjects: &FlowSubjectTable,
        eligible: &FxHashSet<TextRange>,
    ) -> FxHashSet<FlowSubjectId> {
        let fields =
            subjects::constructor_class(root).map(|class| subjects::uninitialized_fields(&class));
        let this = subjects.get(&FlowSubject::This);
        let root_range = root.syntax().text_trimmed_range();
        (0..)
            .zip(subjects.as_slice())
            .filter(|(_, subject)| match subject {
                FlowSubject::Binding(range) => {
                    eligible.contains(range)
                        && self
                            .model
                            .as_binding_by_range(*range)
                            .is_some_and(|binding| {
                                is_uninitialized_declaration(
                                    &binding,
                                    !root_range.contains_range(*range),
                                )
                            })
                }
                FlowSubject::Member { object, key } => {
                    this == Some(*object)
                        && fields.as_ref().is_some_and(|fields| fields.contains(key))
                }
                FlowSubject::This | FlowSubject::Pattern(_) => false,
            })
            .map(|(id, _)| id)
            .collect()
    }

    /// Returns the `(subject, point)` pairs that a test of the subject can
    /// reach, including the test itself, for each of the `walked` subjects.
    ///
    /// `property_only` holds the subjects that only tests of their properties
    /// narrow. Other variables are walked first, before property paths and
    /// the subjects in `property_only`, so those never take the work limit
    /// from such a variable. When the limit runs out on a path or on a subject
    /// in `property_only`, the result keeps no pair of any of them. When it
    /// runs out on another variable, this returns `None`.
    fn reachable_from_tests(
        &self,
        nodes: &[FlowNode],
        subjects: &[FlowSubject],
        mentions: &[(FlowNodeId, FlowSubjectId)],
        property_only: &FxHashSet<FlowSubjectId>,
        mut walked: Vec<FlowSubjectId>,
    ) -> Option<FxHashSet<(FlowSubjectId, FlowNodeId)>> {
        let successors = successors(nodes);
        // Paths and subjects in `property_only` lose their reads first.
        let is_secondary = |subject: FlowSubjectId| {
            property_only.contains(&subject)
                || matches!(
                    subjects.get(subject as usize),
                    Some(FlowSubject::Member { .. })
                )
        };
        walked.sort_unstable_by_key(|subject| (is_secondary(*subject), *subject));
        walked.dedup();
        let mut reachable = FxHashSet::default();
        let mut remaining = self.relevance_budget;
        'subjects: for subject in walked {
            let mut seen = vec![false; nodes.len()];
            let mut pending: Vec<FlowNodeId> = mentions
                .iter()
                .filter(|(_, mentioned)| *mentioned == subject)
                .map(|(node, _)| *node)
                .collect();
            while let Some(node) = pending.pop() {
                let Some(left) = remaining.checked_sub(1) else {
                    if !is_secondary(subject) {
                        return None;
                    }
                    reachable.retain(|(subject, _)| !is_secondary(*subject));
                    break 'subjects;
                };
                remaining = left;
                if std::mem::replace(&mut seen[node as usize], true) {
                    continue;
                }
                reachable.insert((subject, node));
                pending.extend(&successors[node as usize]);
            }
        }
        Some(reachable)
    }
}

/// Returns the nodes that each node of `nodes` flows directly into.
fn successors(nodes: &[FlowNode]) -> Vec<Vec<FlowNodeId>> {
    let mut successors = vec![Vec::new(); nodes.len()];
    for (index, node) in (0..).zip(nodes) {
        match node {
            FlowNode::Start => {}
            FlowNode::Join(predecessors) => {
                for predecessor in predecessors {
                    successors[*predecessor as usize].push(index);
                }
            }
            FlowNode::Condition { antecedent, .. } | FlowNode::Write { antecedent, .. } => {
                successors[*antecedent as usize].push(index);
            }
        }
    }
    successors
}

/// Replaces with [`FlowGuard::Keep`] each [`FlowGuard::ReadonlyPath`] guard in
/// `guards` whose path has a replaceable object. Returns the subjects of the
/// other such guards, then the subset of them that no other guard tests or
/// narrows. `subjects` are the root's subjects, each at the index of its id.
///
/// An object is replaceable when a test of the root can give it a type with
/// members that its declared type lacks. `instanceof`, a type-guard function,
/// and `typeof` with a primitive kind or `"function"` can replace a member
/// with another type, as `o instanceof Derived` replaces `Base` with
/// `Derived`, whose property can be mutable where `Base` declares it readonly.
/// Inference decides whether a path is readonly from the declared types of the
/// objects along it, while TypeScript decides it from their types at the
/// read. Both agree while the tests of those objects only remove members
/// from their types. Biome's narrowed types can't stand in for TypeScript's,
/// because Biome keeps a member that TypeScript replaces when, for example,
/// it can't tell whether the member derives from the tested class.
fn readonly_tested_subjects(
    guards: &mut [FlowGuard],
    subjects: &[FlowSubject],
) -> (FxHashSet<FlowSubjectId>, FxHashSet<FlowSubjectId>) {
    let tested = |guard: &FlowGuard| match guard {
        FlowGuard::Test { subject, .. } => Some(*subject),
        FlowGuard::Property { object, .. } => Some(*object),
        FlowGuard::Keep
        | FlowGuard::Incomplete
        | FlowGuard::ReadonlyPath(_)
        | FlowGuard::Both { .. }
        | FlowGuard::Bypass { .. } => None,
    };
    let mut replaced = FxHashSet::default();
    let mut wrapped = FxHashSet::default();
    for guard in guards.iter() {
        match guard {
            FlowGuard::Test {
                subject,
                test:
                    FlowTest::Instanceof(_)
                    | FlowTest::Predicate { .. }
                    | FlowTest::Typeof(
                        TypeofKind::BigInt
                        | TypeofKind::Boolean
                        | TypeofKind::Function
                        | TypeofKind::Number
                        | TypeofKind::String
                        | TypeofKind::Symbol,
                    ),
                ..
            } => {
                replaced.insert(*subject);
            }
            FlowGuard::ReadonlyPath(inner) => {
                wrapped.insert(*inner);
            }
            _ => {}
        }
    }
    let has_replaced_object = |mut subject: FlowSubjectId| {
        while let Some(FlowSubject::Member { object, .. }) = subjects.get(subject as usize) {
            // A table stores an object before its members, so a later object
            // means the table is malformed, and the guard is dropped.
            if replaced.contains(object) || *object >= subject {
                return true;
            }
            subject = *object;
        }
        false
    };
    let mut readonly_tested = FxHashSet::default();
    for index in 0..guards.len() {
        let FlowGuard::ReadonlyPath(inner) = guards[index] else {
            continue;
        };
        match guards.get(inner).and_then(tested) {
            Some(subject) if !has_replaced_object(subject) => {
                readonly_tested.insert(subject);
            }
            _ => guards[index] = FlowGuard::Keep,
        }
    }
    let mut only_readonly_tested = readonly_tested.clone();
    for (index, guard) in guards.iter().enumerate() {
        if !wrapped.contains(&index)
            && let Some(subject) = tested(guard)
        {
            only_readonly_tested.remove(&subject);
        }
    }
    (readonly_tested, only_readonly_tested)
}

/// Code inside a closure that the function holding it doesn't run in its own
/// flow, or where names may not mean what the semantic model resolves:
/// class members that are not functions, such as fields, decorators, types,
/// `declare` statements, and `with` statements.
const CLOSURE_BARRIER_KINDS: SyntaxKindSet<JsLanguage> = AnyJsClassMember::KIND_SET
    .union(JsDecorator::KIND_SET)
    .union(AnyTsType::KIND_SET)
    .union(TsDeclareStatement::KIND_SET)
    .union(JsWithStatement::KIND_SET);

/// Class members and object members that are functions, whose computed key
/// runs where their class or object is created, not in their own flow.
const MEMBER_KINDS: SyntaxKindSet<JsLanguage> =
    AnyJsClassMember::KIND_SET.union(AnyJsControlFlowRoot::KIND_SET);

declare_node_union! {
    /// A statement after which every path may end: a jump, or a loop that
    /// [`RootCollector::first_exit`] finds infinite.
    AnyPathEnd = JsReturnStatement
        | JsThrowStatement
        | JsBreakStatement
        | JsContinueStatement
        | JsWhileStatement
        | JsDoWhileStatement
        | JsForStatement
}

/// The statements that [`RootCollector::first_exit`] looks for, and the
/// nested roots it skips.
const EXIT_SCAN_KINDS: SyntaxKindSet<JsLanguage> =
    AnyPathEnd::KIND_SET.union(AnyJsControlFlowRoot::KIND_SET);

/// The roots that TypeScript's `isFunctionLike` accepts: an assignment in
/// another one, or in a nested one, is in another function. Namespaces and
/// class static blocks belong to the function around them.
const FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = AnyJsFunction::KIND_SET
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET)
    .union(JsMethodClassMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsConstructorClassMember::KIND_SET);

/// The statements that TypeScript's `extendAssignmentPosition` extends an
/// assignment to the end of: variable and expression statements, `if`,
/// loops, `with`, `switch`, `try`, and class declarations, including the
/// exported and default-exported forms.
const ASSIGNMENT_EXTENDING_KINDS: SyntaxKindSet<JsLanguage> = JsVariableStatement::KIND_SET
    .union(JsVariableDeclarationClause::KIND_SET)
    .union(JsExpressionStatement::KIND_SET)
    .union(JsIfStatement::KIND_SET)
    .union(JsDoWhileStatement::KIND_SET)
    .union(JsWhileStatement::KIND_SET)
    .union(JsForStatement::KIND_SET)
    .union(JsForInStatement::KIND_SET)
    .union(JsForOfStatement::KIND_SET)
    .union(JsWithStatement::KIND_SET)
    .union(JsSwitchStatement::KIND_SET)
    .union(JsTryStatement::KIND_SET)
    .union(JsTryFinallyStatement::KIND_SET)
    .union(JsClassDeclaration::KIND_SET)
    .union(JsClassExportDefaultDeclaration::KIND_SET);

/// Returns the function that holds `node`, as [`FUNCTION_KINDS`] lists them,
/// or `None` for code outside every function.
fn enclosing_function(node: &impl AstNode<Language = JsLanguage>) -> Option<JsSyntaxNode> {
    node.syntax()
        .ancestors()
        .find(|ancestor| FUNCTION_KINDS.matches(ancestor.kind()))
}

/// Returns the position that the assignment of the variable `target` gives
/// its last assignment, as TypeScript's `extendAssignmentPosition`
/// does: the end of the outermost statement of [`ASSIGNMENT_EXTENDING_KINDS`]
/// around `target` that starts after `declared_at`, where the variable is
/// declared, or the start of `target` without one.
///
/// So a read in a closure created inside such a statement, as in
/// `if (c) { x = "a"; const h = () => x; }`, comes before the position and
/// starts from the declared type, as in TypeScript, while one created after
/// it, as in `{ x = "a"; } const h = () => x;`, comes after it.
fn assignment_position(target: &JsIdentifierAssignment, declared_at: TextSize) -> TextSize {
    let mut position = target.range().start();
    for ancestor in target.syntax().ancestors().skip(1) {
        let range = ancestor.text_trimmed_range();
        if range.start() <= declared_at {
            break;
        }
        if ASSIGNMENT_EXTENDING_KINDS.matches(ancestor.kind()) {
            position = range.end();
        }
    }
    position
}

/// Binding patterns, with their elements and lists, which can enclose a binding.
const BINDING_PATTERN_KINDS: SyntaxKindSet<JsLanguage> = JsArrayBindingPattern::KIND_SET
    .union(JsArrayBindingPatternElement::KIND_SET)
    .union(JsArrayBindingPatternElementList::KIND_SET)
    .union(JsArrayBindingPatternRestElement::KIND_SET)
    .union(JsObjectBindingPattern::KIND_SET)
    .union(JsObjectBindingPatternProperty::KIND_SET)
    .union(JsObjectBindingPatternPropertyList::KIND_SET)
    .union(JsObjectBindingPatternRest::KIND_SET)
    .union(JsObjectBindingPatternShorthandProperty::KIND_SET);

/// Returns whether TypeScript gives the variable `binding` a type that evolves
/// with its writes. Such a variable is a `let` or a `var` without a type
/// annotation whose initializer is `null`, `undefined`, or an empty array
/// literal. A variable without an initializer has no collected type in Biome.
fn is_evolving_declaration(binding: &Binding) -> bool {
    let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
        binding.tree().declaration()
    else {
        return false;
    };
    if declarator.variable_annotation().is_some() {
        return false;
    }
    let Some(initializer) = declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
    else {
        return false;
    };
    match initializer.omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNullLiteralExpression(_),
        ) => true,
        AnyJsExpression::JsArrayExpression(array) => array.elements().is_empty(),
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .ok()
            .and_then(|name| name.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == "undefined"),
        _ => false,
    }
}

/// Returns whether `binding` is declared by a `let` or a `var` that may be
/// read before it is assigned, in the root that declares it, or in a root
/// nested in that one when `enclosed` is true. See
/// [`RootCollector::uninitialized_subjects`] for the conditions.
fn is_uninitialized_declaration(binding: &Binding, enclosed: bool) -> bool {
    let Some(declaration) = binding.tree().declaration() else {
        return false;
    };
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration)
    else {
        return false;
    };
    if matches!(
        declarator.variable_annotation(),
        Some(AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(_))
    ) {
        return false;
    }
    // The head of a `for...in` or `for...of` loop assigns its variables on
    // every iteration, before the body.
    let (kind, initialized) = match declarator.declaration() {
        Some(AnyJsVariableDeclaration::JsVariableDeclaration(declaration)) => {
            if declaration
                .parent::<JsVariableDeclarationClause>()
                .and_then(|clause| clause.parent::<TsDeclareStatement>())
                .is_some()
            {
                return false;
            }
            (
                declaration.variable_kind(),
                declarator.initializer().is_some(),
            )
        }
        Some(AnyJsVariableDeclaration::JsForVariableDeclaration(declaration)) => {
            (declaration.variable_kind(), true)
        }
        None => return false,
    };
    let never_assigned = || !initialized && binding.all_writes().next().is_none();
    match kind {
        Ok(JsVariableKind::Let) if enclosed => never_assigned(),
        Ok(JsVariableKind::Let) => !initialized,
        Ok(JsVariableKind::Var) => !enclosed || never_assigned(),
        Ok(JsVariableKind::Const | JsVariableKind::Using) | Err(_) => false,
    }
}

/// Returns whether a destructuring default can supply the value of `binding`,
/// either its own default, as in `{ a = 1 }`, or the default of an enclosing
/// pattern, as in `{ pair: [a] = [1] }`.
fn has_destructuring_default(binding: &JsIdentifierBinding) -> bool {
    binding
        .syntax()
        .ancestors()
        .skip(1)
        .take_while(|node| BINDING_PATTERN_KINDS.matches(node.kind()))
        .any(|node| {
            JsArrayBindingPatternElement::cast_ref(&node)
                .is_some_and(|element| element.init().is_some())
                || JsObjectBindingPatternProperty::cast_ref(&node)
                    .is_some_and(|property| property.init().is_some())
                || JsObjectBindingPatternShorthandProperty::cast_ref(&node)
                    .is_some_and(|property| property.init().is_some())
        })
}

#[cfg(test)]
mod tests {
    use crate::js_module_info::{JsModuleInfo, JsModuleVisitor, TypeInferenceMode};
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_languages::JsFileSource;
    use biome_rowan::{AstNode, TextRange, TextSize, WalkEvent};
    use camino::Utf8PathBuf;
    use std::sync::Arc;

    fn module(source: &str) -> JsModuleInfo {
        module_at(source, "/index.ts")
    }

    /// Collects the TypeScript `source` as if it were the module at `path`.
    fn module_at(source: &str, path: &str) -> JsModuleInfo {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        let root = parsed.tree();
        let model = Arc::new(semantic_model(&root, SemanticModelOptions::default()));
        JsModuleVisitor::new(
            root,
            Utf8PathBuf::from(path),
            model,
            TypeInferenceMode::RawTypesOnly,
        )
        .collect_info()
    }

    fn marked(source: &str, marker: &str, expression: &str) -> TextRange {
        let start = source.find(&format!("/*{marker}*/")).unwrap() + marker.len() + 4;
        assert!(source[start..].starts_with(expression));
        TextRange::at(
            TextSize::try_from(start).unwrap(),
            TextSize::try_from(expression.len()).unwrap(),
        )
    }

    #[test]
    fn eval_and_arguments_in_nested_functions_keep_enclosing_flow() {
        for closure in ["eval('value = null')", "arguments"] {
            let source = format!(
                "function f(value: string | null) {{ const g = () => {closure}; if (value !== null) {{ /*read*/value; }} }}"
            );
            let info = module(&source);
            assert!(
                info.flow.is_candidate(marked(&source, "read", "value")),
                "{closure}"
            );
        }
    }

    #[test]
    fn nested_roots_narrow_the_variables_of_enclosing_roots_by_their_own_tests() {
        const SOURCE: &str = r#"
            function f(outer: string | null, tested: string | null) {
                const g = (own: string | null) =>
                    own !== null && outer !== null && /*own*/own && /*outer*/outer;
                if (tested !== null) {
                    const h = () => /*captured*/tested;
                    function k() { /*declared*/tested; }
                }
            }
        "#;
        let info = module(SOURCE);
        // A test in `f` reaches a read in a closure that `f` creates, but
        // not one in a function declaration, which starts its own flow.
        for (marker, expression, expected) in [
            ("own", "own", true),
            ("outer", "outer", true),
            ("captured", "tested", true),
            ("declared", "tested", false),
        ] {
            assert_eq!(
                info.flow.is_candidate(marked(SOURCE, marker, expression)),
                expected,
                "{marker}"
            );
        }
    }

    #[test]
    fn declared_function_names_belong_to_the_enclosing_root() {
        for source in [
            "declare function g(): void; if (!g) { /*read*/g; }",
            "export declare function g(): void; if (!g) { /*read*/g; }",
            "function g(): void; if (!g) { /*read*/g; }",
        ] {
            let read = marked(source, "read", "g");
            assert!(module(source).flow.is_candidate(read), "{source}");
        }
    }

    #[test]
    fn declaration_files_get_no_flow() {
        const SOURCE: &str =
            "function f(value: string | null) { if (value !== null) { /*read*/value; } }";
        let read = marked(SOURCE, "read", "value");
        // A path without a known file source still gets flow.
        for path in ["/index.ts", "/index"] {
            assert!(module_at(SOURCE, path).flow.is_candidate(read), "{path}");
        }
        for path in ["/index.d.ts", "/index.d.mts", "/index.d.cts", "/INDEX.D.TS"] {
            assert!(!module_at(SOURCE, path).flow.is_candidate(read), "{path}");
        }
    }

    #[test]
    fn roots_over_the_relevance_budget_keep_no_reads() {
        const SOURCE: &str =
            "function f(value: string | null) { if (value !== null) { /*read*/value; } }";
        let info = module(SOURCE);
        let read = marked(SOURCE, "read", "value");
        assert!(info.flow.is_candidate(read));
        let mut roots = super::FlowRootScanner::default();
        for event in info.semantic_model.root().syntax().preorder() {
            match event {
                WalkEvent::Enter(node) => roots.enter(&node),
                WalkEvent::Leave(node) => roots.leave(&node),
            }
        }
        let flow = super::ModuleFlow::collect_with_relevance_budget(
            roots,
            &info.semantic_model,
            &info.raw_types,
            &info.raw_binding_types,
            0,
        );
        assert!(!flow.is_candidate(read));
    }

    #[test]
    fn a_budget_spent_on_paths_keeps_the_variables() {
        const SOURCE: &str = "function f(x: string | null, o: { v: string | null }) { if (x !== null && o.v !== null) { /*x*/x; /*v*/o.v; } }";
        let info = module(SOURCE);
        let (x, v) = (marked(SOURCE, "x", "x"), marked(SOURCE, "v", "o.v"));
        assert!(info.flow.is_candidate(x) && info.flow.is_candidate(v));
        let collect = |budget| {
            let mut roots = super::FlowRootScanner::default();
            for event in info.semantic_model.root().syntax().preorder() {
                match event {
                    WalkEvent::Enter(node) => roots.enter(&node),
                    WalkEvent::Leave(node) => roots.leave(&node),
                }
            }
            super::ModuleFlow::collect_with_relevance_budget(
                roots,
                &info.semantic_model,
                &info.raw_types,
                &info.raw_binding_types,
                budget,
            )
        };
        // The smallest budget that keeps the variable leaves nothing for the
        // path, so collection drops the path and keeps the variable.
        let budget = (0..1024)
            .find(|budget| collect(*budget).is_candidate(x))
            .unwrap();
        let flow = collect(budget);
        assert!(!flow.is_candidate(v));
        assert!(!collect(budget - 1).is_candidate(x));
    }

    #[test]
    fn paths_on_this_narrow_outside_static_members() {
        const SOURCE: &str = r#"
            class K {
                x: string | null = null;
                static s: string | null = null;
                m() { if (this.x) { /*method*/this.x; } }
                constructor() {
                    if (this.x) { /*constructor*/this.x; }
                    const g = () => { if (this.x) { /*arrow*/this.x; } };
                }
                static n() { if (this.s) { /*static*/this.s; } }
                static { if (this.s) { /*block*/this.s; } }
                f = () => { if (this.x) { /*field*/this.x; } };
            }
        "#;
        let info = module(SOURCE);
        for (marker, expression, expected) in [
            ("method", "this.x", true),
            ("constructor", "this.x", true),
            ("arrow", "this.x", true),
            ("static", "this.s", false),
            ("block", "this.s", false),
            ("field", "this.x", true),
        ] {
            assert_eq!(
                info.flow.is_candidate(marked(SOURCE, marker, expression)),
                expected,
                "{marker}"
            );
        }
    }

    #[test]
    fn targets_of_writes_and_deletes_are_not_reads() {
        const SOURCE: &str = r#"
            function f(o: { v: { w: string | null } | null; n: number | null }) {
                if (o.v !== null && o.n !== null) {
                    /*object*/o.v.w = null;
                    o.n += 1;
                    delete /*deleted*/o.n;
                    /*after*/o.n;
                }
            }
        "#;
        let info = module(SOURCE);
        // The object of a target is read, and a path is read after the
        // writes of it.
        for (marker, expression, expected) in [
            ("object", "o.v", true),
            ("object", "o.v.w", false),
            ("deleted", "o.n", false),
            ("after", "o.n", true),
        ] {
            assert_eq!(
                info.flow.read(marked(SOURCE, marker, expression)).is_some(),
                expected,
                "{marker}: {expression}"
            );
        }
    }

    #[test]
    fn objects_of_tested_properties_are_property_only_subjects() {
        const SOURCE: &str = r#"
            function f(s: { k: "a" } | { k: "b" }, t: { k: "a" } | { k: "b" } | null) {
                if (s.k === "a") { /*s*/s; }
                if (t && t.k === "a") { /*t*/t; }
            }
        "#;
        let info = module(SOURCE);
        let reads: Vec<_> = ["s", "t"]
            .into_iter()
            .map(|marker| info.flow.read(marked(SOURCE, marker, marker)).unwrap())
            .collect();
        let root = info.flow.root(reads[0].root).unwrap();
        // `s` is tested only through `s.k`, while `t` is also tested itself.
        assert!(root.is_property_only(reads[0].subject));
        assert!(!root.is_property_only(reads[1].subject));
    }

    #[test]
    fn variables_read_before_they_are_assigned_can_be_assignment_only() {
        const SOURCE: &str = r#"
            declare function read(): number;
            function f() { var n = read(); n = read(); /*n*/n; }
        "#;
        let info = module(SOURCE);
        let read = info.flow.read(marked(SOURCE, "n", "n")).unwrap();
        let root = info.flow.root(read.root).unwrap();
        assert!(root.is_uninitialized(read.subject));
        assert!(root.is_assignment_only(read.subject));
    }

    #[test]
    fn variables_read_in_several_roots_are_property_only_when_they_are_in_each() {
        const SOURCE: &str = r#"
            declare const s: { k: "a" } | { k: "b" };
            declare const t: { k: "a" } | { k: "b" };
            function shape() { if (s.k === "a") { /*s*/s; } if (t.k === "a") { /*t*/t; } }
            function tested() { if (typeof s === "object") { s; } if (t.k === "b") { t; } }
        "#;
        let info = module(SOURCE);
        let binding = |name: &str| {
            let read = info.flow.read(marked(SOURCE, name, name)).unwrap();
            info.flow
                .root(read.root)
                .and_then(|root| root.subject_binding(read.subject))
                .unwrap()
        };
        // Narrowing of a variable starts from one type in every root. The
        // test of `s` itself in `tested` needs that type, so `s` is not
        // property-only in `shape` either.
        assert!(!info.flow.is_property_only_binding(binding("s")));
        assert!(info.flow.is_property_only_binding(binding("t")));
    }

    #[test]
    fn candidates_made_only_by_reads_of_a_property_only_binding_name_it() {
        const SOURCE: &str = r#"
            declare function g(...values: unknown[]): void;
            function f(s: { k: "a" } | { k: "b" }, x: string | null) {
                if (s.k === "a" && x) { /*read*/s; /*call*/g(s); /*shared*/g(s, x); /*other*/x; }
            }
        "#;
        let info = module(SOURCE);
        let read = info.flow.read(marked(SOURCE, "read", "s")).unwrap();
        let Some(&super::FlowSubject::Binding(binding)) =
            info.flow.root(read.root).unwrap().subject(read.subject)
        else {
            panic!("`s` must be a binding subject");
        };
        assert!(info.flow.is_property_only_binding(binding));
        // A candidate that also reads `x` must be evaluated even when no test
        // of a property can narrow `s`.
        for (marker, expression, named) in [
            ("read", "s", true),
            ("call", "g(s)", true),
            ("shared", "g(s, x)", false),
            ("other", "x", false),
        ] {
            let range = marked(SOURCE, marker, expression);
            assert!(info.flow.is_candidate(range), "{marker}");
            assert_eq!(
                info.flow.property_only_candidate(range),
                named.then_some(binding),
                "{marker}"
            );
        }
    }

    #[test]
    fn objects_of_held_properties_are_property_only_subjects() {
        const SOURCE: &str = r#"
            function f(s: { k: "a" } | { k: "b" }, o: { v: string | null }) {
                const { k } = s;
                const v = o.v;
                if (k === "a" && v) { /*s*/s; /*o*/o; }
            }
        "#;
        let info = module(SOURCE);
        let read = info.flow.read(marked(SOURCE, "s", "s")).unwrap();
        let root = info.flow.root(read.root).unwrap();
        assert!(root.is_property_only(read.subject));
        // The annotation of `o` shows that it is not a union, so its read is
        // dropped without resolving anything.
        assert!(!info.flow.is_candidate(marked(SOURCE, "o", "o")));
    }

    #[test]
    fn subjects_of_in_are_property_only_subjects() {
        const SOURCE: &str = r#"
            function f(x: { a: 1 } | { b: 2 }, y: { a: 1 } | { b: 2 } | null, z: string | { a: 1 }) {
                if ("a" in x) { /*x*/x; }
                if (y && "a" in y) { /*y*/y; }
                if ("a" in z) { /*z*/z; }
            }
        "#;
        let info = module(SOURCE);
        let read = info.flow.read(marked(SOURCE, "x", "x")).unwrap();
        let root = info.flow.root(read.root).unwrap();
        // `x` is tested only by `in`, while `y` is also tested itself.
        assert!(root.is_property_only(read.subject));
        let Some(&super::FlowSubject::Binding(binding)) = root.subject(read.subject) else {
            panic!("`x` must be a binding subject");
        };
        assert_eq!(
            info.flow.property_only_candidate(marked(SOURCE, "x", "x")),
            Some(binding)
        );
        let read = info.flow.read(marked(SOURCE, "y", "y")).unwrap();
        assert!(!root.is_property_only(read.subject));
        // The annotation of `z` shows that it has one object type, so its read
        // is dropped without resolving anything.
        assert!(!info.flow.is_candidate(marked(SOURCE, "z", "z")));
    }

    #[test]
    fn subjects_of_instanceof_are_property_only_subjects() {
        const SOURCE: &str = r#"
            function f(x: Date | string, y: Date | null, z: string | null, w: unknown, v: Date | { a: 1 }) {
                if (x instanceof Date) { /*x*/x; }
                if (y && y instanceof Date) { /*y*/y; }
                if (z instanceof Date) { /*z*/z; }
                if (w instanceof Date) { /*w*/w; }
                if (v instanceof Date || "a" in v) { /*v*/v; }
            }
        "#;
        let info = module(SOURCE);
        let read = info.flow.read(marked(SOURCE, "x", "x")).unwrap();
        let root = info.flow.root(read.root).unwrap();
        // `x` is tested only by `instanceof`, which also narrows a type with
        // a single object type, while `y` is also tested itself.
        assert!(root.is_property_only(read.subject));
        assert!(root.is_instance_tested(read.subject));
        let Some(&super::FlowSubject::Binding(binding)) = root.subject(read.subject) else {
            panic!("`x` must be a binding subject");
        };
        assert!(info.flow.is_instance_tested_binding(binding));
        assert_eq!(
            info.flow.property_only_candidate(marked(SOURCE, "x", "x")),
            Some(binding)
        );
        let read = info.flow.read(marked(SOURCE, "y", "y")).unwrap();
        assert!(!root.is_property_only(read.subject));
        // The annotation of `z` shows that it has no object type, so its read
        // is dropped without resolving anything.
        assert!(!info.flow.is_candidate(marked(SOURCE, "z", "z")));
        let read = info.flow.read(marked(SOURCE, "w", "w")).unwrap();
        assert!(root.is_instance_tested(read.subject));
        // `in` also tests `v`, whose single object type is enough for
        // `instanceof`.
        let read = info.flow.read(marked(SOURCE, "v", "v")).unwrap();
        assert!(root.is_property_only(read.subject));
        assert!(root.is_instance_tested(read.subject));
    }

    #[test]
    fn subjects_of_type_predicates_with_object_targets_are_property_only_subjects() {
        const SOURCE: &str = r#"
            import type { Imported } from "./types";
            interface Foo { a: number }
            type Alias = Foo | { b: 1 };
            declare function isFoo(v: unknown): v is Foo;
            declare function isAlias(v: unknown): v is Alias;
            declare function isText(v: unknown): v is string | number;
            declare function isImported(v: unknown): v is Imported;
            function f(x: Foo | string, y: Foo | null, z: string | null, w: unknown, a: string | null, t: string | null, i: string | null) {
                if (isFoo(x)) { /*x*/x; }
                if (y && isFoo(y)) { /*y*/y; }
                if (isFoo(z)) { /*z*/z; }
                if (isFoo(w)) { /*w*/w; }
                if (isAlias(a)) { /*a*/a; }
                if (isText(t)) { /*t*/t; }
                if (isImported(i)) { /*i*/i; }
            }
        "#;
        let info = module(SOURCE);
        let read = info.flow.read(marked(SOURCE, "x", "x")).unwrap();
        let root = info.flow.root(read.root).unwrap();
        // Like `instanceof`, a target whose members are all object types
        // narrows only a type with an object type or `unknown`.
        assert!(root.is_property_only(read.subject));
        assert!(root.is_instance_tested(read.subject));
        let read = info.flow.read(marked(SOURCE, "y", "y")).unwrap();
        assert!(!root.is_property_only(read.subject));
        let read = info.flow.read(marked(SOURCE, "w", "w")).unwrap();
        assert!(root.is_instance_tested(read.subject));
        // The annotations of `z` and `a` show that they have no object type,
        // so their reads are dropped without resolving anything. This also
        // holds for `a`, whose type-guard function targets an alias of a
        // union of object types.
        for marker in ["z", "a"] {
            assert!(
                !info.flow.is_candidate(marked(SOURCE, marker, marker)),
                "{marker}"
            );
        }
        // A target that may have a primitive member, or that names an import,
        // which can be an alias of a primitive, narrows every type.
        for marker in ["t", "i"] {
            let read = info.flow.read(marked(SOURCE, marker, marker)).unwrap();
            assert!(!root.is_property_only(read.subject), "{marker}");
        }
    }

    #[test]
    fn reads_that_only_instanceof_reaches_are_candidates_named_by_their_variable() {
        const SOURCE: &str = r#"
            declare function g(...values: unknown[]): unknown;
            function f() {
                const x = g();
                if (x instanceof Date) { /*inside*/x; }
                if (!x) { return; }
                /*after*/x;
            }
        "#;
        let info = module(SOURCE);
        let inside = marked(SOURCE, "inside", "x");
        let read = info.flow.read(inside).unwrap();
        let root = info.flow.root(read.root).unwrap();
        assert!(!root.is_property_only(read.subject));
        let Some(&super::FlowSubject::Binding(binding)) = root.subject(read.subject) else {
            panic!("`x` must be a binding subject");
        };
        // Only the `instanceof` test reaches the first read, which narrows
        // only when the type of `x` can, so inference can skip it.
        assert_eq!(info.flow.property_only_candidate(inside), Some(binding));
        assert!(!info.flow.is_property_only_binding(binding));
        // The truthiness test reaches the second read.
        let after = marked(SOURCE, "after", "x");
        assert!(info.flow.is_candidate(after));
        assert_eq!(info.flow.property_only_candidate(after), None);
    }

    #[test]
    fn reads_of_elements_of_tested_patterns_are_candidates_named_by_the_pattern() {
        const SOURCE: &str = r#"
            declare function g(...values: unknown[]): void;
            type S = { kind: "a"; v: 1 } | { kind: "b"; v: 2 };
            function f({ kind, v }: S, x: string | null) {
                if (kind === "a" && x) { /*v*/v; /*kind*/kind; /*call*/g(v); /*shared*/g(v, x); }
            }
        "#;
        let info = module(SOURCE);
        let read = info.flow.read(marked(SOURCE, "v", "v")).unwrap();
        let root = info.flow.root(read.root).unwrap();
        let pattern = root.element_pattern(read.subject).unwrap();
        let Some(&super::FlowSubject::Pattern(range)) = root.subject(pattern) else {
            panic!("an element must map to its pattern");
        };
        assert_eq!(&SOURCE[range], "{ kind, v }");
        assert!(root.is_property_only(pattern));
        assert!(info.flow.pattern_type(range).is_some());
        // The read of `v` is a read point of the pattern, not of `v`, which
        // no condition tests.
        assert!(root.reads_at(pattern, read.point));
        assert!(!root.reads_at(read.subject, read.point));
        // `kind` is tested itself, so its read is a candidate for two reasons.
        for (marker, expression, named) in [
            ("v", "v", true),
            ("kind", "kind", false),
            ("call", "g(v)", true),
            ("shared", "g(v, x)", false),
        ] {
            let candidate = marked(SOURCE, marker, expression);
            assert!(info.flow.is_candidate(candidate), "{marker}");
            assert_eq!(
                info.flow.property_only_candidate(candidate),
                named.then_some(range),
                "{marker}"
            );
        }
    }

    #[test]
    fn elements_of_patterns_that_cannot_narrow_are_not_candidates() {
        const SOURCE: &str = r#"
            type S = { kind: "a"; v: 1 } | { kind: "b"; v: 2 };
            function g() { return 1 as const; }
            function mutable(s: S) { let { kind, v } = s; if (kind === "a") { /*mutable*/v; } }
            function single(s: S) { const { kind } = s; const { v } = s; if (kind === "a") { /*single*/v; } }
            function written({ kind, v }: S) { v = g(); if (kind === "a") { /*written*/v; } }
            function enclosingDefault(w: { a?: S }, d: S) { const { a: { kind, v } = d } = w; if (kind === "a") { /*enclosingDefault*/v; } }
            function closure({ kind, v }: S) { const h = () => { if (kind === "a") { /*closure*/v; } }; }
            function notUnion({ a, b }: { a: string | null; b: number }) { if (a) { /*notUnion*/b; } }
            function named({ kind, v }: S) { if (kind === "a") { /*named*/v; } }
        "#;
        let info = module(SOURCE);
        for (marker, expected) in [
            ("mutable", false),
            ("single", false),
            ("written", false),
            ("enclosingDefault", false),
            // A test in a closure narrows a pattern of the enclosing root.
            ("closure", true),
            // The annotation shows that the value is not a union of objects.
            ("notUnion", false),
            ("named", true),
        ] {
            let expression = if marker == "notUnion" { "b" } else { "v" };
            assert_eq!(
                info.flow.is_candidate(marked(SOURCE, marker, expression)),
                expected,
                "{marker}"
            );
        }
    }

    #[test]
    fn held_properties_that_the_root_writes_narrow_nothing() {
        const SOURCE: &str = r#"
            function f(s: { k: "a" } | { k: "b" }, t: { k: "a" } | { k: "b" }) {
                const k = s.k;
                const { k: j } = t;
                const ok = s.k === "a";
                s.k = "b";
                if (k === "a" && j === "a" && ok) { /*s*/s; /*t*/t; }
                if (s.k === "a") { /*direct*/s; }
            }
        "#;
        let info = module(SOURCE);
        let read = info.flow.read(marked(SOURCE, "t", "t")).unwrap();
        let root = info.flow.root(read.root).unwrap();
        // Each condition is decomposed once per outcome.
        let mut objects: Vec<_> = root
            .guards
            .iter()
            .filter_map(|guard| match guard {
                super::FlowGuard::Property {
                    object, aliased, ..
                } => Some((root.subject_binding(*object).unwrap(), *aliased)),
                _ => None,
            })
            .collect();
        objects.dedup();
        let s = root.subject_binding(info.flow.read(marked(SOURCE, "s", "s")).unwrap().subject);
        let t = root.subject_binding(read.subject);
        // Only the guard of `t` from a `const` and the direct test of `s.k`
        // remain, because the root writes `s.k` between the `const`
        // declarations and the first test.
        assert_eq!(objects, [(t.unwrap(), true), (s.unwrap(), false)]);
    }

    #[test]
    fn objects_whose_declared_type_is_not_a_union_of_objects_keep_no_reads() {
        const SOURCE: &str = r#"
            type Shape = { k: "a" } | { k: "b" };
            declare function make(): Shape;
            function f(
                object: { v: string | null },
                primitives: string | number | undefined,
                nullable: { k: "a" } | null,
                named: Shape,
            ) {
                const inferred = make();
                const instance = new Date();
                if (object.v && primitives.length && nullable.k && named.k && inferred.k && instance.getTime) {
                    /*object*/object;
                    /*primitives*/primitives;
                    /*nullable*/nullable;
                    /*named*/named;
                    /*inferred*/inferred;
                    /*instance*/instance;
                }
            }
        "#;
        let info = module(SOURCE);
        for (marker, expected) in [
            ("object", false),
            ("primitives", false),
            // One object type and `null` can't be told apart by a property.
            ("nullable", false),
            ("named", true),
            ("inferred", true),
            // `new` gives one instance.
            ("instance", false),
        ] {
            assert_eq!(
                info.flow.is_candidate(marked(SOURCE, marker, marker)),
                expected,
                "{marker}"
            );
        }
    }

    #[test]
    fn candidates_are_reachable_reads_and_their_evaluated_ancestors() {
        const SOURCE: &str = r#"
            declare function id<T>(value: T): T;
            function f(value: string | null, loose) {
                /*before*/value;
                if (value !== null && loose) {
                    /*member*/id(/*read*/value).length;
                    /*array*/[value];
                    /*loose*/loose;
                }
            }
        "#;
        let info = module(SOURCE);
        for (marker, expression, expected) in [
            ("before", "value", false),
            ("read", "value", true),
            ("member", "id(/*read*/value).length", true),
            ("member", "id(/*read*/value)", true),
            ("array", "[value]", false),
            ("loose", "loose", false),
        ] {
            let range = marked(SOURCE, marker, expression);
            assert_eq!(
                info.flow.is_candidate(range),
                expected,
                "{marker}: {expression}"
            );
            let resolved = info
                .flow
                .candidate(range)
                .and_then(|pointer| pointer.try_to_node(info.semantic_model.root().syntax()));
            assert_eq!(
                resolved.map(|expression| expression.range()),
                expected.then_some(range),
                "{marker}: {expression}"
            );
        }
    }

    #[test]
    fn receivers_that_cannot_be_nullish_are_not_tested() {
        const SOURCE: &str = r#"
            interface X { f(v: unknown): void; k: "a" | "b" }
            function f(x: X | undefined, s: string) {
                const o = { f(v: unknown) {}, k: "a" };
                o?.f(/*object*/o);
                s?.charAt(/*string*/s as any);
                x?.f(/*nullable*/x);
                if (o.k === "a") /*afterProperty*/o;
            }
        "#;
        let info = module(SOURCE);
        for (marker, expression, expected) in [
            ("object", "o", false),
            ("string", "s", false),
            ("nullable", "x", true),
            // A test of a property alone narrows `o` only when its type is a
            // union of objects, so `o` keeps no reads.
            ("afterProperty", "o", false),
        ] {
            assert_eq!(
                info.flow.is_candidate(marked(SOURCE, marker, expression)),
                expected,
                "{marker}"
            );
        }
    }
}
