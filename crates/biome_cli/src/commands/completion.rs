//! Shell completion for the Biome CLI.
//!
//! Completion works in two halves. A short script, installed once per shell,
//! runs `biome` with bpaf's hidden `--bpaf-complete-rev=<N>` argument each
//! time the user presses Tab. bpaf then answers with the candidates for the
//! current command line. Because the script only forwards the command line, it
//! doesn't change when commands or options change.
//!
//! This module provides the `completion` command, which prints or installs
//! that script, and the candidates for argument values that bpaf can't derive
//! from the parser, such as rule names.

use crate::{CliDiagnostic, CliSession};
use biome_console::{ConsoleExt, markup};
use biome_fs::FileSystemExt;
use biome_service::documentation::RulesVisitor;
use camino::Utf8PathBuf;
use directories::BaseDirs;
use std::collections::BTreeMap;
use std::io;
use std::str::FromStr;

/// A shell that `biome completion` can produce a script for.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    Elvish,
}

impl CompletionShell {
    const ALL: [Self; 4] = [Self::Bash, Self::Zsh, Self::Fish, Self::Elvish];

    const fn as_str(self) -> &'static str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::Elvish => "elvish",
        }
    }

    /// Returns the completion script for this shell.
    // The scripts are byte-for-byte what bpaf prints for
    // `--bpaf-complete-style-<shell>` when the program is named `biome`. Each
    // script passes a `--bpaf-complete-rev` number that selects bpaf's output
    // format for that shell, so the scripts must change when bpaf changes
    // those numbers. The `completion_scripts_match_bpaf` test compares them
    // with bpaf's output.
    const fn script(self) -> &'static str {
        match self {
            Self::Bash => {
                r#"_bpaf_dynamic_completion()
{
    line="$1 --bpaf-complete-rev=8 ${COMP_WORDS[@]:1}"
    if [[ ${COMP_WORDS[-1]} == "" ]]; then
        line="${line} \"\""
    fi
    source <( eval ${line})
}
complete -o nosort -F _bpaf_dynamic_completion biome
"#
            }
            Self::Zsh => {
                r#"#compdef biome
local line
line="${words[1]} --bpaf-complete-rev=7 ${words[@]:1}"
if [[ ${words[-1]} == "" ]]; then
    line="${line} \"\""
fi
source <(eval ${line})

"#
            }
            Self::Fish => {
                r#"function _bpaf_dynamic_completion
    set -l current (commandline --tokenize --current-process)
    set -l tmpline --bpaf-complete-rev=9 $current[2..]
    if test (commandline --current-process) != (string trim (commandline --current-process))
        set tmpline $tmpline ""
    end
    eval $current[1] \"$tmpline\"
end

complete --no-files --command biome --arguments '(_bpaf_dynamic_completion)'

"#
            }
            Self::Elvish => {
                r#"set edit:completion:arg-completer[biome] = { |@args| var args = $args[1..];
     var @lines = ( biome --bpaf-complete-rev=1 $@args );
     use str;
     for line $lines {
         var @arg = (str:split "\t" $line)
         try {
             edit:complex-candidate $arg[0] &display=( printf "%-19s %s" $arg[0] $arg[1] )
         } catch {
             edit:complex-candidate $line
         }
     }
}
"#
            }
        }
    }

    /// Returns where `--write` installs the script for this shell, and what
    /// the user must do afterward for the shell to load it.
    ///
    /// Bash and fish load scripts from these directories automatically. Zsh
    /// has no such directory, so the script goes to `~/.zfunc` and the user
    /// adds that directory to `fpath`. Elvish loads completions only from its
    /// configuration file, which Biome doesn't edit, so this returns an error.
    fn install_target(self) -> Result<InstallTarget, CliDiagnostic> {
        fn home_directory_not_found() -> CliDiagnostic {
            CliDiagnostic::io_error(io::Error::new(
                io::ErrorKind::NotFound,
                "Biome can't find a home directory.",
            ))
        }
        let target = match self {
            Self::Bash => InstallTarget {
                path: UserDirectories::new()
                    .ok_or_else(home_directory_not_found)?
                    .data
                    .join("bash-completion/completions/biome"),
                next_step: "Restart the shell to enable them. Bash loads them when the bash-completion package is installed.",
            },
            Self::Zsh => InstallTarget {
                path: UserDirectories::new()
                    .ok_or_else(home_directory_not_found)?
                    .home
                    .join(".zfunc/_biome"),
                next_step: "To enable them, add this line to your .zshrc before compinit runs, then restart the shell:\n  fpath+=(~/.zfunc)",
            },
            Self::Fish => InstallTarget {
                path: UserDirectories::new()
                    .ok_or_else(home_directory_not_found)?
                    .config
                    .join("fish/completions/biome.fish"),
                next_step: "Restart the shell to enable them.",
            },
            Self::Elvish => {
                return Err(CliDiagnostic::incompatible_arguments(
                    "--write",
                    "elvish",
                    "Elvish loads completions from rc.elv, which Biome doesn't edit. Run `biome completion elvish` and add its output to rc.elv instead.",
                ));
            }
        };
        Ok(target)
    }
}

impl FromStr for CompletionShell {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|shell| shell.as_str() == value)
            .ok_or_else(|| "expected one of bash, zsh, fish, or elvish".to_string())
    }
}

struct InstallTarget {
    path: Utf8PathBuf,
    next_step: &'static str,
}

/// The directories that bash-completion and fish load completion scripts from.
struct UserDirectories {
    home: Utf8PathBuf,
    /// `$XDG_DATA_HOME`, which defaults to `~/.local/share`.
    data: Utf8PathBuf,
    /// `$XDG_CONFIG_HOME`, which defaults to `~/.config`.
    config: Utf8PathBuf,
}

impl UserDirectories {
    /// Returns `None` when the home directory is unknown or isn't a UTF-8 path.
    fn new() -> Option<Self> {
        let base = BaseDirs::new()?;
        let home = Utf8PathBuf::from_path_buf(base.home_dir().to_path_buf()).ok()?;
        // bash-completion and fish use the XDG base directories on every
        // platform, but `directories` follows them only on Linux. On macOS it
        // returns `~/Library/Application Support`, where neither shell looks.
        let (data, config) = cfg_select! {
            target_os = "linux" => {
                (
                    Utf8PathBuf::from_path_buf(base.data_dir().to_path_buf()).ok()?,
                    Utf8PathBuf::from_path_buf(base.config_dir().to_path_buf()).ok()?,
                )
            }
            _ => (home.join(".local/share"), home.join(".config")),
        };
        Some(Self { home, data, config })
    }
}

/// Runs the `completion` command.
pub(crate) fn completion(
    session: CliSession,
    shell: CompletionShell,
    write: bool,
) -> Result<(), CliDiagnostic> {
    let console = session.app.console;

    if !write {
        console.append_raw(shell.script());
        return Ok(());
    }

    let fs = session.app.workspace.fs();
    let target = shell.install_target()?;
    if let Some(directory) = target.path.parent() {
        fs.create_dir_all(directory)?;
    }
    fs.create(&target.path)?
        .set_content(shell.script().as_bytes())?;

    console.log(markup! {
        "Wrote "{shell.as_str()}" completions to "<Emphasis>{target.path.as_str()}</Emphasis>".\n"{target.next_step}
    });
    Ok(())
}

/// Converts an argument value that was read as text into its typed form.
///
/// Arguments with suggested values are read as text first, because bpaf calls
/// a suggestion function only after the partial value converts successfully,
/// and a partial name such as `noDeb` isn't a valid rule selector.
pub(crate) fn parse_value<T: FromStr>(value: String) -> Result<T, T::Err> {
    value.parse()
}

/// Suggests shell names for `biome completion`.
#[expect(
    clippy::ptr_arg,
    reason = "bpaf passes the partial value with the type of the argument"
)]
pub(crate) fn complete_shell_name(input: &String) -> Vec<(&'static str, Option<&'static str>)> {
    CompletionShell::ALL
        .into_iter()
        .map(CompletionShell::as_str)
        .filter(|name| name.starts_with(input.as_str()))
        .map(|name| (name, None))
        .collect()
}

/// Suggests rule names and `daemon-logs` for `biome explain`.
#[expect(
    clippy::ptr_arg,
    reason = "bpaf passes the partial value with the type of the argument"
)]
pub(crate) fn complete_explain_name(input: &String) -> Vec<(String, Option<String>)> {
    let mut candidates = BTreeMap::new();
    candidates.insert(
        "daemon-logs".to_string(),
        "Prints the path of the daemon server log directory",
    );
    for rule in RulesVisitor::new().rules() {
        candidates.insert(rule.name().to_string(), rule.summary());
    }
    filter_candidates(candidates, input)
}

/// Suggests values for `--only` and `--skip` of `biome lint`: lint rules,
/// lint rule groups, domains, and `plugin`.
#[expect(
    clippy::ptr_arg,
    reason = "bpaf passes the partial value with the type of the argument"
)]
pub(crate) fn complete_lint_selector(input: &String) -> Vec<(String, Option<String>)> {
    complete_selector(input, false)
}

/// Suggests values for `--only` and `--skip` of `biome check` and `biome ci`:
/// the values for `biome lint`, plus assist actions and their groups.
#[expect(
    clippy::ptr_arg,
    reason = "bpaf passes the partial value with the type of the argument"
)]
pub(crate) fn complete_analyzer_selector(input: &String) -> Vec<(String, Option<String>)> {
    complete_selector(input, true)
}

/// Suggests `group/rule` selectors once the input contains `/`, and bare rule
/// names, groups, domains, and `plugin` otherwise. Users usually remember a
/// rule's name but not its group, so bare names are offered first.
fn complete_selector(input: &str, include_actions: bool) -> Vec<(String, Option<String>)> {
    let qualified = input.contains('/');
    let mut candidates = BTreeMap::new();
    let rules = RulesVisitor::new();
    let rules = rules
        .rules()
        .filter(|rule| include_actions || !rule.is_action());
    for rule in rules {
        if qualified {
            candidates.insert(format!("{}/{}", rule.group(), rule.name()), rule.summary());
            continue;
        }
        candidates.insert(rule.name().to_string(), rule.summary());
        let group_description = if rule.is_action() {
            "Group of assist actions"
        } else {
            "Group of lint rules"
        };
        candidates.insert(rule.group().to_string(), group_description);
        for domain in rule.domains() {
            candidates.insert(domain.as_str().to_string(), "Domain of lint rules");
        }
    }
    if !qualified {
        candidates.insert("plugin".to_string(), "All analyzer plugins");
    }
    filter_candidates(candidates, input)
}

fn filter_candidates(
    candidates: BTreeMap<String, &'static str>,
    input: &str,
) -> Vec<(String, Option<String>)> {
    candidates
        .into_iter()
        .filter(|(candidate, _)| candidate.starts_with(input))
        .map(|(candidate, description)| (candidate, Some(description.to_string())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_configuration::analyzer::AnalyzerSelector;

    fn candidates(suggestions: Vec<(String, Option<String>)>) -> Vec<String> {
        suggestions
            .into_iter()
            .map(|(candidate, _)| candidate)
            .collect()
    }

    #[test]
    fn install_targets_are_in_the_shell_directories() {
        let bash = CompletionShell::Bash.install_target().unwrap().path;
        let zsh = CompletionShell::Zsh.install_target().unwrap().path;
        let fish = CompletionShell::Fish.install_target().unwrap().path;

        assert!(
            bash.ends_with("bash-completion/completions/biome"),
            "{bash}"
        );
        assert!(zsh.ends_with(".zfunc/_biome"), "{zsh}");
        assert!(fish.ends_with("fish/completions/biome.fish"), "{fish}");
        assert!(CompletionShell::Elvish.install_target().is_err());

        if !cfg!(target_os = "linux") {
            let home = BaseDirs::new().unwrap().home_dir().to_path_buf();
            assert_eq!(
                bash,
                home.join(".local/share/bash-completion/completions/biome")
            );
            assert_eq!(fish, home.join(".config/fish/completions/biome.fish"));
        }
    }

    #[test]
    fn selector_suggestions_parse_as_selectors() {
        let suggestions = candidates(complete_analyzer_selector(&String::new()));
        let qualified = candidates(complete_analyzer_selector(&"a11y/".to_string()));
        assert!(!qualified.is_empty());
        for suggestion in suggestions.iter().chain(&qualified) {
            assert!(
                suggestion.parse::<AnalyzerSelector>().is_ok(),
                "`{suggestion}` isn't a valid selector"
            );
        }
    }

    #[test]
    fn selector_suggestions_match_partial_input() {
        let suggestions = candidates(complete_lint_selector(&"noDebug".to_string()));
        assert_eq!(suggestions, ["noDebugger"]);

        let suggestions = candidates(complete_lint_selector(&"suspicious/noDebug".to_string()));
        assert_eq!(suggestions, ["suspicious/noDebugger"]);

        let suggestions = candidates(complete_lint_selector(&"react".to_string()));
        assert_eq!(suggestions, ["react", "reactNative"]);
    }

    #[test]
    fn lint_selectors_exclude_assist_actions() {
        let lint = candidates(complete_lint_selector(&"useSortedKeys".to_string()));
        assert!(lint.is_empty(), "got {lint:?}");

        let analyzer = candidates(complete_analyzer_selector(&"useSortedKeys".to_string()));
        assert_eq!(analyzer, ["useSortedKeys"]);
    }

    #[test]
    fn explain_suggestions_include_daemon_logs_and_rules() {
        let suggestions = candidates(complete_explain_name(&"d".to_string()));
        assert!(suggestions.contains(&"daemon-logs".to_string()));

        let suggestions = candidates(complete_explain_name(&"noDebug".to_string()));
        assert_eq!(suggestions, ["noDebugger"]);
    }
}
