use crate::{CliDiagnostic, CliSession};
use biome_configuration::Configuration;
use biome_console::{markup, ConsoleExt};
use biome_fs::ConfigName;
use biome_service::configuration::create_config;

pub(crate) fn init(session: CliSession, emit_jsonc: bool) -> Result<(), CliDiagnostic> {
    let fs = session.app.workspace.fs();
    create_config(fs, Configuration::init(), emit_jsonc)?;
    let file_created = if emit_jsonc {
        ConfigName::biome_jsonc()
    } else {
        ConfigName::biome_json()
    };
    session.app.console.log(markup! {
"
Welcome to Biome! Let's get you started...

"<Info><Emphasis>"Files created "</Emphasis></Info>"

  "<Dim>"- "</Dim><Emphasis>{file_created}</Emphasis>"
    Your project configuration. See "<Hyperlink href="https://biomejs.dev/reference/configuration">"https://biomejs.dev/reference/configuration"</Hyperlink>"

"<Info><Emphasis>"Next Steps "</Emphasis></Info>"

  "<Dim>"1."</Dim>" "<Emphasis>"Setup an editor extension"</Emphasis>"
     Get live errors as you type and format when you save.
     Learn more at "<Hyperlink href="https://biomejs.dev/guides/integrate-in-editor/">"https://biomejs.dev/guides/integrate-in-editor/"</Hyperlink>"

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
    });
    Ok(())
}
