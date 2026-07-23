//! Role-aware declaration manifest for global types codegen.

use biome_rowan::Text;

use crate::generate_global_types::collect::{DeclarationKind, DeclarationRecord, ScopePath};

/// Role a declaration contributes to a global name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GlobalDeclarationRole {
    Type,
    Value,
}

/// Ordered manifest of global declaration groups.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalManifest {
    groups: Box<[GlobalDeclarationGroup]>,
}

impl GlobalManifest {
    /// Returns the global group with the given name.
    pub fn global_group(&self, name: &str) -> Option<&GlobalDeclarationGroup> {
        self.groups.iter().find(|group| group.name.text() == name)
    }
}

/// Declarations for one global name and scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalDeclarationGroup {
    scope: ScopePath,
    name: Text,
    declarations: Box<[DeclarationRecord]>,
    has_type: bool,
    has_value: bool,
}

impl GlobalDeclarationGroup {
    /// Source-order declarations that contributed to this group.
    pub fn declarations(&self) -> &[DeclarationRecord] {
        &self.declarations
    }

    /// Returns whether this group has the requested declaration role.
    pub fn has_role(&self, role: GlobalDeclarationRole) -> bool {
        match role {
            GlobalDeclarationRole::Type => self.has_type,
            GlobalDeclarationRole::Value => self.has_value,
        }
    }
}

/// Mutable builder used while records for one global group are collected.
struct GlobalDeclarationGroupBuilder {
    scope: ScopePath,
    name: Text,
    pending_declarations: Vec<DeclarationRecord>,
    has_type: bool,
    has_value: bool,
}

impl GlobalDeclarationGroupBuilder {
    /// Starts a group from its first declaration.
    fn new(record: DeclarationRecord) -> Self {
        let role = role_for_kind(&record.kind);
        Self {
            scope: record.scope.clone(),
            name: Text::from(record.declared_name.clone()),
            pending_declarations: vec![record],
            has_type: role == GlobalDeclarationRole::Type,
            has_value: role == GlobalDeclarationRole::Value,
        }
    }

    /// Adds another declaration to the group.
    fn push(&mut self, record: DeclarationRecord) {
        match role_for_kind(&record.kind) {
            GlobalDeclarationRole::Type => self.has_type = true,
            GlobalDeclarationRole::Value => self.has_value = true,
        }
        self.pending_declarations.push(record);
    }

    /// Freezes the builder into the manifest representation.
    fn into_group(self) -> GlobalDeclarationGroup {
        GlobalDeclarationGroup {
            scope: self.scope,
            name: self.name,
            declarations: self.pending_declarations.into_boxed_slice(),
            has_type: self.has_type,
            has_value: self.has_value,
        }
    }
}

/// Builds the global manifest from collector records.
pub fn build_global_manifest(records: Vec<DeclarationRecord>) -> GlobalManifest {
    let mut groups: Vec<GlobalDeclarationGroupBuilder> = Vec::new();

    for record in records {
        if !is_global_scope(&record.scope) {
            continue;
        }

        if let Some(group) = groups.iter_mut().find(|group| {
            group.scope == record.scope && group.name.text() == record.declared_name.text()
        }) {
            group.push(record);
        } else {
            groups.push(GlobalDeclarationGroupBuilder::new(record));
        }
    }

    GlobalManifest {
        groups: groups
            .into_iter()
            .map(GlobalDeclarationGroupBuilder::into_group)
            .collect(),
    }
}

/// Returns whether a record belongs to the top-level global scope.
fn is_global_scope(scope: &ScopePath) -> bool {
    matches!(scope, ScopePath::Global)
}

/// Classifies a declaration kind by its TypeScript global role.
fn role_for_kind(kind: &DeclarationKind) -> GlobalDeclarationRole {
    match kind {
        DeclarationKind::Interface | DeclarationKind::TypeAlias => GlobalDeclarationRole::Type,
        DeclarationKind::DeclareFunction
        | DeclarationKind::VariableDeclarator { .. }
        | DeclarationKind::ImportEquals => GlobalDeclarationRole::Value,
    }
}
