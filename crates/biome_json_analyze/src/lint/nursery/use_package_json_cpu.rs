use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_json_syntax::JsonRoot;
use biome_rule_options::use_package_json_cpu::UsePackageJsonCpuOptions;

use crate::utils::is_package_json;

declare_lint_rule! {
    /// Require the `cpu` field in `package.json`.
    ///
    /// The `cpu` field lists the processor types that a package can run on, such as `x64` or
    /// `arm64`. Package managers read it to avoid installing the package on a processor that the
    /// package doesn't support. Without the field, the package is installed on every processor,
    /// and a package that can't run there fails later, when it is used.
    ///
    /// This rule reports a `package.json` file whose outermost object has no `cpu` field. A `cpu`
    /// field inside another object doesn't count. The rule doesn't check the value of the field.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic,file=package.json
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
    ///     "cpu": ["x64", "arm64"]
    /// }
    /// ```
    ///
    pub UsePackageJsonCpu {
        version: "next",
        name: "usePackageJsonCpu",
        language: "json",
        recommended: false,
        sources: &[RuleSource::EslintPackageJson("require-cpu").same()],
    }
}

impl Rule for UsePackageJsonCpu {
    type Query = Ast<JsonRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UsePackageJsonCpuOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !is_package_json(ctx.file_path()) {
            return None;
        }

        let value = ctx.query().value().ok()?;
        let object = value.as_json_object_value()?;
        object.find_member("cpu").is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let value = ctx.query().value().ok()?;
        let object = value.as_json_object_value()?;
        let range = object.l_curly_token().ok()?.text_trimmed_range();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"package.json"</Emphasis>" file is missing the "<Emphasis>"cpu"</Emphasis>" field."
                },
            )
            .note(markup! {
                "Package managers read this field to avoid installing the package on a processor that the package doesn't support."
            })
            .note(markup! {
                "Add a "<Emphasis>"cpu"</Emphasis>" field that lists the processor types the package supports, for example "<Emphasis>"[\"x64\", \"arm64\"]"</Emphasis>"."
            }),
        )
    }
}
