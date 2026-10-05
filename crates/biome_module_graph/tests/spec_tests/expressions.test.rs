use super::*;

#[test]
fn test_infer_module_types_evaluates_typeof_operator_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const value = 1;
            export function readValue(): string {
                return "value";
            }

            export const valueType = typeof value;
            export const functionType = typeof readValue;
            export const unknownType = typeof notDeclared;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let value_type_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "valueType")
        .expect("valueType binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, value_type_ty),
        "number"
    ));

    let function_type_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "functionType")
        .expect("functionType binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, function_type_ty),
        "function"
    ));

    let unknown_type_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "unknownType")
        .expect("unknownType binding type must be inferred");
    let unknown_type_ty = inferred.resolve_type(&db, unknown_type_ty);
    let InferredTypeData::Union(union) = unknown_type_ty else {
        panic!("unknown typeof result must be inferred as a union, got {unknown_type_ty:?}");
    };
    assert_eq!(union.types(&db).len(), 8);
    for value in [
        "bigint",
        "boolean",
        "function",
        "number",
        "object",
        "string",
        "symbol",
        "undefined",
    ] {
        assert!(contains_inferred_string_literal(
            &db,
            unknown_type_ty,
            value
        ));
    }

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_typeof_operator_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_arithmetic_unary_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const numeric = 1 + 2;
            export const textual = "value" + numeric;
            export const unknown = notDeclared + alsoMissing;
            export const negative = -numeric;
            export const inverted = ~numeric;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let numeric_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "numeric")
        .expect("numeric binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, numeric_ty)
    ));

    let textual_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "textual")
        .expect("textual binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, textual_ty)
    ));

    let unknown_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "unknown")
        .expect("unknown binding type must be inferred");
    assert_eq!(
        inferred.resolve_type(&db, unknown_ty),
        InferredTypeData::Unknown
    );

    let negative_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "negative")
        .expect("negative binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, negative_ty)
    ));

    let inverted_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "inverted")
        .expect("inverted binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, inverted_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_arithmetic_unary_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_array_element_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const tuple = ["value", 1];
            export const tupleFirst = tuple[0];
            export const numbers: number[] = [1, 2];
            export const arrayFirst = numbers[0];
            export const [destructured] = tuple;

            for (const item of numbers) {
                item;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let tuple_first_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "tupleFirst")
        .expect("tupleFirst binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, tuple_first_ty)
    ));

    let array_first_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "arrayFirst")
        .expect("arrayFirst binding type must be inferred");
    let array_first_ty = inferred.resolve_type(&db, array_first_ty);
    assert!(contains_inferred_number(&db, array_first_ty));
    assert!(contains_inferred_undefined(&db, array_first_ty));

    let destructured_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "destructured")
        .expect("destructured binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, destructured_ty)
    ));

    let item_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "item")
        .expect("item binding type must be inferred");
    assert!(is_inferred_number(&db, inferred.resolve_type(&db, item_ty)));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_array_element_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_static_member_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Box {
                static label: string;
                value: number;
                optional?: string;
            }

            export const object = { name: "value" };
            export const objectName = object.name;
            export const staticLabel = Box.label;
            export const box: Box = {} as Box;
            export const memberValue = box.value;
            export const optionalValue = box.optional;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let object_name_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "objectName")
        .expect("objectName binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, object_name_ty)
    ));

    let static_label_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "staticLabel")
        .expect("staticLabel binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, static_label_ty)
    ));

    let member_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "memberValue")
        .expect("memberValue binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, member_value_ty)
    ));

    let optional_value_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "optionalValue")
            .expect("optionalValue binding type must be inferred");
    let optional_value_ty = inferred.resolve_type(&db, optional_value_ty);
    assert!(contains_inferred_string(&db, optional_value_ty));
    assert!(contains_inferred_undefined(&db, optional_value_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_static_member_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_call_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readValue(): string {
                return "value";
            }

            export function identity<T>(input: T): T {
                return input;
            }

            export const text = readValue();
            export const numeric = identity(1);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "text")
        .expect("text binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, value_ty)
    ));

    let numeric_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "numeric")
        .expect("numeric binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, numeric_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_call_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_coerces_tuples_in_additions() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const values = [1, 2] as const;
            export const text = values + "!";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let text_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "text")
        .expect("text binding type must be inferred");
    assert!(is_inferred_string(&db, inferred.resolve_type(&db, text_ty)));

    assert_inferred_type_snapshot(
        "test_infer_module_types_coerces_tuples_in_additions",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_new_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Box {
                value: string;

                constructor(contents: string) {}
            }

            export class Empty {
                count: number;
            }

            export const box = new Box("value");
            export const boxValue = box.value;
            export const empty = new Empty();
            export const count = empty.count;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let box_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "box")
        .expect("box binding type must be inferred");
    assert!(matches!(
        inferred.resolve_type(&db, box_ty),
        InferredTypeData::InstanceOf(_)
    ));

    let value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "boxValue")
        .expect("boxValue binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, value_ty)
    ));

    let count_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "count")
        .expect("count binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, count_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_new_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_preserves_new_expression_generic_instances_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Box<T> {
                value: T;

                constructor(value: T) {
                    this.value = value;
                }
            }

            export const explicit = new Box<string>("value");
            export const explicitValue = explicit.value;

            export const inferred = new Box(1);
            export const inferredValue = inferred.value;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let explicit_value_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "explicitValue")
            .expect("explicitValue binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, explicit_value_ty)
    ));

    let inferred_value_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "inferredValue")
            .expect("inferredValue binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, inferred_value_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_preserves_new_expression_generic_instances_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_infers_new_expression_nested_generic_instances_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Box<T> {
                value: T;

                constructor(value: T) {
                    this.value = value;
                }
            }

            export class ArrayBox<T> {
                value: T;

                constructor(values: Array<T>) {
                    this.value = values[0] as T;
                }
            }

            export class CallbackBox<T> {
                value: T;

                constructor(read: () => T) {
                    this.value = read();
                }
            }

            export const directValue = new Box("text").value;

            export const values: Array<number> = [1];
            export const arrayValue = new ArrayBox(values).value;

            export const callbackValue = new CallbackBox(() => true).value;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let direct_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "directValue")
        .expect("directValue binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, direct_value_ty)
    ));

    let array_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "arrayValue")
        .expect("arrayValue binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, array_value_ty)
    ));

    let callback_value_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "callbackValue")
            .expect("callbackValue binding type must be inferred");
    assert!(is_inferred_boolean(
        &db,
        inferred.resolve_type(&db, callback_value_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_infers_new_expression_nested_generic_instances_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_conditional_logical_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export let flag: boolean = true;
            export let maybeText: string | undefined = "value";

            export const choice = flag ? "value" : 1;
            export const andValue = maybeText && 1;
            export const orValue = maybeText || 1;
            export const nullishValue = maybeText ?? 1;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let choice_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "choice")
        .expect("choice binding type must be inferred");
    let choice_ty = inferred.resolve_type(&db, choice_ty);
    assert!(
        contains_inferred_string(&db, choice_ty),
        "choice must contain string, got {}",
        format_inferred_type(&db, choice_ty)
    );
    assert!(
        contains_inferred_number(&db, choice_ty),
        "choice must contain number, got {}",
        format_inferred_type(&db, choice_ty)
    );

    let and_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "andValue")
        .expect("andValue binding type must be inferred");
    let and_value_ty = inferred.resolve_type(&db, and_value_ty);
    assert!(contains_inferred_string_literal(&db, and_value_ty, ""));
    assert!(contains_inferred_number(&db, and_value_ty));
    assert!(contains_inferred_undefined(&db, and_value_ty));

    let or_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "orValue")
        .expect("orValue binding type must be inferred");
    let or_value_ty = inferred.resolve_type(&db, or_value_ty);
    assert!(contains_inferred_string(&db, or_value_ty));
    assert!(contains_inferred_number(&db, or_value_ty));
    assert!(!contains_inferred_undefined(&db, or_value_ty));

    let nullish_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "nullishValue")
        .expect("nullishValue binding type must be inferred");
    let nullish_value_ty = inferred.resolve_type(&db, nullish_value_ty);
    assert!(contains_inferred_string(&db, nullish_value_ty));
    assert!(contains_inferred_number(&db, nullish_value_ty));
    assert!(!contains_inferred_undefined(&db, nullish_value_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_conditional_logical_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_conditional_logical_fast_paths_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const truthyAnd = "yes" && 1;
            export const falsyAnd = 0 && missing;
            export const nullish = null ?? 1;
            export const nonNullish = "s" ?? missing;
            export const conditional = true ? "yes" : 1;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let truthy_and_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "truthyAnd")
        .expect("truthyAnd binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, truthy_and_ty)
    ));

    let falsy_and_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "falsyAnd")
        .expect("falsyAnd binding type must be inferred");
    assert!(is_inferred_number_literal(
        &db,
        inferred.resolve_type(&db, falsy_and_ty),
        "0"
    ));

    let nullish_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "nullish")
        .expect("nullish binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, nullish_ty)
    ));

    let non_nullish_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "nonNullish")
        .expect("nonNullish binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, non_nullish_ty),
        "s"
    ));

    let conditional_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "conditional")
        .expect("conditional binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, conditional_ty),
        "yes"
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_conditional_logical_fast_paths_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_filters_conditional_subsets_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export let maybeText: string | null = "value";
            export let zeroOrText: string | 0 = "value";
            export class Box {}
            export let maybeBox: Box | null = new Box();

            export const nullishResult = maybeText ?? "fallback";
            export const andResult = zeroOrText && true;
            export const nullishBox = maybeBox ?? "fallback";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let nullish_result_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "nullishResult")
            .expect("nullishResult binding type must be inferred");
    let nullish_result_ty = inferred.resolve_type(&db, nullish_result_ty);
    assert!(contains_inferred_string(&db, nullish_result_ty));
    assert!(!contains_inferred_null(&db, nullish_result_ty));

    let and_result_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "andResult")
        .expect("andResult binding type must be inferred");
    let and_result_ty = inferred.resolve_type(&db, and_result_ty);
    assert!(contains_inferred_number_literal(&db, and_result_ty, "0"));
    assert!(contains_inferred_boolean(&db, and_result_ty));

    let nullish_box_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "nullishBox")
        .expect("nullishBox binding type must be inferred");
    let nullish_box_ty = inferred.resolve_type(&db, nullish_box_ty);
    assert!(contains_inferred_instance(&db, nullish_box_ty));
    assert!(contains_inferred_string(&db, nullish_box_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_filters_conditional_subsets_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_this_and_super_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Base {
                label: string;
            }

            export class Derived extends Base {
                value: number;

                read() {
                    const own = this.value;
                    const inherited = super.label;
                    return own;
                }
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let own_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "own")
        .expect("own binding type must be inferred");
    assert!(is_inferred_number(&db, inferred.resolve_type(&db, own_ty)));

    let inherited_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "inherited")
        .expect("inherited binding type must be inferred");
    let inherited_ty = inferred.resolve_type(&db, inherited_ty);
    assert!(
        is_inferred_string(&db, inherited_ty),
        "inherited must be string, got {}",
        format_inferred_type(&db, inherited_ty)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_this_and_super_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_destructuring_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const object: { a: string; b?: number; c: boolean } = {
                a: "value",
                c: true,
            };
            export const { a, b, ...rest } = object;

            export const numbers: number[] = [1, 2];
            export const [head, ...tail] = numbers;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let a_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "a")
        .expect("a binding type must be inferred");
    assert!(is_inferred_string(&db, inferred.resolve_type(&db, a_ty)));

    let b_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "b")
        .expect("b binding type must be inferred");
    let b_ty = inferred.resolve_type(&db, b_ty);
    assert!(contains_inferred_number(&db, b_ty));
    assert!(contains_inferred_undefined(&db, b_ty));

    let rest_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "rest")
        .expect("rest binding type must be inferred");
    let rest_ty = inferred.resolve_type(&db, rest_ty);
    assert!(object_member_ty_by_name(&db, rest_ty, "a").is_none());
    assert!(object_member_ty_by_name(&db, rest_ty, "b").is_none());
    let (_, rest_c_ty) =
        object_member_ty_by_name(&db, rest_ty, "c").expect("rest must retain the c member");
    assert!(contains_inferred_boolean(&db, rest_c_ty));

    let head_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "head")
        .expect("head binding type must be inferred");
    let head_ty = inferred.resolve_type(&db, head_ty);
    assert!(contains_inferred_number(&db, head_ty));
    assert!(contains_inferred_undefined(&db, head_ty));

    let tail_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "tail")
        .expect("tail binding type must be inferred");
    let tail_ty = inferred.resolve_type(&db, tail_ty);
    let InferredTypeData::InstanceOf(tail_instance) = tail_ty else {
        panic!("tail must be inferred as an array instance, got {tail_ty:?}");
    };
    assert!(tail_instance.ty(&db).is_array_class(&db));
    assert_eq!(tail_instance.type_parameters(&db).len(), 1);
    assert!(is_inferred_number(
        &db,
        tail_instance.type_parameters(&db)[0]
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_destructuring_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_destructuring_edge_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Base {
                inherited: number;
            }

            export class Derived extends Base {
                static a: number;
                static keep: boolean;
                own: string;
            }

            export const { a, ...staticRest } = Derived;

            export const derived: Derived = {} as Derived;
            export const { inherited, ...instanceRest } = derived;

            export const tuple = ["value", 1, true];
            export const [first, ...tupleRest] = tuple;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let a_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "a")
        .expect("a binding type must be inferred");
    assert!(is_inferred_number(&db, inferred.resolve_type(&db, a_ty)));

    let static_rest_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "staticRest")
        .expect("staticRest binding type must be inferred");
    let static_rest_ty = inferred.resolve_type(&db, static_rest_ty);
    assert!(object_member_ty_by_name(&db, static_rest_ty, "a").is_none());
    let (_, keep_ty) = object_member_ty_by_name(&db, static_rest_ty, "keep")
        .expect("static rest must retain keep");
    assert!(contains_inferred_boolean(&db, keep_ty));

    let inherited_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "inherited")
        .expect("inherited binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, inherited_ty)
    ));

    let instance_rest_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "instanceRest")
        .expect("instanceRest binding type must be inferred");
    let instance_rest_ty = inferred.resolve_type(&db, instance_rest_ty);
    assert!(object_member_ty_by_name(&db, instance_rest_ty, "inherited").is_none());
    let (_, own_ty) = object_member_ty_by_name(&db, instance_rest_ty, "own")
        .expect("instance rest must retain own");
    assert!(contains_inferred_string(&db, own_ty));

    let tuple_rest_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "tupleRest")
        .expect("tupleRest binding type must be inferred");
    let tuple_rest_ty = inferred.resolve_type(&db, tuple_rest_ty);
    let InferredTypeData::Tuple(tuple) = tuple_rest_ty else {
        panic!("tupleRest must be inferred as a tuple, got {tuple_rest_ty:?}");
    };
    assert_eq!(tuple.elements(&db).len(), 2);
    assert!(is_inferred_number(&db, tuple.elements(&db)[0].ty));
    assert!(is_inferred_boolean(&db, tuple.elements(&db)[1].ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_destructuring_edge_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_this_and_super_edge_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Box {
                value: number;

                read() {
                    const arrow = () => {
                        const arrowValue = this.value;
                        return arrowValue;
                    };

                    function nested() {
                        const plainValue = this.value;
                        return plainValue;
                    }

                    return arrow();
                }
            }

            export class Solo {
                read() {
                    const noParent = super.value;
                    return noParent;
                }
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let arrow_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "arrowValue")
        .expect("arrowValue binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, arrow_value_ty)
    ));

    let plain_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "plainValue")
        .expect("plainValue binding type must be inferred");
    assert_eq!(
        inferred.resolve_type(&db, plain_value_ty),
        InferredTypeData::Unknown
    );

    let no_parent_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "noParent")
        .expect("noParent binding type must be inferred");
    assert_eq!(
        inferred.resolve_type(&db, no_parent_ty),
        InferredTypeData::Unknown
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_this_and_super_edge_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_preserves_generic_class_this_parameters() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            class Box<T> {
                constructor(readonly value: T) {}
                read() { return this.value; }
            }

            export const direct = new Box(Promise.resolve("value")).value;
            export const method = new Box(Promise.resolve("value")).read;
            export const result = method();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_inferred_type_snapshot(
        "test_infer_module_types_preserves_generic_class_this_parameters",
        &db,
        &fs,
    );

    for name in ["direct", "result"] {
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name)
            .expect("binding type must be inferred");
        let ty = normalize_type(&db, module, ty);
        assert!(
            is_inferred_promise_instance(&db, ty),
            "{name} must be a Promise, got {}",
            format_inferred_type(&db, ty)
        );
    }

    let method = inferred_binding_ty_by_name(&db, module, inferred, "method")
        .expect("method binding type must be inferred");
    let method = normalize_type(&db, module, method);
    assert!(matches!(
        method,
        InferredTypeData::Function(function)
            if matches!(function.return_type(&db), InferredReturnType::Type(ty) if is_inferred_promise_instance(&db, *ty))
    ));
}

#[test]
fn test_infer_module_types_resolves_inherited_static_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            class Base {
                static read(): Promise<void> {
                    return Promise.resolve();
                }
            }
            class Derived extends Base {}
            export const result = Derived.read();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_inherited_static_members",
        &db,
        &fs,
    );
    let result = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");
    assert!(is_inferred_promise_instance(
        &db,
        inferred.resolve_type(&db, result)
    ));
}
