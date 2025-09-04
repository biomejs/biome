use biome_console::fmt;

use crate::context::internal::{SeverityDiagnostic, TagsDiagnostic};
use crate::{
    Category, DiagnosticTags, Error, Resource, Severity, SourceCode,
    diagnostic::internal::AsDiagnostic,
    location::{AsResource, AsSourceCode, AsSpan},
};

/// This trait is implemented for all types implementing [Diagnostic](super::Diagnostic)
/// and the [Error] struct, and exposes various combinator methods to enrich
/// existing diagnostics with additional information.
pub trait DiagnosticExt: internal::Sealed + Sized {
    /// Returns a new diagnostic with the provided `message` as a message and
    /// description, and `self` as a source diagnostic. This is useful to
    /// create chains of diagnostics, where high level errors wrap lower level
    /// causes.
    fn context<M>(self, message: M) -> Error
    where
        Self: 'static,
        M: fmt::Display + 'static,
        Error: From<internal::ContextDiagnostic<M, Self>>;

    /// Returns a new diagnostic using the provided `category` if `self`
    /// doesn't already have one.
    fn with_category(self, category: &'static Category) -> Error
    where
        Error: From<internal::CategoryDiagnostic<Self>>;

    /// Returns a new diagnostic using the provided `path` if `self`
    /// doesn't already have one.
    fn with_file_path(self, path: impl AsResource) -> Error
    where
        Error: From<internal::FilePathDiagnostic<Self>>;

    /// Returns a new diagnostic using the provided `span` instead of the one in `self`.
    fn with_file_span(self, span: impl AsSpan) -> Error
    where
        Error: From<internal::FileSpanDiagnostic<Self>>;

    /// Returns a new diagnostic using the provided `source_code` if `self`
    /// doesn't already have one.
    fn with_file_source_code(self, source_code: impl AsSourceCode) -> Error
    where
        Error: From<internal::FileSourceCodeDiagnostic<Self>>;

    /// Returns a new diagnostic with additional `tags`
    fn with_tags(self, tags: DiagnosticTags) -> Error
    where
        Error: From<internal::TagsDiagnostic<Self>>;

    /// Returns a new diagnostic with additional `severity`
    fn with_severity(self, severity: Severity) -> Error
    where
        Error: From<internal::SeverityDiagnostic<Self>>;
}

impl<E: AsDiagnostic> internal::Sealed for E {}

impl<E: AsDiagnostic> DiagnosticExt for E {
    fn context<M>(self, message: M) -> Error
    where
        E: 'static,
        M: fmt::Display + 'static,
        Error: From<internal::ContextDiagnostic<M, E>>,
    {
        Error::from(internal::ContextDiagnostic {
            message,
            source: self,
        })
    }

    fn with_category(self, category: &'static Category) -> Error
    where
        Error: From<internal::CategoryDiagnostic<Self>>,
    {
        Error::from(internal::CategoryDiagnostic {
            category,
            source: self,
        })
    }

    fn with_file_path(self, path: impl AsResource) -> Error
    where
        Error: From<internal::FilePathDiagnostic<E>>,
    {
        Error::from(internal::FilePathDiagnostic {
            path: path.as_resource().map(Resource::to_owned),
            source: self,
        })
    }

    fn with_file_span(self, span: impl AsSpan) -> Error
    where
        Error: From<internal::FileSpanDiagnostic<E>>,
    {
        Error::from(internal::FileSpanDiagnostic {
            span: span.as_span(),
            source: self,
        })
    }

    fn with_file_source_code(self, source_code: impl AsSourceCode) -> Error
    where
        Error: From<internal::FileSourceCodeDiagnostic<Self>>,
    {
        Error::from(internal::FileSourceCodeDiagnostic {
            source_code: source_code.as_source_code().map(SourceCode::to_owned),
            source: self,
        })
    }

    fn with_tags(self, tags: DiagnosticTags) -> Error
    where
        Error: From<internal::TagsDiagnostic<Self>>,
    {
        Error::from(internal::TagsDiagnostic { tags, source: self })
    }

    fn with_severity(self, severity: Severity) -> Error
    where
        Error: From<internal::SeverityDiagnostic<Self>>,
    {
        Error::from(internal::SeverityDiagnostic {
            severity,
            source: self,
        })
    }
}

pub trait Context<T, E>: internal::Sealed {
    /// If `self` is an error, returns a new diagnostic with the provided
    /// `message` as a message and description, and `self` as a source
    /// diagnostic. This is useful to create chains of diagnostics, where high
    /// level errors wrap lower level causes.
    fn context<M>(self, message: M) -> Result<T, Error>
    where
        E: 'static,
        M: fmt::Display + 'static,
        Error: From<internal::ContextDiagnostic<M, E>>;

    /// If `self` is an error, returns a new diagnostic using the provided
    /// `category` if `self` doesn't already have one.
    fn with_category(self, category: &'static Category) -> Result<T, Error>
    where
        Error: From<internal::CategoryDiagnostic<E>>;

    /// If `self` is an error, returns a new diagnostic using the provided
    /// `path` if `self` doesn't already have one.
    fn with_file_path(self, path: impl AsResource) -> Result<T, Error>
    where
        Error: From<internal::FilePathDiagnostic<E>>;

    /// If `self` is an error, returns a new diagnostic using the provided
    /// `severity` if `self` doesn't already have one.
    fn with_severity(self, severity: Severity) -> Result<T, Error>
    where
        Error: From<internal::SeverityDiagnostic<E>>;

    /// If `self` is an error, returns a new diagnostic using the provided
    /// `tags` if `self` doesn't already have one.
    fn with_tags(self, tags: DiagnosticTags) -> Result<T, Error>
    where
        Error: From<internal::TagsDiagnostic<E>>;

    /// If `self` is an error, returns a new diagnostic using the provided
    /// `span` instead of the one returned by `self`.
    ///
    /// This is useful in multi-language documents, where a given diagnostic
    /// may be originally emitted with a span relative to a specific substring
    /// of a larger document, and later needs to have its position remapped to
    /// be relative to the entire file instead.
    fn with_file_span(self, span: impl AsSpan) -> Result<T, Error>
    where
        Error: From<internal::FileSpanDiagnostic<E>>;
}

impl<T, E: AsDiagnostic> internal::Sealed for Result<T, E> {}

impl<T, E: AsDiagnostic> Context<T, E> for Result<T, E> {
    fn context<M>(self, message: M) -> Result<T, Error>
    where
        E: 'static,
        M: fmt::Display + 'static,
        Error: From<internal::ContextDiagnostic<M, E>>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(source.context(message)),
        }
    }

    fn with_category(self, category: &'static Category) -> Result<T, Error>
    where
        Error: From<internal::CategoryDiagnostic<E>>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(source.with_category(category)),
        }
    }

    fn with_file_path(self, path: impl AsResource) -> Result<T, Error>
    where
        Error: From<internal::FilePathDiagnostic<E>>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(source.with_file_path(path)),
        }
    }

    fn with_severity(self, severity: Severity) -> Result<T, Error>
    where
        Error: From<SeverityDiagnostic<E>>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(source.with_severity(severity)),
        }
    }

    fn with_tags(self, tags: DiagnosticTags) -> Result<T, Error>
    where
        Error: From<TagsDiagnostic<E>>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(source.with_tags(tags)),
        }
    }

    fn with_file_span(self, span: impl AsSpan) -> Result<T, Error>
    where
        Error: From<internal::FileSpanDiagnostic<E>>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(source) => Err(source.with_file_span(span)),
        }
    }
}

mod internal {
    //! These types need to be declared as public as they're referred to in the
    //! `where` clause of other public items, but as they're not part of the
    //! public API they are declared in a private module so they're not
    //! accessible outside of the crate

    use std::{fmt::Debug, io};

    use biome_console::{fmt, markup};
    use biome_rowan::TextRange;
    use biome_text_edit::TextEdit;

    use crate::{
        Advices, Backtrace, Category, Diagnostic, DiagnosticTags, LineIndex, LineIndexBuf,
        Location, LogCategory, Resource, Severity, SourceCode, Visit,
        diagnostic::internal::AsDiagnostic,
    };

    /// This trait is inherited by `DiagnosticExt` and `Context`, since it's
    /// not visible outside of `biome_diagnostics` this prevents these extension
    /// traits from being implemented on other types outside of this module
    ///
    /// Making these traits "sealed" is mainly intended as a stability
    /// guarantee, if these traits were simply public any change to their
    /// signature or generic implementations would be a breaking change for
    /// downstream implementations, so preventing these traits from ever being
    /// implemented in downstream crates ensures this doesn't happen.
    pub trait Sealed {}

    /// Diagnostic type returned by [super::DiagnosticExt::context], uses
    /// `message` as its message and description, and `source` as its source
    /// diagnostic.
    pub struct ContextDiagnostic<M, E> {
        pub(super) message: M,
        pub(super) source: E,
    }

    impl<M: fmt::Display + 'static, E: AsDiagnostic> Diagnostic for ContextDiagnostic<M, E> {
        fn category(&self) -> Option<&'static Category> {
            self.source.as_diagnostic().category()
        }

        fn severity(&self) -> Severity {
            self.source.as_diagnostic().severity()
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut writer = DisplayMarkup(fmt);
            let mut fmt = fmt::Formatter::new(&mut writer);
            fmt.write_markup(markup!({ self.message }))
                .map_err(|_| std::fmt::Error)
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            fmt::Display::fmt(&self.message, fmt)
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().advices(visitor)
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().verbose_advices(visitor)
        }

        fn location(&self) -> Location<'_> {
            self.source.as_diagnostic().location()
        }

        fn tags(&self) -> DiagnosticTags {
            self.source.as_diagnostic().tags()
        }

        fn source(&self) -> Option<&dyn Diagnostic> {
            Some(self.source.as_dyn())
        }
    }

    impl<M: fmt::Display, E: Debug> Debug for ContextDiagnostic<M, E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Diagnostic")
                .field("message", &DebugMarkup(&self.message))
                .field("source", &self.source)
                .finish()
        }
    }

    /// Helper wrapper implementing [Debug] for types implementing [fmt::Display],
    /// prints a debug representation of the markup generated by printing `T`.
    struct DebugMarkup<T>(T);

    impl<T: fmt::Display> Debug for DebugMarkup<T> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let buffer = markup!({ self.0 }).to_owned();
            Debug::fmt(&buffer, fmt)
        }
    }

    /// Helper wrapper implementing [fmt::Write] for [std::fmt::Formatter].
    struct DisplayMarkup<'a, 'b>(&'a mut std::fmt::Formatter<'b>);

    impl fmt::Write for DisplayMarkup<'_, '_> {
        fn write_str(&mut self, _: &fmt::MarkupElements<'_>, content: &str) -> io::Result<()> {
            self.0.write_str(content).map_err(io::Error::other)
        }

        fn write_fmt(
            &mut self,
            _: &fmt::MarkupElements<'_>,
            content: std::fmt::Arguments<'_>,
        ) -> io::Result<()> {
            self.0.write_fmt(content).map_err(io::Error::other)
        }
    }

    /// Diagnostic type returned by [super::DiagnosticExt::with_category],
    /// uses `category` as its category if `source` doesn't return one.
    pub struct CategoryDiagnostic<E> {
        pub(super) category: &'static Category,
        pub(super) source: E,
    }

    impl<E: Debug> Debug for CategoryDiagnostic<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Diagnostic")
                .field("category", &self.category)
                .field("source", &self.source)
                .finish()
        }
    }

    impl<E: AsDiagnostic> Diagnostic for CategoryDiagnostic<E> {
        fn category(&self) -> Option<&'static Category> {
            Some(
                self.source
                    .as_diagnostic()
                    .category()
                    .unwrap_or(self.category),
            )
        }

        fn severity(&self) -> Severity {
            self.source.as_diagnostic().severity()
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.source.as_diagnostic().description(fmt)
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            self.source.as_diagnostic().message(fmt)
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().advices(visitor)
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().verbose_advices(visitor)
        }

        fn location(&self) -> Location<'_> {
            self.source.as_diagnostic().location()
        }

        fn tags(&self) -> DiagnosticTags {
            self.source.as_diagnostic().tags()
        }
    }

    /// Diagnostic type returned by [super::DiagnosticExt::with_file_path],
    /// uses `path` as its location path if `source` doesn't return one.
    pub struct FilePathDiagnostic<E> {
        pub(super) path: Option<Resource<String>>,
        pub(super) source: E,
    }

    impl<E: Debug> Debug for FilePathDiagnostic<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Diagnostic")
                .field("path", &self.path)
                .field("source", &self.source)
                .finish()
        }
    }

    impl<E: AsDiagnostic> Diagnostic for FilePathDiagnostic<E> {
        fn category(&self) -> Option<&'static Category> {
            self.source.as_diagnostic().category()
        }

        fn severity(&self) -> Severity {
            self.source.as_diagnostic().severity()
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.source.as_diagnostic().description(fmt)
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            self.source.as_diagnostic().message(fmt)
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().advices(visitor)
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().verbose_advices(visitor)
        }

        fn location(&self) -> Location<'_> {
            let loc = self.source.as_diagnostic().location();
            Location {
                resource: match loc.resource {
                    Some(Resource::Argv) => Some(Resource::Argv),
                    Some(Resource::Memory) => Some(Resource::Memory),
                    Some(Resource::File(file)) => {
                        if let Some(Resource::File(path)) = &self.path {
                            Some(Resource::File(path.as_ref()))
                        } else {
                            Some(Resource::File(file))
                        }
                    }
                    None => self.path.as_ref().map(Resource::as_deref),
                },
                span: loc.span,
                source_code: loc.source_code,
            }
        }

        fn tags(&self) -> DiagnosticTags {
            self.source.as_diagnostic().tags()
        }
    }

    /// Diagnostic type returned by [super::DiagnosticExt::with_file_span],
    /// uses `span` as its location span instead of the one returned by `source`.
    pub struct FileSpanDiagnostic<E> {
        pub(super) span: Option<TextRange>,
        pub(super) source: E,
    }

    impl<E: Debug> Debug for FileSpanDiagnostic<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Diagnostic")
                .field("span", &self.span)
                .field("source", &self.source)
                .finish()
        }
    }

    impl<E: AsDiagnostic> Diagnostic for FileSpanDiagnostic<E> {
        fn category(&self) -> Option<&'static Category> {
            self.source.as_diagnostic().category()
        }

        fn severity(&self) -> Severity {
            self.source.as_diagnostic().severity()
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.source.as_diagnostic().description(fmt)
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            self.source.as_diagnostic().message(fmt)
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().advices(visitor)
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().verbose_advices(visitor)
        }

        fn location(&self) -> Location<'_> {
            let loc = self.source.as_diagnostic().location();
            Location {
                resource: loc.resource,
                span: self.span.or(loc.span),
                source_code: loc.source_code,
            }
        }

        fn tags(&self) -> DiagnosticTags {
            self.source.as_diagnostic().tags()
        }
    }

    /// Diagnostic type returned by [super::DiagnosticExt::with_file_source_code],
    /// uses `source_code` as its location source code if `source` doesn't
    /// return one.
    pub struct FileSourceCodeDiagnostic<E> {
        pub(super) source_code: Option<SourceCode<String, LineIndexBuf>>,
        pub(super) source: E,
    }

    impl<E: Debug> Debug for FileSourceCodeDiagnostic<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Diagnostic")
                .field("source_code", &self.source_code)
                .field("source", &self.source)
                .finish()
        }
    }

    impl<E: AsDiagnostic> Diagnostic for FileSourceCodeDiagnostic<E> {
        fn category(&self) -> Option<&'static Category> {
            self.source.as_diagnostic().category()
        }

        fn severity(&self) -> Severity {
            self.source.as_diagnostic().severity()
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.source.as_diagnostic().description(fmt)
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            self.source.as_diagnostic().message(fmt)
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            if let Some(source_code) = &self.source_code {
                let mut visitor = FileSourceCodeVisitor {
                    visitor,
                    source_code: source_code.as_deref(),
                };

                self.source.as_diagnostic().advices(&mut visitor)
            } else {
                self.source.as_diagnostic().advices(visitor)
            }
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            if let Some(source_code) = &self.source_code {
                let mut visitor = FileSourceCodeVisitor {
                    visitor,
                    source_code: source_code.as_deref(),
                };

                self.source.as_diagnostic().verbose_advices(&mut visitor)
            } else {
                self.source.as_diagnostic().verbose_advices(visitor)
            }
        }

        fn location(&self) -> Location<'_> {
            let location = self.source.as_diagnostic().location();
            Location {
                source_code: location
                    .source_code
                    .or_else(|| Some(self.source_code.as_ref()?.as_deref())),
                ..location
            }
        }

        fn tags(&self) -> DiagnosticTags {
            self.source.as_diagnostic().tags()
        }
    }

    /// Helper wrapper for a [Visitor], automatically inject `source_code` into
    /// the location of code frame advices if they don't have one already.
    struct FileSourceCodeVisitor<'a> {
        visitor: &'a mut dyn Visit,
        source_code: SourceCode<&'a str, &'a LineIndex>,
    }

    impl Visit for FileSourceCodeVisitor<'_> {
        fn record_log(&mut self, category: LogCategory, text: &dyn fmt::Display) -> io::Result<()> {
            self.visitor.record_log(category, text)
        }

        fn record_list(&mut self, list: &[&dyn fmt::Display]) -> io::Result<()> {
            self.visitor.record_list(list)
        }

        fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
            self.visitor.record_frame(Location {
                source_code: Some(location.source_code.unwrap_or(self.source_code)),
                ..location
            })
        }

        fn record_diff(&mut self, diff: &TextEdit) -> io::Result<()> {
            self.visitor.record_diff(diff)
        }

        fn record_backtrace(
            &mut self,
            title: &dyn fmt::Display,
            backtrace: &Backtrace,
        ) -> io::Result<()> {
            self.visitor.record_backtrace(title, backtrace)
        }

        fn record_command(&mut self, command: &str) -> io::Result<()> {
            self.visitor.record_command(command)
        }

        fn record_group(
            &mut self,
            title: &dyn fmt::Display,
            advice: &dyn Advices,
        ) -> io::Result<()> {
            self.visitor.record_group(title, advice)
        }
    }

    /// Diagnostic type returned by [super::DiagnosticExt::with_tags],
    /// merges `tags` with the tags of its source
    pub struct TagsDiagnostic<E> {
        pub(super) tags: DiagnosticTags,
        pub(super) source: E,
    }

    impl<E: Debug> Debug for TagsDiagnostic<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Diagnostic")
                .field("tags", &self.tags)
                .field("source", &self.source)
                .finish()
        }
    }

    impl<E: AsDiagnostic> Diagnostic for TagsDiagnostic<E> {
        fn category(&self) -> Option<&'static Category> {
            self.source.as_diagnostic().category()
        }

        fn severity(&self) -> Severity {
            self.source.as_diagnostic().severity()
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.source.as_diagnostic().description(fmt)
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            self.source.as_diagnostic().message(fmt)
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().advices(visitor)
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().verbose_advices(visitor)
        }

        fn location(&self) -> Location<'_> {
            self.source.as_diagnostic().location()
        }

        fn tags(&self) -> DiagnosticTags {
            self.source.as_diagnostic().tags() | self.tags
        }
    }

    /// Diagnostic type returned by [super::DiagnosticExt::with_severity],
    /// replaces `severity` with the severity of its source
    pub struct SeverityDiagnostic<E> {
        pub(super) severity: Severity,
        pub(super) source: E,
    }

    impl<E: Debug> Debug for SeverityDiagnostic<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Diagnostic")
                .field("severity", &self.severity)
                .field("source", &self.source)
                .finish()
        }
    }

    impl<E: AsDiagnostic> Diagnostic for SeverityDiagnostic<E> {
        fn category(&self) -> Option<&'static Category> {
            self.source.as_diagnostic().category()
        }

        fn severity(&self) -> Severity {
            self.severity
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.source.as_diagnostic().description(fmt)
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            self.source.as_diagnostic().message(fmt)
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().advices(visitor)
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            self.source.as_diagnostic().verbose_advices(visitor)
        }

        fn location(&self) -> Location<'_> {
            self.source.as_diagnostic().location()
        }

        fn tags(&self) -> DiagnosticTags {
            self.source.as_diagnostic().tags()
        }
    }
}
