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
fn builtin_class_calls_infer_declared_returns() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        export const anonymous = Symbol();
        export const described = Symbol("entry");
        export const numbered = Symbol(42);
        const create = Symbol;
        export const aliased = create("alias");
        export const errorMessage = Error("failure").message;
        declare const boxed: Symbol;
        export const invalidBoxCall = boxed();
        export const invalidInstanceCall = new Error("failure")();
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let inferred = infer_module_types(&db, module).unwrap();
    for name in ["anonymous", "described", "numbered", "aliased"] {
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name).unwrap();
        assert_eq!(
            inferred.resolve_type(&db, ty),
            InferredTypeData::Symbol,
            "{name}"
        );
    }
    let message = inferred_binding_ty_by_name(&db, module, inferred, "errorMessage").unwrap();
    assert!(is_inferred_string(&db, inferred.resolve_type(&db, message)));
    for name in ["invalidBoxCall", "invalidInstanceCall"] {
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name).unwrap();
        assert_eq!(
            inferred.resolve_type(&db, ty),
            InferredTypeData::Unknown,
            "{name}"
        );
    }
}

#[test]
fn symbol_static_members_infer_registry_calls_and_well_known_keys() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        export const registered = Symbol.for("entry");
        export const key = Symbol.keyFor(registered);
        export const iterator = Symbol.iterator;
        export const asyncIterator = Symbol.asyncIterator;
        export const tag = Symbol.toStringTag;
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
    for name in ["registered", "iterator", "asyncIterator", "tag"] {
        assert_eq!(
            binding(name).expand_canonical_global(&db),
            InferredTypeData::Symbol,
            "{name}"
        );
    }
    let key = binding("key");
    assert!(contains_inferred_string(&db, key));
    let InferredTypeData::Union(key) = key else {
        panic!("expected optional registry key")
    };
    assert!(key.types(&db).contains(&InferredTypeData::Undefined));
    assert_inferred_type_snapshot(
        "symbol_static_members_infer_registry_calls_and_well_known_keys",
        &db,
        &fs,
    );
}

#[test]
fn math_members_infer_as_statics_from_declarations() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        export const pi = Math.PI;
        export const e = Math.E;
        export const floored = Math.floor(1.5);
        export const largest = Math.max(1, 2, 3);
        export const power = Math.pow(2, 8);
        export const random = Math.random();
        export const truncated = Math.trunc(1.5);
        export const sign = Math.sign(-1);
        export const root = Math.cbrt(27);
        export const distance = Math.hypot(3, 4);
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
    for name in [
        "pi",
        "e",
        "floored",
        "largest",
        "power",
        "random",
        "truncated",
        "sign",
        "root",
        "distance",
    ] {
        assert!(is_inferred_number(&db, binding(name)), "{name}");
    }
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

#[test]
fn iterator_results_infer_yield_and_completion_values() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare const iterator: Iterator<string, number, boolean>;
        declare const result: IteratorResult<string, number>;
        export const nextResult = iterator.next(true);
        export const nextValue = nextResult.value;
        export const value = result.value;
        export const done = result.done;
        declare const defaultResult: IteratorResult<string>;
        export const defaultValue = defaultResult.value;
        type Defaulted<T, U = T> = IteratorYieldResult<U>;
        declare const dependentDefault: Defaulted<string>;
        export const dependentValue = dependentDefault.value;
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let inferred = infer_module_types(&db, module).unwrap();
    for name in ["value", "nextValue"] {
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name).unwrap();
        let ty = inferred.resolve_type(&db, ty);
        assert!(contains_inferred_string(&db, ty), "{name}: {ty:?}");
        assert!(contains_inferred_number(&db, ty), "{name}: {ty:?}");
    }
    let ty = inferred_binding_ty_by_name(&db, module, inferred, "defaultValue").unwrap();
    let ty = inferred.resolve_type(&db, ty);
    assert_eq!(ty, InferredTypeData::AnyKeyword);
    let ty = inferred_binding_ty_by_name(&db, module, inferred, "dependentValue").unwrap();
    assert!(is_inferred_string(&db, inferred.resolve_type(&db, ty)));
}

#[test]
fn iterable_annotations_resolve_declared_generic_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        export function retain(values: Iterable<number, string>) { return values; }
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let inferred = infer_module_types(&db, module).unwrap();
    let value = inferred_function_return_ty_by_name(&db, module, inferred, "retain").unwrap();
    let InferredTypeData::InstanceOf(instance) = value else {
        panic!("expected Iterable instance, got {value:?}")
    };
    let InferredTypeData::Interface(interface) = instance.ty(&db) else {
        panic!("expected declared interface")
    };
    assert_eq!(interface.name(&db).text(), "Iterable");
    assert!(is_inferred_number(&db, instance.type_parameters(&db)[0]));
    assert!(is_inferred_string(&db, instance.type_parameters(&db)[1]));
}

#[test]
fn lowered_constructors_preserve_explicit_collection_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        export const mapValue = new Map<string, number>().get("key");
        export const entryValue = new Map<string, number>([["key", 1]]).get("key");
        const createMap = Map;
        export const aliasedValue = new createMap<number, string>().get(1);
        declare const key: object;
        export const weakValue = new WeakMap<object, string>().get(key);
        export const setHas = new Set<string>().has("key");
        export const time = new Date(0).getTime();
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    let inferred = infer_module_types(&db, module).unwrap();
    for name in ["mapValue", "entryValue"] {
        let value = inferred_binding_ty_by_name(&db, module, inferred, name).unwrap();
        let value = inferred.resolve_type(&db, value);
        assert!(
            contains_inferred_number(&db, value),
            "{name}: {}",
            format_inferred_type(&db, value)
        );
    }
    for name in ["aliasedValue", "weakValue"] {
        let value = inferred_binding_ty_by_name(&db, module, inferred, name).unwrap();
        let value = inferred.resolve_type(&db, value);
        assert!(
            contains_inferred_string(&db, value),
            "{name}: {}",
            format_inferred_type(&db, value)
        );
    }
    let time = inferred_binding_ty_by_name(&db, module, inferred, "time").unwrap();
    assert!(is_inferred_number(&db, inferred.resolve_type(&db, time)));
    let has = inferred_binding_ty_by_name(&db, module, inferred, "setHas").unwrap();
    assert!(is_inferred_boolean(&db, inferred.resolve_type(&db, has)));
}

#[test]
fn regexp_declarations_infer_results_and_constructor_calls() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare const annotated: RegExpExecArray;
        export const annotatedIndex = annotated.index;
        export const annotatedInput = annotated.input;
        export const first = annotated["0"];
        export const length = annotated.length;
        export const execute = /ab/.exec;
        export const result = /ab/.exec("abc");
        export const resultIndex = result?.index;
        export const resultInput = result?.input;
        export const constructed = new RegExp("ab", "g").exec("abc");
        export const called = RegExp("ab", "g").exec("abc");
        export const constructedIndex = constructed?.index;
        export const calledIndex = called?.index;
        export const copied = new RegExp(/ab/).source;
        export const calledCopy = RegExp(/ab/).source;
        const create = RegExp;
        export const aliased = create("ab").test("abc");
        export const constructedAlias = new create("ab").lastIndex;
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
    for name in ["annotatedIndex", "length", "constructedAlias"] {
        assert!(
            is_inferred_number(&db, binding(name)),
            "{name}: {:?}",
            binding(name)
        );
    }
    for name in ["annotatedInput", "first", "copied", "calledCopy"] {
        assert!(
            is_inferred_string(&db, binding(name)),
            "{name}: {:?}",
            binding(name)
        );
    }
    assert!(is_inferred_boolean(&db, binding("aliased")));
    for name in ["resultIndex", "constructedIndex", "calledIndex"] {
        assert!(
            contains_inferred_number(&db, binding(name)),
            "{name}: {:?}",
            binding(name)
        );
    }
    assert!(contains_inferred_string(&db, binding("resultInput")));
    for name in ["result", "constructed", "called"] {
        let ty = binding(name);
        assert!(contains_inferred_null(&db, ty), "{name}: {ty:?}");
    }
    assert_inferred_type_snapshot(
        "regexp_declarations_infer_results_and_constructor_calls",
        &db,
        &fs,
    );
}

#[test]
fn array_from_calls_use_lowered_overloads() {
    use biome_module_graph::{BindingTypeInput, infer_binding_type};

    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare const values: ArrayLike<string>;
        declare const iterable: Iterable<number>;
        export const copied = Array.from(values);
        export const iterated = Array.from(iterable);
        export const mapped = Array.from(values, value => 1);
        export const withThis = Array.from(values, value => true, {});
        export const mappedIterable = Array.from(iterable, value => "text");
        export const indexes = Array.from(values, (value, index) => index);
        const from = Array.from;
        export const aliased = from(values, value => "text");
        export const length = values.length;
        export const noArguments = Array.from();
        declare const array: string[];
        export const instanceFrom = array.from;
    "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db.module_for_path(Utf8Path::new("/src/index.ts")).unwrap();
    db.clear_salsa_events();
    let binding = |name| {
        let input = BindingTypeInput::new(&db, module, binding_range_by_name(&db, module, name));
        normalize_type(&db, module, infer_binding_type(&db, input).unwrap())
    };
    for name in [
        "copied",
        "iterated",
        "mapped",
        "withThis",
        "mappedIterable",
        "indexes",
        "aliased",
    ] {
        let ty = binding(name);
        let InferredTypeData::InstanceOf(instance) = ty else {
            panic!("{name}: expected array, got {ty:?}");
        };
        let InferredTypeData::Class(class) = instance.ty(&db).expand_canonical_global(&db) else {
            panic!("{name}: expected Array class");
        };
        assert_eq!(class.name(&db).as_ref().map(Text::text), Some("Array"));
        let [element] = instance.type_parameters(&db).as_ref() else {
            panic!("{name}: expected one element type");
        };
        match name {
            "mapped" | "indexes" => {
                assert!(is_inferred_number(&db, *element), "{name}: {element:?}")
            }
            "withThis" => assert!(is_inferred_boolean(&db, *element)),
            "mappedIterable" | "aliased" => assert!(is_inferred_string(&db, *element)),
            _ => {}
        }
    }
    assert!(is_inferred_number(&db, binding("length")));
    assert_eq!(binding("noArguments"), InferredTypeData::Unknown);
    assert_eq!(binding("instanceFrom"), InferredTypeData::Unknown);
    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, infer_module_types, module, &events);
}

#[test]
fn generated_computed_members_resolve_on_values() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare const map: Map<string, number>;
        declare const set: Set<string>;
        declare const weak: WeakMap<object, string>;
        declare const date: Date;
        declare const expression: RegExp;
        export const mathTag = Math[Symbol.toStringTag];
        export const mapTag = map[Symbol.toStringTag];
        export const setTag = set[Symbol.toStringTag];
        export const weakTag = weak[Symbol.toStringTag];
        export const arraySpecies = Array[Symbol.species];
        export const regexpSpecies = RegExp[Symbol.species];
        export const search = expression[Symbol.search]("text");
        export const text = date[Symbol.toPrimitive]("string");
        export const number = date[Symbol.toPrimitive]("number");
        export const replaced = expression[Symbol.replace]("text", "replacement");
        export const replacedCallback = expression[Symbol.replace]("text", () => "replacement");
        const tag = Symbol.toStringTag;
        export const aliasedTag = map[tag];
        declare const optional: Map<string, number> | undefined;
        export const optionalTag = optional?.[tag];
        interface TagName {
            "Symbol.toStringTag": number;
        }
        interface TaggedMap extends Map<string, number>, TagName {}
        declare const tagged: TaggedMap;
        export const symbolTag = tagged[Symbol.toStringTag];
        export const stringTag = tagged["Symbol.toStringTag"];
        export function shadowed(Symbol: { toStringTag: "size" }) {
            return map[Symbol.toStringTag];
        }
        "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    assert_inferred_type_snapshot("generated_computed_members_resolve_on_values", &db, &fs);
}

#[test]
fn generated_computed_iterators_preserve_collection_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        declare const map: Map<string, number>;
        declare const set: Set<string>;
        export const entries = map[Symbol.iterator]();
        export const values = set[Symbol.iterator]();
        export const entry = entries.next().value;
        export const value = values.next().value;
        export const repeated = entries[Symbol.iterator]();
        const iterator: typeof Symbol.iterator = Symbol.iterator;
        export const aliased = map[iterator]();
        "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    assert_inferred_type_snapshot(
        "generated_computed_iterators_preserve_collection_arguments",
        &db,
        &fs,
    );
}

#[test]
fn generated_intl_namespace_infers_constructor_and_method_results() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
        export const compared = new Intl.Collator("en").compare("a", "b");
        export const calledCompare = Intl.Collator().compare("a", "b");
        export const numeric = Intl.Collator().resolvedOptions().numeric;
        export const numberText = new Intl.NumberFormat("en").format(42);
        export const calledNumberText = Intl.NumberFormat().format(42);
        export const dateText = new Intl.DateTimeFormat("en").format(0);
        export const calledDateText = Intl.DateTimeFormat().format(new Date());
        export const calendar = Intl.DateTimeFormat().resolvedOptions().calendar;
        export const locales = Intl.NumberFormat.supportedLocalesOf(["en"]);
        export const prototypeText = Intl.NumberFormat.prototype.format(42);
        const Format = Intl.NumberFormat;
        export const aliasedText = new Format().format(42);
        export const localeMethod = new Date().toLocaleString;
        "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    assert_inferred_type_snapshot(
        "generated_intl_namespace_infers_constructor_and_method_results",
        &db,
        &fs,
    );
}
