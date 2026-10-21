use super::*;
use biome_js_type_info::resolved::{
    InferredFunction, InferredNamedFunctionParameter, InferredTuple, InferredTupleElementType,
};

fn inferred_function_type<'db>(
    db: &'db dyn ModuleDb,
    parameters: Vec<InferredFunctionParameter<'db>>,
) -> InferredTypeData<'db> {
    InferredTypeData::Function(InferredFunction::new(
        db,
        Box::default(),
        parameters.into_boxed_slice(),
        InferredReturnType::Type(InferredTypeData::VoidKeyword),
        false,
        None,
    ))
}

fn inferred_function_parameter<'db>(
    name: &'static str,
    ty: InferredTypeData<'db>,
    is_rest: bool,
) -> InferredFunctionParameter<'db> {
    InferredFunctionParameter::Named(InferredNamedFunctionParameter {
        name: Text::new_static(name),
        ty,
        is_optional: false,
        is_rest,
    })
}

fn inferred_tuple_type<'db>(
    db: &'db dyn ModuleDb,
    elements: Vec<InferredTupleElementType<'db>>,
) -> InferredTypeData<'db> {
    InferredTypeData::Tuple(InferredTuple::new(db, elements.into_boxed_slice(), false))
}

fn inferred_tuple_element(ty: InferredTypeData<'_>, is_rest: bool) -> InferredTupleElementType<'_> {
    InferredTupleElementType {
        ty,
        name: None,
        is_optional: false,
        is_rest,
    }
}

fn nested_rest_tuple<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    depth: usize,
) -> InferredTypeData<'db> {
    let mut ty = inferred_tuple_type(db, Vec::from([inferred_tuple_element(ty, false)]));
    for _ in 0..depth {
        ty = inferred_tuple_type(db, Vec::from([inferred_tuple_element(ty, true)]));
    }
    ty
}

#[test]
fn test_infer_call_argument_type_handles_deep_typeof_wrappers() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function consume(callback: () => void) {}
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let mut consume = inferred_binding_ty_by_name(&db, module, inferred, "consume")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("consume binding type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");

    for _ in 0..512 {
        consume = InferredTypeData::TypeofType(InferredTypeofType::new(&db, consume));
    }

    let input = CallArgumentTypeInput::new(
        &db,
        consume,
        Vec::from([InferredCallArgumentType::Argument(callback)]).into_boxed_slice(),
        0,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("call expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_call_argument_type_preserves_parameter_mapping_after_spread() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function consume(prefix: number, callback: () => void) {}
            export const prefixes: number[] = [];
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let consume = inferred_binding_ty_by_name(&db, module, inferred, "consume")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("consume binding type must be inferred");
    let prefixes = inferred_binding_ty_by_name(&db, module, inferred, "prefixes")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("prefixes binding type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");
    let input = CallArgumentTypeInput::new(
        &db,
        consume,
        Vec::from([
            InferredCallArgumentType::Spread(prefixes),
            InferredCallArgumentType::Argument(callback),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("call expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_call_argument_type_preserves_parameter_mapping_after_tuple_spread() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function consume(prefix: number, label: string, callback: () => void) {}
            export const prefixes: [number, string] = [0, "label"];
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let consume = inferred_binding_ty_by_name(&db, module, inferred, "consume")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("consume binding type must be inferred");
    let prefixes = inferred_binding_ty_by_name(&db, module, inferred, "prefixes")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("prefixes binding type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");
    let input = CallArgumentTypeInput::new(
        &db,
        consume,
        Vec::from([
            InferredCallArgumentType::Spread(prefixes),
            InferredCallArgumentType::Argument(callback),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("call expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_call_argument_type_expands_nested_tuple_rest_spread() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function consume(
                prefix: number,
                label: string,
                enabled: boolean,
                callback: () => void,
            ) {}
            type Prefix = [number, ...[string, boolean]];
            export const prefixes: Prefix = [0, "label", true];
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let consume = inferred_binding_ty_by_name(&db, module, inferred, "consume")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("consume binding type must be inferred");
    let prefixes = inferred_binding_ty_by_name(&db, module, inferred, "prefixes")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("prefixes binding type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");
    let input = CallArgumentTypeInput::new(
        &db,
        consume,
        Vec::from([
            InferredCallArgumentType::Spread(prefixes),
            InferredCallArgumentType::Argument(callback),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("call expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_call_argument_type_handles_recursive_tuple_spreads() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Recursive<T> = [...Recursive<T>];
            export declare const recursive: Recursive<number>;
            export declare function actual(callback: () => void): void;
            export declare function formal(
                ...args: [...Recursive<number>, () => void]
            ): void;
            export const callback = () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let binding_type = |name| {
        inferred_binding_ty_by_name(&db, module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        )
    };
    let actual = binding_type("actual");
    let formal = binding_type("formal");
    let recursive = binding_type("recursive");
    let callback = binding_type("callback");

    let actual_input = CallArgumentTypeInput::new(
        &db,
        actual,
        Vec::from([
            InferredCallArgumentType::Spread(recursive),
            InferredCallArgumentType::Argument(callback),
        ])
        .into_boxed_slice(),
        1,
    );
    let formal_input = CallArgumentTypeInput::new(
        &db,
        formal,
        Vec::from([InferredCallArgumentType::Argument(callback)]).into_boxed_slice(),
        0,
    );

    for expected in [
        infer_call_argument_type(&db, actual_input),
        infer_call_argument_type(&db, formal_input),
    ] {
        let expected = expected.expect("callback expected type must be inferred");
        assert!(
            InferredType::new(&db, expected).function_returns_void(),
            "expected void callback, got {expected:?}"
        );
    }
}

#[test]
fn test_infer_call_argument_type_bounds_actual_tuple_spread_expansion() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function consume(prefix: number, callback: () => void) {}
            export const callback = () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let consume = inferred_binding_ty_by_name(&db, module, inferred, "consume")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("consume binding type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");

    for (case, depth) in [("below limit", 256), ("above limit", 1024)] {
        let prefixes = nested_rest_tuple(&db, InferredTypeData::Number, depth);
        let input = CallArgumentTypeInput::new(
            &db,
            consume,
            Vec::from([
                InferredCallArgumentType::Spread(prefixes),
                InferredCallArgumentType::Argument(callback),
            ])
            .into_boxed_slice(),
            1,
        );
        let expected = infer_call_argument_type(&db, input)
            .unwrap_or_else(|| panic!("{case} expected type must be inferred"));

        assert!(
            InferredType::new(&db, expected).function_returns_void(),
            "expected void callback for {case}, got {expected:?}"
        );
    }
}

#[test]
fn test_infer_call_argument_type_bounds_formal_tuple_rest_expansion() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const callback = () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");

    for (case, depth) in [("below limit", 128), ("above limit", 1024)] {
        let prefixes = nested_rest_tuple(&db, InferredTypeData::Number, depth);
        let callback_suffix = nested_rest_tuple(&db, callback, 1);
        let rest_tuple = inferred_tuple_type(
            &db,
            Vec::from([
                inferred_tuple_element(prefixes, true),
                inferred_tuple_element(callback_suffix, true),
            ]),
        );
        let callee = inferred_function_type(
            &db,
            Vec::from([inferred_function_parameter("args", rest_tuple, true)]),
        );
        let input = CallArgumentTypeInput::new(
            &db,
            callee,
            Vec::from([
                InferredCallArgumentType::Argument(InferredTypeData::Number),
                InferredCallArgumentType::Argument(callback),
            ])
            .into_boxed_slice(),
            1,
        );
        let expected = infer_call_argument_type(&db, input)
            .unwrap_or_else(|| panic!("{case} expected type must be inferred"));

        assert!(
            InferredType::new(&db, expected).function_returns_void(),
            "expected void callback for {case}, got {expected:?}"
        );
    }
}

#[test]
fn test_infer_call_argument_type_preserves_wide_formal_tuple_suffix() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const callback = () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");

    let mut elements = (0..2048)
        .map(|_| InferredTupleElementType {
            ty: InferredTypeData::Number,
            name: None,
            is_optional: true,
            is_rest: false,
        })
        .collect::<Vec<_>>();
    elements.push(inferred_tuple_element(callback, false));
    let rest_tuple = inferred_tuple_type(&db, elements);
    let callee = inferred_function_type(
        &db,
        Vec::from([inferred_function_parameter("args", rest_tuple, true)]),
    );
    let input = CallArgumentTypeInput::new(
        &db,
        callee,
        Vec::from([InferredCallArgumentType::Argument(callback)]).into_boxed_slice(),
        0,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("callback expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_call_argument_type_maps_formal_rest_tuple_elements() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function fixed(...args: [number, () => void]): void;
            export declare function variadic(
                ...args: [number, ...string[], () => void]
            ): void;
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");

    for (case, name, args, argument_index) in [
        (
            "fixed",
            "fixed",
            Vec::from([
                InferredCallArgumentType::Argument(InferredTypeData::Number),
                InferredCallArgumentType::Argument(callback),
            ]),
            1,
        ),
        (
            "empty middle rest",
            "variadic",
            Vec::from([
                InferredCallArgumentType::Argument(InferredTypeData::Number),
                InferredCallArgumentType::Argument(callback),
            ]),
            1,
        ),
        (
            "populated middle rest",
            "variadic",
            Vec::from([
                InferredCallArgumentType::Argument(InferredTypeData::Number),
                InferredCallArgumentType::Argument(InferredTypeData::String),
                InferredCallArgumentType::Argument(InferredTypeData::String),
                InferredCallArgumentType::Argument(callback),
            ]),
            3,
        ),
    ] {
        let callee = inferred_binding_ty_by_name(&db, module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        );
        let input =
            CallArgumentTypeInput::new(&db, callee, args.into_boxed_slice(), argument_index);
        let expected = infer_call_argument_type(&db, input)
            .unwrap_or_else(|| panic!("{case} expected type must be inferred"));

        assert!(
            InferredType::new(&db, expected).function_returns_void(),
            "expected void callback for {case}, got {expected:?}"
        );
    }
}

#[test]
fn test_infer_call_argument_type_substitutes_generic_tuple_spread() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Consumer {
                (prefix: string, label: string, callback: () => Promise<void>): void;
                (prefix: number, label: string, callback: () => void): void;
            }
            export declare const consume: Consumer;
            type Prefixes<T> = [T, string];
            export const prefixes: Prefixes<number> = [0, "label"];
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let consume = inferred_binding_ty_by_name(&db, module, inferred, "consume")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("consume binding type must be inferred");
    let prefixes = inferred_binding_ty_by_name(&db, module, inferred, "prefixes")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("prefixes binding type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");
    let input = CallArgumentTypeInput::new(
        &db,
        consume,
        Vec::from([
            InferredCallArgumentType::Spread(prefixes),
            InferredCallArgumentType::Argument(callback),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("call expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_call_argument_type_ignores_missing_later_argument_for_single_signature() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function consume(callback: () => void, marker: number) {}
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let consume = inferred_binding_ty_by_name(&db, module, inferred, "consume")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("consume binding type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");
    let input = CallArgumentTypeInput::new(
        &db,
        consume,
        Vec::from([InferredCallArgumentType::Argument(callback)]).into_boxed_slice(),
        0,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("call expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_call_argument_type_checks_fixed_overload_suffix_beyond_sequence_limit() {
    let mut source = String::new();
    for (marker_type, callback_return) in [("string", "string"), ("boolean", "void")] {
        source.push_str("export declare function consume(");
        for index in 0..1024 {
            if index > 0 {
                source.push_str(", ");
            }
            source.push_str(&format!("value{index}: number"));
        }
        source.push_str(&format!(
            ", marker: {marker_type}, callback: () => {callback_return}): void;\n"
        ));
    }
    source.push_str("export const callback = () => {};\n");

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let consume = inferred_overload_ty_by_name(&db, module, inferred, "consume")
        .expect("consume overload type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");

    let mut args = vec![InferredCallArgumentType::Argument(InferredTypeData::Number); 1024];
    args.push(InferredCallArgumentType::Argument(
        InferredTypeData::Boolean,
    ));
    args.push(InferredCallArgumentType::Argument(callback));
    let input = CallArgumentTypeInput::new(&db, consume, args.into_boxed_slice(), 1025);
    let expected =
        infer_call_argument_type(&db, input).expect("callback expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_constructor_argument_type_selects_overload_by_arity() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Consumer {
                constructor(prefix: number, callback: () => void);
                constructor(prefix: number, callback: () => Promise<void>, marker: number);
                constructor(
                    _prefix: number,
                    _callback: (() => void) | (() => Promise<void>),
                    _marker?: number,
                ) {}
            }
            export const prefixes: number[] = [];
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let mut consumer = inferred_binding_ty_by_name(&db, module, inferred, "Consumer")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("Consumer binding type must be inferred");
    let prefixes = inferred_binding_ty_by_name(&db, module, inferred, "prefixes")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("prefixes binding type must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .expect("callback binding type must be inferred");
    for _ in 0..512 {
        consumer = InferredTypeData::TypeofType(InferredTypeofType::new(&db, consumer));
    }
    let input = CallArgumentTypeInput::new(
        &db,
        consumer,
        Vec::from([
            InferredCallArgumentType::Spread(prefixes),
            InferredCallArgumentType::Argument(inferred.resolve_type(&db, callback)),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected = infer_constructor_argument_type(&db, input)
        .expect("constructor expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_constructor_argument_type_resolves_canonical_global_signature() {
    let fs = MemoryFileSystem::default();
    let db = build_js_test_module_db(&fs, &[], true);
    let InferredTypeData::GlobalType(promise_id) = InferredTypeData::promise_class() else {
        panic!("expected canonical Promise type");
    };
    let promise = biome_js_type_info::global_types(&db).get(promise_id);
    let input = CallArgumentTypeInput::new(
        &db,
        promise,
        Vec::from([InferredCallArgumentType::Argument(
            InferredTypeData::Unknown,
        )])
        .into_boxed_slice(),
        0,
    );
    let expected = infer_constructor_argument_type(&db, input)
        .expect("Promise constructor callback type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_constructor_argument_type_supports_interface_and_object_signatures() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Result {}
            export interface InterfaceConsumer {
                new(callback: () => void): Result;
                new(callback: () => Promise<void>, marker: number): Result;
            }
            export type ObjectConsumer = {
                new(callback: () => void): Result;
                new(callback: () => Promise<void>, marker: number): Result;
            };
            export declare const interfaceConsumer: InterfaceConsumer;
            export declare const objectConsumer: ObjectConsumer;
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let callback = inferred_binding_ty_by_name(&db, module, inferred, "callback")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("callback binding type must be inferred");

    for name in ["interfaceConsumer", "objectConsumer"] {
        let callee = inferred_binding_ty_by_name(&db, module, inferred, name).map_or_else(
            || panic!("{name} binding type must be inferred"),
            |ty| inferred.resolve_type(&db, ty),
        );
        let input = CallArgumentTypeInput::new(
            &db,
            callee,
            Vec::from([InferredCallArgumentType::Argument(callback)]).into_boxed_slice(),
            0,
        );
        let expected = infer_constructor_argument_type(&db, input)
            .unwrap_or_else(|| panic!("{name} expected type must be inferred"));

        assert!(
            InferredType::new(&db, expected).function_returns_void(),
            "expected void callback for {name}, got {expected:?}"
        );
    }
}

#[test]
fn test_infer_call_argument_type_applies_explicit_type_arguments_to_overload_sets() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function run<T>(value: T, callback: (value: T) => void): void;
            export declare function run<T>(value: T | null, callback: (value: T | null) => void): void;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let run = inferred_overload_ty_by_name(&db, module, inferred, "run")
        .expect("run overload set must be inferred");

    // `run<string>(null, callback)`: the callee carries the explicit argument
    // exactly as `callee_reference` records it for a call expression.
    let callee = InferredTypeData::instance_of(&db, run, Box::from([InferredTypeData::String]));
    let input = CallArgumentTypeInput::new(
        &db,
        callee,
        Vec::from([
            InferredCallArgumentType::Argument(InferredTypeData::Null),
            InferredCallArgumentType::Argument(InferredTypeData::Unknown),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("callback expected type must be inferred");

    // `<string>` turns the first overload into `(value: string, ...)`, which
    // `null` rejects, so the callback comes from the nullable overload.
    let InferredTypeData::Function(function) = expected else {
        panic!(
            "expected a callback type, got {}",
            format_inferred_type(&db, expected)
        );
    };
    let value_ty = function.parameters(&db)[0].ty();
    let InferredTypeData::Union(union) = value_ty else {
        panic!(
            "callback parameter must be `string | null`, got {}",
            format_inferred_type(&db, value_ty)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );
}

#[test]
fn test_infer_call_argument_type_checks_overload_candidacy_for_explicit_type_arguments() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function pick<T extends string>(x: T, callback: (value: T) => void): void;
            export declare function pick<T>(x: T, callback: (value: T | null) => void): void;

            export declare function f<T>(x: T, callback: (value: T) => void): void;
            export declare function f(x: null, callback: (value: object) => void): void;
            export declare function f<T>(x: T | null, callback: (value: T | null) => void): void;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let callback_parameter = |name: &str, type_argument, first_argument| {
        let overloads = inferred_overload_ty_by_name(&db, module, inferred, name)
            .unwrap_or_else(|| panic!("{name} overload set must be inferred"));
        let callee = InferredTypeData::instance_of(&db, overloads, Box::from([type_argument]));
        let input = CallArgumentTypeInput::new(
            &db,
            callee,
            Vec::from([
                InferredCallArgumentType::Argument(first_argument),
                InferredCallArgumentType::Argument(InferredTypeData::Unknown),
            ])
            .into_boxed_slice(),
            1,
        );
        let expected = infer_call_argument_type(&db, input)
            .unwrap_or_else(|| panic!("{name} callback expected type must be inferred"));
        let InferredTypeData::Function(function) = expected else {
            panic!(
                "{name} must expect a callback, got {}",
                format_inferred_type(&db, expected)
            );
        };
        function.parameters(&db)[0].ty()
    };

    // `pick<number>(0, cb)`: `number` violates `T extends string`, so the
    // callback comes from the unconstrained overload.
    let value_ty = callback_parameter("pick", InferredTypeData::Number, InferredTypeData::Number);
    let InferredTypeData::Union(union) = value_ty else {
        panic!(
            "pick callback parameter must be `number | null`, got {}",
            format_inferred_type(&db, value_ty)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty))
    );

    // `f<string>(null, cb)`: the non-generic overload is not a candidate, and
    // `(x: string)` rejects `null`, so the nullable overload is selected.
    let value_ty = callback_parameter("f", InferredTypeData::String, InferredTypeData::Null);
    let InferredTypeData::Union(union) = value_ty else {
        panic!(
            "f callback parameter must be `string | null`, got {}",
            format_inferred_type(&db, value_ty)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );
}

#[test]
fn test_infer_call_argument_type_accepts_union_type_arguments_within_generic_constraints() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function pick<T extends string | number | boolean>(
                x: T,
                callback: (value: T | null) => void,
            ): void;
            export declare function pick<T>(x: T, callback: (value: T) => void): void;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let overloads = inferred_overload_ty_by_name(&db, module, inferred, "pick")
        .expect("pick overload set must be inferred");

    // `pick<string | number>(0, cb)`: every member of the written union
    // satisfies the constraint, so the callback comes from the first
    // overload and accepts `null`.
    let type_argument = InferredTypeData::union_from_types(
        &db,
        Vec::from([InferredTypeData::String, InferredTypeData::Number]),
    );
    let callee = InferredTypeData::instance_of(&db, overloads, Box::from([type_argument]));
    let input = CallArgumentTypeInput::new(
        &db,
        callee,
        Vec::from([
            InferredCallArgumentType::Argument(InferredTypeData::Number),
            InferredCallArgumentType::Argument(InferredTypeData::Unknown),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("callback expected type must be inferred");
    let InferredTypeData::Function(function) = expected else {
        panic!(
            "expected a callback type, got {}",
            format_inferred_type(&db, expected)
        );
    };
    let value_ty = function.parameters(&db)[0].ty();
    let InferredTypeData::Union(union) = value_ty else {
        panic!(
            "callback parameter must be `string | number | null`, got {}",
            format_inferred_type(&db, value_ty)
        );
    };
    let members = union.types(&db);
    assert!(
        members.contains(&InferredTypeData::Null),
        "{}",
        format_inferred_type(&db, value_ty)
    );
    assert!(members.iter().any(|ty| is_inferred_string(&db, *ty)));
    assert!(members.iter().any(|ty| is_inferred_number(&db, *ty)));
}

#[test]
fn test_infer_call_argument_type_accepts_callable_type_arguments_within_generic_constraints() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function pick<T extends () => unknown>(
                factory: T,
                callback: (value: T | null) => void,
            ): void;
            export declare function pick<T>(factory: T, callback: (value: T) => void): void;

            export declare const makeNumber: () => Promise<number>;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let overloads = inferred_overload_ty_by_name(&db, module, inferred, "pick")
        .expect("pick overload set must be inferred");
    let make_number = inferred_binding_ty_by_name(&db, module, inferred, "makeNumber")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("makeNumber binding type must be inferred");

    // `pick<() => Promise<number>>(makeNumber, cb)`: the constraint's
    // `unknown` return accepts a Promise, so the callback comes from the
    // first overload and accepts `null`.
    let callee = InferredTypeData::instance_of(&db, overloads, Box::from([make_number]));
    let input = CallArgumentTypeInput::new(
        &db,
        callee,
        Vec::from([
            InferredCallArgumentType::Argument(make_number),
            InferredCallArgumentType::Argument(InferredTypeData::Unknown),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("callback expected type must be inferred");
    let InferredTypeData::Function(function) = expected else {
        panic!(
            "expected a callback type, got {}",
            format_inferred_type(&db, expected)
        );
    };
    let value_ty = function.parameters(&db)[0].ty();
    let InferredTypeData::Union(union) = value_ty else {
        panic!(
            "callback parameter must be `(() => Promise<number>) | null`, got {}",
            format_inferred_type(&db, value_ty)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(union.types(&db).iter().any(|ty| {
        ty.callable_function(&db)
            .is_some_and(|function| function.returns_promise(&db))
    }));
}

#[test]
fn test_infer_call_argument_type_uses_instantiated_generic_alias_call_signatures() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Handlers<T> = {
                (value: T, callback: (value: T) => void): void;
                (value: T | null, callback: (value: T | null) => void): void;
            };
            export declare const handlers: Handlers<string>;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let handlers = inferred_binding_ty_by_name(&db, module, inferred, "handlers")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("handlers binding type must be inferred");

    // `handlers(null, cb)`: the alias's `string` binds `T` before overloads
    // are tested, and is not treated as a call-site type argument.
    let input = CallArgumentTypeInput::new(
        &db,
        handlers,
        Vec::from([
            InferredCallArgumentType::Argument(InferredTypeData::Null),
            InferredCallArgumentType::Argument(InferredTypeData::Unknown),
        ])
        .into_boxed_slice(),
        1,
    );
    let expected =
        infer_call_argument_type(&db, input).expect("callback expected type must be inferred");
    let InferredTypeData::Function(function) = expected else {
        panic!(
            "expected a callback type, got {}",
            format_inferred_type(&db, expected)
        );
    };
    let value_ty = function.parameters(&db)[0].ty();
    let InferredTypeData::Union(union) = value_ty else {
        panic!(
            "callback parameter must be `string | null`, got {}",
            format_inferred_type(&db, value_ty)
        );
    };
    assert!(union.types(&db).contains(&InferredTypeData::Null));
    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| is_inferred_string(&db, *ty))
    );
}

#[test]
fn test_infer_call_argument_type_keeps_strict_callback_return_types_for_async_callbacks() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export declare function voided(
                factory: () => string | void,
                callback: (value: "voided") => void,
            ): void;
            export declare function voided(
                factory: () => Promise<string>,
                callback: (value: "promise") => void,
            ): void;

            export declare function constrained<T extends string>(
                factory: () => T,
                callback: (value: "constrained") => void,
            ): void;
            export declare function constrained(
                factory: () => Promise<string>,
                callback: (value: "promise") => void,
            ): void;

            export declare const asyncFactory: () => Promise<string>;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let async_factory = inferred_binding_ty_by_name(&db, module, inferred, "asyncFactory")
        .map(|ty| inferred.resolve_type(&db, ty))
        .expect("asyncFactory binding type must be inferred");
    let callback_parameter = |name: &str| {
        let overloads = inferred_overload_ty_by_name(&db, module, inferred, name)
            .unwrap_or_else(|| panic!("{name} overload set must be inferred"));
        let input = CallArgumentTypeInput::new(
            &db,
            overloads,
            Vec::from([
                InferredCallArgumentType::Argument(async_factory),
                InferredCallArgumentType::Argument(InferredTypeData::Unknown),
            ])
            .into_boxed_slice(),
            1,
        );
        let expected = infer_call_argument_type(&db, input)
            .unwrap_or_else(|| panic!("{name} callback expected type must be inferred"));
        let InferredTypeData::Function(function) = expected else {
            panic!(
                "{name} must expect a callback, got {}",
                format_inferred_type(&db, expected)
            );
        };
        function.parameters(&db)[0].ty()
    };

    // Neither `string | void` nor `T extends string` accepts a Promise, so
    // the callback comes from the Promise overload in both cases.
    assert!(is_inferred_string_literal(
        &db,
        callback_parameter("voided"),
        "promise"
    ));
    assert!(is_inferred_string_literal(
        &db,
        callback_parameter("constrained"),
        "promise"
    ));
}
