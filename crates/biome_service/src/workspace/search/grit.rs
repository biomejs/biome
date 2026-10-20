use crate::WorkspaceError;
use crate::diagnostics::{QueryDiagnostic, SearchError};
use crate::settings::SettingsWithEditor;
use crate::workspace::{PatternId, SearchCapture, SearchLanguage, SearchMatch, SearchQuery};
use biome_fs::BiomePath;
use biome_grit_patterns::{
    CompilePatternOptions, CssTargetLanguage, GritQuery, GritQueryEffect, GritTargetFile,
    GritTargetLanguage, JsTargetLanguage, JsonTargetLanguage, compile_pattern_with_options,
};
use biome_languages::DocumentFileSource;
use biome_parser::AnyParse;
use biome_rowan::TextRange;
use papaya::HashMap;
use rustc_hash::FxBuildHasher;
use std::collections::BTreeMap;
use std::fmt::Display;

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
        include_captures: bool,
    ) -> Result<Vec<SearchMatch>, WorkspaceError> {
        let patterns = self.patterns.pin();
        let query = patterns
            .get(&pattern_id)
            .ok_or_else(WorkspaceError::invalid_pattern)?;
        let result = query
            .execute(GritTargetFile::new(path.as_path(), parse.clone()))
            .map_err(to_search_error)?;
        // Recording captures runs the query on candidate nodes only, which can
        // find fewer matches than `execute`, so the matches always come from
        // `execute` and the captures are looked up by the range of each match.
        let mut captures = if include_captures {
            query
                .captures_by_match(GritTargetFile::new(path.as_path(), parse))
                .map_err(to_search_error)?
        } else {
            BTreeMap::new()
        };

        let mut matches = Vec::new();
        for effect in result.effects {
            let GritQueryEffect::Match(found) = effect else {
                continue;
            };
            for range in &found.ranges {
                matches.push(SearchMatch {
                    range: to_text_range(range.start_byte, range.end_byte),
                    captures: captures
                        .remove(range)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|capture| SearchCapture {
                            name: capture.name,
                            range: to_text_range(capture.range.start_byte, capture.range.end_byte),
                        })
                        .collect(),
                });
            }
        }

        Ok(matches)
    }
}

fn to_search_error(error: impl Display) -> WorkspaceError {
    WorkspaceError::SearchError(SearchError::QueryError(QueryDiagnostic(error.to_string())))
}

fn to_text_range(start_byte: u32, end_byte: u32) -> TextRange {
    TextRange::new(start_byte.into(), end_byte.into())
}
