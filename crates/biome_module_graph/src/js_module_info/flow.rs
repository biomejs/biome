//! Syntax-only flow facts that let type inference refine variable reads.
//!
//! Collection records, for each execution root that tests one of its own
//! variables, a graph of possible paths whose conditions are already
//! decomposed into guards on semantic bindings. It also records the incoming
//! path of every read such a guard can reach. Type inference later applies the
//! guards to resolved types without inspecting syntax again.
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
pub(crate) use guards::{FlowGuard, FlowTest};
pub(crate) use scanner::FlowRootScanner;

use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::{JsDeclarationKind, SemanticModel};
use biome_js_type_info::{RawTypeData, TypeReference, is_raw_narrowing_invariant};
use biome_rowan::{AstNode, TextRange};
use builder::{BuiltRoot, build_root};
use rustc_hash::{FxHashMap, FxHashSet};

/// Index of a node in [`RootFlow::nodes`].
pub(crate) type FlowNodeId = usize;

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
}

/// A read whose type a test in its execution root may refine.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FlowRead {
    /// Index of the read's execution root in the module flow.
    pub(crate) root: usize,
    /// The flow that reaches the read, before evaluating it.
    pub(crate) point: FlowNodeId,
    /// Range of the binding the read refers to.
    pub(crate) binding: TextRange,
}

/// Flow facts for one module.
#[derive(Debug, Default)]
pub(crate) struct ModuleFlow {
    roots: Vec<RootFlow>,
    reads: FxHashMap<TextRange, FlowRead>,
    candidates: FxHashSet<TextRange>,
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
        let collector = RootCollector {
            model,
            raw_types,
            raw_binding_types,
            relevance_budget,
        };
        for root in roots.selected() {
            collector.add_root(&mut flow, root);
        }
        flow
    }

    /// Returns whether the expression at `range` may have a flow-sensitive type.
    pub(crate) fn is_candidate(&self, range: TextRange) -> bool {
        self.candidates.contains(&range)
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
}

impl RootCollector<'_> {
    fn add_root(&self, flow: &mut ModuleFlow, root: &AnyJsControlFlowRoot) {
        let Some(BuiltRoot {
            nodes,
            guards,
            mentions,
            reads,
        }) = build_root(root, self.model)
        else {
            return;
        };
        let tested: FxHashSet<TextRange> = mentions.iter().map(|(_, binding)| *binding).collect();
        let mut eligible = FxHashMap::<TextRange, bool>::default();
        let reads: Vec<_> = reads
            .into_iter()
            .filter(|read| {
                tested.contains(&read.binding)
                    && *eligible
                        .entry(read.binding)
                        .or_insert_with(|| self.is_eligible(read.binding, root))
            })
            .collect();
        if reads.is_empty() {
            return;
        }

        let Some(reachable) = self.reachable_from_tests(&nodes, &mentions, &reads) else {
            return;
        };
        let index = flow.roots.len();
        let mut kept = false;
        for read in reads {
            if !reachable.contains(&(read.binding, read.point)) {
                continue;
            }
            kept = true;
            let identifier = AnyFlowExpression::JsIdentifierExpression(read.identifier);
            flow.candidates.insert(identifier.range());
            flow.candidates.extend(
                identifier
                    .evaluated_ancestors()
                    .map(|ancestor| ancestor.range()),
            );
            flow.reads.insert(
                identifier.range(),
                FlowRead {
                    root: index,
                    point: read.point,
                    binding: read.binding,
                },
            );
        }
        if kept {
            flow.roots.push(RootFlow {
                nodes: nodes.into_boxed_slice(),
                guards: guards.into_boxed_slice(),
            });
        }
    }

    /// Returns whether reads of the binding at `range` can be narrowed in `root`.
    fn is_eligible(&self, range: TextRange, root: &AnyJsControlFlowRoot) -> bool {
        let Some(binding) = self.model.as_binding_by_range(range) else {
            return false;
        };
        // The semantic index matches the start offset, not the entire range.
        // A `var` initializer is not a semantic write and may follow an
        // earlier guard.
        if binding.range() != range
            || binding.is_imported()
            || binding.declaration_kind() == JsDeclarationKind::HoistedValue
            || binding.all_writes().next().is_some()
        {
            return false;
        }
        // Untyped and `any` bindings never narrow, so their reads need no flow.
        if self
            .raw_binding_types
            .get(&range)
            .is_none_or(|reference| is_raw_narrowing_invariant(reference, self.raw_types))
        {
            return false;
        }
        binding
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(AnyJsControlFlowRoot::cast)
            .is_some_and(|declaration_root| declaration_root == *root)
    }

    /// Returns the `(binding, point)` pairs that a test of the binding can
    /// reach, including the test itself.
    ///
    /// Returns `None` when the root's work limit is spent.
    fn reachable_from_tests(
        &self,
        nodes: &[FlowNode],
        mentions: &[(FlowNodeId, TextRange)],
        reads: &[builder::BuiltRead],
    ) -> Option<FxHashSet<(TextRange, FlowNodeId)>> {
        let mut successors = vec![Vec::new(); nodes.len()];
        for (index, node) in nodes.iter().enumerate() {
            match node {
                FlowNode::Start => {}
                FlowNode::Join(predecessors) => {
                    for predecessor in predecessors {
                        successors[*predecessor].push(index);
                    }
                }
                FlowNode::Condition { antecedent, .. } => successors[*antecedent].push(index),
            }
        }
        let bindings: FxHashSet<TextRange> = reads.iter().map(|read| read.binding).collect();
        let mut reachable = FxHashSet::default();
        let mut remaining = self.relevance_budget;
        for binding in bindings {
            let mut seen = vec![false; nodes.len()];
            let mut pending: Vec<FlowNodeId> = mentions
                .iter()
                .filter(|(_, mentioned)| *mentioned == binding)
                .map(|(node, _)| *node)
                .collect();
            while let Some(node) = pending.pop() {
                remaining = remaining.checked_sub(1)?;
                if std::mem::replace(&mut seen[node], true) {
                    continue;
                }
                reachable.insert((binding, node));
                pending.extend(&successors[node]);
            }
        }
        Some(reachable)
    }
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
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        let root = parsed.tree();
        let model = Arc::new(semantic_model(&root, SemanticModelOptions::default()));
        JsModuleVisitor::new(
            root,
            Utf8PathBuf::from("/index.ts"),
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
    fn dynamic_scope_references_in_nested_functions_disable_enclosing_flow() {
        for (closure, expected) in [("value", true), ("eval('value = null')", false)] {
            let source = format!(
                "function f(value: string | null) {{ const g = () => {closure}; if (value !== null) {{ /*read*/value; }} }}"
            );
            let info = module(&source);
            assert_eq!(
                info.flow.is_candidate(marked(&source, "read", "value")),
                expected,
                "{closure}"
            );
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
                WalkEvent::Enter(node) => roots.enter(&node, &info.semantic_model),
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
            assert_eq!(
                info.flow.is_candidate(marked(SOURCE, marker, expression)),
                expected,
                "{marker}: {expression}"
            );
        }
    }
}
