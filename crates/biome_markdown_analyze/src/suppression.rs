use biome_analyze::{AnalyzerSuppression, Suppression, to_analyzer_suppressions};
use biome_rowan::TextRange;
use biome_suppression::{SuppressionDiagnostic, parse_suppression_comment};

pub struct MarkdownSuppression;

impl Suppression for MarkdownSuppression {
    type Diagnostic = SuppressionDiagnostic;

    fn parse_comment<'a>(
        &self,
        text: &'a str,
        range: TextRange,
    ) -> Vec<Result<AnalyzerSuppression<'a>, Self::Diagnostic>> {
        let mut result = Vec::new();
        for suppression in parse_suppression_comment(text) {
            match suppression {
                Ok(suppression) => result.extend(
                    to_analyzer_suppressions(suppression, range)
                        .into_iter()
                        .map(Ok),
                ),
                Err(error) => result.push(Err(error)),
            }
        }
        result
    }
}
