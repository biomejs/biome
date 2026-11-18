use camino::Utf8Path;

/// Returns `true` if `path` is a YAML file directly inside a `.github/workflows` directory.
pub fn is_github_workflow(path: &Utf8Path) -> bool {
    if !matches!(path.extension(), Some("yml" | "yaml")) {
        return false;
    }
    let mut ancestors = path.ancestors().skip(1).filter_map(Utf8Path::file_name);
    ancestors.next() == Some("workflows") && ancestors.next() == Some(".github")
}

/// Returns `true` if `path` is a GitHub action metadata file, named `action.yml` or `action.yaml`.
pub fn is_github_action(path: &Utf8Path) -> bool {
    matches!(path.file_name(), Some("action.yml" | "action.yaml"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn detects_workflow_paths() {
        for path in [
            ".github/workflows/ci.yml",
            "repo/.github/workflows/ci.yaml",
            "/abs/.github/workflows/release.yml",
        ] {
            assert!(is_github_workflow(Utf8Path::new(path)), "{path}");
        }
        for path in [
            "ci.yml",
            ".github/ci.yml",
            ".github/workflows/ci.json",
            ".github/workflows/nested/ci.yml",
            "github/workflows/ci.yml",
            ".github/actions/setup/action.yml",
        ] {
            assert!(!is_github_workflow(Utf8Path::new(path)), "{path}");
        }
    }

    #[test]
    fn detects_action_paths() {
        for path in [
            "action.yml",
            "action.yaml",
            ".github/actions/setup/action.yml",
            "/abs/repo/action.yaml",
        ] {
            assert!(is_github_action(Utf8Path::new(path)), "{path}");
        }
        for path in [
            "actions.yml",
            "action.json",
            "my-action.yml",
            ".github/workflows/action",
            ".github/workflows/ci.yml",
        ] {
            assert!(!is_github_action(Utf8Path::new(path)), "{path}");
        }
    }
}
