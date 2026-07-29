use crate::services::database::DbService;
use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, Rule, RuleDiagnostic,
    RuleDomain, RuleKey, RuleMetadata, RuleSource, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
    context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::AnyJsImportLike;
use biome_module_graph::{JsImportPath, JsImportPhase, JsModuleInfo, ModuleDb, ModuleInfoKind};
use biome_resolver::ResolvedPath;
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange};
use biome_rule_options::no_import_cycles::NoImportCyclesOptions;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::Deref;
use std::sync::{Arc, RwLock};

declare_lint_rule! {
    /// Prevent import cycles.
    ///
    /// This rule warns when a file imports another file that, either directly
    /// or indirectly, imports the original file again.
    ///
    /// Cycles can lead to symbols that are unexpectedly `undefined` and are
    /// generally considered poor code hygiene.
    ///
    /// If a cycle is detected, it is advised to move code such that imports
    /// only go in a single direction, i.e. they don't point "back" to the
    /// importing file.
    ///
    /// However, files that import themselves are allowed, and the rule won't trigger for these use cases.
    /// This allows for encapsulation of functions/variables into a namespace instead of using a
    /// static class (triggers [noStaticOnlyClass](https://biomejs.dev/linter/rules/no-static-only-class)).
    ///
    /// :::note
    /// This rule is computationally expensive. If you are particularly
    /// pressed for lint time, or don't think you have an issue with dependency
    /// cycles, you may not want this rule enabled.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,file=foobar.js
    /// import { baz } from "./baz.js";
    ///
    /// export function foo() {
    ///     baz();
    /// }
    ///
    /// export function bar() {
    ///     console.log("foobar");
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,file=baz.js
    /// import { bar } from "./foobar.js";
    ///
    /// export function baz() {
    ///     bar();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=foo.js
    /// import { baz } from "./baz.js";
    ///
    /// export function foo() {
    ///     baz();
    /// }
    /// ```
    ///
    /// ```js,file=bar.js
    /// export function bar() {
    ///     console.log("foobar");
    /// }
    /// ```
    ///
    /// ```js,file=baz.js
    /// import { bar } from "./bar.js";
    ///
    /// export function baz() {
    ///     bar();
    /// }
    /// ```
    ///
    /// ```js,file=foobaz.js
    /// export function foo() {
    ///     console.log("foobaz");
    /// }
    ///
    /// export * as baz from './foobaz.js';
    ///
    /// import { baz } from './foobaz.js';
    /// ```
    ///
    /// ```ts,file=types.ts
    /// import type { bar } from "./qux.ts";
    ///
    /// export type Foo = {
    ///   bar: typeof bar;
    /// };
    /// ```
    ///
    /// ```ts,file=qux.ts
    /// import type { Foo } from "./types.ts";
    ///
    /// export function bar(foo: Foo) {
    ///     console.log(foo);
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides the options described below.
    ///
    /// ### `ignoreTypes`
    ///
    /// Ignores type-only imports when finding an import cycle. A type-only import (`import type`)
    /// will be removed by the compiler, so it cuts an import cycle at runtime. Note that named type
    /// imports (`import { type Foo }`) aren't considered as type-only because it's not removed by
    /// the compiler if the `verbatimModuleSyntax` option is enabled. Enabled by default.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreTypes": false
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```ts,file=types.ts
    /// import type { bar } from "./qux.ts";
    ///
    /// export type Foo = {
    ///   bar: typeof bar;
    /// };
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic,file=qux.ts
    /// import type { Foo } from "./types.ts";
    ///
    /// export function bar(foo: Foo) {
    ///     console.log(foo);
    /// }
    /// ```
    pub NoImportCycles {
        version: "2.0.0",
        name: "noImportCycles",
        language: "js",
        sources: &[
            RuleSource::EslintImport("no-cycle").same(),
        ],
        severity: Severity::Warning,
        recommended: false,
        domains: &[RuleDomain::Project],
    }
}

/// Wraps [`DbService`] to additionally warm the process-wide SCC cache (see
/// `scc_data` below) as a side effect of `FromServices::from_services`.
///
/// `RuleContext::new` calls `from_services` *before* the profiler's rule timer
/// starts (`registry.rs` builds the context, then starts `RuleRunTimer`, then
/// calls `Rule::run`) -- so whichever invocation is the first to construct
/// this service anywhere in the whole run pays the one-time SCC-graph-build
/// cost in that untimed window, not inside a timed `run()` call. Without this,
/// `--profile-rules` attributes the ~30-200ms build to whichever file's rule
/// invocation happens to race there first, which reads as a wildly expensive,
/// non-reproducible per-file spike even though it has nothing to do with that
/// file. This type only exists to carry that warm-up; it's kept local to this
/// rule (rather than added to `DbService` itself) since no other rule needs it.
#[derive(Clone)]
pub struct SccWarmedDbService(DbService);

impl Deref for SccWarmedDbService {
    type Target = DbService;

    fn deref(&self) -> &DbService {
        &self.0
    }
}

impl FromServices for SccWarmedDbService {
    fn from_services(
        rule_key: &RuleKey,
        rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let inner = DbService::from_services(rule_key, rule_metadata, services)?;
        scc_data(inner.db());
        Ok(Self(inner))
    }
}

impl Phase for SccWarmedDbService {
    fn phase() -> Phases {
        DbService::phase()
    }
}

/// Identical to [`ResolvedImports`](crate::services::database::ResolvedImports),
/// except it resolves to [`SccWarmedDbService`] instead of [`DbService`] directly.
#[derive(Clone)]
pub struct SccAwareImports<N>(N);

impl<N, L> QueryMatch for SccAwareImports<N>
where
    L: Language,
    N: AstNode<Language = L> + 'static,
{
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl<N, L> Queryable for SccAwareImports<N>
where
    L: Language + 'static,
    N: AstNode<Language = L> + 'static,
{
    type Input = SyntaxNode<L>;
    type Output = N;

    type Language = L;
    type Services = SccWarmedDbService;

    fn build_visitor(analyzer: &mut impl AddVisitor<L>, _: &L::Root) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

impl Rule for NoImportCycles {
    type Query = SccAwareImports<AnyJsImportLike>;
    type State = Box<[ResolvedPath]>;
    type Signals = Option<Self::State>;
    type Options = NoImportCyclesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module_info = ctx.js_module_info_for_path(ctx.file_path())?;
        let node = ctx.query();

        let JsImportPath {
            resolved_path,
            phase,
        } = module_info.get_import_path_by_js_node(node)?;

        let options = ctx.options();
        if options.ignore_types() && *phase == JsImportPhase::Type {
            return None;
        }

        let resolved_path_path = resolved_path.as_path()?;

        // Don't check for cycles through node_modules imports.
        if is_node_modules_path(resolved_path_path) {
            return None;
        }

        // Fast-reject pre-filter: `find_cycle` below is an uncached DFS through
        // the whole reachable import graph, invoked once per import statement in
        // the project (this rule's own doc comment already calls it "computationally
        // expensive"). Most imports in a healthy codebase aren't part of any cycle
        // at all, so first check -- in O(1) against a once-per-generation-precomputed
        // strongly-connected-components table -- whether *this specific import*
        // could possibly close a loop back to the current file (not just whether the
        // current file is cyclic via *some* import; a file with one real cycle can
        // still have plenty of other, unrelated, non-cyclic imports). If it can't,
        // we can skip the DFS entirely without altering the result.
        if !scc_data(ctx.db()).same_cyclic_scc(ctx.file_path(), resolved_path_path) {
            return None;
        }

        let imports = ctx.js_module_info_for_path(resolved_path_path)?;

        find_cycle(ctx, resolved_path, imports)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let cwd = Utf8PathBuf::from(
            std::env::current_dir()
                .map(|cwd| cwd.to_string_lossy().to_string())
                .unwrap_or_default(),
        );

        let mut note = markup!("This import resolves to ").to_owned();
        for (i, path) in state.iter().enumerate() {
            if i > 0 {
                note.extend_with(markup!("\n    ... which imports "));
            }

            let Some(path) = path.as_path() else {
                continue;
            };

            match path.strip_prefix(&cwd) {
                Ok(relative_path) => {
                    note.extend_with(markup!(<Info>{relative_path.as_str()}</Info>))
                }
                Err(_) => note.extend_with(markup!(<Info>{path.as_str()}</Info>)),
            }
        }
        note.extend_with(markup!("\n    ... which is the file we're importing from."));

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup!("This import is part of a cycle."),
            )
            .note(note),
        )
    }
}

/// Attempts to find a cycle by traversing all imports from `start_path` and
/// finding those that lead back to `ctx.file_path()`.
///
/// Cycles that don't lead back to `ctx.file_path()` are not reported, since
/// they should be reported for the respective files instead.
///
/// `imports` are the imports found in `start_path`.
///
/// If a cycle is found, this returns a vector with all the paths involved in
/// the cycle, starting with `start_path` and ending with `ctx.file_path()`.
fn find_cycle(
    ctx: &RuleContext<NoImportCycles>,
    start_path: &ResolvedPath,
    mut module_info: JsModuleInfo,
) -> Option<Box<[ResolvedPath]>> {
    let options = ctx.options();
    let mut seen = FxHashSet::default();
    let mut stack: Vec<(ResolvedPath, JsModuleInfo)> = Vec::new();

    'outer: loop {
        for JsImportPath {
            resolved_path,
            phase,
        } in module_info.all_import_paths()
        {
            if options.ignore_types() && phase == JsImportPhase::Type {
                continue;
            }

            let Some(path) = resolved_path.as_path() else {
                continue;
            };

            // Skip node_modules paths — we don't traverse into dependencies.
            if is_node_modules_path(path) {
                continue;
            }

            if !seen.insert(resolved_path.clone()) {
                continue;
            }

            if path == ctx.file_path() {
                // https://github.com/biomejs/biome/issues/6569
                // prevent flagging on import cycles when they are isolated to a single file
                if stack.is_empty() && start_path.as_path() == Some(path) {
                    continue;
                }

                // Return all the paths from `start_path` to `resolved_path`:
                let paths = Some(start_path.clone())
                    .into_iter()
                    .chain(stack.iter().map(|(path, _)| path.clone()))
                    .chain(Some(resolved_path.clone()))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();

                return Some(paths);
            }

            if let Some(next_module_info) = ctx.js_module_info_for_path(path) {
                stack.push((resolved_path.clone(), module_info));
                module_info = next_module_info;
                continue 'outer;
            }
        }

        match stack.pop() {
            Some((_previous_path, previous_module_info)) => {
                module_info = previous_module_info;
            }
            None => break,
        }
    }

    None
}

/// Returns `true` if the given path is inside a `node_modules` directory.
fn is_node_modules_path(path: &Utf8Path) -> bool {
    path.components().any(|c| c.as_str() == "node_modules")
}

/// Precomputed strongly-connected-component membership for the whole import
/// graph, used as the `same_cyclic_scc` pre-filter. See `scc_data` for how
/// this is built and cached.
struct SccData {
    /// Which SCC each module belongs to. A module absent from this map was
    /// never reached while building the graph (e.g. an unresolvable or
    /// non-JS module) and trivially cannot be part of a cycle.
    scc_of: FxHashMap<Utf8PathBuf, u32>,
    /// Number of modules in each SCC, indexed by SCC id.
    scc_size: Vec<u32>,
}

impl SccData {
    /// Returns `true` if `from` and `to` could possibly be connected by a
    /// genuine import cycle.
    ///
    /// Every call site already has a real edge `from -> to` in hand (`to` is
    /// the resolved target of an actual import statement in `from`), so this
    /// check is exact, not a heuristic: a cycle back to `from` exists if and
    /// only if `to` can also reach `from`, which (given the edge already
    /// established `from` reaches `to`) is precisely the condition for `from`
    /// and `to` being mutually reachable -- i.e. sharing a non-trivial SCC.
    /// `false` is therefore a *proof* no cycle can exist through this edge,
    /// letting the caller skip the expensive DFS entirely; checking the
    /// specific target (rather than just "is `from` cyclic via *some*
    /// import") also correctly skips imports out of an otherwise-cyclic file
    /// that don't themselves close any loop.
    ///
    /// A direct self-import (`from == to`) correctly returns `false` unless
    /// `from` also participates in a larger cycle through some *other*
    /// import: a lone self-edge keeps a module alone in its own size-1 SCC,
    /// which this treats the same as "no cycle" -- matching the pre-existing,
    /// unchanged exception in `find_cycle` for isolated self-imports (see the
    /// `stack.is_empty()` check there, and issue #6569).
    fn same_cyclic_scc(&self, from: &Utf8Path, to: &Utf8Path) -> bool {
        let (Some(&from_id), Some(&to_id)) = (self.scc_of.get(from), self.scc_of.get(to)) else {
            return false;
        };
        from_id == to_id && self.scc_size[from_id as usize] > 1
    }
}

/// Runs Kosaraju's algorithm (two DFS passes: forward, then over the transpose
/// graph) over an integer-indexed adjacency list, assigning every node in
/// `0..forward.len()` to a strongly-connected component. Both passes are
/// iterative (an explicit stack) to avoid overflowing the stack on a project
/// with a deep or wide import graph.
///
/// Takes and returns dense integer ids rather than paths, on purpose:
/// `build_scc_data` (the only non-test caller) interns every module's path
/// into an id once, up front, specifically so this traversal -- which visits
/// every edge at least twice, once per pass -- never has to hash or clone a
/// `Utf8PathBuf` (an owned string) per edge. That string traffic dominated
/// this function's real-world cost back when it was path-keyed throughout:
/// ~200ms to build the SCC decomposition for astra's ~5100-module graph in a
/// debug build. It's a one-time, cached-per-generation cost (see
/// `SCC_CACHE`), so it was never a per-file tax, but it's still real,
/// measurable work that a `Vec` index instead of a string hash mostly avoids.
///
/// Returns `(scc_of, scc_size)`: `scc_of[node_id]` is that node's SCC id,
/// indices into `scc_size`.
fn compute_sccs_indexed(forward: &[Vec<u32>]) -> (Vec<u32>, Vec<u32>) {
    let n = forward.len();

    // Pass 1: iterative DFS over the forward graph, recording finish order.
    // A per-node "next child index" array replaces the more common per-frame
    // `(node, child_idx)` stack tuple -- equivalent because a node is only
    // ever on the stack once (it's marked visited before being pushed), so
    // its own cursor never needs to be saved or restored across frames.
    let mut visited = vec![false; n];
    let mut next_child: Vec<u32> = vec![0; n];
    let mut finish_order: Vec<u32> = Vec::with_capacity(n);
    let mut stack: Vec<u32> = Vec::new();

    for start in 0..n as u32 {
        if visited[start as usize] {
            continue;
        }
        visited[start as usize] = true;
        stack.push(start);
        while let Some(&node) = stack.last() {
            let node_idx = node as usize;
            let children = &forward[node_idx];
            let cursor = next_child[node_idx] as usize;
            if cursor < children.len() {
                next_child[node_idx] += 1;
                let next = children[cursor];
                if !visited[next as usize] {
                    visited[next as usize] = true;
                    stack.push(next);
                }
            } else {
                finish_order.push(node);
                stack.pop();
            }
        }
    }

    // Build the reverse (transpose) adjacency list.
    let mut reverse: Vec<Vec<u32>> = vec![Vec::new(); n];
    for (from_id, tos) in forward.iter().enumerate() {
        for &to_id in tos {
            reverse[to_id as usize].push(from_id as u32);
        }
    }

    // Pass 2: process nodes in reverse finish order; each DFS over the reverse
    // graph from an unvisited node discovers exactly one SCC.
    let mut scc_of: Vec<u32> = vec![u32::MAX; n];
    let mut scc_size: Vec<u32> = Vec::new();
    let mut stack: Vec<u32> = Vec::new();

    for &start in finish_order.iter().rev() {
        if scc_of[start as usize] != u32::MAX {
            continue;
        }
        let scc_id = scc_size.len() as u32;
        scc_size.push(0);

        stack.push(start);
        while let Some(node) = stack.pop() {
            let node_idx = node as usize;
            if scc_of[node_idx] != u32::MAX {
                continue;
            }
            scc_of[node_idx] = scc_id;
            scc_size[scc_id as usize] += 1;
            for &pred in &reverse[node_idx] {
                if scc_of[pred as usize] == u32::MAX {
                    stack.push(pred);
                }
            }
        }
    }

    // Every id in `0..n` is visited by the pass-1 loop above (it starts a
    // fresh DFS from any node not already visited), so every id ends up in
    // `finish_order` and pass 2 always assigns it a real SCC id -- `scc_of`
    // never has a leftover `u32::MAX` by this point.
    (scc_of, scc_size)
}

/// Extracts the whole project's JS import graph from `db` and computes its
/// `SccData`.
///
/// Deliberately includes *every* resolvable, non-`node_modules` edge --
/// type-only imports too, regardless of any file's own `ignoreTypes` setting.
/// `ignoreTypes` is a per-rule-instance option that can differ **per file**
/// (a file can override it via its own Biome configuration scope, as the
/// `includeTypes.ts`/`types.ts` fixtures exercise: the former lints with
/// `ignoreTypes: false`, the latter with the default `true`), so there is no
/// single correct value to build *one* shared graph with. Including every
/// edge unconditionally makes this a conservative over-approximation: it can
/// only ever make `could_be_cyclic` return `true` *more* often than the
/// precise per-file answer would, never less. A `false` from this graph is
/// still a hard proof that no path exists in *any* subset of these edges, so
/// the fast-reject skip remains exactly as safe; the only cost is that a
/// handful of files whose only would-be cycle is entirely type-only edges
/// they've chosen to ignore fall back to the exact (still-correct,
/// still-`ignoreTypes`-aware) DFS instead of being skipped -- a missed
/// optimization, never a wrong answer.
///
/// Self-edges (`target == path`) are dropped rather than added to the graph:
/// a node can always trivially "reach" itself while its own children are
/// being explored (see `compute_sccs_indexed`'s `visited` guard), so a
/// self-edge can never change any node's SCC assignment or size -- it would
/// only ever be redundant with edges already there, or absent for a module
/// with no other cyclic relationship, in which case `same_cyclic_scc`
/// correctly falls out to `false` (size-1 SCC) without needing this tracked
/// separately.
///
/// Enumerates `db.for_each_module` twice rather than building a path-keyed
/// adjacency map in one pass: an edge can point *forward* to a module not yet
/// seen in iteration order, so every module needs an id assigned before any
/// edge can be resolved into that id space. Two integer-only passes are
/// cheaper overall than one pass that builds a `Utf8PathBuf`-keyed map and
/// then re-interns it (which is what this used to do, and what
/// `compute_sccs_indexed`'s doc comment measures) -- `for_each_module`
/// iterates an already-populated index (see its impl in `biome_workspace_db`),
/// not a fresh computation, so the second walk is cheap.
fn build_scc_data(db: &dyn ModuleDb) -> SccData {
    // Pass 1: assign every JS module a dense integer id, and remember its
    // path -- the only point paths get cloned, once each, ever (the actual
    // traversal in `compute_sccs_indexed` runs entirely over ids).
    let mut id_of: FxHashMap<Utf8PathBuf, u32> = FxHashMap::default();
    let mut path_of: Vec<Utf8PathBuf> = Vec::new();
    db.for_each_module(&mut |path, kind| {
        if !matches!(kind, ModuleInfoKind::Js(_)) {
            return;
        }
        id_of.insert(path.to_path_buf(), path_of.len() as u32);
        path_of.push(path.to_path_buf());
    });
    let n = path_of.len();

    // Pass 2: resolve every import edge straight into the id space built
    // above -- no intermediate path-keyed adjacency map to build and then
    // immediately re-intern.
    let mut forward: Vec<Vec<u32>> = vec![Vec::new(); n];
    db.for_each_module(&mut |path, kind| {
        let ModuleInfoKind::Js(module_info) = kind else {
            return;
        };
        // `path` matched the same `ModuleInfoKind::Js` predicate in pass 1
        // above, so it normally always has an id here; the only way it
        // wouldn't is a concurrent `--watch`/LSP update adding this module
        // between the two `for_each_module` calls, in which case skipping it
        // for this generation is correct -- the next generation bump (from
        // that very update) triggers a full rebuild anyway.
        let Some(&from_id) = id_of.get(path) else {
            return;
        };
        for JsImportPath { resolved_path, .. } in module_info.all_import_paths() {
            let Some(target) = resolved_path.as_path() else {
                continue;
            };
            if is_node_modules_path(target) || target == path {
                continue;
            }
            // A target that isn't itself a key of `id_of` resolved to a
            // non-JS module (e.g. CSS) -- `compute_sccs_indexed` only starts
            // its forward pass from `0..n`, so such an edge could never be
            // traversed anyway; dropping it here changes no result.
            if let Some(&to_id) = id_of.get(target) {
                forward[from_id as usize].push(to_id);
            }
        }
    });

    let (scc_of_idx, scc_size) = compute_sccs_indexed(&forward);

    let mut scc_of: FxHashMap<Utf8PathBuf, u32> = FxHashMap::default();
    for (idx, path) in path_of.into_iter().enumerate() {
        scc_of.insert(path, scc_of_idx[idx]);
    }

    SccData { scc_of, scc_size }
}

/// Process-wide cache for `SccData`, keyed by `ModuleDb::module_graph_generation()`
/// so a long-running process (the `--watch` CLI mode or the LSP/daemon) rebuilds
/// it when files change instead of serving a stale graph forever. An `RwLock`
/// (not a `Mutex`) is used deliberately: this is read on every rule invocation
/// (the very call site the profiler-lock-contention fix upstream of this one
/// addressed), so concurrent readers must not serialize against each other --
/// only the rare rebuild-on-generation-change path needs exclusive access.
static SCC_CACHE: RwLock<Option<(u64, Arc<SccData>)>> = RwLock::new(None);

/// Returns the current `SccData`, rebuilding it if this is the first call or
/// if `db`'s generation has advanced since it was last built.
fn scc_data(db: &dyn ModuleDb) -> Arc<SccData> {
    let current_gen = db.module_graph_generation();

    if let Some((cached_gen, data)) = SCC_CACHE.read().unwrap().as_ref()
        && *cached_gen == current_gen
    {
        return data.clone();
    }

    let mut cache = SCC_CACHE.write().unwrap();
    // Re-check: another thread may have rebuilt while we were waiting for the
    // write lock.
    if let Some((cached_gen, data)) = cache.as_ref()
        && *cached_gen == current_gen
    {
        return data.clone();
    }
    let data = Arc::new(build_scc_data(db));
    *cache = Some((current_gen, data.clone()));
    data
}

#[cfg(test)]
mod scc_tests {
    use super::compute_sccs_indexed;
    use camino::Utf8PathBuf;
    use rustc_hash::FxHashMap;

    fn graph(edges: &[(&str, &[&str])]) -> FxHashMap<Utf8PathBuf, Vec<Utf8PathBuf>> {
        edges
            .iter()
            .map(|(from, tos)| {
                (
                    Utf8PathBuf::from(from),
                    tos.iter().map(Utf8PathBuf::from).collect(),
                )
            })
            .collect()
    }

    /// Bridges these tests' path-keyed fixtures to `compute_sccs_indexed`'s
    /// integer-only API: intern every path to an id, run the real algorithm,
    /// translate `scc_of` back to paths so the existing `scc_id_of` assertions
    /// below don't need to change.
    fn compute_sccs(
        g: &FxHashMap<Utf8PathBuf, Vec<Utf8PathBuf>>,
    ) -> (FxHashMap<Utf8PathBuf, u32>, Vec<u32>) {
        let mut id_of: FxHashMap<Utf8PathBuf, u32> = FxHashMap::default();
        let mut path_of: Vec<Utf8PathBuf> = Vec::new();
        for path in g.keys() {
            id_of.insert(path.clone(), path_of.len() as u32);
            path_of.push(path.clone());
        }

        let mut forward: Vec<Vec<u32>> = vec![Vec::new(); path_of.len()];
        for (from, tos) in g {
            let from_id = id_of[from] as usize;
            for to in tos {
                if let Some(&to_id) = id_of.get(to) {
                    forward[from_id].push(to_id);
                }
            }
        }

        let (scc_of_idx, scc_size) = compute_sccs_indexed(&forward);

        let mut scc_of: FxHashMap<Utf8PathBuf, u32> = FxHashMap::default();
        for (idx, path) in path_of.into_iter().enumerate() {
            scc_of.insert(path, scc_of_idx[idx]);
        }
        (scc_of, scc_size)
    }

    fn scc_id_of(scc_of: &FxHashMap<Utf8PathBuf, u32>, node: &str) -> u32 {
        *scc_of
            .get(&Utf8PathBuf::from(node))
            .unwrap_or_else(|| panic!("{node} was never visited"))
    }

    #[test]
    fn dag_has_no_shared_sccs() {
        // a -> b -> c, no cycle: every node should be alone in its own SCC.
        let g = graph(&[("a", &["b"]), ("b", &["c"]), ("c", &[])]);
        let (scc_of, scc_size) = compute_sccs(&g);
        let ids = [
            scc_id_of(&scc_of, "a"),
            scc_id_of(&scc_of, "b"),
            scc_id_of(&scc_of, "c"),
        ];
        assert_ne!(ids[0], ids[1]);
        assert_ne!(ids[1], ids[2]);
        assert_ne!(ids[0], ids[2]);
        for id in ids {
            assert_eq!(scc_size[id as usize], 1);
        }
    }

    #[test]
    fn two_node_cycle_shares_one_scc() {
        // a <-> b, plus an unrelated c that imports into the cycle but isn't
        // part of it (mirrors the `valid.js` -> {invalidFoobar <-> invalidBaz}
        // fixture: importing *into* a cycle doesn't make you a member of it).
        let g = graph(&[("a", &["b"]), ("b", &["a"]), ("c", &["a"])]);
        let (scc_of, scc_size) = compute_sccs(&g);
        let a = scc_id_of(&scc_of, "a");
        let b = scc_id_of(&scc_of, "b");
        let c = scc_id_of(&scc_of, "c");
        assert_eq!(a, b, "a and b form a 2-cycle, must share an SCC");
        assert_eq!(scc_size[a as usize], 2);
        assert_ne!(c, a, "c only imports into the cycle, it isn't part of it");
        assert_eq!(scc_size[c as usize], 1);
    }

    #[test]
    fn three_node_cycle_shares_one_scc() {
        let g = graph(&[("a", &["b"]), ("b", &["c"]), ("c", &["a"])]);
        let (scc_of, scc_size) = compute_sccs(&g);
        let a = scc_id_of(&scc_of, "a");
        assert_eq!(a, scc_id_of(&scc_of, "b"));
        assert_eq!(a, scc_id_of(&scc_of, "c"));
        assert_eq!(scc_size[a as usize], 3);
    }

    #[test]
    fn disjoint_cycles_get_different_sccs() {
        let g = graph(&[
            ("a", &["b"]),
            ("b", &["a"]),
            ("c", &["d"]),
            ("d", &["c"]),
        ]);
        let (scc_of, _) = compute_sccs(&g);
        assert_eq!(scc_id_of(&scc_of, "a"), scc_id_of(&scc_of, "b"));
        assert_eq!(scc_id_of(&scc_of, "c"), scc_id_of(&scc_of, "d"));
        assert_ne!(scc_id_of(&scc_of, "a"), scc_id_of(&scc_of, "c"));
    }

    #[test]
    fn diamond_shape_has_no_cycle() {
        //   a
        //  / \
        // b   c
        //  \ /
        //   d
        let g = graph(&[
            ("a", &["b", "c"]),
            ("b", &["d"]),
            ("c", &["d"]),
            ("d", &[]),
        ]);
        let (scc_of, scc_size) = compute_sccs(&g);
        let ids: Vec<u32> = ["a", "b", "c", "d"]
            .iter()
            .map(|n| scc_id_of(&scc_of, n))
            .collect();
        for &id in &ids {
            assert_eq!(scc_size[id as usize], 1);
        }
        // all four distinct
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                assert_ne!(ids[i], ids[j]);
            }
        }
    }

    #[test]
    fn larger_cycle_with_a_chord() {
        // a -> b -> c -> d -> a (4-cycle), plus a chord b -> d.
        // Still one SCC of size 4: the chord doesn't split it.
        let g = graph(&[
            ("a", &["b"]),
            ("b", &["c", "d"]),
            ("c", &["d"]),
            ("d", &["a"]),
        ]);
        let (scc_of, scc_size) = compute_sccs(&g);
        let a = scc_id_of(&scc_of, "a");
        for n in ["b", "c", "d"] {
            assert_eq!(a, scc_id_of(&scc_of, n));
        }
        assert_eq!(scc_size[a as usize], 4);
    }

    fn scc_data_from(edges: &[(&str, &[&str])]) -> super::SccData {
        let g = graph(edges);
        let (scc_of, scc_size) = compute_sccs(&g);
        super::SccData { scc_of, scc_size }
    }

    #[test]
    fn same_cyclic_scc_true_within_a_cycle() {
        let data = scc_data_from(&[("a", &["b"]), ("b", &["a"])]);
        assert!(data.same_cyclic_scc(&Utf8PathBuf::from("a"), &Utf8PathBuf::from("b")));
        assert!(data.same_cyclic_scc(&Utf8PathBuf::from("b"), &Utf8PathBuf::from("a")));
    }

    #[test]
    fn same_cyclic_scc_false_for_an_import_out_of_a_cycle() {
        // Mirrors the `valid.js` fixture: a file that is itself cyclic (a<->b)
        // *also* imports an unrelated file c. Checking specifically the a->c
        // edge must say "no cycle here", even though a is cyclic via b.
        let data = scc_data_from(&[("a", &["b", "c"]), ("b", &["a"]), ("c", &[])]);
        assert!(data.same_cyclic_scc(&Utf8PathBuf::from("a"), &Utf8PathBuf::from("b")));
        assert!(!data.same_cyclic_scc(&Utf8PathBuf::from("a"), &Utf8PathBuf::from("c")));
    }

    #[test]
    fn same_cyclic_scc_false_for_a_lone_self_import() {
        // a imports only itself, nothing else cyclic exists: matches the
        // pre-existing "isolated self-import" exception (issue #6569) --
        // should not be treated as a cycle.
        let data = scc_data_from(&[("a", &[])]);
        assert!(!data.same_cyclic_scc(&Utf8PathBuf::from("a"), &Utf8PathBuf::from("a")));
    }

    #[test]
    fn same_cyclic_scc_false_across_disjoint_cycles() {
        let data = scc_data_from(&[
            ("a", &["b"]),
            ("b", &["a"]),
            ("c", &["d"]),
            ("d", &["c"]),
        ]);
        assert!(!data.same_cyclic_scc(&Utf8PathBuf::from("a"), &Utf8PathBuf::from("c")));
    }

    #[test]
    fn same_cyclic_scc_false_for_unknown_paths() {
        let data = scc_data_from(&[("a", &["b"]), ("b", &["a"])]);
        assert!(!data.same_cyclic_scc(&Utf8PathBuf::from("a"), &Utf8PathBuf::from("nonexistent")));
    }
}
