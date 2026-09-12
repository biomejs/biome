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
    assert_eq!(map_instance.ty(&db), InferredTypeData::map_class());
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
    assert_eq!(set_instance.ty(&db), InferredTypeData::set_class());
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
        InferredTypeData::weak_map_class()
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

#[test]
fn weak_map_members_infer_calls_with_instance_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare const strings: WeakMap<object, string>;
        declare const numbers: WeakMap<object, number>;
        declare const key: object;
        export const text = strings.get(key);
        export const number = numbers.get(key);
        export const present = strings.has(key);
        export const removed = numbers.delete(key);
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let inferred = infer_module_types(&db, module).unwrap();
    for (name, expected) in [
        ("text", InferredTypeData::String),
        ("number", InferredTypeData::Number),
    ] {
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name).unwrap();
        let ty = inferred.resolve_type(&db, ty);
        let InferredTypeData::Union(union) = ty else {
            panic!("expected optional value for {name}, got {ty:?}")
        };
        assert!(
            union
                .types(&db)
                .iter()
                .any(|ty| inferred.resolve_type(&db, *ty) == expected),
            "{name}: {} {:?}",
            format_inferred_type(&db, ty),
            union.types(&db)
        );
        assert!(union.types(&db).contains(&InferredTypeData::Undefined));
    }
    for name in ["present", "removed"] {
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name).unwrap();
        assert_eq!(inferred.resolve_type(&db, ty), InferredTypeData::Boolean);
    }
}

#[test]
fn map_and_set_members_infer_instance_values_and_callback_parameters() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare const map: Map<string, number>;
        declare const set: Set<string>;
        export const value = map.get("key");
        export const mapSize = map.size;
        export const setSize = set.size;
        export const mapHas = map.has("key");
        export const setHas = set.has("value");
        export const mapDeleted = map.delete("key");
        export const setDeleted = set.delete("value");
        export const mapCleared = map.clear();
        export const setCleared = set.clear();
        export const mapVisit = map.forEach;
        export const setVisit = set.forEach;
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let inferred = infer_module_types(&db, module).unwrap();
    let binding = |name| {
        inferred.resolve_type(
            &db,
            inferred_binding_ty_by_name(&db, module, inferred, name).unwrap(),
        )
    };
    assert!(contains_inferred_number(&db, binding("value")));
    let InferredTypeData::Union(value) = binding("value") else {
        panic!("expected optional value")
    };
    assert!(value.types(&db).contains(&InferredTypeData::Undefined));
    for name in ["mapSize", "setSize"] {
        assert!(is_inferred_number(&db, binding(name)));
    }
    for name in ["mapHas", "setHas", "mapDeleted", "setDeleted"] {
        assert!(is_inferred_boolean(&db, binding(name)));
    }
    for name in ["mapCleared", "setCleared"] {
        assert_eq!(binding(name), InferredTypeData::VoidKeyword);
    }
    for (name, expected_values) in [
        (
            "mapVisit",
            [InferredTypeData::Number, InferredTypeData::String],
        ),
        (
            "setVisit",
            [InferredTypeData::String, InferredTypeData::String],
        ),
    ] {
        let InferredTypeData::Function(method) = binding(name) else {
            panic!("expected method")
        };
        let InferredTypeData::Function(callback) = method.parameters(&db)[0].ty() else {
            panic!("expected callback")
        };
        for (parameter, expected) in callback.parameters(&db).iter().zip(expected_values) {
            assert_eq!(inferred.resolve_type(&db, parameter.ty()), expected);
        }
        let owner = callback.parameters(&db)[2].ty();
        let InferredTypeData::InstanceOf(owner) = owner else {
            panic!("expected callback owner")
        };
        let member = if name == "mapVisit" { "get" } else { "has" };
        assert!(
            inferred
                .find_member_type(&db, InferredTypeData::InstanceOf(owner), member)
                .is_some()
        );
        assert_eq!(
            owner
                .type_parameters(&db)
                .last()
                .copied()
                .map(|ty| inferred.resolve_type(&db, ty)),
            Some(expected_values[0])
        );
    }
}

#[test]
fn date_and_regexp_scalar_members_infer_from_declarations() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare const date: Date;
        export const time = date.getTime();
        export const adjusted = date.setHours(12, 30);
        export const iso = date.toISOString();
        export const locale = date.toLocaleDateString();
        export const json = date.toJSON();
        export const matched = /ab/.test("abc");
        export const source = /ab/.source;
        export const global = /ab/g.global;
        export const ignoreCase = /ab/i.ignoreCase;
        export const multiline = /ab/m.multiline;
        export const lastIndex = /ab/g.lastIndex;
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let inferred = infer_module_types(&db, module).unwrap();
    let binding = |name| {
        inferred.resolve_type(
            &db,
            inferred_binding_ty_by_name(&db, module, inferred, name).unwrap(),
        )
    };
    for name in ["time", "adjusted", "lastIndex"] {
        assert!(is_inferred_number(&db, binding(name)), "{name}");
    }
    for name in ["iso", "locale", "json", "source"] {
        assert!(is_inferred_string(&db, binding(name)), "{name}");
    }
    for name in ["matched", "global", "ignoreCase", "multiline"] {
        assert!(is_inferred_boolean(&db, binding(name)), "{name}");
    }
}
