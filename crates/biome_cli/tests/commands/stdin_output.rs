use crate::run_cli;
use biome_console::{Console, LogLevel, Markup};
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use std::io::Write;
use std::process::{Command, Stdio};

const SOURCE: &str = "const warning = `\u{26a0}`;\nconst success = `\u{2714}`;\n";

#[derive(Default)]
struct SanitizingConsole {
    stdout: String,
    stderr: String,
    stdin: Option<String>,
}

impl SanitizingConsole {
    fn with_stdin(content: &str) -> Self {
        Self {
            stdin: Some(content.to_string()),
            ..Self::default()
        }
    }

    fn push(&mut self, level: LogLevel, content: &str) {
        match level {
            LogLevel::Error => self.stderr.push_str(content),
            LogLevel::Log => self.stdout.push_str(content),
        }
    }

    fn render(args: Markup) -> String {
        args.to_owned()
            .0
            .into_iter()
            .map(|node| node.content)
            .collect::<String>()
            .replace('\u{26a0}', "!")
            .replace('\u{2714}', "\u{221a}")
    }
}

impl Console for SanitizingConsole {
    fn println(&mut self, level: LogLevel, args: Markup) {
        let mut content = Self::render(args);
        content.push('\n');
        self.push(level, &content);
    }

    fn print(&mut self, level: LogLevel, args: Markup) {
        self.push(level, &Self::render(args));
    }

    fn print_raw(&mut self, level: LogLevel, content: &str) {
        self.push(level, content);
    }

    fn read(&mut self) -> Option<String> {
        self.stdin.take()
    }
}

fn assert_stdin_output(args: &[&str]) {
    let mut console = SanitizingConsole::with_stdin(SOURCE);
    let (_, result) = run_cli(MemoryFileSystem::default(), &mut console, Args::from(args));

    assert!(result.is_ok(), "CLI returned {result:?}");
    assert_eq!(console.stdout, SOURCE);
}

#[test]
fn format_stdin_preserves_unicode_source() {
    assert_stdin_output(&["format", "--stdin-file-path=probe.ts"]);
}

#[test]
fn check_write_stdin_preserves_unicode_source() {
    assert_stdin_output(&["check", "--write", "--stdin-file-path=probe.ts"]);
}

#[test]
fn extension_error_preserves_unicode_source() {
    assert_stdin_output(&["format", "--stdin-file-path=probe"]);
}

#[test]
fn env_console_preserves_unicode_source() {
    for colors in ["--colors=off", "--colors=force"] {
        let mut child = Command::new(env!("CARGO_BIN_EXE_biome"))
            .args(["format", colors, "--stdin-file-path=probe.ts"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to run biome");

        child
            .stdin
            .take()
            .expect("missing stdin")
            .write_all(SOURCE.as_bytes())
            .expect("failed to write stdin");

        let output = child.wait_with_output().expect("failed to read output");
        assert!(
            output.status.success(),
            "biome failed with {colors}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, SOURCE.as_bytes(), "colors: {colors}");
    }
}
