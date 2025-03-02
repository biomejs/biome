use biome_console::fmt::Formatter;
use biome_console::{ConsoleExt, fmt, markup};
use biome_service::workspace::ServerInfo;

use crate::{CliDiagnostic, CliSession, VERSION};

/// Handle of the `version` command. Prints a more in detail version of biome.
pub(crate) fn full_version(session: CliSession) -> Result<(), CliDiagnostic> {
    session.app.console.log(markup! {
    "CLI:        "{VERSION}
    });

    match session.app.workspace.server_info() {
        None => {
            session.app.console.log(markup! {
                "Server:     "<Dim>"not connected"</Dim>
            });
        }
        Some(info) => {
            session.app.console.log(markup! {
"Server:
  Name:     "{info.name}"
  Version:  "{DisplayServerVersion(info)}
            });
        }
    };

    Ok(())
}

pub(super) struct DisplayServerVersion<'a>(pub &'a ServerInfo);

impl fmt::Display for DisplayServerVersion<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match &self.0.version {
            None => markup!(<Dim>"-"</Dim>).fmt(fmt),
            Some(version) => {
                write!(fmt, "{version}")
            }
        }
    }
}
