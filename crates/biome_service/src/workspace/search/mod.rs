#[cfg(feature = "lang_grit")]
pub(crate) mod grit;

use crate::WorkspaceError;
use crate::file_handlers::DocumentFileSource;
use crate::settings::SettingsWithEditor;
use crate::workspace::PatternId;
use biome_css_syntax::TextRange;
use biome_fs::BiomePath;
use biome_parser::AnyParse;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum SearchLanguage {
    Css,
    #[default]
    Js,
    Json,
}

impl FromStr for SearchLanguage {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "css" => Self::Css,
            "javascript" => Self::Js,
            "json" => Self::Json,
            _ => return Err("Target language must be one of: css, javascript, json"),
        })
    }
}

pub trait SearchQuery: Send + Sync {
    fn parse_pattern(
        &self,
        pattern: &str,
        target_language: SearchLanguage,
    ) -> Result<PatternId, WorkspaceError>;
    fn drop_pattern(&self, pattern_id: PatternId);
    fn search(
        &self,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
        _parse: AnyParse,
        _settings: &SettingsWithEditor,
        _: PatternId,
    ) -> Result<Vec<TextRange>, WorkspaceError>;

    fn generate_pattern_id(&self) -> PatternId {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let counter = COUNTER.fetch_add(1, Ordering::AcqRel);
        format!("p{counter}").into()
    }
}

pub struct NoopQueryProvider {}

impl SearchQuery for NoopQueryProvider {
    fn parse_pattern(&self, _: &str, _: SearchLanguage) -> Result<PatternId, WorkspaceError> {
        Err(WorkspaceError::feature_not_enabled())
    }

    fn drop_pattern(&self, _: PatternId) {}

    fn search(
        &self,
        _: &BiomePath,
        _: &DocumentFileSource,
        _: AnyParse,
        _: &SettingsWithEditor,
        _: PatternId,
    ) -> Result<Vec<TextRange>, WorkspaceError> {
        Ok(vec![])
    }
}
