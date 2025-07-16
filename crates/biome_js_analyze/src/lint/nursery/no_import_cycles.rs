use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::AnyJsImportLike;
use biome_module_graph::{JsModuleInfo, ResolvedPath};
use biome_rowan::AstNode;
use biome_rule_options::no_import_cycles::NoImportCyclesOptions;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;

use crate::services::module_graph::ResolvedImports;

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
    /// **`foobar.js`**
    /// ```js
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
    /// **`baz.js`**
    /// ```js
    /// import { bar } from "./foobar.js";
    ///
    /// export function baz() {
    ///     bar();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// **`foo.js`**
    /// ```js
    /// import { baz } from "./baz.js";
    ///
    /// export function foo() {
    ///     baz();
    /// }
    /// ```
    ///
    /// **`bar.js`**
    /// ```js
    /// export function bar() {
    ///     console.log("foobar");
    /// }
    /// ```
    ///
    /// **`baz.js`**
    /// ```js
    /// import { bar } from "./bar.js";
    ///
    /// export function baz() {
    ///     bar();
    /// }
    /// ```
    ///
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
    type State = Box<[Box<str>]>;
    type Signals = Option<Self::State>;
    type Options = NoImportCyclesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module_info = ctx.module_info_for_path(ctx.file_path())?;

        let node = ctx.query();
        let resolved_path = module_info
            .get_import_path_by_js_node(node)
            .and_then(ResolvedPath::as_path)?;

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
) -> Option<Box<[Box<str>]>> {
    let mut seen = FxHashSet::default();
    let mut stack: Vec<(Box<str>, JsModuleInfo)> = Vec::new();

    'outer: loop {
        for resolved_path in module_info.all_import_paths() {
            let Some(path) = resolved_path.as_path() else {
                continue;
            };

            if !seen.insert(resolved_path.clone()) {
                continue;
            }

            if path == ctx.file_path() {
                // Return all the paths from `start_path` to `resolved_path`:
                let paths = Some(start_path.as_str())
                    .into_iter()
                    .map(Box::from)
                    .chain(stack.into_iter().map(|(path, _)| path))
                    .chain(Some(Box::from(path.as_str())))
                    .collect();
                return Some(paths);
            }

            if let Some(next_module_info) = ctx.module_info_for_path(path) {
                stack.push((path.as_str().into(), module_info));
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
