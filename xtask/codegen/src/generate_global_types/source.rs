//! TypeScript source acquisition and default library discovery for global types codegen.

use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, anyhow, bail};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsBindingPattern, AnyJsExpression, AnyJsLiteralExpression,
    JsVariableDeclarator,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, Text, TokenText};
use biome_string_case::StrLikeExtension;
use git2::{FetchOptions, Oid, Repository, ResetType};

use crate::generate_global_types::SourcePin;

/// Default remote used to acquire TypeScript sources.
const DEFAULT_TYPESCRIPT_REPO_URL: &str = "https://github.com/microsoft/TypeScript.git";

/// Fetch depth used for the pinned tag over a remote transport: only the
/// pinned commit, with no ancestry behind it.
const SHALLOW_FETCH_DEPTH: i32 = 1;

/// Subdirectory under the OS temporary directory used as the per-pin checkout cache.
const TYPESCRIPT_CACHE_DIR_NAME: &str = "biome-global-types";

/// TypeScript command line parser path relative to a checkout root.
const COMMAND_LINE_PARSER_RELATIVE_PATH: &str = "src/compiler/commandLineParser.ts";

/// TypeScript default library directory path relative to a checkout root.
const DEFAULT_LIBRARY_RELATIVE_DIR: &str = "lib";

/// TypeScript default library filenames must start with this marker before the lib key.
const DEFAULT_LIBRARY_FILE_PREFIX: &str = "lib.";

/// TypeScript default library discovery is limited to declaration files.
const DEFAULT_LIBRARY_FILE_SUFFIX: &str = ".d.ts";

/// Default base TypeScript library filenames checked for highest sort priority.
const BASE_DEFAULT_LIBRARY_FILES: &[&str] = &["lib.es6.d.ts", "lib.d.ts"];

/// Triple-slash directives are only recognized in comments that start with `///`.
const TRIPLE_SLASH_PREFIX: &str = "///";

/// TypeScript dependency directives use the XML-like `<reference ...>` form.
const REFERENCE_DIRECTIVE_PREFIX: &str = "<reference";

/// `path` references are file dependencies resolved relative to the current declaration.
const REFERENCE_PATH_ATTRIBUTE: &str = "path";

/// `lib` references are default-library dependencies resolved through TypeScript's lib map.
const REFERENCE_LIB_ATTRIBUTE: &str = "lib";

/// `types` references require external packages that this generator does not vendor.
const REFERENCE_TYPES_ATTRIBUTE: &str = "types";

/// `no-default-lib` references affect TypeScript's default library selection but add no file edge.
const REFERENCE_NO_DEFAULT_LIB_ATTRIBUTE: &str = "no-default-lib";

/// Explicit lib profile roots used as discovery starting points.
///
/// Discovery is restricted to these filenames plus their reference closure;
/// adding a new root requires an explicit selection-table change. `pub` so
/// `generate_global_types::run` can pass the production list to `discover` and
/// tests can pass scenario-specific lists.
pub const PROFILE_ROOTS: &[&str] = &[
    "lib.es5.d.ts",
    "lib.es2015.collection.d.ts",
    "lib.es2015.iterable.d.ts",
    "lib.es2015.promise.d.ts",
    "lib.es2018.promise.d.ts",
    "lib.es2020.promise.d.ts",
    "lib.es2021.promise.d.ts",
    "lib.es2024.promise.d.ts",
    "lib.es2025.promise.d.ts",
    "lib.es2015.symbol.wellknown.d.ts",
    "lib.es2015.reflect.d.ts",
    "lib.esnext.disposable.d.ts",
];

/// Priority used by TypeScript for the base default library files.
const BASE_DEFAULT_LIBRARY_PRIORITY: usize = 0;

/// Priority offset TypeScript uses for entries from `libEntries`.
const DEFAULT_LIBRARY_PRIORITY_OFFSET: usize = 1;

/// Priority offset TypeScript uses for files outside default-library ordering.
const OUTSIDE_DEFAULT_LIBRARY_PRIORITY_OFFSET: usize = 2;

/// A canonical filesystem path that has been proven to stay within a root.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CanonicalPath(PathBuf);

impl CanonicalPath {
    /// Canonicalizes `relative` under `root`, rejecting paths that escape `root`.
    pub fn from_within(root: &Path, relative: &str) -> anyhow::Result<Self> {
        let canonical_root = fs::canonicalize(root)
            .with_context(|| format!("failed to canonicalize root {}", root.display()))?;
        let candidate = canonical_root.join(relative);
        canonicalize_within(&canonical_root, &candidate)
    }

    /// Exposes the already-canonicalized, root-checked path. The caller does
    /// not need to revalidate containment.
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

/// An acquired TypeScript checkout and metadata derived from it.
pub struct AcquiredCheckout {
    root: CanonicalPath,
    pin: SourcePin,
}

impl AcquiredCheckout {
    /// Canonical root of the acquired TypeScript checkout.
    pub fn root(&self) -> &CanonicalPath {
        &self.root
    }

    /// Pinned TypeScript source tag and commit.
    pub fn pin(&self) -> &SourcePin {
        &self.pin
    }
}

/// Parsed entries from TypeScript's `libEntries` table.
pub struct LibEntries {
    /// Library keys in source order, preserving duplicates.
    pub libs: Vec<Text>,
    /// Library key to filename map; later duplicate keys overwrite earlier ones.
    pub lib_map: BTreeMap<Text, Text>,
}

/// A discovered TypeScript declaration file after dependency traversal.
pub struct DiscoveredFile {
    /// Canonical filesystem path, proven to stay within the checkout root.
    pub path: CanonicalPath,
    /// Checkout-relative path using forward slashes.
    pub repo_relative: String,
    /// Raw file bytes read from the checkout.
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TrackedSourceFile {
    path: CanonicalPath,
    repo_relative: String,
}

/// Options controlling TypeScript source acquisition.
#[derive(Default)]
pub struct SourceOptions {
    /// Alternate TypeScript repository URL used by tests; production runs use
    /// [`DEFAULT_TYPESCRIPT_REPO_URL`].
    pub repo_url_override: Option<PathBuf>,
}

/// Acquires a pinned TypeScript checkout in the per-OS temporary cache.
pub fn acquire(pin: &SourcePin, opts: &SourceOptions) -> anyhow::Result<AcquiredCheckout> {
    let checkout_path = env::temp_dir()
        .join(TYPESCRIPT_CACHE_DIR_NAME)
        .join(cache_key(pin));

    if !checkout_path.exists() {
        clone_checkout(&checkout_path, pin, opts)?;
    }

    let canonical_root = CanonicalPath(
        fs::canonicalize(&checkout_path)
            .with_context(|| format!("failed to canonicalize {}", checkout_path.display()))?,
    );

    Ok(AcquiredCheckout {
        root: canonical_root,
        pin: pin.clone(),
    })
}

/// Parses TypeScript's `libEntries` table from `commandLineParser.ts`.
pub fn parse_lib_entries(checkout: &AcquiredCheckout) -> anyhow::Result<LibEntries> {
    let bytes = read_source_file(
        &checkout
            .root
            .as_path()
            .join(COMMAND_LINE_PARSER_RELATIVE_PATH),
    )?;
    let source = std::str::from_utf8(&bytes).context("commandLineParser.ts is not valid UTF-8")?;
    let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());

    if parsed.has_errors() {
        bail!(
            "failed to parse commandLineParser.ts: {} diagnostics",
            parsed.diagnostics().len()
        );
    }

    let root = parsed.syntax();
    for node in root.descendants() {
        let Some(declarator) = JsVariableDeclarator::cast(node) else {
            continue;
        };
        let Ok(id) = declarator.id() else {
            continue;
        };
        let Some(name) = binding_name(&id) else {
            continue;
        };
        if name != "libEntries" {
            continue;
        }

        let Some(initializer) = declarator.initializer() else {
            bail!("libEntries declarator has no initializer");
        };
        let expression = initializer
            .expression()
            .context("failed to read libEntries initializer expression")?;
        let Some(array) = array_expression(&expression) else {
            bail!("libEntries initializer is not an array expression");
        };

        let mut libs = Vec::new();
        let mut lib_map = BTreeMap::new();
        for (index, element) in array.elements().into_iter().enumerate() {
            let element = element
                .with_context(|| format!("failed to read libEntries element at index {index}"))?;
            let tuple_expression = match element {
                AnyJsArrayElement::AnyJsExpression(expression) => expression,
                AnyJsArrayElement::JsArrayHole(_) | AnyJsArrayElement::JsSpread(_) => {
                    bail!("libEntries element at index {index} is not a tuple array");
                }
            };
            let Some(tuple_array) = array_expression(&tuple_expression) else {
                bail!("libEntries element at index {index} is not a tuple array");
            };
            let (key, filename) = tuple_elements(tuple_array)
                .with_context(|| format!("failed to parse libEntries tuple at index {index}"))?;
            validate_lib_entry_filename(index, filename.text())?;

            libs.push(key.clone());
            lib_map.insert(key, filename);
        }

        if libs.is_empty() {
            bail!("libEntries array is empty");
        }

        return Ok(LibEntries { libs, lib_map });
    }

    Err(anyhow!("could not find libEntries in commandLineParser.ts"))
}

/// Discovers default library files by walking each profile root's reference
/// closure and sorting the result by TypeScript's default-library priority.
///
/// `profile_roots` is the entry-point filename list (typically [`PROFILE_ROOTS`]).
/// Tests pass scenario-specific lists to lock the resulting closure order.
pub fn discover(
    checkout: &AcquiredCheckout,
    libs: &LibEntries,
    profile_roots: &[&str],
) -> anyhow::Result<Vec<DiscoveredFile>> {
    let default_library_path =
        CanonicalPath::from_within(checkout.root.as_path(), DEFAULT_LIBRARY_RELATIVE_DIR)
            .context("failed to resolve TypeScript lib directory")?;
    let mut visited = HashSet::new();
    let mut discovered = Vec::<DiscoveredFile>::new();

    for filename in profile_roots {
        let relative = default_library_relative_path(filename);
        let source = validate_reference_source_file(checkout.root.as_path(), &relative)
            .with_context(|| format!("profile root {filename:?} is not a tracked regular file"))?;

        if visited.contains(&source.repo_relative) {
            continue;
        }
        visited.insert(source.repo_relative.clone());

        let mut stack = vec![make_frame(source, checkout.root.as_path(), libs)?];
        while !stack.is_empty() {
            let child_path = {
                let frame = stack.last_mut().expect("stack is not empty");
                frame.children_remaining.pop().map(|reference| {
                    let parent_dir = frame.source.path.as_path().parent().ok_or_else(|| {
                        anyhow!(
                            "{} has no parent directory",
                            frame.source.path.as_path().display()
                        )
                    });

                    let parent_dir = parent_dir?;
                    resolve_child_reference(checkout.root.as_path(), parent_dir, &reference, libs)
                })
            };

            let Some(child_path) = child_path else {
                let frame = stack.pop().expect("stack is not empty");
                discovered.push(discovered_file_from_frame(frame));
                continue;
            };
            let child_path = child_path?;

            if !visited.contains(&child_path.repo_relative) {
                visited.insert(child_path.repo_relative.clone());
                stack.push(make_frame(child_path, checkout.root.as_path(), libs)?);
            }
        }
    }

    discovered.sort_by_cached_key(|file| {
        default_library_priority(
            file.path.as_path(),
            default_library_path.as_path(),
            &libs.libs,
        )
    });

    Ok(discovered)
}

/// Stack frame used while traversing TypeScript triple-slash dependency references.
struct Frame {
    source: TrackedSourceFile,
    bytes: Vec<u8>,
    children_remaining: Vec<TripleSlashReference>,
}

/// File or default-library reference parsed from a triple-slash directive.
enum TripleSlashReference {
    File(String),
    Lib(String),
}

/// Returns the per-pin cache directory basename `"<tag>-<sha>"` used to
/// namespace acquired checkouts.
fn cache_key(pin: &SourcePin) -> String {
    format!("{}-{}", pin.tag(), pin.sha())
}

/// Fetches the pinned TypeScript tag with `git2` and resets the checkout to the
/// pinned commit.
fn clone_checkout(
    checkout_path: &Path,
    pin: &SourcePin,
    opts: &SourceOptions,
) -> anyhow::Result<()> {
    if let Some(parent) = checkout_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    let repo_url = match opts.repo_url_override.as_deref() {
        Some(path) => path.to_str().ok_or_else(|| {
            anyhow!(
                "repository override path is not valid UTF-8: {}",
                path.display()
            )
        })?,
        None => DEFAULT_TYPESCRIPT_REPO_URL,
    };

    let repository = Repository::init(checkout_path)
        .with_context(|| format!("failed to initialize {}", checkout_path.display()))?;

    // Only the remote can be shallow-fetched; the local transport (test overrides) rejects a depth.
    let mut fetch_options = FetchOptions::new();
    if opts.repo_url_override.is_none() {
        fetch_options.depth(SHALLOW_FETCH_DEPTH);
    }

    let tag_refspec = format!("+refs/tags/{tag}:refs/tags/{tag}", tag = pin.tag());
    repository
        .remote_anonymous(repo_url)
        .with_context(|| format!("failed to create remote for {repo_url}"))?
        .fetch(&[tag_refspec.as_str()], Some(&mut fetch_options), None)
        .with_context(|| {
            format!(
                "failed to fetch TypeScript tag {} from {repo_url}",
                pin.tag()
            )
        })?;

    let oid =
        Oid::from_str(pin.sha()).with_context(|| format!("invalid pinned commit {}", pin.sha()))?;
    let object = repository.find_object(oid, None).with_context(|| {
        format!(
            "pinned commit {} was not fetched from tag {}",
            pin.sha(),
            pin.tag()
        )
    })?;
    repository
        .reset(&object, ResetType::Hard, None)
        .with_context(|| format!("failed to reset TypeScript checkout to {}", pin.sha()))?;

    Ok(())
}

/// Canonicalizes `path` and bails if the result escapes `root`.
fn canonicalize_within(root: &Path, path: &Path) -> anyhow::Result<CanonicalPath> {
    let canonical_path = fs::canonicalize(path)
        .with_context(|| format!("failed to canonicalize {}", path.display()))?;
    if !canonical_path.starts_with(root) {
        bail!(
            "path {} escapes root {}",
            canonical_path.display(),
            root.display()
        );
    }

    Ok(CanonicalPath(canonical_path))
}

/// Reads a source file from the validated checkout.
fn read_source_file(path: &Path) -> anyhow::Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("failed to read {}", path.display()))
}

/// Returns the identifier text when the pattern is a single identifier binding.
fn binding_name(pattern: &AnyJsBindingPattern) -> Option<TokenText> {
    let binding = pattern.as_any_js_binding()?;
    let identifier = binding.as_js_identifier_binding()?;
    Some(identifier.name_token().ok()?.token_text_trimmed())
}

/// Strips surrounding parentheses and `as`/`satisfies` wrappers to expose
/// the underlying `JsArrayExpression`, returning `None` for any other shape.
fn array_expression(expression: &AnyJsExpression) -> Option<biome_js_syntax::JsArrayExpression> {
    let mut current = expression.clone().omit_parentheses();
    loop {
        current = match current {
            AnyJsExpression::JsArrayExpression(array) => return Some(array),
            AnyJsExpression::TsAsExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsInstantiationExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsNonNullAssertionExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsSatisfiesExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            _ => return None,
        };
    }
}

/// Extracts a `[key, filename]` pair of string literals from a 2-element array.
fn tuple_elements(array: biome_js_syntax::JsArrayExpression) -> Option<(Text, Text)> {
    let mut elements = array.elements().into_iter();
    let first = tuple_element_text(elements.next()?)?;
    let second = tuple_element_text(elements.next()?)?;
    if elements.next().is_some() {
        return None;
    }

    Some((first, second))
}

/// Returns the string-literal text of one tuple element, or `None` if it is
/// missing, holed, spread, or not a string literal.
fn tuple_element_text(element: biome_rowan::SyntaxResult<AnyJsArrayElement>) -> Option<Text> {
    let element = element.ok()?;
    let AnyJsArrayElement::AnyJsExpression(expression) = element else {
        return None;
    };
    string_literal_text(&expression)
}

/// Returns the unquoted text of a direct string literal expression, or `None`
/// for any other shape (parenthesized, `as`/`satisfies`-wrapped, computed, …).
fn string_literal_text(expression: &AnyJsExpression) -> Option<Text> {
    let AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(
        string,
    )) = expression
    else {
        return None;
    };

    Some(Text::from(string.inner_string_text().ok()?))
}

/// Bails when a `libEntries` filename does not look like a `lib.*.d.ts` shape.
fn validate_lib_entry_filename(index: usize, filename: &str) -> anyhow::Result<()> {
    if filename.is_empty()
        || filename.contains('/')
        || filename.contains('\\')
        || Path::new(filename).is_absolute()
        || !filename.starts_with(DEFAULT_LIBRARY_FILE_PREFIX)
        || !filename.ends_with(DEFAULT_LIBRARY_FILE_SUFFIX)
        || Path::new(filename)
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        bail!(
            "libEntries filename at index {index} must be a default-library basename ending in .d.ts: {filename:?}"
        );
    }

    Ok(())
}

/// Reads the file from the validated checkout, parses its triple-slash
/// references, and returns a stack frame ready for the discovery DFS.
fn make_frame(
    source_file: TrackedSourceFile,
    root: &Path,
    libs: &LibEntries,
) -> anyhow::Result<Frame> {
    let bytes = read_source_file(source_file.path.as_path())?;
    let source = std::str::from_utf8(&bytes).with_context(|| {
        format!(
            "{} is not valid UTF-8",
            source_file.path.as_path().display()
        )
    })?;
    let mut children_remaining =
        parse_triple_slash_references(source, source_file.path.as_path(), root, libs)?;
    children_remaining.reverse();

    Ok(Frame {
        source: source_file,
        bytes,
        children_remaining,
    })
}

/// Returns every `<reference ... />` directive found in the file's leading
/// comments, in source order.
fn parse_triple_slash_references(
    source: &str,
    path: &Path,
    root: &Path,
    libs: &LibEntries,
) -> anyhow::Result<Vec<TripleSlashReference>> {
    let parsed = parse(source, JsFileSource::d_ts(), JsParserOptions::default());
    // Refuse to extract triple-slash references from a file that did not parse
    // cleanly: a missing leading-trivia token would silently produce an empty
    // reference list and let the collector run against incomplete coverage.
    if parsed.has_errors() {
        bail!(
            "{} produced parser diagnostics while extracting triple-slash references",
            path.display()
        );
    }
    let syntax = parsed.syntax();
    let Some(first_token) = syntax.first_token() else {
        return Ok(Vec::new());
    };

    first_token
        .leading_trivia()
        .pieces()
        .filter(|piece| piece.is_comments())
        .filter_map(|piece| {
            parse_triple_slash_reference(piece.text(), path, root, libs).transpose()
        })
        .collect()
}

/// Parses a single `///` comment as a TypeScript reference directive,
/// returning the resolved edge or `None` for unrelated comments.
fn parse_triple_slash_reference(
    comment: &str,
    path: &Path,
    root: &Path,
    libs: &LibEntries,
) -> anyhow::Result<Option<TripleSlashReference>> {
    let Some(directive) = reference_pragma_arguments(comment) else {
        return Ok(None);
    };

    // TypeScript's `processPragmasIntoFields` only suppresses other reference
    // attributes when the literal value is `"true"`. Any other value (including
    // `"false"`) falls through to the regular precedence chain, so we must
    // match that semantics rather than treating any presence as a suppressor.
    if matches!(
        directive_attribute(directive, REFERENCE_NO_DEFAULT_LIB_ATTRIBUTE)?,
        Some("true"),
    ) {
        Ok(None)
    } else if let Some(types) = directive_attribute(directive, REFERENCE_TYPES_ATTRIBUTE)? {
        bail!(
            "{} contains unsupported triple-slash types reference {types:?}",
            path.display()
        );
    } else if let Some(lib_reference) = directive_attribute(directive, REFERENCE_LIB_ATTRIBUTE)? {
        let lib_key = to_typescript_file_name_lowercase(lib_reference);
        if !libs.lib_map.contains_key(lib_key.as_str()) {
            bail!(
                "{} references unknown lib {lib_reference:?} under {}",
                path.display(),
                root.display()
            );
        }
        Ok(Some(TripleSlashReference::Lib(lib_key)))
    } else if let Some(path_reference) = directive_attribute(directive, REFERENCE_PATH_ATTRIBUTE)? {
        Ok(Some(TripleSlashReference::File(path_reference.to_owned())))
    } else {
        Ok(None)
    }
}

/// Returns the attribute slice of a `/// <reference ... />` comment when the
/// shape matches TypeScript's `tripleSlashXMLCommentStartRegEx`.
fn reference_pragma_arguments(comment: &str) -> Option<&str> {
    let directive = comment.trim_start().strip_prefix(TRIPLE_SLASH_PREFIX)?;
    let directive = directive.trim_start().strip_prefix('<')?;
    let end = directive.find("/>")?;
    let directive = &directive[..end];
    let name_end = directive
        .char_indices()
        .find_map(|(index, char)| char.is_ascii_whitespace().then_some(index))?;
    let name = &directive[..name_end];
    if !name.eq_ignore_ascii_case(REFERENCE_DIRECTIVE_PREFIX.trim_start_matches('<')) {
        return None;
    }

    Some(&directive[name_end..])
}

/// Returns the value of `attribute="..."` from a directive body using ASCII-
/// case-insensitive attribute-name matching with word boundaries.
///
/// `Ok(None)` means the attribute simply isn't present; `Err(_)` means a
/// matching attribute name was found but its quoted value was malformed (for
/// example, the closing quote ran past the directive body). TypeScript itself
/// reports `Invalid_reference_directive_syntax` in that case, so we surface
/// the parse error rather than silently treating it as "absent".
fn directive_attribute<'directive>(
    directive: &'directive str,
    attribute: &str,
) -> anyhow::Result<Option<&'directive str>> {
    let lower_directive = directive.to_ascii_lowercase_cow();
    let lower_attribute = attribute.to_ascii_lowercase_cow();
    let mut search_start = 0;
    while search_start < directive.len() {
        let Some(relative_index) = lower_directive[search_start..].find(lower_attribute.as_ref())
        else {
            return Ok(None);
        };
        let attribute_start = search_start + relative_index;
        let attribute_end = attribute_start + lower_attribute.len();

        if !is_attribute_boundary_before(directive, attribute_start)
            || !is_attribute_boundary_after(directive, attribute_end)
        {
            search_start = attribute_end;
            continue;
        }

        let after_name = directive[attribute_end..].trim_start();
        let Some(after_equals) = after_name.strip_prefix('=') else {
            search_start = attribute_end;
            continue;
        };
        let after_equals = after_equals.trim_start();
        let Some(quote) = after_equals.as_bytes().first().copied() else {
            bail!("triple-slash directive attribute {attribute:?} has no value in {directive:?}");
        };
        if quote != b'"' && quote != b'\'' {
            search_start = attribute_end;
            continue;
        }

        let value_start = 1;
        let Some(value_end) = after_equals[value_start..]
            .bytes()
            .position(|byte| byte == quote)
        else {
            bail!(
                "triple-slash directive attribute {attribute:?} has an unterminated quoted value in {directive:?}"
            );
        };
        return Ok(Some(&after_equals[value_start..value_start + value_end]));
    }

    Ok(None)
}

/// ASCII-lowercases `value` for TypeScript's case-insensitive lib-key lookup,
/// borrowing through `Cow` when the input is already lowercase.
fn to_typescript_file_name_lowercase(value: &str) -> String {
    value.to_ascii_lowercase_cow().into_owned()
}

/// True when the character preceding `index` is a word boundary (whitespace,
/// start of input, or the opening `<` of the directive).
fn is_attribute_boundary_before(text: &str, index: usize) -> bool {
    text[..index]
        .chars()
        .next_back()
        .is_none_or(|char| char.is_ascii_whitespace() || char == '<')
}

/// True when the character at `index` is a word boundary (whitespace, `=`,
/// `/`, or end of input).
fn is_attribute_boundary_after(text: &str, index: usize) -> bool {
    text[index..]
        .chars()
        .next()
        .is_none_or(|char| char.is_ascii_whitespace() || char == '=' || char == '/')
}

/// Resolves a `<reference>` directive to a tracked, validated source file
/// inside the pinned checkout.
fn resolve_child_reference(
    root: &Path,
    parent_dir: &Path,
    reference: &TripleSlashReference,
    libs: &LibEntries,
) -> anyhow::Result<TrackedSourceFile> {
    match reference {
        TripleSlashReference::Lib(reference) => {
            let filename = libs
                .lib_map
                .get(reference.as_str())
                .ok_or_else(|| anyhow!("unknown lib reference {reference:?}"))?;
            let relative = default_library_relative_path(filename.text());
            validate_reference_source_file(root, &relative).with_context(|| {
                format!("lib reference {reference:?} is not a tracked regular file")
            })
        }
        TripleSlashReference::File(reference) => {
            // `root` is already canonical (it comes from `AcquiredCheckout::root`, which
            // is built with `CanonicalPath::from_within`), so we skip the extra
            // `fs::canonicalize` per file reference.
            let relative =
                normalize_typescript_reference_relative_path(root, parent_dir, reference)
                    .with_context(|| format!("failed to resolve path reference {reference:?}"))?;
            validate_reference_source_file(root, &relative).with_context(|| {
                format!("path reference {reference:?} is not a tracked regular file")
            })
        }
    }
}

/// TypeScript-equivalent `normalizePath(combinePaths(...))`: folds `\` to `/`,
/// resolves `.`/`..` segments, rejects rooted paths and NUL bytes.
fn normalize_typescript_reference_relative_path(
    root: &Path,
    parent_dir: &Path,
    reference: &str,
) -> anyhow::Result<String> {
    // Only allocate when the reference actually uses Windows separators; the
    // common Unix-style path case keeps borrowing the caller's string slice.
    let reference_owned: String;
    let reference: &str = if reference.contains('\\') {
        reference_owned = reference.replace('\\', "/");
        &reference_owned
    } else {
        reference
    };
    if is_typescript_rooted_path(reference) {
        bail!(
            "path reference {reference:?} escapes root {}",
            root.display()
        );
    }

    let parent_relative = repo_relative_path(root, parent_dir)?;
    // Borrow segment slices into `parent_relative` and `reference` instead of
    // owning per-segment Strings; both source buffers outlive `parts`.
    let mut parts: Vec<&str> = Vec::new();
    if !parent_relative.is_empty() {
        parts.extend(parent_relative.split('/'));
    }

    for segment in reference.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    bail!(
                        "path reference {reference:?} escapes root {}",
                        root.display()
                    );
                }
            }
            segment => {
                if segment.contains('\0') {
                    bail!("path reference {reference:?} contains a NUL byte");
                }
                parts.push(segment);
            }
        }
    }

    if parts.is_empty() {
        bail!("path reference {reference:?} does not resolve to a file");
    }

    Ok(parts.join("/"))
}

/// True when `path` starts with `/` or a Windows drive letter prefix (`C:`).
fn is_typescript_rooted_path(path: &str) -> bool {
    let bytes = path.as_bytes();
    path.starts_with('/')
        || matches!(
            bytes.get(0..2),
            Some([drive, b':']) if drive.is_ascii_alphabetic()
        )
}

/// Validates a checkout-relative path layer by layer: lexical shape,
/// no symlink components, tracked regular file in git, then canonicalize
/// inside the checkout root.
fn validate_reference_source_file(
    root: &Path,
    relative: &str,
) -> anyhow::Result<TrackedSourceFile> {
    validate_forward_slash_relative_path(root, relative)?;
    validate_no_symlink_components(root, relative)?;

    let candidate = forward_slash_relative_path(root, relative);
    let path = canonicalize_within(root, &candidate)?;

    let metadata = fs::metadata(path.as_path())
        .with_context(|| format!("failed to inspect {}", path.as_path().display()))?;
    if !metadata.is_file() {
        bail!(
            "resolved TypeScript source path {} is not a regular file",
            path.as_path().display()
        );
    }

    Ok(TrackedSourceFile {
        path,
        repo_relative: relative.to_owned(),
    })
}

/// Bails when `relative` is empty, rooted, contains `\`, `.`, `..`, or a
/// per-segment rooted prefix (Windows drive letter).
fn validate_forward_slash_relative_path(root: &Path, relative: &str) -> anyhow::Result<()> {
    if relative.is_empty() || is_typescript_rooted_path(relative) {
        bail!(
            "path reference {relative:?} escapes root {}",
            root.display()
        );
    }

    for segment in relative.split('/') {
        if segment.is_empty()
            || segment == "."
            || segment == ".."
            || segment.contains('\\')
            || is_typescript_rooted_path(segment)
        {
            bail!("path reference {relative:?} is not a normalized checkout-relative path");
        }
    }

    Ok(())
}

/// Walks each path segment under `root` and bails when any intermediate
/// component is a symlink, so canonicalization cannot escape the checkout.
fn validate_no_symlink_components(root: &Path, relative: &str) -> anyhow::Result<()> {
    let mut path = root.to_path_buf();
    for segment in relative.split('/') {
        path.push(segment);
        let metadata = fs::symlink_metadata(&path)
            .with_context(|| format!("failed to inspect {}", path.display()))?;
        if metadata.file_type().is_symlink() {
            bail!(
                "resolved TypeScript source path {} contains symlink component {}",
                forward_slash_relative_path(root, relative).display(),
                path.display()
            );
        }
    }

    Ok(())
}

/// Joins a forward-slash `relative` path onto `root`, segment by segment.
fn forward_slash_relative_path(root: &Path, relative: &str) -> PathBuf {
    let mut path = root.to_path_buf();
    for segment in relative.split('/') {
        path.push(segment);
    }
    path
}

/// Materializes a discovery stack frame into the per-file output record.
fn discovered_file_from_frame(frame: Frame) -> DiscoveredFile {
    DiscoveredFile {
        path: frame.source.path,
        repo_relative: frame.source.repo_relative,
        bytes: frame.bytes,
    }
}

/// Returns `path` re-expressed relative to `root` and forward-slashed.
fn repo_relative_path(root: &Path, path: &Path) -> anyhow::Result<String> {
    let relative = path
        .strip_prefix(root)
        .with_context(|| format!("{} is not under {}", path.display(), root.display()))?;
    forward_slash_path(relative)
}

/// Joins a path's normal components with `/`, bailing on absolute prefixes
/// or non-UTF-8 components.
fn forward_slash_path(path: &Path) -> anyhow::Result<String> {
    path.components()
        .try_fold(String::new(), |mut acc, component| {
            let Component::Normal(part) = component else {
                bail!("path {} contains a non-normal component", path.display());
            };
            let part = part
                .to_str()
                .ok_or_else(|| anyhow!("path {} is not valid UTF-8", path.display()))?;
            if !acc.is_empty() {
                acc.push('/');
            }
            acc.push_str(part);
            Ok(acc)
        })
}

/// Builds the repo-relative path of a default library file, joining the
/// `lib/` prefix used by every TypeScript declaration in `libEntries`.
fn default_library_relative_path(filename: &str) -> String {
    format!("{DEFAULT_LIBRARY_RELATIVE_DIR}/{filename}")
}

/// TypeScript-equivalent default library sort key: base files first, then
/// `libEntries` order, then files outside the default library tree.
fn default_library_priority(path: &Path, default_library_path: &Path, libs: &[Text]) -> usize {
    let outside_priority = libs.len() + OUTSIDE_DEFAULT_LIBRARY_PRIORITY_OFFSET;
    let Ok(relative) = path.strip_prefix(default_library_path) else {
        return outside_priority;
    };

    let Some(filename) = relative.file_name().and_then(|filename| filename.to_str()) else {
        return outside_priority;
    };

    if BASE_DEFAULT_LIBRARY_FILES.contains(&filename) {
        return BASE_DEFAULT_LIBRARY_PRIORITY;
    }

    let Some(stripped) = filename.strip_prefix(DEFAULT_LIBRARY_FILE_PREFIX) else {
        return outside_priority;
    };
    let Some(stripped) = stripped.strip_suffix(DEFAULT_LIBRARY_FILE_SUFFIX) else {
        return outside_priority;
    };

    libs.iter()
        .position(|lib| lib.text() == stripped)
        .map_or(outside_priority, |index| {
            index + DEFAULT_LIBRARY_PRIORITY_OFFSET
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_library_priority_base_files_only_win_inside_default_library_dir() {
        let libs = vec![Text::from("es6")];
        let default_library_path = Path::new("checkout").join(DEFAULT_LIBRARY_RELATIVE_DIR);
        let outside_priority = libs.len() + OUTSIDE_DEFAULT_LIBRARY_PRIORITY_OFFSET;

        for filename in BASE_DEFAULT_LIBRARY_FILES {
            assert_eq!(
                default_library_priority(
                    &default_library_path.join(filename),
                    &default_library_path,
                    &libs,
                ),
                BASE_DEFAULT_LIBRARY_PRIORITY
            );
            assert_eq!(
                default_library_priority(
                    &Path::new("outside").join(filename),
                    &default_library_path,
                    &libs,
                ),
                outside_priority
            );
        }
    }
}
