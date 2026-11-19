use crate::suppressions::Suppressions;
use crate::{ControlFlow, MetadataRegistry, RuleCategory, SignalRuleKey};
use biome_rowan::{TextRange, TextSize};

/// Analyzes one embedded source and retains the result for the host document.
pub trait SnippetAnalyzer<Break> {
    /// The completed result returned to the host after this snippet runs.
    ///
    /// All snippets in a host analysis use the same output type. The host
    /// chooses it for its operation, such as lint diagnostics or code actions.
    type Output;

    /// Returns the offset of the snippet's first byte in the host document.
    fn diagnostics_offset(&self) -> TextSize;

    /// Returns the registry of rules defined by the snippet's analyzer.
    fn metadata(&self) -> &'static MetadataRegistry;

    /// Runs the snippet's typed analyzer with the host's signal inspector.
    ///
    /// A `Break` stops analysis of the remaining snippets.
    fn run(&mut self, inspector: EmbeddedSignalInspector<'_, '_>) -> ControlFlow<Break>;

    /// Returns the result collected by [`Self::run`].
    ///
    /// The host calls this after the snippet analysis completes.
    fn into_output(self: Box<Self>) -> Self::Output;
}

/// Checks ignore comments in the surrounding file using the snippet's position.
pub struct EmbeddedSignalInspector<'guest, 'registry> {
    host: &'guest mut Suppressions<'registry>,
    offset: TextSize,
}

impl<'guest, 'registry> EmbeddedSignalInspector<'guest, 'registry> {
    pub(crate) fn new(host: &'guest mut Suppressions<'registry>, offset: TextSize) -> Self {
        Self { host, offset }
    }

    /// Checks whether a finding from the snippet is ignored by a comment in the file.
    /// Records which comment was used.
    pub(crate) fn suppresses(
        &mut self,
        category: RuleCategory,
        rule: &SignalRuleKey,
        instances: &[Box<str>],
        range: TextRange,
    ) -> bool {
        self.host
            .suppresses(category, rule, instances, range + self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnalyzerSuppression, Never, RuleKey};
    use std::sync::LazyLock;

    struct TestSnippet {
        observed_offset: TextSize,
    }

    impl SnippetAnalyzer<Never> for TestSnippet {
        type Output = TextSize;

        fn diagnostics_offset(&self) -> TextSize {
            self.observed_offset
        }

        fn metadata(&self) -> &'static MetadataRegistry {
            static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(MetadataRegistry::default);
            &METADATA
        }

        fn run(&mut self, inspector: EmbeddedSignalInspector<'_, '_>) -> ControlFlow<Never> {
            self.observed_offset = inspector.offset;
            ControlFlow::Continue(())
        }

        fn into_output(self: Box<Self>) -> Self::Output {
            self.observed_offset
        }
    }

    #[test]
    fn boxed_snippet_receives_the_host_offset_and_returns_its_output() {
        let metadata = MetadataRegistry::default();
        let mut suppressions = Suppressions::new(&metadata);
        let mut snippet: Box<dyn SnippetAnalyzer<Never, Output = TextSize>> =
            Box::new(TestSnippet {
                observed_offset: TextSize::from(0),
            });

        let offset = TextSize::from(42);
        assert_eq!(snippet.diagnostics_offset(), TextSize::from(0));
        assert!(matches!(
            snippet.run(EmbeddedSignalInspector::new(&mut suppressions, offset)),
            ControlFlow::Continue(())
        ));
        assert_eq!(snippet.into_output(), offset);
    }

    #[test]
    fn inspector_matches_guest_ranges_in_the_host_document() {
        let mut metadata = MetadataRegistry::default();
        metadata.insert_rule("group", "rule");
        let mut suppressions = Suppressions::new(&metadata);
        let comment_range = TextRange::new(TextSize::from(100), TextSize::from(105));
        suppressions
            .push_suppression(
                &AnalyzerSuppression::rule(
                    RuleCategory::Lint,
                    "group/rule",
                    ("reason", TextRange::default()),
                ),
                comment_range,
                true,
            )
            .unwrap();
        suppressions.expand_range(TextRange::new(TextSize::from(105), TextSize::from(150)), 0);

        let rule = SignalRuleKey::Rule(RuleKey::new("group", "rule"));
        let mut inspector = EmbeddedSignalInspector::new(&mut suppressions, TextSize::from(120));
        assert!(inspector.suppresses(
            RuleCategory::Lint,
            &rule,
            &[],
            TextRange::new(TextSize::from(0), TextSize::from(5)),
        ));
        assert!(!inspector.suppresses(
            RuleCategory::Lint,
            &rule,
            &[],
            TextRange::new(TextSize::from(40), TextSize::from(45)),
        ));
        assert!(suppressions.line_suppressions[0].did_suppress_signal);
    }
}
