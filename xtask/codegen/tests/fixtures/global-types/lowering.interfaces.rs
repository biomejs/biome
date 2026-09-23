/// Identities of generated globals, allocated after the hand-written manifest.
#[expect(dead_code, reason = "Rust code only names the globals it inspects")]
pub(crate) mod ids {
    use crate::globals::GlobalTypeId;
    use crate::{RawTypeId, TypeId};
    const FIRST_GENERATED_ID: usize = crate::globals_ids::PREDEFINED_ID_ROWS.len();
    pub(crate) const CATALOG_ID_GLOBAL_TYPE_ID: GlobalTypeId = GlobalTypeId::new(
        TypeId::new(FIRST_GENERATED_ID),
    );
    pub(crate) const GLOBAL_CATALOG_ID: RawTypeId = RawTypeId::Global(
        CATALOG_ID_GLOBAL_TYPE_ID,
    );
    pub(crate) const ITEM_ID_GLOBAL_TYPE_ID: GlobalTypeId = GlobalTypeId::new(
        TypeId::new(FIRST_GENERATED_ID + 1),
    );
    pub(crate) const GLOBAL_ITEM_ID: RawTypeId = RawTypeId::Global(
        ITEM_ID_GLOBAL_TYPE_ID,
    );
    pub(crate) const NAMED_ID_GLOBAL_TYPE_ID: GlobalTypeId = GlobalTypeId::new(
        TypeId::new(FIRST_GENERATED_ID + 2),
    );
    pub(crate) const GLOBAL_NAMED_ID: RawTypeId = RawTypeId::Global(
        NAMED_ID_GLOBAL_TYPE_ID,
    );
    pub(crate) const TAGGED_ID_GLOBAL_TYPE_ID: GlobalTypeId = GlobalTypeId::new(
        TypeId::new(FIRST_GENERATED_ID + 3),
    );
    pub(crate) const GLOBAL_TAGGED_ID: RawTypeId = RawTypeId::Global(
        TAGGED_ID_GLOBAL_TYPE_ID,
    );
    pub(crate) const UNSELECTED_ID_GLOBAL_TYPE_ID: GlobalTypeId = GlobalTypeId::new(
        TypeId::new(FIRST_GENERATED_ID + 4),
    );
    pub(crate) const GLOBAL_UNSELECTED_ID: RawTypeId = RawTypeId::Global(
        UNSELECTED_ID_GLOBAL_TYPE_ID,
    );
}
/// Names of generated globals in ID order.
pub(crate) const GENERATED_GLOBAL_NAMES: &[&str] = &[
    "Catalog",
    "Item",
    "Named",
    "Tagged",
    "Unselected",
];
/// Globals that type annotations can name, sorted by name.
pub(crate) const TYPE_GLOBALS: &[(&str, crate::globals::GlobalTypeId)] = &[
    ("Catalog", ids::CATALOG_ID_GLOBAL_TYPE_ID),
    ("Item", ids::ITEM_ID_GLOBAL_TYPE_ID),
    ("Named", ids::NAMED_ID_GLOBAL_TYPE_ID),
    ("Tagged", ids::TAGGED_ID_GLOBAL_TYPE_ID),
    ("Unselected", ids::UNSELECTED_ID_GLOBAL_TYPE_ID),
];
/// Globals that expressions can name, sorted by name.
pub(crate) const VALUE_GLOBALS: &[(&str, crate::globals::GlobalTypeId)] = &[];
/// Registers all generated global type data into the resolver builder.
pub(crate) fn set_generated_global_type_data(
    builder: &mut crate::globals_builder::GlobalsResolverBuilder,
) {
    builder.set_type_data(ids::CATALOG_ID_GLOBAL_TYPE_ID, global_catalog());
    builder.set_type_data(ids::ITEM_ID_GLOBAL_TYPE_ID, global_item());
    builder.set_type_data(ids::NAMED_ID_GLOBAL_TYPE_ID, global_named());
    builder.set_type_data(ids::TAGGED_ID_GLOBAL_TYPE_ID, global_tagged());
    builder.set_type_data(ids::UNSELECTED_ID_GLOBAL_TYPE_ID, global_unselected());
}
fn global_catalog() -> crate::TypeData {
    crate::TypeData::Interface(
        Box::new(crate::Interface {
            name: biome_rowan::Text::new_static("Catalog"),
            type_parameters: Box::default(),
            extends: Box::new([
                crate::RawTypeId::Local(crate::TypeId::new(7)).into(),
                crate::RawTypeId::Local(crate::TypeId::new(8)).into(),
            ]),
            members: Box::new([
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("item"),
                    ),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::NamedOptional(
                        biome_rowan::Text::new_static("selected"),
                    ),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("state"),
                    ),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("empty"),
                    ),
                    ty: crate::globals::GLOBAL_NULL_KEYWORD_ID.into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("label"),
                    ),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::NamedOptional(
                        biome_rowan::Text::new_static("find"),
                    ),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(5)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("update"),
                    ),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(6)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("count"),
                    ),
                    ty: crate::globals::GLOBAL_NUMBER_KEYWORD_ID.into(),
                },
            ]),
        }),
    )
}
fn global_item() -> crate::TypeData {
    crate::TypeData::Interface(
        Box::new(crate::Interface {
            name: biome_rowan::Text::new_static("Item"),
            type_parameters: Box::default(),
            extends: Box::default(),
            members: Box::new([
                crate::TypeMember {
                    kind: crate::TypeMemberKind::NamedOptional(
                        biome_rowan::Text::new_static("owner"),
                    ),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                },
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("title"),
                    ),
                    ty: crate::globals::GLOBAL_STRING_KEYWORD_ID.into(),
                },
            ]),
        }),
    )
}
fn global_named() -> crate::TypeData {
    crate::TypeData::Interface(
        Box::new(crate::Interface {
            name: biome_rowan::Text::new_static("Named"),
            type_parameters: Box::default(),
            extends: Box::default(),
            members: Box::new([
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("name"),
                    ),
                    ty: crate::globals::GLOBAL_STRING_KEYWORD_ID.into(),
                },
            ]),
        }),
    )
}
fn global_tagged() -> crate::TypeData {
    crate::TypeData::Interface(
        Box::new(crate::Interface {
            name: biome_rowan::Text::new_static("Tagged"),
            type_parameters: Box::default(),
            extends: Box::default(),
            members: Box::new([
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("tag"),
                    ),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                },
            ]),
        }),
    )
}
fn global_unselected() -> crate::TypeData {
    crate::TypeData::Interface(
        Box::new(crate::Interface {
            name: biome_rowan::Text::new_static("Unselected"),
            type_parameters: Box::default(),
            extends: Box::default(),
            members: Box::new([
                crate::TypeMember {
                    kind: crate::TypeMemberKind::Named(
                        biome_rowan::Text::new_static("name"),
                    ),
                    ty: crate::globals::GLOBAL_STRING_KEYWORD_ID.into(),
                },
            ]),
        }),
    )
}
static CATALOG_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 9]> = std::sync::LazyLock::new(||
[
    crate::TypeData::instance_of(crate::TypeInstance {
        ty: crate::globals::GLOBAL_ITEM_ID.into(),
        type_parameters: Box::default(),
    }),
    crate::TypeData::Union(
        Box::new(
            crate::Union(
                Box::new([
                    crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    crate::globals::GLOBAL_NULL_KEYWORD_ID.into(),
                ]),
            ),
        ),
    ),
    crate::TypeData::Literal(
        Box::new(crate::Literal::String(biome_rowan::Text::new_static("pending").into())),
    ),
    crate::TypeData::Union(
        Box::new(
            crate::Union(
                Box::new([
                    crate::globals::GLOBAL_BOOLEAN_KEYWORD_ID.into(),
                    crate::RawTypeId::Local(crate::TypeId::new(2)).into(),
                ]),
            ),
        ),
    ),
    crate::TypeData::Literal(
        Box::new(crate::Literal::String(biome_rowan::Text::new_static("ready").into())),
    ),
    crate::TypeData::Function(
        Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(biome_rowan::Text::new_static("find")),
            parameters: Box::new([
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("candidate"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("state"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(3)).into(),
                    is_optional: true,
                    is_rest: false,
                }),
            ]),
            return_type: crate::ReturnType::Type(
                crate::RawTypeId::Local(crate::TypeId::new(1)).into(),
            ),
        }),
    ),
    crate::TypeData::Function(
        Box::new(crate::Function {
            is_async: false,
            type_parameters: Box::default(),
            name: None,
            parameters: Box::new([
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("item"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(0)).into(),
                    is_optional: false,
                    is_rest: false,
                }),
                crate::FunctionParameter::Named(crate::NamedFunctionParameter {
                    name: biome_rowan::Text::new_static("label"),
                    ty: crate::RawTypeId::Local(crate::TypeId::new(4)).into(),
                    is_optional: true,
                    is_rest: false,
                }),
            ]),
            return_type: crate::ReturnType::Type(crate::globals::GLOBAL_VOID_ID.into()),
        }),
    ),
    crate::TypeData::instance_of(crate::TypeInstance {
        ty: crate::globals::GLOBAL_NAMED_ID.into(),
        type_parameters: Box::default(),
    }),
    crate::TypeData::instance_of(crate::TypeInstance {
        ty: crate::globals::GLOBAL_TAGGED_ID.into(),
        type_parameters: Box::default(),
    }),
]);
static ITEM_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> = std::sync::LazyLock::new(||
[
    crate::TypeData::instance_of(crate::TypeInstance {
        ty: crate::globals::GLOBAL_CATALOG_ID.into(),
        type_parameters: Box::default(),
    }),
]);
static TAGGED_LOCAL_TYPES: std::sync::LazyLock<[crate::TypeData; 1]> = std::sync::LazyLock::new(||
[
    crate::TypeData::Literal(
        Box::new(crate::Literal::String(biome_rowan::Text::new_static("tag").into())),
    ),
]);
/// Supporting types in dependency order, indexed relative to their owning global.
pub(crate) fn generated_local_types(
    owner: crate::globals::GlobalTypeId,
) -> &'static [crate::TypeData] {
    match owner {
        ids::CATALOG_ID_GLOBAL_TYPE_ID => &*CATALOG_LOCAL_TYPES,
        ids::ITEM_ID_GLOBAL_TYPE_ID => &*ITEM_LOCAL_TYPES,
        ids::TAGGED_ID_GLOBAL_TYPE_ID => &*TAGGED_LOCAL_TYPES,
        _ => &[],
    }
}
