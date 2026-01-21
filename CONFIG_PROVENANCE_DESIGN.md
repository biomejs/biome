# Configuration Provenance Tracking Design for Biome

## 1. Overview

This document describes the design for a configuration debugging feature that tracks which configuration source (extends, overrides, editorconfig) sets each field value in Biome's configuration system.

### Goals
- Track provenance at leaf-value granularity including array indices
- Support querying via CLI, LSP, and Workspace API
- Handle extends chains, nested configs, overrides, and editorconfig
- Use lazy evaluation for file-specific override queries
- Maintain backward compatibility and performance

## 2. Core Data Structures

### 2.1 Field Path Representation (Capture Phase)

**Purpose**: FieldQuery is used ONLY during **provenance capture** (deserialization phase) to store field paths with pointers to the actual JSON nodes. It is NOT used for query matching.

```rust
// crates/biome_configuration/src/provenance/field_query.rs (NEW)

use biome_json_syntax::JsonMemberName;
use biome_rowan::AstPtr;

/// A segment in a configuration field query
#[derive(Debug)]
pub enum FieldQuerySegment {
    /// A named field: stores a pointer to the JsonMemberName node
    /// AstPtr is thread-safe (Send + Sync) and only 8 bytes
    Field(AstPtr<JsonMemberName>),

    /// An array index: [2]
    Index(usize),
}

/// Represents a captured field path during configuration deserialization
/// Examples: "formatter.indentWidth", "extends[2]", "overrides[1].includes[0]"
///
/// **Important**: This structure is used ONLY during the capture phase when we have
/// access to the JsonRoot and can create proper AstPtr references. User queries
/// (from CLI/LSP) remain as strings and are processed using the QueryVisitor pattern.
///
/// Thread-safe structure using AstPtr (Send + Sync) for minimal memory usage.
/// Field names are resolved on-demand using JsonRoot when displaying or comparing.
#[derive(Debug)]
pub struct FieldQuery {
    segments: Vec<FieldQuerySegment>,
}

impl FieldQuery {
    /// Convert to string representation for display/debugging
    /// Requires JsonRoot to resolve AstPtr field names
    pub fn to_string(&self, root: &JsonRoot) -> String {
        let mut result = String::new();
        for (i, segment) in self.segments.iter().enumerate() {
            match segment {
                FieldQuerySegment::Field(ptr) => {
                    if i > 0 {
                        result.push('.');
                    }
                    // Resolve the AstPtr to get the field name
                    let node = ptr.to_node(root.syntax());
                    if let Ok(text) = node.inner_string_text() {
                        result.push_str(&text);
                    }
                }
                FieldQuerySegment::Index(idx) => {
                    result.push('[');
                    result.push_str(&idx.to_string());
                    result.push(']');
                }
            }
        }
        result
    }

    /// Check if this query starts with another query (for prefix matching during capture)
    pub fn starts_with(&self, prefix: &FieldQuery) -> bool {
        if prefix.segments.len() > self.segments.len() {
            return false;
        }
        self.segments[..prefix.segments.len()] == prefix.segments
    }
}
```

**Usage Example**:
```rust
// FieldQuery is built during deserialization by ProvenanceImpl
// which maintains a path_stack and constructs FieldQuery directly:
let path = FieldQuery {
    segments: vec![
        FieldQuerySegment::Field(AstPtr::new(&member_name_node)),
        FieldQuerySegment::Field(AstPtr::new(&inner_member_node)),
    ]
};

// To display the captured path, resolve AstPtr using JsonRoot
assert_eq!(path.to_string(&root), "formatter.indentWidth");

// During capture, check if we're inside an override block
let override_prefix = /* FieldQuery built during traversal */;
if path.starts_with(&override_prefix) {
    // This entry belongs to an override
}
```

**Important**: User queries from CLI/LSP are NOT parsed into FieldQuery. They remain as strings and are processed by the QueryVisitor (see Section 2.2) which navigates the JsonRoot directly.

### 2.2 Query Logic (Query Phase)

**Purpose**: Process user queries (from CLI/LSP) by matching against captured provenance entries and returning the winning configuration source.

**Design Philosophy**: We return only the **final/winning** entry that determines the actual configuration value, not a full history chain. This simplifies the implementation and matches the user's primary need: "where does this value come from?"

```rust
// crates/biome_service/src/workspace/provenance_query.rs (NEW)

use biome_json_syntax::{JsonRoot, JsonMemberName};
use biome_configuration::provenance::{ProvenanceEntry, ProvenanceIndex, FieldQuery, FieldQuerySegment};
use camino::Utf8Path;
use std::iter::Peekable;
use std::str::Chars;

/// Parsed query using string slices (zero allocations)
struct ParsedQuery<'a> {
    segments: Vec<ParsedSegment<'a>>,
}

/// A query segment that references the original query string
enum ParsedSegment<'a> {
    Field(&'a str),  // String slice into original query - no allocation!
    Index(usize),    // Parsed integer
}

/// Parse errors with position information for good DX
#[derive(Debug)]
pub enum QueryParseError {
    MalformedIndex { position: usize },
    InvalidIndex { position: usize, value: String },
    UnexpectedEnd,
}

/// Query provenance entries to find the winning configuration source
///
/// Returns the entry with the highest merge_order (last applied = winner).
/// If file_path is provided, considers matching overrides; otherwise ignores all overrides.
pub fn query_provenance(
    query: &str,
    provenance_index: &ProvenanceIndex,
    json_root: &JsonRoot,
    file_path: Option<&Utf8Path>,
) -> Result<Option<ProvenanceEntry>, QueryParseError> {
    // 1. Parse query into ParsedQuery with &str slices (zero allocations)
    let parsed_query = parse_query_segments(query)?;

    // 2. Find winner using max_by_key (no Vec allocation!)
    let winner = provenance_index
        .entries
        .iter()
        .filter(|entry| {
            // Match using ParsedQuery and FieldQuery
            if !field_path_matches(&entry.field_path, &parsed_query, json_root) {
                return false;
            }

            // Handle overrides
            match &entry.source {
                ProvenanceSource::Override { index, .. } => {
                    file_path
                        .and_then(|path| provenance_index.override_metadata.get(*index))
                        .map(|meta| meta.matchers.iter().any(|m| m.matches(path)))
                        .unwrap_or(false)
                }
                _ => true,  // Non-override sources always included
            }
        })
        .max_by_key(|e| e.merge_order)
        .cloned();

    Ok(winner)
}

/// Parse query string into segments using &str slices (zero allocations)
///
/// Examples:
///   "formatter.indentWidth" → [Field("formatter"), Field("indentWidth")]
///   "overrides[1].linter" → [Field("overrides"), Index(1), Field("linter")]
fn parse_query_segments(query: &str) -> Result<ParsedQuery, QueryParseError> {
    let mut segments = Vec::new();
    let mut start = 0;
    let mut i = 0;
    let bytes = query.as_bytes();

    while i < bytes.len() {
        match bytes[i] {
            b'.' => {
                if i > start {
                    // Field segment: slice from start to i
                    segments.push(ParsedSegment::Field(&query[start..i]));
                }
                start = i + 1;
                i += 1;
            }
            b'[' => {
                if i > start {
                    segments.push(ParsedSegment::Field(&query[start..i]));
                }
                // Parse array index
                i += 1;
                let index_start = i;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                if i >= bytes.len() || bytes[i] != b']' {
                    return Err(QueryParseError::MalformedIndex { position: index_start });
                }
                // Parse the number slice
                let index_str = &query[index_start..i];
                let index = index_str.parse::<usize>()
                    .map_err(|_| QueryParseError::InvalidIndex {
                        position: index_start,
                        value: index_str.to_string(),
                    })?;
                segments.push(ParsedSegment::Index(index));
                i += 1; // skip ']'
                start = i;
                // Skip optional '.' after ']'
                if i < bytes.len() && bytes[i] == b'.' {
                    i += 1;
                    start = i;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    // Add final segment if any
    if start < bytes.len() {
        segments.push(ParsedSegment::Field(&query[start..]));
    }

    Ok(ParsedQuery { segments })
}

/// Check if a FieldQuery matches the ParsedQuery (zero allocations)
fn field_path_matches(
    field_path: &FieldQuery,
    parsed_query: &ParsedQuery,
    json_root: &JsonRoot,
) -> bool {
    if field_path.segments.len() != parsed_query.segments.len() {
        return false;
    }

    // Zip and compare segments
    for (entry_seg, query_seg) in field_path.segments.iter().zip(&parsed_query.segments) {
        match (entry_seg, query_seg) {
            (FieldQuerySegment::Field(ptr), ParsedSegment::Field(name)) => {
                let node = ptr.to_node(json_root.syntax());
                if !node_name_equals(&node, name) {
                    return false;
                }
            }
            (FieldQuerySegment::Index(a), ParsedSegment::Index(b)) => {
                if a != b {  // Direct integer comparison - no allocation!
                    return false;
                }
            }
            _ => return false,  // Mismatched segment types (Field vs Index)
        }
    }

    true
}

/// Compare node field name against &str slice without allocation
fn node_name_equals(node: &JsonMemberName, expected: &str) -> bool {
    // Extract field name from node and compare against slice
    // Implementation depends on biome_json_syntax API
    if let Ok(text) = node.inner_string_text() {
        text.text() == expected
    } else {
        false
    }
}
```

**Key Design Points**:

1. **Zero allocations**: ParsedSegment::Field holds `&str` slices into original query string
2. **Parse-once, match-many**: Query parsed once, then matched against multiple entries
3. **Good error handling**: Separate parse errors from match failures, with position info
4. **Iterator-based filtering**: Uses `max_by_key` instead of collect + sort
5. **Direct comparisons**: Integer comparison for indices, string slice comparison for fields

**Example Usage**:

```rust
// Query without file path (CLI or generic LSP hover)
let result = query_provenance(
    "formatter.indentWidth",
    &provenance_index,
    &json_root,
    None, // No file path = ignore overrides
);
// Returns: BaseConfig or ExtendedConfig entry (whichever has higher merge_order)

// Query with file path (LSP hover in specific file)
let result = query_provenance(
    "formatter.indentWidth",
    &provenance_index,
    &json_root,
    Some(Path::new("src/main.ts")),
);
// Returns: Could be Override entry if it matches "src/main.ts" and has highest merge_order
```

### 2.3 Provenance Source

```rust
// crates/biome_configuration/src/provenance/source.rs (NEW)

// Note: Assumes LoadedLocation enum exists in biome_configuration crate
// to track where a configuration file was discovered (e.g., InProject, InParentDirectory, etc.)

/// Identifies which source set a configuration value
#[derive(Debug, Clone)]
pub enum ProvenanceSource {
    /// The base/root configuration file being loaded
    BaseConfig {
        /// Absolute path to the config file
        path: Utf8PathBuf,

        /// Where this config was found (project root, parent folder, etc.)
        loaded_location: LoadedLocation,
    },

    /// An extended configuration from the 'extends' array
    ExtendedConfig {
        /// The file that set this value
        path: Utf8PathBuf,

        /// How we got here (for nested extends)
        /// Example: ["main.json", "react.json", "react-base.json"]
        /// The last entry is the file that actually set the value
        resolution_path: Vec<Utf8PathBuf>,
    },

    /// An .editorconfig file
    EditorConfig {
        /// Path to the .editorconfig file
        path: Utf8PathBuf,
    },

    /// An override pattern from the 'overrides' array
    Override {
        /// Which configuration file contained this override
        config_source: Box<ProvenanceSource>,

        /// The glob patterns (for display purposes)
        includes: Vec<String>,
    },

    /// CLI argument (--indent-width=4)
    CliArgument {
        /// The argument string for reference
        argument: String,
    },

    /// Default/fallback value (not explicitly configured)
    Default,
}

impl ProvenanceSource {
    /// Get the config file path for this source
    /// This is the actual configuration file containing the value
    pub fn config_path(&self) -> &Utf8Path {
        match self {
            Self::BaseConfig { path, .. } => path,
            Self::ExtendedConfig { path, .. } => path,
            Self::EditorConfig { path, .. } => path,
            Self::Override { config_source, .. } => config_source.config_path(),
            _ => panic!("config_path() called on source without a path"),
        }
    }
}
```

**Key Point**: We use `merge_order` (timestamp) instead of indices to determine which value wins. The file path in `ExtendedConfig` tells us which file set the value, and `resolution_path` shows how we got there for nested extends.

### 2.4 Provenance Entry (Store Syntax Nodes, Not Serialized Values)

```rust
// crates/biome_configuration/src/provenance/entry.rs (NEW)

use biome_json_syntax::AnyJsonValue;
use biome_rowan::AstNode;

/// A single record of a field being set
#[derive(Debug, Clone)]
pub struct ProvenanceEntry {
    /// Path to the field that was set
    pub field_query: FieldQuery,

    /// Where this value came from
    pub source: ProvenanceSource,

    /// Pointer to the JSON value node in the parsed tree
    /// AstPtr is thread-safe (Send + Sync) and can be resolved later
    /// by calling workspace.get_parse() to retrieve the JsonRoot
    pub value_ptr: AstPtr<AnyJsonValue>,

    /// Merge order: lower = earlier, higher = later (wins)
    pub merge_order: u64,
}

// Note: ProvenanceEntry stores AstPtr instead of the actual node.
// Value extraction is done by the caller after query_provenance() returns:
// 1. query_provenance() returns Option<ProvenanceEntry>
// 2. Caller resolves the AstPtr: entry.value_node.to_node(root.syntax())
// 3. Caller extracts value and range from the resolved node for display
```

**Why store syntax nodes instead of serialized values?**
- **Memory efficient**: Syntax nodes are already in memory from parsing; no need to serialize to JSON
- **Preserve original formatting**: We can extract the exact text as it appeared in the source
- **No serialization overhead**: We skip the `serde_json::to_value()` conversion during storage
- **Extract on demand**: Value strings are only computed during query display (rare operation)

**Example**: For `extends: ["base.json", "react.json"]` where both set `linter.enabled`:
```rust
ProvenanceEntry {
    field_query: "linter.enabled",
    source: ExtendedConfig { path: "base.json", ... },
    value_node: AnyJsonValue::JsonBooleanValue(...), // ← Syntax node, not serialized
    merge_order: 1,
}
// When displayed: entry.value_as_string() returns "false"

ProvenanceEntry {
    field_query: "linter.enabled",
    source: ExtendedConfig { path: "react.json", ... },
    value_node: AnyJsonValue::JsonBooleanValue(...), // ← Syntax node
    merge_order: 2,  // ← Higher = later = wins!
}
// When displayed: entry.value_as_string() returns "true"
```

### 2.5 Override Provenance Metadata

```rust
// crates/biome_configuration/src/provenance/override_metadata.rs (NEW)

/// Metadata about an override pattern for lazy evaluation
/// Stored in ProvenanceIndex to enable file-specific queries
#[derive(Debug)]
pub struct OverrideProvenanceMetadata {
    /// Source of this override (which config file)
    pub source: ProvenanceSource, // Will always be ProvenanceSource::Override variant

    /// The index of this override in the overrides array
    /// Used by QueryVisitor to match entries like "overrides[N].field"
    pub index: usize,

    /// The compiled glob patterns for matching files
    pub matchers: Vec<GlobMatcher>,

    /// Merge order: when this override was encountered during config loading
    pub merge_order: u64,
}

// Note: field_entries are NOT pre-extracted. QueryVisitor finds them on-demand
// by checking if entries' FieldQuery starts with "overrides[index]"

/// A glob matcher for file paths
#[derive(Debug)]
pub struct GlobMatcher {
    glob: biome_glob::Glob,
}

impl GlobMatcher {
    pub fn matches(&self, path: &Utf8Path) -> bool {
        self.glob.is_match(path)
    }
}

impl OverrideProvenanceMetadata {
    /// Create from an OverridePattern during configuration loading
    pub fn from_override_pattern(
        pattern: &OverridePattern,
        config_source: ProvenanceSource,
        provenance_entries: Vec<ProvenanceEntry>,
        merge_order: u64,
    ) -> Result<Self, ConfigError> {
        // Extract matchers from pattern.includes
        let matchers = match &pattern.includes {
            Some(OverrideGlobs::Globs(globs)) => {
                globs.iter()
                    .map(|g| GlobMatcher {
                        glob: g.clone(),
                    })
                    .collect()
            }
            Some(OverrideGlobs::EditorconfigGlob(glob)) => {
                vec![GlobMatcher {
                    glob: glob.clone().into(),
                }]
            }
            None => Vec::new(),
        };

        // Build field entries map from provenance entries
        let mut field_entries = HashMap::new();
        for entry in provenance_entries {
            field_entries.insert(entry.field_path.clone(), entry);
        }

        let source = ProvenanceSource::Override {
            config_source: Box::new(config_source.clone()),
            includes: matchers.iter().map(|m| m.glob.to_string()).collect(),
        };

        Ok(Self {
            source,
            matchers,
            field_entries,
            merge_order,
        })
    }

    /// Check if this override applies to the given file path
    pub fn matches_file(&self, path: &Utf8Path) -> bool {
        self.matchers.iter().any(|m| m.matches(path))
    }

    /// Get provenance for a specific field if this override sets it
    pub fn get_field_provenance(&self, field_query: &FieldQuery) -> Option<&ProvenanceEntry> {
        self.field_entries.get(field_path)
    }
}
```

**How Override Tracking Works**:

1. **During Config Loading**: When we encounter `overrides` array in any config (base or extended):
   ```rust
   for override_pattern in config.overrides.iter() {
       merge_counter += 1;

       // Extract all fields set by this override
       let entries = extract_override_entries(override_pattern, &config_source, merge_counter);

       // Create metadata
       let metadata = OverrideProvenanceMetadata::from_override_pattern(
           override_pattern,
           config_source.clone(),
           entries,
           merge_counter,
       )?;

       provenance_recorder.add_override_metadata(metadata);
   }
   ```

2. **From Extended Configs**: Overrides in extended configs are tracked with their source:
   ```rust
   // In apply_extends():
   for extended_config in extended_configs.iter() {
       let extend_source = ProvenanceSource::ExtendedConfig {
           path: extended_config_path.clone(),
           resolution_path: vec![base_config_path.clone(), extended_config_path.clone()],
       };

       // Process extended config's overrides
       for override_pattern in extended_config.overrides.iter() {
           merge_counter += 1;
           let entries = extract_override_entries(override_pattern, &extend_source, merge_counter);
           let metadata = OverrideProvenanceMetadata::from_override_pattern(
               override_pattern,
               extend_source.clone(), // Override came from the extended config
               entries,
               merge_counter,
           )?;

           provenance_recorder.add_override_metadata(metadata);
       }
   }
   ```

3. **During Query with File Path** (Lazy Evaluation):

> **Note**: The actual query implementation is in `Workspace::query_configuration_provenance()` (see **Section 5.2.3**).
>
> **Query Approach**: User queries remain as strings until the Workspace has access to JsonRoot. The QueryVisitor (Section 2.2) then navigates the JSON tree following the query path and collects matching provenance entries on-demand. This avoids the complexity of parsing strings into FieldQuery structures and provides better error messages when fields don't exist.

### 2.6 Provenance Capture Implementation

> **Note**: The actual provenance capture implementation is detailed in **Section 3: "Provenance Capture via Deserialization"**.
>
> This section previously described a merge-based `ProvenanceContext` that tracked changes during the merge phase, but the final design uses deserialization-time capture for better separation of concerns and to solve the override extraction problem.
>
> **See**:
> - **Section 3.2** for `ProvenanceImpl` (replaces the old `ProvenanceContext`)
> - **Section 3.3** for the `CaptureProvenance` derive macro
> - **Section 4.1** for usage in configuration loading
>
> **Why the change**:
> - `OverridePattern` doesn't derive `Merge`, so merge-based tracking can't capture its fields
> - Syntax nodes are available during deserialization, making it the natural capture point
> - Keeps Merge trait simple and focused on merging logic

## 3. Provenance Capture via Deserialization

### 3.1 Extend DeserializationContext

The key insight is that syntax nodes are available during deserialization, so we should capture them at that time rather than during merging. This is done via an opt-in derive macro.

```rust
// crates/biome_deserialize/src/lib.rs (MODIFY)

use biome_json_syntax::{AnyJsonValue, JsonMemberName};
use biome_rowan::{AstPtr, TextRange};

/// Trait for capturing provenance information during deserialization
pub trait Provenance {
    /// Push a field name onto the current path
    /// Takes JsonMemberName reference, creates AstPtr internally for thread safety
    fn push_field(&mut self, name: &JsonMemberName);

    /// Push an array index onto the current path
    fn push_index(&mut self, index: usize);

    /// Pop the last segment from the current path
    fn pop(&mut self);

    /// Get the current field path
    fn current_path(&self) -> FieldQuery;

    /// Capture a syntax node at the current path
    /// Creates an AstPtr from the node for thread-safe storage
    fn capture_value(&mut self, path: FieldQuery, node: &AnyJsonValue);
}

pub trait DeserializationContext {
    // ... existing methods (diagnostics, range, etc.) ...

    /// Returns a mutable reference to provenance capture if enabled
    /// Returns None for standard deserialization without provenance tracking
    fn provenance(&mut self) -> Option<&mut dyn Provenance>;
}

// Standard context doesn't capture provenance
impl DeserializationContext for DefaultDeserializationContext {
    // ... existing implementations ...

    fn provenance(&mut self) -> Option<&mut dyn Provenance> {
        None  // Standard deserialization doesn't track provenance
    }
}
```

### 3.2 Provenance-Aware Deserialization Context

```rust
// crates/biome_configuration/src/provenance/context.rs (NEW)

use biome_json_syntax::AnyJsonValue;
use biome_rowan::TextRange;
use std::collections::HashMap;

/// Implementation of provenance capture
pub struct ProvenanceImpl {
    /// Stack tracking current path during deserialization
    path_stack: Vec<FieldQuerySegment>,

    /// Captured AST pointers mapped by their field path
    /// We store AstPtr instead of the actual node for thread safety
    captured_ptrs: HashMap<FieldQuery, AstPtr<AnyJsonValue>>,

    /// The configuration source being deserialized
    current_source: ProvenanceSource,

    /// Timestamp for precedence tracking
    merge_order: u64,
}

impl ProvenanceImpl {
    pub fn new(source: ProvenanceSource, merge_order: u64) -> Self {
        Self {
            path_stack: Vec::new(),
            captured_ptrs: HashMap::new(),
            current_source: source,
            merge_order,
        }
    }

    pub fn take_entries(self) -> Vec<ProvenanceEntry> {
        self.captured_ptrs
            .into_iter()
            .map(|(field_path, value_ptr)| ProvenanceEntry {
                field_path,
                source: self.current_source.clone(),
                value_ptr,
                merge_order: self.merge_order,
            })
            .collect()
    }
}

impl Provenance for ProvenanceImpl {
    fn push_field(&mut self, name: &JsonMemberName) {
        // Create thread-safe AstPtr from the syntax node
        let ptr = AstPtr::new(name);
        self.path_stack.push(FieldQuerySegment::Field(ptr));
    }

    fn push_index(&mut self, index: usize) {
        self.path_stack.push(FieldQuerySegment::Index(index));
    }

    fn pop(&mut self) {
        self.path_stack.pop();
    }

    fn current_path(&self) -> FieldQuery {
        FieldQuery {
            segments: self.path_stack.clone(),
        }
    }

    fn capture_value(&mut self, path: FieldQuery, node: &AnyJsonValue) {
        // Create a thread-safe AstPtr from the node
        let ptr = AstPtr::new(node);
        self.captured_ptrs.insert(path, ptr);
    }
}

/// Deserialization context with provenance tracking enabled
pub struct ProvenanceAwareDeserializationContext {
    base: DefaultDeserializationContext,
    capture: ProvenanceImpl,
}

impl ProvenanceAwareDeserializationContext {
    pub fn new(source: ProvenanceSource, merge_order: u64) -> Self {
        Self {
            base: DefaultDeserializationContext::new(""),
            capture: ProvenanceImpl::new(source, merge_order),
        }
    }

    pub fn take_entries(self) -> Vec<ProvenanceEntry> {
        self.capture.take_entries()
    }
}

impl DeserializationContext for ProvenanceAwareDeserializationContext {
    // Delegate standard methods to base context
    fn diagnostics(&self) -> &[Diagnostic] {
        self.base.diagnostics()
    }

    // ... other delegations ...

    fn provenance(&mut self) -> Option<&mut dyn Provenance> {
        Some(&mut self.capture)  // ✓ Provenance tracking enabled
    }
}
```

### 3.3 CaptureProvenance Derive Macro

This is an opt-in macro that wraps deserialization to capture syntax nodes:

```rust
// crates/biome_deserialize_macros/src/capture_provenance_derive.rs (NEW)

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Opt-in derive macro that captures syntax nodes during deserialization
///
/// Usage:
/// ```
/// #[derive(Deserializable, CaptureProvenance)]
/// struct OverridePattern {
///     pub includes: Option<OverrideGlobs>,
///     pub formatter: Option<OverrideFormatterConfiguration>,
/// }
/// ```
#[proc_macro_derive(CaptureProvenance)]
pub fn derive_capture_provenance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("CaptureProvenance only supports structs"),
    };

    // Generate field capture logic
    let field_captures = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            if let Some(capture) = ctx.provenance() {
                capture.push_field(#field_name_str);

                // If the field value is a leaf (not a nested object), capture it
                if let Some(value) = &self.#field_name {
                    if let Some(json_value) = /* extract AnyJsonValue for this field */ {
                        capture.capture_value(
                            capture.current_path(),
                            json_value,
                            json_value.range()
                        );
                    }
                }

                capture.pop();
            }
        }
    });

    // Note: Full implementation would integrate with the Deserializable visitor pattern
    // to capture nodes as they're being deserialized, not after

    TokenStream::from(quote! {
        // Implementation hooks into Deserializable to capture nodes during traversal
        // This is a conceptual outline - full implementation requires deeper integration
    })
}
```

**Implementation Note**: The macro would need to integrate deeply with the `Deserializable` trait and visitor pattern to capture nodes during the deserialization traversal. The exact implementation depends on how we want to hook into the existing deserialization infrastructure.

### 3.4 Simplified Merge Trait (No Node Capture)

With provenance capture moved to deserialization, the Merge trait only needs to track merge order for precedence:

```rust
// crates/biome_deserialize/src/merge.rs (NO CHANGES NEEDED)

// The existing Merge trait remains unchanged:
pub trait Merge: Sized {
    fn merge_with(&mut self, other: Self);
}

// Implementations remain as-is - no provenance tracking needed
impl<T: Merge> Merge for Option<T> {
    fn merge_with(&mut self, other: Self) {
        match (self.as_mut(), other) {
            (Some(this), Some(other)) => this.merge_with(other),
            (None, Some(other)) => *self = Some(other),
            _ => {}
        }
    }
}
```

**Key Benefits**:
- Merge trait stays simple and focused on merging logic
- No backward compatibility concerns
- All provenance capture happens at deserialization time
- Works for both merged configs (via Merge) and non-merged configs (like OverridePattern)
- Uses `JsonMemberName` syntax nodes instead of strings for field paths, providing:
  - Exact source locations for each field name
  - Preserved original formatting
  - No string allocation during path construction
  - Can extract field name text on-demand via `inner_string_text()`

## 4. Integration with Configuration Loading

### 4.1 Loading Configuration with Provenance

The key change is that provenance capture happens during deserialization, not merging:

```rust
// crates/biome_service/src/configuration.rs (MODIFY)

/// Load a configuration file with provenance tracking
pub fn load_configuration_with_provenance(
    fs: &dyn FileSystem,
    config_path: &Utf8Path,
    source: ProvenanceSource,
    merge_order: u64,
) -> Result<(Configuration, Vec<ProvenanceEntry>), WorkspaceError> {
    let content = fs.read_file_from_db(config_path)?;
    let parse = parse_json(&content, JsonParserOptions::default());

    // Create provenance-aware deserialization context
    let mut ctx = ProvenanceAwareDeserializationContext::new(source, merge_order);

    // Deserialize with provenance capture enabled
    let config: Configuration = match parse.tree().value() {
        Ok(value) => Deserializable::deserialize(&mut ctx, &value, "")
            .ok_or_else(|| WorkspaceError::DeserializationFailed)?,
        Err(_) => return Err(WorkspaceError::ParseFailed),
    };

    // Extract captured provenance entries
    let entries = ctx.take_entries();

    Ok((config, entries))
}

impl ConfigurationExt for Configuration {
    fn apply_extends(
        &mut self,
        base_path: &Utf8Path,
        base_external_path: &Utf8Path,
        fs: &dyn FileSystem,
        enable_provenance: bool,
    ) -> Result<ApplyExtendsResult, ConfigError> {
        let mut aggregated_entries = Vec::new();
        let mut override_metadata_list = Vec::new();
        let mut merge_counter = 0u64;

        // Load base config with provenance if enabled
        if enable_provenance {
            let base_source = ProvenanceSource::BaseConfig {
                path: base_path.to_path_buf(),
                loaded_location: LoadedLocation::InProject,
            };

            let (_, base_entries) = load_configuration_with_provenance(
                fs,
                base_path,
                base_source,
                merge_counter
            )?;

            aggregated_entries.extend(base_entries);
        }

        // Process extends array
        let mut accumulated_config = Configuration::default();
        let mut extended_configs_metadata = Vec::new();

        if let Some(extends) = &self.extends {
            for extend_path in extends.iter() {
                merge_counter += 1;

                let resolved_path = resolve_extend_path(base_path, extend_path)?;

                let extend_source = ProvenanceSource::ExtendedConfig {
                    path: resolved_path.clone(),
                    resolution_path: vec![base_path.to_path_buf(), resolved_path.clone()],
                };

                // Load extended config with provenance
                let (extended_config, extended_entries) = if enable_provenance {
                    load_configuration_with_provenance(
                        fs,
                        &resolved_path,
                        extend_source.clone(),
                        merge_counter
                    )?
                } else {
                    let config = load_configuration(fs, &resolved_path)?;
                    (config, Vec::new())
                };

                // Merge extended config (simple merge, no provenance tracking needed)
                accumulated_config.merge_with(extended_config.clone());

                // Collect provenance entries
                aggregated_entries.extend(extended_entries);

                // Handle overrides from extended config
                if enable_provenance {
                    if let Some(overrides) = &extended_config.overrides {
                        override_metadata_list.extend(
                            extract_override_metadata(
                                overrides,
                                extend_source,
                                &mut merge_counter,
                                &aggregated_entries  // ← Entries already captured during deserialization
                            )
                        );
                    }
                }

                extended_configs_metadata.push(ExtendedConfigMetadata {
                    path: resolved_path,
                    // ... other metadata
                });
            }
        }

        // Merge base config on top of extended configs
        merge_counter += 1;
        std::mem::swap(self, &mut accumulated_config);
        self.merge_with(accumulated_config);

        // Handle base config overrides
        if enable_provenance {
            if let Some(overrides) = &self.overrides {
                let base_source = ProvenanceSource::BaseConfig {
                    path: base_path.to_path_buf(),
                    loaded_location: LoadedLocation::InProject,
                };

                override_metadata_list.extend(
                    extract_override_metadata(
                        overrides,
                        base_source,
                        &mut merge_counter,
                        &aggregated_entries
                    )
                );
            }
        }

        Ok(ApplyExtendsResult {
            extended_configurations: extended_configs_metadata,
            diagnostics: vec![],
            provenance_entries: aggregated_entries,
            override_metadata: override_metadata_list,
        })
    }
}
```

### 4.2 Extract Override Metadata

Override metadata is extracted to enable lazy evaluation during queries. We only need to store the glob matchers and merge order:

```rust
// crates/biome_service/src/configuration.rs

/// Extract override metadata for lazy evaluation during queries
///
/// Note: We don't pre-filter entries here. The QueryVisitor will match override
/// entries on-demand when a file_path is provided.
fn extract_override_metadata(
    overrides: &Overrides,
    source: ProvenanceSource,
    merge_counter: &mut u64,
) -> Vec<OverrideProvenanceMetadata> {
    overrides.0.iter().enumerate().map(|(index, pattern)| {
        *merge_counter += 1;

        // Build glob matchers from includes
        let matchers = pattern.includes
            .as_ref()
            .map(|globs| build_glob_matchers(globs))
            .unwrap_or_default();

        OverrideProvenanceMetadata {
            source: source.clone(),
            index,  // Store the override index for matching
            matchers,
            merge_order: *merge_counter,
        }
    }).collect()
}
```

**Key Changes from Original Design**:
- **No pre-filtering**: We don't extract `field_entries` HashMap here
- **Simpler metadata**: Just store glob matchers, index, and merge_order
- **Lazy matching**: QueryVisitor will find matching entries by checking if entry's FieldQuery starts with "overrides[index]"
- **On-demand evaluation**: Override entries are only matched when file_path is provided to query
```

### 4.3 Add CaptureProvenance to Config Types

```rust
// crates/biome_configuration/src/overrides.rs (MODIFY)

// Add CaptureProvenance to OverridePattern and nested configs
#[derive(Clone, Debug, Default, Deserialize, Deserializable, CaptureProvenance, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OverridePattern {
    pub includes: Option<OverrideGlobs>,
    pub javascript: Option<JsConfiguration>,
    pub formatter: Option<OverrideFormatterConfiguration>,
    pub linter: Option<OverrideLinterConfiguration>,
    // ... other fields
}

#[derive(Clone, Debug, Deserialize, Deserializable, CaptureProvenance, Eq, PartialEq, Serialize)]
pub struct OverrideFormatterConfiguration {
    pub enabled: Option<bool>,
    pub indent_width: Option<u8>,
    // ... other fields
}

// Similarly for other nested override configs
```

**Key Improvements**:
- ✅ Syntax nodes captured during deserialization (not after)
- ✅ Works for both merged configs and overrides
- ✅ No complex extraction logic needed
- ✅ Override provenance problem solved elegantly

## 5. Storage and Query API

### 5.1 ProvenanceIndex (Storage)

```rust
// crates/biome_service/src/workspace/provenance.rs (NEW)

/// Complete provenance index for a loaded configuration
/// Stored in Settings, contains AstPtr (thread-safe) pointers to value nodes
#[derive(Debug)]
pub struct ProvenanceIndex {
    /// Flat list of all provenance entries, sorted by merge_order
    /// No HashMap needed - queries do linear search with on-demand field path resolution
    /// Typical configs have 100-300 entries, so O(n) is fast enough for user-initiated queries
    pub(crate) entries: Vec<ProvenanceEntry>,

    /// Override metadata for lazy file-specific evaluation
    pub(crate) override_metadata: Vec<OverrideProvenanceMetadata>,

    /// The base configuration file path
    pub(crate) base_config_path: Option<Utf8PathBuf>,
}

impl ProvenanceIndex {
    pub fn build(
        mut entries: Vec<ProvenanceEntry>,
        override_metadata: Vec<OverrideProvenanceMetadata>,
        base_config_path: Option<Utf8PathBuf>,
    ) -> Self {
        // Sort entries by merge_order for consistent query results
        entries.sort_by_key(|e| e.merge_order);

        Self {
            entries,
            override_metadata,
            base_config_path,
        }
    }
}

```

**Note**: Since we return only the winning `ProvenanceEntry` (not a full chain), there's no need for separate `ProvenanceChain` or `ProvenanceChainEntry` types. The query simply returns `Option<ProvenanceEntry>`, and the caller resolves the AstPtr to get the value and location when needed.

### 5.2 Integration with Workspace and Settings

This section shows how provenance information flows from configuration loading through to the query API.

#### 5.2.1 Configuration Loading with Provenance

```rust
// crates/biome_service/src/configuration.rs (MODIFY)

/// Result of loading a configuration with provenance tracking
pub struct LoadedConfigurationWithProvenance {
    pub configuration: Configuration,
    pub provenance_index: ProvenanceIndex,
    pub diagnostics: Vec<Diagnostic>,
}

/// Load configuration and build provenance index
pub fn load_configuration_with_provenance(
    fs: &dyn FileSystem,
    base_path: &Utf8Path,
    enable_provenance: bool,
) -> Result<LoadedConfigurationWithProvenance, WorkspaceError> {
    if !enable_provenance {
        // Fast path: no provenance tracking
        let configuration = load_configuration(fs, base_path)?;
        return Ok(LoadedConfigurationWithProvenance {
            configuration,
            provenance_index: ProvenanceIndex::empty(),
            diagnostics: vec![],
        });
    }

    // 1. Load base config with provenance capture
    let mut merge_counter = 0u64;
    let base_source = ProvenanceSource::BaseConfig {
        path: base_path.to_path_buf(),
        loaded_location: LoadedLocation::InProject,
    };

    let (mut configuration, mut provenance_entries) =
        load_single_config_with_provenance(fs, base_path, base_source.clone(), merge_counter)?;

    // 2. Apply extends and collect provenance
    let extends_result = configuration.apply_extends(
        base_path,
        base_path,
        fs,
        true, // enable_provenance
    )?;

    provenance_entries.extend(extends_result.provenance_entries);
    let override_metadata = extends_result.override_metadata;

    // 3. Load editorconfig if present
    if let Some((editor_config, editor_entries)) = load_editorconfig_with_provenance(fs, base_path)? {
        merge_counter += 1;
        configuration.merge_with(editor_config);
        provenance_entries.extend(editor_entries);
    }

    // 4. Build ProvenanceIndex
    let provenance_index = ProvenanceIndex::build(
        provenance_entries,
        override_metadata,
        Some(base_path.to_path_buf()),
    );

    Ok(LoadedConfigurationWithProvenance {
        configuration,
        provenance_index,
        diagnostics: extends_result.diagnostics,
    })
}

/// Helper: Load a single config file with provenance
fn load_single_config_with_provenance(
    fs: &dyn FileSystem,
    config_path: &Utf8Path,
    source: ProvenanceSource,
    merge_order: u64,
) -> Result<(Configuration, Vec<ProvenanceEntry>), WorkspaceError> {
    let content = fs.read_file_from_db(config_path)?;
    let parse = parse_json(&content, JsonParserOptions::default());

    // Create provenance-aware deserialization context
    let mut ctx = ProvenanceAwareDeserializationContext::new(source, merge_order);

    // Deserialize with provenance capture enabled
    let config: Configuration = match parse.tree().value() {
        Ok(value) => Deserializable::deserialize(&mut ctx, &value, "")
            .ok_or_else(|| WorkspaceError::DeserializationFailed)?,
        Err(_) => return Err(WorkspaceError::ParseFailed),
    };

    // Extract captured provenance entries
    let entries = ctx.take_entries();

    Ok((config, entries))
}
```

#### 5.2.2 Settings Integration

Each project in Biome's workspace has its own `Settings` instance, which may contain provenance information:

```rust
// crates/biome_service/src/settings.rs (MODIFY)

use std::sync::Arc;
use biome_configuration::provenance::{ProvenanceIndex, ProvenanceError};

pub struct Settings {
    // ... existing fields like `linter_enabled`, `formatter`, etc. ...

    /// Provenance information, if tracking was enabled during config load
    /// Wrapped in Arc for cheap cloning across workspace operations
    ///
    /// Note: Each project (identified by ProjectKey) has its own Settings instance,
    /// and thus its own provenance index. In monorepos, nested configs also have
    /// their own Settings with separate provenance tracking.
    provenance: Option<Arc<ProvenanceIndex>>,
}

impl Settings {
    /// Create new Settings from loaded configuration
    pub fn from_loaded_config(
        configuration: Configuration,
        provenance_index: Option<ProvenanceIndex>,
        // ... other parameters
    ) -> Self {
        Self {
            // ... populate other fields from configuration ...
            provenance: provenance_index.map(Arc::new),
        }
    }

    /// Get provenance index if available
    ///
    /// Note: Direct querying is done via Workspace::query_configuration_provenance()
    /// which handles AstPtr resolution using get_parse(). This method only provides
    /// access to the raw provenance index.
    pub fn provenance(&self) -> Option<&ProvenanceIndex> {
        self.provenance.as_deref()
    }

    /// Check if provenance tracking is enabled
    pub fn has_provenance(&self) -> bool {
        self.provenance.is_some()
    }
}
```

**Multi-Project Architecture Notes**:
- Each `ProjectKey` maps to a `ProjectData` which contains `root_settings: Settings`
- Monorepos may have `nested_settings: BTreeMap<Utf8PathBuf, Settings>` for nested configs
- Each Settings instance (root or nested) has its own `Option<Arc<ProvenanceIndex>>`
- Provenance queries must specify which project (via `project_key`) to query

#### 5.2.3 Workspace API

The Workspace manages multiple projects, each identified by a `ProjectKey`. Provenance queries must specify which project to query:

```rust
// crates/biome_service/src/workspace.rs (MODIFY)

pub trait Workspace {
    // ... existing methods ...

    /// Query configuration provenance for a specific project
    ///
    /// Returns the winning configuration source that set a specific field value.
    /// Only the final/winning entry is returned (highest merge_order = last applied).
    /// Requires that the workspace was initialized with provenance tracking enabled.
    ///
    /// # Arguments
    /// * `params` - Query parameters including project key, field path, and optional file path
    ///
    /// # Returns
    /// * `Ok(Some(ProvenanceEntry))` - The winning provenance entry for the field
    /// * `Ok(None)` - No matching entries found (field not configured)
    /// * `Err(WorkspaceError)` - If provenance is not enabled or project not found
    fn query_configuration_provenance(
        &self,
        params: QueryProvenanceParams,
    ) -> Result<Option<ProvenanceEntry>, WorkspaceError>;
}

/// Parameters for querying configuration provenance
#[derive(Debug)]
pub struct QueryProvenanceParams {
    /// The project to query
    /// Required because workspace manages multiple projects
    pub project_key: ProjectKey,

    /// Field path to query (e.g., "formatter.indentWidth")
    pub field_query: String,

    /// Optional file path for override resolution
    /// If provided:
    /// 1. Overrides matching this file will be included in the chain
    /// 2. In monorepos, this determines which Settings to use (root vs nested)
    pub file_path: Option<Utf8PathBuf>,
}

/// Implementation for the workspace server using simple query_provenance function
impl Workspace for WorkspaceServer {
    fn query_configuration_provenance(
        &self,
        params: QueryProvenanceParams,
    ) -> Result<Option<ProvenanceEntry>, WorkspaceError> {
        // 1. Get the Projects manager
        let projects = self.projects();

        // 2. Select the appropriate Settings for this project and file path
        let settings = if let Some(file_path) = &params.file_path {
            // Use get_settings_based_on_path to select between root_settings and nested_settings
            projects
                .get_settings_based_on_path(params.project_key, file_path)
                .ok_or_else(|| WorkspaceError::ProjectNotFound {
                    project_key: params.project_key,
                })?
        } else {
            // No file path provided, use root settings
            projects
                .get_root_settings(params.project_key)
                .ok_or_else(|| WorkspaceError::ProjectNotFound {
                    project_key: params.project_key,
                })?
        };

        // 3. Get provenance index from settings
        let provenance_index = settings
            .provenance()
            .ok_or(WorkspaceError::ProvenanceNotEnabled)?;

        // 4. Get the JsonRoot for the base config file
        // We need this to resolve AstPtr in field_path_matches
        let base_config_path = provenance_index.base_config_path
            .as_ref()
            .ok_or(WorkspaceError::ProvenanceNotEnabled)?;

        let parse = self.get_parse(base_config_path)?;
        let json_root = match parse {
            AnyParse::Node(node_parse) => {
                node_parse.root.to_language_root::<JsonRoot>()
                    .ok_or_else(|| WorkspaceError::InvalidParse {
                        path: base_config_path.clone(),
                    })?
            }
            _ => return Err(WorkspaceError::InvalidParse {
                path: base_config_path.clone(),
            }),
        };

        // 5. Query provenance using simple filtering function
        let result = query_provenance(
            &params.field_query,
            &provenance_index,
            &json_root,
            params.file_path.as_deref(),
        );

        Ok(result)
    }
}
```

**Key Implementation Details**:

1. **Simple filtering**: Iterate through provenance entries and filter by matching field_path
2. **Conditional override handling**: Skip override entries when file_path not provided
3. **Last wins**: Sort by merge_order and return the highest (most recently applied)
4. **Single JsonRoot**: Only loads the base config file's root for resolving AstPtr
5. **Optional return**: Returns `Option<ProvenanceEntry>` - None if field not configured
```

**Key Points**:
1. **ProjectKey is required**: All workspace operations need to know which project to operate on
2. **Settings selection**: `Projects.get_settings_based_on_path()` automatically selects between:
   - `root_settings`: For files in the project root or when no file path is provided
   - `nested_settings`: For files in nested config directories (monorepos)
3. **Monorepo support**: Each nested config has its own Settings and ProvenanceIndex

#### 5.2.4 Enabling Provenance Tracking

Provenance tracking is controlled via workspace initialization:

```rust
// crates/biome_service/src/workspace/server.rs

pub struct WorkspaceSettings {
    // ... existing settings ...

    /// Enable configuration provenance tracking
    /// Default: false (disabled for performance)
    pub enable_provenance: bool,
}

impl WorkspaceServer {
    pub fn new(settings: WorkspaceSettings) -> Self {
        // When loading configuration, check enable_provenance flag
        let loaded_config = if settings.enable_provenance {
            load_configuration_with_provenance(&fs, &config_path, true)?
        } else {
            // Standard loading without provenance
            let configuration = load_configuration(&fs, &config_path)?;
            LoadedConfigurationWithProvenance {
                configuration,
                provenance_index: ProvenanceIndex::empty(),
                diagnostics: vec![],
            }
        };

        let settings = Settings::from_loaded_config(
            loaded_config.configuration,
            if settings.enable_provenance {
                Some(loaded_config.provenance_index)
            } else {
                None
            },
        );

        Self {
            settings: Arc::new(RwLock::new(settings)),
            // ... other fields ...
        }
    }
}
```

#### 5.2.5 Complete Query Flow with Multi-Project Support

```
1. User/Tool wants to know provenance
   └─> CLI: biome explain config formatter.indentWidth
   └─> LSP: Hover over config value in biome.json

2. Determine ProjectKey (required for multi-project workspace)
   CLI approach:
   └─> Current working directory → find project by path
   └─> projects.find_project_for_path(cwd) → project_key

   LSP approach:
   └─> Config file URI → extract path
   └─> projects.find_project_for_path(config_file_path) → project_key

3. Call Workspace API with project_key
   └─> workspace.query_configuration_provenance(QueryProvenanceParams {
           project_key: project_key,  // ← Required for multi-project
           field_query: "formatter.indentWidth",
           file_path: Some("src/main.js"),  // ← Optional, for override resolution
       })

4. Workspace selects appropriate Settings
   a. Get Projects manager
   b. If file_path provided:
      └─> projects.get_settings_based_on_path(project_key, file_path)
          → Checks nested_settings first, falls back to root_settings
   c. If no file_path:
      └─> projects.get_root_settings(project_key)

5. Workspace delegates to query_provenance function
   └─> query_provenance("formatter.indentWidth", provenance_index, json_root, Some("src/main.js"))

6. query_provenance finds the winning entry
   a. Parse field path into segments: ["formatter", "indentWidth"]
   b. Filter all entries by matching field_path
   c. If file_path provided, evaluate overrides:
      - Skip override entries that don't match file_path
   d. If no file_path provided, skip ALL override entries
   e. Sort remaining entries by merge_order
   f. Return the last entry (highest merge_order = winner)

7. Return Option<ProvenanceEntry>
   └─> Returns the single winning entry, or None if field not configured
   └─> Caller resolves AstPtr to get value and location
```

**Monorepo Example Flow**:
```
Project structure:
/workspace
  ├─ biome.json (root config)
  ├─ packages/
  │   ├─ app/
  │   │   ├─ biome.json (nested config)
  │   │   └─ src/main.js
  │   └─ lib/
  │       └─ src/util.js

Query for packages/app/src/main.js:
1. project_key = find_project_for_path("/workspace") → ProjectKey(1)
2. file_path = "/workspace/packages/app/src/main.js"
3. Projects.get_settings_based_on_path(ProjectKey(1), file_path):
   - Checks nested_settings for "/workspace/packages/app" → Found!
   - Returns Settings with provenance from packages/app/biome.json
4. Query provenance from nested Settings
5. Result includes config chain from packages/app/biome.json + its extends

Query for packages/lib/src/util.js:
1. project_key = ProjectKey(1)
2. file_path = "/workspace/packages/lib/src/util.js"
3. Projects.get_settings_based_on_path(ProjectKey(1), file_path):
   - Checks nested_settings for "/workspace/packages/lib" → Not found
   - Falls back to root_settings
4. Query provenance from root Settings
5. Result includes config chain from root biome.json
```

## 6. CLI Command Implementation

### 6.1 Diagnostic Structs

```rust
// crates/biome_service/src/workspace/provenance.rs (ADD)

use biome_diagnostics::{Diagnostic, Location, category, Category, Severity};
use biome_console::{markup, fmt};
use biome_rowan::TextRange;
use serde::{Deserialize, Serialize};

/// Diagnostic for displaying a provenance entry
#[derive(Debug)]
pub struct ProvenanceEntryDiagnostic {
    /// File path where this value was set
    #[location(resource)]
    pub file_path: String,

    /// Text range in the file
    #[location(span)]
    pub span: TextRange,

    /// Source code of the file
    #[location(source_code)]
    pub source_code: String,

    /// Source that set this value
    pub source: ProvenanceSource,

    /// The value that was set
    pub value: String,
}

impl Diagnostic for ProvenanceEntryDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("configuration"))
    }

    fn severity(&self) -> Severity {
        Severity::Information
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup! {
            {self.source}" set it to "<Emphasis>{self.value}</Emphasis>
        })
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{} set it to {}", self.source, self.value)
    }
}

impl ProvenanceEntry {
    /// Convert entry to diagnostic for display
    ///
    /// Resolves the AstPtr to extract value and loads source code for code frame display.
    pub fn to_diagnostic(&self, json_root: &JsonRoot, fs: &dyn FileSystem) -> Result<ProvenanceEntryDiagnostic, WorkspaceError> {
        // Resolve AstPtr to get value
        let value_node = self.value_node.to_node(json_root.syntax());
        let value_text = extract_value_text(&value_node);
        let value_range = value_node.range();

        // Get file path from source
        let file_path = self.source.config_path();

        // Load source code for code frame display
        let source_code = fs.read_file_from_db(file_path)?;

        Ok(ProvenanceEntryDiagnostic {
            file_path: file_path.to_string(),
            span: value_range,
            source_code,
            source: self.source.clone(),
            value: value_text,
        })
    }
}

impl ProvenanceSource {
    /// Get the config file path for this source
    /// This is the actual configuration file containing the value
    pub fn config_path(&self) -> &Utf8Path {
        match self {
            ProvenanceSource::BaseConfig { path, .. } => path,
            ProvenanceSource::ExtendedConfig { path, .. } => path,
            ProvenanceSource::EditorConfig { path, .. } => path,
            ProvenanceSource::Override { container_source, .. } => container_source.config_path(),
        }
    }
}

/// Display implementation for CLI (rich formatting with markup)
impl biome_console::fmt::Display for ProvenanceSource {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            ProvenanceSource::BaseConfig { path, .. } => {
                let file_name = path.file_name().unwrap_or("biome.json");
                fmt.write_markup(markup! {
                    "base config ("<Emphasis>{file_name}</Emphasis>")"
                })
            }
            ProvenanceSource::ExtendedConfig { path, resolution_path, .. } => {
                let file_name = path.file_name().unwrap_or("");
                if resolution_path.len() > 2 {
                    let chain: Vec<_> = resolution_path.iter()
                        .take(resolution_path.len() - 1)
                        .map(|p| p.file_name().unwrap_or(""))
                        .collect();
                    let chain_str = chain.join(" → ");
                    fmt.write_markup(markup! {
                        "extended config via "<Emphasis>{chain_str}</Emphasis>" → "<Emphasis>{file_name}</Emphasis>
                    })
                } else {
                    fmt.write_markup(markup! {
                        "extended config ("<Emphasis>{file_name}</Emphasis>")"
                    })
                }
            }
            ProvenanceSource::EditorConfig { path, .. } => {
                let file_name = path.file_name().unwrap_or(".editorconfig");
                fmt.write_markup(markup! {
                    ".editorconfig ("<Emphasis>{file_name}</Emphasis>")"
                })
            }
            ProvenanceSource::Override { config_source, includes, .. } => {
                let file_name = config_source.config_path().file_name().unwrap_or("config");
                let patterns = includes.join(", ");
                fmt.write_markup(markup! {
                    "override in "<Emphasis>{file_name}</Emphasis>" matching "<Emphasis>{patterns}</Emphasis>
                })
            }
        }
    }
}

/// Display implementation for LSP (plain text)
impl std::fmt::Display for ProvenanceSource {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvenanceSource::BaseConfig { path, .. } => {
                let file_name = path.file_name().unwrap_or("biome.json");
                write!(fmt, "base config ({})", file_name)
            }
            ProvenanceSource::ExtendedConfig { path, resolution_path, .. } => {
                let file_name = path.file_name().unwrap_or("");
                if resolution_path.len() > 2 {
                    let chain: Vec<_> = resolution_path.iter()
                        .take(resolution_path.len() - 1)
                        .map(|p| p.file_name().unwrap_or(""))
                        .collect();
                    write!(fmt, "extended config via {} → {}", chain.join(" → "), file_name)
                } else {
                    write!(fmt, "extended config ({})", file_name)
                }
            }
            ProvenanceSource::EditorConfig { path, .. } => {
                let file_name = path.file_name().unwrap_or(".editorconfig");
                write!(fmt, ".editorconfig ({})", file_name)
            }
            ProvenanceSource::Override { config_source, includes, .. } => {
                let file_name = config_source.config_path().file_name().unwrap_or("config");
                write!(fmt, "override in {} matching {}", file_name, includes.join(", "))
            }
        }
    }
}
```

### 6.2 CLI Command

The `biome explain config` command is implemented as a subcommand that uses a custom scanner to crawl only configuration files in the project.

#### 6.2.1 Doc Enum Update

```rust
// crates/biome_service/src/documentation/mod.rs (UPDATE)

use camino::Utf8PathBuf;

#[derive(Debug, Clone)]
pub enum Doc {
    Rule(ExplainRule),

    /// Configuration field query - explain where a config value comes from
    Config {
        field_query: String,
        file_path: Option<Utf8PathBuf>,
    },

    DaemonLogs,
    Unknown(String),
}
```

#### 6.2.2 ExplainSubcommand Enum

```rust
// crates/biome_cli/src/commands/mod.rs (ADD)

#[derive(Debug, Clone, Bpaf)]
pub enum ExplainSubcommand {
    /// Explain where a configuration field value comes from
    ///
    /// Shows the complete chain of configuration sources (base config, extends, overrides)
    /// that contribute to the final value of a field.
    ///
    /// Examples:
    ///   biome explain config formatter.indentWidth
    ///   biome explain config linter.enabled --file src/test.js
    #[bpaf(command("config"))]
    Config {
        /// Field path to query (e.g. "formatter.indentWidth", "linter.enabled")
        #[bpaf(positional("FIELD_QUERY"))]
        field_query: String,

        /// Optional file path to query for (includes override evaluation)
        #[bpaf(long("file"), short('f'), argument("PATH"))]
        file: Option<Utf8PathBuf>,
    },

    /// Explain a lint rule or assist action (existing functionality)
    #[bpaf(command("rule"))]
    Rule {
        #[bpaf(positional("NAME"))]
        name: String,
    },

    /// Show daemon logs location (existing functionality)
    #[bpaf(command("daemon-logs"))]
    DaemonLogs,
}
```

#### 6.2.3 BiomeCommand::Explain Refactoring

```rust
// crates/biome_cli/src/commands/mod.rs (UPDATE)

#[derive(Debug, Bpaf, Clone)]
pub enum BiomeCommand {
    // ... other commands ...

    /// Shows documentation for a lint rule, configuration field, or other Biome features
    ///
    /// Examples:
    /// ```shell
    /// # Explain a configuration field
    /// biome explain config formatter.indentWidth
    ///
    /// # Explain for a specific file (includes overrides)
    /// biome explain config linter.enabled --file src/test.js
    ///
    /// # Explain a lint rule
    /// biome explain rule noDebugger
    ///
    /// # Show daemon logs location
    /// biome explain daemon-logs
    /// ```
    #[bpaf(command)]
    Explain {
        #[bpaf(external(explain_subcommand))]
        subcommand: ExplainSubcommand,
    },

    // ... other commands ...
}
```

#### 6.2.4 ScanKind::ConfigFiles Variant

```rust
// crates/biome_service/src/scanner.rs (UPDATE)

#[derive(Debug, Clone)]
pub enum ScanKind {
    /// Don't scan any files
    NoScanner,

    /// Scan all known file types (default)
    KnownFiles,

    /// Scan only specific target paths
    TargetedKnownFiles {
        target_paths: Vec<BiomePath>,
        // ...
    },

    /// Scan only configuration files (biome.json, biome.jsonc, .editorconfig)
    /// Used by `biome explain config` command to crawl project configs
    ConfigFiles,

    /// Scan entire project structure
    Project,

    /// Type-aware scanning
    TypeAware,
}
```

#### 6.2.5 Scanner Config File Filtering Implementation

```rust
// crates/biome_service/src/scanner.rs (UPDATE)

impl Scanner {
    fn should_process_file(&self, path: &BiomePath) -> bool {
        match &self.kind {
            ScanKind::ConfigFiles => {
                // Only process configuration files
                if let Some(file_name) = path.file_name() {
                    matches!(
                        file_name,
                        "biome.json" | "biome.jsonc" | ".editorconfig"
                    )
                } else {
                    false
                }
            }
            ScanKind::KnownFiles => {
                // Existing logic for known file types
                // ...
            }
            // ... other variants ...
        }
    }
}
```

#### 6.2.6 Explain Command Dispatcher

```rust
// crates/biome_cli/src/commands/explain.rs (UPDATE)

use biome_service::documentation::Doc;
use crate::commands::ExplainSubcommand;

pub(crate) fn explain(
    session: CliSession,
    subcommand: ExplainSubcommand,
) -> Result<(), CliDiagnostic> {
    match subcommand {
        ExplainSubcommand::Config { field_query, file } => {
            let doc = Doc::Config {
                field_query,
                file_path: file,
            };
            explain_config(session, doc)
        }

        ExplainSubcommand::Rule { name } => {
            // Parse rule name and call existing print_rule logic
            let doc = Doc::from_str(&name)?;
            match doc {
                Doc::Rule(explain_rule) => print_rule(session, &explain_rule),
                Doc::Unknown(arg) => {
                    Err(CliDiagnostic::unexpected_argument(arg, "explain rule"))
                }
                _ => unreachable!(),
            }
        }

        ExplainSubcommand::DaemonLogs => {
            let doc = Doc::DaemonLogs;
            // Show daemon logs location (existing logic)
            show_daemon_logs(session)
        }
    }
}
```

#### 6.2.7 explain_config Implementation

```rust
// crates/biome_cli/src/commands/explain.rs (ADD)

use biome_cli_diagnostics::CliDiagnostic;
use biome_console::markup;
use biome_diagnostics::PrintDiagnostic;
use biome_service::documentation::Doc;
use biome_service::workspace::{QueryProvenanceParams, ScanKind};
use crate::CliSession;

fn explain_config(
    mut session: CliSession,
    doc: Doc,
) -> Result<(), CliDiagnostic> {
    let Doc::Config { field_query, file_path } = doc else {
        unreachable!("explain_config called with non-Config doc");
    };

    // Step 1: Use ScanKind::ConfigFiles to crawl only configuration files
    // This ensures the workspace has loaded all config files in the project
    session.app.traverse(
        ScanKind::ConfigFiles,
        |signal| {
            // Just scan, don't process - we only need configs loaded
            Ok(signal)
        },
    )?;

    // Step 2: Determine project key from file path or current directory
    let project_key = if let Some(ref path) = file_path {
        session.app.workspace.find_project_for_path(path)?
    } else {
        let cwd = std::env::current_dir()
            .map_err(|e| CliDiagnostic::io_error(e))?;
        session.app.workspace.find_project_for_path(&cwd)?
    };

    // Step 3: Query provenance chain from workspace
    // Workspace::query_configuration_provenance() resolves AstPtr using get_parse()
    let chain = session.app.workspace.query_configuration_provenance(
        QueryProvenanceParams {
            project_key,
            field_query: field_query.clone(),
            file_path: file_path.clone(),
        }
    )?;

    // Step 4: Display final value summary
    session.app.console.log(markup! {
        <Info>"Configuration field: "</Info><Emphasis>{field_query}</Emphasis>"\n"
    });

    if let Some(ref path) = file_path {
        session.app.console.log(markup! {
            <Info>"For file: "</Info><Emphasis>{path}</Emphasis>"\n"
        });
    }

    session.app.console.log(markup! {
        <Info>"Final value: "</Info><Success>{chain.final_value}</Success>"\n\n"
    });

    // Step 5: Convert chain to diagnostics and display
    let diagnostics = chain.to_diagnostics(&*session.app.fs)?;

    session.app.console.log(markup! {
        <Emphasis>"Configuration chain (in merge order):"</Emphasis>"\n\n"
    });

    for (idx, diagnostic) in diagnostics.iter().enumerate() {
        session.app.console.log(markup! {
            <Info>{format!("{}. ", idx + 1)}</Info>
        });

        // Print diagnostic with code frame showing exact location
        session.app.console.log(markup! {
            {PrintDiagnostic::simple(diagnostic)}
        });

        session.app.console.log(markup! { "\n" });
    }

    Ok(())
}
```

#### 6.2.8 CLI Usage Examples

```bash
# Query a configuration field (global)
biome explain config formatter.indentWidth

# Query for a specific file (includes override evaluation)
biome explain config linter.enabled --file src/test.js

# Query nested configuration
biome explain config javascript.formatter.quoteStyle

# Query array-indexed configuration
biome explain config overrides[0].include

# Short form of --file flag
biome explain config linter.enabled -f src/test.js
```

#### 6.2.9 Example Output

```
Configuration field: formatter.indentWidth
Final value: 2

Configuration chain (in merge order):

1. base config (biome.json) set it to 4
   ┌─ /project/biome.json:3:20
   │
 3 │   "indentWidth": 4
   │                  ^

2. extended config (prettier-compat.json) set it to 2 ← FINAL VALUE
   ┌─ /project/configs/prettier-compat.json:2:20
   │
 2 │   "indentWidth": 2
   │                  ^
```

## 7. LSP Integration

### 7.1 Hover Request Handler

The hover feature uses Biome's existing `LineIndex` infrastructure for Position↔TextRange conversion:

```rust
// crates/biome_lsp/src/handlers/hover.rs (NEW or MODIFY)

use biome_line_index::LineIndex;
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_json_syntax::JsonRoot;
use biome_rowan::AstNode;
use tower_lsp_server::ls_types::{self as lsp, Hover, HoverContents, MarkupContent, MarkupKind};

/// Handle textDocument/hover request for configuration files
pub async fn hover(
    session: &Session,
    params: lsp::HoverParams,
) -> LspResult<Option<Hover>> {
    let uri = params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    // Only handle hover for config files (biome.json, biome.jsonc)
    if !is_config_file(&uri) {
        return Ok(None);
    }

    // 1. Get file content and parse
    let source = session.get_file_content(&uri)?;
    let parse_result = parse_json(&source, JsonParserOptions::default());
    let root = parse_result.tree();

    // 2. Get LineIndex for Position↔TextRange conversion
    let line_index = LineIndex::new(&source);

    // 3. Convert LSP Position → TextOffset
    let text_offset = line_index.offset(lsp::Position {
        line: position.line,
        character: position.character,
    })?;

    // 4. Find the JSON node at the cursor position
    let node_at_cursor = root.syntax().token_at_offset(text_offset).right_biased()?;
    let json_node = node_at_cursor.parent_ancestors()
        .find_map(|node| AnyJsonValue::cast(node))?;

    // 5. Extract the field path as a string by traversing up the AST
    let field_query_string = extract_field_path_string_from_cursor(&root, &json_node)?;

    // 6. Determine project key from config file URI
    let config_path = uri_to_path(&uri)?;
    let project_key = session.workspace().find_project_for_path(&config_path)?;

    // 7. Query provenance from workspace using string query
    let entry = session.workspace().query_configuration_provenance(QueryProvenanceParams {
        project_key,
        field_query: field_query_string.clone(),
        file_path: None,  // LSP hover for generic config, not file-specific
    })?;

    // If no provenance entry found, no hover info
    let entry = match entry {
        Some(e) => e,
        None => return Ok(None),
    };

    // 8. Resolve value from AstPtr
    let value_node = entry.value_node.to_node(root.syntax());
    let value_text = extract_value_text(&value_node);
    let value_range = value_node.range();

    // 9. Format provenance information as Markdown
    let markdown = format!(
        "### Configuration: `{}`\n\n**Value**: `{}`\n\n**Source**: {}",
        field_query_string,
        value_text,
        format_provenance_source(&entry.source)
    );

    // 10. Convert TextRange to LSP Range for highlighting
    let lsp_range = line_index.to_proto_range(value_range);

    Ok(Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: markdown,
        }),
        range: Some(lsp_range),
    }))
}

/// Extract the field path as a string from cursor position by traversing the AST
///
/// Example: cursor on the value `4` in `{"formatter": {"indentWidth": 4}}`
/// Returns: "formatter.indentWidth"
fn extract_field_path_string_from_cursor(
    root: &JsonRoot,
    node: &AnyJsonValue,
) -> Option<String> {
    let mut segments = Vec::new();
    let mut current_node = node.syntax().clone();

    // Traverse upwards, collecting field names and array indices
    while let Some(parent) = current_node.parent() {
        if let Some(member) = JsonMember::cast(parent.clone()) {
            if let Ok(name) = member.name() {
                if let AnyJsonMemberName::JsonMemberName(name_node) = name {
                    // Extract the field name as a string
                    if let Ok(field_name) = name_node.inner_string_text() {
                        segments.push(FieldPathSegment::Field(field_name.text().to_string()));
                    }
                }
            }
        } else if let Some(array) = JsonArrayValue::cast(parent.clone()) {
            // Find the index of this element in the array
            if let Some(index) = find_array_index(&array, &current_node) {
                segments.push(FieldPathSegment::Index(index));
            }
        }

        current_node = parent;
    }

    // Reverse because we collected from child to root
    segments.reverse();

    // Build the string representation
    let mut result = String::new();
    for (i, segment) in segments.iter().enumerate() {
        match segment {
            FieldPathSegment::Field(name) => {
                if i > 0 {
                    result.push('.');
                }
                result.push_str(name);
            }
            FieldPathSegment::Index(idx) => {
                result.push('[');
                result.push_str(&idx.to_string());
                result.push(']');
            }
        }
    }

    Some(result)
}

/// Helper enum for building field path string
enum FieldPathSegment {
    Field(String),
    Index(usize),
}
```

### 7.2 Key Points

1. **No Pre-Computed Locations**: We don't store `SourceLocation` with line/column. Instead:
   - Syntax nodes contain `TextRange` (byte offsets)
   - `LineIndex` converts `TextRange` ↔ `lsp::Range` on demand

2. **Position Conversion Flow**:
   ```
   User hovers → LSP Position → LineIndex.offset() → TextOffset
   → find AST node → extract FieldQuery → query provenance
   → get value_node.range() → LineIndex.to_proto_range() → LSP Range
   ```

3. **Memory Efficient**: Only 8 bytes per `TextRange` stored, all conversions happen on-demand during hover

4. **Reuses Biome Infrastructure**: `LineIndex` is already used throughout Biome for LSP operations

## 8. Implementation Phases

### Phase 1: Core Infrastructure
1. Create provenance data structures (FieldQuery, ProvenanceSource, ProvenanceEntry)
2. Extend `DeserializationContext` trait with `provenance()` method
3. Implement `Provenance` trait
4. Create `ProvenanceAwareDeserializationContext`

**Deliverable**: Core provenance infrastructure ready for capture

**Note**: No `SourceLocation` struct needed - we use `TextRange` from syntax nodes and convert to line/column on-demand via `LineIndex`

### Phase 2: CaptureProvenance Derive Macro
1. Create new `capture_provenance_derive.rs` proc macro
2. Integrate with `Deserializable` visitor pattern to capture syntax nodes
3. Add field path tracking during deserialization traversal
4. Test with simple configuration structs

**Deliverable**: Working `#[derive(CaptureProvenance)]` macro

### Phase 3: Configuration Loading Integration
1. Create `load_configuration_with_provenance()` function
2. Modify `apply_extends` to use provenance-aware loading
3. Track merge order using counter
4. Collect provenance entries from each loaded config
5. Add `#[derive(CaptureProvenance)]` to `Configuration`, `OverridePattern`, and nested config types

**Deliverable**: Provenance tracked for base + extended configs + overrides

### Phase 4: Query API & Storage
1. Implement ProvenanceIndex with HashMap storage
2. Create query method with lazy override evaluation
3. Integrate with Settings struct
4. Add serialization support

**Deliverable**: Working query API accessible from Workspace

### Phase 5: CLI & LSP
1. Implement `biome explain config` command
2. Create LSP hover provider for config files
3. Add documentation

**Deliverable**: User-facing features complete

### Phase 6: EditorConfig & Polish
1. Add editorconfig source tracking
2. Capture source locations during deserialization
3. Performance optimization
4. Comprehensive testing

**Deliverable**: Complete, production-ready feature

## 9. Example Scenarios

### Scenario 1: Simple Extends Override

**Configuration**:
```json
// biome.json
{
  "extends": ["base.json", "react.json"]
}

// base.json
{
  "formatter": {
    "indentStyle": "space"
  },
  "linter": {
    "enabled": false
  }
}

// react.json
{
  "formatter": {
    "quoteStyle": "single"
  },
  "linter": {
    "enabled": true
  }
}
```

**Query**: `biome explain config linter.enabled`

**Output**:
```
Final value for linter.enabled: true

Configuration chain (in merge order):
  1. /project/configs/base.json:5:5 configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × extended config (base.json) set it to false

    >  5 │     "enabled": false
         │     ^^^^^^^^^^^^^^^^

  2. /project/configs/react.json:4:5 configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × extended config (react.json) set it to true ← FINAL VALUE

    >  4 │     "enabled": true
         │     ^^^^^^^^^^^^^^^
```

### Scenario 2: Override for Specific Files

**Configuration**:
```json
// biome.json
{
  "linter": {
    "enabled": true
  },
  "overrides": [
    {
      "includes": ["*.test.js"],
      "linter": {
        "enabled": false
      }
    }
  ]
}
```

**Query 1**: `biome explain config linter.enabled`
```
Final value for linter.enabled: true

Configuration chain (in merge order):
  1. /project/biome.json:2:5 configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × base config (biome.json) set it to true ← FINAL VALUE

    >  2 │     "enabled": true
         │     ^^^^^^^^^^^^^^^
```

**Query 2**: `biome explain config linter.enabled --file src/utils.test.js`
```
Final value for linter.enabled: false

Configuration chain (in merge order):
  1. /project/biome.json:2:5 configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × base config (biome.json) set it to true

    >  2 │     "enabled": true
         │     ^^^^^^^^^^^^^^^

  2. /project/biome.json:7:9 configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × override in base config (biome.json) matching *.test.js set it to false ← FINAL VALUE

    >  7 │         "enabled": false
         │         ^^^^^^^^^^^^^^^^
```

### Scenario 3: Nested Extends with Overrides

**Configuration**:
```json
// biome.json
{
  "extends": ["base.json"]
}

// base.json
{
  "extends": ["@company/biome-config"],
  "overrides": [
    {
      "includes": ["*.test.js"],
      "linter": { "enabled": false }
    }
  ]
}

// node_modules/@company/biome-config/biome.json
{
  "linter": { "enabled": true }
}
```

**Query**: `biome explain config linter.enabled --file test.js`

**Output**:
```
Final value for linter.enabled: false

Configuration chain (in merge order):
  1. /project/node_modules/@company/biome-config/biome.json:2:12 configuration ━━━━━━━━━━━━━━━━━━━

    × extended config via biome.json → base.json → @company/biome-config set it to true

    >  2 │   "linter": { "enabled": true }
         │             ^^^^^^^^^^^^^^^^^^

  2. /project/configs/base.json:4:17 configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    × override in extended config (base.json) matching *.test.js set it to false ← FINAL VALUE

    >  4 │       "linter": { "enabled": false }
         │                 ^^^^^^^^^^^^^^^^^^^
```

## 10. Key Design Decisions & Rationale

### Decision 1: Merge Order Instead of Indices
**Rationale**: Using a simple counter (merge_order) instead of tracking indices in the extends array is simpler and more flexible. It naturally handles nested extends and makes it clear which value "wins" (higher merge_order = later = wins).

### Decision 2: Lazy Override Evaluation
**Rationale**: Pre-computing all possible file matches would be memory-intensive and wasteful. By storing override metadata with glob patterns and evaluating on query, we use less memory and only compute when needed.

### Decision 3: Optional ProvenanceContext Parameter
**Rationale**: Adding an optional parameter to Merge trait maintains backward compatibility while enabling tracking. Existing code continues to work with `None`, new code can opt-in with `Some(ctx)`.

### Decision 4: FieldQuery with Segments
**Rationale**: A structured path representation (Field/Index segments) is more robust than string parsing and enables programmatic manipulation. It supports arbitrary nesting and array indices.

### Decision 5: Separate Override Metadata Storage
**Rationale**: Overrides need special handling because they're conditional (file-pattern based). Storing them separately in `OverrideProvenanceMetadata` allows lazy evaluation without complicating the main provenance index.

### Decision 6: Use Diagnostics for Display (No Pre-Computed Locations)
**Rationale**: Storing only TextRange (8 bytes) instead of pre-computed line/column/snippet (~40+ bytes) reduces memory usage by ~80%. Provenance queries convert entries to Biome diagnostics which handle code frame generation internally. Since provenance queries are rare user-initiated operations, the CPU cost of computing line/column via diagnostics is acceptable and amortized through source caching. This follows the principle: compute small information on demand rather than storing it, especially when rarely accessed. It also ensures consistent formatting with other Biome output.

## 11. Open Questions & Future Enhancements

### Open Questions
1. **Performance**: How much overhead does provenance tracking add? Should it be opt-in via flag?
2. **Source Location Capture**: Best way to capture locations during deserialization without major refactoring?
3. **Serialization**: What format for sending provenance over RPC to LSP?

### Future Enhancements
1. **Interactive Mode**: `biome config explore` with interactive navigation through config chain
2. **Config Diff**: Show what changed between two configs
3. **Config Validation**: Warn about shadowed/unused values
4. **Visual Tools**: Web UI for visualizing config merges
5. **Performance Monitoring**: Track which configs cause slow loads

## 12. Testing Strategy

### Unit Tests
- FieldQuery parsing and manipulation
- ProvenanceSource display formatting
- Provenance trait and ProvenanceImpl
- CaptureProvenance derive macro code generation
- Override metadata creation and matching
- DeserializationContext with provenance tracking

### Integration Tests
- Config loading with extends tracking
- Override provenance for various glob patterns
- Nested extends resolution
- EditorConfig integration

### End-to-End Tests
- CLI command with various scenarios
- LSP hover in config files
- Complex real-world config setups

## 13. Documentation

### User Documentation
- Guide: "Understanding Configuration Provenance"
- Reference: `biome config explain` command
- Examples: Common debugging scenarios

### Developer Documentation
- Architecture: How provenance tracking works
- API: ProvenanceIndex and query methods
- Contributing: Adding provenance to new config types

---

## Summary

This design provides a comprehensive configuration provenance tracking system for Biome that:

1. **Tracks all sources**: Base config, extends (including nested), overrides, editorconfig, CLI args
2. **Captures at deserialization time**: Syntax nodes captured when JSON is parsed, not during merge
3. **Opt-in via derive macro**: `#[derive(CaptureProvenance)]` for types that need tracking
4. **Uses merge order**: Simple counter-based approach instead of complex indices
5. **Supports lazy evaluation**: Overrides evaluated on-demand for specific files
6. **Memory efficient**: Stores only `TextRange` in syntax nodes (8 bytes), uses Biome's `LineIndex` to convert to LSP positions on-demand during hover
7. **Integrates cleanly**: Extends `DeserializationContext` with optional provenance capture
8. **User-friendly**: Clear CLI output and LSP hover integration
9. **Extensible**: Can add new source types and query capabilities

Key innovations:
- **Deserialization-time capture**: Syntax nodes captured during initial parse, solving the override extraction problem
- **No Merge trait changes**: Merge stays simple, all provenance happens in deserialization
- **Opt-in design**: Only types with `CaptureProvenance` derive pay the cost
- **Universal approach**: Works for both merged configs (via Merge) and non-merged configs (like OverridePattern)
- **Uses existing infrastructure**: Reuses Biome's `LineIndex` for Position↔TextRange conversion instead of custom location tracking
- Using `merge_order` timestamps combined with lazy override evaluation
- Source file caching during query display to minimize I/O

Architecture Benefits:
- ✅ **Override problem solved**: `OverridePattern` doesn't use `Merge`, but `CaptureProvenance` captures its syntax nodes during deserialization
- ✅ **Less invasive**: No changes to existing Merge trait or implementations
- ✅ **Better separation**: Deserialization handles capture, Merge handles merging logic
- ✅ **Memory efficient**: Stores only syntax nodes (already in memory), not serialized strings
- ✅ **Precise source tracking**: Uses `JsonMemberName` syntax nodes for field paths instead of strings, providing exact source locations and preserved formatting
