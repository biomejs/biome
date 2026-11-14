//! Queries that index the interfaces that modules merge into declarations
//! they don't own: module augmentations, `declare global` blocks, and the
//! top-level interfaces of declaration scripts.
//!
//! TypeScript applies these merges to the whole program, whether or not the
//! module that reads a declaration imports the module that contributes to
//! it. The index therefore covers every module that Biome indexes, which can
//! be more or fewer files than the program that a project's `tsconfig`
//! describes.
//!
//! A contribution is never copied into its target. The index maps each
//! target to handles of the contributing declarations, and the target's data
//! includes the handles (see [`InferredTypeData::with_contributions`]). The
//! queries here read only module graph inputs and never resolve a type, so
//! they can't join an inference cycle, and the resolution of any global can
//! read them.
//!
//! Each projection backdates unless its own entry changes, so a declaration
//! depends only on the contributions that it has.

use super::interned::{AugmentedExportInput, GlobalNameInput, GlobalTypeContributionsInput};
use crate::db::type_inference::{ExportOriginResult, find_export_origin, module_for_import};
use crate::type_inference::profiling::{
    TypeInferenceProfileOrigin, TypeInferenceQueryKind, execute_query,
};
use crate::{
    JsExport, JsOwnExport, ModuleDb, ModuleGraphGeneration, ModuleInfo, ModuleInfoKind,
    ResolutionMode, resolve_module_import,
};
use biome_js_type_info::{
    GlobalTypeId, ImportSymbol, RawTypeData, RawTypeId, TypeId, TypeReference,
    global_type_id_for_type_name,
    interned_types::{LocalTypeId, ModuleKey, TypeData as InferredTypeData},
};
use biome_rowan::Text;
use rustc_hash::FxHashMap;
use salsa::plumbing::AsId;

/// Most named exports and imports followed from an augmented name to the
/// declaration that it merges into.
const MAX_AUGMENTATION_TARGET_STEPS: usize = 16;

/// A contribution's declaration: the module that declares it and its type in
/// that module's table.
pub(crate) type ContributionHandle = (ModuleKey, LocalTypeId);

/// The contributions to the interface and class declarations of one module,
/// as [`declaration_contributions`] returns them: each declaration's type in
/// the module's table, sorted, with its contributions in lookup order.
pub(crate) type DeclarationContributions = Box<[(TypeId, Box<[ContributionHandle]>)]>;

// #region AUGMENTATION QUERIES

/// Returns the interfaces that `module` merges into declarations that it
/// doesn't own, with the modules that its augmentations name.
///
/// An augmentation whose specifier doesn't resolve to an indexed module
/// contributes nothing. Returns no entries when `module` isn't a JavaScript
/// module or has type inference disabled.
#[salsa::tracked(returns(ref))]
pub(crate) fn module_augmentations(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> ModuleAugmentationTargets {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::document(module),
        "module_augmentations",
        || module_augmentations_impl(db, module),
    )
}

/// Returns the declaration that an augmentation of the name `name` in
/// `module` merges into.
///
/// The search follows named and blanket re-exports, as an import of the name
/// does, and then a binding that imports the name from another module, as
/// TypeScript merges into the target of an alias. It reads only module graph
/// inputs.
///
/// For example, an augmentation of `X` in `./barrel` merges into the `X`
/// that `./inner` declares:
///
/// ```ts
/// // inner.ts
/// export interface X { x: 1 }
///
/// // barrel.ts
/// export * from "./inner";
/// ```
#[salsa::tracked(returns(copy))]
pub(crate) fn augmentation_target<'db>(
    db: &'db dyn ModuleDb,
    input: AugmentedExportInput<'db>,
) -> AugmentationTarget {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::document(input.module(db)),
        "augmentation_target",
        || augmentation_target_impl(db, input.module(db), input.name(db).clone()),
    )
}

/// Indexes the contributions of every indexed module by their targets.
///
/// Contributions are ordered by the path of the module that declares them,
/// then by source order, which is the order that member lookup searches
/// them in. The index re-executes when the set of modules changes or when
/// the contributions or targets of one module change; otherwise each revision
/// verifies one query per module.
#[salsa::tracked(returns(ref))]
pub(crate) fn augmentation_index(
    db: &dyn ModuleDb,
    generation: ModuleGraphGeneration,
) -> AugmentationIndex {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::Inherited,
        "augmentation_index",
        || augmentation_index_impl(db, generation),
    )
}

/// Returns the contributions to the interface and class declarations of
/// `module`, sorted by the type of each declaration in the module's table.
#[salsa::tracked(returns(ref))]
pub(crate) fn declaration_contributions(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> DeclarationContributions {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::document(module),
        "declaration_contributions",
        || {
            augmentation_index(db, ModuleGraphGeneration::get(db))
                .declarations
                .get(&module)
                .cloned()
                .unwrap_or_default()
        },
    )
}

/// Returns the contributions to a global that the standard library
/// declares.
#[salsa::tracked(returns(ref))]
pub(crate) fn global_type_contributions<'db>(
    db: &'db dyn ModuleDb,
    input: GlobalTypeContributionsInput<'db>,
) -> Box<[ContributionHandle]> {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::Inherited,
        "global_type_contributions",
        || {
            augmentation_index(db, ModuleGraphGeneration::get(db))
                .lib_globals
                .get(&input.id(db))
                .cloned()
                .unwrap_or_default()
        },
    )
}

/// Returns the declaration of a global that the standard library doesn't
/// declare and that one module declares, or `None` when no module or more
/// than one declares it.
#[salsa::tracked(returns(copy))]
pub(crate) fn global_name_declaration<'db>(
    db: &'db dyn ModuleDb,
    input: GlobalNameInput<'db>,
) -> Option<ContributionHandle> {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::Inherited,
        "global_name_declaration",
        || {
            augmentation_index(db, ModuleGraphGeneration::get(db))
                .globals
                .get(input.name(db))
                .copied()
                .flatten()
        },
    )
}

/// Returns the exports that only augmentations add to `module`, such as
/// `Fresh` in `declare module "./a" { interface Fresh {} }` when `./a`
/// exports no `Fresh`, with the declaration of each.
///
/// The first augmentation of a name, by module path and then source order,
/// is its declaration, and the others are its contributions.
#[salsa::tracked(returns(ref))]
pub(crate) fn module_new_exports(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Box<[(Text, ContributionHandle)]> {
    execute_query(
        TypeInferenceQueryKind::Exports,
        TypeInferenceProfileOrigin::document(module),
        "module_new_exports",
        || {
            augmentation_index(db, ModuleGraphGeneration::get(db))
                .new_exports
                .get(&module)
                .cloned()
                .unwrap_or_default()
        },
    )
}

// #endregion

// #region ENTRY POINTS

/// Returns the interface declarations that the indexed modules merge into
/// the global `id` of the standard library, through `declare global` blocks
/// and the top-level interfaces of declaration scripts, as
/// [`biome_js_type_info::TypeDb::global_contributions`] describes.
pub fn global_contributions(db: &dyn ModuleDb, id: GlobalTypeId) -> &[(ModuleKey, LocalTypeId)] {
    global_type_contributions(db, GlobalTypeContributionsInput::new(db, id))
}

// #endregion

// #region QUERY HELPER FUNCTIONS

/// The result of [`module_augmentations`].
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct ModuleAugmentationTargets {
    /// For each interface of the module's `declare module "specifier" {}`
    /// blocks, the module that the specifier names, the interface's name,
    /// and its type in this module's table, in source order.
    augmentations: Box<[(ModuleInfo, Text, TypeId)]>,
    /// The interfaces that the module declares as globals, with their types
    /// in this module's table, in source order.
    globals: Box<[(Text, TypeId)]>,
}

impl ModuleAugmentationTargets {
    fn is_empty(&self) -> bool {
        self.augmentations.is_empty() && self.globals.is_empty()
    }
}

/// The declaration that an augmentation merges into, as
/// [`augmentation_target`] finds it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AugmentationTarget {
    /// An interface or a class declared in `module`.
    Declaration { module: ModuleInfo, type_id: TypeId },
    /// The module exports nothing of the name, so the augmentation adds the
    /// export.
    NewExport,
    /// The export is ambiguous, the search ran out of work, or the export
    /// isn't an interface or a class, so the augmentation merges into
    /// nothing that Biome can see.
    None,
}

/// The result of [`augmentation_index`].
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct AugmentationIndex {
    /// The contributions to the interface and class declarations of each
    /// module, sorted by the declaration's type.
    declarations: FxHashMap<ModuleInfo, DeclarationContributions>,
    /// The declarations of the exports that only augmentations declare, by
    /// the module that the augmentations name.
    new_exports: FxHashMap<ModuleInfo, Box<[(Text, ContributionHandle)]>>,
    /// The contributions to the globals of the standard library.
    lib_globals: FxHashMap<GlobalTypeId, Box<[ContributionHandle]>>,
    /// The globals that only the project declares, with their declaration
    /// when exactly one module declares them.
    globals: FxHashMap<Text, Option<ContributionHandle>>,
}

fn module_augmentations_impl(db: &dyn ModuleDb, module: ModuleInfo) -> ModuleAugmentationTargets {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        return ModuleAugmentationTargets::default();
    };
    let contributions = &info.contributions;
    let augmentations = contributions
        .augmentations
        .iter()
        .filter_map(|augmentation| {
            module_for_import(db, module, &augmentation.import).map(|target| (target, augmentation))
        })
        .flat_map(|(target, augmentation)| {
            augmentation
                .interfaces
                .iter()
                .map(move |interface| (target, interface.name.clone(), interface.type_id))
        })
        .collect();
    let globals = contributions
        .globals
        .iter()
        .map(|interface| (interface.name.clone(), interface.type_id))
        .collect();
    ModuleAugmentationTargets {
        augmentations,
        globals,
    }
}

fn augmentation_target_impl(
    db: &dyn ModuleDb,
    mut module: ModuleInfo,
    mut name: Text,
) -> AugmentationTarget {
    for step in 0..MAX_AUGMENTATION_TARGET_STEPS {
        let (origin, origin_name) = match find_export_origin(db, module, name) {
            ExportOriginResult::Found { module, name } => (module, name),
            // Only the module that the augmentation names gains an export.
            ExportOriginResult::Missing if step == 0 => return AugmentationTarget::NewExport,
            ExportOriginResult::Missing
            | ExportOriginResult::Ambiguous
            | ExportOriginResult::Indeterminate => return AugmentationTarget::None,
        };
        let ModuleInfoKind::Js(info) = origin.kind(db) else {
            return AugmentationTarget::None;
        };
        let reference = match info.exports.get(origin_name.text()) {
            Some(JsExport::Own(own) | JsExport::OwnType(own)) => match own {
                JsOwnExport::Binding(range) => info.raw_binding_types.get(range).cloned(),
                JsOwnExport::Type(type_id) => Some(TypeReference::from(RawTypeId::Local(*type_id))),
                JsOwnExport::Namespace(_) => None,
            },
            Some(JsExport::Reexport(_) | JsExport::ReexportType(_)) | None => None,
        };
        match reference {
            Some(TypeReference::Resolved(RawTypeId::Local(type_id))) => {
                return declaration_type_id(&info.raw_types, type_id).map_or(
                    AugmentationTarget::None,
                    |type_id| AugmentationTarget::Declaration {
                        module: origin,
                        type_id,
                    },
                );
            }
            // A binding that imports the name merges into what it imports.
            Some(TypeReference::Import(import)) => {
                let ImportSymbol::Named(imported) = &import.symbol else {
                    return AugmentationTarget::None;
                };
                let resolved = resolve_module_import(
                    db,
                    origin,
                    &import.specifier,
                    ResolutionMode::JavaScript,
                );
                let Some(target) = resolved
                    .path()
                    .as_path()
                    .and_then(|path| db.module_for_path(path))
                else {
                    return AugmentationTarget::None;
                };
                module = target;
                name = imported.clone();
            }
            _ => return AugmentationTarget::None,
        }
    }
    AugmentationTarget::None
}

/// Returns the type of the interface or class that the raw type `type_id`
/// is, or refers to as the type meaning of an export, as for an export of a
/// class with the interfaces merged into it, or of an interface with a
/// namespace of its name.
fn declaration_type_id(raw_types: &[RawTypeData], type_id: TypeId) -> Option<TypeId> {
    let referenced = match raw_types.get(type_id.index())? {
        RawTypeData::Class(_) | RawTypeData::Interface(_) => return Some(type_id),
        RawTypeData::Reference(reference) => reference,
        RawTypeData::MergedReference(reference) => reference.ty.as_ref()?,
        _ => return None,
    };
    let TypeReference::Resolved(RawTypeId::Local(referenced)) = referenced else {
        return None;
    };
    matches!(
        raw_types.get(referenced.index())?,
        RawTypeData::Class(_) | RawTypeData::Interface(_)
    )
    .then_some(*referenced)
}

fn augmentation_index_impl(
    db: &dyn ModuleDb,
    generation: ModuleGraphGeneration,
) -> AugmentationIndex {
    let _ = generation.value(db);
    let mut contributing = Vec::new();
    db.for_each_module(&mut |module| {
        let targets = module_augmentations(db, module);
        if !targets.is_empty() {
            contributing.push((module, targets));
        }
    });
    // Modules come in no particular order.
    contributing.sort_unstable_by(|(left, _), (right, _)| left.path(db).cmp(right.path(db)));

    let mut declarations = FxHashMap::<ModuleInfo, FxHashMap<TypeId, Vec<_>>>::default();
    let mut new_exports = FxHashMap::<(ModuleInfo, Text), Vec<(ModuleInfo, TypeId)>>::default();
    let mut lib_globals = FxHashMap::<GlobalTypeId, Vec<_>>::default();
    let mut globals = FxHashMap::<Text, Vec<(ModuleInfo, TypeId)>>::default();
    for (module, targets) in contributing {
        for (target, name, type_id) in &targets.augmentations {
            let input = AugmentedExportInput::new(db, *target, name.clone());
            match augmentation_target(db, input) {
                AugmentationTarget::Declaration {
                    module: declaring,
                    type_id: declaration,
                } => declarations
                    .entry(declaring)
                    .or_default()
                    .entry(declaration)
                    .or_default()
                    .push(handle(module, *type_id)),
                AugmentationTarget::NewExport => new_exports
                    .entry((*target, name.clone()))
                    .or_default()
                    .push((module, *type_id)),
                AugmentationTarget::None => {}
            }
        }
        for (name, type_id) in &targets.globals {
            match global_type_id_for_type_name(name.text()) {
                Some(id) => lib_globals
                    .entry(id)
                    .or_default()
                    .push(handle(module, *type_id)),
                None => globals
                    .entry(name.clone())
                    .or_default()
                    .push((module, *type_id)),
            }
        }
    }

    let mut index = AugmentationIndex::default();
    let mut new_exports_by_module = FxHashMap::<ModuleInfo, Vec<_>>::default();
    for ((module, name), contributions) in new_exports {
        let ((declaring, declaration), others) = contributions
            .split_first()
            .map(|(first, others)| (*first, others))
            .expect("every new export has a contribution");
        new_exports_by_module
            .entry(module)
            .or_default()
            .push((name, handle(declaring, declaration)));
        if !others.is_empty() {
            declarations
                .entry(declaring)
                .or_default()
                .entry(declaration)
                .or_default()
                .extend(
                    others
                        .iter()
                        .map(|(module, type_id)| handle(*module, *type_id)),
                );
        }
    }
    index.new_exports = new_exports_by_module
        .into_iter()
        .map(|(module, mut exports)| {
            exports.sort_unstable_by(|(left, _), (right, _)| left.text().cmp(right.text()));
            (module, exports.into_boxed_slice())
        })
        .collect();
    index.declarations = declarations
        .into_iter()
        .map(|(module, contributions)| {
            let mut contributions = contributions
                .into_iter()
                .map(|(type_id, handles)| (type_id, handles.into_boxed_slice()))
                .collect::<Vec<_>>();
            contributions.sort_unstable_by_key(|(type_id, _)| type_id.index());
            (module, contributions.into_boxed_slice())
        })
        .collect();
    index.lib_globals = lib_globals
        .into_iter()
        .map(|(id, handles)| (id, handles.into_boxed_slice()))
        .collect();
    index.globals = globals
        .into_iter()
        .map(|(name, declarations)| {
            let declaration = match declarations.as_slice() {
                [(module, type_id)] => Some(handle(*module, *type_id)),
                _ => None,
            };
            (name, declaration)
        })
        .collect();
    index
}

fn handle(module: ModuleInfo, type_id: TypeId) -> ContributionHandle {
    (
        ModuleKey::new(module.as_id()),
        LocalTypeId::new(type_id.index()),
    )
}

/// Returns the declaration that `handle` names, as a local handle.
pub(crate) fn contribution_type<'db>(
    db: &'db dyn ModuleDb,
    (module, type_id): ContributionHandle,
) -> InferredTypeData<'db> {
    InferredTypeData::Local(biome_js_type_info::interned_types::LocalTypeHandle::new(
        db, module, type_id,
    ))
}

// #endregion
