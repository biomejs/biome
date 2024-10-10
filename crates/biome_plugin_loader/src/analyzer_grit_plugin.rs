use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    rc::Rc,
};

use biome_analyze::RuleDiagnostic;
use biome_console::markup;
use biome_diagnostics::category;
use biome_fs::FileSystem;
use biome_grit_patterns::{
    compile_pattern, GritQuery, GritQueryResult, GritTargetFile, GritTargetLanguage,
    JsTargetLanguage,
};
use biome_parser::AnyParse;
use biome_rowan::TextRange;

use crate::{AnalyzerPlugin, PluginDiagnostic};

/// Definition of an analyzer plugin.
#[derive(Clone, Debug)]
pub struct AnalyzerGritPlugin {
    grit_query: Rc<GritQuery>,
}

impl AnalyzerGritPlugin {
    pub fn load(fs: &dyn FileSystem, path: &Path) -> Result<Self, PluginDiagnostic> {
        let source = fs.read_file_from_path(path)?;
        let query = compile_pattern(
            &source,
            Some(path),
            // TODO: Target language should be determined dynamically.
            GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
        )?;

        Ok(Self {
            grit_query: Rc::new(query),
        })
    }
}

impl AnalyzerPlugin for AnalyzerGritPlugin {
    fn evaluate(&self, root: AnyParse, path: PathBuf) -> Vec<RuleDiagnostic> {
        let name: &str = self.grit_query.name.as_deref().unwrap_or("anonymous");

        let file = GritTargetFile { parse: root, path };
        match self.grit_query.execute(file) {
            Ok((results, logs)) => results
                .into_iter()
                .filter_map(|result| match result {
                    GritQueryResult::Match(match_) => Some(match_),
                    GritQueryResult::Rewrite(_) | GritQueryResult::CreateFile(_) => None,
                })
                .map(|match_| {
                    RuleDiagnostic::new(
                        category!("plugin"),
                        match_.ranges.into_iter().next().map(from_grit_range),
                        markup!(<Emphasis>{name}</Emphasis>" matched"),
                    )
                })
                .chain(logs.iter().map(|log| {
                    RuleDiagnostic::new(
                        category!("plugin"),
                        log.range.map(from_grit_range),
                        markup!(<Emphasis>{name}</Emphasis>" logged: "<Info>{log.message}</Info>),
                    )
                    .verbose()
                }))
                .collect(),
            Err(error) => vec![RuleDiagnostic::new(
                category!("plugin"),
                None::<TextRange>,
                markup!(<Emphasis>{name}</Emphasis>" errored: "<Error>{error.to_string()}</Error>),
            )],
        }
    }
}

fn from_grit_range(range: grit_util::Range) -> TextRange {
    TextRange::new(range.start_byte.into(), range.end_byte.into())
}
