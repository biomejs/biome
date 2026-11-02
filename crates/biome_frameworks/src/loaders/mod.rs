//! The loaders of the frameworks that Biome supports.

mod astro;
mod starlight;

pub use astro::AstroLoader;
pub use starlight::StarlightLoader;

use crate::{Contribution, Facet, LoaderContext};
use biome_glob::Glob;
use std::str::FromStr;

/// Visits the files of the framework that match the first of `globs`, each
/// relative to the directory of the framework, that matches files.
///
/// When none matches, the framework has a layout that the loader doesn't
/// know, so `facet` is incomplete.
fn visit_framework_files(
    ctx: &LoaderContext,
    globs: &[&str],
    facet: Facet,
    contributions: &mut Vec<Contribution>,
) {
    let framework_path = &ctx.framework().path;
    for glob in globs {
        let Ok(glob) = Glob::from_str(glob) else {
            continue;
        };
        let paths = ctx.glob(framework_path, &glob);
        if !paths.is_empty() {
            contributions.extend(paths.into_iter().map(Contribution::Visit));
            return;
        }
    }
    contributions.push(Contribution::Incomplete(facet));
}
