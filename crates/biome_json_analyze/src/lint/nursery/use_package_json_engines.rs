use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_json_syntax::JsonRoot;
use biome_rule_options::use_package_json_engines::UsePackageJsonEnginesOptions;

use crate::utils::is_package_json;

declare_lint_rule! {
    /// Require the `engines` field in `package.json`.
    ///
    /// The `engines` field lists the versions of Node.js and other tools that a package works
    /// with. Package managers such as npm read this field and warn when someone installs the
    /// package with a version outside that range. Without the field, nobody gets that warning,
    /// and the package can fail in ways that are hard to connect to an unsupported version.
    ///
    /// The rule only checks that the field exists at the top level of the file. It does not check
    /// what the field contains. It applies to every `package.json` file, including packages that
    /// set `"private": true`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,file=package.json,expect_diagnostic
    /// {
    ///     "name": "my-package",
    ///     "version": "1.0.0"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///     "name": "my-package",
    ///     "version": "1.0.0",
    ///     "engines": {
    ///         "node": ">=20"
    ///     }
    /// }
    /// ```
    ///
    pub UsePackageJsonEngines {
        version: "next",
        name: "usePackageJsonEngines",
        language: "json",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintPackageJson("require-engines").same()],
    }
}

impl Rule for UsePackageJsonEngines {
    type Query = Ast<JsonRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UsePackageJsonEnginesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !is_package_json(ctx.file_path()) {
            return None;
        }

        let value = ctx.query().value().ok()?;
        let object = value.as_json_object_value()?;

        if object.find_member("engines").is_some() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let value = ctx.query().value().ok()?;
        let object = value.as_json_object_value()?;
        let l_curly_token = object.l_curly_token().ok()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                l_curly_token.text_trimmed_range(),
                markup! {
                    "This "<Emphasis>"package.json"</Emphasis>" file is missing the "<Emphasis>"engines"</Emphasis>" field."
                },
            )
            .note(markup! {
                "The "<Emphasis>"engines"</Emphasis>" field tells package managers which versions of Node.js and other tools the package works with. Without it, the package can be installed with an unsupported version and no warning is shown."
            })
            .note(markup! {
                "Add an "<Emphasis>"engines"</Emphasis>" field that lists the supported versions, for example "<Emphasis>"\"engines\": { \"node\": \">=20\" }"</Emphasis>"."
            }),
        )
    }
}
