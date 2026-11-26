use crate::TraversalSummary;
use crate::runner::collector::Collector;
use crate::runner::crawler::{Crawler, EvaluatedPath};
use crate::runner::impls::collectors::default::{CollectorSummary, DefaultCollector};
use crate::runner::impls::commands::traversal::TraverseResult;
use crate::runner::impls::handlers::default::DefaultHandler;
use crate::runner::process_file::ProcessFile;
use std::time::Duration;

pub(crate) struct DefaultCrawler<P>(P);

impl<P> Crawler<TraverseResult> for DefaultCrawler<P>
where
    P: ProcessFile,
{
    type Handler = DefaultHandler;
    type ProcessFile = P;
    type Collector = DefaultCollector;

    fn output(
        collector_result: CollectorSummary,
        evaluated_paths: Vec<EvaluatedPath>,
        _duration: Duration,
    ) -> TraverseResult {
        let CollectorSummary {
            duration,
            scanner_duration,
            errors,
            traversal_errors,
            warnings,
            infos,
            suggested_fixes_skipped,
            diagnostics_not_printed,
            diagnostics,
            changed,
            unchanged,
            matches,
            skipped,
        } = collector_result;

        TraverseResult {
            summary: TraversalSummary {
                changed,
                unchanged,
                matches,
                duration,
                scanner_duration,
                errors,
                warnings,
                infos,
                skipped,
                suggested_fixes_skipped,
                diagnostics_not_printed,
            },
            diagnostics,
            evaluated_paths,
            traversal_errors,
        }
    }
}

impl Crawler<()> for () {
    type Handler = ();
    type ProcessFile = ();
    type Collector = ();

    fn output(_: <Self::Collector as Collector>::Result, _: Vec<EvaluatedPath>, _: Duration) {}
}
