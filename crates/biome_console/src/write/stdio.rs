use std::io::{self, Write};

#[cfg(unix)]
use std::os::fd::{AsFd, AsRawFd, RawFd};

use termcolor::WriteColor;

use crate::{Markup, fmt};

use super::Termcolor;

pub(crate) fn write_to_color_writer<W>(
    writer: &mut W,
    args: Markup,
    newline: bool,
) -> io::Result<()>
where
    W: WriteColor,
{
    let mut termcolor = Termcolor(&mut *writer);
    fmt::Formatter::new(&mut termcolor).write_markup(args)?;

    if newline {
        writer.write_all(b"\n")?;
    }

    Ok(())
}

#[cfg(unix)]
fn is_retryable_write_error(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
        || matches!(
            err.raw_os_error(),
            Some(code) if code == libc::EAGAIN || code == libc::EWOULDBLOCK
        )
}

#[cfg(unix)]
fn wait_until_writable(raw_fd: RawFd) -> io::Result<()> {
    let mut descriptor = libc::pollfd {
        fd: raw_fd,
        events: libc::POLLOUT,
        revents: 0,
    };

    loop {
        // SAFETY: `descriptor` points to one valid `pollfd` for the duration of the call.
        let result = unsafe { libc::poll(&mut descriptor, 1, -1) };

        if result > 0 {
            if descriptor.revents & libc::POLLOUT != 0 {
                return Ok(());
            }

            return Err(io::Error::other(
                "failed waiting for console output to become writable",
            ));
        }

        let err = io::Error::last_os_error();
        if err.kind() == io::ErrorKind::Interrupted {
            continue;
        }

        return Err(err);
    }
}

#[cfg(unix)]
fn write_with_retry<W, Wait>(
    writer: &mut W,
    buf: &[u8],
    wait_until_writable: &mut Wait,
) -> io::Result<usize>
where
    W: Write,
    Wait: FnMut() -> io::Result<()>,
{
    loop {
        match writer.write(buf) {
            Ok(bytes_written) => return Ok(bytes_written),
            Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) if is_retryable_write_error(&err) => wait_until_writable()?,
            Err(err) => return Err(err),
        }
    }
}

#[cfg(unix)]
fn write_all_with_retry<W, Wait>(
    writer: &mut W,
    mut buf: &[u8],
    wait_until_writable: &mut Wait,
) -> io::Result<()>
where
    W: Write,
    Wait: FnMut() -> io::Result<()>,
{
    while !buf.is_empty() {
        let bytes_written = write_with_retry(writer, buf, wait_until_writable)?;

        if bytes_written == 0 {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "failed to write buffered console output",
            ));
        }

        buf = &buf[bytes_written..];
    }

    Ok(())
}

#[cfg(unix)]
fn flush_with_retry<W, Wait>(writer: &mut W, wait_until_writable: &mut Wait) -> io::Result<()>
where
    W: Write,
    Wait: FnMut() -> io::Result<()>,
{
    loop {
        match writer.flush() {
            Ok(()) => return Ok(()),
            Err(err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) if is_retryable_write_error(&err) => wait_until_writable()?,
            Err(err) => return Err(err),
        }
    }
}

#[cfg(unix)]
struct BlockingWriter<W> {
    inner: W,
}

#[cfg(unix)]
impl<W> BlockingWriter<W> {
    fn new(inner: W) -> Self {
        Self { inner }
    }
}

#[cfg(unix)]
impl<W> Write for BlockingWriter<W>
where
    W: Write + AsFd,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let raw_fd = self.inner.as_fd().as_raw_fd();
        let mut wait_until_writable = || wait_until_writable(raw_fd);
        write_with_retry(&mut self.inner, buf, &mut wait_until_writable)
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let raw_fd = self.inner.as_fd().as_raw_fd();
        let mut wait_until_writable = || wait_until_writable(raw_fd);
        write_all_with_retry(&mut self.inner, buf, &mut wait_until_writable)
    }

    fn flush(&mut self) -> io::Result<()> {
        let raw_fd = self.inner.as_fd().as_raw_fd();
        let mut wait_until_writable = || wait_until_writable(raw_fd);
        flush_with_retry(&mut self.inner, &mut wait_until_writable)
    }
}

#[cfg(unix)]
pub(crate) fn write_to_std_stream<W>(
    writer: W,
    supports_color: bool,
    args: Markup,
    newline: bool,
) -> io::Result<()>
where
    W: Write + AsFd,
{
    if supports_color {
        let mut writer = termcolor::Ansi::new(BlockingWriter::new(writer));
        write_to_color_writer(&mut writer, args, newline)
    } else {
        let mut writer = termcolor::NoColor::new(BlockingWriter::new(writer));
        write_to_color_writer(&mut writer, args, newline)
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::collections::VecDeque;
    use std::io;

    use super::{flush_with_retry, write_all_with_retry, write_with_retry};

    enum WriteStep {
        Write(usize),
        WouldBlock,
        Interrupted,
        Error(io::ErrorKind),
    }

    enum FlushStep {
        Ok,
        WouldBlock,
        Interrupted,
    }

    struct MockWriter {
        write_steps: VecDeque<WriteStep>,
        flush_steps: VecDeque<FlushStep>,
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            match self.write_steps.pop_front().expect("missing write step") {
                WriteStep::Write(limit) => {
                    let bytes_written = limit.min(buf.len());
                    self.written.extend_from_slice(&buf[..bytes_written]);
                    Ok(bytes_written)
                }
                WriteStep::WouldBlock => Err(io::Error::from(io::ErrorKind::WouldBlock)),
                WriteStep::Interrupted => Err(io::Error::from(io::ErrorKind::Interrupted)),
                WriteStep::Error(kind) => Err(io::Error::from(kind)),
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            match self.flush_steps.pop_front().unwrap_or(FlushStep::Ok) {
                FlushStep::Ok => Ok(()),
                FlushStep::WouldBlock => Err(io::Error::from(io::ErrorKind::WouldBlock)),
                FlushStep::Interrupted => Err(io::Error::from(io::ErrorKind::Interrupted)),
            }
        }
    }

    #[test]
    fn write_with_retry_returns_actual_byte_count() {
        let mut writer = MockWriter {
            write_steps: VecDeque::from([WriteStep::WouldBlock, WriteStep::Write(2)]),
            flush_steps: VecDeque::new(),
            written: Vec::new(),
        };
        let mut wait_calls = 0;

        let bytes_written = write_with_retry(&mut writer, b"abcd", &mut || {
            wait_calls += 1;
            Ok(())
        })
        .unwrap();

        assert_eq!(bytes_written, 2);
        assert_eq!(writer.written, b"ab");
        assert_eq!(wait_calls, 1);
    }

    #[test]
    fn write_all_retries_after_partial_write_and_would_block() {
        let mut writer = MockWriter {
            write_steps: VecDeque::from([
                WriteStep::Write(2),
                WriteStep::WouldBlock,
                WriteStep::Write(4),
            ]),
            flush_steps: VecDeque::new(),
            written: Vec::new(),
        };
        let mut wait_calls = 0;

        write_all_with_retry(&mut writer, b"abcdef", &mut || {
            wait_calls += 1;
            Ok(())
        })
        .unwrap();

        assert_eq!(writer.written, b"abcdef");
        assert_eq!(wait_calls, 1);
    }

    #[test]
    fn write_all_returns_write_zero_when_no_progress() {
        let mut writer = MockWriter {
            write_steps: VecDeque::from([WriteStep::Write(0)]),
            flush_steps: VecDeque::new(),
            written: Vec::new(),
        };

        let err = write_all_with_retry(&mut writer, b"abcdef", &mut || Ok(())).unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::WriteZero);
    }

    #[test]
    fn write_all_stops_on_non_retryable_errors() {
        let mut writer = MockWriter {
            write_steps: VecDeque::from([WriteStep::Error(io::ErrorKind::BrokenPipe)]),
            flush_steps: VecDeque::new(),
            written: Vec::new(),
        };
        let mut wait_calls = 0;

        let err = write_all_with_retry(&mut writer, b"abcdef", &mut || {
            wait_calls += 1;
            Ok(())
        })
        .unwrap_err();

        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
        assert_eq!(wait_calls, 0);
        assert!(writer.written.is_empty());
    }

    #[test]
    fn flush_retries_after_would_block() {
        let mut writer = MockWriter {
            write_steps: VecDeque::new(),
            flush_steps: VecDeque::from([FlushStep::WouldBlock, FlushStep::Ok]),
            written: Vec::new(),
        };
        let mut wait_calls = 0;

        flush_with_retry(&mut writer, &mut || {
            wait_calls += 1;
            Ok(())
        })
        .unwrap();

        assert_eq!(wait_calls, 1);
    }

    #[test]
    fn interrupted_writes_and_flushes_retry_without_waiting() {
        let mut writer = MockWriter {
            write_steps: VecDeque::from([WriteStep::Interrupted, WriteStep::Write(3)]),
            flush_steps: VecDeque::from([FlushStep::Interrupted, FlushStep::Ok]),
            written: Vec::new(),
        };
        let mut wait_calls = 0;

        write_all_with_retry(&mut writer, b"abc", &mut || {
            wait_calls += 1;
            Ok(())
        })
        .unwrap();
        flush_with_retry(&mut writer, &mut || {
            wait_calls += 1;
            Ok(())
        })
        .unwrap();

        assert_eq!(writer.written, b"abc");
        assert_eq!(wait_calls, 0);
    }
}
