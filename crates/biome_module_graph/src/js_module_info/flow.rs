//! Syntax-only flow facts that let type inference refine variable reads.
//!
//! Collection records, for each execution root that tests one of its own
//! variables, a graph of possible paths whose conditions are already
//! decomposed into guards on subjects. It also records the incoming path of
//! every read such a guard can reach. Type inference later applies the guards
//! to resolved types without inspecting syntax again.
//!
//! A subject is what a runtime test checks and a read narrows: a variable, or
//! a property path that starts at a variable or at `this`, such as `o.v`,
//! `a.b.c`, `t[0]`, or `this.#cache`. Guards and reads refer to a subject by
//! its id in the root's subject table, so a test and a read of the same subject
//! match without comparing syntax.
//!
//! An execution root is a body analyzed independently, such as a module,
//! function, or method. Only `let`, `const`, and parameter bindings that are
//! never reassigned and are read in the root that declares them participate,
//! so refinements cannot cross closure boundaries. Each root has its own
//! `this`, and paths on it narrow only where Biome types `this` as TypeScript
//! does, as [`subjects::this_can_narrow`] describes. A property path narrows
//! only when its variable or `this` does and the root never writes the path,
//! one of its prefixes, or a property of one of its prefixes through a key that
//! is not a literal. The graph does not invalidate facts on writes or calls,
//! which is why written subjects are excluded.

mod builder;
mod expressions;
mod guards;
mod scanner;
mod subjects;
mod writes;

pub(crate) use expressions::AnyFlowExpression;
pub(crate) use guards::{FlowGuard, FlowTest, guard_signature};
pub(crate) use scanner::FlowRootScanner;

use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::{JsDeclarationKind, SemanticModel};
use biome_js_syntax::{
    JsArrayBindingPatternElement, JsIdentifierBinding, JsObjectBindingPatternProperty,
    JsObjectBindingPatternShorthandProperty, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode,
    JsThisExpression,
};
use biome_js_type_info::{
    RawTypeData, TypeReference, is_raw_narrowing_invariant, raw_may_be_object_union,
};
use biome_rowan::{AstNode, AstPtr, Text, TextRange};
use builder::{BuiltRoot, build_root};
use guards::GuardFunctions;
use rustc_hash::{FxHashMap, FxHashSet};
use writes::WrittenPaths;

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
    /// The `this` of the execution root. A table stores it only in a root
    /// where [`subjects::this_can_narrow`] holds.
    This,
    /// The property `key` of the subject `object`, as in `o.v` or `o["v"]`.
    ///
    /// A table stores `object` before its members, so `object` is always a
    /// smaller id than the member.
    Member {
        object: FlowSubjectId,
        key: PropertyKey,
    },
}

/// The property that a step of a subject's path reads.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum PropertyKey {
    /// A property named by an identifier, with its escapes decoded, or by an
    /// element access key whose spelling is the property's name, so `o.v`,
    /// `o.\u0076`, and `o["v"]` read the same property, and so do `t[0]` and
    /// `t["0"]`.
    Name(Text),
    /// A private name, such as `#v` in `o.#v`, without its `#` and with its
    /// escapes decoded. It never matches a [`Self::Name`], so `o["#v"]`, which
    /// reads a property whose name starts with `#`, is another subject.
    Private(Text),
}

/// Returns the subject that the path of `subject` starts at, a
/// [`FlowSubject::Binding`] or [`FlowSubject::This`], given the `subjects` of
/// its root, each at the index of its id.
///
/// Returns `None` when `subjects` has no such subject.
fn subject_root(subjects: &[FlowSubject], mut subject: FlowSubjectId) -> Option<&FlowSubject> {
    loop {
        match subjects.get(subject as usize)? {
            root @ (FlowSubject::Binding(_) | FlowSubject::This) => return Some(root),
            FlowSubject::Member { object, .. } => {
                if *object >= subject {
                    return None;
                }
                subject = *object;
            }
        }
    }
}

/// The subjects of one execution root under construction, each stored once.
///
/// The builder records reads and the condition decomposer records runtime
/// tests through the same table, so a read and a test of the same subject get
/// the same id. A table made with [`Default`] stores no [`FlowSubject::This`].
#[derive(Default)]
struct FlowSubjectTable {
    subjects: Vec<FlowSubject>,
    ids: FxHashMap<FlowSubject, FlowSubjectId>,
    /// The execution root whose `this` can be a subject, or `None` when
    /// [`subjects::this_can_narrow`] does not hold for it.
    this_root: Option<JsSyntaxNode>,
}

impl FlowSubjectTable {
    /// Returns an empty table for the execution root `root`, which stores its
    /// `this` when `this_can_narrow` holds.
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

    /// Returns the id of the variable that `reference` refers to, adding the
    /// variable to the table on first use.
    ///
    /// Returns `None` when `reference` refers to no declaration, as a global
    /// does, or when the table has used every [`FlowSubjectId`].
    fn reference(
        &mut self,
        model: &SemanticModel,
        reference: &JsReferenceIdentifier,
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
    fn this(&mut self, expression: &JsThisExpression) -> Option<FlowSubjectId> {
        let root = self.this_root.as_ref()?;
        let in_member_owner = expression
            .syntax()
            .ancestors()
            .take_while(|node| node != root)
            .any(|node| {
                matches!(
                    node.kind(),
                    JsSyntaxKind::JS_CLASS_DECLARATION
                        | JsSyntaxKind::JS_CLASS_EXPRESSION
                        | JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION
                        | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
                        | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
                        | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
                )
            });
        if in_member_owner {
            return None;
        }
        self.intern(FlowSubject::This)
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
    /// The subjects with reads that conditions narrow only through
    /// [`FlowGuard::Property`], such as `s` when the only condition that
    /// mentions it is `s.kind === "c"`. Such a test narrows only a union, so
    /// these subjects need no solve when their declared type is not one.
    pub(crate) property_only: FxHashSet<FlowSubjectId>,
}

impl RootFlow {
    /// Returns the points where the root reads `subject`, sorted, or an empty
    /// slice when it keeps no read of it.
    pub(crate) fn read_points(&self, subject: FlowSubjectId) -> &[FlowNodeId] {
        self.read_points.get(&subject).map_or(&[], |points| points)
    }

    /// Returns whether conditions narrow `subject` only through
    /// [`FlowGuard::Property`], as [`Self::property_only`] describes.
    pub(crate) fn is_property_only(&self, subject: FlowSubjectId) -> bool {
        self.property_only.contains(&subject)
    }

    /// Returns the subject with id `subject`, or `None` when the root has no
    /// such subject.
    pub(crate) fn subject(&self, subject: FlowSubjectId) -> Option<&FlowSubject> {
        self.subjects.get(subject as usize)
    }

    /// Returns the range of the variable that the path of `subject` starts
    /// at, which is the variable itself for a [`FlowSubject::Binding`].
    ///
    /// Returns `None` for `this` and a path on it, and when the root has no
    /// such subject.
    pub(crate) fn subject_binding(&self, subject: FlowSubjectId) -> Option<TextRange> {
        match subject_root(&self.subjects, subject)? {
            FlowSubject::Binding(range) => Some(*range),
            FlowSubject::This | FlowSubject::Member { .. } => None,
        }
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
}

/// Flow facts for one module.
#[derive(Debug, Default)]
pub(crate) struct ModuleFlow {
    roots: Vec<RootFlow>,
    reads: FxHashMap<TextRange, FlowRead>,
    /// Points to each candidate expression, keyed by its trimmed range.
    candidates: FxHashMap<TextRange, AstPtr<AnyFlowExpression>>,
    /// The declarations of the bindings in [`RootFlow::property_only`].
    property_only_bindings: FxHashSet<TextRange>,
    /// The candidates that are candidates only because they are, or contain,
    /// reads of one binding in [`Self::property_only_bindings`], keyed by
    /// range, with that binding's declaration. Such a candidate keeps its
    /// type when no test of a property can narrow the binding.
    property_only_candidates: FxHashMap<TextRange, TextRange>,
}

impl ModuleFlow {
    /// Collects flow for the execution roots that a [`FlowRootScanner`]
    /// selected.
    ///
    /// Reads are kept only when their binding has a narrowable collected type,
    /// as decided by [`is_raw_narrowing_invariant`], and a test of that binding
    /// can reach them. Execution roots that exceed a construction or relevance
    /// work limit keep no reads.
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
        };
        for selected in roots.selected() {
            collector.add_root(
                &mut flow,
                &selected.root,
                roots.bindings(selected),
                selected.has_this,
            );
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
    /// through [`FlowGuard::Property`], as [`RootFlow::property_only`]
    /// describes.
    pub(crate) fn is_property_only_binding(&self, range: TextRange) -> bool {
        self.property_only_bindings.contains(&range)
    }

    /// Returns the declaration of the binding whose reads alone make the
    /// expression at `range` a candidate, when that binding is one that
    /// [`Self::is_property_only_binding`] accepts.
    pub(crate) fn property_only_candidate(&self, range: TextRange) -> Option<TextRange> {
        self.property_only_candidates.get(&range).copied()
    }

    /// Records `expression` as a candidate. `binding` is the declaration of
    /// the binding in [`Self::property_only_bindings`] whose read makes it a
    /// candidate, or `None` for a read of another subject.
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
}

struct RootCollector<'a> {
    model: &'a SemanticModel,
    raw_types: &'a [RawTypeData],
    raw_binding_types: &'a FxHashMap<TextRange, TypeReference>,
    relevance_budget: usize,
    functions: GuardFunctions<'a>,
}

impl RootCollector<'_> {
    /// Collects the flow of `root`, which declares `bindings` in its own code
    /// and uses `this` there when `has_this` is true.
    fn add_root(
        &mut self,
        flow: &mut ModuleFlow,
        root: &AnyJsControlFlowRoot,
        bindings: &[JsIdentifierBinding],
        has_this: bool,
    ) {
        let eligible: FxHashSet<TextRange> = bindings
            .iter()
            .filter_map(|binding| self.eligible_range(binding))
            .collect();
        let this_can_narrow = has_this && subjects::this_can_narrow(root);
        // Only bindings declared by the root itself and its own `this` can
        // narrow in it, so without either the graph would keep no reads.
        if eligible.is_empty() && !this_can_narrow {
            return;
        }
        let Some(BuiltRoot {
            nodes,
            guards,
            subjects,
            mut mentions,
            mut property_only,
            reads,
        }) = build_root(root, self.model, &mut self.functions, this_can_narrow)
        else {
            return;
        };
        // A test of a property narrows its object only when the object's
        // declared type is a union with at least two object types, so a
        // variable that only such tests mention keeps its declared type when
        // its collected type shows, without resolving it, that it is not one.
        // Dropping it here spares its reads the flow queries.
        let unnarrowable: FxHashSet<FlowSubjectId> = property_only
            .iter()
            .copied()
            .filter(|subject| match subjects.as_slice().get(*subject as usize) {
                Some(FlowSubject::Binding(binding)) => self
                    .raw_binding_types
                    .get(binding)
                    .is_none_or(|reference| !raw_may_be_object_union(reference, self.raw_types)),
                Some(FlowSubject::This | FlowSubject::Member { .. }) | None => false,
            })
            .collect();
        if !unnarrowable.is_empty() {
            property_only.retain(|subject| !unnarrowable.contains(subject));
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
        // Scanning for writes visits the whole root, so it runs only when a
        // path could narrow.
        let written = reads
            .iter()
            .any(|read| tested.contains(&read.subject) && is_path(read.subject))
            .then(|| WrittenPaths::new(root, self.model, &subjects));
        let narrowable: FxHashSet<FlowSubjectId> = tested
            .iter()
            .copied()
            .filter(|subject| {
                // The table stores `this` only when it can narrow.
                subject_root(subjects.as_slice(), *subject).is_some_and(|root| match root {
                    FlowSubject::Binding(binding) => eligible.contains(binding),
                    FlowSubject::This => true,
                    FlowSubject::Member { .. } => false,
                }) && written
                    .as_ref()
                    .is_none_or(|written| !written.cancels(subjects.as_slice(), *subject))
            })
            .collect();
        let reads: Vec<_> = reads
            .into_iter()
            .filter(|read| narrowable.contains(&read.subject))
            .collect();
        if reads.is_empty() {
            return;
        }

        let Some(reachable) = self.reachable_from_tests(
            &nodes,
            subjects.as_slice(),
            &mentions,
            &property_only,
            &reads,
        ) else {
            return;
        };
        let index = flow.roots.len();
        let mut read_points: FxHashMap<_, Vec<_>> = FxHashMap::default();
        for read in reads {
            if !reachable.contains(&(read.subject, read.point)) {
                continue;
            }
            read_points
                .entry(read.subject)
                .or_default()
                .push(read.point);
            let binding = match subjects.as_slice().get(read.subject as usize) {
                Some(FlowSubject::Binding(binding)) if property_only.contains(&read.subject) => {
                    flow.property_only_bindings.insert(*binding);
                    Some(*binding)
                }
                _ => None,
            };
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
                },
            );
        }
        if !read_points.is_empty() {
            let property_only = property_only
                .into_iter()
                .filter(|subject| read_points.contains_key(subject))
                .collect();
            flow.roots.push(RootFlow {
                nodes: nodes.into_boxed_slice(),
                guards: guards.into_boxed_slice(),
                subjects: subjects.into_subjects().into_boxed_slice(),
                property_only,
                read_points: read_points
                    .into_iter()
                    .map(|(subject, mut points)| {
                        points.sort_unstable();
                        points.dedup();
                        (subject, points.into_boxed_slice())
                    })
                    .collect(),
            });
        }
    }

    /// Returns the range of `binding` when the root that declares it can
    /// narrow its reads.
    fn eligible_range(&self, binding: &JsIdentifierBinding) -> Option<TextRange> {
        let range = binding.range();
        // The declared type of a destructured variable ignores defaults, so it
        // can lack the type of a default that supplies the value.
        if has_destructuring_default(binding) {
            return None;
        }
        let binding = self.model.as_binding_by_range(range)?;
        // The semantic index matches the start offset, not the entire range.
        // A `var` initializer is not a semantic write and may follow an
        // earlier guard.
        if binding.range() != range
            || binding.is_imported()
            || binding.declaration_kind() == JsDeclarationKind::HoistedValue
            || binding.all_writes().next().is_some()
        {
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
        Some(range)
    }

    /// Returns the `(subject, point)` pairs that a test of the subject can
    /// reach, including the test itself.
    ///
    /// Variables are walked before property paths and before the subjects in
    /// `property_only`, which only tests of their properties narrow, so the
    /// work limit never takes narrowing away from a variable because of
    /// them. Returns no pair of a path or of a subject in `property_only` when
    /// the limit runs out on them, and `None` when it runs out on the other
    /// variables.
    fn reachable_from_tests(
        &self,
        nodes: &[FlowNode],
        subjects: &[FlowSubject],
        mentions: &[(FlowNodeId, FlowSubjectId)],
        property_only: &FxHashSet<FlowSubjectId>,
        reads: &[builder::BuiltRead],
    ) -> Option<FxHashSet<(FlowSubjectId, FlowNodeId)>> {
        let mut successors = vec![Vec::new(); nodes.len()];
        for (index, node) in (0..).zip(nodes) {
            match node {
                FlowNode::Start => {}
                FlowNode::Join(predecessors) => {
                    for predecessor in predecessors {
                        successors[*predecessor as usize].push(index);
                    }
                }
                FlowNode::Condition { antecedent, .. } => {
                    successors[*antecedent as usize].push(index);
                }
            }
        }
        // Paths and subjects in `property_only` lose their reads first.
        let is_secondary = |subject: FlowSubjectId| {
            property_only.contains(&subject)
                || matches!(
                    subjects.get(subject as usize),
                    Some(FlowSubject::Member { .. })
                )
        };
        let mut read_subjects: Vec<FlowSubjectId> = reads.iter().map(|read| read.subject).collect();
        read_subjects.sort_unstable_by_key(|subject| (is_secondary(*subject), *subject));
        read_subjects.dedup();
        let mut reachable = FxHashSet::default();
        let mut remaining = self.relevance_budget;
        'subjects: for subject in read_subjects {
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

/// Returns whether a destructuring default can supply the value of `binding`,
/// either its own default, as in `{ a = 1 }`, or the default of an enclosing
/// pattern, as in `{ pair: [a] = [1] }`.
fn has_destructuring_default(binding: &JsIdentifierBinding) -> bool {
    binding
        .syntax()
        .ancestors()
        .skip(1)
        .take_while(|node| {
            matches!(
                node.kind(),
                JsSyntaxKind::JS_ARRAY_BINDING_PATTERN
                    | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT
                    | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
                    | JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
                    | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN
                    | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY
                    | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
                    | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST
                    | JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
            )
        })
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
    fn only_the_root_that_declares_a_variable_narrows_its_reads() {
        const SOURCE: &str = r#"
            function f(outer: string | null) {
                const g = (own: string | null) =>
                    own !== null && outer !== null && /*own*/own && /*outer*/outer;
            }
        "#;
        let info = module(SOURCE);
        for (marker, expression, expected) in [("own", "own", true), ("outer", "outer", false)] {
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
        // path, which is then dropped alone.
        let budget = (0..1024)
            .find(|budget| collect(*budget).is_candidate(x))
            .unwrap();
        let flow = collect(budget);
        assert!(!flow.is_candidate(v));
        assert!(!collect(budget - 1).is_candidate(x));
    }

    #[test]
    fn paths_on_this_narrow_outside_constructors_and_static_members() {
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
            ("constructor", "this.x", false),
            ("arrow", "this.x", false),
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
    fn written_and_deleted_paths_are_not_reads() {
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
        // The paths can still be candidates, because their object `o` is a
        // read that the tests of its properties can narrow.
        for (marker, expression, expected) in [
            ("object", "o.v", true),
            ("object", "o.v.w", false),
            ("deleted", "o.n", false),
            ("after", "o.n", false),
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
}
