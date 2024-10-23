use std::path::Path;

use crate::grit_context::GritQueryContext;
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::GritTargetLanguage;
use grit_pattern_matcher::{
    constant::Constant,
    pattern::{File, FilePtr, FileRegistry, ResolvedFile, ResolvedPattern},
};
use grit_util::{error::GritResult, Ast};
use path_absolutize::Absolutize;

#[derive(Clone, Debug, PartialEq)]
pub enum GritFile<'a> {
    Resolved(Box<ResolvedFile<'a, GritQueryContext>>),
    Ptr(FilePtr),
}

impl<'a> File<'a, GritQueryContext> for GritFile<'a> {
    fn name(&self, files: &FileRegistry<'a, GritQueryContext>) -> GritResolvedPattern<'a> {
        match self {
            Self::Resolved(resolved) => resolved.name.clone(),
            Self::Ptr(ptr) => GritResolvedPattern::from_path_binding(files.get_file_name(*ptr)),
        }
    }

    fn absolute_path(
        &self,
        files: &FileRegistry<'a, GritQueryContext>,
        language: &GritTargetLanguage,
    ) -> GritResult<GritResolvedPattern<'a>> {
        match self {
            Self::Resolved(resolved) => {
                let name = resolved.name.text(files, language)?;
                let absolute_path = Path::new(name.as_ref()).absolutize()?;
                Ok(ResolvedPattern::from_constant(Constant::String(
                    absolute_path.to_string_lossy().to_string(),
                )))
            }
            Self::Ptr(ptr) => Ok(ResolvedPattern::from_path_binding(
                files.get_absolute_path(*ptr)?,
            )),
        }
    }

    fn body(&self, files: &FileRegistry<'a, GritQueryContext>) -> GritResolvedPattern<'a> {
        match self {
            Self::Resolved(resolved) => resolved.body.clone(),
            Self::Ptr(ptr) => {
                let file = &files.get_file_owner(*ptr);
                GritResolvedPattern::from_tree(&file.tree)
            }
        }
    }

    fn binding(&self, files: &FileRegistry<'a, GritQueryContext>) -> GritResolvedPattern<'a> {
        match self {
            Self::Resolved(resolved) => resolved.body.clone(),
            Self::Ptr(ptr) => {
                let file = &files.get_file_owner(*ptr);
                ResolvedPattern::from_node_binding(file.tree.root_node())
            }
        }
    }
}
