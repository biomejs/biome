use super::*;

#[test]
fn test_infer_module_types_resolves_generic_builtin_instances_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readMap(value: Map<string, number>): Map<string, number> {
                return value;
            }

            export function readSet(value: Set<string>): Set<string> {
                return value;
            }

            export function readWeakMap(value: WeakMap<object, string>): WeakMap<object, string> {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let map_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readMap")
        .expect("readMap return type must be inferred");
    let InferredTypeData::InstanceOf(map_instance) = map_ty else {
        panic!("readMap must return a Map instance, got {map_ty:?}");
    };
    assert_eq!(map_instance.ty(&db), InferredTypeData::map_class(&db));
    assert_eq!(map_instance.type_parameters(&db).len(), 2);
    assert!(is_inferred_string(
        &db,
        map_instance.type_parameters(&db)[0]
    ));
    assert!(is_inferred_number(
        &db,
        map_instance.type_parameters(&db)[1]
    ));

    let set_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readSet")
        .expect("readSet return type must be inferred");
    let InferredTypeData::InstanceOf(set_instance) = set_ty else {
        panic!("readSet must return a Set instance, got {set_ty:?}");
    };
    assert_eq!(set_instance.ty(&db), InferredTypeData::set_class(&db));
    assert_eq!(set_instance.type_parameters(&db).len(), 1);
    assert!(is_inferred_string(
        &db,
        set_instance.type_parameters(&db)[0]
    ));

    let weak_map_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readWeakMap")
            .expect("readWeakMap return type must be inferred");
    let InferredTypeData::InstanceOf(weak_map_instance) = weak_map_ty else {
        panic!("readWeakMap must return a WeakMap instance, got {weak_map_ty:?}");
    };
    assert_eq!(
        weak_map_instance.ty(&db),
        InferredTypeData::weak_map_class(&db)
    );
    assert_eq!(weak_map_instance.type_parameters(&db).len(), 2);
    assert!(is_inferred_string(
        &db,
        weak_map_instance.type_parameters(&db)[1]
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_generic_builtin_instances_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_resolves_builtin_global_identities_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readRegExp(value: RegExp): RegExp {
                return value;
            }

            export function readDate(value: Date): Date {
                return value;
            }

            export function readError(value: Error): Error {
                return value;
            }

            export function readSymbol(value: Symbol): Symbol {
                return value;
            }

            export function readDisposable(value: Disposable): Disposable {
                return value;
            }

            export function readAsyncDisposable(value: AsyncDisposable): AsyncDisposable {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    for (function_name, class_name) in [
        ("readRegExp", "RegExp"),
        ("readDate", "Date"),
        ("readError", "Error"),
        ("readSymbol", "Symbol"),
    ] {
        let ty = inferred_function_return_ty_by_name(&db, index_module, inferred, function_name)
            .unwrap_or_else(|| panic!("{function_name} return type must be inferred"));
        let InferredTypeData::InstanceOf(instance) = ty else {
            panic!("{function_name} must return a {class_name} instance, got {ty:?}");
        };
        let InferredTypeData::Class(class) = instance.ty(&db) else {
            panic!("{function_name} must return a {class_name} instance, got {ty:?}");
        };
        assert_eq!(class.name(&db).as_ref().map(Text::text), Some(class_name));
    }

    for (function_name, interface_name) in [
        ("readDisposable", "Disposable"),
        ("readAsyncDisposable", "AsyncDisposable"),
    ] {
        let ty = inferred_function_return_ty_by_name(&db, index_module, inferred, function_name)
            .unwrap_or_else(|| panic!("{function_name} return type must be inferred"));
        let InferredTypeData::InstanceOf(instance) = ty else {
            panic!("{function_name} must return an {interface_name} instance, got {ty:?}");
        };
        let InferredTypeData::Interface(interface) = instance.ty(&db) else {
            panic!("{function_name} must return an {interface_name} instance, got {ty:?}");
        };
        assert_eq!(interface.name(&db).text(), interface_name);
    }

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_builtin_global_identities_on_build",
        &db,
        &fs,
    );
}
