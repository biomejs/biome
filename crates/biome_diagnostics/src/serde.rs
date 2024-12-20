use std::io;

use biome_console::{fmt, markup, MarkupBuf};
use biome_rowan::TextSize;
use biome_text_edit::TextEdit;
use biome_text_size::TextRange;
use serde::{
    de::{self, SeqAccess},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{
    diagnostic::internal::AsDiagnostic, diagnostic::DiagnosticTag, Advices as _, Backtrace,
    Category, DiagnosticTags, LogCategory, Resource, Severity, SourceCode, Visit,
};

/// Serializable representation for a [Diagnostic](super::Diagnostic).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Diagnostic {
    category: Option<&'static Category>,
    severity: Severity,
    description: String,
    message: MarkupBuf,
    advices: Advices,
    verbose_advices: Advices,
    location: Location,
    tags: DiagnosticTags,
    source: Option<Box<Self>>,
}

impl Diagnostic {
    pub fn new<D: AsDiagnostic>(diag: D) -> Self {
        Self::new_impl(diag.as_diagnostic())
    }

    fn new_impl<D: super::Diagnostic + ?Sized>(diag: &D) -> Self {
        let category = diag.category();

        let severity = diag.severity();

        let description = PrintDescription(diag).to_string();

        let mut message = MarkupBuf::default();
        let mut fmt = fmt::Formatter::new(&mut message);
        // SAFETY: Writing to a MarkupBuf should never fail
        diag.message(&mut fmt).unwrap();

        let mut advices = Advices::new();
        // SAFETY: The Advices visitor never returns an error
        diag.advices(&mut advices).unwrap();

        let mut verbose_advices = Advices::new();
        // SAFETY: The Advices visitor never returns an error
        diag.verbose_advices(&mut verbose_advices).unwrap();

        let location = diag.location().into();

        let tags = diag.tags();

        let source = diag.source().map(Self::new_impl).map(Box::new);

        Self {
            category,
            severity,
            description,
            message,
            advices,
            verbose_advices,
            location,
            tags,
            source,
        }
    }

    pub fn with_offset(mut self, offset: TextSize) -> Self {
        self.location.span = self
            .location
            .span
            .map(|span| TextRange::new(span.start() + offset, span.end() + offset));
        self
    }
}

impl super::Diagnostic for Diagnostic {
    fn category(&self) -> Option<&'static Category> {
        self.category
    }

    fn severity(&self) -> Severity {
        self.severity
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.description)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup! { {self.message} })
    }

    fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        self.advices.record(visitor)
    }

    fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        self.verbose_advices.record(visitor)
    }

    fn location(&self) -> super::Location<'_> {
        super::Location::builder()
            .resource(&self.location.path)
            .span(&self.location.span)
            .source_code(&self.location.source_code)
            .build()
    }

    fn tags(&self) -> DiagnosticTags {
        self.tags
    }

    fn source(&self) -> Option<&dyn super::Diagnostic> {
        self.source
            .as_deref()
            .map(|source| source as &dyn super::Diagnostic)
    }
}

/// Wrapper type implementing [std::fmt::Display] for types implementing [Diagnostic](super::Diagnostic),
/// prints the description of the diagnostic as a string.
struct PrintDescription<'fmt, D: ?Sized>(pub &'fmt D);

impl<D: super::Diagnostic + ?Sized> std::fmt::Display for PrintDescription<'_, D> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.description(fmt).map_err(|_| std::fmt::Error)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Location {
    path: Option<Resource<String>>,
    span: Option<TextRange>,
    source_code: Option<String>,
}

impl From<super::Location<'_>> for Location {
    fn from(loc: super::Location<'_>) -> Self {
        Self {
            path: loc.resource.map(super::Resource::to_owned),
            span: loc.span,
            source_code: loc
                .source_code
                .map(|source_code| source_code.text.to_string()),
        }
    }
}

/// Implementation of [Visitor] collecting serializable [Advice] into a vector.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Advices {
    advices: Vec<Advice>,
}

impl Advices {
    fn new() -> Self {
        Self {
            advices: Vec::new(),
        }
    }
}

impl Visit for Advices {
    fn record_log(&mut self, category: LogCategory, text: &dyn fmt::Display) -> io::Result<()> {
        self.advices
            .push(Advice::Log(category, markup!({ text }).to_owned()));
        Ok(())
    }

    fn record_list(&mut self, list: &[&dyn fmt::Display]) -> io::Result<()> {
        self.advices.push(Advice::List(
            list.iter()
                .map(|item| markup!({ item }).to_owned())
                .collect(),
        ));
        Ok(())
    }

    fn record_frame(&mut self, location: super::Location<'_>) -> io::Result<()> {
        self.advices.push(Advice::Frame(location.into()));
        Ok(())
    }

    fn record_diff(&mut self, diff: &TextEdit) -> io::Result<()> {
        self.advices.push(Advice::Diff(diff.clone()));
        Ok(())
    }

    fn record_backtrace(
        &mut self,
        title: &dyn fmt::Display,
        backtrace: &Backtrace,
    ) -> io::Result<()> {
        self.advices.push(Advice::Backtrace(
            markup!({ title }).to_owned(),
            backtrace.clone(),
        ));
        Ok(())
    }

    fn record_command(&mut self, command: &str) -> io::Result<()> {
        self.advices.push(Advice::Command(command.into()));
        Ok(())
    }

    fn record_group(
        &mut self,
        title: &dyn fmt::Display,
        advice: &dyn super::Advices,
    ) -> io::Result<()> {
        let mut advices = Advices::new();
        advice.record(&mut advices)?;

        self.advices
            .push(Advice::Group(markup!({ title }).to_owned(), advices));
        Ok(())
    }
}

impl super::Advices for Advices {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        for advice in &self.advices {
            advice.record(visitor)?;
        }

        Ok(())
    }
}

/// Serializable representation of a [Diagnostic](super::Diagnostic) advice
///
/// See the [Visitor] trait for additional documentation on all the supported
/// advice types.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(test, derive(Eq, PartialEq))]
enum Advice {
    Log(LogCategory, MarkupBuf),
    List(Vec<MarkupBuf>),
    Frame(Location),
    Diff(TextEdit),
    Backtrace(MarkupBuf, Backtrace),
    Command(String),
    Group(MarkupBuf, Advices),
}

impl super::Advices for Advice {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        match self {
            Advice::Log(category, text) => visitor.record_log(*category, text),
            Advice::List(list) => {
                let as_display: Vec<&dyn fmt::Display> =
                    list.iter().map(|item| item as &dyn fmt::Display).collect();
                visitor.record_list(&as_display)
            }
            Advice::Frame(location) => visitor.record_frame(super::Location {
                resource: location.path.as_ref().map(super::Resource::as_deref),
                span: location.span,
                source_code: location.source_code.as_deref().map(|text| SourceCode {
                    text,
                    line_starts: None,
                }),
            }),
            Advice::Diff(diff) => visitor.record_diff(diff),
            Advice::Backtrace(title, backtrace) => visitor.record_backtrace(title, backtrace),
            Advice::Command(command) => visitor.record_command(command),
            Advice::Group(title, advice) => visitor.record_group(title, advice),
        }
    }
}

impl From<DiagnosticTag> for DiagnosticTags {
    fn from(tag: DiagnosticTag) -> Self {
        match tag {
            DiagnosticTag::Fixable => DiagnosticTags::FIXABLE,
            DiagnosticTag::Internal => DiagnosticTags::INTERNAL,
            DiagnosticTag::UnnecessaryCode => DiagnosticTags::UNNECESSARY_CODE,
            DiagnosticTag::DeprecatedCode => DiagnosticTags::DEPRECATED_CODE,
            DiagnosticTag::Verbose => DiagnosticTags::VERBOSE,
        }
    }
}

// Custom `serde` implementation for `DiagnosticTags` as a list of `DiagnosticTag` enum
impl Serialize for DiagnosticTags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut flags = Vec::new();

        if self.contains(Self::FIXABLE) {
            flags.push(DiagnosticTag::Fixable);
        }

        if self.contains(Self::INTERNAL) {
            flags.push(DiagnosticTag::Internal);
        }

        if self.contains(Self::UNNECESSARY_CODE) {
            flags.push(DiagnosticTag::UnnecessaryCode);
        }

        if self.contains(Self::DEPRECATED_CODE) {
            flags.push(DiagnosticTag::DeprecatedCode);
        }

        serializer.collect_seq(flags)
    }
}

impl<'de> Deserialize<'de> for DiagnosticTags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DiagnosticTags;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "DiagnosticTags")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut result = DiagnosticTags::empty();

                while let Some(item) = seq.next_element::<DiagnosticTag>()? {
                    result |= DiagnosticTags::from(item);
                }

                Ok(result)
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for DiagnosticTags {
    fn schema_name() -> String {
        String::from("DiagnosticTags")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <Vec<DiagnosticTag>>::json_schema(gen)
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use biome_text_size::{TextRange, TextSize};
    use serde_json::{from_value, json, to_value, Value};

    use crate::{
        self as biome_diagnostics, {Advices, LogCategory, Visit},
    };
    use biome_diagnostics_macros::Diagnostic;

    #[derive(Debug, Diagnostic)]
    #[diagnostic(
        severity = Warning,
        category = "internalError/io",
        message(
            description = "text description",
            message(<Emphasis>"markup message"</Emphasis>),
        ),
        tags(INTERNAL)
    )]
    struct TestDiagnostic {
        #[location(resource)]
        path: String,
        #[location(span)]
        span: TextRange,
        #[location(source_code)]
        source_code: String,
        #[advice]
        advices: TestAdvices,
        #[verbose_advice]
        verbose_advices: TestAdvices,
    }

    impl Default for TestDiagnostic {
        fn default() -> Self {
            TestDiagnostic {
                path: String::from("path"),
                span: TextRange::new(TextSize::from(0), TextSize::from(6)),
                source_code: String::from("source_code"),
                advices: TestAdvices,
                verbose_advices: TestAdvices,
            }
        }
    }

    #[derive(Debug)]
    struct TestAdvices;

    impl Advices for TestAdvices {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_log(LogCategory::Warn, &"log")?;
            Ok(())
        }
    }

    fn serialized() -> Value {
        let advices = json!([
            {
                "log": [
                    "warn",
                    [
                        {
                            "content": "log",
                            "elements": []
                        }
                    ]
                ]
            }
        ]);

        json!({
            "category": "internalError/io",
            "severity": "warning",
            "description": "text description",
            "message": [
                {
                    "content": "markup message",
                    "elements": [
                        "Emphasis"
                    ]
                }
            ],
            "advices": {
                "advices": advices
            },
            "verboseAdvices": {
                "advices": advices
            },
            "location": {
                "path": {
                    "file": "path"
                },
                "sourceCode": "source_code",
                "span": [
                    0,
                    6
                ]
            },
            "tags": [
                "internal"
            ],
            "source": null
        })
    }

    #[test]
    fn test_serialize() {
        let diag = TestDiagnostic::default();
        let diag = super::Diagnostic::new(diag);
        let json = to_value(&diag).unwrap();

        let expected = serialized();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_deserialize() {
        let json = serialized();
        let diag: super::Diagnostic = from_value(json).unwrap();

        let expected = TestDiagnostic::default();
        let expected = super::Diagnostic::new(expected);

        assert_eq!(diag, expected);
    }
}
