use crate::github_actions::is_github_workflow;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_github_workflow_name::UseGithubWorkflowNameOptions;
use biome_yaml_syntax::{
    AnyYamlBlockInBlockContent, AnyYamlBlockMapEntry, AnyYamlBlockNode, AnyYamlDocument,
    YamlBlockMapping, YamlRoot,
};

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
        domains: &[RuleDomain::Github],
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

        let root = ctx.query();
        let AnyYamlDocument::YamlDocument(document) = root.documents().iter().next()? else {
            return None;
        };
        let root_range = root.syntax().text_trimmed_range();
        let mapping = match document.node() {
            None => return Some(root_range),
            Some(AnyYamlBlockNode::YamlBlockInBlockNode(node)) => match node.content().ok()? {
                AnyYamlBlockInBlockContent::YamlBlockMapping(mapping) => mapping,
                _ => return Some(root_range),
            },
            // Flow mappings and bogus nodes aren't inspected, so they can't be reported.
            Some(_) => return None,
        };
        let Some((value, range)) = find_name_entry(&mapping) else {
            return Some(root_range);
        };
        let is_empty = match value {
            None => true,
            Some(value) => value
                .inner_string_text()
                .is_ok_and(|text| text.trim().is_empty()),
        };
        is_empty.then_some(range)
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

/// Returns the value and range of the `name` entry of `mapping`, matching both implicit
/// (`name: CI`) and explicit (`? name`) entries.
fn find_name_entry(mapping: &YamlBlockMapping) -> Option<(Option<AnyYamlBlockNode>, TextRange)> {
    mapping.entries().iter().find_map(|entry| match entry {
        AnyYamlBlockMapEntry::YamlBlockMapImplicitEntry(entry) => {
            let key = entry.key()?.inner_string_text().ok()?;
            (key.text() == "name").then(|| (entry.value(), entry.syntax().text_trimmed_range()))
        }
        AnyYamlBlockMapEntry::YamlBlockMapExplicitEntry(entry) => {
            let key = entry.key()?.inner_string_text().ok()?;
            (key.text() == "name").then(|| (entry.value(), entry.syntax().text_trimmed_range()))
        }
        AnyYamlBlockMapEntry::YamlBogusBlockMapEntry(_) => None,
    })
}
