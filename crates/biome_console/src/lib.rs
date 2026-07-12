#![deny(clippy::use_self)]

use std::io;
use std::io::{IsTerminal, Read, Write};
use std::panic::RefUnwindSafe;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};
use write::{StringBuffer, Termcolor};

pub mod fmt;
mod markup;
mod utils;
mod write;

pub use self::markup::{Markup, MarkupBuf, MarkupElement, MarkupNode};
pub use biome_markup::markup;
pub use utils::*;

/// Determines the "output stream" a message should get printed to
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    /// Print the message to the `Error` stream of the console, for instance
    /// "stderr" for the [EnvConsole]
    Error,
    /// Print the message to the `Log` stream of the console, for instance
    /// "stdout" for the [EnvConsole]
    Log,
}

/// Generic abstraction over printing markup and diagnostics to an output,
/// which can be a terminal, a file, a memory buffer ...
pub trait Console: Send + Sync + RefUnwindSafe {
    /// Prints a message (formatted using [markup!]) to the console.
    ///
    /// It adds a new line at the end.
    fn println(&mut self, level: LogLevel, args: Markup);

    /// Prints a message (formatted using [markup!]) to the console.
    fn print(&mut self, level: LogLevel, args: Markup);

    /// It reads from a source, and if this source contains something, it's converted into a [String]
    fn read(&mut self) -> Option<String>;

    /// It returns a string of the messages have been written to the buffer. Applicable only to certain consoles
    fn dump(&mut self) -> Option<String> {
        None
    }

    /// It clears the internal buffer of the console, if applicable
    fn clear(&mut self) {}
}

/// Extension trait for [Console] providing convenience printing methods
pub trait ConsoleExt: Console {
    /// Prints a piece of markup with level [LogLevel::Error]
    fn error(&mut self, args: Markup);

    /// Prints a piece of markup with level [LogLevel::Log]
    ///
    /// Logs a message, adds a new line at the end.
    fn log(&mut self, args: Markup);

    /// Prints a piece of markup with level [LogLevel::Log]
    ///
    /// It doesn't add any line
    fn append(&mut self, args: Markup);
}

impl<T: Console + ?Sized> ConsoleExt for T {
    fn error(&mut self, args: Markup) {
        self.println(LogLevel::Error, args);
    }

    fn log(&mut self, args: Markup) {
        self.println(LogLevel::Log, args);
    }

    fn append(&mut self, args: Markup) {
        self.print(LogLevel::Log, args);
    }
}

/// Implementation of [Console] printing messages to the standard output and standard error
pub struct EnvConsole {
    /// Channel to print messages
    out: StandardStream,
    /// Channel to print errors
    err: StandardStream,
    /// Channel to read arbitrary input
    r#in: io::Stdin,
}

#[derive(Debug, Clone)]
pub enum ColorMode {
    /// Always print color using either ANSI or the Windows Console API
    Enabled,
    /// Never print colors
    Disabled,
    /// Print colors if stdout / stderr are determined to be TTY / Console
    /// streams, and the `TERM=dumb` and `NO_COLOR` environment variables are
    /// not set
    Auto,
}

impl EnvConsole {
    fn compute_color(colors: ColorMode) -> (ColorChoice, ColorChoice) {
        match colors {
            ColorMode::Enabled => (ColorChoice::Always, ColorChoice::Always),
            ColorMode::Disabled => (ColorChoice::Never, ColorChoice::Never),
            ColorMode::Auto => {
                let stdout = if io::stdout().is_terminal() {
                    ColorChoice::Auto
                } else {
                    ColorChoice::Never
                };

                let stderr = if io::stderr().is_terminal() {
                    ColorChoice::Auto
                } else {
                    ColorChoice::Never
                };

                (stdout, stderr)
            }
        }
    }

    pub fn new(colors: ColorMode) -> Self {
        let (out_mode, err_mode) = Self::compute_color(colors);

        Self {
            out: StandardStream::stdout(out_mode),
            err: StandardStream::stderr(err_mode),
            r#in: io::stdin(),
        }
    }

    pub fn set_color(&mut self, colors: ColorMode) {
        let (out_mode, err_mode) = Self::compute_color(colors);
        self.out = StandardStream::stdout(out_mode);
        self.err = StandardStream::stderr(err_mode);
    }
}

impl Default for EnvConsole {
    fn default() -> Self {
        Self::new(ColorMode::Auto)
    }
}

impl Console for EnvConsole {
    fn println(&mut self, level: LogLevel, args: Markup) {
        let mut out = match level {
            LogLevel::Error => self.err.lock(),
            LogLevel::Log => self.out.lock(),
        };
        write_to_console(&mut out, args, true);
    }

    fn print(&mut self, level: LogLevel, args: Markup) {
        let mut out = match level {
            LogLevel::Error => self.err.lock(),
            LogLevel::Log => self.out.lock(),
        };
        write_to_console(&mut out, args, false);
    }

    fn read(&mut self) -> Option<String> {
        // Here we check if stdin is redirected. If not, we bail.
        //
        // Doing this check allows us to pipe stdin to biome, without expecting
        // user content when we call `read_to_string`
        if io::stdin().is_terminal() {
            return None;
        }
        let mut handle = self.r#in.lock();
        let mut buffer = String::new();
        let result = handle.read_to_string(&mut buffer);
        // Skipping the error for now
        if result.is_ok() { Some(buffer) } else { None }
    }
}

/// Render `args` as markup into `out`, optionally followed by a newline.
///
/// A broken-pipe error from the underlying stream is treated as a clean
/// termination: when the consumer has closed the pipe (e.g. `biome ... | head`
/// or a CI step that times out early) writing will fail with
/// `io::ErrorKind::BrokenPipe`, and panicking would mask the legitimate output
/// already produced before the pipe closed. Any other I/O error is unexpected
/// and panics, matching the previous behaviour of `.unwrap()` on the write
/// results.
fn write_to_console<W: WriteColor>(out: &mut W, args: Markup, with_newline: bool) {
    check_write(
        fmt::Formatter::new(&mut Termcolor(out)).write_markup(args),
        "failed to write markup to console",
    );

    if with_newline {
        check_write(writeln!(out), "failed to write to console");
    }
}

/// Inspect a write result: a `BrokenPipe` error is silently swallowed (the
/// consumer has closed the pipe, so the partial output we managed to flush is
/// the final answer); any other I/O error is unexpected and panics with
/// `"{msg}: {err}"` to surface the failure loudly.
fn check_write(result: io::Result<()>, msg: &str) {
    if let Err(e) = result {
        if e.kind() != io::ErrorKind::BrokenPipe {
            panic!("{msg}: {e}");
        }
    }
}

/// Implementation of [Console] storing all printed messages to a memory buffer
#[derive(Default, Debug)]
pub struct BufferConsole {
    pub out_buffer: Vec<Message>,
    pub in_buffer: Vec<String>,
    pub print_json: bool,
}

impl BufferConsole {
    pub fn with_json(mut self) -> Self {
        self.print_json = true;
        self
    }
}

/// Individual message entry printed to a [BufferConsole]
#[derive(Debug)]
pub struct Message {
    pub level: LogLevel,
    pub content: MarkupBuf,
}

impl Console for BufferConsole {
    fn println(&mut self, level: LogLevel, args: Markup) {
        self.out_buffer.push(Message {
            level,
            content: args.to_owned(),
        });
    }

    fn print(&mut self, level: LogLevel, args: Markup) {
        self.out_buffer.push(Message {
            level,
            content: args.to_owned(),
        });
    }
    fn read(&mut self) -> Option<String> {
        if self.in_buffer.is_empty() {
            None
        } else {
            // for the time being we simple return the first message, as we don't
            // particular use case for multiple prompts
            Some(self.in_buffer[0].clone())
        }
    }
}

#[derive(Debug, Default)]
pub struct FileBufferConsole {
    out: Vec<Message>,
}

impl FileBufferConsole {}

impl Console for FileBufferConsole {
    fn println(&mut self, level: LogLevel, args: Markup) {
        self.out.push(Message {
            level,
            content: args.to_owned(),
        });
    }

    fn print(&mut self, level: LogLevel, args: Markup) {
        self.out.push(Message {
            level,
            content: args.to_owned(),
        });
    }

    fn read(&mut self) -> Option<String> {
        None
    }

    fn dump(&mut self) -> Option<String> {
        let mut buffer = Vec::new();
        let mut write = StringBuffer::new(&mut buffer);
        let mut fmt = fmt::Formatter::new(&mut write);

        for message in self.out.iter() {
            for element in message.content.0.iter() {
                fmt.write_str(element.content.as_str()).unwrap();
            }
        }

        String::from_utf8(buffer).ok()
    }

    fn clear(&mut self) {
        self.out.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock writer that fails every write/flush with `BrokenPipe`. Models the
    /// scenario where stdout/stderr has been redirected to a pipe whose reader
    /// closed early (e.g. `biome ... | head`).
    struct BrokenPipeWriter;

    impl io::Write for BrokenPipeWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }
    }

    impl WriteColor for BrokenPipeWriter {
        fn supports_color(&self) -> bool {
            false
        }

        fn set_color(&mut self, _spec: &ColorSpec) -> io::Result<()> {
            Ok(())
        }

        fn reset(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// `Vec<u8>` does not implement `WriteColor` on its own; wrap it so we
    /// can verify the happy path goes through formatter + writeln machinery.
    struct BufWriteColor {
        buf: Vec<u8>,
    }

    impl io::Write for BufWriteColor {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buf.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl WriteColor for BufWriteColor {
        fn supports_color(&self) -> bool {
            false
        }

        fn set_color(&mut self, _spec: &ColorSpec) -> io::Result<()> {
            Ok(())
        }

        fn reset(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn write_to_console_does_not_panic_on_broken_pipe() {
        let mut out = BrokenPipeWriter;
        // Both calls exercise the markup-write path (the first write into
        // the broken pipe fails with BrokenPipe). Neither must panic.
        write_to_console(&mut out, markup!({ "hello" }), true);
        write_to_console(&mut out, markup!({ "world" }), false);
    }

    #[test]
    fn write_to_console_writes_happy_path() {
        let mut out = BufWriteColor { buf: Vec::new() };
        write_to_console(&mut out, markup!({ "hi" }), true);
        write_to_console(&mut out, markup!({ "!" }), false);
        let rendered = String::from_utf8(out.buf).unwrap();
        assert!(rendered.contains("hi"));
        assert!(rendered.contains("!"));
        // The first call appended a newline; the second did not, so the
        // output ends with the bang rather than a trailing blank line.
        assert!(rendered.ends_with("!"));
    }
}
