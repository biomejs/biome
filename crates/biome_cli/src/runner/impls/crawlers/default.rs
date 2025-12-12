use crate::TraversalSummary;
use crate::execute::traverse::TraverseResult;
use crate::runner::crawler::{Crawler, CrawlerContext};
use crate::runner::impls::collectors::default::{CollectorSummary, DefaultCollector};
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

    fn output<Ctx>(
        ctx: &Ctx,
        collector_result: CollectorSummary,
        evaluated_paths: BTreeSet<BiomePath>,
        _duration: Duration,
    ) -> TraverseResult
    where
        Ctx: CrawlerContext,
    {
        let CollectorSummary {
            duration,
            scanner_duration,
            errors,
            warnings,
            infos,
            suggested_fixes_skipped,
            diagnostics_not_printed,
            diagnostics,
        } = collector_result;

        let changed = ctx.changed();
        let unchanged = ctx.unchanged();
        let matches = ctx.matches();
        let skipped = ctx.skipped();

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

    type CollectorOutput = ();
}
