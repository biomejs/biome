use crate::grit_context::GritQueryContext;
use crate::grit_target_language::GritTargetLanguage;
use crate::resolved_pattern::GritResolvedPattern;
use grit_pattern_matcher::pattern::{File, FileRegistry};

pub(crate) struct GritFile;

impl<'a> File<'a, GritQueryContext> for GritFile {
    fn name(&self, _files: &FileRegistry<'a, GritQueryContext>) -> GritResolvedPattern {
        todo!()
    }

    fn absolute_path(
        &self,
        _files: &FileRegistry<'a, GritQueryContext>,
        _language: &GritTargetLanguage,
    ) -> anyhow::Result<GritResolvedPattern> {
        todo!()
    }

    fn body(&self, _files: &FileRegistry<'a, GritQueryContext>) -> GritResolvedPattern {
        todo!()
    }

    fn binding(&self, _files: &FileRegistry<'a, GritQueryContext>) -> GritResolvedPattern {
        todo!()
    }
}
