use crate::github_actions::is_github_workflow;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::TextRange;
use biome_rule_options::use_github_workflow_name::UseGithubWorkflowNameOptions;
use biome_yaml_syntax::YamlRoot;

declare_lint_rule! {
    /// Require a `name` for GitHub Actions workflows.
    ///
    /// GitHub shows the `name` of a workflow in the Actions tab, in pull request checks, and in
    /// notifications. When a workflow has no `name`, GitHub shows the path of the file instead,
    /// for example `.github/workflows/ci.yml`. A short, descriptive name makes the workflow easier
    /// to find and to tell apart from the other workflows of the repository.
    ///
    /// The rule only checks files in a `.github/workflows` directory that end in `.yml` or
    /// `.yaml`. It checks the top-level `name` of the workflow. It doesn't check the names of jobs
    /// and steps. A workflow is also reported when `name` has no value, or when its value is
    /// empty or only whitespace.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```yaml,file=.github/workflows/ci.yml,expect_diagnostic
    /// on: push
    /// jobs:
    ///   build:
    ///     runs-on: ubuntu-latest
    ///     steps:
    ///       - run: echo "Hello"
    /// ```
    ///
    /// ### Valid
    ///
    /// ```yaml,file=.github/workflows/ci.yml
    /// name: CI
    /// on: push
    /// jobs:
    ///   build:
    ///     runs-on: ubuntu-latest
    ///     steps:
    ///       - run: echo "Hello"
    /// ```
    ///
    pub UseGithubWorkflowName {
        version: "next",
        name: "useGithubWorkflowName",
        language: "yaml",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::GitHub],
    }
}

impl Rule for UseGithubWorkflowName {
    type Query = Ast<YamlRoot>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseGithubWorkflowNameOptions;

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
                    "This workflow doesn't have a "<Emphasis>"name"</Emphasis>"."
                },
            )
            .note(markup! {
                "Without a name, GitHub shows the path of the workflow file in the Actions tab and in pull request checks, which makes workflows harder to tell apart."
            })
            .note(markup! {
                "Add a top-level "<Emphasis>"name"</Emphasis>" key that describes what the workflow does."
            }),
        )
    }
}
