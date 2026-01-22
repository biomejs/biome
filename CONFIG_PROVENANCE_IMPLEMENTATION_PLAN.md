# Configuration Provenance Tracking - Implementation Plan with Graphite Stacking

## Overview

This document outlines the implementation strategy for the Configuration Provenance Tracking feature using **Graphite's stacked PRs workflow**. Each phase will be developed as a separate PR stacked on top of the previous one, enabling parallel development and incremental code review.

## Why Stacked PRs?

- **Smaller, focused reviews**: Each PR contains a logical unit of work (~100-500 lines)
- **Parallel development**: Don't wait for Phase 1 to be merged before starting Phase 2
- **Faster feedback cycles**: Reviewers can approve/comment on individual phases
- **Easier to understand**: Each PR has a clear purpose and deliverable
- **Safer iterations**: Can address feedback on Phase 1 while continuing work on Phase 2+

## Graphite Workflow

### Initial Setup
```bash
# Work on feature branch to preserve design documents
git checkout feat/config-debugging
```

> **Important**: Per Biome's contributing guidelines, new features that affect end users must target the `next` branch, not `main`. Configuration Provenance Tracking is a user-facing feature.
>
> **Strategy**: We'll develop all phases on `feat/config-debugging` branch (based on `next`), creating stacked PRs within this feature branch. This preserves our design documents throughout development. When complete, we'll create a final PR from `feat/config-debugging` → `next`.

### Creating Each Phase
For each phase below, we'll:
1. Make code changes for that phase
2. Run `gt create --all --message "feat(provenance): <phase description>"`
3. Run `gt submit` to create the PR
4. Continue to next phase (stacked on current branch)

### Handling Feedback
If Phase 1 gets review feedback while working on Phase 3:
```bash
gt checkout <phase-1-branch>
# Make fixes
gt modify --all
gt submit --stack  # Updates all PRs in the stack
```

### Merging
PRs will be merged sequentially from bottom to top:
1. Phase 1 merges first
2. Phase 2's base automatically updates to main, then merges
3. Continue until all phases merged

---

## 📋 Implementation Phases (Stacked PRs)

### Phase 1: Core Provenance Data Structures
**PR Title**: `feat(provenance): add core data structures and traits`

**Description**: Foundational types for provenance tracking with zero breaking changes.

**Changes**:
- Create `crates/biome_configuration/src/provenance/` module
  - `field_query.rs` - FieldQuery, FieldQuerySegment
  - `source.rs` - ProvenanceSource enum
  - `entry.rs` - ProvenanceEntry with AstPtr<AnyJsonValue>
  - `override_metadata.rs` - OverrideProvenanceMetadata, GlobMatcher
  - `mod.rs` - Re-export public types

**Tests**:
- Unit tests for FieldQuery construction and manipulation
- Unit tests for ProvenanceSource display formatting
- Unit tests for FieldQuerySegment comparison

**Deliverable**: Core types compile and pass tests. No integration yet.

**Files affected**: ~4-5 new files, ~300-400 LOC

**Graphite commands**:
```bash
git checkout feat/config-debugging
# Implement Phase 1
gt create --all --message "feat(provenance): add core data structures and traits"
gt submit
```

---

### Phase 2: Extend Deserialization Infrastructure
**PR Title**: `feat(provenance): extend deserialization context with provenance tracking`

**Description**: Add provenance capture capability to deserialization, opt-in via trait.

**Changes**:
- Modify `crates/biome_deserialize/src/lib.rs`:
  - Add `Provenance` trait (push_field, push_index, pop, current_path, capture_value)
  - Add `provenance()` method to `DeserializationContext` trait
  - Default implementation returns `None` (backward compatible)
- Create `crates/biome_configuration/src/provenance/context.rs`:
  - `ProvenanceImpl` struct implementing `Provenance`
  - `ProvenanceAwareDeserializationContext` wrapping default context

**Tests**:
- Test `ProvenanceImpl` path tracking (push/pop)
- Test `capture_value()` stores AstPtr correctly
- Verify backward compatibility (existing code still works)

**Deliverable**: Deserialization infrastructure ready to capture provenance.

**Files affected**: ~2 modified, ~2 new files, ~250-300 LOC

**Graphite commands**:
```bash
# Already on phase-1 branch
# Implement Phase 2
gt create --all --message "feat(provenance): extend deserialization context with provenance tracking"
gt submit --stack
```

---

### Phase 3: Query Logic and Matching
**PR Title**: `feat(provenance): implement zero-allocation query parsing and matching`

**Description**: Core query engine for finding provenance entries.

**Changes**:
- Create `crates/biome_service/src/workspace/provenance_query.rs`:
  - `parse_query_segments()` - Parse "formatter.indentWidth" into ParsedQuery
  - `ParsedQuery` and `ParsedSegment` (zero-copy slices)
  - `query_provenance()` - Filter entries, return winner
  - `field_path_matches()` - Compare FieldQuery vs ParsedQuery
  - `QueryParseError` with position info

**Tests**:
- Test query parsing: "formatter.indentWidth" → correct segments
- Test query parsing with arrays: "overrides[0].linter" → Field, Index, Field
- Test malformed queries return proper errors
- Test field_path_matches with various FieldQuery/ParsedQuery combos
- Test winner selection (highest merge_order)

**Deliverable**: Query engine works with hand-crafted provenance entries.

**Files affected**: ~1 new file, ~300-350 LOC

**Graphite commands**:
```bash
# Already on phase-2 branch
# Implement Phase 3
gt create --all --message "feat(provenance): implement zero-allocation query parsing and matching"
gt submit --stack
```

---

### Phase 4: ProvenanceIndex and Storage
**PR Title**: `feat(provenance): add provenance index storage and settings integration`

**Description**: Storage layer for provenance, integrated with Settings.

**Changes**:
- Create `crates/biome_service/src/workspace/provenance.rs`:
  - `ProvenanceIndex` struct (entries Vec, override_metadata Vec, base_config_path)
  - `ProvenanceIndex::build()` - Sort entries by merge_order
- Modify `crates/biome_service/src/settings.rs`:
  - Add `provenance: Option<Arc<ProvenanceIndex>>` field
  - Add `provenance()`, `has_provenance()` methods
  - Update `from_loaded_config()` to accept optional ProvenanceIndex

**Tests**:
- Test ProvenanceIndex::build sorts entries correctly
- Test Settings stores and retrieves provenance
- Test Arc cloning is cheap

**Deliverable**: Can store provenance in Settings, query it manually.

**Files affected**: ~1 new, ~1 modified, ~150-200 LOC

**Graphite commands**:
```bash
# Already on phase-3 branch
# Implement Phase 4
gt create --all --message "feat(provenance): add provenance index storage and settings integration"
gt submit --stack
```

---

### Phase 5: CaptureProvenance Derive Macro
**PR Title**: `feat(provenance): add CaptureProvenance derive macro`

**Description**: Opt-in derive macro to capture syntax nodes during deserialization.

**Changes**:
- Create `crates/biome_deserialize_macros/src/capture_provenance_derive.rs`:
  - Proc macro that hooks into Deserializable visitor
  - Generates code to call `provenance()` methods during deserialization
  - Tracks path stack (push field names, indices)
  - Captures leaf values with AstPtr
- Update `crates/biome_deserialize_macros/src/lib.rs` to export the macro
- Add integration tests with simple struct

**Tests**:
- Test macro generates correct code for simple struct
- Test captured FieldQuery matches actual JSON structure
- Test AstPtr points to correct syntax nodes

**Deliverable**: `#[derive(CaptureProvenance)]` works on test structs.

**Files affected**: ~2 new, ~1 modified, ~400-500 LOC (macro code is verbose)

**Challenge**: Deep integration with Deserializable - may need to iterate based on how visitor works.

**Graphite commands**:
```bash
# Already on phase-4 branch
# Implement Phase 5
gt create --all --message "feat(provenance): add CaptureProvenance derive macro"
gt submit --stack
```

---

### Phase 6: Configuration Loading with Provenance
**PR Title**: `feat(provenance): integrate provenance capture into config loading`

**Description**: Wire provenance capture into actual configuration loading.

**Changes**:
- Modify `crates/biome_service/src/configuration.rs`:
  - Create `load_configuration_with_provenance()` function
  - Use `ProvenanceAwareDeserializationContext` during deserialization
  - Collect entries from `ctx.take_entries()`
  - Create `LoadedConfigurationWithProvenance` struct
- Modify `Configuration::apply_extends()`:
  - Add `enable_provenance: bool` parameter
  - Track `merge_counter: u64` across all config loads
  - Aggregate entries from base + extended configs
  - Create `extract_override_metadata()` function
- Add `#[derive(CaptureProvenance)]` to:
  - `Configuration`
  - `OverridePattern`
  - `FormatterConfiguration`, `LinterConfiguration`, etc.

**Tests**:
- Test loading simple config captures base entries
- Test extends chain captures all entries with correct merge_order
- Test override metadata extraction with globs

**Deliverable**: Full provenance tracking during config loading.

**Files affected**: ~5-10 modified (config types), ~300-400 LOC

**Graphite commands**:
```bash
# Already on phase-5 branch
# Implement Phase 6
gt create --all --message "feat(provenance): integrate provenance capture into config loading"
gt submit --stack
```

---

### Phase 7: Workspace Query API
**PR Title**: `feat(provenance): add workspace query API for provenance`

**Description**: Public API for querying provenance from workspace.

**Changes**:
- Modify `crates/biome_service/src/workspace.rs`:
  - Add `query_configuration_provenance()` method to `Workspace` trait
  - Create `QueryProvenanceParams` struct (project_key, field_query, file_path)
  - Implement for `WorkspaceServer`:
    - Get appropriate Settings (root vs nested based on file_path)
    - Call `query_provenance()` with JsonRoot from `get_parse()`
    - Handle override evaluation if file_path provided
- Create `WorkspaceSettings::enable_provenance` flag

**Tests**:
- Test query without file_path returns base config entry
- Test query with file_path includes matching override
- Test query with non-matching file_path excludes override
- Test multi-project workspace selects correct Settings

**Deliverable**: Can query provenance via workspace.query_configuration_provenance().

**Files affected**: ~2-3 modified, ~200-250 LOC

**Graphite commands**:
```bash
# Already on phase-6 branch
# Implement Phase 7
gt create --all --message "feat(provenance): add workspace query API for provenance"
gt submit --stack
```

---

### Phase 8: CLI Command Implementation
**PR Title**: `feat(cli): add 'biome explain config' command`

**Description**: User-facing CLI command to explain config provenance.

**Changes**:
- Modify `crates/biome_cli/src/commands/mod.rs`:
  - Create `ExplainSubcommand` enum (Config, Rule, DaemonLogs)
  - Update `BiomeCommand::Explain` to use subcommand
- Modify `crates/biome_cli/src/commands/explain.rs`:
  - Implement `explain_config()` function
  - Call `workspace.query_configuration_provenance()`
  - Convert ProvenanceEntry to diagnostic
  - Display with code frame
- Modify `crates/biome_service/src/scanner.rs`:
  - Add `ScanKind::ConfigFiles` variant
  - Filter to only biome.json, biome.jsonc, .editorconfig
- Create `crates/biome_service/src/workspace/provenance.rs`:
  - `ProvenanceEntryDiagnostic` struct
  - Implement `Diagnostic` trait for rich CLI output
  - Implement `Display` for `ProvenanceSource` (with markup)

**Tests**:
- Integration test: CLI command with simple config
- Test output formatting
- Test --file flag includes overrides
- Test error messages for invalid queries

**Deliverable**: `biome explain config formatter.indentWidth` works end-to-end.

**Files affected**: ~4-5 modified, ~400-500 LOC

**Graphite commands**:
```bash
# Already on phase-7 branch
# Implement Phase 8
gt create --all --message "feat(cli): add 'biome explain config' command"
gt submit --stack
```

---

### Phase 9: LSP Hover Integration
**PR Title**: `feat(lsp): add provenance hover for config files`

**Description**: LSP hover shows provenance info in config files.

**Changes**:
- Create `crates/biome_lsp/src/handlers/hover.rs` (or modify if exists):
  - Check if file is config file (biome.json/biome.jsonc)
  - Use `LineIndex` to convert LSP Position → TextOffset
  - Find JSON node at cursor via `token_at_offset()`
  - Extract field path string by traversing AST upwards
  - Query provenance via workspace API
  - Format result as Markdown hover
  - Convert TextRange → LSP Range for highlighting

**Tests**:
- Test hover on config field shows provenance
- Test hover on non-config file returns None
- Test Position → TextOffset → node lookup
- Test field path extraction from cursor position

**Deliverable**: Hover in VSCode shows provenance info for config fields.

**Files affected**: ~1-2 new/modified, ~250-300 LOC

**Graphite commands**:
```bash
# Already on phase-8 branch
# Implement Phase 9
gt create --all --message "feat(lsp): add provenance hover for config files"
gt submit --stack
```

---

### Phase 10: EditorConfig and Polish
**PR Title**: `feat(provenance): add editorconfig support and optimizations`

**Description**: Complete feature with editorconfig, performance tuning, docs.

**Changes**:
- Add EditorConfig provenance tracking:
  - Create `ProvenanceSource::EditorConfig` entries
  - Capture during editorconfig loading
- Performance optimizations:
  - Benchmark provenance overhead
  - Consider making opt-in via flag if >10% overhead
  - Cache parsed queries if beneficial
- Documentation:
  - Add user guide to website docs
  - Add developer architecture docs
  - Add troubleshooting guide
- Comprehensive testing:
  - End-to-end tests for complex scenarios
  - Performance regression tests
  - Edge case testing (empty configs, malformed paths, etc.)

**Tests**:
- Test editorconfig provenance capture
- Performance benchmarks
- Complex integration scenarios

**Deliverable**: Production-ready, optimized, well-documented feature.

**Files affected**: ~10-15 modified (tests, docs), ~500-600 LOC

**Graphite commands**:
```bash
# Already on phase-9 branch
# Implement Phase 10
gt create --all --message "feat(provenance): add editorconfig support and optimizations"
gt submit --stack
```

---

## 🔧 Handling Feedback During Development

### Scenario: Phase 1 gets review comments while working on Phase 5

```bash
# Currently on phase-5 branch
gt checkout <phase-1-branch>

# Make the requested changes
# ... edit files ...

# Amend the commit (keeps history clean)
gt modify --all

# This auto-restacks all child branches (Phase 2-5) on top of updated Phase 1
# Push updates to all PRs in the stack
gt submit --stack
```

Graphite will:
1. Update Phase 1's PR with your fixes
2. Automatically rebase Phase 2 on updated Phase 1
3. Automatically rebase Phase 3 on updated Phase 2
4. Continue through Phase 5
5. Force-push all updated branches

Reviewers will see:
- Phase 1: Updated with fixes
- Phase 2-5: May have slight commit hash changes, but content unchanged (just rebased)

### Scenario: Need to insert a new phase between Phase 2 and Phase 3

```bash
gt checkout <phase-2-branch>
# Make new changes
gt create --insert --all --message "feat(provenance): add intermediate optimization"
# This creates a new branch between Phase 2 and Phase 3
gt submit --stack
```

---

## 🚀 Starting the Implementation

### Initial Setup
```bash
# Ensure Graphite is configured
gt init

# Start from feat/config-debugging branch (preserves design docs)
git checkout feat/config-debugging

# Begin Phase 1
# ... implement Phase 1 changes ...
gt create --all --message "feat(provenance): add core data structures and traits"
gt submit --draft  # Mark as draft initially if you want feedback before official review

# Immediately start Phase 2 (stacked on Phase 1)
# ... implement Phase 2 changes ...
gt create --all --message "feat(provenance): extend deserialization context with provenance tracking"
gt submit --stack

# Continue pattern for remaining phases...
```

### Tips for Success
- **Keep PRs focused**: Each phase should have a single, clear purpose
- **Test incrementally**: Don't wait until Phase 10 to test Phase 1
- **Regular syncs**: Run `gt sync` daily to stay current with main

---

## 🎯 Timeline Estimate

Assuming ~1-2 days per phase (implementation + review):

- **Phase 1-4** (Foundation): ~1 week
- **Phase 5-7** (Integration): ~1 week  
- **Phase 8-10** (UI + Polish): ~1 week

**Total**: ~3 weeks with overlapping development and review cycles

With stacked PRs, phases can be worked on in parallel with reviews, so actual calendar time may be compressed to ~2 weeks.

---

## 📚 Additional Resources

- **Graphite Docs**: https://graphite.dev/docs
- **Biome Contributing**: See CONTRIBUTING.md
- **Design Document**: CONFIG_PROVENANCE_DESIGN.md (full technical specification)

---

## ✅ Pre-Implementation Checklist

Before starting Phase 1:

- [ ] Read full CONFIG_PROVENANCE_DESIGN.md
- [ ] Understand Biome's deserialization system (`biome_deserialize` crate)
- [ ] Understand Biome's configuration loading (`biome_service/src/configuration.rs`)
- [ ] Set up Graphite CLI (`gt init`)
- [ ] Confirm workspace compiles and tests pass on main
- [ ] Read Biome's CONTRIBUTING.md for conventions
- [ ] Plan AI usage and disclosure strategy

---

This implementation plan provides a clear path forward with manageable, reviewable chunks of work. Each phase builds logically on the previous one, and the stacked PR approach enables efficient parallel development and review cycles.
