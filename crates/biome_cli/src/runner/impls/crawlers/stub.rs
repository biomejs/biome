use crate::runner::collector::Collector;
use crate::runner::crawler::{Crawler, CrawlerContext};
use biome_fs::BiomePath;
use std::collections::BTreeSet;
use std::time::Duration;

pub(crate) struct StubCrawler;

impl Crawler<()> for StubCrawler {
    type Handler = ();
    type ProcessFile = ();
    type Collector = ();
    fn output<Ctx>(
        _ctx: &Ctx,
        _collector_result: <Self::Collector as Collector>::Result,
        _evaluated_paths: BTreeSet<BiomePath>,
        _duration: Duration,
    ) -> ()
    where
        Ctx: CrawlerContext,
    {
        ()
    }

    type CollectorOutput = ();
}
