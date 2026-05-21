//! Code-generator that downloads the TypeScript standard library and emits
//! Biome's built-in global type declarations.

use crate::GlobalTypesArgs;

use anyhow::bail;
use sha2::{Digest as _, Sha256};
use std::fmt::{Debug, Write as _};
use std::path::Path;

pub mod collect;
mod emit;
pub mod source;

pub const DEFAULT_TYPESCRIPT_TAG: &str = "v6.0.3";

/// Commit SHA pinned alongside [`DEFAULT_TYPESCRIPT_TAG`].
pub const DEFAULT_TYPESCRIPT_SHA: &str = "050880ce59e30b356b686bd3144efe24f875ebc8";

pub use source_pin::SourcePin;

mod source_pin {
    use anyhow::bail;

    /// Length of a full SHA-1 git commit hash expressed in hexadecimal.
    const GIT_SHA1_HEX_LENGTH: usize = 40;

    /// Help text appended to every SourcePin::new tag-format error.
    const TAG_FORMAT_HELP: &str =
        "expected v<major>.<minor>.<patch> with optional -prerelease suffix";

    /// A pinned TypeScript release: git tag + commit SHA.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct SourcePin {
        tag: String,
        sha: String,
    }

    impl SourcePin {
        /// Builds a validated TypeScript source pin.
        pub fn new(tag: &str, sha: &str) -> anyhow::Result<Self> {
            let bytes = tag.as_bytes();
            let Some(&b'v') = bytes.first() else {
                bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
            };

            let mut index = 1;
            let consume_ascii_digits = |index: &mut usize| {
                let start = *index;
                while let Some(&byte) = bytes.get(*index) {
                    if !byte.is_ascii_digit() {
                        break;
                    }
                    *index += 1;
                }

                *index > start
            };

            if !consume_ascii_digits(&mut index) {
                bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
            }
            if bytes.get(index) != Some(&b'.') {
                bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
            }

            index += 1;
            if !consume_ascii_digits(&mut index) {
                bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
            }
            if bytes.get(index) != Some(&b'.') {
                bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
            }

            index += 1;
            if !consume_ascii_digits(&mut index) {
                bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
            }

            if let Some(&next) = bytes.get(index) {
                if next != b'-' {
                    bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
                }

                index += 1;
                let suffix_start = index;
                while let Some(&byte) = bytes.get(index) {
                    if !matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-') {
                        bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
                    }
                    index += 1;
                }

                if index == suffix_start {
                    bail!("invalid TypeScript source tag {tag:?}: {TAG_FORMAT_HELP}");
                }
            }

            if sha.len() != GIT_SHA1_HEX_LENGTH
                || !sha
                    .bytes()
                    .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
            {
                bail!(
                    "invalid TypeScript git commit SHA {sha:?}: expected {GIT_SHA1_HEX_LENGTH} lowercase hexadecimal characters"
                );
            }

            Ok(Self {
                tag: tag.to_owned(),
                sha: sha.to_owned(),
            })
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
}

/// Deterministic coverage counts produced by the typed declaration collector.
pub struct CollectorSummary {
    /// Total number of `Collected` declarations across every discovered file.
    pub declaration_count: usize,
    /// Total number of coverage outcomes (collected + scope events + diagnostics)
    /// across every discovered file.
    pub coverage_outcome_count: usize,
    /// SHA-256 hex of the cumulative `Debug`-formatted records and coverage
    /// outcomes in TypeScript Program order; embedded in the generated module.
    pub collector_output_sha256_hex: String,
}

/// Acquires the pinned TypeScript checkout, parses `libEntries`, walks the
/// reference closure, and emits `crates/biome_js_type_info/src/generated/global_types.rs`.
pub fn run(args: GlobalTypesArgs, mode: xtask_glue::Mode) -> anyhow::Result<()> {
    let workspace_root = xtask_glue::project_root();
    run_with_workspace_root(args, mode, &workspace_root)
}

/// Runs global-types codegen using `workspace_root` as the generated-output root.
///
/// This keeps the source cache rooted at the actual repository while allowing
/// tests to verify the real emit/verify path against an isolated output tree.
pub fn run_with_workspace_root(
    args: GlobalTypesArgs,
    mode: xtask_glue::Mode,
    workspace_root: &Path,
) -> anyhow::Result<()> {
    if args.verify && (args.ts_tag.is_some() || args.ts_sha.is_some()) {
        anyhow::bail!(
            "--verify cannot be combined with --ts-tag or --ts-sha; verification always uses the default pin"
        );
    }
    if args.ts_tag.is_some() != args.ts_sha.is_some() {
        anyhow::bail!("--ts-tag and --ts-sha must be supplied together");
    }

    let pin = SourcePin::new(
        args.ts_tag.as_deref().unwrap_or(DEFAULT_TYPESCRIPT_TAG),
        args.ts_sha.as_deref().unwrap_or(DEFAULT_TYPESCRIPT_SHA),
    )?;
    let opts = source::SourceOptions {
        offline: args.offline,
        verify: args.verify,
        repo_url_override: None,
    };

    let checkout = source::acquire(&pin, &opts)?;
    let libs = source::parse_lib_entries(&checkout)?;
    let source_files = source::discover(&checkout, &libs, source::PROFILE_ROOTS)?;
    let collector_summary = collect_discovered_sources(&source_files)?;

    emit::emit_global_types(
        checkout.pin(),
        checkout.command_line_parser_sha256(),
        &source_files,
        &collector_summary,
        mode,
        workspace_root,
    )?;
    Ok(())
}

/// Runs the typed declaration collector over every discovered source and
/// fails the run if any pinned file produced a fatal parser diagnostic.
fn collect_discovered_sources(
    source_files: &[source::DiscoveredFile],
) -> anyhow::Result<CollectorSummary> {
    let mut declaration_count = 0;
    let mut coverage_outcome_count = 0;
    let mut collector_output_sha256 = Sha256::new();
    let mut fatal_diagnostics = Vec::new();

    for source_file in source_files {
        let output = collect::collect(source_file);
        update_collector_output_hash(&mut collector_output_sha256, source_file, &output);
        declaration_count += output.records.len();
        coverage_outcome_count += output.coverage.len();
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

    let collector_output_digest = collector_output_sha256.finalize();
    Ok(CollectorSummary {
        declaration_count,
        coverage_outcome_count,
        collector_output_sha256_hex: sha256_hex(&collector_output_digest),
    })
}

/// Folds one collector output into the cumulative SHA-256 used to detect
/// silent changes in the typed declaration coverage across codegen runs.
fn update_collector_output_hash(
    hasher: &mut Sha256,
    source_file: &source::DiscoveredFile,
    output: &collect::CollectorOutput,
) {
    hasher.update(b"file ");
    hasher.update(source_file.repo_relative.as_bytes());
    hasher.update(b"\nrecords\n");
    for record in &output.records {
        update_hash_debug_line(hasher, record);
    }
    hasher.update(b"coverage\n");
    for outcome in &output.coverage {
        update_hash_debug_line(hasher, outcome);
    }
}

/// Streams one `Debug` line into the cumulative hash without an intermediate
/// `String` buffer.
fn update_hash_debug_line<T: Debug>(hasher: &mut Sha256, value: &T) {
    writeln!(&mut Sha256Writer(hasher), "{value:?}").expect("writing to Sha256 cannot fail");
}

/// `fmt::Write` adapter that feeds formatted text into a `Sha256`.
struct Sha256Writer<'a>(&'a mut Sha256);

impl std::fmt::Write for Sha256Writer<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.update(s.as_bytes());
        Ok(())
    }
}

/// Lowercase hex encoding for a SHA-256 digest, used for the collector digest.
fn sha256_hex(bytes: &[u8]) -> String {
    source::encode_sha256_hex(bytes)
}

/// Categories that should abort codegen instead of being recorded as
/// coverage diagnostics.
fn is_fatal_collector_diagnostic(category: &str) -> bool {
    matches!(
        category,
        "invalid_utf8" | "parser_error" | "unsupported_root"
    )
}

#[cfg(test)]
mod tests {
    use super::SourcePin;

    #[test]
    fn source_pin_accepts_prerelease_tag_and_lowercase_sha() {
        let pin = SourcePin::new("v6.0.3-rc.1", "a3f1b2c4d5e6f7890123456789abcdef01234567")
            .expect("valid source pin should be accepted");

        assert_eq!(pin.tag(), "v6.0.3-rc.1");
        assert_eq!(pin.sha(), "a3f1b2c4d5e6f7890123456789abcdef01234567");
    }

    #[test]
    fn source_pin_rejects_invalid_tags() {
        for tag in [
            "v1.2.3-",
            "v1.2.3-bad_suffix",
            "v1.2.3.4",
            "v1.a.3",
            "1.2.3",
            "abc",
        ] {
            assert!(
                SourcePin::new(tag, "a3f1b2c4d5e6f7890123456789abcdef01234567").is_err(),
                "invalid tag {tag:?} should be rejected"
            );
        }
    }

    #[test]
    fn source_pin_rejects_header_injection_inputs() {
        let sha = "a3f1b2c4d5e6f7890123456789abcdef01234567";
        for tag in [
            "v1.2.3\n",
            "v1.2.3\r",
            "v1.2.3*/",
            "v1.2.3-foo\n",
            "v1.2.3-foo*/bar",
        ] {
            assert!(
                SourcePin::new(tag, sha).is_err(),
                "tag {tag:?} must be rejected to prevent header injection"
            );
        }
    }

    #[test]
    fn source_pin_rejects_invalid_shas() {
        for sha in [
            "abc",
            "C63DE15A992D37F0D6CEC03AC7631872838602CB",
            "gggggggggggggggggggggggggggggggggggggggg",
        ] {
            assert!(
                SourcePin::new("v6.0.3", sha).is_err(),
                "invalid SHA {sha:?} should be rejected"
            );
        }
    }
}
