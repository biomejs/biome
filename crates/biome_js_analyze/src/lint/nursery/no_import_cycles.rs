use std::collections::BTreeSet;

use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_dependency_graph::ModuleImports;
use biome_js_syntax::inner_string_text;
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

        let name_token = ctx.query().module_name_token()?;
        let specifier_text = inner_string_text(&name_token);
        let specifier = specifier_text.text();
        let import = file_imports
            .static_imports
            .get(specifier)
            .or_else(|| file_imports.dynamic_imports.get(specifier))?;
        let resolved_path = import.resolved_path.as_ref().ok()?;

        let imports = ctx.imports_for_path(resolved_path)?;
        find_cycle(
            ctx,
            imports,
            &mut BTreeSet::new(),
            vec![resolved_path.to_string()],
        )
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

fn find_cycle(
    ctx: &RuleContext<NoImportCycles>,
    imports: ModuleImports,
    seen: &mut BTreeSet<String>,
    mut stack: Vec<String>,
) -> Option<Vec<String>> {
    for (_specifier, import) in imports
        .static_imports
        .into_iter()
        .chain(imports.dynamic_imports)
    {
        let Ok(resolved_path) = import.resolved_path else {
            continue;
        };

        if resolved_path == ctx.file_path() {
            stack.push(resolved_path.into_string());
            return Some(stack);
        }

        if !seen.insert(resolved_path.to_string()) {
            continue;
        }

        let result = ctx.imports_for_path(&resolved_path).and_then(|imports| {
            let mut stack = stack.clone();
            stack.push(resolved_path.to_string());
            find_cycle(ctx, imports, seen, stack)
        });
        if let Some(cycle) = result {
            return Some(cycle);
        }
    }

    None
}
