use std::collections::HashSet;

use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_dependency_graph::ModuleImports;
use biome_diagnostics::Severity;
use biome_js_syntax::{inner_string_text, AnyJsImportLike};
use biome_rowan::AstNode;
use camino::{Utf8Path, Utf8PathBuf};

use crate::services::dependency_graph::ResolvedImports;

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
        version: "next",
        name: "noImportCycles",
        language: "js",
        sources: &[
            RuleSource::EslintImport("no-cycle"),
        ],
        severity: Severity::Warning,
        recommended: false,
    }
}

impl Rule for NoImportCycles {
    type Query = ResolvedImports;
    type State = Vec<String>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let file_imports = ctx.imports_for_path(ctx.file_path())?;

        let node = ctx.query();
        let name_token = node.module_name_token()?;
        let specifier_text = inner_string_text(&name_token);
        let specifier = specifier_text.text();
        let import = if is_static_import(node) {
            file_imports.static_imports.get(specifier)
        } else {
            file_imports.dynamic_imports.get(specifier)
        }?;
        let resolved_path = import.resolved_path.as_ref().ok()?;

        let imports = ctx.imports_for_path(resolved_path)?;
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
    mut imports: ModuleImports,
) -> Option<Vec<String>> {
    let mut seen = HashSet::new();
    let mut stack = Vec::new();

    'outer: loop {
        while let Some((_specifier, import)) = imports.drain_one() {
            let Ok(resolved_path) = import.resolved_path else {
                continue;
            };

            if resolved_path == ctx.file_path() {
                // Return all the paths from `start_path` to `resolved_path`:
                let paths = Some(start_path.to_string())
                    .into_iter()
                    .chain(stack.into_iter().map(|(path, _)| path))
                    .chain(Some(resolved_path.into()))
                    .collect();
                return Some(paths);
            }

            // FIXME: Use `get_or_insert_with()` once it's stabilized.
            //        See: https://github.com/rust-lang/rust/issues/60896
            if seen.contains(resolved_path.as_str()) {
                continue;
            }

            seen.insert(resolved_path.to_string());

            if let Some(resolved_imports) = ctx.imports_for_path(&resolved_path) {
                stack.push((resolved_path.into(), imports));
                imports = resolved_imports;
                continue 'outer;
            }
        }

        match stack.pop() {
            Some((_previous_path, previous_imports)) => {
                imports = previous_imports;
            }
            None => break,
        }
    }

    None
}

fn is_static_import(node: &AnyJsImportLike) -> bool {
    matches!(node, AnyJsImportLike::JsModuleSource(_))
}
