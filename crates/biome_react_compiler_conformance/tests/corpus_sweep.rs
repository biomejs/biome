//! Full-corpus differential sweep (DEV ONLY, `#[ignore]`d).
//!
//! Walks the React Compiler fixture corpus from the local `react/` checkout and
//! runs every in-scope fixture through both the Biome frontend and the OXC
//! oracle, classifying each into:
//!   - **Match** — identical diagnostics.
//!   - **Tier-1 gap** — Biome's converter could not produce an AST
//!     (`UnsupportedSyntax`/`MissingSyntax`/`InvalidLiteral`) or panicked.
//!   - **Tier-2 divergence** — both compiled but diagnostics differ.
//!   - **Skipped** — `@flow` pragma or Biome parse error (out of scope).
//!
//! This is the gap-discovery tool, not a CI gate. Run with:
//!   cargo test -p biome_react_compiler_conformance --test corpus_sweep -- --ignored --nocapture
//!
//! The fixtures dir defaults to the sibling `react/` checkout; override with
//! `REACT_COMPILER_FIXTURES=/path/to/fixtures`.
//!
//! # Triage of the remaining skip-list entries
//!
//! After the import-binding and object-method-scope fixes, the corpus is at
//! ~99.6% diagnostics parity (1691 MATCH / 0 Tier-1 / 6 Tier-2). The 6 residual
//! divergences in `conformance_skiplist.txt` are each one of:
//!
//! - **Oracle under-reports (Biome is correct).** On
//!   `error.bug-invariant-local-or-context-references.js` and
//!   `preserve-memo-validation/error.useCallback-conditional-access-noAlloc.ts`,
//!   Biome emits exactly the error the fixture's own `.expect.md` documents,
//!   while the OXC oracle emits nothing — OXC loses the local-vs-context and the
//!   `?.`-vs-`.` dependency distinctions. Not Biome bugs.
//! - **Backend (Rust-port) frontier.** `todo-round3_promote_used_temps.js` is
//!   minified CommonJS prod code whose header documents a known
//!   `PromoteUsedTemporaries` divergence in the compiler's Rust port — unrelated
//!   to Biome's frontend.
//! - **Deep edge cases on `error.` fixtures, both frontends still error.**
//!   `error.default-param-accesses-local.js` (Biome compiles where Babel/OXC
//!   bail with a reorderability Todo on an arrow-valued default param),
//!   `rules-of-hooks/error.invalid-hook-optional-property.js` (Biome flags Hooks
//!   but with a different reason for an optional-chained hook call), and
//!   `error.invalid-unclosed-eslint-suppression.js` (a multi-error fixture where
//!   the secondary errors differ; both report VoidUseMemo).
//!
//! None of the residuals affect compilation of valid code, so none can produce
//! a false positive in the `useReactCompiler` rule.

use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};

use biome_js_syntax::JsFileSource;
use biome_react_compiler::ReactCompilerError;
use biome_react_compiler_conformance::{biome_diag_keys_from_root, oxc_diag_keys};

fn fixtures_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("REACT_COMPILER_FIXTURES") {
        return PathBuf::from(dir);
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../react/compiler/packages/babel-plugin-react-compiler/src/__tests__/fixtures/compiler")
}

fn collect_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_sources(&path, out);
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("js" | "jsx" | "mjs" | "ts" | "tsx")
        ) {
            out.push(path);
        }
    }
}

fn source_types(path: &Path) -> (JsFileSource, oxc_span::SourceType) {
    match path.extension().and_then(|e| e.to_str()) {
        Some("ts") => (JsFileSource::ts(), oxc_span::SourceType::ts()),
        Some("tsx") => (JsFileSource::tsx(), oxc_span::SourceType::tsx()),
        // js / jsx / mjs
        _ => (JsFileSource::jsx(), oxc_span::SourceType::jsx()),
    }
}

/// Normalized signature for a Tier-1 converter failure, with ranges stripped so
/// identical failure modes aggregate.
fn gap_signature(error: &ReactCompilerError) -> String {
    match error {
        ReactCompilerError::UnsupportedSyntax { kind, .. } => {
            format!("UnsupportedSyntax({kind:?})")
        }
        ReactCompilerError::MissingSyntax { node, field } => {
            format!("MissingSyntax({node}.{field})")
        }
        ReactCompilerError::InvalidLiteral { reason, .. } => format!("InvalidLiteral({reason})"),
        ReactCompilerError::CompilerDiagnostic { .. } => "CompilerDiagnostic".to_string(),
        ReactCompilerError::CompilerOutput(_) => "CompilerOutput/panic".to_string(),
    }
}

#[test]
#[ignore = "dev-only full-corpus sweep; run with --ignored --nocapture"]
fn corpus_sweep() {
    // The OXC oracle recurses deeply on some fixtures; the Biome side already
    // runs on its own 64MB stack, but the OXC call runs inline. A stack overflow
    // aborts the process (it is not unwindable), so run the whole sweep on a
    // large stack.
    std::thread::Builder::new()
        .stack_size(512 * 1024 * 1024)
        .spawn(run_sweep)
        .expect("spawn sweep thread")
        .join()
        .expect("sweep thread panicked");
}

fn run_sweep() {
    let dir = fixtures_dir();
    assert!(
        dir.is_dir(),
        "fixtures dir not found: {}\nset REACT_COMPILER_FIXTURES or check out react/ at the pinned rev",
        dir.display()
    );

    // The sweep deliberately exercises panicky inputs; silence the default hook
    // so the report stays readable.
    std::panic::set_hook(Box::new(|_| {}));

    let mut sources = Vec::new();
    collect_sources(&dir, &mut sources);
    sources.sort();

    let (mut matched, mut skipped_flow, mut skipped_parse, mut tier2, mut oxc_panic) =
        (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut gaps: BTreeMap<String, (u32, String)> = BTreeMap::new();
    let mut tier2_samples: Vec<String> = Vec::new();
    let mut tier2_buckets: BTreeMap<String, u32> = BTreeMap::new();
    let mut tier2_missing: BTreeMap<String, u32> = BTreeMap::new();
    // Fixture paths that diverge from the oracle (Tier-1 gaps or Tier-2), for
    // the checked-in skip-list ratchet.
    let mut diverging: Vec<String> = Vec::new();

    for path in &sources {
        let Ok(source) = std::fs::read_to_string(path) else {
            continue;
        };
        if source.contains("@flow") {
            skipped_flow += 1;
            continue;
        }
        let (js_type, oxc_type) = source_types(path);

        // Parse once on the Biome side; a parser error is out of scope.
        let parsed = biome_js_parser::parse(&source, js_type, biome_js_parser::JsParserOptions::default());
        if parsed.has_errors() {
            skipped_parse += 1;
            continue;
        }

        let tree = parsed.tree();
        let biome = biome_diag_keys_from_root(&tree, &source, js_type);
        let oxc = match catch_unwind(AssertUnwindSafe(|| oxc_diag_keys(&source, oxc_type))) {
            Ok(keys) => keys,
            Err(_) => {
                oxc_panic += 1;
                continue;
            }
        };

        match biome {
            Err(error) => {
                let entry = gaps.entry(gap_signature(&error)).or_insert((0, String::new()));
                entry.0 += 1;
                if entry.1.is_empty() {
                    entry.1 = rel(path, &dir);
                }
                diverging.push(rel(path, &dir));
            }
            Ok(biome_keys) => {
                if biome_keys == oxc {
                    matched += 1;
                } else {
                    tier2 += 1;
                    // Bucket by the distinct set of Biome-side categories so we
                    // can tell the dominant scope-conversion failure apart from
                    // genuinely separate divergences.
                    let mut cats: Vec<&str> =
                        biome_keys.iter().map(|(c, _)| c.as_str()).collect();
                    cats.sort_unstable();
                    cats.dedup();
                    let sig = if cats.is_empty() {
                        "<none>".to_string()
                    } else {
                        cats.join("+")
                    };
                    *tier2_buckets.entry(sig).or_insert(0) += 1;

                    // When Biome emits nothing, the signal is what OXC detected
                    // that Biome missed — bucket those by OXC-side categories.
                    if biome_keys.is_empty() {
                        let mut ocats: Vec<&str> = oxc.iter().map(|(c, _)| c.as_str()).collect();
                        ocats.sort_unstable();
                        ocats.dedup();
                        *tier2_missing.entry(ocats.join("+")).or_insert(0) += 1;
                    }

                    diverging.push(rel(path, &dir));
                    if tier2_samples.len() < 25 {
                        tier2_samples.push(format!(
                            "  {}\n    biome: {:?}\n    oxc:   {:?}",
                            rel(path, &dir),
                            biome_keys,
                            oxc
                        ));
                    }
                }
            }
        }
    }

    let _ = std::panic::take_hook();

    let tier1: u32 = gaps.values().map(|(n, _)| n).sum();
    println!("\n===== React Compiler conformance sweep =====");
    println!("fixtures dir: {}", dir.display());
    println!("total source files:   {}", sources.len());
    println!("skipped (@flow):      {skipped_flow}");
    println!("skipped (parse err):  {skipped_parse}");
    println!("oxc panicked:         {oxc_panic}");
    println!("---");
    println!("MATCH:                {matched}");
    println!("TIER-1 converter gap: {tier1}");
    println!("TIER-2 divergence:    {tier2}");
    println!("\n----- Tier-1 gap signatures (distinct) -----");
    let mut gap_rows: Vec<_> = gaps.iter().collect();
    gap_rows.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    for (sig, (count, sample)) in gap_rows {
        println!("  {count:>5}  {sig}   e.g. {sample}");
    }
    println!("\n----- Tier-2 buckets by Biome-side category set -----");
    let mut bucket_rows: Vec<_> = tier2_buckets.iter().collect();
    bucket_rows.sort_by(|a, b| b.1.cmp(a.1));
    for (sig, count) in bucket_rows {
        println!("  {count:>5}  {sig}");
    }
    println!("\n----- Tier-2 'Biome emits nothing': by OXC-side categories missed -----");
    let mut missing_rows: Vec<_> = tier2_missing.iter().collect();
    missing_rows.sort_by(|a, b| b.1.cmp(a.1));
    for (sig, count) in missing_rows {
        println!("  {count:>5}  {sig}");
    }
    println!("\n----- Tier-2 divergences (first {}) -----", tier2_samples.len());
    for sample in &tier2_samples {
        println!("{sample}");
    }
    println!("============================================\n");

    // --- skip-list ratchet ---
    // `conformance_skiplist.txt` records the fixtures known to diverge from the
    // oracle. The sweep fails if a fixture diverges that is NOT listed (a
    // regression) or if a listed fixture now matches (the list is stale and
    // should shrink). Regenerate with `UPDATE_SKIPLIST=1`.
    diverging.sort();
    diverging.dedup();
    let skiplist_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/conformance_skiplist.txt");

    if std::env::var("UPDATE_SKIPLIST").is_ok() {
        let mut contents = String::from(
            "# Fixtures whose React Compiler diagnostics diverge from the OXC oracle.\n\
             # Generated by `UPDATE_SKIPLIST=1 cargo test --test corpus_sweep -- --ignored`.\n\
             # Each line is a fixture path relative to the corpus root. The sweep fails\n\
             # on any divergence not listed here (regression) or any listed fixture that\n\
             # now matches (stale — remove it). Shrinking this list is the goal.\n",
        );
        for path in &diverging {
            contents.push_str(path);
            contents.push('\n');
        }
        std::fs::write(&skiplist_path, contents).expect("write skiplist");
        println!("Wrote {} entries to {}", diverging.len(), skiplist_path.display());
        return;
    }

    let listed: std::collections::HashSet<String> = std::fs::read_to_string(&skiplist_path)
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect();
    let diverging_set: std::collections::HashSet<&String> = diverging.iter().collect();

    let new: Vec<&String> = diverging.iter().filter(|p| !listed.contains(*p)).collect();
    let stale: Vec<&String> = listed.iter().filter(|p| !diverging_set.contains(p)).collect();

    println!("skip-list: {} listed, {} diverging now, {} new, {} stale", listed.len(), diverging.len(), new.len(), stale.len());
    for path in &new {
        println!("  NEW divergence (regression): {path}");
    }
    for path in &stale {
        println!("  STALE entry (now matches, remove it): {path}");
    }
    assert!(
        new.is_empty() && stale.is_empty(),
        "conformance regressed against the skip-list ({} new, {} stale). Run UPDATE_SKIPLIST=1 to regenerate after intentional changes.",
        new.len(),
        stale.len()
    );
}

fn rel(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .display()
        .to_string()
}
