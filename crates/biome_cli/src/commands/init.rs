use crate::{CliDiagnostic, CliSession};
use biome_console::{markup, ConsoleExt, HorizontalLine};
use biome_fs::ConfigName;
use biome_service::{create_config, PartialConfiguration};

pub(crate) fn init(mut session: CliSession, emit_jsonc: bool) -> Result<(), CliDiagnostic> {
    let fs = &mut session.app.fs;
    create_config(fs, PartialConfiguration::init(), emit_jsonc)?;
    let file_created = if emit_jsonc {
        format!("{}: ", ConfigName::biome_jsonc())
    } else {
        format!("{}: ", ConfigName::biome_json())
    };

    session.app.console.log(markup! {
"\n"<Inverse>"Welcome to Biome! Let's get you started..."</Inverse>"

"<Info><Emphasis>"Files created "</Emphasis></Info>{HorizontalLine::new(106)}"

  "<Dim>"- "</Dim><Emphasis>{file_created}</Emphasis>"Your project configuration. Documentation: "<Hyperlink href="https://biomejs.dev/reference/configuration">"https://biomejs.dev/reference/configuration"</Hyperlink>"

"<Info><Emphasis>"Next Steps "</Emphasis></Info>{HorizontalLine::new(109)}"

  "<Dim>"1."</Dim>" "<Emphasis>"Setup an editor extension"</Emphasis>"
     Get live errors as you type and format when you save. Learn more: "<Hyperlink href="https://biomejs.dev/guides/getting-started#editor-setup">"https://biomejs.dev/guides/getting-started#editor-setup"</Hyperlink>"

  "<Dim>"2."</Dim>" "<Emphasis>"Try a command"</Emphasis>"
     "<Italic>"biome ci"</Italic>" checks for lint errors and verifies formatting. Run " <Italic>"biome --help"</Italic>" for a full list of commands and options.

  "<Dim>"3."</Dim>" "<Emphasis>"Read the documentation"</Emphasis>"
     Our website serves as a comprehensive source of guides and documentation: "<Hyperlink href="https://biomejs.dev">"https://biomejs.dev"</Hyperlink>"

  "<Dim>"4."</Dim>" "<Emphasis>"Get involved in the community"</Emphasis>"
     Ask questions, get support, or contribute by participating on GitHub ("<Hyperlink href="https://github.com/biomejs/biome">"https://github.com/biomejs/biome"</Hyperlink>"),
     or join our community Discord ("<Hyperlink href="https://discord.gg/BypW39g6Yc">"https://discord.gg/BypW39g6Yc"</Hyperlink>")"
    });

    Ok(())
}
