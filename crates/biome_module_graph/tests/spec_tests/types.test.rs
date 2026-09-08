use super::*;

#[test]
fn test_named_type_ids_are_sorted_and_deduplicated() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            class ClassType {}
            enum EnumType { Value }
            interface InterfaceType { first: string }
            interface InterfaceType { second: number }
            module ModuleType {}
            namespace NamespaceType {}
            type AliasType = string;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert!(
        inferred
            .named_type_ids
            .windows(2)
            .all(|ids| ids[0] < ids[1]),
        "named type IDs must be sorted and deduplicated"
    );

    let ModuleInfoKind::Js(info) = module.kind(&db) else {
        panic!("module must contain JavaScript information");
    };
    let mut names = inferred
        .named_type_ids
        .iter()
        .map(|id| {
            info.local_type_name(*id)
                .expect("named type must have a name")
                .to_string()
        })
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(
        names,
        [
            "ClassType",
            "InterfaceType",
            "InterfaceType",
            "ModuleType",
            "NamespaceType",
        ]
    );
}
#[test]
fn test_infer_module_types_poison_unresolved_union_variants() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Failed = "known" | MissingType;
            type Explicit = "known" | unknown;

            export function failed(value: Failed): Failed { return value; }
            export function explicit(value: Explicit): Explicit { return value; }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let failed = inferred_function_return_ty_by_name(&db, module, inferred, "failed")
        .expect("failed return type must be inferred");
    let failed = normalize_type(&db, module, failed);
    assert_eq!(failed, InferredTypeData::Unknown);
    assert!(!InferredType::new(&db, failed).is_inferred());

    let explicit = inferred_function_return_ty_by_name(&db, module, inferred, "explicit")
        .expect("explicit return type must be inferred");
    let explicit = normalize_type(&db, module, explicit);
    assert_ne!(explicit, InferredTypeData::Unknown);
    assert!(InferredType::new(&db, explicit).is_inferred());
}
#[test]
fn test_infer_module_types_resolves_shorthand_value_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            const job = () => Promise.resolve("done");
            const api = { job };
            export const result = api.job();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");
    let result = normalize_type(&db, module, result);
    assert!(is_inferred_promise_instance(&db, result));
}
#[test]
fn test_infer_module_types_resolves_local_multisegment_qualifiers() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            namespace Outer {
                export namespace Inner {
                    export interface Value { field: string; }
                }
            }

            declare const value: Outer.Inner.Value;
            export const result = value.field;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");
    assert!(is_inferred_string(&db, normalize_type(&db, module, result)));
}

#[test]
fn test_infer_module_types_keeps_sibling_namespace_members_isolated() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            namespace Alpha {
                export const alpha = "alpha";
            }

            module Beta {
                export const beta = 1;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let alpha = inferred_binding_ty_by_name(&db, module, inferred, "Alpha")
        .expect("Alpha type must be inferred");
    assert!(inferred.find_member_type(&db, alpha, "alpha").is_some());
    assert!(inferred.find_member_type(&db, alpha, "beta").is_none());

    let beta = inferred_binding_ty_by_name(&db, module, inferred, "Beta")
        .expect("Beta type must be inferred");
    assert!(inferred.find_member_type(&db, beta, "beta").is_some());
    assert!(inferred.find_member_type(&db, beta, "alpha").is_none());
}

#[test]
fn test_infer_module_types_aggregates_merged_namespace_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            namespace Merged {
                export const first = "first";
            }

            namespace Merged {
                export const second = 2;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let merged = inferred_binding_ty_by_name(&db, module, inferred, "Merged")
        .expect("Merged type must be inferred");

    assert!(inferred.find_member_type(&db, merged, "first").is_some());
    assert!(inferred.find_member_type(&db, merged, "second").is_some());
}

#[test]
fn test_infer_module_types_resolves_record_index_signature_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readRecord(value: Record<string, number>): Record<string, number> {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let record_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readRecord")
        .expect("readRecord return type must be inferred");
    let InferredTypeData::InstanceOf(record_instance) = record_ty else {
        panic!("readRecord must return an object instance, got {record_ty:?}");
    };
    let InferredTypeData::Object(object) = record_instance.ty(&db) else {
        panic!("readRecord must return an object instance, got {record_ty:?}");
    };
    assert_eq!(object.members(&db).len(), 1);

    let item_ty = inferred
        .find_member_type(&db, record_ty, "item")
        .expect("Record<string, number> must expose a number string index signature");
    assert!(is_inferred_number(&db, item_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_record_index_signature_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_resolves_utility_type_members_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Source = {
                [key: string]: string | number | undefined;
                name: string;
                value: number;
                optional?: string;
            };

            export function readPick(
                value: Pick<Source, "name" | "value">,
            ): Pick<Source, "name" | "value"> {
                return value;
            }

            export function readOmit(
                value: Omit<Source, "value">,
            ): Omit<Source, "value"> {
                return value;
            }

            export function readPartial(value: Partial<Source>): Partial<Source> {
                return value;
            }

            export function readRequired(value: Required<Source>): Required<Source> {
                return value;
            }

            export function readReadonly(value: Readonly<Source>): Readonly<Source> {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let pick_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readPick")
        .expect("readPick return type must be inferred");
    let (_, pick_name_ty) =
        object_member_ty_by_name(&db, pick_ty, "name").expect("Pick<Source, ...> must keep name");
    let (_, pick_value_ty) =
        object_member_ty_by_name(&db, pick_ty, "value").expect("Pick<Source, ...> must keep value");
    assert!(is_inferred_string(&db, pick_name_ty));
    assert!(is_inferred_number(&db, pick_value_ty));
    assert!(object_member_ty_by_name(&db, pick_ty, "optional").is_none());
    assert!(
        inferred
            .find_member_type(&db, pick_ty, "anything")
            .is_none()
    );

    let omit_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readOmit")
        .expect("readOmit return type must be inferred");
    let (_, omit_name_ty) =
        object_member_ty_by_name(&db, omit_ty, "name").expect("Omit<Source, ...> must keep name");
    assert!(is_inferred_string(&db, omit_name_ty));
    assert!(object_member_ty_by_name(&db, omit_ty, "value").is_none());
    let omit_index_ty = inferred
        .find_member_type(&db, omit_ty, "anything")
        .expect("Omit<Source, ...> must preserve the string index signature");
    assert!(contains_inferred_string(&db, omit_index_ty));
    assert!(contains_inferred_number(&db, omit_index_ty));

    let partial_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readPartial")
            .expect("readPartial return type must be inferred");
    let (partial_name_kind, partial_name_ty) =
        object_member_ty_by_name(&db, partial_ty, "name").expect("Partial<Source> must keep name");
    assert!(partial_name_kind.is_optional());
    assert!(contains_inferred_string(&db, partial_name_ty));
    assert!(contains_inferred_undefined(&db, partial_name_ty));

    let required_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readRequired")
            .expect("readRequired return type must be inferred");
    let (required_optional_kind, required_optional_ty) =
        object_member_ty_by_name(&db, required_ty, "optional")
            .expect("Required<Source> must keep optional");
    assert!(!required_optional_kind.is_optional());
    assert!(is_inferred_string(&db, required_optional_ty));
    assert!(!contains_inferred_undefined(&db, required_optional_ty));

    let readonly_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readReadonly")
            .expect("readReadonly return type must be inferred");
    let (_, readonly_name_ty) = object_member_ty_by_name(&db, readonly_ty, "name")
        .expect("Readonly<Source> must keep name");
    assert!(is_inferred_string(&db, readonly_name_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_utility_type_members_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_preserves_legacy_rhs_never_intersection_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Named = {
                name: string;
            };

            export function readRightNever(value: Named & never): Named & never {
                return value;
            }

            export function readLeftNever(value: never & Named): never & Named {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let right_never_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readRightNever")
            .expect("readRightNever return type must be inferred");
    let right_name_ty = inferred
        .find_member_type(&db, right_never_ty, "name")
        .expect("Named & never must preserve Named members for legacy parity");
    assert!(is_inferred_string(&db, right_name_ty));

    let left_never_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readLeftNever")
            .expect("readLeftNever return type must be inferred");
    assert!(
        inferred
            .find_member_type(&db, left_never_ty, "name")
            .is_none()
    );
}

#[test]
fn test_infer_module_types_normalizes_nested_compounds_with_cycle_detector() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Named = {
                name: string;
            };
            type Valued = {
                value: number;
            };
            type NestedUnion = string | (number | ("literal" | string));
            type NestedIntersection = Named & (Valued & never);

            export function readUnion(value: NestedUnion): NestedUnion {
                return value;
            }

            export function readIntersection(
                value: NestedIntersection,
            ): NestedIntersection {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let union_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readUnion")
        .map(|ty| normalize_type(&db, index_module, ty))
        .expect("readUnion return type must be inferred");
    assert!(contains_inferred_string(&db, union_ty));
    assert!(contains_inferred_number(&db, union_ty));

    let intersection_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readIntersection")
            .expect("readIntersection return type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, intersection_ty, "name")
        .expect("nested intersection must preserve Named.name");
    assert!(is_inferred_string(&db, name_ty));
    let value_ty = inferred
        .find_member_type(&db, intersection_ty, "value")
        .expect("nested intersection must preserve Valued.value");
    assert!(is_inferred_number(&db, value_ty));
}

#[test]
fn test_infer_module_types_preserves_generic_interface_instantiation_parameters() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export interface ImportedBox<T> {
                value: T;
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { ImportedBox } from "./types.ts";

            interface LocalBox<T> {
                value: T;
            }

            export function readLocalBox(value: LocalBox<number>): LocalBox<number> {
                return value;
            }

            export function readImportedBox(value: ImportedBox<string>): ImportedBox<string> {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let local_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readLocalBox")
            .expect("readLocalBox return type must be inferred");
    let InferredTypeData::InstanceOf(local_box_instance) = local_box_ty else {
        panic!("readLocalBox must return an instance type, got {local_box_ty:?}");
    };
    assert_eq!(local_box_instance.type_parameters(&db).len(), 1);
    assert!(is_inferred_number(
        &db,
        local_box_instance.type_parameters(&db)[0]
    ));
    let InferredTypeData::Local(local_box_target) = local_box_instance.ty(&db) else {
        panic!("readLocalBox must target a local handle");
    };
    assert_eq!(local_box_target.module(&db), inferred.module_key);
    let local_value_ty = inferred
        .find_member_type(&db, local_box_ty, "value")
        .expect("LocalBox<number>.value must be inferred");
    assert!(is_inferred_number(&db, local_value_ty));

    let imported_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readImportedBox")
            .expect("readImportedBox return type must be inferred");
    let InferredTypeData::InstanceOf(imported_box_instance) = imported_box_ty else {
        panic!("readImportedBox must return an instance type, got {imported_box_ty:?}");
    };
    assert_eq!(imported_box_instance.type_parameters(&db).len(), 1);
    assert!(is_inferred_string(
        &db,
        imported_box_instance.type_parameters(&db)[0]
    ));
    let InferredTypeData::Local(imported_box_target) = imported_box_instance.ty(&db) else {
        panic!("readImportedBox must target a local handle");
    };
    assert_ne!(imported_box_target.module(&db), inferred.module_key);
    let imported_value_ty = inferred
        .find_member_type(&db, imported_box_ty, "value")
        .expect("ImportedBox<string>.value must be inferred");
    assert!(is_inferred_string(&db, imported_value_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_preserves_generic_interface_instantiation_parameters",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_resolves_nested_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export type Foo = string;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Foo } from "./types.ts";

            export function value(): Foo {
                return "value";
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    assert!(inferred.types.iter().any(|ty| {
        matches!(
            *ty,
            InferredTypeData::Function(function)
                if matches!(
                    function.return_type(&db),
                    InferredReturnType::Type(return_ty)
                        if is_inferred_instance_of(&db, *return_ty, InferredTypeData::String),
                )
        )
    }));
}

#[test]
fn test_infer_module_types_resolves_nested_object_member_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export type Foo = string;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Foo } from "./types.ts";

            export type Boxed = {
                value: Foo;
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    assert!(inferred.types.iter().any(|ty| {
        matches!(
            *ty,
            InferredTypeData::Object(object)
                if object.members(&db).iter().any(|member| {
                    matches!(
                        &member.kind,
                        InferredTypeMemberKind::Named(name) if name.text() == "value",
                    ) && is_inferred_instance_of(&db, member.ty, InferredTypeData::String)
                })
        )
    }));
}

#[test]
fn test_infer_module_types_resolves_nested_union_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export type Foo = string;
            export type Bar = number;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Foo, Bar } from "./types.ts";

            export type Value = Foo | Bar;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    assert!(inferred.types.iter().any(|ty| {
        matches!(
            *ty,
            InferredTypeData::Union(union)
                if union
                    .types(&db)
                    .iter()
                    .any(|ty| is_inferred_instance_of(&db, *ty, InferredTypeData::String))
                    && union
                        .types(&db)
                        .iter()
                        .any(|ty| is_inferred_instance_of(&db, *ty, InferredTypeData::Number))
        )
    }));
}

#[test]
fn test_infer_module_types_uses_local_handle_for_recursive_class_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Foo {
                name: string;

                static create(): Foo {
                    return new Foo();
                }
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let (class_index, class) = inferred
        .types
        .iter()
        .enumerate()
        .find_map(|(index, ty)| match ty {
            InferredTypeData::Class(class)
                if class
                    .name(&db)
                    .as_ref()
                    .is_some_and(|name| name.text() == "Foo") =>
            {
                Some((index, class))
            }
            _ => None,
        })
        .expect("Foo class type must be inferred");

    let return_ty = class
        .members(&db)
        .iter()
        .find_map(|member| {
            if !matches!(
                &member.kind,
                InferredTypeMemberKind::NamedStatic(name) if name.text() == "create",
            ) {
                return None;
            }

            let InferredTypeData::Function(function) = member.ty else {
                return None;
            };
            match function.return_type(&db) {
                InferredReturnType::Type(return_ty) => Some(*return_ty),
                InferredReturnType::Predicate(_) | InferredReturnType::Asserts(_) => None,
            }
        })
        .expect("Foo.create return type must be inferred");

    let InferredTypeData::InstanceOf(instance) = return_ty else {
        panic!("Foo.create must return an instance type");
    };
    let InferredTypeData::Local(local) = instance.ty(&db) else {
        panic!("Foo.create must return a local handle to Foo");
    };

    assert_eq!(local.module(&db), inferred.module_key);
    assert_eq!(local.type_id(&db).index(), class_index);
    assert!(!InferredType::new(&db, InferredTypeData::Local(local)).is_inferred());

    let name_ty = inferred
        .find_member_type(&db, return_ty, "name")
        .expect("Foo.create().name must be inferred");
    assert!(is_inferred_string(&db, name_ty));

    let create_ty = inferred
        .find_member_type(&db, InferredTypeData::Local(local), "create")
        .expect("Foo.create must be found through the local handle");
    assert!(matches!(create_ty, InferredTypeData::Function(_)));
}

#[test]
fn test_infer_module_types_uses_local_handles_for_recursive_interfaces() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface A {
                b: B;
            }

            export interface B {
                a: A;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let (a_index, a_interface) = inferred
        .types
        .iter()
        .enumerate()
        .find_map(|(index, ty)| match ty {
            InferredTypeData::Interface(interface) if interface.name(&db).text() == "A" => {
                Some((index, *interface))
            }
            _ => None,
        })
        .expect("A interface type must be inferred");
    let (b_index, b_interface) = inferred
        .types
        .iter()
        .enumerate()
        .find_map(|(index, ty)| match ty {
            InferredTypeData::Interface(interface) if interface.name(&db).text() == "B" => {
                Some((index, *interface))
            }
            _ => None,
        })
        .expect("B interface type must be inferred");

    let a_b_ty = interface_member_ty(&db, a_interface, "b").expect("A.b must be inferred");
    let b_a_ty = interface_member_ty(&db, b_interface, "a").expect("B.a must be inferred");

    assert_eq!(local_type_id_of_instance(&db, a_b_ty), Some(b_index));
    assert_eq!(local_type_id_of_instance(&db, b_a_ty), Some(a_index));

    let b_from_a = inferred
        .find_member_type(&db, b_a_ty, "b")
        .expect("B.a.b must be inferred through A's local handle");
    let a_from_b = inferred
        .find_member_type(&db, a_b_ty, "a")
        .expect("A.b.a must be inferred through B's local handle");

    assert_eq!(local_type_id_of_instance(&db, b_from_a), Some(b_index));
    assert_eq!(local_type_id_of_instance(&db, a_from_b), Some(a_index));
}

#[test]
fn test_infer_module_types_resolves_inherited_class_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Base {
                name: string;

                static label(): string {
                    return "base";
                }
            }

            export class Derived extends Base {
                value: number;
            }

            export const derived: Derived = new Derived();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let derived_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "derived")
        .expect("derived binding type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived instance must inherit Base.name");
    assert!(is_inferred_string(&db, name_ty));

    let derived_class_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "Derived")
        .expect("Derived class type must be inferred");
    let label_ty = inferred
        .find_member_type(&db, derived_class_ty, "label")
        .expect("Derived class must inherit Base.label");
    assert!(matches!(label_ty, InferredTypeData::Function(_)));
}

#[test]
fn test_infer_module_types_resolves_inherited_interface_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Base {
                name: string;
            }

            export interface Derived extends Base {
                value: number;
            }

            export const derived: Derived = {
                name: "derived",
                value: 1,
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let derived_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "derived")
        .expect("derived binding type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived interface must inherit Base.name");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_intersection_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type WithName = {
                name: string;
            };

            type WithValue = {
                value: number;
            };

            export const combined: WithName & WithValue = {
                name: "combined",
                value: 1,
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let combined_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "combined")
        .expect("combined binding type must be inferred");

    let name_ty = inferred
        .find_member_type(&db, combined_ty, "name")
        .expect("intersection must expose WithName.name");
    assert!(is_inferred_string(&db, name_ty));

    let value_ty = inferred
        .find_member_type(&db, combined_ty, "value")
        .expect("intersection must expose WithValue.value");
    assert!(is_inferred_number(&db, value_ty));
}

#[test]
fn test_infer_module_types_resolves_union_member_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type StringValue = {
                value: string;
            };

            type NumberValue = {
                value: number;
            };

            export const item: StringValue | NumberValue = {
                value: "item",
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let item_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "item")
        .expect("item binding type must be inferred");
    let value_ty = inferred
        .find_member_type(&db, item_ty, "value")
        .expect("union must expose shared value member");

    assert!(matches!(
        value_ty,
        InferredTypeData::Union(union)
            if union
                .types(&db)
                .iter()
                .any(|ty| is_inferred_string(&db, *ty))
                && union
                    .types(&db)
                    .iter()
                    .any(|ty| is_inferred_number(&db, *ty))
    ));
}

#[test]
fn test_infer_module_types_resolves_generic_constraint_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readName<T extends { name: string }>(value: T): T {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_name_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readName")
        .expect("readName binding type must be inferred");
    let InferredTypeData::Function(read_name) = inferred.resolve_type(&db, read_name_ty) else {
        panic!("readName must be inferred as a function");
    };
    let value_ty = read_name
        .parameters(&db)
        .iter()
        .find_map(|parameter| match parameter {
            InferredFunctionParameter::Named(parameter) if parameter.name.text() == "value" => {
                Some(parameter.ty)
            }
            _ => None,
        })
        .expect("value parameter type must be inferred");

    let name_ty = inferred
        .find_member_type(&db, value_ty, "name")
        .expect("generic constraint must expose name");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_string_index_signature_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Dictionary = {
                [key: string]: number;
            };

            export const dictionary: Dictionary = {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let dictionary_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "dictionary")
        .expect("dictionary binding type must be inferred");

    let value_ty = inferred
        .find_member_type(&db, dictionary_ty, "anything")
        .expect("string index signature must expose arbitrary string members");
    assert!(is_inferred_number(&db, value_ty));
}

#[test]
fn test_infer_module_types_resolves_computed_string_literal_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const object = {
                ["name"]: "object",
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let object_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "object")
        .expect("object binding type must be inferred");

    let name_ty = inferred
        .find_member_type(&db, object_ty, "name")
        .expect("computed string literal member must match its literal name");
    assert!(is_inferred_string(&db, name_ty));

    assert!(inferred.find_member_type(&db, object_ty, "other").is_none());
}

#[test]
fn test_infer_module_types_resolves_merged_reference_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Foo {
                typeName: string;
            }

            const Foo = {
                valueName: 1,
            };

            export { Foo };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let ModuleInfoKind::Js(js_info) = index_module.kind(&db) else {
        panic!("module must be JavaScript");
    };
    let foo_ty = match js_info.exports.get("Foo").and_then(JsExport::as_own_export) {
        Some(JsOwnExport::Type(resolved_id)) => inferred
            .types
            .get(resolved_id.index())
            .copied()
            .expect("Foo export type must be inferred"),
        _ => panic!("Foo export must have a type"),
    };
    let type_name_ty = inferred
        .find_member_type(&db, foo_ty, "typeName")
        .expect("merged type side must expose Foo.typeName");
    assert!(is_inferred_string(&db, type_name_ty));

    let value_name_ty = inferred
        .find_member_type(&db, foo_ty, "valueName")
        .expect("merged value side must expose Foo.valueName");
    assert!(is_inferred_number(&db, value_name_ty));
}

#[test]
fn test_normalize_terminal_type_does_not_depend_on_module_inference() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export const value = 1;");

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    db.clear_salsa_events();
    let input = NormalizeTypeInput::new(&db, module, InferredTypeData::String);
    assert_eq!(normalize_type_query(&db, input), InferredTypeData::String);
    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);

    let module_kind = module.kind(&db).clone();
    salsa::Setter::to(module.set_kind(&mut db), module_kind);

    db.clear_salsa_events();
    let input = NormalizeTypeInput::new(&db, module, InferredTypeData::String);
    assert_eq!(normalize_type_query(&db, input), InferredTypeData::String);
    let events = db.take_salsa_events();
    let input = NormalizeTypeInput::new(&db, module, InferredTypeData::String);
    assert_function_query_was_not_run(&db, normalize_type_query, input, &events);
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn test_normalize_type_collapses_equal_merged_reference_targets() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Foo = {
                name: "foo";
            };

            const Foo = {
                name: "foo",
            } as const;

            export { Foo };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let ModuleInfoKind::Js(js_info) = index_module.kind(&db) else {
        panic!("module must be JavaScript");
    };
    let foo_ty = match js_info.exports.get("Foo").and_then(JsExport::as_own_export) {
        Some(JsOwnExport::Type(resolved_id)) => inferred
            .types
            .get(resolved_id.index())
            .copied()
            .expect("Foo export type must be inferred"),
        _ => panic!("Foo export must have a type"),
    };

    assert!(
        matches!(foo_ty, InferredTypeData::MergedReference(_)),
        "Foo export must be a merged reference, got {foo_ty:?}"
    );

    let InferredTypeData::MergedReference(reference) = foo_ty else {
        panic!("Foo export must be a merged reference, got {foo_ty:?}");
    };
    let target = reference
        .targets(&db)
        .next()
        .expect("merged reference must have a target");
    let expected_ty = normalize_type(&db, index_module, target);
    let duplicated_target_ty = InferredTypeData::MergedReference(InferredMergedReference::new(
        &db,
        Some(target),
        Some(target),
        None,
    ));
    let normalized_ty = normalize_type(&db, index_module, duplicated_target_ty);
    assert_eq!(normalized_ty, expected_ty);

    let name_ty = inferred
        .find_member_type(&db, normalized_ty, "name")
        .expect("normalized merged reference must expose Foo.name");
    assert!(is_inferred_string(&db, name_ty));

    assert_inferred_type_snapshot(
        "test_normalize_type_collapses_equal_merged_reference_targets",
        &db,
        &fs,
    );
}

#[test]
fn test_normalize_type_resolves_typeof_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            const source = {
                name: "source",
            } as const;

            type Source = typeof source;

            export function readSource(value: Source): Source {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let source_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readSource")
        .expect("readSource return type must be inferred");
    let normalized_ty = normalize_type(&db, index_module, source_ty);
    let InferredTypeData::Object(_) = normalized_ty else {
        panic!("typeof type must normalize to its target object, got {normalized_ty:?}");
    };

    let name_ty = inferred
        .find_member_type(&db, normalized_ty, "name")
        .expect("normalized typeof type must expose source.name");
    assert!(is_inferred_string(&db, name_ty));

    assert_inferred_type_snapshot("test_normalize_type_resolves_typeof_type", &db, &fs);
}

#[test]
fn test_normalize_type_preserves_recursive_local_edge() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Tree = number | Promise<Tree>;

            export function readTree(value: Tree): Tree {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let tree_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readTree")
        .expect("readTree return type must be inferred");
    let tree_index = local_type_id_of_instance(&db, tree_ty)
        .expect("readTree must return an instance of the local Tree type");
    let normalized_ty = normalize_type(&db, index_module, tree_ty);

    let InferredTypeData::Union(union) = normalized_ty else {
        panic!("recursive Tree type must normalize to a union, got {normalized_ty:?}");
    };
    let normalized_tree = format_inferred_type(&db, normalized_ty);
    assert!(
        normalized_tree.contains("Tree"),
        "recursive local edge must format with its source type name: {normalized_tree}"
    );
    assert!(
        !normalized_tree.contains("local type"),
        "recursive local edge must not expose raw local type IDs: {normalized_tree}"
    );
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty)),
        "recursive Tree union must keep its number branch"
    );
    assert!(
        union.types(&db).iter().any(|ty| {
            matches!(
                ty,
                InferredTypeData::InstanceOf(instance)
                    if instance.type_parameters(&db).iter().any(|parameter| {
                        matches!(
                            parameter,
                            InferredTypeData::Local(local)
                                if local.type_id(&db).index() == tree_index
                        )
                        || local_type_id_of_instance(&db, *parameter) == Some(tree_index)
                    })
            )
        }),
        "recursive Tree union must keep the recursive local edge: {normalized_tree}"
    );

    assert_inferred_type_snapshot(
        "test_normalize_type_preserves_recursive_local_edge",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_preserves_shadowed_class_method_generic() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            class Box<T> {
                map<T>(value: T): T {
                    return value;
                }
            }

            declare const box: Box<number>;
            export const result = box.map("value");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");

    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, result_ty),
        "value"
    ));
}

#[test]
fn test_infer_module_types_normalizes_union_variants_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readValue(value: string | (number | string)): string | (number | string) {
                return value;
            }

            export function readBoolean(value: true | false): true | false {
                return value;
            }

            export function readStringLiteral(value: string | "literal"): string | "literal" {
                return value;
            }

            export function readNumberLiteral(value: number | 1): number | 1 {
                return value;
            }

            export function readBigIntLiteral(value: bigint | 1n): bigint | 1n {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let value_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readValue")
        .expect("readValue return type must be inferred");
    let InferredTypeData::Union(union) = value_ty else {
        panic!("readValue must return a union, got {value_ty:?}");
    };
    assert_eq!(union.types(&db).len(), 2);
    assert!(
        union
            .types(&db)
            .iter()
            .all(|ty| !matches!(ty, InferredTypeData::Union(_)))
    );
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty))
    );

    let boolean_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readBoolean")
            .expect("readBoolean return type must be inferred");
    assert_eq!(boolean_ty, InferredTypeData::Boolean);

    let string_literal_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readStringLiteral")
            .expect("readStringLiteral return type must be inferred");
    assert_eq!(string_literal_ty, InferredTypeData::String);

    let number_literal_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readNumberLiteral")
            .expect("readNumberLiteral return type must be inferred");
    assert_eq!(number_literal_ty, InferredTypeData::Number);

    let bigint_literal_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readBigIntLiteral")
            .expect("readBigIntLiteral return type must be inferred");
    assert_eq!(bigint_literal_ty, InferredTypeData::BigInt);

    assert_inferred_type_snapshot(
        "test_infer_module_types_normalizes_union_variants_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_normalizes_intersection_variants_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type WithName = {
                name: string;
            };

            type WithValue = {
                value: number;
            };

            export function readCombined(
                value: WithName & (WithValue & WithName),
            ): WithName & (WithValue & WithName) {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let combined_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readCombined")
            .expect("readCombined return type must be inferred");
    let InferredTypeData::Intersection(intersection) = combined_ty else {
        panic!("readCombined must return an intersection, got {combined_ty:?}");
    };
    assert_eq!(intersection.types(&db).len(), 2);
    assert!(
        intersection
            .types(&db)
            .iter()
            .all(|ty| !matches!(ty, InferredTypeData::Intersection(_)))
    );

    let name_ty = inferred
        .find_member_type(&db, combined_ty, "name")
        .expect("normalized intersection must expose WithName.name");
    assert!(is_inferred_string(&db, name_ty));

    let value_ty = inferred
        .find_member_type(&db, combined_ty, "value")
        .expect("normalized intersection must expose WithValue.value");
    assert!(is_inferred_number(&db, value_ty));

    let normalized_ty = normalize_type(&db, index_module, combined_ty);
    let InferredTypeData::Object(normalized_object) = normalized_ty else {
        panic!("normalized Local intersection must become an object, got {normalized_ty:?}");
    };
    assert_eq!(normalized_object.members(&db).len(), 2);

    let normalized_name_ty = inferred
        .find_member_type(&db, normalized_ty, "name")
        .expect("normalized object must expose WithName.name");
    assert!(is_inferred_string(&db, normalized_name_ty));

    let normalized_value_ty = inferred
        .find_member_type(&db, normalized_ty, "value")
        .expect("normalized object must expose WithValue.value");
    assert!(is_inferred_number(&db, normalized_value_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_normalizes_intersection_variants_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_normalizes_primitive_intersections_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readString(value: string & string): string & string {
                return value;
            }

            export function readNever(value: string & number): string & number {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let string_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readString")
        .expect("readString return type must be inferred");
    assert_eq!(string_ty, InferredTypeData::String);

    let never_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readNever")
        .expect("readNever return type must be inferred");
    assert_eq!(never_ty, InferredTypeData::NeverKeyword);

    assert_inferred_type_snapshot(
        "test_infer_module_types_normalizes_primitive_intersections_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_merges_inline_object_intersections_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readCombined(
                value: { name: string } & { value: number },
            ): { name: string } & { value: number } {
                return value;
            }

            export function readValue(
                value: { value: string } & { value: number },
            ): { value: string } & { value: number } {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let combined_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readCombined")
            .expect("readCombined return type must be inferred");
    let InferredTypeData::Object(combined_object) = combined_ty else {
        panic!("readCombined must return a merged object, got {combined_ty:?}");
    };
    assert_eq!(combined_object.members(&db).len(), 2);

    let name_ty = inferred
        .find_member_type(&db, combined_ty, "name")
        .expect("merged object must expose name");
    assert!(is_inferred_string(&db, name_ty));

    let value_ty = inferred
        .find_member_type(&db, combined_ty, "value")
        .expect("merged object must expose value");
    assert!(is_inferred_number(&db, value_ty));

    let duplicate_value_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readValue")
            .expect("readValue return type must be inferred");
    let duplicate_value_member_ty = inferred
        .find_member_type(&db, duplicate_value_ty, "value")
        .expect("merged duplicate member must expose value");
    assert!(contains_inferred_string(&db, duplicate_value_member_ty));
    assert!(contains_inferred_number(&db, duplicate_value_member_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_merges_inline_object_intersections_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_merges_function_intersections_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readFunction(
                value: (() => string) & (() => number),
            ): (() => string) & (() => number) {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let function_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readFunction")
            .expect("readFunction return type must be inferred");
    let InferredTypeData::Function(function) = function_ty else {
        panic!("readFunction must return a merged function, got {function_ty:?}");
    };
    let InferredReturnType::Type(return_ty) = function.return_type(&db) else {
        panic!("merged function return type must be a type");
    };

    assert!(contains_inferred_string(&db, *return_ty));
    assert!(contains_inferred_number(&db, *return_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_merges_function_intersections_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_merges_mixed_intersections_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readCallableObject(
                value: (() => string) & { value: number },
            ): (() => string) & { value: number } {
                return value;
            }

            export function readPrimitive(
                value: string & { value: number },
            ): string & { value: number } {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let callable_object_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readCallableObject")
            .expect("readCallableObject return type must be inferred");
    let InferredTypeData::Object(callable_object) = callable_object_ty else {
        panic!("readCallableObject must return a merged object, got {callable_object_ty:?}");
    };
    assert_eq!(callable_object.members(&db).len(), 1);
    let value_ty = inferred
        .find_member_type(&db, callable_object_ty, "value")
        .expect("merged callable object must expose value");
    assert!(is_inferred_number(&db, value_ty));

    let primitive_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readPrimitive")
            .expect("readPrimitive return type must be inferred");
    let InferredTypeData::Intersection(primitive) = primitive_ty else {
        panic!("readPrimitive must preserve its branded intersection, got {primitive_ty:?}");
    };
    assert!(primitive.types(&db).contains(&InferredTypeData::String));
    assert!(
        primitive
            .types(&db)
            .iter()
            .any(|ty| matches!(ty, InferredTypeData::Object(_)))
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_merges_mixed_intersections_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_resolves_members_of_explicit_array_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/index.ts".into(),
        r#"
            declare const values: Array<number>;
            export const mapped = values.map(async value => value);
            export const chained = [1, 2, 3].map(value => value).map(async value => value);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for name in ["mapped", "chained"] {
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name)
            .expect("binding type must be inferred");
        let ty = inferred.resolve_type(&db, ty);
        assert!(
            is_inferred_array_of_promises(&db, ty),
            "{name} must be inferred as an array of Promises, got {}",
            format_inferred_type(&db, ty),
        );
    }
}

#[test]
fn test_infer_module_types_resolves_this_member_in_object_method() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/index.ts".into(),
        r#"
            const api = {
                promise: Promise.resolve("value"),
                getPromise() {
                    return this.promise;
                },
            };

            export const result = api.getPromise();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");
    assert!(is_inferred_promise_instance(
        &db,
        inferred.resolve_type(&db, result)
    ));
}
