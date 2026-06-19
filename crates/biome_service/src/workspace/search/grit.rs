use crate::WorkspaceError;
use crate::diagnostics::{QueryDiagnostic, SearchError};
use crate::settings::SettingsWithEditor;
use crate::workspace::{PatternId, SearchLanguage, SearchQuery};
use biome_css_syntax::TextRange;
use biome_fs::BiomePath;
use biome_grit_patterns::{
    CompilePatternOptions, CssTargetLanguage, GritQuery, GritQueryEffect, GritTargetFile,
    GritTargetLanguage, JsTargetLanguage, JsonTargetLanguage, compile_pattern_with_options,
};
use biome_languages::DocumentFileSource;
use biome_parser::AnyParse;
use papaya::HashMap;
use rustc_hash::FxBuildHasher;

#[derive(Default)]
pub struct GritSearchQuery {
    patterns: HashMap<PatternId, GritQuery, FxBuildHasher>,
}

impl SearchQuery for GritSearchQuery {
    fn parse_pattern(
        &self,
        pattern: &str,
        target_language: SearchLanguage,
    ) -> Result<PatternId, WorkspaceError> {
        let target_language = match target_language {
            SearchLanguage::Css => GritTargetLanguage::from(CssTargetLanguage),
            SearchLanguage::Js => GritTargetLanguage::from(JsTargetLanguage),
            SearchLanguage::Json => GritTargetLanguage::from(JsonTargetLanguage),
        };
        let options = CompilePatternOptions::default().with_default_language(target_language);
        let pattern = compile_pattern_with_options(pattern, options)?;

        let pattern_id = self.generate_pattern_id();
        self.patterns.pin().insert(pattern_id.clone(), pattern);
        Ok(pattern_id)
    }

    fn drop_pattern(&self, pattern_id: PatternId) {
        self.patterns.pin().remove(&pattern_id);
    }

    fn search(
        &self,
        path: &BiomePath,
        _file_source: &DocumentFileSource,
        parse: AnyParse,
        _settings: &SettingsWithEditor,
        pattern_id: PatternId,
    ) -> Result<Vec<TextRange>, WorkspaceError> {
        let patterns = self.patterns.pin();
        let query = patterns
            .get(&pattern_id)
            .ok_or_else(WorkspaceError::invalid_pattern)?;
        let result = query
            .execute(GritTargetFile::new(path.as_path(), parse))
            .map_err(|err| {
                WorkspaceError::SearchError(SearchError::QueryError(QueryDiagnostic(
                    err.to_string(),
                )))
            })?;

        let matches = result
            .effects
            .into_iter()
            .flat_map(|result| match result {
                GritQueryEffect::Match(m) => m.ranges,
                _ => Vec::new(),
            })
            .map(|range| TextRange::new(range.start_byte.into(), range.end_byte.into()))
            .collect();

        Ok(matches)
    }
}
