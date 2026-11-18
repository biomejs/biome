//! Detection of the continuous integration (CI) service running Biome.

use crate::Environment;

/// A continuous integration service, recognized by the variables it sets in
/// the environment of the jobs it runs.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CiProvider {
    GithubActions,
    Gitlab,
    Circleci,
    AzurePipelines,
    Buildkite,
    Jenkins,
    /// A service that sets the conventional `CI` variable, but none of the
    /// variables specific to the services above.
    Other,
}

impl CiProvider {
    /// Returns the CI service running the current process, or `None` when
    /// the environment doesn't belong to a CI job.
    ///
    /// The variables specific to each service take precedence over `CI`,
    /// which most services also set.
    pub fn from_environment(environment: &dyn Environment) -> Option<Self> {
        // GitHub documents the value as exactly `true`:
        // https://docs.github.com/actions/learn-github-actions/variables#default-environment-variables
        if environment.var("GITHUB_ACTIONS").as_deref() == Some("true") {
            Some(Self::GithubActions)
        } else if environment.is_enabled("GITLAB_CI") {
            Some(Self::Gitlab)
        } else if environment.is_enabled("CIRCLECI") {
            Some(Self::Circleci)
        } else if environment.is_enabled("TF_BUILD") {
            Some(Self::AzurePipelines)
        } else if environment.is_enabled("BUILDKITE") {
            Some(Self::Buildkite)
        } else if environment
            .var("JENKINS_URL")
            .is_some_and(|url| !url.trim().is_empty())
        {
            Some(Self::Jenkins)
        } else if environment.is_enabled("CI") {
            Some(Self::Other)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryEnvironment;

    fn detect(variables: &[(&str, &str)]) -> Option<CiProvider> {
        let environment = variables.iter().fold(
            MemoryEnvironment::default(),
            |environment, (name, value)| environment.with_var(*name, *value),
        );
        CiProvider::from_environment(&environment)
    }

    #[test]
    fn detects_each_provider() {
        let cases = [
            ("GITHUB_ACTIONS", "true", CiProvider::GithubActions),
            ("GITLAB_CI", "true", CiProvider::Gitlab),
            ("CIRCLECI", "true", CiProvider::Circleci),
            ("TF_BUILD", "True", CiProvider::AzurePipelines),
            ("BUILDKITE", "true", CiProvider::Buildkite),
            (
                "JENKINS_URL",
                "https://ci.example.com/",
                CiProvider::Jenkins,
            ),
        ];

        for (name, value, provider) in cases {
            assert_eq!(
                detect(&[(name, value), ("CI", "true")]),
                Some(provider),
                "{name}={value}"
            );
        }
    }

    #[test]
    fn generic_ci_variable_is_other() {
        assert_eq!(detect(&[("CI", "true")]), Some(CiProvider::Other));
        assert_eq!(detect(&[("CI", "1")]), Some(CiProvider::Other));
    }

    #[test]
    fn github_actions_requires_exact_true() {
        assert_eq!(detect(&[("GITHUB_ACTIONS", "false")]), None);
        assert_eq!(
            detect(&[("GITHUB_ACTIONS", "false"), ("CI", "true")]),
            Some(CiProvider::Other)
        );
    }

    #[test]
    fn not_ci() {
        assert_eq!(detect(&[]), None);
        assert_eq!(detect(&[("CI", "false")]), None);
        assert_eq!(detect(&[("CI", "0")]), None);
        assert_eq!(detect(&[("CI", "")]), None);
        assert_eq!(detect(&[("JENKINS_URL", "")]), None);
    }
}
