use crate::TraversalSummary;
use crate::runner::collector::Collector;
use crate::runner::crawler::Crawler;
use crate::runner::impls::collectors::default::{CollectorSummary, DefaultCollector};
use crate::runner::impls::commands::traversal::TraverseResult;
use crate::runner::impls::handlers::default::DefaultHandler;
use crate::runner::process_file::ProcessFile;
use biome_fs::BiomePath;
use std::collections::BTreeSet;
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
        evaluated_paths: BTreeSet<BiomePath>,
        _duration: Duration,
    ) -> TraverseResult {
        let CollectorSummary {
            duration,
            scanner_duration,
            errors,
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
        }
    }
}

impl Crawler<()> for () {
    type Handler = ();
    type ProcessFile = ();
    type Collector = ();

    fn output(_: <Self::Collector as Collector>::Result, _: BTreeSet<BiomePath>, _: Duration) {}
}
