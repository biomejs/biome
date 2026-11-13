//! Syntax-only flow facts that let type inference refine variable reads.
//!
//! Collection records, for each execution root that tests one of its own
//! variables, a graph of possible paths whose conditions are already
//! decomposed into guards on subjects. It also records the incoming path of
//! every read such a guard can reach. Type inference later applies the guards
//! to resolved types without inspecting syntax again.
//!
//! A subject is what a runtime test checks and a read narrows. Guards and
//! reads refer to a subject by its id in the root's subject table, so a test
//! and a read of the same subject match without comparing syntax. Every
//! subject is a semantic binding.
//!
//! An execution root is a body analyzed independently, such as a module,
//! function, or method. Only `let`, `const`, and parameter bindings that are
//! never reassigned and are read in the root that declares them participate,
//! so refinements cannot cross closure boundaries. The graph does not
//! invalidate facts on writes or calls, which is why written bindings are
//! excluded.

mod builder;
mod expressions;
mod guards;
mod scanner;

pub(crate) use expressions::AnyFlowExpression;
pub(crate) use guards::{FlowGuard, FlowTest, guard_signature};
pub(crate) use scanner::FlowRootScanner;

use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::{JsDeclarationKind, SemanticModel};
use biome_js_syntax::{
    JsArrayBindingPatternElement, JsIdentifierBinding, JsObjectBindingPatternProperty,
    JsObjectBindingPatternShorthandProperty, JsReferenceIdentifier, JsSyntaxKind,
};
use biome_js_type_info::{RawTypeData, TypeReference, is_raw_narrowing_invariant};
use biome_rowan::{AstNode, AstPtr, TextRange};
use builder::{BuiltRoot, build_root};
use guards::GuardFunctions;
use rustc_hash::{FxHashMap, FxHashSet};

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
}

/// The subjects of one execution root under construction, each stored once.
///
/// The builder records reads and the condition decomposer records runtime
/// tests through the same table, so a read and a test of the same subject get
/// the same id.
#[derive(Default)]
struct FlowSubjectTable {
    subjects: Vec<FlowSubject>,
    ids: FxHashMap<FlowSubject, FlowSubjectId>,
}

impl FlowSubjectTable {
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
}

impl RootFlow {
    /// Returns the points where the root reads `subject`, sorted, or an empty
    /// slice when it keeps no read of it.
    pub(crate) fn read_points(&self, subject: FlowSubjectId) -> &[FlowNodeId] {
        self.read_points.get(&subject).map_or(&[], |points| points)
    }

    /// Returns the subject with id `subject`, or `None` when the root has no
    /// such subject.
    pub(crate) fn subject(&self, subject: FlowSubjectId) -> Option<&FlowSubject> {
        self.subjects.get(subject as usize)
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
            collector.add_root(&mut flow, &selected.root, roots.bindings(selected));
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

    /// Returns the flow of the identifier read at `range`.
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
    fn add_root(
        &mut self,
        flow: &mut ModuleFlow,
        root: &AnyJsControlFlowRoot,
        bindings: &[JsIdentifierBinding],
    ) {
        let eligible: FxHashSet<TextRange> = bindings
            .iter()
            .filter_map(|binding| self.eligible_range(binding))
            .collect();
        // Only bindings declared by the root itself can narrow in it, so
        // without one the graph would keep no reads.
        if eligible.is_empty() {
            return;
        }
        let Some(BuiltRoot {
            nodes,
            guards,
            subjects,
            mentions,
            reads,
        }) = build_root(root, self.model, &mut self.functions)
        else {
            return;
        };
        let tested: FxHashSet<FlowSubjectId> =
            mentions.iter().map(|(_, subject)| *subject).collect();
        let reads: Vec<_> = reads
            .into_iter()
            .filter(|read| {
                tested.contains(&read.subject)
                    && subjects
                        .get(read.subject as usize)
                        .is_some_and(|subject| match subject {
                            FlowSubject::Binding(range) => eligible.contains(range),
                        })
            })
            .collect();
        if reads.is_empty() {
            return;
        }

        let Some(reachable) = self.reachable_from_tests(&nodes, &mentions, &reads) else {
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
            let identifier = AnyFlowExpression::JsIdentifierExpression(read.identifier);
            flow.candidates
                .insert(identifier.range(), AstPtr::new(&identifier));
            flow.candidates.extend(
                identifier
                    .evaluated_ancestors()
                    .map(|ancestor| (ancestor.range(), AstPtr::new(&ancestor))),
            );
            flow.reads.insert(
                identifier.range(),
                FlowRead {
                    root: index,
                    point: read.point,
                    subject: read.subject,
                },
            );
        }
        if !read_points.is_empty() {
            flow.roots.push(RootFlow {
                nodes: nodes.into_boxed_slice(),
                guards: guards.into_boxed_slice(),
                subjects: subjects.into_boxed_slice(),
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
    /// Returns `None` when the root's work limit is spent.
    fn reachable_from_tests(
        &self,
        nodes: &[FlowNode],
        mentions: &[(FlowNodeId, FlowSubjectId)],
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
        let subjects: FxHashSet<FlowSubjectId> = reads.iter().map(|read| read.subject).collect();
        let mut reachable = FxHashSet::default();
        let mut remaining = self.relevance_budget;
        for subject in subjects {
            let mut seen = vec![false; nodes.len()];
            let mut pending: Vec<FlowNodeId> = mentions
                .iter()
                .filter(|(_, mentioned)| *mentioned == subject)
                .map(|(node, _)| *node)
                .collect();
            while let Some(node) = pending.pop() {
                remaining = remaining.checked_sub(1)?;
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
