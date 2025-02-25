use crate::commands::daemon::read_most_recent_log_file;
use crate::service::enumerate_pipes;
use crate::{service, CliDiagnostic, CliSession, VERSION};
use biome_configuration::{ConfigurationPathHint, Rules};
use biome_console::fmt::{Display, Formatter};
use biome_console::{
    fmt, markup, ConsoleExt, DebugDisplay, DisplayOption, HorizontalLine, KeyValuePair, Padding,
    SOFT_LINE,
};
use biome_diagnostics::termcolor::{ColorChoice, WriteColor};
use biome_diagnostics::{termcolor, PrintDescription};
use biome_flags::biome_env;
use biome_fs::{FileSystem, OsFileSystem};
use biome_service::configuration::{load_configuration, LoadedConfiguration};
use biome_service::settings::Settings;
use biome_service::workspace::{client, RageEntry, RageParams};
use biome_service::Workspace;
use camino::Utf8PathBuf;
use std::{env, io, ops::Deref};
use terminal_size::terminal_size;
use tokio::runtime::Runtime;

/// Handler for the `rage` command
pub(crate) fn rage(
    session: CliSession,
    daemon_logs: bool,
    formatter: bool,
    linter: bool,
) -> Result<(), CliDiagnostic> {
    let terminal_supports_colors = termcolor::BufferWriter::stdout(ColorChoice::Auto)
        .buffer()
        .supports_color();

    let biome_env = biome_env();

    session.app.console.log(markup!("CLI:\n"
    {KeyValuePair("Version", markup!({VERSION}))}
    {KeyValuePair("Color support", markup!({DebugDisplay(terminal_supports_colors)}))}

    {Section("Platform")}
    {KeyValuePair("CPU Architecture", markup!({std::env::consts::ARCH}))}
    {KeyValuePair("OS", markup!({std::env::consts::OS}))}
    {Section("Environment")}
    {biome_env}
    {EnvVarOs("NO_COLOR")}
    {EnvVarOs("TERM")}
    {EnvVarOs("JS_RUNTIME_VERSION")}
    {EnvVarOs("JS_RUNTIME_NAME")}
    {EnvVarOs("NODE_PACKAGE_MANAGER")}

    {RageConfiguration { fs: session.app.workspace.fs(), formatter, linter }}
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
                session.app.console.log(markup!({ RunningBiomeServer }));
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
struct RunningBiomeServer;

impl Display for RunningBiomeServer {
    fn fmt(&self, f: &mut Formatter) -> io::Result<()> {
        let versions = match enumerate_pipes() {
            Ok(iter) => iter,
            Err(err) => {
                (markup! {<Error>"\u{2716} Enumerating Biome instances failed:"</Error>}).fmt(f)?;
                return writeln!(f, " {err}");
            }
        };

        for (version, path) in versions {
            if version == biome_configuration::VERSION {
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
                        let header = "Running Biome Server: ";
                        let width = {
                            if cfg!(debug_assertions) {
                                78
                            } else {
                                terminal_size().map_or(78, |(width, _)| width.0 as usize)
                            }
                        };
                        let width = width.saturating_sub(header.len());

                        markup!("\n"<Emphasis>{header}</Emphasis>{HorizontalLine::new(width)}"

"<Info>"\u{2139} The client isn't connected to any server but rage discovered this running Biome server."</Info>"
")
                .fmt(f)?;

                        match client(transport, Box::new(OsFileSystem::default())) {
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
                let header = "Incompatible Biome Server: ";
                let width = {
                    if cfg!(debug_assertions) {
                        78
                    } else {
                        terminal_size().map_or(78, |(width, _)| width.0 as usize)
                    }
                };
                let width = width.saturating_sub(header.len());
                markup!("\n"<Emphasis>{header}</Emphasis>{HorizontalLine::new(width)}"

"<Info>"\u{2139} Rage discovered this running server using an incompatible version of Biome."</Info>"
")
        .fmt(f)?;

                markup!(
                    {Section("Server")}
                    {KeyValuePair("Version", markup!({version.as_str()}))}
                    {KeyValuePair("Path", markup!({path.as_str()}))}
                )
                .fmt(f)?;
            }
        }

        Ok(())
    }
}

struct RageConfiguration<'a> {
    fs: &'a dyn FileSystem,
    formatter: bool,
    linter: bool,
}

impl Display for RageConfiguration<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        Section("Biome Configuration").fmt(fmt)?;

        match load_configuration(self.fs, ConfigurationPathHint::default()) {
            Ok(loaded_configuration) => {
                if loaded_configuration.directory_path.is_none() {
                    markup! {
                        {KeyValuePair("Status", markup!(<Dim>"Not set"</Dim>))}
                        {ConfigPath("unset")}
                    }
                    .fmt(fmt)?;
                } else {
                    let LoadedConfiguration {
                        configuration,
                        diagnostics,
                        directory_path,
                        file_path,
                        ..
                    } = loaded_configuration;
                    let vcs_enabled = configuration.is_vcs_enabled();
                    let mut settings = Settings::default();
                    settings
                        .merge_with_configuration(configuration.clone(), None, None, &[])
                        .unwrap();

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

                    let config_path = file_path.as_ref().map_or_else(
                        || directory_path.as_ref().unwrap().as_str(),
                        |path| path.as_str(),
                    );

                    markup! (
                        {KeyValuePair("Status", status)}
                        {ConfigPath(config_path)}
                        {KeyValuePair("Formatter enabled", markup!({DebugDisplay(settings.is_formatter_enabled())}))}
                        {KeyValuePair("Linter enabled", markup!({DebugDisplay(settings.is_linter_enabled())}))}
                        {KeyValuePair("Assist enabled", markup!({DebugDisplay(settings.is_assist_enabled())}))}
                        {KeyValuePair("VCS enabled", markup!({DebugDisplay(vcs_enabled)}))}
                    ).fmt(fmt)?;

                    // Print formatter configuration if --formatter option is true
                    if self.formatter {
                        let formatter_configuration = configuration.get_formatter_configuration();
                        let includes = formatter_configuration.includes.map(|list| {
                            list.iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        });
                        markup! (
                            {Section("Formatter")}
                            {KeyValuePair("Format with errors", markup!({DisplayOption(configuration.get_formatter_configuration().format_with_errors)}))}
                            {KeyValuePair("Indent style", markup!({DisplayOption(formatter_configuration.indent_style)}))}
                            {KeyValuePair("Indent width", markup!({DisplayOption(formatter_configuration.indent_width)}))}
                            {KeyValuePair("Line ending", markup!({DisplayOption(formatter_configuration.line_ending)}))}
                            {KeyValuePair("Line width", markup!({DisplayOption(formatter_configuration.line_width)}))}
                            {KeyValuePair("Attribute position", markup!({DisplayOption(formatter_configuration.attribute_position)}))}
                            {KeyValuePair("Bracket spacing", markup!({DisplayOption(formatter_configuration.bracket_spacing)}))}
                            {KeyValuePair("Includes", markup!({DisplayOption(includes)}))}
                        ).fmt(fmt)?;

                        let javascript_formatter_configuration =
                            configuration.get_javascript_formatter_configuration();
                        markup! (
                            {Section("JavaScript Formatter")}
                            {KeyValuePair("Enabled", markup!({DisplayOption(javascript_formatter_configuration.enabled)}))}
                            {KeyValuePair("JSX quote style", markup!({DisplayOption(javascript_formatter_configuration.jsx_quote_style)}))}
                            {KeyValuePair("Quote properties", markup!({DisplayOption(javascript_formatter_configuration.quote_properties)}))}
                            {KeyValuePair("Trailing commas", markup!({DisplayOption(javascript_formatter_configuration.trailing_commas)}))}
                            {KeyValuePair("Semicolons", markup!({DisplayOption(javascript_formatter_configuration.semicolons)}))}
                            {KeyValuePair("Arrow parentheses", markup!({DisplayOption(javascript_formatter_configuration.arrow_parentheses)}))}
                            {KeyValuePair("Bracket spacing", markup!({DisplayOption(javascript_formatter_configuration.bracket_spacing)}))}
                            {KeyValuePair("Bracket same line", markup!({DisplayOption(javascript_formatter_configuration.bracket_same_line)}))}
                            {KeyValuePair("Quote style", markup!({DisplayOption(javascript_formatter_configuration.quote_style)}))}
                            {KeyValuePair("Indent style", markup!({DisplayOption(javascript_formatter_configuration.indent_style)}))}
                            {KeyValuePair("Indent width", markup!({DisplayOption(javascript_formatter_configuration.indent_width)}))}
                            {KeyValuePair("Line ending", markup!({DisplayOption(javascript_formatter_configuration.line_ending)}))}
                            {KeyValuePair("Line width", markup!({DisplayOption(javascript_formatter_configuration.line_width.map(|lw| lw.value()))}))}
                            {KeyValuePair("Attribute position", markup!({DisplayOption(javascript_formatter_configuration.attribute_position)}))}
                        )
                        .fmt(fmt)?;

                        let json_formatter_configuration =
                            configuration.get_json_formatter_configuration();
                        markup! (
                            {Section("JSON Formatter")}
                            {KeyValuePair("Enabled", markup!({DisplayOption(json_formatter_configuration.enabled)}))}
                            {KeyValuePair("Indent style", markup!({DisplayOption(json_formatter_configuration.indent_style)}))}
                            {KeyValuePair("Indent width", markup!({DisplayOption(json_formatter_configuration.indent_width)}))}
                            {KeyValuePair("Line ending", markup!({DisplayOption(json_formatter_configuration.line_ending)}))}
                            {KeyValuePair("Line width", markup!({DisplayOption(json_formatter_configuration.line_width.map(|lw| lw.value()))}))}
                            {KeyValuePair("Trailing Commas", markup!({DisplayOption(json_formatter_configuration.trailing_commas)}))}
                            {KeyValuePair("Expand lists", markup!({DisplayOption(json_formatter_configuration.expand)}))}
                        ).fmt(fmt)?;

                        let css_formatter_configuration =
                            configuration.get_css_formatter_configuration();
                        markup! (
                            {Section("CSS Formatter")}
                            {KeyValuePair("Enabled", markup!({DisplayOption(css_formatter_configuration.enabled)}))}
                            {KeyValuePair("Indent style", markup!({DisplayOption(css_formatter_configuration.indent_style)}))}
                            {KeyValuePair("Indent width", markup!({DisplayOption(css_formatter_configuration.indent_width)}))}
                            {KeyValuePair("Line ending", markup!({DisplayOption(css_formatter_configuration.line_ending)}))}
                            {KeyValuePair("Line width", markup!({DisplayOption(css_formatter_configuration.line_width)}))}
                            {KeyValuePair("Quote style", markup!({DisplayOption(css_formatter_configuration.quote_style)}))}
                        ).fmt(fmt)?;

                        let graphql_formatter_configuration =
                            configuration.get_graphql_formatter_configuration();
                        markup! (
                            {Section("GraphQL Formatter")}
                            {KeyValuePair("Enabled", markup!({DisplayOption(graphql_formatter_configuration.enabled)}))}
                            {KeyValuePair("Indent style", markup!({DisplayOption(graphql_formatter_configuration.indent_style)}))}
                            {KeyValuePair("Indent width", markup!({DisplayOption(graphql_formatter_configuration.indent_width)}))}
                            {KeyValuePair("Line ending", markup!({DisplayOption(graphql_formatter_configuration.line_ending)}))}
                            {KeyValuePair("Line width", markup!({DisplayOption(graphql_formatter_configuration.line_width)}))}
                            {KeyValuePair("Bracket spacing", markup!({DisplayOption(graphql_formatter_configuration.bracket_spacing)}))}
                            {KeyValuePair("Quote style", markup!({DisplayOption(graphql_formatter_configuration.quote_style)}))}
                        ).fmt(fmt)?;
                    }

                    // Print linter configuration if --linter option is true
                    if self.linter {
                        let linter_configuration = configuration.get_linter_rules();

                        let javascript_linter = configuration.get_javascript_linter_configuration();
                        let json_linter = configuration.get_json_linter_configuration();
                        let css_linter = configuration.get_css_linter_configuration();
                        let graphql_linter = configuration.get_graphql_linter_configuration();
                        markup! (
                            {Section("Linter")}
                            {KeyValuePair("JavaScript enabled", markup!({DisplayOption(javascript_linter.enabled)}))}
                            {KeyValuePair("JSON enabled", markup!({DisplayOption(json_linter.enabled)}))}
                            {KeyValuePair("CSS enabled", markup!({DisplayOption(css_linter.enabled)}))}
                            {KeyValuePair("GraphQL enabled", markup!({DisplayOption(graphql_linter.enabled)}))}
                            {KeyValuePair("Recommended", markup!({DisplayOption(linter_configuration.recommended)}))}
                            {RageConfigurationLintRules("Enabled rules", linter_configuration)}
                        ).fmt(fmt)?;
                    }
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

struct RageConfigurationLintRules<'a>(&'a str, Rules);

impl Display for RageConfigurationLintRules<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> io::Result<()> {
        let rules_str = self.0;
        let padding = Padding::new(2);
        let padding_rules = Padding::new(4);
        fmt.write_markup(markup! {{padding}{rules_str}":"})?;
        fmt.write_markup(markup! {{SOFT_LINE}})?;
        let rules = self.1.as_enabled_rules();
        let rules = rules.iter().collect::<std::collections::BTreeSet<_>>();
        for rule in rules {
            fmt.write_markup(markup! {{padding_rules}{rule}})?;
            fmt.write_markup(markup! {{SOFT_LINE}})?;
        }

        Ok(())
    }
}

struct EnvVarOs(&'static str);

impl fmt::Display for EnvVarOs {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let name = self.0;
        match env::var_os(name) {
            None => KeyValuePair(name, markup! { <Dim>"unset"</Dim> }).fmt(fmt),
            Some(value) => KeyValuePair(name, markup! {{DisplayOption(value.to_str())}}).fmt(fmt),
        }
    }
}

struct Section<'a>(&'a str);

impl Display for Section<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        writeln!(fmt, "\n{}:", self.0)
    }
}

struct BiomeServerLog;

impl Display for BiomeServerLog {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        if let Ok(Some(log)) = read_most_recent_log_file(
            biome_env().biome_log_path.value().map(Utf8PathBuf::from),
            biome_env()
                .biome_log_prefix
                .value()
                .unwrap_or("server.log".to_string()),
        ) {
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

/// Prints the config path, but only if it is set.
struct ConfigPath<'a>(&'a str);

impl Display for ConfigPath<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let path = self.0;
        if path.is_empty() {
            KeyValuePair("Path", markup! { <Dim>"unset"</Dim> }).fmt(fmt)
        } else {
            KeyValuePair("Path", markup!({ DebugDisplay(path) })).fmt(fmt)
        }
    }
}
