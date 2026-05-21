#![cfg(feature = "global_types")]

use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

use anyhow::{Context as _, Result, bail};
use sha2::{Digest as _, Sha256};
use xtask_codegen::{
    GlobalTypesArgs,
    generate_global_types::{
        SourcePin,
        collect::{CollectorOutput, CoverageOutcome, collect},
        source::{CanonicalPath, LibEntries, SourceOptions, acquire, discover, parse_lib_entries},
    },
};

const TEMP_CREATE_RETRIES: usize = 32;

const COLLECTOR_FIXTURE_COUNT: usize = 9;

const SHA256_HEX_LENGTH: usize = 64;

const TEMP_REPO_PREFIX: &str = "bgt";

const FIXTURE_TAG_PREFIX: &str = "v0.0.0";

/// A non-existent commit-like value used to isolate cache-miss tests.
const MISSING_COMMIT_SHA: &str = "0000000000000000000000000000000000000000";

/// Error text expected when offline mode cannot use a populated cache.
const EXPECTED_OFFLINE_CACHE_MISS: &str = "offline";

/// Error text expected when the cached checkout HEAD differs from the pin.
const EXPECTED_HEAD_MISMATCH: &str = "HEAD mismatch";

/// Error text expected when the cached checkout tag ref differs from the pin.
const EXPECTED_TAG_REF_MISMATCH: &str = "tag-ref mismatch";

/// Error text expected when the cached checkout has uncommitted modifications.
const EXPECTED_DIRTY_TREE: &str = "uncommitted modifications";

/// Error text expected when local git index flags would hide worktree changes.
const EXPECTED_INDEX_FLAGS: &str = "index flags";

/// Error text expected when verify mode receives explicit TypeScript overrides.
const EXPECTED_VERIFY_OVERRIDE_REJECTION: &str = "--verify";

/// Error text expected when only one TypeScript override is supplied.
const EXPECTED_SINGLE_OVERRIDE_REJECTION: &str = "--ts-tag and --ts-sha";

/// Error text expected when `libEntries` contains a non-basename declaration filename.
const EXPECTED_INVALID_LIB_ENTRY_FILENAME: &str = "libEntries filename";

/// Error text expected when the real codegen path sees parser diagnostics in pinned sources.
const EXPECTED_PINNED_SOURCE_PARSER_DIAGNOSTIC: &str = "parser diagnostics";

/// Error text expected when a triple-slash path reference resolves to bytes not tracked by git.
const EXPECTED_UNTRACKED_REFERENCE: &str = "not a tracked regular file";

const SINGLE_LIB_ENTRY: &str = "    [\"es5\", \"lib.es5.d.ts\"],\n";

/// libEntries rows that exercise ordered libs and Map-style overwrite semantics.
const DUPLICATE_LIB_ENTRIES: &str = "\
    [\"es5\", \"lib.es5.d.ts\"],
    [\"duplicate\", \"lib.first.d.ts\"],
    [\"duplicate\", \"lib.second.d.ts\"],
";

const INVALID_LIB_ENTRY_FILENAMES: &[&str] = &[
    "",
    "../src/compiler/commandLineParser.ts",
    "lib.es5\\evil.d.ts",
    "/tmp/lib.es5.d.ts",
    "es5.d.ts",
    "lib.es5.ts",
    "lib.es5/../../src/compiler/commandLineParser.d.ts",
];

const EXPECTED_DUPLICATE_LIBS: &[u8] = b"es5\nduplicate\nduplicate\n";

const EXPECTED_DUPLICATE_LIB_MAP: &[u8] = b"duplicate=lib.second.d.ts\nes5=lib.es5.d.ts\n";

/// Error fragment emitted by `xtask_codegen::update` in `Mode::Verify` when content drifts.
const EXPECTED_VERIFY_DRIFT: &str = "is not up-to-date";

/// `libEntries` rows describing a chain `es5 -> shared -> deep` used by the
/// transitive-dependency program-order test.
const TRANSITIVE_LIB_ENTRIES: &str = "\
    [\"es5\", \"lib.es5.d.ts\"],
    [\"shared\", \"lib.shared.d.ts\"],
    [\"deep\", \"lib.deep.d.ts\"],
";

/// Discovered files in TypeScript Program order for the transitive chain.
/// `discover` orders by `default_library_priority`, which buckets non-base libs
/// by their `libs` index, so the produced order matches the `libs` declaration.
const EXPECTED_PROGRAM_ORDER: &[&str] = &[
    "lib/lib.es5.d.ts",
    "lib/lib.shared.d.ts",
    "lib/lib.deep.d.ts",
];

const EXPECTED_SHADOWED_LIB_REFERENCES: &[&str] = &["lib/lib.es5.d.ts", "lib/lib.shared.d.ts"];

/// Profile-root selection for the transitive-dependency program-order test.
/// `discover` walks this list and follows the triple-slash `lib` references.
const TRANSITIVE_PROFILE_ROOTS: &[&str] = &["lib.es5.d.ts"];

const COMMAND_LINE_PARSER_PATH: &str = "src/compiler/commandLineParser.ts";

const COLLECTOR_FIXTURE_DIR: &str = "tests/fixtures/global-types";

const CARRIAGE_RETURN_BYTE: u8 = b'\r';

const LINE_FEED_BYTE: u8 = b'\n';

const COLLECTOR_FIXTURES: [&str; COLLECTOR_FIXTURE_COUNT] = [
    "collector.basic",
    "collector.multiple-var",
    "collector.qualified-namespace",
    "collector.unsupported",
    "collector.declare-global",
    "collector.external-module",
    "collector.empty-external-module",
    "collector.recursive-scope-accounting",
    "collector.export-unsupported",
];

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

const COMMITTED_CODEGEN_PATH: &str = "crates/biome_js_type_info/src/generated/global_types.rs";

struct FixtureRepo {
    temp: TempDir,
    tag: String,
    head: String,
    command_line_parser_sha256: String,
}

impl FixtureRepo {
    fn path(&self) -> &Path {
        self.temp.path()
    }

    fn source_pin(&self) -> SourcePin {
        source_pin(self.tag.as_str(), self.head.as_str())
    }
}

/// Temporary directory that retries name collisions and removes itself on drop.
struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Result<Self> {
        for _ in 0..TEMP_CREATE_RETRIES {
            let name = unique_name(prefix);
            let path = std::env::temp_dir().join(name);
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error).context("failed to create temporary directory"),
            }
        }

        bail!("failed to create unique temporary directory after bounded retries")
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Removes an arbitrary filesystem path when a test exits.
struct PathCleanup {
    path: PathBuf,
}

impl PathCleanup {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for PathCleanup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Builds source acquisition options for tests that do not override the repo URL.
fn source_options(offline: bool, verify: bool) -> SourceOptions {
    SourceOptions {
        offline,
        verify,
        repo_url_override: None,
    }
}

/// Builds a validated source pin from fixture tag/SHA values.
fn source_pin(tag: &str, sha: &str) -> SourcePin {
    SourcePin::new(tag, sha).expect("test TypeScript source pin must be valid")
}

/// Creates a fixture git repository with a minimal TypeScript lib table and one committed file.
fn fixture_git_repo(lib_entries: &str) -> Result<FixtureRepo> {
    let temp = TempDir::new(TEMP_REPO_PREFIX)?;
    run_git(temp.path(), &["init"])?;
    run_git(
        temp.path(),
        &["config", "user.email", "global-types@example.com"],
    )?;
    run_git(temp.path(), &["config", "user.name", "Global Types Tests"])?;
    write_typescript_fixture_files(temp.path(), lib_entries)?;
    run_git(temp.path(), &["add", "."])?;
    run_git(temp.path(), &["commit", "-m", "seed TypeScript fixture"])?;
    let tag = fixture_tag_for_temp_dir(&temp)?;
    run_git(temp.path(), &["tag", tag.as_str()])?;
    let head = git_stdout_trimmed(temp.path(), &["rev-parse", "HEAD"])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(temp.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp,
        tag,
        head,
        command_line_parser_sha256,
    })
}

/// Writes the minimal TypeScript source tree needed by the global-types source codegen.
fn write_typescript_fixture_files(repo: &Path, lib_entries: &str) -> Result<()> {
    fs::create_dir_all(repo.join("src/compiler"))?;
    fs::create_dir_all(repo.join("lib"))?;
    fs::write(
        repo.join(COMMAND_LINE_PARSER_PATH),
        format!("export const libEntries: [string, string][] = [\n{lib_entries}];\n"),
    )?;
    fs::write(repo.join("lib/lib.es5.d.ts"), "interface SeedGlobal {}\n")?;
    fs::write(
        repo.join("lib/lib.first.d.ts"),
        "interface FirstDuplicate {}\n",
    )?;
    fs::write(
        repo.join("lib/lib.second.d.ts"),
        "interface SecondDuplicate {}\n",
    )?;
    Ok(())
}

/// Builds a fixture repo whose `lib.es5.d.ts` references `lib="shared"`, which
/// transitively references `lib="deep"`. Used to lock `discover`'s closure
/// ordering against TypeScript Program order.
fn fixture_git_repo_with_transitive_libs() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(TRANSITIVE_LIB_ENTRIES)?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference lib=\"shared\"/>\ninterface SeedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib/lib.shared.d.ts"),
        "/// <reference lib=\"deep\"/>\ninterface SharedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib/lib.deep.d.ts"),
        "interface DeepGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(
        repo.path(),
        &["commit", "-m", "wire transitive lib references"],
    )?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    // Move the tag so the recorded HEAD still resolves via `tag^{commit}`.
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture where `lib="shared"` resolves inside the default lib directory.
fn fixture_git_repo_with_shadowed_root_lib_reference() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(TRANSITIVE_LIB_ENTRIES)?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference lib=\"shared\"/>\ninterface SeedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib/lib.shared.d.ts"),
        "interface DefaultLibGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib.shared.d.ts"),
        "interface RootEscapeGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(
        repo.path(),
        &["commit", "-m", "shadow lib reference at checkout root"],
    )?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture whose path reference targets an ignored, untracked file.
fn fixture_git_repo_with_ignored_path_reference() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
    fs::write(repo.path().join(".gitignore"), "ignored.d.ts\n")?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference path=\"../ignored.d.ts\"/>\ninterface SeedGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(repo.path(), &["commit", "-m", "reference ignored file"])?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

#[cfg(unix)]
/// Builds a fixture whose path reference crosses an untracked symlinked directory.
fn fixture_git_repo_with_intermediate_symlink_path_reference() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference path=\"../alias/aliased.d.ts\"/>\ninterface SeedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("src/compiler/aliased.d.ts"),
        "interface AliasedGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(
        repo.path(),
        &["commit", "-m", "reference alias path target"],
    )?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture with a Windows-style path separator in a triple-slash reference.
fn fixture_git_repo_with_backslash_path_reference() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
    fs::create_dir_all(repo.path().join("lib/sub"))?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference path=\"sub\\dep.d.ts\"/>\ninterface SeedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib/sub/dep.d.ts"),
        "interface BackslashPathGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(repo.path(), &["commit", "-m", "reference backslash path"])?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture that locks TypeScript's reference attribute precedence.
fn fixture_git_repo_with_mixed_reference_attributes() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(TRANSITIVE_LIB_ENTRIES)?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference path=\"../missing-path.d.ts\" lib=\"shared\"/>\ninterface SeedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib/lib.shared.d.ts"),
        "interface MixedReferenceLibGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(
        repo.path(),
        &["commit", "-m", "mix triple slash reference attributes"],
    )?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture where `no-default-lib` suppresses the same directive's path edge.
fn fixture_git_repo_with_no_default_lib_and_path_reference() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference no-default-lib=\"true\" path=\"../missing-path.d.ts\"/>\ninterface SeedGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(
        repo.path(),
        &["commit", "-m", "mix no-default-lib and path"],
    )?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture for TypeScript's XML pragma shape and case rules.
fn fixture_git_repo_with_typescript_pragma_shape_references() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(TRANSITIVE_LIB_ENTRIES)?;
    fs::create_dir_all(repo.path().join("lib/sub"))?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "\
/// <REFERENCE LIB=\"shared\"/>
/// <referencepath path=\"sub/ignored-referencepath.d.ts\"/>
/// <reference path=\"sub/ignored-open-reference.d.ts\">
interface SeedGlobal {}
",
    )?;
    fs::write(
        repo.path().join("lib/lib.shared.d.ts"),
        "interface SharedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib/sub/ignored-referencepath.d.ts"),
        "interface IgnoredReferencePathGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib/sub/ignored-open-reference.d.ts"),
        "interface IgnoredOpenReferenceGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(
        repo.path(),
        &["commit", "-m", "exercise TypeScript pragma shape"],
    )?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture proving `lib` reference names are matched case-insensitively.
fn fixture_git_repo_with_mixed_case_lib_reference() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(TRANSITIVE_LIB_ENTRIES)?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference lib=\"SHARED\"/>\ninterface SeedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib/lib.shared.d.ts"),
        "interface SharedGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(repo.path(), &["commit", "-m", "reference mixed-case lib"])?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture whose path reference points outside the default lib directory.
fn fixture_git_repo_with_external_path_reference() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
    fs::create_dir_all(repo.path().join("extras"))?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference path=\"../extras/dep.d.ts\"/>\ninterface SeedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("extras/dep.d.ts"),
        "interface CommittedBlobGlobal {}\n",
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(repo.path(), &["commit", "-m", "reference external dep"])?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

#[cfg(unix)]
/// Builds a fixture where a default-lib reference resolves to a symlink escape.
fn fixture_git_repo_with_symlinked_default_lib_reference() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(TRANSITIVE_LIB_ENTRIES)?;
    fs::write(
        repo.path().join("lib/lib.es5.d.ts"),
        "/// <reference lib=\"shared\"/>\ninterface SeedGlobal {}\n",
    )?;
    fs::write(
        repo.path().join("lib.shared.d.ts"),
        "interface RootSymlinkEscapeGlobal {}\n",
    )?;
    std::os::unix::fs::symlink(
        "../lib.shared.d.ts",
        repo.path().join("lib/lib.shared.d.ts"),
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(
        repo.path(),
        &["commit", "-m", "symlink lib reference outside lib"],
    )?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

#[cfg(unix)]
/// Builds a fixture whose selected profile root is itself a symlink.
fn fixture_git_repo_with_symlinked_profile_root() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
    fs::remove_file(repo.path().join("lib/lib.es5.d.ts"))?;
    std::os::unix::fs::symlink(
        "../src/compiler/commandLineParser.ts",
        repo.path().join("lib/lib.es5.d.ts"),
    )?;
    run_git(repo.path(), &["add", "."])?;
    run_git(repo.path(), &["commit", "-m", "symlink profile root"])?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture with all profile roots present but one malformed source file.
fn fixture_git_repo_with_malformed_lib() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
    for filename in xtask_codegen::generate_global_types::source::PROFILE_ROOTS {
        fs::write(
            repo.path().join("lib").join(filename),
            format!("interface {} {{}}\n", filename.replace(['.', '-'], "_")),
        )?;
    }
    fs::write(repo.path().join("lib/lib.es5.d.ts"), "interface Broken {\n")?;
    run_git(repo.path(), &["add", "."])?;
    run_git(repo.path(), &["commit", "-m", "make lib malformed"])?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Builds a fixture repository containing every production profile root.
fn fixture_git_repo_with_all_profile_roots() -> Result<FixtureRepo> {
    let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
    for filename in xtask_codegen::generate_global_types::source::PROFILE_ROOTS {
        fs::write(
            repo.path().join("lib").join(filename),
            format!("interface {} {{}}\n", filename.replace(['.', '-'], "_")),
        )?;
    }
    run_git(repo.path(), &["add", "."])?;
    run_git(repo.path(), &["commit", "-m", "add profile roots"])?;
    let head = git_stdout_trimmed(repo.path(), &["rev-parse", "HEAD"])?;
    run_git(repo.path(), &["tag", "-f", repo.tag.as_str()])?;
    let command_line_parser_sha256 =
        sha256_hex(&fs::read(repo.path().join(COMMAND_LINE_PARSER_PATH))?);

    Ok(FixtureRepo {
        temp: repo.temp,
        tag: repo.tag,
        head,
        command_line_parser_sha256,
    })
}

/// Appends one committed file and returns the new HEAD SHA.
fn append_commit(repo: &Path, relative_file: &str) -> Result<String> {
    fs::write(repo.join(relative_file), "interface AfterSeed {}\n")?;
    run_git(repo, &["add", "."])?;
    run_git(repo, &["commit", "-m", "append fixture commit"])?;
    git_stdout_trimmed(repo, &["rev-parse", "HEAD"])
}

/// Seeds the TypeScript cache from a fixture repository using a real git clone.
fn seed_cache_from_repo(pin: &SourcePin, repo: &Path) -> Result<PathCleanup> {
    let cache_path = typescript_cache_path(pin);
    if cache_path.exists() {
        fs::remove_dir_all(&cache_path)?;
    }
    let Some(cache_parent) = cache_path.parent() else {
        bail!("cache path has no parent: {}", cache_path.display());
    };
    fs::create_dir_all(cache_parent)?;
    let repo_arg = path_to_str(repo)?;
    let cache_arg = path_to_str(&cache_path)?;
    run_git(
        &xtask_glue::project_root(),
        &["clone", "--no-hardlinks", repo_arg, cache_arg],
    )?;
    Ok(PathCleanup::new(cache_path))
}

/// Clears the cache directory for `pin` and returns a cleanup guard.
fn clean_cache_path(pin: &SourcePin) -> Result<PathCleanup> {
    let cache_path = typescript_cache_path(pin);
    if cache_path.exists() {
        fs::remove_dir_all(&cache_path)?;
    }
    Ok(PathCleanup::new(cache_path))
}

/// Returns the production cache path for a TypeScript source pin.
fn typescript_cache_path(pin: &SourcePin) -> PathBuf {
    xtask_glue::project_root()
        .join("target/xtask/typescript")
        .join(format!("{}-{}", pin.tag(), pin.sha()))
}

/// Returns the acquire-lock path that production code derives for `pin`.
fn acquire_lock_path_for_test(pin: &SourcePin) -> PathBuf {
    let cache_path = typescript_cache_path(pin);
    let cache_parent = cache_path
        .parent()
        .expect("cache path should have a parent");
    cache_parent.join(format!(".lock-{}-{}", pin.tag(), pin.sha()))
}

/// Returns a production-shaped temporary checkout path for stale-cleanup tests.
fn temporary_checkout_path_for_test(pin: &SourcePin, filename: &str) -> PathBuf {
    let cache_path = typescript_cache_path(pin);
    let cache_parent = cache_path
        .parent()
        .expect("cache path should have a parent");
    cache_parent
        .join(".tmp")
        .join(temporary_checkout_namespace_for_test(pin))
        .join(filename)
}

/// Mirrors production's hashed namespace for temporary checkout directories.
fn temporary_checkout_namespace_for_test(pin: &SourcePin) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pin.tag().as_bytes());
    hasher.update([0]);
    hasher.update(pin.sha().as_bytes());
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(SHA256_HEX_LENGTH);
    for byte in digest {
        write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }
    hex
}

/// Runs the global-types command with test-controlled pin overrides.
fn run_global_types(
    verify: bool,
    ts_tag: Option<&str>,
    ts_sha: Option<&str>,
) -> xtask_glue::Result<()> {
    xtask_codegen::generate_global_types::run(
        GlobalTypesArgs {
            verify,
            ts_tag: ts_tag.map(str::to_owned),
            ts_sha: ts_sha.map(str::to_owned),
            offline: true,
        },
        if verify {
            xtask_glue::Mode::Verify
        } else {
            xtask_glue::Mode::Overwrite
        },
    )
}

/// Runs git in `cwd` and returns stdout bytes on success.
fn run_git(cwd: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git").args(args).current_dir(cwd).output()?;
    ensure_success(output, args)
}

/// Runs git and trims UTF-8 stdout for commands that return a single value.
fn git_stdout_trimmed(cwd: &Path, args: &[&str]) -> Result<String> {
    let stdout = run_git(cwd, args)?;
    let text = String::from_utf8(stdout).context("git stdout was not UTF-8")?;
    Ok(text.trim().to_owned())
}

/// Converts a process output into stdout or an error that includes stderr.
fn ensure_success(output: Output, args: &[&str]) -> Result<Vec<u8>> {
    if output.status.success() {
        return Ok(output.stdout);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    bail!("git {:?} failed: {stderr}", args)
}

/// Builds a unique name from a stable prefix, hexadecimal process id, and atomic counter.
///
/// No timestamp is included so temporary paths stay shorter on Windows.
fn unique_name(prefix: &str) -> String {
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}-{:x}-{counter:x}", std::process::id())
}

/// Builds a valid local git tag name from a temporary directory name.
fn fixture_tag_for_temp_dir(temp: &TempDir) -> Result<String> {
    let Some(name) = temp.path().file_name().and_then(|name| name.to_str()) else {
        bail!("temporary directory path has no UTF-8 final component");
    };
    Ok(format!("{FIXTURE_TAG_PREFIX}-{name}"))
}

/// Converts a path to UTF-8 for git arguments used by fixture tests.
fn path_to_str(path: &Path) -> Result<&str> {
    let Some(value) = path.to_str() else {
        bail!("path is not UTF-8: {}", path.display());
    };
    Ok(value)
}

/// Asserts that a fallible operation failed with the expected top-level message.
fn expect_error_contains<T>(result: Result<T>, expected: &str) -> Result<()> {
    let Err(error) = result else {
        bail!("expected an error but got success");
    };
    let message = error.to_string();

    assert!(
        message.contains(expected),
        "expected error {message:?} to contain {expected:?}"
    );

    Ok(())
}

/// Asserts that every expected fragment appears somewhere in an error chain.
fn expect_error_chain_contains_all<T>(result: Result<T>, expected: &[&str]) -> Result<()> {
    let Err(error) = result else {
        bail!("expected an error but got success");
    };
    let messages: Vec<String> = error.chain().map(ToString::to_string).collect();

    for expected in expected {
        assert!(
            messages.iter().any(|message| message.contains(expected)),
            "expected error chain {messages:?} to contain {expected:?}"
        );
    }

    Ok(())
}

/// Serializes ordered lib names to a byte string for compact assertions.
fn libs_bytes(entries: &LibEntries) -> Vec<u8> {
    let mut bytes = Vec::new();
    for lib in &entries.libs {
        bytes.extend_from_slice(lib.as_bytes());
        bytes.push(LINE_FEED_BYTE);
    }
    bytes
}

/// Serializes the lib map to sorted `name=file` lines for compact assertions.
fn lib_map_bytes(map: &BTreeMap<biome_rowan::Text, biome_rowan::Text>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for (name, file) in map {
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(b'=');
        bytes.extend_from_slice(file.as_bytes());
        bytes.push(LINE_FEED_BYTE);
    }
    bytes
}

/// Normalizes CRLF bytes to LF without allocating when no carriage returns are present.
fn normalize_lf(bytes: Vec<u8>) -> Vec<u8> {
    if !bytes.contains(&CARRIAGE_RETURN_BYTE) {
        return bytes;
    }

    let mut normalized = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == CARRIAGE_RETURN_BYTE {
            let next_index = index + 1;
            if next_index < bytes.len() && bytes[next_index] == LINE_FEED_BYTE {
                index = next_index;
                continue;
            }
        }
        normalized.push(byte);
        index += 1;
    }
    normalized
}

/// Computes lowercase SHA-256 hex for fixture files.
fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(SHA256_HEX_LENGTH);
    for byte in digest {
        write!(&mut hex, "{byte:02x}").expect("writing to String cannot fail");
    }
    hex
}

/// Formats collector output using stable `Debug` sections for fixture files.
fn stringify_collector_output(output: &CollectorOutput) -> String {
    let mut text = String::new();
    text.push_str("records\n");
    for record in &output.records {
        writeln!(text, "{record:?}").expect("writing to String cannot fail");
    }
    text.push_str("coverage\n");
    for outcome in &output.coverage {
        writeln!(text, "{outcome:?}").expect("writing to String cannot fail");
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_codegen_module_has_expected_shape() -> Result<()> {
        let path = xtask_glue::project_root().join(COMMITTED_CODEGEN_PATH);
        let content = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;

        for needle in [
            "//! Generated by `just gen-global-types`. Do not edit.",
            "pub(crate) const GENERATED_TYPESCRIPT_TAG: &str =",
            "pub(crate) const GENERATED_TYPESCRIPT_SHA: &str =",
            "pub(crate) const COMMAND_LINE_PARSER_SHA256: &str =",
            "pub(crate) const GENERATED_SOURCE_FILES: &[(&str, &str)] = &[",
            "pub(crate) const GENERATED_COLLECTED_DECLARATION_COUNT: usize =",
            "pub(crate) const GENERATED_COVERAGE_OUTCOME_COUNT: usize =",
            "pub(crate) const GENERATED_COLLECTOR_OUTPUT_SHA256: &str =",
            "pub(crate) const GENERATED_GLOBAL_TYPE_COUNT: usize =",
            "pub(crate) const MIGRATED_PREDEFINED_IDS: &[crate::globals::GlobalTypeId] =",
            "pub(crate) fn set_generated_global_type_data(",
        ] {
            assert!(
                content.contains(needle),
                "committed codegen module missing expected fragment: {needle}"
            );
        }

        Ok(())
    }

    #[test]
    fn offline_cache_miss_fails() -> Result<()> {
        let tag = format!("v0.0.0-{}", unique_name("missing"));
        let pin = source_pin(tag.as_str(), MISSING_COMMIT_SHA);
        let _cache_cleanup = clean_cache_path(&pin)?;
        let options = source_options(true, false);

        expect_error_contains(acquire(&pin, &options), EXPECTED_OFFLINE_CACHE_MISS)
    }

    #[test]
    fn source_pin_validates_tag_and_sha() {
        assert!(SourcePin::new("v1.2.3", MISSING_COMMIT_SHA).is_ok());
        assert!(SourcePin::new("v1.2.3-beta.1", MISSING_COMMIT_SHA).is_ok());
        assert!(SourcePin::new("1.2.3", MISSING_COMMIT_SHA).is_err());
        assert!(SourcePin::new("v1.2", MISSING_COMMIT_SHA).is_err());
        assert!(SourcePin::new("v1.2.3", "ABCDEF0000000000000000000000000000000000").is_err());
    }

    #[test]
    fn seeded_offline_cache_succeeds() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);

        let checkout = acquire(&pin, &options)?;

        assert_eq!(checkout.pin().tag().as_bytes(), repo.tag.as_bytes());
        assert_eq!(checkout.pin().sha().as_bytes(), repo.head.as_bytes());
        assert_eq!(
            checkout.command_line_parser_sha256().as_bytes(),
            repo.command_line_parser_sha256.as_bytes()
        );

        Ok(())
    }

    #[test]
    fn cache_hit_ignores_stale_acquire_lock() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let cache_path = typescript_cache_path(&pin);
        let cache_parent = cache_path
            .parent()
            .context("cache path should have a parent")?;
        let lock_path = cache_parent.join(format!(".lock-{}-{}", pin.tag(), pin.sha()));
        fs::write(&lock_path, "stale-pid\n")?;
        let options = source_options(true, false);

        let checkout = acquire(&pin, &options)?;

        assert_eq!(checkout.pin().sha().as_bytes(), pin.sha().as_bytes());
        assert!(
            !lock_path.exists(),
            "cache hits should remove stranded acquire locks"
        );

        Ok(())
    }

    #[test]
    fn cache_miss_recovers_stale_acquire_lock() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = repo.source_pin();
        let _cache_cleanup = clean_cache_path(&pin)?;
        let cache_path = typescript_cache_path(&pin);
        let cache_parent = cache_path
            .parent()
            .context("cache path should have a parent")?;
        fs::create_dir_all(cache_parent)?;
        let lock_path = acquire_lock_path_for_test(&pin);
        fs::create_dir(&lock_path)?;
        fs::write(lock_path.join("owner"), "99999999:stale\n")?;
        let options = SourceOptions {
            offline: false,
            verify: false,
            repo_url_override: Some(repo.path().to_path_buf()),
        };

        let checkout = acquire(&pin, &options)?;

        assert_eq!(checkout.pin().sha().as_bytes(), pin.sha().as_bytes());
        assert!(
            !lock_path.exists(),
            "cache miss should recover stale acquire lock directories"
        );

        Ok(())
    }

    #[test]
    fn cache_hit_head_mismatch_fails() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pinned_head = repo.head.clone();
        append_commit(repo.path(), "lib/lib.after-head-mismatch.d.ts")?;
        let pin = source_pin(repo.tag.as_str(), pinned_head.as_str());
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);

        expect_error_contains(acquire(&pin, &options), EXPECTED_HEAD_MISMATCH)
    }

    #[test]
    fn tag_commit_mismatch_fails() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let second_head = append_commit(repo.path(), "lib/lib.after-tag-mismatch.d.ts")?;
        let pin = source_pin(repo.tag.as_str(), second_head.as_str());
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);

        expect_error_contains(acquire(&pin, &options), EXPECTED_TAG_REF_MISMATCH)
    }

    #[test]
    fn cache_hit_dirty_tree_fails() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = source_pin(repo.tag.as_str(), repo.head.as_str());
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let cache_path = typescript_cache_path(&pin);
        fs::write(
            cache_path.join("lib/lib.es5.d.ts"),
            "interface DirtyMutation {}\n",
        )?;
        let options = source_options(true, false);

        expect_error_contains(acquire(&pin, &options), EXPECTED_DIRTY_TREE)
    }

    #[test]
    fn cache_hit_index_flags_do_not_hide_dirty_tree() -> Result<()> {
        for (set_flag, clear_flag) in [
            ("--assume-unchanged", "--no-assume-unchanged"),
            ("--skip-worktree", "--no-skip-worktree"),
        ] {
            let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
            let pin = source_pin(repo.tag.as_str(), repo.head.as_str());
            let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
            let cache_path = typescript_cache_path(&pin);
            run_git(&cache_path, &["update-index", set_flag, "lib/lib.es5.d.ts"])?;
            fs::write(
                cache_path.join("lib/lib.es5.d.ts"),
                "interface HiddenDirtyMutation {}\n",
            )?;
            let options = source_options(true, false);

            expect_error_contains(acquire(&pin, &options), EXPECTED_INDEX_FLAGS)
                .with_context(|| format!("{set_flag} must not hide dirty source bytes"))?;

            let _ = run_git(
                &cache_path,
                &["update-index", clear_flag, "lib/lib.es5.d.ts"],
            );
        }

        Ok(())
    }

    #[test]
    fn verify_rejects_single_overrides() -> Result<()> {
        expect_error_contains(
            run_global_types(true, Some("v0.0.0"), None),
            EXPECTED_VERIFY_OVERRIDE_REJECTION,
        )?;
        expect_error_contains(
            run_global_types(true, None, Some(MISSING_COMMIT_SHA)),
            EXPECTED_VERIFY_OVERRIDE_REJECTION,
        )?;
        expect_error_contains(
            run_global_types(false, Some("v0.0.0"), None),
            EXPECTED_SINGLE_OVERRIDE_REJECTION,
        )?;
        expect_error_contains(
            run_global_types(false, None, Some(MISSING_COMMIT_SHA)),
            EXPECTED_SINGLE_OVERRIDE_REJECTION,
        )
    }

    #[test]
    fn run_rejects_invalid_source_pin_overrides() -> Result<()> {
        expect_error_contains(
            run_global_types(false, Some("v0.0.0\n"), Some(MISSING_COMMIT_SHA)),
            "invalid TypeScript source tag",
        )?;
        expect_error_contains(
            run_global_types(
                false,
                Some("v0.0.0"),
                Some("ABCDEF0000000000000000000000000000000000"),
            ),
            "invalid TypeScript git commit SHA",
        )
    }

    #[test]
    fn duplicate_lib_entries_preserve_order_and_last_write_wins() -> Result<()> {
        let repo = fixture_git_repo(DUPLICATE_LIB_ENTRIES)?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;

        let entries = parse_lib_entries(&checkout)?;

        assert_eq!(libs_bytes(&entries), EXPECTED_DUPLICATE_LIBS);
        assert_eq!(lib_map_bytes(&entries.lib_map), EXPECTED_DUPLICATE_LIB_MAP);

        Ok(())
    }

    #[test]
    fn lib_entries_reject_invalid_filenames() -> Result<()> {
        for filename in INVALID_LIB_ENTRY_FILENAMES {
            let lib_entries = format!("    [\"es5\", {filename:?}],\n");
            let repo = fixture_git_repo(&lib_entries)?;
            let pin = repo.source_pin();
            let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
            let options = source_options(true, false);
            let checkout = acquire(&pin, &options)?;

            expect_error_contains(
                parse_lib_entries(&checkout),
                EXPECTED_INVALID_LIB_ENTRY_FILENAME,
            )
            .with_context(|| format!("filename {filename:?} should be rejected"))?;
        }

        Ok(())
    }

    #[test]
    fn failed_clone_validation_does_not_poison_cache() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let second_head = append_commit(repo.path(), "lib/lib.after-clone-validation.d.ts")?;
        let pin = source_pin(repo.tag.as_str(), second_head.as_str());
        let _cache_cleanup = clean_cache_path(&pin)?;
        let options = SourceOptions {
            offline: false,
            verify: false,
            repo_url_override: Some(repo.path().to_path_buf()),
        };

        expect_error_contains(acquire(&pin, &options), EXPECTED_HEAD_MISMATCH)?;
        assert!(
            !typescript_cache_path(&pin).exists(),
            "failed temp validation must not leave a poisoned cache directory behind"
        );

        Ok(())
    }

    #[test]
    fn repo_url_override_accepts_dash_prefixed_relative_paths() -> Result<()> {
        let relative_repo = PathBuf::from(unique_name("-bgt"));
        let repo_path = std::env::current_dir()?.join(&relative_repo);
        if repo_path.exists() {
            bail!(
                "temporary repository path already exists: {}",
                repo_path.display()
            );
        }
        fs::create_dir(&repo_path)?;
        let _repo_cleanup = PathCleanup::new(repo_path.clone());
        run_git(&repo_path, &["init"])?;
        run_git(
            &repo_path,
            &["config", "user.email", "global-types@example.com"],
        )?;
        run_git(&repo_path, &["config", "user.name", "Global Types Tests"])?;
        write_typescript_fixture_files(&repo_path, SINGLE_LIB_ENTRY)?;
        run_git(&repo_path, &["add", "."])?;
        run_git(&repo_path, &["commit", "-m", "seed dash-prefixed repo"])?;
        let relative_repo_name = path_to_str(&relative_repo)?;
        let tag = format!("{FIXTURE_TAG_PREFIX}-{relative_repo_name}");
        run_git(&repo_path, &["tag", tag.as_str()])?;
        let head = git_stdout_trimmed(&repo_path, &["rev-parse", "HEAD"])?;
        let pin = source_pin(tag.as_str(), head.as_str());
        let _cache_cleanup = clean_cache_path(&pin)?;
        let options = SourceOptions {
            offline: false,
            verify: false,
            repo_url_override: Some(relative_repo),
        };

        let checkout = acquire(&pin, &options)?;

        assert_eq!(checkout.pin().sha(), pin.sha());

        Ok(())
    }

    #[test]
    fn acquire_removes_stale_temporary_checkouts_for_pin() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = repo.source_pin();
        let _cache_cleanup = clean_cache_path(&pin)?;
        let cache_path = typescript_cache_path(&pin);
        let cache_parent = cache_path
            .parent()
            .context("cache path should have a parent")?;
        fs::create_dir_all(cache_parent)?;
        let stale_temp = temporary_checkout_path_for_test(&pin, "stale.0");
        fs::create_dir_all(&stale_temp)?;
        let options = SourceOptions {
            offline: false,
            verify: false,
            repo_url_override: Some(repo.path().to_path_buf()),
        };

        let _checkout = acquire(&pin, &options)?;

        assert!(
            !stale_temp.exists(),
            "acquire should remove stale temp checkout dirs for the same pin"
        );

        Ok(())
    }

    #[test]
    fn acquire_keeps_temporary_checkouts_for_other_pins() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = repo.source_pin();
        let _cache_cleanup = clean_cache_path(&pin)?;
        let foreign_tag = format!("{}-{}.extra", pin.tag(), pin.sha());
        let foreign_pin = source_pin(&foreign_tag, MISSING_COMMIT_SHA);
        let foreign_temp = temporary_checkout_path_for_test(&foreign_pin, "stale.0");
        fs::create_dir_all(&foreign_temp)?;
        let options = SourceOptions {
            offline: false,
            verify: false,
            repo_url_override: Some(repo.path().to_path_buf()),
        };

        let _checkout = acquire(&pin, &options)?;

        assert!(
            foreign_temp.exists(),
            "acquire must not remove temporary checkout dirs owned by another pin"
        );

        Ok(())
    }

    #[test]
    fn cache_hit_keeps_temporary_checkouts_for_other_pins() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let foreign_tag = format!("{}-{}.extra", pin.tag(), pin.sha());
        let foreign_pin = source_pin(&foreign_tag, MISSING_COMMIT_SHA);
        let foreign_temp = temporary_checkout_path_for_test(&foreign_pin, "stale.1");
        fs::create_dir_all(&foreign_temp)?;
        let options = source_options(true, false);

        let _checkout = acquire(&pin, &options)?;

        assert!(
            foreign_temp.exists(),
            "cache hits must not remove temporary checkout dirs owned by another pin"
        );

        Ok(())
    }

    #[test]
    fn acquire_removes_stale_temporary_checkouts_on_cache_hit() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let stale_temp = temporary_checkout_path_for_test(&pin, "stale.1");
        fs::create_dir_all(&stale_temp)?;
        let options = source_options(true, false);

        let _checkout = acquire(&pin, &options)?;

        assert!(
            !stale_temp.exists(),
            "cache hits should also remove stale temp checkout dirs for the same pin"
        );

        Ok(())
    }

    #[test]
    fn path_references_reject_ignored_untracked_files() -> Result<()> {
        let repo = fixture_git_repo_with_ignored_path_reference()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let cache_path = typescript_cache_path(&pin);
        fs::write(
            cache_path.join("ignored.d.ts"),
            "interface IgnoredGlobal {}\n",
        )?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        expect_error_chain_contains_all(
            discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS),
            &[EXPECTED_UNTRACKED_REFERENCE, "ignored.d.ts"],
        )
    }

    #[cfg(unix)]
    #[test]
    fn path_references_reject_untracked_intermediate_symlink_dirs() -> Result<()> {
        let repo = fixture_git_repo_with_intermediate_symlink_path_reference()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let cache_path = typescript_cache_path(&pin);
        std::os::unix::fs::symlink("src/compiler", cache_path.join("alias"))?;
        let libs = parse_lib_entries(&checkout)?;

        expect_error_chain_contains_all(
            discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS),
            &["symlink", "alias"],
        )
    }

    #[test]
    fn path_references_normalize_backslashes_like_typescript() -> Result<()> {
        let repo = fixture_git_repo_with_backslash_path_reference()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        let discovered = discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS)?;
        let actual: Vec<&str> = discovered
            .iter()
            .map(|file| file.repo_relative.as_str())
            .collect();

        assert!(
            actual.contains(&"lib/sub/dep.d.ts"),
            "TypeScript-style slash normalization should include lib/sub/dep.d.ts, got {actual:?}"
        );

        Ok(())
    }

    #[test]
    fn triple_slash_reference_precedence_matches_typescript() -> Result<()> {
        let repo = fixture_git_repo_with_mixed_reference_attributes()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        let discovered = discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS)?;
        let actual: Vec<&str> = discovered
            .iter()
            .map(|file| file.repo_relative.as_str())
            .collect();

        assert_eq!(
            actual,
            ["lib/lib.es5.d.ts", "lib/lib.shared.d.ts"],
            "lib references should take precedence over path references in mixed directives"
        );

        Ok(())
    }

    #[test]
    fn no_default_lib_precedence_ignores_path_references() -> Result<()> {
        let repo = fixture_git_repo_with_no_default_lib_and_path_reference()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        let discovered = discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS)?;
        let actual: Vec<&str> = discovered
            .iter()
            .map(|file| file.repo_relative.as_str())
            .collect();

        assert_eq!(
            actual,
            ["lib/lib.es5.d.ts"],
            "no-default-lib directives should not also process path references"
        );

        Ok(())
    }

    #[test]
    fn triple_slash_xml_shape_and_case_match_typescript() -> Result<()> {
        let repo = fixture_git_repo_with_typescript_pragma_shape_references()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        let discovered = discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS)?;
        let actual: Vec<&str> = discovered
            .iter()
            .map(|file| file.repo_relative.as_str())
            .collect();

        assert_eq!(
            actual,
            ["lib/lib.es5.d.ts", "lib/lib.shared.d.ts"],
            "TypeScript accepts uppercase XML pragma names but ignores malformed reference pragmas"
        );

        Ok(())
    }

    #[test]
    fn lib_references_lowercase_names_like_typescript() -> Result<()> {
        let repo = fixture_git_repo_with_mixed_case_lib_reference()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        let discovered = discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS)?;
        let actual: Vec<&str> = discovered
            .iter()
            .map(|file| file.repo_relative.as_str())
            .collect();

        assert_eq!(
            actual,
            ["lib/lib.es5.d.ts", "lib/lib.shared.d.ts"],
            "TypeScript lowercases lib reference names before libMap lookup"
        );

        Ok(())
    }

    #[test]
    fn discovery_reads_pinned_git_blobs_after_cache_worktree_mutation() -> Result<()> {
        let repo = fixture_git_repo_with_external_path_reference()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let cache_path = typescript_cache_path(&pin);
        run_git(
            &cache_path,
            &["update-index", "--skip-worktree", "extras/dep.d.ts"],
        )?;
        fs::write(
            cache_path.join("extras/dep.d.ts"),
            "interface DirtyWorktreeGlobal {}\n",
        )?;
        let libs = parse_lib_entries(&checkout)?;

        let discovered = discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS)?;
        let dep = discovered
            .iter()
            .find(|file| file.repo_relative == "extras/dep.d.ts")
            .context("expected extras/dep.d.ts to be discovered")?;

        assert!(
            dep.bytes_lf
                .windows(b"CommittedBlobGlobal".len())
                .any(|window| window == b"CommittedBlobGlobal"),
            "discovery must use committed blob bytes"
        );
        assert!(
            !dep.bytes_lf
                .windows(b"DirtyWorktreeGlobal".len())
                .any(|window| window == b"DirtyWorktreeGlobal"),
            "worktree mutations after acquire must not affect generated source bytes"
        );

        Ok(())
    }

    #[test]
    fn command_line_parser_reads_pinned_git_blob_after_worktree_mutation() -> Result<()> {
        let repo = fixture_git_repo(SINGLE_LIB_ENTRY)?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        fs::write(
            typescript_cache_path(&pin).join(COMMAND_LINE_PARSER_PATH),
            "export const libEntries = [broken worktree bytes];\n",
        )?;

        let entries = parse_lib_entries(&checkout)?;

        assert_eq!(libs_bytes(&entries), b"es5\n");

        Ok(())
    }

    #[test]
    fn run_fails_on_pinned_source_parser_diagnostics() -> Result<()> {
        let repo = fixture_git_repo_with_malformed_lib()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;

        expect_error_contains(
            xtask_codegen::generate_global_types::run(
                GlobalTypesArgs {
                    verify: false,
                    ts_tag: Some(repo.tag.clone()),
                    ts_sha: Some(repo.head.clone()),
                    offline: true,
                },
                xtask_glue::Mode::Verify,
            ),
            EXPECTED_PINNED_SOURCE_PARSER_DIAGNOSTIC,
        )
    }

    #[test]
    fn collector_fixture_round_trip() -> Result<()> {
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR")).join(COLLECTOR_FIXTURE_DIR);
        assert!(
            fixture_root.exists(),
            "fixture root missing: {}",
            fixture_root.display()
        );

        for fixture in COLLECTOR_FIXTURES {
            let dts_name = format!("{fixture}.d.ts");
            let expected_name = format!("{fixture}.expected.txt");
            let dts_path = fixture_root.join(&dts_name);
            let expected_path = fixture_root.join(expected_name);

            let bytes =
                normalize_lf(fs::read(&dts_path).with_context(|| {
                    format!("failed to read collector fixture {}", dts_path.display())
                })?);
            let expected = fs::read(&expected_path).with_context(|| {
                format!(
                    "failed to read collector fixture expectation {}",
                    expected_path.display()
                )
            })?;
            let relative_path = path_to_str(Path::new(&dts_name))?;
            let canonical_path = CanonicalPath::from_within(&fixture_root, relative_path)?;
            let discovered = xtask_codegen::generate_global_types::source::DiscoveredFile {
                path: canonical_path,
                repo_relative: dts_name,
                sha256_hex: sha256_hex(&bytes),
                bytes_lf: bytes,
            };

            let output = collect(&discovered);
            let coverage_collected = output
                .coverage
                .iter()
                .filter(|outcome| matches!(outcome, CoverageOutcome::Collected(_)))
                .count();
            assert_eq!(
                output.records.len(),
                coverage_collected,
                "fixture {fixture}: records ({}) != coverage Collected count ({})",
                output.records.len(),
                coverage_collected
            );

            let actual = stringify_collector_output(&output);

            if std::env::var_os("BIOME_GLOBAL_TYPES_UPDATE_FIXTURES").is_some() {
                fs::write(&expected_path, actual.as_bytes())?;
            } else {
                assert_eq!(
                    actual.as_bytes(),
                    expected.as_slice(),
                    "collector fixture {fixture} round-trip mismatch"
                );
            }
        }

        Ok(())
    }

    #[test]
    fn discovers_transitive_dependencies_in_program_order() -> Result<()> {
        let repo = fixture_git_repo_with_transitive_libs()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        let discovered = discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS)?;
        let actual: Vec<&str> = discovered
            .iter()
            .map(|file| file.repo_relative.as_str())
            .collect();

        assert_eq!(
            actual, EXPECTED_PROGRAM_ORDER,
            "discovery order must match TypeScript Program order"
        );

        Ok(())
    }

    #[test]
    fn lib_references_resolve_inside_default_lib_directory() -> Result<()> {
        let repo = fixture_git_repo_with_shadowed_root_lib_reference()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        let discovered = discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS)?;
        let actual: Vec<&str> = discovered
            .iter()
            .map(|file| file.repo_relative.as_str())
            .collect();

        assert_eq!(
            actual, EXPECTED_SHADOWED_LIB_REFERENCES,
            "lib references must resolve relative to the default lib directory, not checkout root"
        );
        assert!(
            discovered.iter().any(|file| file
                .bytes_lf
                .windows(b"DefaultLibGlobal".len())
                .any(|window| window == b"DefaultLibGlobal")),
            "discovery should read the default lib directory file"
        );
        assert!(
            discovered.iter().all(|file| !file
                .bytes_lf
                .windows(b"RootEscapeGlobal".len())
                .any(|window| window == b"RootEscapeGlobal")),
            "discovery must not read checkout-root shadow files for lib references"
        );

        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn lib_references_reject_symlink_escape_from_default_lib_directory() -> Result<()> {
        let repo = fixture_git_repo_with_symlinked_default_lib_reference()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        expect_error_chain_contains_all(
            discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS),
            &["symlink"],
        )
    }

    #[cfg(unix)]
    #[test]
    fn profile_roots_reject_symlinked_source_files() -> Result<()> {
        let repo = fixture_git_repo_with_symlinked_profile_root()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let options = source_options(true, false);
        let checkout = acquire(&pin, &options)?;
        let libs = parse_lib_entries(&checkout)?;

        expect_error_chain_contains_all(
            discover(&checkout, &libs, TRANSITIVE_PROFILE_ROOTS),
            &["symlink"],
        )
    }

    #[test]
    fn verify_detects_committed_codegen_drift() -> Result<()> {
        let repo = fixture_git_repo_with_all_profile_roots()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let temp = TempDir::new("verify-drift")?;
        let path = temp.path().join(COMMITTED_CODEGEN_PATH);
        fs::create_dir_all(path.parent().expect("temp path has parent"))?;
        fs::write(
            &path,
            "deliberate drift marker not matching template output",
        )?;

        let Err(error) = xtask_codegen::generate_global_types::run_with_workspace_root(
            GlobalTypesArgs {
                verify: false,
                ts_tag: Some(repo.tag.clone()),
                ts_sha: Some(repo.head.clone()),
                offline: true,
            },
            xtask_glue::Mode::Verify,
            temp.path(),
        ) else {
            bail!("expected global-types verify to fail after drift was introduced");
        };
        let message = error.to_string();
        assert!(
            message.contains(EXPECTED_VERIFY_DRIFT),
            "verify-mode error {message:?} must contain {EXPECTED_VERIFY_DRIFT:?}"
        );
        let temp_prefix = temp.path().to_string_lossy();
        assert!(
            message.contains(&*temp_prefix),
            "verify-mode error {message:?} must point at the temp workspace root {temp_prefix:?}, \
             not the real repo root"
        );

        Ok(())
    }

    #[test]
    fn verify_passes_when_committed_codegen_matches() -> Result<()> {
        let repo = fixture_git_repo_with_all_profile_roots()?;
        let pin = repo.source_pin();
        let _cache_cleanup = seed_cache_from_repo(&pin, repo.path())?;
        let temp = TempDir::new("verify-match")?;

        xtask_codegen::generate_global_types::run_with_workspace_root(
            GlobalTypesArgs {
                verify: false,
                ts_tag: Some(repo.tag.clone()),
                ts_sha: Some(repo.head.clone()),
                offline: true,
            },
            xtask_glue::Mode::Overwrite,
            temp.path(),
        )?;

        xtask_codegen::generate_global_types::run_with_workspace_root(
            GlobalTypesArgs {
                verify: false,
                ts_tag: Some(repo.tag.clone()),
                ts_sha: Some(repo.head.clone()),
                offline: true,
            },
            xtask_glue::Mode::Verify,
            temp.path(),
        )?;

        Ok(())
    }

}
