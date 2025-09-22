use crate::services::module_graph::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::AnyJsImportLike;
use biome_module_graph::{JsImportPath, JsImportPhase, JsModuleInfo};
use biome_resolver::ResolvedPath;
use biome_rowan::AstNode;
use biome_rule_options::no_import_cycles::NoImportCyclesOptions;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;

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

impl Rule for NoImportCycles {
    type Query = ResolvedImports<AnyJsImportLike>;
    type State = Vec<String>;
    type Signals = Option<Self::State>;
    type Options = NoImportCyclesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module_info = ctx.module_info_for_path(ctx.file_path())?;
        let node = ctx.query();

        let JsImportPath {
            resolved_path,
            phase,
        } = module_info.get_import_path_by_js_node(node)?;

        let options = ctx.options();
        if options.ignore_types && *phase == JsImportPhase::Type {
            return None;
        }

        let resolved_path = resolved_path.as_path()?;
        let imports = ctx.module_info_for_path(resolved_path)?;

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

            match Utf8Path::new(path).strip_prefix(&cwd) {
                Ok(relative_path) => {
                    note.extend_with(markup!(<Info>{relative_path.as_str()}</Info>))
                }
                Err(_) => note.extend_with(markup!(<Info>{path}</Info>)),
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
    start_path: &Utf8Path,
    mut module_info: JsModuleInfo,
) -> Option<Vec<String>> {
    let options = ctx.options();
    let mut seen = FxHashSet::default();
    let mut stack: Vec<(ResolvedPath, JsModuleInfo)> = Vec::new();

    'outer: loop {
        for JsImportPath {
            resolved_path,
            phase,
        } in module_info.all_import_paths()
        {
            if options.ignore_types && phase == JsImportPhase::Type {
                continue;
            }

            let Some(path) = resolved_path.as_path() else {
                continue;
            };

            if !seen.insert(resolved_path.clone()) {
                continue;
            }

            if path == ctx.file_path() {
                // Return all the paths from `start_path` to `resolved_path`:
                let paths = Some(start_path.to_string())
                    .into_iter()
                    .chain(
                        stack
                            .iter()
                            .filter_map(|(path, _)| path.as_path())
                            .map(ToString::to_string),
                    )
                    .chain(Some(path.to_string()))
                    .collect();
                return Some(paths);
            }

            if let Some(next_module_info) = ctx.module_info_for_path(path) {
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
