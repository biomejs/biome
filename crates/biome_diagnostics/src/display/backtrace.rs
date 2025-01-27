use std::{borrow::Cow, path::PathBuf};
use std::{cell::Cell, fmt::Write as _, io, os::raw::c_void, path::Path, slice};

use biome_console::{fmt, markup};
use serde::{Deserialize, Serialize};

use super::IndentWriter;

/// The [Backtrace] type can be used to capture a native Rust stack trace, to
/// be displayed a diagnostic advice for native errors.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Backtrace {
    inner: BacktraceKind,
}

impl Default for Backtrace {
    // Do not inline this function to ensure it creates a stack frame, so that
    // internal functions above it in the backtrace can be hidden when the
    // backtrace is printed
    #[inline(never)]
    fn default() -> Self {
        Self::capture(Backtrace::default as usize)
    }
}

impl Backtrace {
    /// Take a snapshot of the current state of the stack and return it as a [Backtrace].
    pub fn capture(top_frame: usize) -> Self {
        Self {
            inner: BacktraceKind::Native(NativeBacktrace::new(top_frame)),
        }
    }

    /// Since the `capture` function only takes a lightweight snapshot of the
    /// stack, it's necessary to perform an additional resolution step to map
    /// the list of instruction pointers on the stack to actual symbol
    /// information (like function name and file location) before printing the
    /// backtrace.
    pub(super) fn resolve(&mut self) {
        if let BacktraceKind::Native(inner) = &mut self.inner {
            inner.resolve();
        }
    }

    fn frames(&self) -> BacktraceFrames<'_> {
        match &self.inner {
            BacktraceKind::Native(inner) => BacktraceFrames::Native(inner.frames()),
            BacktraceKind::Serialized(inner) => BacktraceFrames::Serialized(inner),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.frames().is_empty()
    }
}

impl serde::Serialize for Backtrace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let frames = match &self.inner {
            BacktraceKind::Native(backtrace) => {
                let mut backtrace = backtrace.clone();
                backtrace.resolve();

                let frames: Vec<_> = backtrace
                    .frames()
                    .iter()
                    .map(SerializedFrame::from)
                    .collect();

                Cow::Owned(frames)
            }
            BacktraceKind::Serialized(frames) => Cow::Borrowed(frames),
        };

        frames.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Backtrace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(Self {
            inner: BacktraceKind::Serialized(<Vec<SerializedFrame>>::deserialize(deserializer)?),
        })
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Backtrace {
    fn schema_name() -> String {
        String::from("Backtrace")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <Vec<SerializedFrame>>::json_schema(gen)
    }
}

/// Internal representation of a [Backtrace], can be either a native backtrace
/// instance or a vector of serialized frames.
#[derive(Clone, Debug)]
enum BacktraceKind {
    Native(NativeBacktrace),
    Serialized(Vec<SerializedFrame>),
}

#[cfg(test)]
impl PartialEq for BacktraceKind {
    fn eq(&self, _other: &Self) -> bool {
        if let (BacktraceKind::Serialized(this), BacktraceKind::Serialized(other)) = (self, _other)
        {
            return this == other;
        }

        false
    }
}

#[cfg(test)]
impl Eq for BacktraceKind {}

/// Wrapper type for a native backtrace instance.
#[derive(Clone, Debug)]
struct NativeBacktrace {
    backtrace: ::backtrace::Backtrace,
    /// Pointer to the top frame, this frame and every entry above it on the
    /// stack will not be displayed in the printed stack trace.
    top_frame: usize,
    /// Pointer to the bottom frame, this frame and every entry below it on the
    /// stack will not be displayed in the printed stack trace.
    bottom_frame: usize,
}

impl NativeBacktrace {
    fn new(top_frame: usize) -> Self {
        Self {
            backtrace: ::backtrace::Backtrace::new_unresolved(),
            top_frame,
            bottom_frame: bottom_frame(),
        }
    }

    fn resolve(&mut self) {
        self.backtrace.resolve();
    }

    /// Returns the list of frames for this backtrace, truncated to the
    /// `top_frame` and `bottom_frame`.
    fn frames(&self) -> &'_ [::backtrace::BacktraceFrame] {
        let mut frames = self.backtrace.frames();

        let top_frame = frames.iter().position(|frame| {
            frame.symbols().iter().any(|symbol| {
                symbol
                    .addr()
                    .is_some_and(|addr| addr as usize == self.top_frame)
            })
        });

        if let Some(top_frame) = top_frame {
            if let Some(bottom_frames) = frames.get(top_frame + 1..) {
                frames = bottom_frames;
            }
        }

        let bottom_frame = frames.iter().position(|frame| {
            frame.symbols().iter().any(|symbol| {
                symbol
                    .addr()
                    .is_some_and(|addr| addr as usize == self.bottom_frame)
            })
        });

        if let Some(bottom_frame) = bottom_frame {
            if let Some(top_frames) = frames.get(..bottom_frame + 1) {
                frames = top_frames;
            }
        }

        frames
    }
}

thread_local! {
    /// This cell holds the address of the function that conceptually sits at the
    /// "bottom" of the backtraces created on the current thread (all the frames
    /// below this will be hidden when the backtrace is printed)
    ///
    /// This value is thread-local since different threads will generally have
    /// different values for the bottom frame address: for the main thread this
    /// will be the address of the `main` function, while on worker threads
    /// this will be the start function for the thread (see the documentation
    /// of [set_bottom_frame] for examples of where to set the bottom frame).
    static BOTTOM_FRAME: Cell<Option<usize>> = const { Cell::new(None) };
}

/// Registers a function pointer as the "bottom frame" for this thread: all
/// instances of [Backtrace] created on this thread will omit this function and
/// all entries below it on the stack
///
/// ## Examples
///
/// On the main thread:
/// ```
/// # use biome_diagnostics::set_bottom_frame;
/// # #[expect(clippy::needless_doctest_main)]
/// pub fn main() {
///     set_bottom_frame(main as usize);
///
///     // ...
/// }
/// ```
///
/// On worker threads:
/// ```
/// # use biome_diagnostics::set_bottom_frame;
/// fn worker_thread() {
///     set_bottom_frame(worker_thread as usize);
///
///     // ...
/// }
///
/// std::thread::spawn(worker_thread);
/// ```
pub fn set_bottom_frame(ptr: usize) {
    BOTTOM_FRAME.with(|cell| {
        cell.set(Some(ptr));
    });
}

fn bottom_frame() -> usize {
    BOTTOM_FRAME.with(|cell| cell.get().unwrap_or(0))
}

pub(super) fn print_backtrace(
    fmt: &mut fmt::Formatter<'_>,
    backtrace: &Backtrace,
) -> io::Result<()> {
    for (frame_index, frame) in backtrace.frames().iter().enumerate() {
        if frame.ip().is_null() {
            continue;
        }

        fmt.write_fmt(format_args!("{frame_index:4}: "))?;

        let mut slot = None;
        let mut fmt = IndentWriter::wrap(fmt, &mut slot, false, "      ");

        for symbol in frame.symbols().iter() {
            if let Some(name) = symbol.name() {
                fmt.write_fmt(format_args!("{name:#}"))?;
            }

            fmt.write_str("\n")?;

            if let Some(filename) = symbol.filename() {
                let mut slot = None;
                let mut fmt = IndentWriter::wrap(&mut fmt, &mut slot, true, "    ");

                // Print a hyperlink if the file exists on disk
                let href = if filename.exists() {
                    Some(format!("file:///{}", filename.display()))
                } else {
                    None
                };

                // Build up the text of the link from the file path, the line number and column number
                let mut text = filename.display().to_string();

                if let Some(lineno) = symbol.lineno() {
                    // SAFETY: Writing a `u32` to a string should not fail
                    write!(text, ":{lineno}").unwrap();

                    if let Some(colno) = symbol.colno() {
                        // SAFETY: Writing a `u32` to a string should not fail
                        write!(text, ":{colno}").unwrap();
                    }
                }

                if let Some(href) = href {
                    fmt.write_markup(markup! {
                        "at "
                        <Hyperlink href={href}>{text}</Hyperlink>
                        "\n"
                    })?;
                } else {
                    fmt.write_markup(markup! {
                        "at "{text}"\n"
                    })?;
                }
            }
        }
    }

    Ok(())
}

/// Serializable representation of a backtrace frame.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "schema",
    derive(schemars::JsonSchema),
    schemars(rename = "BacktraceFrame")
)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct SerializedFrame {
    ip: u64,
    symbols: Vec<SerializedSymbol>,
}

impl From<&'_ backtrace::BacktraceFrame> for SerializedFrame {
    fn from(frame: &'_ backtrace::BacktraceFrame) -> Self {
        Self {
            ip: frame.ip() as u64,
            symbols: frame.symbols().iter().map(SerializedSymbol::from).collect(),
        }
    }
}

/// Serializable representation of a backtrace frame symbol.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(
    feature = "schema",
    derive(schemars::JsonSchema),
    schemars(rename = "BacktraceSymbol")
)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct SerializedSymbol {
    name: Option<String>,
    filename: Option<PathBuf>,
    lineno: Option<u32>,
    colno: Option<u32>,
}

impl From<&'_ backtrace::BacktraceSymbol> for SerializedSymbol {
    fn from(symbol: &'_ backtrace::BacktraceSymbol) -> Self {
        Self {
            name: symbol.name().map(|name| format!("{name:#}")),
            filename: symbol.filename().map(ToOwned::to_owned),
            lineno: symbol.lineno(),
            colno: symbol.colno(),
        }
    }
}

enum BacktraceFrames<'a> {
    Native(&'a [::backtrace::BacktraceFrame]),
    Serialized(&'a [SerializedFrame]),
}

impl BacktraceFrames<'_> {
    fn iter(&self) -> BacktraceFramesIter<'_> {
        match self {
            Self::Native(inner) => BacktraceFramesIter::Native(inner.iter()),
            Self::Serialized(inner) => BacktraceFramesIter::Serialized(inner.iter()),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Native(inner) => inner.is_empty(),
            Self::Serialized(inner) => inner.is_empty(),
        }
    }
}

enum BacktraceFramesIter<'a> {
    Native(slice::Iter<'a, ::backtrace::BacktraceFrame>),
    Serialized(slice::Iter<'a, SerializedFrame>),
}

impl<'a> Iterator for BacktraceFramesIter<'a> {
    type Item = BacktraceFrame<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Native(inner) => inner.next().map(BacktraceFrame::Native),
            Self::Serialized(inner) => inner.next().map(BacktraceFrame::Serialized),
        }
    }
}

enum BacktraceFrame<'a> {
    Native(&'a ::backtrace::BacktraceFrame),
    Serialized(&'a SerializedFrame),
}

impl BacktraceFrame<'_> {
    fn ip(&self) -> *mut c_void {
        match self {
            Self::Native(inner) => inner.ip(),
            Self::Serialized(inner) => inner.ip as *mut c_void,
        }
    }

    fn symbols(&self) -> BacktraceSymbols<'_> {
        match self {
            Self::Native(inner) => BacktraceSymbols::Native(inner.symbols()),
            Self::Serialized(inner) => BacktraceSymbols::Serialized(&inner.symbols),
        }
    }
}

enum BacktraceSymbols<'a> {
    Native(&'a [::backtrace::BacktraceSymbol]),
    Serialized(&'a [SerializedSymbol]),
}

impl BacktraceSymbols<'_> {
    fn iter(&self) -> BacktraceSymbolsIter<'_> {
        match self {
            Self::Native(inner) => BacktraceSymbolsIter::Native(inner.iter()),
            Self::Serialized(inner) => BacktraceSymbolsIter::Serialized(inner.iter()),
        }
    }
}

enum BacktraceSymbolsIter<'a> {
    Native(slice::Iter<'a, ::backtrace::BacktraceSymbol>),
    Serialized(slice::Iter<'a, SerializedSymbol>),
}

impl<'a> Iterator for BacktraceSymbolsIter<'a> {
    type Item = BacktraceSymbol<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Native(inner) => inner.next().map(BacktraceSymbol::Native),
            Self::Serialized(inner) => inner.next().map(BacktraceSymbol::Serialized),
        }
    }
}

enum BacktraceSymbol<'a> {
    Native(&'a ::backtrace::BacktraceSymbol),
    Serialized(&'a SerializedSymbol),
}

impl BacktraceSymbol<'_> {
    fn name(&self) -> Option<String> {
        match self {
            Self::Native(inner) => inner.name().map(|name| format!("{name:#}")),
            Self::Serialized(inner) => inner.name.clone(),
        }
    }

    fn filename(&self) -> Option<&Path> {
        match self {
            Self::Native(inner) => inner.filename(),
            Self::Serialized(inner) => inner.filename.as_deref(),
        }
    }

    fn lineno(&self) -> Option<u32> {
        match self {
            Self::Native(inner) => inner.lineno(),
            Self::Serialized(inner) => inner.lineno,
        }
    }

    fn colno(&self) -> Option<u32> {
        match self {
            Self::Native(inner) => inner.colno(),
            Self::Serialized(inner) => inner.colno,
        }
    }
}
