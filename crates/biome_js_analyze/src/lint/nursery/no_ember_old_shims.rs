use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::JsImport;
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Disallow importing from deprecated Ember shim modules.
    ///
    /// These old shims were used for Ember < 2.0 and should be replaced with modern imports.
    /// The shim modules include:
    /// - `ember-data/` - use `@ember-data/` instead
    /// - `ember-legacy-built-in-components` - use `@glimmer/component` or modern alternatives
    /// - `ember-string` - use `@ember/string` instead
    /// - `ember-routing/` - use `@ember/routing/` instead
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import Model from 'ember-data/model';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import TextField from 'ember-legacy-built-in-components';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { dasherize } from 'ember-string';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import Route from 'ember-routing/route';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import Model from '@ember-data/model';
    /// ```
    ///
    /// ```js
    /// import Component from '@glimmer/component';
    /// ```
    ///
    /// ```js
    /// import { dasherize } from '@ember/string';
    /// ```
    ///
    /// ```js
    /// import Route from '@ember/routing/route';
    /// ```
    ///
    pub NoEmberOldShims {
        version: "next",
        name: "noEmberOldShims",
        language: "js",
        recommended: true,
    }
}

impl Rule for NoEmberOldShims {
    type Query = Ast<JsImport>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import = ctx.query();

        // Get the import source
        let import_clause = import.import_clause().ok()?;
        let source = import_clause.source().ok()?;
        let source_text = source.inner_string_text().ok()?;
        let path = source_text.text();

        // Check if the import path contains any deprecated shim modules
        if path.starts_with("ember-data/")
            || path == "ember-legacy-built-in-components"
            || path == "ember-string"
            || path.starts_with("ember-routing/")
        {
            return Some(import.range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Don't import from deprecated Ember shim modules."
                },
            )
            .note(markup! {
                "Use modern Ember module imports instead."
            })
            .note(markup! {
                "See: "<Hyperlink href="https://github.com/ember-cli/ember-rfc176-data">"https://github.com/ember-cli/ember-rfc176-data"</Hyperlink>
            }),
        )
    }
}
