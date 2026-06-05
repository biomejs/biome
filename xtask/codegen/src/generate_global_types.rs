//! Code-generator that downloads the TypeScript standard library and emits
//! Biome's built-in global type declarations.

use anyhow::bail;
use std::path::Path;

pub mod collect;
mod emit;
pub mod source;

pub const DEFAULT_TYPESCRIPT_TAG: &str = "v6.0.3";

/// Commit SHA pinned alongside [`DEFAULT_TYPESCRIPT_TAG`].
pub const DEFAULT_TYPESCRIPT_SHA: &str = "050880ce59e30b356b686bd3144efe24f875ebc8";

/// A pinned TypeScript release: git tag + commit SHA.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourcePin {
    tag: String,
    sha: String,
}

impl SourcePin {
    /// Builds a TypeScript source pin from a tag and commit SHA.
    pub fn new(tag: &str, sha: &str) -> Self {
        Self {
            tag: tag.to_owned(),
            sha: sha.to_owned(),
        }
    }

    /// Git tag of the TypeScript release, for example `v6.0.3`.
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// Commit SHA the tag must resolve to.
    pub fn sha(&self) -> &str {
        &self.sha
    }
}

/// Acquires the pinned TypeScript checkout, parses `libEntries`, walks the
/// reference closure, and emits `crates/biome_js_type_info/src/generated/global_types.rs`.
pub fn run() -> anyhow::Result<()> {
    let workspace_root = xtask_glue::project_root();
    let pin = SourcePin::new(DEFAULT_TYPESCRIPT_TAG, DEFAULT_TYPESCRIPT_SHA);
    let opts = source::SourceOptions {
        offline: false,
        repo_url_override: None,
    };
    run_with_workspace_root(&pin, &opts, &workspace_root)
}

/// Runs global-types codegen for an explicit source pin and options, writing the
/// generated module under `workspace_root`.
///
/// Tests drive the real acquire/emit path through this entry point with a
/// fixture pin and an isolated output tree.
pub fn run_with_workspace_root(
    pin: &SourcePin,
    opts: &source::SourceOptions,
    workspace_root: &Path,
) -> anyhow::Result<()> {
    let checkout = source::acquire(pin, opts)?;
    let libs = source::parse_lib_entries(&checkout)?;
    let source_files = source::discover(&checkout, &libs, source::PROFILE_ROOTS)?;
    collect_discovered_sources(&source_files)?;

    emit::emit_global_types(checkout.pin(), workspace_root)?;
    Ok(())
}

/// Runs the typed declaration collector over every discovered source and
/// fails the run if any pinned file produced a fatal parser diagnostic.
fn collect_discovered_sources(source_files: &[source::DiscoveredFile]) -> anyhow::Result<()> {
    let mut fatal_diagnostics = Vec::new();

    for source_file in source_files {
        let output = collect::collect(source_file);
        fatal_diagnostics.extend(output.coverage.into_iter().filter_map(|outcome| {
            let collect::CoverageOutcome::Diagnostic(diagnostic) = outcome else {
                return None;
            };
            is_fatal_collector_diagnostic(diagnostic.category).then(|| {
                format!(
                    "{}: {} at {:?} ({:?}){}",
                    diagnostic.file,
                    diagnostic.category,
                    diagnostic.range,
                    diagnostic.syntax_kind,
                    diagnostic
                        .detail
                        .as_ref()
                        .map(|detail| format!(": {detail}"))
                        .unwrap_or_default()
                )
            })
        }));
    }

    if !fatal_diagnostics.is_empty() {
        bail!(
            "global types collector found parser diagnostics in pinned TypeScript sources:\n{}",
            fatal_diagnostics.join("\n")
        );
    }

    Ok(())
}

/// Categories that should abort codegen instead of being recorded as
/// coverage diagnostics.
fn is_fatal_collector_diagnostic(category: &str) -> bool {
    matches!(
        category,
        "invalid_utf8" | "parser_error" | "unsupported_root"
    )
}
