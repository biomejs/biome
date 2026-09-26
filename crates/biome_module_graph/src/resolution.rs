use crate::{ModuleDb, ModuleInfo, SUPPORTED_EXTENSIONS};
use biome_resolver::{
    ResolveOptions, ResolvedSpecifier, ResolverDb, ResolverDbAdapter, ResolverPathData,
    resolve_with_metadata,
};
use camino::Utf8Path;

const JAVASCRIPT_EXTENSION_ALIASES: &[(&str, &[&str])] = &[
    ("js", &["ts", "tsx", "d.ts", "js", "jsx"]),
    ("mjs", &["mts", "d.mts", "mjs"]),
    ("cjs", &["cts", "d.cts", "cjs"]),
];

const HTML_SCRIPT_EXTENSIONS: &[&str] = &[
    "ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs", "json", "node", "vue", "astro", "svelte",
];

/// The resolution rules that apply to an import, which depend on the kind of
/// document that contains it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResolutionMode {
    /// Rules for imports of JavaScript and TypeScript modules.
    ///
    /// Resolution follows Node.js and TypeScript: the `exports` and `imports`
    /// of `package.json` with the `types`, `import` and `default` conditions,
    /// the path mappings of `tsconfig.json`, `index` files, and the
    /// extensions in [SUPPORTED_EXTENSIONS]. An import of a `.js` file may
    /// resolve to the matching `.ts` file. Built-in Node.js and Bun modules
    /// resolve to a [ResolveError::NodeBuiltIn] or
    /// [ResolveError::BunBuiltIn] error.
    ///
    /// [ResolveError::NodeBuiltIn]: biome_resolver::ResolveError::NodeBuiltIn
    /// [ResolveError::BunBuiltIn]: biome_resolver::ResolveError::BunBuiltIn
    JavaScript,

    /// Rules for stylesheet imports: `@import` rules of CSS files, and
    /// `<link rel="stylesheet">` elements and `<style>` blocks of HTML-like
    /// documents.
    ///
    /// Like in browsers, a specifier without a leading `./`, such as
    /// `theme.css`, is first resolved relative to the importing file, and only
    /// then as a package. Only `.css` files are resolved.
    Css,

    /// Rules for imports made by `<script>` blocks of HTML-like documents,
    /// such as HTML, Vue, Svelte and Astro files.
    ///
    /// These are the [ResolutionMode::JavaScript] rules, extended with the
    /// `.vue`, `.svelte` and `.astro` component extensions.
    HtmlScript,
}

/// Identifies one module-resolution query.
#[salsa::interned]
#[derive(Debug)]
pub struct ResolutionRequest<'db> {
    #[returns(copy)]
    pub base_directory: ResolverPathData,
    pub specifier: String,
    #[returns(copy)]
    pub mode: ResolutionMode,
}

/// Resolves one interned module request using the path info in the database.
#[salsa::tracked]
pub fn resolve_module_request<'db>(
    db: &'db dyn ResolverDb,
    request: ResolutionRequest<'db>,
) -> ResolvedSpecifier {
    let options = resolve_options(request.mode(db));
    let proxy = ResolverDbAdapter::new(db);
    resolve_with_metadata(
        request.specifier(db),
        request.base_directory(db).path(db),
        &proxy,
        &options,
    )
    .into()
}

/// Resolves `specifier` relative to `base_directory`.
///
/// Every module-graph consumer resolves through this function, so indexing,
/// lint rules and type inference share the same memoized results.
pub fn resolve_specifier<'db>(
    db: &'db dyn ResolverDb,
    base_directory: &Utf8Path,
    specifier: &str,
    mode: ResolutionMode,
) -> &'db ResolvedSpecifier {
    let base_directory = db.resolver_paths().get_or_create(db, base_directory);
    let request = ResolutionRequest::new(db, base_directory, specifier.to_string(), mode);
    resolve_module_request(db, request)
}

/// Resolves `specifier` as imported by `module`.
pub fn resolve_module_import<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    specifier: &str,
    mode: ResolutionMode,
) -> &'db ResolvedSpecifier {
    let module_path = module.path(db);
    let base_directory = module_path.parent().unwrap_or(module_path);
    resolve_specifier(db, base_directory, specifier, mode)
}

fn resolve_options(mode: ResolutionMode) -> ResolveOptions<'static> {
    match mode {
        ResolutionMode::JavaScript => ResolveOptions {
            condition_names: &["types", "import", "default"],
            default_files: &["index"],
            extensions: SUPPORTED_EXTENSIONS,
            extension_aliases: JAVASCRIPT_EXTENSION_ALIASES,
            resolve_node_builtins: true,
            resolve_bun_builtins: true,
            resolve_types: true,
            ..Default::default()
        },
        ResolutionMode::Css => ResolveOptions {
            assume_relative: true,
            extensions: &["css"],
            ..Default::default()
        },
        ResolutionMode::HtmlScript => ResolveOptions {
            condition_names: &["types", "import", "default"],
            default_files: &["index"],
            extensions: HTML_SCRIPT_EXTENSIONS,
            extension_aliases: JAVASCRIPT_EXTENSION_ALIASES,
            resolve_node_builtins: true,
            resolve_bun_builtins: true,
            resolve_types: true,
            ..Default::default()
        },
    }
}
