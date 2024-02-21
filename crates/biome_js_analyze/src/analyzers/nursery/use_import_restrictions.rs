use biome_analyze::{
    context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::JsModuleSource;
use biome_rowan::{AstNode, TokenText};

const INDEX_BASENAMES: &[&str] = &["index", "mod"];

const SOURCE_EXTENSIONS: &[&str] = &["js", "ts", "cjs", "cts", "mjs", "mts", "jsx", "tsx"];

declare_rule! {
    /// Disallows package private imports.
    ///
    /// This rules enforces the following restrictions:
    ///
    /// ## Package private visibility
    ///
    /// All exported symbols, such as types, functions or other things that may be exported, are
    /// considered to be "package private". This means that modules that reside in the same
    /// directory, as well as submodules of those "sibling" modules, are allowed to import them,
    /// while any other modules that are further away in the file system are restricted from
    /// importing them. A symbol's visibility may be extended by re-exporting from an index file.
    ///
    /// Notes:
    ///
    /// * This rule only applies to relative imports. External dependencies are exempted.
    /// * This rule only applies to imports for JavaScript and TypeScript files. Imports for
    ///   resources such as images or CSS files are exempted.
    ///
    /// Source: https://github.com/uhyo/eslint-plugin-import-access
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // Attempt to import from `foo.js` from outside its `sub` module.
    /// import { fooPackageVariable } from "./sub/foo.js";
    /// ```
    /// ```js,expect_diagnostic
    /// // Attempt to import from `bar.ts` from outside its `aunt` module.
    /// import { barPackageVariable } from "../aunt/bar.ts";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Assumed to resolve to a JS/TS file.
    /// import { fooPackageVariable } from "./sub/foo";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // If the `sub/foo` module is inaccessible, so is its index file.
    /// import { fooPackageVariable } from "./sub/foo/index.js";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Imports within the same module are always allowed.
    /// import { fooPackageVariable } from "./foo.js";
    ///
    /// // Resources (anything other than JS/TS files) are exempt.
    /// import { barResource } from "../aunt/bar.png";
    ///
    /// // A parent index file is accessible like other modules.
    /// import { internal } from "../../index.js";
    ///
    /// // If the `sub` module is accessible, so is its index file.
    /// import { subPackageVariable } from "./sub/index.js";
    ///
    /// // Library imports are exempt.
    /// import useAsync from "react-use/lib/useAsync";
    /// ```
    ///
    pub UseImportRestrictions {
        version: "1.0.0",
        name: "useImportRestrictions",
        source: RuleSource::EslintImportAccess("eslint-plugin-import-access"),
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
    }
}

impl Rule for UseImportRestrictions {
    type Query = Ast<JsModuleSource>;
    type State = ImportRestrictionsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let Ok(path) = binding.inner_string_text() else {
            return None;
        };

        get_restricted_import(&path)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let ImportRestrictionsState { path, suggestion } = state;

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Importing package private symbols is prohibited from outside the module directory."
            },
        )
        .note(markup! {
            "Please import from "<Emphasis>{suggestion}</Emphasis>" instead "
            "(you may need to re-export the symbol(s) from "<Emphasis>{path}</Emphasis>")."
        });

        Some(diagnostic)
    }
}

pub struct ImportRestrictionsState {
    /// The path that is being restricted.
    path: String,

    /// Suggestion from which to import instead.
    suggestion: String,
}

fn get_restricted_import(module_path: &TokenText) -> Option<ImportRestrictionsState> {
    if !module_path.starts_with('.') {
        return None;
    }

    let mut path_parts: Vec<_> = module_path.text().split('/').collect();
    let mut index_filename = None;

    if let Some(extension) = get_extension(&path_parts) {
        if !SOURCE_EXTENSIONS.contains(&extension) {
            return None; // Resource files are exempt.
        }

        if let Some(basename) = get_basename(&path_parts) {
            if INDEX_BASENAMES.contains(&basename) {
                // We pop the index file because it shouldn't count as a path,
                // component, but we store the file name so we can add it to
                // both the reported path and the suggestion.
                index_filename = path_parts.last().copied();
                path_parts.pop();
            }
        }
    }

    let is_restricted = path_parts
        .iter()
        .filter(|&&part| part != "." && part != "..")
        .count()
        > 1;
    if !is_restricted {
        return None;
    }

    let mut suggestion_parts = path_parts[..path_parts.len() - 1].to_vec();

    // Push the index file if it exists. This makes sure the reported path
    // matches the import path exactly.
    if let Some(index_filename) = index_filename {
        path_parts.push(index_filename);

        // Assumes the user probably wants to use an index file that has the
        // same name as the original.
        suggestion_parts.push(index_filename);
    }

    Some(ImportRestrictionsState {
        path: path_parts.join("/"),
        suggestion: suggestion_parts.join("/"),
    })
}

fn get_basename<'a>(path_parts: &'_ [&'a str]) -> Option<&'a str> {
    path_parts.last().map(|&part| match part.find('.') {
        Some(dot_index) if dot_index > 0 && dot_index < part.len() - 1 => &part[..dot_index],
        _ => part,
    })
}

fn get_extension<'a>(path_parts: &'_ [&'a str]) -> Option<&'a str> {
    path_parts.last().and_then(|part| match part.find('.') {
        Some(dot_index) if dot_index > 0 && dot_index < part.len() - 1 => {
            Some(&part[dot_index + 1..])
        }
        _ => None,
    })
}
