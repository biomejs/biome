use crate::{CliDiagnostic, CliSession};
use rome_console::{markup, ConsoleExt, HorizontalLine};
use rome_service::configuration::Configuration;
use rome_service::create_config;

pub(crate) fn init(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let fs = &mut session.app.fs;
    create_config(fs, Configuration::default())?;

    session.app.console.log(markup! {
"\n"<Inverse>"Welcome to Biome! Let's get you started..."</Inverse>"

"<Info><Emphasis>"Files created "</Emphasis></Info>{HorizontalLine::new(106)}"

  "<Dim>"- "</Dim><Emphasis>"biome.json: "</Emphasis>"Your project configuration. Documentation: "<Hyperlink href="https://rome.tools/configuration">"https://rome.tools/configuration"</Hyperlink>"

"<Info><Emphasis>"Next Steps "</Emphasis></Info>{HorizontalLine::new(109)}"

  "<Dim>"1."</Dim>" "<Emphasis>"Setup an editor extension"</Emphasis>"
     Get live errors as you type and format when you save. Learn more: "<Hyperlink href="https://rome.tools/editors">"https://rome.tools/editors"</Hyperlink>"

  "<Dim>"2."</Dim>" "<Emphasis>"Try a command"</Emphasis>"
     "<Italic>"rome ci"</Italic>" checks for lint errors and verifies formatting. Run " <Italic>"rome --help"</Italic>" for a full list of commands and options.

  "<Dim>"3."</Dim>" "<Emphasis>"Read the documentation"</Emphasis>"
     Our website serves as a comprehensive source of guides and documentation: "<Hyperlink href="https://docs.rome.tools">"https://docs.rome.tools"</Hyperlink>"

  "<Dim>"4."</Dim>" "<Emphasis>"Get involved in the community"</Emphasis>"
     Ask questions, get support, or contribute by participating on GitHub ("<Hyperlink href="https://github.com/rome/tools">"https://github.com/rome/tools"</Hyperlink>"),
     or join our community Discord ("<Hyperlink href="https://discord.gg/rome">"https://discord.gg/rome"</Hyperlink>")"
    });

    Ok(())
}
