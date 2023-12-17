use biome_console::fmt::{Display, Formatter};
use biome_console::{fmt, markup, ConsoleExt, HorizontalLine, Markup};
use biome_diagnostics::termcolor::{ColorChoice, WriteColor};
use biome_diagnostics::{termcolor, PrintDescription};
use biome_fs::FileSystem;
use biome_service::configuration::{load_configuration, LoadedConfiguration};
use biome_service::workspace::{client, RageEntry, RageParams};
use biome_service::{ConfigurationBasePath, DynRef, Workspace};
use std::{env, io, ops::Deref};
use tokio::runtime::Runtime;

use crate::commands::daemon::read_most_recent_log_file;
use crate::service::enumerate_pipes;
use crate::{service, CliDiagnostic, CliSession, VERSION};

/// Handler for the `rage` command
pub(crate) fn rage(session: CliSession, daemon_logs: bool) -> Result<(), CliDiagnostic> {
    let terminal_supports_colors = termcolor::BufferWriter::stdout(ColorChoice::Auto)
        .buffer()
        .supports_color();

    session.app.console.log(markup!("CLI:\n"
    {KeyValuePair("Version", markup!({VERSION}))}
    {KeyValuePair("Color support", markup!({DebugDisplay(terminal_supports_colors)}))}

    {Section("Platform")}
    {KeyValuePair("CPU Architecture", markup!({std::env::consts::ARCH}))}
    {KeyValuePair("OS", markup!({std::env::consts::OS}))}

    {Section("Environment")}
    {EnvVarOs("BIOME_LOG_DIR")}
    {EnvVarOs("NO_COLOR")}
    {EnvVarOs("TERM")}
    {EnvVarOs("JS_RUNTIME_VERSION")}
    {EnvVarOs("JS_RUNTIME_NAME")}
    {EnvVarOs("NODE_PACKAGE_MANAGER")}

    {RageConfiguration(&session.app.fs)}
    {WorkspaceRage(session.app.workspace.deref())}
    ));

    if daemon_logs {
        match session.app.workspace.server_info() {
            Some(_) => {
                session.app.console.log(markup!({
                    ConnectedClientServerLog(session.app.workspace.deref())
                }));
            }
            None => {
                session
                    .app
                    .console
                    .log(markup!("Discovering running Biome servers..."));
                session.app.console.log(markup!({ RunningRomeServer }));
            }
        }
    }
    Ok(())
}

struct WorkspaceRage<'a>(&'a dyn Workspace);

impl Display for WorkspaceRage<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let workspace = self.0;

        let rage_result = workspace.rage(RageParams {});

        match rage_result {
            Ok(result) => {
                for entry in result.entries {
                    match entry {
                        RageEntry::Section(title) => {
                            Section(&title).fmt(fmt)?;
                        }
                        RageEntry::Pair { name, value } => {
                            KeyValuePair(&name, markup!({ value })).fmt(fmt)?;
                        }
                        RageEntry::Markup(markup) => markup.fmt(fmt)?,
                    }
                }

                Ok(())
            }
            Err(err) => {
                writeln!(fmt)?;
                (markup! {<Error>"\u{2716} Workspace rage failed:"</Error>}).fmt(fmt)?;

                writeln!(fmt, " {err}")
            }
        }
    }
}

/// Prints information about other running biome server instances.
struct RunningRomeServer;

impl Display for RunningRomeServer {
    fn fmt(&self, f: &mut Formatter) -> io::Result<()> {
        let versions = match enumerate_pipes() {
            Ok(iter) => iter,
            Err(err) => {
                (markup! {<Error>"\u{2716} Enumerating Biome instances failed:"</Error>}).fmt(f)?;
                return writeln!(f, " {err}");
            }
        };

        for version in versions {
            if version == biome_service::VERSION {
                let runtime = Runtime::new()?;
                match service::open_transport(runtime) {
                    Ok(None) => {
                        markup!(
                            {Section("Server")}
                            {KeyValuePair("Status", markup!(<Dim>"stopped"</Dim>))}
                        )
                        .fmt(f)?;
                        continue;
                    }
                    Ok(Some(transport)) => {
                        markup!("\n"<Emphasis>"Running Biome Server:"</Emphasis>" "{HorizontalLine::new(78)}"

"<Info>"\u{2139} The client isn't connected to any server but rage discovered this running Biome server."</Info>"
")
                .fmt(f)?;

                        match client(transport) {
                            Ok(client) => WorkspaceRage(client.deref()).fmt(f)?,
                            Err(err) => {
                                markup!(<Error>"\u{2716} Failed to connect: "</Error>).fmt(f)?;
                                writeln!(f, "{err}")?;
                            }
                        }
                    }
                    Err(err) => {
                        markup!("\n"<Error>"\u{2716} Failed to connect: "</Error>).fmt(f)?;
                        writeln!(f, "{err}")?;
                    }
                }

                BiomeServerLog.fmt(f)?;
            } else {
                markup!("\n"<Emphasis>"Incompatible Biome Server:"</Emphasis>" "{HorizontalLine::new(78)}"

"<Info>"\u{2139} Rage discovered this running server using an incompatible version of Biome."</Info>"
")
        .fmt(f)?;

                // Version 10.0.0 and below did not include a service version in the pipe name
                let version = if version.is_empty() {
                    "<=10.0.0"
                } else {
                    version.as_str()
                };

                markup!(
                    {Section("Server")}
                    {KeyValuePair("Version", markup!({version}))}
                )
                .fmt(f)?;
            }
        }

        Ok(())
    }
}

struct RageConfiguration<'a, 'app>(&'a DynRef<'app, dyn FileSystem>);

impl Display for RageConfiguration<'_, '_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        Section("Biome Configuration").fmt(fmt)?;

        match load_configuration(self.0, ConfigurationBasePath::default()) {
            Ok(loaded_configuration) => {
                if loaded_configuration.directory_path.is_none() {
                    KeyValuePair("Status", markup!(<Dim>"unset"</Dim>)).fmt(fmt)?;
                } else {
                    let LoadedConfiguration {
                        configuration,
                        diagnostics,
                        ..
                    } = loaded_configuration;
                    let status = if !diagnostics.is_empty() {
                        for diagnostic in diagnostics {
                            (markup! {
                                 {KeyValuePair("Error", markup!{
                                     {format!{"{}", PrintDescription(&diagnostic)}}
                                 })}
                            })
                            .fmt(fmt)?;
                        }
                        markup!(<Dim>"Loaded with errors"</Dim>)
                    } else {
                        markup!(<Dim>"Loaded successfully"</Dim>)
                    };

                    markup! (
                    {KeyValuePair("Status", status)}
                    {KeyValuePair("Formatter disabled", markup!({DebugDisplay(configuration.is_formatter_disabled())}))}
                    {KeyValuePair("Linter disabled", markup!({DebugDisplay(configuration.is_linter_disabled())}))}
                    {KeyValuePair("Organize imports disabled", markup!({DebugDisplay(configuration.is_organize_imports_disabled())}))}
                    {KeyValuePair("VCS disabled", markup!({DebugDisplay(configuration.is_vcs_disabled())}))}
                ).fmt(fmt)?
                }
            }
            Err(err) => markup! (
                {KeyValuePair("Status", markup!(<Error>"Failed to load"</Error>))}
                {KeyValuePair("Error", markup!({format!("{err}")}))}
            )
            .fmt(fmt)?,
        }

        Ok(())
    }
}

struct DebugDisplay<T>(T);

impl<T> Display for DebugDisplay<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> io::Result<()> {
        write!(f, "{:?}", self.0)
    }
}

struct EnvVarOs(&'static str);

impl fmt::Display for EnvVarOs {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let name = self.0;
        match env::var_os(name) {
            None => KeyValuePair(name, markup! { <Dim>"unset"</Dim> }).fmt(fmt),
            Some(value) => KeyValuePair(name, markup! {{DebugDisplay(value)}}).fmt(fmt),
        }
    }
}

struct Section<'a>(&'a str);

impl Display for Section<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        writeln!(fmt, "\n{}:", self.0)
    }
}

struct KeyValuePair<'a>(&'a str, Markup<'a>);

impl Display for KeyValuePair<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let KeyValuePair(key, value) = self;
        write!(fmt, "  {key}:")?;

        let padding_width = 30usize.saturating_sub(key.len() + 1);

        for _ in 0..padding_width {
            fmt.write_str(" ")?;
        }

        value.fmt(fmt)?;

        fmt.write_str("\n")
    }
}

struct BiomeServerLog;

impl Display for BiomeServerLog {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        if let Ok(Some(log)) = read_most_recent_log_file() {
            markup!("\n"<Emphasis><Underline>"Biome Server Log:"</Underline></Emphasis>"

"<Warn>"\u{26a0} Please review the content of the log file before sharing it publicly as it may contain sensitive information:
  * Path names that may reveal your name, a project name, or the name of your employer.
  * Source code
"</Warn>)
            .fmt(fmt)?;

            write!(fmt, "\n{log}")?;
        }

        Ok(())
    }
}

/// Prints the server logs but only if the client is connected to a biome server.
struct ConnectedClientServerLog<'a>(&'a dyn Workspace);

impl Display for ConnectedClientServerLog<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        if self.0.server_info().is_some() {
            BiomeServerLog.fmt(fmt)
        } else {
            Ok(())
        }
    }
}
