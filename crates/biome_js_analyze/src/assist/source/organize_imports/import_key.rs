use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsExportClause, AnyJsImportClause, AnyJsModuleItem,
    AnyJsModuleSource, JsExport, JsImport, JsImportAssertion,
};
use biome_rowan::AstNode;

use super::{
    comparable_token::ComparableToken, import_groups, import_source,
    specifiers_attributes::JsNamedSpecifiers,
};

/// Type used to determine the order between imports
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ImportKey {
    pub group: u16,
    pub source: import_source::ImportSource<ComparableToken>,
    pub has_no_attributes: bool,
    pub kind: ImportStatementKind,
    /// Slot index of the import in the module.
    /// This is used as a last resort for ensuring a strict total order between imports.
    pub slot_index: u32,
}
impl ImportKey {
    pub fn new(info: ImportInfo, groups: &import_groups::ImportGroups) -> Self {
        let candidate = import_groups::ImportSourceCandidate::new(info.source.inner().0.text());
        Self {
            group: groups.index(&candidate),
            source: info.source,
            has_no_attributes: info.has_no_attributes,
            kind: info.kind,
            slot_index: info.slot_index,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[enumflags2::bitflags]
#[repr(u8)]
pub enum ImportStatementKind {
    DefaultType = 1 << 0,
    Default = 1 << 1,
    DefaultNamespace = 1 << 2,
    DefaultNamed = 1 << 3,
    NamespaceType = 1 << 4,
    Namespace = 1 << 5,
    NamedType = 1 << 6,
    Named = 1 << 7,
}
impl ImportStatementKind {
    pub fn has_type_token(self) -> bool {
        (ImportStatementKind::DefaultType
            | ImportStatementKind::NamespaceType
            | ImportStatementKind::NamedType)
            .contains(self)
    }

    pub fn is_mergeable(self, kinds: ImportStatementKinds) -> bool {
        match self {
            ImportStatementKind::DefaultNamed => kinds.contains(ImportStatementKind::Named),
            ImportStatementKind::Named => kinds
                .0
                .intersects(ImportStatementKind::DefaultNamed | ImportStatementKind::Named),
            ImportStatementKind::NamedType => kinds.contains(ImportStatementKind::NamedType),
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ImportStatementKinds(enumflags2::BitFlags<ImportStatementKind>);
impl ImportStatementKinds {
    pub fn contains(self, kind: ImportStatementKind) -> bool {
        self.0.contains(kind)
    }

    pub fn insert(&mut self, kind: ImportStatementKind) {
        self.0 |= kind;
    }

    pub fn clear(&mut self) {
        self.0 = Default::default();
    }
}

/// Type that gathers information extracted from an import or an export.
#[derive(Debug)]
pub struct ImportInfo {
    /// Slot index of the import in the module.
    pub slot_index: u32,
    pub kind: ImportStatementKind,
    pub source: import_source::ImportSource<ComparableToken>,
    pub has_no_attributes: bool,
}
impl ImportInfo {
    pub fn from_module_item(
        item: &AnyJsModuleItem,
    ) -> Option<(Self, Option<JsNamedSpecifiers>, Option<JsImportAssertion>)> {
        match item {
            AnyJsModuleItem::AnyJsStatement(_) => None,
            AnyJsModuleItem::JsExport(export) => Self::from_export(export),
            AnyJsModuleItem::JsImport(import) => Self::from_import(import),
        }
    }

    fn from_import(
        value: &JsImport,
    ) -> Option<(Self, Option<JsNamedSpecifiers>, Option<JsImportAssertion>)> {
        let (kind, named_specifiers, source, attributes) = match value.import_clause().ok()? {
            AnyJsImportClause::JsImportBareClause(_) => {
                return None;
            }
            AnyJsImportClause::JsImportCombinedClause(clause) => {
                let (kind, named_specifiers) = match clause.specifier().ok()? {
                    AnyJsCombinedSpecifier::JsNamedImportSpecifiers(specifiers) => {
                        (ImportStatementKind::DefaultNamed, Some(specifiers))
                    }
                    AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(_) => {
                        (ImportStatementKind::DefaultNamespace, None)
                    }
                };
                (kind, named_specifiers, clause.source(), clause.assertion())
            }
            AnyJsImportClause::JsImportDefaultClause(clause) => (
                if clause.type_token().is_some() {
                    ImportStatementKind::DefaultType
                } else {
                    ImportStatementKind::Default
                },
                None,
                clause.source(),
                clause.assertion(),
            ),
            AnyJsImportClause::JsImportNamedClause(clause) => {
                let named_specifiers = clause.named_specifiers().ok();
                (
                    if clause.type_token().is_some() {
                        ImportStatementKind::NamedType
                    } else {
                        ImportStatementKind::Named
                    },
                    named_specifiers,
                    clause.source(),
                    clause.assertion(),
                )
            }
            AnyJsImportClause::JsImportNamespaceClause(clause) => (
                if clause.type_token().is_some() {
                    ImportStatementKind::NamespaceType
                } else {
                    ImportStatementKind::Namespace
                },
                None,
                clause.source(),
                clause.assertion(),
            ),
        };
        let Ok(AnyJsModuleSource::JsModuleSource(source)) = source else {
            return None;
        };
        Some((
            Self {
                source: ComparableToken(source.inner_string_text().ok()?).into(),
                has_no_attributes: attributes.is_none(),
                kind,
                slot_index: value.syntax().index() as u32,
            },
            named_specifiers.map(JsNamedSpecifiers::JsNamedImportSpecifiers),
            attributes,
        ))
    }

    fn from_export(
        value: &JsExport,
    ) -> Option<(Self, Option<JsNamedSpecifiers>, Option<JsImportAssertion>)> {
        let (kind, _first_local_name, named_specifiers, source, attributes) =
            match value.export_clause().ok()? {
                AnyJsExportClause::JsExportFromClause(clause) => (
                    if clause.type_token().is_some() {
                        ImportStatementKind::NamespaceType
                    } else {
                        ImportStatementKind::Namespace
                    },
                    clause
                        .export_as()
                        .and_then(|export_as| export_as.exported_name().ok()),
                    None,
                    clause.source(),
                    clause.assertion(),
                ),
                AnyJsExportClause::JsExportNamedFromClause(clause) => (
                    if clause.type_token().is_some() {
                        ImportStatementKind::NamedType
                    } else {
                        ImportStatementKind::Named
                    },
                    clause
                        .specifiers()
                        .into_iter()
                        .flatten()
                        .next()
                        .and_then(|x| x.source_name().ok()),
                    Some(clause.specifiers()),
                    clause.source(),
                    clause.assertion(),
                ),
                _ => {
                    return None;
                }
            };
        let Ok(AnyJsModuleSource::JsModuleSource(source)) = source else {
            return None;
        };
        let source = source.inner_string_text().ok()?;
        Some((
            Self {
                source: ComparableToken(source).into(),
                has_no_attributes: attributes.is_none(),
                kind,
                slot_index: value.syntax().index() as u32,
            },
            named_specifiers.map(JsNamedSpecifiers::JsExportNamedFromSpecifierList),
            attributes,
        ))
    }
}
