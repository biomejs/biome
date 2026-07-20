use super::*;
use biome_js_type_info::interned_types::{
    InternedFunction as InferredFunction, InternedTuple as InferredTuple,
    NamedFunctionParameter as InferredNamedFunctionParameter,
    TupleElementType as InferredTupleElementType,
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
    InferredTypeData::Tuple(InferredTuple::new(db, elements.into_boxed_slice()))
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
fn test_infer_constructor_argument_type_resolves_canonical_global() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "");

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let input = CallArgumentTypeInput::new(
        &db,
        InferredTypeData::promise_class(&db),
        Vec::from([InferredCallArgumentType::Argument(
            InferredTypeData::Unknown,
        )])
        .into_boxed_slice(),
        0,
    );
    let expected = infer_constructor_argument_type(&db, input)
        .expect("Promise executor expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void-returning Promise executor, got {expected:?}"
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
