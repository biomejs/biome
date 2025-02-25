use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    io,
    ops::{BitOr, BitOrAssign},
    str::FromStr,
};

use enumflags2::{bitflags, make_bitflags, BitFlags};
use serde::{Deserialize, Serialize};

use biome_console::{fmt, markup};

use crate::{Category, Location, Visit};

/// The `Diagnostic` trait defines the metadata that can be exposed by error
/// types in order to print details diagnostics in the console of the editor
///
/// ## Implementation
///
/// Most types should not have to implement this trait manually, and should
/// instead rely on the `Diagnostic` derive macro also provided by this crate:
///
/// ```
/// # use biome_diagnostics::Diagnostic;
/// #[derive(Debug, Diagnostic)]
/// #[diagnostic(category = "lint/style/noShoutyConstants", tags(FIXABLE))]
/// struct ExampleDiagnostic {
///     #[message]
///     #[description]
///     message: String,
/// }
/// ```
pub trait Diagnostic: Debug {
    /// The category of a diagnostic uniquely identifying this
    /// diagnostic type, such as `lint/correctness/noArguments`, `args/invalid`
    /// or `format/disabled`.
    fn category(&self) -> Option<&'static Category> {
        None
    }

    /// The severity defines whether this diagnostic reports an error, a
    /// warning, an information or a hint to the user.
    fn severity(&self) -> Severity {
        Severity::Error
    }

    /// The description is a text-only explanation of the issue this diagnostic
    /// is reporting, intended for display contexts that do not support rich
    /// markup such as in-editor popovers
    ///
    /// The description should generally be as exhaustive as possible, since
    /// the clients that do not support rendering markup will not render the
    /// advices for the diagnostic either.
    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = fmt;
        Ok(())
    }

    /// An explanation of the issue this diagnostic is reporting
    ///
    /// In general it's better to keep this message as short as possible, and
    /// instead rely on advices to better convey contextual explanations to the
    /// user.
    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let _ = fmt;
        Ok(())
    }

    /// Advices are the main building blocks used compose rich errors. They are
    /// implemented using a visitor pattern, where consumers of a diagnostic
    /// can visit the object and collect the advices that make it up for the
    /// purpose of display or introspection.
    fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let _ = visitor;
        Ok(())
    }

    /// Diagnostics can defines additional advices to be printed if the user
    /// requires more detail about the diagnostic.
    fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let _ = visitor;
        Ok(())
    }

    /// A diagnostic can be tied to a specific "location": this can be a file,
    /// memory buffer, command line argument, etc. It may also be tied to a
    /// specific text range within the content of that location. Finally, it
    /// may also provide the source string for that location (this is required
    /// in order to display a code frame advice for the diagnostic).
    fn location(&self) -> Location<'_> {
        Location::builder().build()
    }

    /// Tags convey additional boolean metadata about the nature of a diagnostic:
    /// - If the diagnostic can be automatically fixed
    /// - If the diagnostic resulted from and internal error
    /// - If the diagnostic is being emitted as part of a crash / fatal error
    /// - If the diagnostic is a warning about a piece of unused or unnecessary code
    /// - If the diagnostic is a warning about a piece of deprecated or obsolete code.
    /// - If the diagnostic is meant to provide more information
    fn tags(&self) -> DiagnosticTags {
        DiagnosticTags::empty()
    }

    /// Similarly to the `source` method of the [std::error::Error] trait, this
    /// returns another diagnostic that's the logical "cause" for this issue.
    /// For instance, a "request failed" diagnostic may have been cause by a
    /// "deserialization error". This allows low-level error to be wrapped in
    /// higher level concepts, while retaining enough information to display
    /// and fix the underlying issue.
    fn source(&self) -> Option<&dyn Diagnostic> {
        None
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
/// The severity to associate to a diagnostic.
pub enum Severity {
    /// Reports a hint.
    Hint,
    /// Reports an information.
    #[default]
    Information,
    /// Reports a warning.
    Warning,
    /// Reports an error.
    Error,
    /// Reports a crash.
    Fatal,
}

impl FromStr for Severity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hint" => Ok(Self::Information),
            "info" => Ok(Self::Information),
            "warn" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            v => Err(format!(
                "Found unexpected value ({v}), valid values are: info, warn, error."
            )),
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hint => write!(f, "hint"),
            Self::Information => write!(f, "info"),
            Self::Warning => write!(f, "warn"),
            Self::Error => write!(f, "error"),
            Self::Fatal => write!(f, "fatal"),
        }
    }
}

impl biome_console::fmt::Display for Severity {
    fn fmt(&self, f: &mut biome_console::fmt::Formatter<'_>) -> io::Result<()> {
        match self {
            Self::Hint => f.write_markup(markup!(<Info>"hint"</Info>)),
            Self::Information => f.write_markup(markup!(<Info>"info"</Info>)),
            Self::Warning => f.write_markup(markup!(<Warn>"warn"</Warn>)),
            Self::Error => f.write_markup(markup!(<Error>"error"</Error>)),
            Self::Fatal => f.write_markup(markup!(<Error>"fatal"</Error>)),
        }
    }
}

/// Internal enum used to automatically generate bit offsets for [DiagnosticTags]
/// and help with the implementation of `serde` and `schemars` for tags.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[bitflags]
#[repr(u8)]
pub(super) enum DiagnosticTag {
    Fixable = 1 << 0,
    Internal = 1 << 1,
    UnnecessaryCode = 1 << 2,
    DeprecatedCode = 1 << 3,
    Verbose = 1 << 4,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DiagnosticTags(BitFlags<DiagnosticTag>);
impl DiagnosticTags {
    /// This diagnostic has a fix suggestion.
    pub const FIXABLE: Self = Self(make_bitflags!(DiagnosticTag::{Fixable}));
    /// This diagnostic results from an internal error.
    pub const INTERNAL: Self = Self(make_bitflags!(DiagnosticTag::{Internal}));
    /// This diagnostic tags unused or unnecessary code, this may change
    /// how the diagnostic is render in editors.
    pub const UNNECESSARY_CODE: Self = Self(make_bitflags!(DiagnosticTag::{UnnecessaryCode}));
    /// This diagnostic tags deprecated or obsolete code, this may change
    /// how the diagnostic is render in editors.
    pub const DEPRECATED_CODE: Self = Self(make_bitflags!(DiagnosticTag::{DeprecatedCode}));
    /// This diagnostic is verbose and should be printed only if the `--verbose` option is provided
    pub const VERBOSE: Self = Self(make_bitflags!(DiagnosticTag::{Verbose}));
    pub const fn all() -> Self {
        Self(BitFlags::ALL)
    }
    pub const fn empty() -> Self {
        Self(BitFlags::EMPTY)
    }
    pub fn insert(&mut self, other: DiagnosticTags) {
        self.0 |= other.0;
    }
    pub fn contains(self, other: impl Into<DiagnosticTags>) -> bool {
        self.0.contains(other.into().0)
    }
    pub const fn union(self, other: Self) -> Self {
        Self(self.0.union_c(other.0))
    }
    pub fn is_empty(self) -> bool {
        self.0.is_empty()
    }
    pub fn is_verbose(&self) -> bool {
        self.contains(DiagnosticTag::Verbose)
    }
}

impl BitOr for DiagnosticTags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        DiagnosticTags(self.0 | rhs.0)
    }
}

impl BitOrAssign for DiagnosticTags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

// Implement the `Diagnostic` on the `Infallible` error type from the standard
// library as a utility for implementing signatures that require a diagnostic
// type when the operation can never fail
impl Diagnostic for Infallible {}

pub(crate) mod internal {
    //! The `AsDiagnostic` trait needs to be declared as public as its referred
    //! to in the `where` clause of other public items, but as it's not part of
    //! the public API it's declared in a private module so it's not accessible
    //! outside of the crate

    use std::fmt::Debug;

    use crate::Diagnostic;

    /// Since [Error](crate::Error) must implement `From<T: Diagnostic>` to
    /// be used with the `?` operator, it cannot implement the [Diagnostic]
    /// trait (as that would conflict with the implementation of `From<T> for T`
    /// in the standard library). The [AsDiagnostic] exists as an internal
    /// implementation detail to bridge this gap and allow various types and
    /// functions in `biome_diagnostics` to be generic over all diagnostics +
    /// `Error`.
    pub trait AsDiagnostic: Debug {
        type Diagnostic: Diagnostic + ?Sized;
        fn as_diagnostic(&self) -> &Self::Diagnostic;
        fn as_dyn(&self) -> &dyn Diagnostic;
    }

    impl<D: Diagnostic> AsDiagnostic for D {
        type Diagnostic = D;

        fn as_diagnostic(&self) -> &Self::Diagnostic {
            self
        }

        fn as_dyn(&self) -> &dyn Diagnostic {
            self
        }
    }
}
