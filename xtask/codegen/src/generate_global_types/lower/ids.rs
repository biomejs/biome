//! Global identities derived from the declaration manifest.
//!
//! Every global declaration group receives an identity, so any declared name can be
//! looked up by type inference. Groups combining `interface X` with
//! `declare var X: XConstructor` share one identity holding a class, which is what a
//! `declare class X` would produce.

use std::collections::BTreeMap;

use anyhow::{Result, bail};
use biome_js_syntax::{AnyTsName, AnyTsType, AnyTsTypeMember, AnyTsVariableAnnotation};
use biome_rowan::Text;
use biome_string_case::Case;

use super::{ParsedSourceCache, is_unique_symbol_property, lower_object_member_name};
use crate::generate_global_types::{
    collect::{DeclarationKind, ScopePath},
    manifest::{GlobalDeclarationGroup, GlobalDeclarationRole, GlobalManifest},
};

/// What a global identity stands for.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlobalSlotKind {
    /// Interfaces and type aliases.
    Type,
    /// Variables and functions.
    Value,
    /// `interface X` merged with `declare var X: C`, where `C` names another interface.
    ///
    /// The identity serves both roles: the class's instance members come from `X`,
    /// and its static members and signatures come from `C`.
    Class { constructor: Text },
    /// Values declared in a `declare namespace` block.
    Namespace,
    /// A `unique symbol` property of a global value's interface, such as `Symbol.iterator`.
    UniqueSymbol,
}

/// One global identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalSlot {
    /// Namespace path followed by the declared name.
    path: Box<[Text]>,
    /// Upper snake case name used to build the Rust constants.
    stem: String,
    kind: GlobalSlotKind,
}

impl GlobalSlot {
    /// Qualified name, such as `Intl.DateTimeFormat`.
    pub fn name(&self) -> String {
        join_path(&self.path)
    }

    /// Declared name without its namespace path.
    pub fn declared_name(&self) -> &Text {
        self.path.last().expect("global paths are never empty")
    }

    /// Namespace path containing the declaration.
    pub fn namespace(&self) -> &[Text] {
        &self.path[..self.path.len() - 1]
    }

    /// Manifest scope containing the declaration.
    pub fn scope(&self) -> ScopePath {
        scope_for(self.namespace())
    }

    pub fn kind(&self) -> &GlobalSlotKind {
        &self.kind
    }

    /// `GlobalTypeId` constant name.
    pub fn id_constant(&self) -> String {
        format!("{}_ID_GLOBAL_TYPE_ID", self.stem)
    }

    /// `RawTypeId` constant name.
    pub fn reference_constant(&self) -> String {
        format!("GLOBAL_{}_ID", self.stem)
    }

    /// Whether type annotations can name this identity.
    pub fn has_type_role(&self) -> bool {
        matches!(
            self.kind,
            GlobalSlotKind::Type | GlobalSlotKind::Class { .. }
        )
    }

    /// Whether expressions can name this identity.
    pub fn has_value_role(&self) -> bool {
        !matches!(self.kind, GlobalSlotKind::Type)
    }
}

/// All global identities in ID order, with lookups by qualified name.
#[derive(Debug, Default)]
pub struct GlobalIds {
    slots: Vec<GlobalSlot>,
    types: BTreeMap<String, usize>,
    values: BTreeMap<String, usize>,
    /// Maps an interface's qualified name and property name to a `UniqueSymbol` slot.
    unique_symbols: BTreeMap<(String, String), usize>,
    /// Maps the qualified name of a class's constructor interface to the class slot.
    constructors: BTreeMap<String, usize>,
}

impl GlobalIds {
    /// Assigns identities to every global and namespace group in source order.
    pub(super) fn assign(
        manifest: &GlobalManifest,
        sources: &mut ParsedSourceCache,
    ) -> Result<Self> {
        let mut ids = Self::default();
        let mut namespaces = Vec::new();

        for group in manifest.groups() {
            let ScopePath::Namespace(namespace) = group.scope() else {
                if group.scope() != &ScopePath::Global {
                    continue;
                }
                ids.push_group(group, &[], sources)?;
                continue;
            };
            for length in 1..=namespace.len() {
                let path = &namespace[..length];
                if !namespaces.iter().any(|known: &Vec<Text>| known == path) {
                    namespaces.push(path.to_vec());
                }
            }
            ids.push_group(group, namespace, sources)?;
        }

        for path in namespaces {
            let name = join_path(&path);
            if ids.values.contains_key(&name) {
                // A value with the same name already owns the value role. Namespace
                // members remain reachable through their qualified names.
                continue;
            }
            ids.push(path.into_boxed_slice(), GlobalSlotKind::Namespace)?;
        }

        ids.assign_unique_symbols(manifest, sources)?;
        Ok(ids)
    }

    pub fn slots(&self) -> &[GlobalSlot] {
        &self.slots
    }

    pub fn slot(&self, index: usize) -> &GlobalSlot {
        &self.slots[index]
    }

    /// Finds the identity that type annotations resolve `name` to.
    pub fn type_slot(&self, name: &str) -> Option<usize> {
        self.types.get(name).copied()
    }

    /// Finds the identity that expressions resolve `name` to.
    pub fn value_slot(&self, name: &str) -> Option<usize> {
        self.values.get(name).copied()
    }

    /// Finds the class whose value has the type of the `constructor` interface.
    pub fn class_for_constructor(&self, constructor: &str) -> Option<usize> {
        self.constructors.get(constructor).copied()
    }

    /// Finds the identity of a `unique symbol` property declared by `interface`.
    pub fn unique_symbol(&self, interface: &str, property: &str) -> Option<usize> {
        self.unique_symbols
            .get(&(interface.to_owned(), property.to_owned()))
            .copied()
    }

    fn push_group(
        &mut self,
        group: &GlobalDeclarationGroup,
        namespace: &[Text],
        sources: &mut ParsedSourceCache,
    ) -> Result<()> {
        let mut path = namespace.to_vec();
        path.push(group.name().clone());
        let path = path.into_boxed_slice();
        let has_type = group.has_role(GlobalDeclarationRole::Type);
        let has_value = group.has_role(GlobalDeclarationRole::Value);
        if has_type && has_value {
            if let Some(constructor) = class_constructor(group, sources)? {
                let mut constructor_path = namespace.to_vec();
                constructor_path.push(constructor.clone());
                self.constructors
                    .insert(join_path(&constructor_path), self.slots.len());
                return self.push(path, GlobalSlotKind::Class { constructor });
            }
            self.push(path.clone(), GlobalSlotKind::Type)?;
            return self.push_with_stem(path, GlobalSlotKind::Value, "_VALUE");
        }
        let kind = if has_type {
            GlobalSlotKind::Type
        } else {
            GlobalSlotKind::Value
        };
        self.push(path, kind)
    }

    fn push(&mut self, path: Box<[Text]>, kind: GlobalSlotKind) -> Result<()> {
        self.push_with_stem(path, kind, "")
    }

    fn push_with_stem(
        &mut self,
        path: Box<[Text]>,
        kind: GlobalSlotKind,
        suffix: &str,
    ) -> Result<()> {
        let stem = format!("{}{suffix}", stem_for(&path));
        if let Some(existing) = self.slots.iter().find(|slot| slot.stem == stem) {
            bail!(
                "globals {} and {} both map to the Rust name {stem}",
                existing.name(),
                join_path(&path)
            );
        }
        let index = self.slots.len();
        let slot = GlobalSlot { path, stem, kind };
        let name = slot.name();
        if slot.has_type_role() {
            self.types.insert(name.clone(), index);
        }
        if slot.has_value_role() {
            self.values.insert(name, index);
        }
        self.slots.push(slot);
        Ok(())
    }

    /// Gives each `unique symbol` property of a global value's interface its own identity.
    fn assign_unique_symbols(
        &mut self,
        manifest: &GlobalManifest,
        sources: &mut ParsedSourceCache,
    ) -> Result<()> {
        let owners = self
            .slots
            .iter()
            .filter(|slot| {
                matches!(
                    slot.kind,
                    GlobalSlotKind::Class { .. } | GlobalSlotKind::Value
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        for owner in owners {
            let Some(group) = manifest.group(&owner.scope(), owner.declared_name().text()) else {
                continue;
            };
            let interface = match &owner.kind {
                GlobalSlotKind::Class { constructor } => constructor.clone(),
                _ => match value_annotation_name(group, sources)? {
                    Some(name) => name,
                    None => continue,
                },
            };
            let Some(interface_group) = manifest.group(&owner.scope(), interface.text()) else {
                continue;
            };
            let mut interface_path = owner.namespace().to_vec();
            interface_path.push(interface.clone());
            let interface_name = join_path(&interface_path);
            for record in interface_group.declarations() {
                if record.kind != DeclarationKind::Interface {
                    continue;
                }
                let Some(declaration) = sources.find_interface_declaration(record)? else {
                    continue;
                };
                for member in declaration.members() {
                    let AnyTsTypeMember::TsPropertySignatureTypeMember(property) = member else {
                        continue;
                    };
                    if !is_unique_symbol_property(&property)? {
                        continue;
                    }
                    let Ok(name) = lower_object_member_name(property.name()?) else {
                        continue;
                    };
                    let key = (interface_name.clone(), name.text().to_owned());
                    if self.unique_symbols.contains_key(&key) {
                        continue;
                    }
                    let mut path = owner.path.to_vec();
                    path.push(name);
                    self.push(path.into_boxed_slice(), GlobalSlotKind::UniqueSymbol)?;
                    self.unique_symbols.insert(key, self.slots.len() - 1);
                }
            }
        }
        Ok(())
    }
}

/// Returns the interface named by `declare var X: C` when `C` is another interface in
/// the same scope.
fn class_constructor(
    group: &GlobalDeclarationGroup,
    sources: &mut ParsedSourceCache,
) -> Result<Option<Text>> {
    if group.declarations().iter().any(|record| {
        !matches!(
            record.kind,
            DeclarationKind::Interface | DeclarationKind::VariableDeclarator { .. }
        )
    }) {
        return Ok(None);
    }
    let Some(constructor) = value_annotation_name(group, sources)? else {
        return Ok(None);
    };
    if &constructor == group.name() {
        return Ok(None);
    }
    Ok(Some(constructor))
}

/// Returns the plain type reference annotating every variable in `group`, if they agree.
fn value_annotation_name(
    group: &GlobalDeclarationGroup,
    sources: &mut ParsedSourceCache,
) -> Result<Option<Text>> {
    let mut name = None;
    for record in group.declarations() {
        if !matches!(record.kind, DeclarationKind::VariableDeclarator { .. }) {
            continue;
        }
        let Some(declarator) = sources.find_variable_declarator(record)? else {
            return Ok(None);
        };
        let Some(AnyTsVariableAnnotation::TsTypeAnnotation(annotation)) =
            declarator.variable_annotation()
        else {
            return Ok(None);
        };
        let AnyTsType::TsReferenceType(reference) = annotation.ty()? else {
            return Ok(None);
        };
        if reference.type_arguments().is_some() {
            return Ok(None);
        }
        let AnyTsName::JsReferenceIdentifier(identifier) = reference.name()? else {
            return Ok(None);
        };
        let current = Text::from(identifier.value_token()?.token_text_trimmed());
        if name.as_ref().is_some_and(|previous| previous != &current) {
            return Ok(None);
        }
        name = Some(current);
    }
    Ok(name)
}

/// Joins a namespace path and name with dots.
pub(super) fn join_path(path: &[Text]) -> String {
    path.iter().map(Text::text).collect::<Vec<_>>().join(".")
}

/// Returns the manifest scope for a namespace path.
pub(super) fn scope_for(namespace: &[Text]) -> ScopePath {
    if namespace.is_empty() {
        ScopePath::Global
    } else {
        ScopePath::Namespace(namespace.to_vec())
    }
}

fn stem_for(path: &[Text]) -> String {
    path.iter()
        .map(|part| Case::Constant.convert(part.text()))
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_global_types::{
        collect::collect,
        manifest::build_global_manifest,
        source::{CanonicalPath, DiscoveredFile},
    };

    fn assign(source: &str) -> Result<GlobalIds> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(
                root,
                "tests/fixtures/global-types/lowering.interfaces.d.ts",
            )?,
            repo_relative: "ids.d.ts".to_owned(),
            bytes: source.as_bytes().to_vec(),
        };
        let manifest = build_global_manifest(collect(&file).records);
        let files = [file];
        let mut sources = ParsedSourceCache::new(&files);
        GlobalIds::assign(&manifest, &mut sources)
    }

    #[test]
    fn every_declaration_group_gets_an_identity() -> Result<()> {
        let ids = assign(
            "interface WeakKeyTypes { object: object; }
            type WeakKey = WeakKeyTypes[keyof WeakKeyTypes];
            interface Box<T> { value: T; }
            interface BoxConstructor { new <T>(value: T): Box<T>; readonly marker: unique symbol; }
            declare var Box: BoxConstructor;
            interface Settings { verbose: boolean; }
            declare var Settings: Settings;
            declare function parseValue(input: string): number;
            declare namespace Formats { interface Options { style: string; } var Default: Options; }",
        )?;
        let names = ids
            .slots()
            .iter()
            .map(|slot| (slot.name(), slot.id_constant(), slot.kind().clone()))
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                (
                    "WeakKeyTypes".to_owned(),
                    "WEAK_KEY_TYPES_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Type
                ),
                (
                    "WeakKey".to_owned(),
                    "WEAK_KEY_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Type
                ),
                (
                    "Box".to_owned(),
                    "BOX_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Class {
                        constructor: Text::from("BoxConstructor")
                    }
                ),
                (
                    "BoxConstructor".to_owned(),
                    "BOX_CONSTRUCTOR_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Type
                ),
                (
                    "Settings".to_owned(),
                    "SETTINGS_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Type
                ),
                (
                    "Settings".to_owned(),
                    "SETTINGS_VALUE_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Value
                ),
                (
                    "parseValue".to_owned(),
                    "PARSE_VALUE_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Value
                ),
                (
                    "Formats.Options".to_owned(),
                    "FORMATS_OPTIONS_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Type
                ),
                (
                    "Formats.Default".to_owned(),
                    "FORMATS_DEFAULT_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Value
                ),
                (
                    "Formats".to_owned(),
                    "FORMATS_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::Namespace
                ),
                (
                    "Box.marker".to_owned(),
                    "BOX_MARKER_ID_GLOBAL_TYPE_ID".to_owned(),
                    GlobalSlotKind::UniqueSymbol
                ),
            ]
        );

        assert_eq!(ids.type_slot("WeakKeyTypes"), Some(0));
        assert_eq!(ids.value_slot("WeakKeyTypes"), None);
        assert_eq!(ids.type_slot("Box"), ids.value_slot("Box"));
        assert_eq!(ids.type_slot("Settings"), Some(4));
        assert_eq!(ids.value_slot("Settings"), Some(5));
        assert_eq!(ids.type_slot("Formats"), None);
        assert_eq!(ids.value_slot("Formats"), Some(9));
        assert_eq!(ids.unique_symbol("BoxConstructor", "marker"), Some(10));
        Ok(())
    }

    #[test]
    fn colliding_rust_names_are_rejected() {
        let error = assign("interface RegExp {} interface Reg_Exp {}").unwrap_err();
        assert!(
            error
                .to_string()
                .contains("both map to the Rust name REG_EXP"),
            "{error}"
        );
    }
}
