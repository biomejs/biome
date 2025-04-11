use biome_js_syntax::{
    AnyJsCombinedSpecifier, AnyJsExportClause, AnyJsImportClause, AnyJsModuleItem,
    AnyJsModuleSource, JsExport, JsImport, JsImportAssertion,
};
use biome_rowan::AstNode;

use super::{
    TypePlacement, comparable_token::ComparableToken, import_groups, import_source,
    specifiers_attributes::JsNamedSpecifiers,
};

/// Type used to determine the order between imports
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ImportKey {
    pub section: ImportSection,
    pub group: u16,
    pub source: import_source::ImportSource<ComparableToken>,
    pub has_no_attributes: bool,
    pub kind: ImportStatementKind,
    /// Slot index of the import in the module.
    /// This is used as a last resort for ensuring a strict total order between imports.
    pub slot_index: u32,
}
impl ImportKey {
    pub fn new(
        info: ImportInfo,
        groups: &import_groups::ImportGroups,
        type_placement: TypePlacement,
    ) -> Self {
        let section = match type_placement {
            TypePlacement::TypesFirst if info.kind.has_type_token() => ImportSection::TypesFirst,
            _ => ImportSection::Mixed,
        };
        Self {
            section,
            group: groups.index(&info),
            source: info.source,
            has_no_attributes: info.has_no_attributes,
            kind: info.kind,
            slot_index: info.slot_index,
        }
    }

    pub fn is_mergeable(&self, other: &Self) -> bool {
        self.source == other.source
            && self.kind.is_mergeable(other.kind.into())
            && self.has_no_attributes
            && other.has_no_attributes
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum ImportSection {
    /// Section reserved for types with the [`TypePlacement::TypesFirst`] setting.
    TypesFirst,
    /// Section for mixed imports.
    #[default]
    Mixed,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[enumflags2::bitflags]
#[repr(u8)]
pub enum ImportStatementKind {
    NamespaceType = 1 << 0,
    DefaultType = 1 << 1,
    NamedType = 1 << 2,
    Namespace = 1 << 3,
    DefaultNamespace = 1 << 4,
    Default = 1 << 5,
    DefaultNamed = 1 << 6,
    Named = 1 << 7,
}
impl ImportStatementKind {
    pub fn has_type_token(self) -> bool {
        (ImportStatementKind::DefaultType
            | ImportStatementKind::NamespaceType
            | ImportStatementKind::NamedType)
            .contains(self)
    }

    pub fn is_mergeable(self, kinds: enumflags2::BitFlags<ImportStatementKind>) -> bool {
        match self {
            ImportStatementKind::Namespace => kinds.contains(ImportStatementKind::Default),
            ImportStatementKind::Default => {
                kinds.intersects(ImportStatementKind::Namespace | ImportStatementKind::Named)
            }
            ImportStatementKind::DefaultNamed => kinds.contains(ImportStatementKind::Named),
            ImportStatementKind::Named => {
                kinds.intersects(ImportStatementKind::DefaultNamed | ImportStatementKind::Named)
            }
            ImportStatementKind::NamedType => kinds.contains(ImportStatementKind::NamedType),
            _ => false,
        }
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
