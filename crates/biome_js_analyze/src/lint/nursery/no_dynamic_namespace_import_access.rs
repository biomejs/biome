use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{JsComputedMemberExpression, JsImportNamespaceClause};
use biome_rowan::{AstNode, TextRange};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow accessing namespace imports dynamically.
    ///
    /// Accessing namespace imports dynamically can prevent efficient tree shaking and increase bundle size.
    /// This happens because the bundler cannot determine which parts of the namespace are used at compile time,
    /// so it must include the entire namespace in the bundle.
    ///
    /// Instead, consider using named imports or if that is not possible
    /// access the namespaced import properties statically.
    ///
    /// If you want to completely disallow namespace imports, consider using the [noNamespaceImport](https://biomejs.dev/linter/rules/no-namespace-import/) rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import * as foo from "foo"
    /// foo["bar"]
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import * as foo from "foo"
    /// const key = "bar"
    /// foo[key]
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import * as foo from "foo"
    /// foo.bar
    /// ```
    ///
    /// ```js
    /// import { bar } from "foo"
    /// bar
    /// ```
    ///
    /// ```js
    /// import messages from "i18n"
    /// const knownMessagesMap = {
    ///  hello: messages.hello,
    ///  goodbye: messages.goodbye
    /// }
    ///
    /// const dynamicKey = "hello"
    /// knownMessagesMap[dynamicKey]
    /// ```
    ///
    pub NoDynamicNamespaceImportAccess {
        version: "1.9.0",
        name: "noDynamicNamespaceImportAccess",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoDynamicNamespaceImportAccess {
    type Query = Semantic<JsImportNamespaceClause>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        find_dynamic_namespace_import_accesses(ctx)
            .map_or(Vec::new(), |x| x)
            .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                 "Avoid accessing namespace imports dynamically, it can prevent efficient tree shaking and increase bundle size."
            },
        )
        .note(markup! {
            "Prefer static property access or use named imports instead."
        });

        Some(diagnostic)
    }
}

fn find_dynamic_namespace_import_accesses(
    ctx: &RuleContext<NoDynamicNamespaceImportAccess>,
) -> Option<Vec<TextRange>> {
    let import_namespace_clause: &JsImportNamespaceClause = ctx.query();

    // Allow type import e.g. `import type * as foo from "foo"`
    if import_namespace_clause.type_token().is_some() {
        return None;
    }

    let specifier = import_namespace_clause.namespace_specifier().ok()?;
    let any_binding = specifier.local_name().ok()?;
    let identifier = any_binding.as_js_identifier_binding()?;
    let reads = identifier.all_reads(ctx.model());

    let ranges = reads
        .into_iter()
        .filter_map(|read| {
            let syntax = read.syntax().parent()?.parent()?;
            let node = JsComputedMemberExpression::cast(syntax)?;

            Some(node.range())
        })
        .collect::<Vec<_>>();

    Some(ranges)
}
