#![cfg(feature = "global_types")]

use std::path::Path;

use xtask_codegen::generate_global_types::{
    collect::collect,
    compare::compare_lowered_globals,
    lower::{
        LoweredGlobalTypes, LoweredMemberKind, LoweredTypeData, LoweredTypeReference,
        lower_global_types,
    },
    manifest::build_global_manifest,
    source::{CanonicalPath, DiscoveredFile},
};

const SOURCE: &str = include_str!("fixtures/global-types/manifest.disposables.d.ts");

fn lower(source: &str) -> anyhow::Result<LoweredGlobalTypes> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file = DiscoveredFile {
        path: CanonicalPath::from_within(
            root,
            "tests/fixtures/global-types/manifest.disposables.d.ts",
        )?,
        repo_relative: "lib/lib.es5.d.ts".to_string(),
        bytes: source.as_bytes().to_vec(),
    };
    let manifest = build_global_manifest(collect(&file).records);
    lower_global_types(&manifest, &[file])
}

#[test]
fn array_from_mapping_signature() -> anyhow::Result<()> {
    let lowered = lower(SOURCE)?;
    compare_lowered_globals(&lowered)?;
    let LoweredTypeData::Class(array) = lowered.global("Array").unwrap().data() else {
        panic!("Array must be a class");
    };
    assert_eq!(
        array.member("from").unwrap().kind(),
        &LoweredMemberKind::NamedStatic
    );
    let LoweredTypeData::Interface(overloads) = lowered.global("Array.from").unwrap().data() else {
        panic!("Array.from must have overloads");
    };
    assert_eq!(overloads.members().len(), 2);
    assert!(
        overloads
            .members()
            .iter()
            .all(|member| member.kind() == &LoweredMemberKind::CallSignature)
    );
    let LoweredTypeData::Function(copy) = lowered.global("Array.from copy").unwrap().data() else {
        panic!("copy overload must be callable");
    };
    assert_eq!(
        copy.type_parameters(),
        &[LoweredTypeReference::Predefined("GLOBAL_T_ID")]
    );
    assert_eq!(copy.parameters().len(), 1);
    assert_eq!(
        copy.parameters()[0].type_reference(),
        &LoweredTypeReference::Predefined("GLOBAL_ARRAY_FROM_SOURCE_ID")
    );
    assert_eq!(
        copy.return_type(),
        &LoweredTypeReference::Predefined("GLOBAL_ARRAY_FROM_RESULT_ID")
    );
    let LoweredTypeData::Function(from) = lowered.global("Array.from mapped").unwrap().data()
    else {
        panic!("Array.from must be callable");
    };
    assert_eq!(
        from.type_parameters(),
        &[
            LoweredTypeReference::Predefined("GLOBAL_T_ID"),
            LoweredTypeReference::Predefined("GLOBAL_U_ID"),
        ]
    );
    assert_eq!(
        from.return_type(),
        &LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ARRAY_U_ID")
    );
    assert_eq!(from.parameters().len(), 3);
    assert_eq!(
        from.parameters()[1].type_reference(),
        &LoweredTypeReference::Predefined("GLOBAL_ARRAY_FROM_CALLBACK_ID")
    );
    assert!(!from.parameters()[1].is_optional());
    assert!(from.parameters()[2].is_optional());
    let LoweredTypeData::Function(callback) = lowered.global("Array.from callback").unwrap().data()
    else {
        panic!("mapper must be callable");
    };
    assert!(callback.type_parameters().is_empty());
    assert_eq!(
        callback.parameters()[0].type_reference(),
        &LoweredTypeReference::Predefined("GLOBAL_T_ID")
    );
    assert_eq!(
        callback.parameters()[1].type_reference(),
        &LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID")
    );
    assert_eq!(
        callback.return_type(),
        &LoweredTypeReference::Predefined("GLOBAL_U_ID")
    );
    Ok(())
}

#[test]
fn array_from_rejects_incompatible_mapping_declarations() {
    for (before, after) in [
        (
            "from<T>(items: ArrayLike<T>): T[]",
            "from<T>(items: ArrayLike<T>): string[]",
        ),
        ("from<T>(items:", "from<T>(items?:"),
        ("ArrayLike<T>", "ArrayLike<string>"),
        ("ArrayLike<T>", "Unrelated<T>"),
        ("mapfn:", "mapfn?:"),
        (
            "(value: T, index: number) => U",
            "(value: T, index: string) => U",
        ),
        (
            "(value: T, index: number) => U",
            "(value: T, index: number) => T",
        ),
        ("from<T, U>", "from<T, U extends string>"),
        ("from<T, U>", "other<T, U>"),
    ] {
        assert!(
            lower(&SOURCE.replace(before, after)).is_err(),
            "accepted {after}"
        );
    }
}

#[test]
fn array_from_accepts_iterable_overloads() -> anyhow::Result<()> {
    lower(&SOURCE.replace("items: ArrayLike<T>", "items: Iterable<T> | ArrayLike<T>"))?;
    Ok(())
}
