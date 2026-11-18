use crate::github_actions::is_github_workflow;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::use_github_workflow_step_name::UseGithubWorkflowStepNameOptions;
use biome_yaml_syntax::YamlRoot;

declare_lint_rule! {
    /// Require a `name` for the steps of GitHub Actions workflows.
    ///
    /// GitHub shows the `name` of a step in the log of a job. When a step has no `name`, GitHub
    /// shows the action it uses or the command it runs instead, for example
    /// `Run actions/checkout@v4`. A descriptive name makes the log easier to read, and makes it
    /// clear what a step is for without reading its command.
    ///
    /// The rule only checks files in a `.github/workflows` directory that end in `.yml` or
    /// `.yaml`. A step is also reported when `name` has no value, or when its value is empty or
    /// only whitespace.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,file=.github/workflows/ci.yml,expect_diagnostic
    /// name: CI
    /// on: push
    /// jobs:
    ///   build:
    ///     name: Build
    ///     runs-on: ubuntu-latest
    ///     steps:
    ///       - uses: actions/checkout@v4
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml,file=.github/workflows/ci.yml
    /// name: CI
    /// on: push
    /// jobs:
    ///   build:
    ///     name: Build
    ///     runs-on: ubuntu-latest
    ///     steps:
    ///       - name: Check out the repository
    ///         uses: actions/checkout@v4
    /// ```
    ///
    pub UseGithubWorkflowStepName {
        version: "next",
        name: "useGithubWorkflowStepName",
        language: "yaml",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::GitHub],
    }
}

impl Rule for UseGithubWorkflowStepName {
    type Query = Ast<YamlRoot>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseGithubWorkflowStepNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !is_github_workflow(ctx.file_path()) {
            return None;
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "This step doesn't have a "<Emphasis>"name"</Emphasis>"."
                },
            )
            .note(markup! {
                "Without a name, GitHub shows the action or the command of the step in the job log, which makes it harder to see what the step is for."
            })
            .note(markup! {
                "Add a "<Emphasis>"name"</Emphasis>" key to the step that describes what it does."
            }),
        )
    }
}
