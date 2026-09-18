use super::Settings;
use crate::projects::{NestedPath, ProjectDb, ProjectInput};
use camino::Utf8Path;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Owns a settings value and compares it by `Arc` allocation identity.
///
/// Unlike `Arc`'s trait implementations, this wrapper does not compare or hash
/// the `Settings` contents. Retaining the `Arc` also prevents its address from
/// being reused while the identity is live.
#[derive(Clone, Debug)]
pub struct SettingsIdentity(Arc<Settings>);

impl SettingsIdentity {
    pub(crate) fn clone_arc(&self) -> Arc<Settings> {
        self.0.clone()
    }

    pub(crate) fn make_mut(&mut self) -> &mut Settings {
        Arc::make_mut(&mut self.0)
    }
}

impl AsRef<Settings> for SettingsIdentity {
    fn as_ref(&self) -> &Settings {
        self.0.as_ref()
    }
}

impl From<Arc<Settings>> for SettingsIdentity {
    fn from(settings: Arc<Settings>) -> Self {
        Self(settings)
    }
}

impl PartialEq for SettingsIdentity {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for SettingsIdentity {}

impl Hash for SettingsIdentity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Arc::as_ptr(&self.0), state);
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SettingsSelectionKey {
    Root,
    Nested(NestedPath),
}

/// Supplies tracked queries with project settings and path-matched overrides.
///
/// Override indices preserve declaration order and are captured for the request
/// path when the selection is constructed.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct SettingsQuerySelection {
    selection: SettingsSelectionKey,
    override_indices: Box<[usize]>,
}

impl SettingsQuerySelection {
    #[cfg(test)]
    pub(crate) fn key(&self) -> SettingsSelectionKey {
        self.selection.clone()
    }

    pub(crate) fn selected_settings(
        &self,
        db: &dyn ProjectDb,
        project: ProjectInput,
    ) -> SettingsIdentity {
        match &self.selection {
            SettingsSelectionKey::Root => project.root_settings(db),
            SettingsSelectionKey::Nested(path) => project
                .nested_settings(db)
                .get(path)
                .cloned()
                .unwrap_or_else(|| project.root_settings(db)),
        }
    }

    pub(crate) fn override_indices(&self) -> &[usize] {
        &self.override_indices
    }
}

#[derive(Clone, Debug)]
struct InlineSettingsQuery {
    settings: SettingsIdentity,
    override_indices: Box<[usize]>,
}

#[derive(Clone)]
pub(crate) struct SettingsQuery {
    project: ProjectInput,
    selection: SettingsQuerySelection,
    inline: Option<InlineSettingsQuery>,
}

impl std::fmt::Debug for SettingsQuery {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SettingsQuery")
            .field("selection", &self.selection)
            .field("has_inline_settings", &self.inline.is_some())
            .finish_non_exhaustive()
    }
}

impl SettingsQuery {
    pub(crate) fn new(
        project: ProjectInput,
        project_selection: SettingsSelectionKey,
        project_settings: &Settings,
        inline_settings: Option<Settings>,
        file_path: &Utf8Path,
    ) -> Self {
        let selection = SettingsQuerySelection {
            selection: project_selection,
            override_indices: project_settings.matching_override_indices(file_path),
        };
        let inline = inline_settings.map(|settings| InlineSettingsQuery {
            override_indices: settings.matching_override_indices(file_path),
            settings: SettingsIdentity::from(Arc::new(settings)),
        });
        Self {
            project,
            selection,
            inline,
        }
    }

    pub(crate) fn project(&self) -> ProjectInput {
        self.project
    }

    pub(crate) fn selection(&self) -> &SettingsQuerySelection {
        &self.selection
    }

    pub(crate) fn inline_settings(&self) -> Option<&SettingsIdentity> {
        self.inline.as_ref().map(|inline| &inline.settings)
    }

    pub(crate) fn override_indices(&self) -> &[usize] {
        self.inline.as_ref().map_or_else(
            || self.selection.override_indices(),
            |inline| &inline.override_indices,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    fn hash(value: &SettingsIdentity) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn settings_identity_uses_arc_allocation_identity() {
        let shared = SettingsIdentity::from(Arc::new(Settings::default()));
        let clone = shared.clone();
        let distinct = SettingsIdentity::from(Arc::new(Settings::default()));

        assert_eq!(shared, clone);
        assert_eq!(hash(&shared), hash(&clone));
        assert_ne!(shared, distinct);
    }
}
