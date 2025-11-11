use crate::{CliDiagnostic, CliSession};
use biome_configuration::vcs::{GIT_IGNORE_FILE_NAME, IGNORE_FILE_NAME, VcsConfiguration};
use biome_configuration::{Configuration, FilesConfiguration};
use biome_console::fmt::{Display, Formatter};
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{Category, Diagnostic, PrintDiagnostic, Severity, category};
use biome_fs::ConfigName;
use biome_service::configuration::create_config;

pub(crate) fn init(session: CliSession, emit_jsonc: bool) -> Result<(), CliDiagnostic> {
    let fs = session.app.workspace.fs();
    let working_directory = fs.working_directory().unwrap_or_default();
    let mut config = Configuration::init();
    let mut vcs_enabled = false;
    let mut dist_enabled = false;
    if fs.path_exists(&working_directory.join(IGNORE_FILE_NAME))
        || fs.path_exists(&working_directory.join(GIT_IGNORE_FILE_NAME))
    {
        vcs_enabled = true;
        config.vcs = Some(VcsConfiguration::new_git_ignore());
    }

    if fs.path_exists(&working_directory.join("dist")) {
        dist_enabled = true;
        config.files = Some(FilesConfiguration {
            includes: Some(vec![
                "**".parse::<biome_glob::NormalizedGlob>().unwrap(),
                "!!**/dist".parse::<biome_glob::NormalizedGlob>().unwrap(),
            ]),
            ignore_unknown: None,
            max_size: None,
            experimental_scanner_ignores: None,
        })
    }

    create_config(fs, config, emit_jsonc)?;
    let file_created = if emit_jsonc {
        ConfigName::biome_jsonc()
    } else {
        ConfigName::biome_json()
    };
    let diagnostic = InitDiagnostic {
        dist_enabled,
        vcs_enabled,
        file_created,
    };
    session
        .app
        .console
        .log(markup! {{PrintDiagnostic::simple(&diagnostic)}});

    Ok(())
}

#[derive(Debug)]
struct InitDiagnostic {
    dist_enabled: bool,
    vcs_enabled: bool,
    file_created: &'static str,
}

impl Diagnostic for InitDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("init"))
    }

    fn severity(&self) -> Severity {
        Severity::Information
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup! {
            {self}
        })
    }
}

impl Display for InitDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::io::Result<()> {
        f.write_markup(markup! {"Welcome to Biome! Let's get you started...

"<Info><Emphasis>"Files created "</Emphasis></Info>"

  "<Dim>"- "</Dim><Emphasis>{self.file_created}</Emphasis>"
    Your project configuration. See "<Hyperlink href="https://biomejs.dev/reference/configuration">"https://biomejs.dev/reference/configuration"</Hyperlink>})?;

        if self.vcs_enabled {
            f.write_markup(markup!{
                "\n\nFound an ignore file. Biome enabled "<Hyperlink href="https://biomejs.dev/guides/integrate-in-vcs">"VCS integration."</Hyperlink>
            })?;
        }

        if self.dist_enabled {
            f.write_markup(markup!{
                "\n\nFound a "<Emphasis>"dist/"</Emphasis>" folder. Biome ignored it "<Hyperlink href="https://biomejs.dev/reference/configuration/#interaction-with-the-scanner">"using the double-exclude syntax."</Hyperlink>
            })?;
        }

        f.write_markup(markup! {<Info><Emphasis>"\n\nNext Steps "</Emphasis></Info>"

  "<Dim>"1."</Dim>" "<Emphasis>"Setup an editor extension"</Emphasis>"
     Get live errors as you type and format when you save.
     Learn more at "<Hyperlink href="https://biomejs.dev/guides/editors/first-party-extensions/">"https://biomejs.dev/guides/editors/first-party-extensions/"</Hyperlink>"

  "<Dim>"2."</Dim>" "<Emphasis>"Try a command"</Emphasis>"
     "<Italic>"biome check"</Italic>"  checks formatting, import sorting, and lint rules.
     "<Italic>"biome --help"</Italic>" displays the available commands.

  "<Dim>"3."</Dim>" "<Emphasis>"Migrate from ESLint and Prettier"</Emphasis>"
     "<Italic>"biome migrate eslint"</Italic>"   migrates your ESLint configuration to Biome.
     "<Italic>"biome migrate prettier"</Italic>" migrates your Prettier configuration to Biome.

  "<Dim>"4."</Dim>" "<Emphasis>"Read the documentation"</Emphasis>"
     Find guides and documentation at "<Hyperlink href="https://biomejs.dev/guides/getting-started/">"https://biomejs.dev/guides/getting-started/"</Hyperlink>"

  "<Dim>"5."</Dim>" "<Emphasis>"Get involved with the community"</Emphasis>"
     Ask questions and contribute on GitHub: "<Hyperlink href="https://github.com/biomejs/biome">"https://github.com/biomejs/biome"</Hyperlink>"
     Seek for help on Discord: "<Hyperlink href="https://biomejs.dev/chat">"https://biomejs.dev/chat"</Hyperlink>"
"
    })?;

        Ok(())
    }
}
