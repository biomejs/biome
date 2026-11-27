use crate::services::database::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsDeclarationClause, AnyJsExportClause,
    AnyJsExportDefaultDeclaration, AnyJsExportNamedSpecifier, AnyJsExpression, AnyJsImportLike,
    AnyJsLiteralExportName, AnyJsModuleItem, AnyTsIdentifierBinding, AnyTsModuleName,
    JsExportNamedFromSpecifier, JsIdentifierBinding, JsLanguage, JsModule, JsModuleItemList,
    JsSyntaxToken, inner_string_text, unescape_js_string,
};
use biome_js_type_info::ImportSymbol;
use biome_module_graph::{
    JsExportedSymbolLookup, JsImport, JsOwnExport, ModuleDb, ModuleInfo, ModuleInfoKind,
    SymbolFromModuleInfo, find_js_exported_symbol,
};
use biome_rowan::{AstNode, SyntaxKindSet, Text, TextRange, WalkEvent};
use biome_rule_options::no_duplicate_exports::NoDuplicateExportsOptions;
use enumflags2::{BitFlags, bitflags};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Entry;

declare_lint_rule! {
    /// Disallow exporting the same name more than once.
    ///
    /// A module can export each name, including `default`, only once.
    /// Otherwise, JavaScript refuses to load the module, or one of the exports
    /// is silently lost.
    ///
    /// The rule also checks TypeScript namespaces and `declare module` blocks,
    /// and the names that `export * from "..."` brings in from other modules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export const foo = 1;
    /// function bar() {}
    /// export { bar as foo };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = 1;
    /// export { foo };
    /// export { foo };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = 1;
    /// export default function bar() {}
    /// export { foo as default };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export { foo } from "./foo.js";
    /// export { bar as foo } from "./bar.js";
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// const foo = 1;
    /// type Foo = string;
    /// export { foo as Foo };
    /// export type { Foo };
    /// ```
    ///
    /// ```js,file=a.js
    /// export const foo = 1;
    /// ```
    ///
    /// ```js,file=b.js
    /// export const foo = 2;
    /// ```
    ///
    /// ```js,expect_diagnostic,file=index.js
    /// export * from "./a.js";
    /// export * from "./b.js";
    /// ```
    ///
    /// ```js,file=c.js
    /// export const foo = 1;
    /// ```
    ///
    /// ```js,expect_diagnostic,file=main.js
    /// export * from "./c.js";
    /// export const foo = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const foo = 1;
    /// export { foo, foo as bar };
    /// export default foo;
    /// export * as utils from "./utils.js";
    ///
    /// const shape = { size: 1 };
    /// export { shape as Shape };
    /// export type Shape = { size: number };
    ///
    /// export namespace Inner {
    ///     export const foo = 2;
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - To report declarations that reuse the name of another declaration in
    ///   the same scope, including exported ones, see
    ///   [`noRedeclare`](https://biomejs.dev/linter/rules/no-redeclare/).
    pub NoDuplicateExports {
        version: "next",
        name: "noDuplicateExports",
        language: "js",
        sources: &[RuleSource::EslintImport("export").inspired()],
        domains: &[RuleDomain::Project],
        severity: Severity::Error,
        recommended: true,
    }
}

impl Rule for NoDuplicateExports {
    type Query = ResolvedImports<JsModuleItemList>;
    type State = DuplicateExport;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateExportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut first_exports: FxHashMap<_, FirstExports> = FxHashMap::default();
        let mut duplicates = Vec::new();
        for item in ctx.query() {
            let AnyJsModuleItem::JsExport(export) = item else {
                continue;
            };
            let Ok(clause) = export.export_clause() else {
                continue;
            };
            for_each_exported_name(&clause, |exported| {
                let range = exported.token.text_trimmed_range();
                let first = first_exports.entry(exported.name.clone()).or_default();
                let earlier = if exported.is_declaration {
                    // The parser already reports duplicate `export default`
                    // declarations, and `noRedeclare` reports exported
                    // declarations that reuse a name, so a declaration is only
                    // compared with the names from export lists.
                    let earlier = first.specifiers.first_in(exported.kinds);
                    first.declarations.insert(exported.kinds, range);
                    earlier
                } else {
                    // TypeScript doesn't allow two names from export lists to
                    // be the same, even if only one of them is a type.
                    let earlier = earliest(
                        first.specifiers.first_in(BitFlags::all()),
                        first.declarations.first_in(exported.kinds),
                    );
                    first.specifiers.insert(exported.kinds, range);
                    earlier
                };
                if let Some(first) = earlier {
                    let kind = if exported.name.text() == "default" {
                        DuplicateKind::Default
                    } else {
                        DuplicateKind::Name
                    };
                    duplicates.push(DuplicateExport {
                        name: exported.name,
                        range,
                        first,
                        kind,
                    });
                }
            });
        }
        // The module graph only describes the top level of a module, and
        // `export * from` is only allowed there and in `declare module` blocks.
        if ctx.query().parent::<JsModule>().is_some() {
            push_export_all_conflicts(ctx, &first_exports, &mut duplicates);
        }
        duplicates.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(rule_category!(), state.range, state);
        Some(match state.kind {
            DuplicateKind::Name | DuplicateKind::Default => diagnostic
                .detail(
                    state.first,
                    if state.kind == DuplicateKind::Default {
                        markup! { "The default export is first defined here." }
                    } else {
                        markup! { "The name is first exported here." }
                    },
                )
                .note(markup! {
                    "Each exported name must be unique. JavaScript refuses to load a module that exports the same name twice, and TypeScript reports an error."
                })
                .note(markup! {
                    "Remove one of the exports, or give one of them a different name."
                }),
            DuplicateKind::ExportAll => diagnostic
                .detail(
                    state.first,
                    markup! { "This module also exports the name." },
                )
                .note(markup! {
                    "When two "<Emphasis>"export * from"</Emphasis>" statements export the same name from different declarations, the module doesn't export the name at all. Importing it fails, and TypeScript reports an error."
                })
                .note(markup! {
                    "Replace one of the "<Emphasis>"export * from"</Emphasis>" statements with "<Emphasis>"export { ... } from"</Emphasis>", listing only the names you need."
                }),
            DuplicateKind::ReplacedExportAll => diagnostic
                .detail(
                    state.first,
                    markup! { "This export replaces the name from the module." },
                )
                .note(markup! {
                    "A name that the module exports itself takes precedence over the same name from "<Emphasis>"export * from"</Emphasis>", so the name from the other module is never exported."
                })
                .note(markup! {
                    "Give the module's own export a different name, or remove it to export the name from the other module."
                }),
        })
    }
}

/// An export that reuses the name of an earlier export in the same module or
/// namespace.
pub struct DuplicateExport {
    /// The reused name, with quotes removed and escape sequences decoded.
    name: Text,
    /// The range of the token that spells the reused name, of the `default`
    /// keyword of an `export default` statement, or of the module specifier of
    /// an `export * from` statement.
    range: TextRange,
    /// The range of the earlier export with the same name.
    first: TextRange,
    kind: DuplicateKind,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum DuplicateKind {
    /// Two exports in the module use the same name.
    Name,
    /// The module has more than one default export.
    Default,
    /// Two `export * from` statements export the same name from different
    /// declarations.
    ExportAll,
    /// An `export * from` statement exports a name that the module also
    /// exports itself.
    ReplacedExportAll,
}

/// Writes the diagnostic message for the duplicate export.
impl biome_console::fmt::Display for DuplicateExport {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        if self.kind == DuplicateKind::Default {
            fmt.write_markup(markup! { "There is more than one default export." })
        } else {
            fmt.write_markup(markup! {
                "The name "<Emphasis>{self.name.text()}</Emphasis>" is exported more than once."
            })
        }
    }
}

/// Pushes a [`DuplicateExport`] for each `export * from` statement that
/// exports a name that the module also exports itself, or that comes from a
/// different declaration than the same name from the earlier `export * from`
/// statements of the module.
///
/// `local_names` holds the names that the module exports itself.
fn push_export_all_conflicts(
    ctx: &RuleContext<NoDuplicateExports>,
    local_names: &FxHashMap<Text, FirstExports>,
    duplicates: &mut Vec<DuplicateExport>,
) {
    let Some(owner) = ctx.module_info_for_path(ctx.file_path()) else {
        return;
    };
    let ModuleInfoKind::Js(owner_info) = owner.kind(ctx.db()) else {
        return;
    };
    // For each name, the range of the first `export * from` that exports it,
    // and one target module for each declaration of the name found so far.
    let mut sources: FxHashMap<_, (_, Vec<_>)> = FxHashMap::default();
    for item in ctx.query() {
        let AnyJsModuleItem::JsExport(export) = item else {
            continue;
        };
        let Ok(AnyJsExportClause::JsExportFromClause(clause)) = export.export_clause() else {
            continue;
        };
        // `export * as ns from` exports a single name, which is checked with
        // the other exports of the module.
        if clause.export_as().is_some() {
            continue;
        }
        let Some(source) = clause
            .source()
            .ok()
            .and_then(|source| source.as_js_module_source().cloned())
        else {
            continue;
        };
        let range = source.range();
        let Some(target) = owner_info
            .get_import_path_by_js_node(&AnyJsImportLike::JsModuleSource(source))
            .and_then(|import_path| {
                import_path
                    .resolve_js(ctx.db(), owner)
                    .path()
                    .as_path()
                    .and_then(|path| ctx.module_info_for_path(path))
            })
        else {
            continue;
        };
        for name in names_exported_by(ctx.db(), target) {
            if let Some(local) = local_names.get(&name).and_then(FirstExports::first_range) {
                // The module's own export can be the same declaration, for
                // example in an import cycle, or when it re-exports the name
                // from the same module.
                if have_different_origins(ctx.db(), &name, owner, target) {
                    duplicates.push(DuplicateExport {
                        name,
                        range,
                        first: local,
                        kind: DuplicateKind::ReplacedExportAll,
                    });
                }
                continue;
            }
            match sources.entry(name) {
                Entry::Vacant(entry) => {
                    entry.insert((range, vec![target]));
                }
                Entry::Occupied(mut entry) => {
                    let name = entry.key().clone();
                    let (first, targets) = entry.get_mut();
                    // Like TypeScript, report each declaration of the name once.
                    if targets
                        .iter()
                        .all(|seen| have_different_origins(ctx.db(), &name, *seen, target))
                    {
                        targets.push(target);
                        duplicates.push(DuplicateExport {
                            name,
                            range,
                            first: *first,
                            kind: DuplicateKind::ExportAll,
                        });
                    }
                }
            }
        }
    }
}

/// Returns the names that `module` exports, including the names from its own
/// `export * from` statements, except `default`, which `export * from` doesn't
/// re-export.
fn names_exported_by(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<Text> {
    let mut names = Vec::new();
    let mut seen_names = FxHashSet::default();
    let mut seen_modules = FxHashSet::default();
    let mut stack = vec![module];
    while let Some(module) = stack.pop() {
        if !seen_modules.insert(module) {
            continue;
        }
        let ModuleInfoKind::Js(info) = module.kind(db) else {
            continue;
        };
        for name in info.exports.keys() {
            if name.text() != "default" && seen_names.insert(name.clone()) {
                names.push(name.clone());
            }
        }
        for reexport in &info.blanket_reexports {
            if let Some(target) = reexport
                .import
                .resolve_js(db, module)
                .path()
                .as_path()
                .and_then(|path| db.module_for_path(path))
            {
                stack.push(target);
            }
        }
    }
    names
}

/// Returns whether `name` comes from different declarations when it's
/// imported from `first` and from `second`.
///
/// Returns `false` when the declaration of either can't be found, because the
/// two may still be the same.
fn have_different_origins(
    db: &dyn ModuleDb,
    name: &Text,
    first: ModuleInfo,
    second: ModuleInfo,
) -> bool {
    match (
        export_origin(db, name.text(), first),
        export_origin(db, name.text(), second),
    ) {
        (Some(first), Some(second)) => first != second,
        _ => false,
    }
}

/// What an exported name refers to, after following re-exports and imports.
#[derive(PartialEq)]
enum ExportOrigin {
    /// A declaration in a module.
    Declaration(JsOwnExport, ModuleInfo),
    /// The namespace object of a module, such as the one that
    /// `export * as ns from` exports.
    Namespace(ModuleInfo),
}

/// Limits how many imports [`export_origin`] follows, because imports can form
/// a cycle.
const MAX_FOLLOWED_IMPORTS: usize = 32;

/// Returns what `name`, imported from `module`, refers to.
///
/// Unlike [`find_js_exported_symbol`], this also follows imports that are
/// exported again, such as `import { a } from "./a"; export { a };`.
fn export_origin(db: &dyn ModuleDb, name: &str, module: ModuleInfo) -> Option<ExportOrigin> {
    let mut name = name.to_string();
    let mut module = module;
    for _ in 0..MAX_FOLLOWED_IMPORTS {
        let JsExportedSymbolLookup::Found(export, owner) =
            find_js_exported_symbol(db, SymbolFromModuleInfo::new(db, name.as_str(), module))
                .clone()
        else {
            return None;
        };
        let ModuleInfoKind::Js(info) = owner.kind(db) else {
            return None;
        };
        let import = match &export {
            JsOwnExport::Binding(range) => info
                .semantic_model
                .as_binding_by_range(*range)
                .and_then(|binding| {
                    let local_name = binding.syntax().text_trimmed().to_string();
                    info.static_imports.get(local_name.as_str()).cloned()
                }),
            JsOwnExport::Namespace(reexport) => {
                let target = resolve_module(db, &reexport.import, owner)?;
                return Some(ExportOrigin::Namespace(target));
            }
            JsOwnExport::Type(_) => None,
        };
        let Some(import) = import else {
            return Some(ExportOrigin::Declaration(export, owner));
        };
        module = resolve_module(db, &import, owner)?;
        name = match &import.symbol {
            ImportSymbol::Named(imported) => imported.text().to_string(),
            ImportSymbol::Default => "default".to_string(),
            ImportSymbol::All => return Some(ExportOrigin::Namespace(module)),
        };
    }
    None
}

fn resolve_module(db: &dyn ModuleDb, import: &JsImport, owner: ModuleInfo) -> Option<ModuleInfo> {
    import
        .resolve_js(db, owner)
        .path()
        .as_path()
        .and_then(|path| db.module_for_path(path))
}

/// Whether a name in an export list, such as `export { a as b }`, is marked as
/// a type with `type`.
///
/// TypeScript lets a type share a name with a value, so whether a declaration
/// and a name from an export list conflict depends on this kind.
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy)]
enum SpecifierKind {
    Type = 1 << 0,
    Value = 1 << 1,
}

/// The earliest exports of one name, kept apart by where they come from,
/// because the rules for when two exports conflict depend on it.
#[derive(Default)]
struct FirstExports {
    /// Names from declarations, such as `export const`, and from `export default`.
    declarations: RangesByKind,
    /// Names from export lists, such as `export { a as b }`, and from
    /// `export * as`.
    specifiers: RangesByKind,
}

/// The range of the earliest export for each [`SpecifierKind`].
#[derive(Default)]
struct RangesByKind {
    types: Option<TextRange>,
    values: Option<TextRange>,
}

/// A name exported by an export statement.
struct ExportedName {
    /// The name with quotes removed and escape sequences decoded.
    name: Text,
    token: JsSyntaxToken,
    /// For a name from an export list, its kind. For a declaration, the kinds
    /// of names from export lists that conflict with it.
    kinds: BitFlags<SpecifierKind>,
    /// Whether the name comes from a declaration, such as `export const`, or
    /// from `export default`, rather than from a list of names such as
    /// `export { a as b }` or from `export * as`.
    is_declaration: bool,
}

/// Calls `f` for each name that `clause` exports.
///
/// `export * from "..."`, `export =` and `export as namespace` don't add any
/// name to the module's own exports, so they are skipped.
fn for_each_exported_name(clause: &AnyJsExportClause, mut f: impl FnMut(ExportedName)) {
    match clause {
        AnyJsExportClause::AnyJsDeclarationClause(declaration) => {
            for_each_declared_name(declaration, &mut f);
        }
        AnyJsExportClause::TsExportDeclareClause(clause) => {
            if let Ok(declaration) = clause.declaration() {
                for_each_declared_name(&declaration, &mut f);
            }
        }
        AnyJsExportClause::JsExportDefaultDeclarationClause(clause) => {
            let kinds = match clause.declaration() {
                Ok(AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(_)) => {
                    SpecifierKind::Type.into()
                }
                Ok(AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)) => {
                    SpecifierKind::Value.into()
                }
                _ => BitFlags::all(),
            };
            if let Ok(token) = clause.default_token() {
                f(ExportedName::new_declaration(token, kinds));
            }
        }
        AnyJsExportClause::JsExportDefaultExpressionClause(clause) => {
            if let Ok(token) = clause.default_token() {
                f(ExportedName::new_declaration(token, BitFlags::all()));
            }
        }
        AnyJsExportClause::JsExportNamedClause(clause) => {
            for specifier in clause.specifiers().into_iter().flatten() {
                let token = match &specifier {
                    AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(specifier) => {
                        specifier.name().and_then(|name| name.value_token())
                    }
                    AnyJsExportNamedSpecifier::JsExportNamedSpecifier(specifier) => specifier
                        .exported_name()
                        .and_then(|name| name.value()),
                };
                let kind = if specifier.exports_only_types() {
                    SpecifierKind::Type
                } else {
                    SpecifierKind::Value
                };
                if let Ok(token) = token {
                    f(ExportedName::new_specifier(token, kind.into()));
                }
            }
        }
        AnyJsExportClause::JsExportNamedFromClause(clause) => {
            let is_type_clause = clause.type_token().is_some();
            for specifier in clause.specifiers().into_iter().flatten() {
                let kind = if is_type_clause || specifier.type_token().is_some() {
                    SpecifierKind::Type
                } else {
                    SpecifierKind::Value
                };
                if let Some(token) = exported_from_name(&specifier) {
                    f(ExportedName::new_specifier(token, kind.into()));
                }
            }
        }
        AnyJsExportClause::JsExportFromClause(clause) => {
            let Some(token) = clause
                .export_as()
                .and_then(|export_as| export_as.exported_name().ok())
                .and_then(|name| name.value().ok())
            else {
                return;
            };
            // `export type * as ns` exports a namespace, which is never a type,
            // so it conflicts with the same declarations as `export * as ns`.
            f(ExportedName::new_specifier(token, SpecifierKind::Value.into()));
        }
        AnyJsExportClause::TsExportAsNamespaceClause(_)
        | AnyJsExportClause::TsExportAssignmentClause(_) => {}
    }
}

/// Calls `f` for each name that `declaration` declares.
fn for_each_declared_name(declaration: &AnyJsDeclarationClause, f: &mut impl FnMut(ExportedName)) {
    let (token, kinds) = match declaration {
        AnyJsDeclarationClause::JsClassDeclaration(declaration) => (
            declaration.id().ok().as_ref().and_then(binding_token),
            BitFlags::all(),
        ),
        AnyJsDeclarationClause::JsFunctionDeclaration(declaration) => (
            declaration.id().ok().as_ref().and_then(binding_token),
            BitFlags::all(),
        ),
        AnyJsDeclarationClause::TsDeclareFunctionDeclaration(declaration) => {
            (declaration.id().ok().as_ref().and_then(binding_token), SpecifierKind::Value.into())
        }
        AnyJsDeclarationClause::TsEnumDeclaration(declaration) => (
            declaration.id().ok().as_ref().and_then(binding_token),
            BitFlags::all(),
        ),
        AnyJsDeclarationClause::TsImportEqualsDeclaration(declaration) => {
            let kinds = if declaration.type_token().is_some() {
                SpecifierKind::Type.into()
            } else {
                BitFlags::all()
            };
            (declaration.id().ok().as_ref().and_then(binding_token), kinds)
        }
        AnyJsDeclarationClause::TsTypeAliasDeclaration(declaration) => (
            declaration
                .binding_identifier()
                .ok()
                .as_ref()
                .and_then(ts_binding_token),
            SpecifierKind::Type.into(),
        ),
        AnyJsDeclarationClause::TsInterfaceDeclaration(declaration) => (
            declaration.id().ok().as_ref().and_then(ts_binding_token),
            SpecifierKind::Type.into(),
        ),
        AnyJsDeclarationClause::TsModuleDeclaration(declaration) => (
            declaration.name().ok().and_then(module_name_token),
            SpecifierKind::Value.into(),
        ),
        AnyJsDeclarationClause::JsVariableDeclarationClause(clause) => {
            let Ok(declaration) = clause.declaration() else {
                return;
            };
            for declarator in declaration.declarators().into_iter().flatten() {
                if let Ok(pattern) = declarator.id() {
                    for_each_pattern_binding(&pattern, |token| {
                        f(ExportedName::new_declaration(token, BitFlags::all()));
                    });
                }
            }
            return;
        }
        // `declare module "foo" {}` and `declare global {}` don't export a name.
        AnyJsDeclarationClause::TsExternalModuleDeclaration(_)
        | AnyJsDeclarationClause::TsGlobalDeclaration(_) => return,
    };
    if let Some(token) = token {
        f(ExportedName::new_declaration(token, kinds));
    }
}

/// Calls `f` with the name token of each variable that `pattern` declares.
fn for_each_pattern_binding(pattern: &AnyJsBindingPattern, mut f: impl FnMut(JsSyntaxToken)) {
    let mut preorder = pattern.syntax().preorder_of_kinds(PATTERN_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        // Default values and computed keys can contain functions, whose
        // parameters aren't exported.
        if let Some(binding) = JsIdentifierBinding::cast(node)
            && let Ok(token) = binding.name_token()
        {
            f(token);
        }
        preorder.skip_subtree();
    }
}

/// Returns the name token of the exported name in `export { a as b } from "..."`.
fn exported_from_name(specifier: &JsExportNamedFromSpecifier) -> Option<JsSyntaxToken> {
    let name = match specifier.export_as() {
        Some(export_as) => export_as.exported_name().ok()?,
        None => specifier.source_name().ok()?,
    };
    literal_export_name_token(&name)
}

fn literal_export_name_token(name: &AnyJsLiteralExportName) -> Option<JsSyntaxToken> {
    name.as_js_literal_export_name()?.value().ok()
}

fn binding_token(binding: &AnyJsBinding) -> Option<JsSyntaxToken> {
    binding.as_js_identifier_binding()?.name_token().ok()
}

fn ts_binding_token(binding: &AnyTsIdentifierBinding) -> Option<JsSyntaxToken> {
    binding.as_ts_identifier_binding()?.name_token().ok()
}

/// Returns the first part of a namespace name, which is the name that
/// `export namespace A.B.C {}` exports.
fn module_name_token(name: AnyTsModuleName) -> Option<JsSyntaxToken> {
    let mut name = name;
    loop {
        match name {
            AnyTsModuleName::AnyTsIdentifierBinding(binding) => return ts_binding_token(&binding),
            AnyTsModuleName::TsQualifiedModuleName(qualified) => name = qualified.left().ok()?,
        }
    }
}

impl ExportedName {
    fn new_declaration(token: JsSyntaxToken, kinds: BitFlags<SpecifierKind>) -> Self {
        Self::new(token, kinds, true)
    }

    fn new_specifier(token: JsSyntaxToken, kinds: BitFlags<SpecifierKind>) -> Self {
        Self::new(token, kinds, false)
    }

    fn new(token: JsSyntaxToken, kinds: BitFlags<SpecifierKind>, is_declaration: bool) -> Self {
        Self {
            name: unescape_js_string(inner_string_text(&token)),
            token,
            kinds,
            is_declaration,
        }
    }
}

impl FirstExports {
    /// Returns the range of the earliest export of the name.
    fn first_range(&self) -> Option<TextRange> {
        earliest(
            self.declarations.first_in(BitFlags::all()),
            self.specifiers.first_in(BitFlags::all()),
        )
    }
}

impl RangesByKind {
    /// Returns the earliest range recorded for any of `kinds`.
    fn first_in(&self, kinds: BitFlags<SpecifierKind>) -> Option<TextRange> {
        earliest(
            self.types.filter(|_| kinds.contains(SpecifierKind::Type)),
            self.values.filter(|_| kinds.contains(SpecifierKind::Value)),
        )
    }

    /// Records `range` for each of `kinds` that has no range yet.
    fn insert(&mut self, kinds: BitFlags<SpecifierKind>, range: TextRange) {
        if kinds.contains(SpecifierKind::Type) {
            self.types.get_or_insert(range);
        }
        if kinds.contains(SpecifierKind::Value) {
            self.values.get_or_insert(range);
        }
    }
}

fn earliest(a: Option<TextRange>, b: Option<TextRange>) -> Option<TextRange> {
    a.into_iter().chain(b).min_by_key(|range| range.start())
}

/// The nodes that the search for variable names stops at: variable names, and
/// expressions, whose names belong to nested functions or classes.
const PATTERN_SEARCH_KINDS: SyntaxKindSet<JsLanguage> =
    JsIdentifierBinding::KIND_SET.union(AnyJsExpression::KIND_SET);
