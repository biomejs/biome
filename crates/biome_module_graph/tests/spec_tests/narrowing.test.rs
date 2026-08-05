use super::*;
use biome_rowan::TextSize;

#[test]
fn test_infer_module_types_narrows_typeof_guarded_references() {
    const SOURCE: &str = r#"
export function guarded(x: number | (() => Promise<void>)) {
    if (typeof x === "function") {
        x;
    }
    x;
}

export function reversed(y: string | undefined) {
    if (typeof y === "undefined") {
        y;
    }
    if ("string" == typeof y) {
        y;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    // Inside the consequent, `x` is narrowed to the callable variant.
    let narrowed_offset = SOURCE.find("x;").expect("guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(narrowed_offset));
    assert!(narrowed.callable_function(&db).is_some());
    assert!(!contains_inferred_number(&db, narrowed));

    // After the `if` statement, `x` still has its declared union type.
    let unnarrowed_offset = SOURCE.rfind("x;").expect("trailing reference must exist");
    let unnarrowed = normalize_type(&db, module, expression_ty_at(unnarrowed_offset));
    assert!(contains_inferred_number(&db, unnarrowed));

    // An `undefined` guard selects the `undefined` variant.
    let undefined_offset = SOURCE
        .find("y;")
        .expect("undefined-guarded reference must exist");
    let narrowed_to_undefined = normalize_type(&db, module, expression_ty_at(undefined_offset));
    assert!(contains_inferred_undefined(&db, narrowed_to_undefined));
    assert!(!contains_inferred_string(&db, narrowed_to_undefined));

    // Reversed operands and `==` are recognized as well.
    let string_offset = SOURCE
        .rfind("y;")
        .expect("string-guarded reference must exist");
    let narrowed_to_string = normalize_type(&db, module, expression_ty_at(string_offset));
    assert!(contains_inferred_string(&db, narrowed_to_string));
    assert!(!contains_inferred_undefined(&db, narrowed_to_string));
}

#[test]
fn test_infer_module_types_narrows_typeof_guards_with_callable_interfaces() {
    const SOURCE: &str = r#"
interface AsyncFn {
    (): Promise<void>;
    tag: string;
}

export function guarded(f: AsyncFn | (() => void) | null) {
    if (typeof f === "function") {
        f;
    }
    if (typeof f === "object") {
        f;
    }
}

class Service {
    run(): void {}
}

export function classValue(c: typeof Service | number) {
    if (typeof c === "function") {
        c;
    }
}
"#;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), SOURCE);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let expression_ty_at = |offset: usize| {
        let start = TextSize::from(offset as u32);
        let range = TextRange::new(start, start + TextSize::from(1));
        inferred
            .expressions
            .get(&range)
            .copied()
            .expect("reference type must be inferred")
    };

    // A callable interface matches the `function` tag, so the guard retains
    // it alongside the plain function and only strips `null`.
    let function_offset = SOURCE
        .find("f;")
        .expect("function-guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(function_offset));
    let formatted = format_inferred_type(&db, narrowed);
    assert!(formatted.contains("interface \"AsyncFn\""), "{formatted}");
    assert!(formatted.contains("Function"), "{formatted}");
    assert!(!formatted.contains("null"), "{formatted}");

    // Under the `object` tag, both callable variants are stripped, while
    // `null` is retained because `typeof null` is `"object"`.
    let object_offset = SOURCE
        .rfind("f;")
        .expect("object-guarded reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(object_offset));
    let formatted = format_inferred_type(&db, narrowed);
    assert!(formatted.contains("null"), "{formatted}");
    assert!(!formatted.contains("AsyncFn"), "{formatted}");
    assert!(!formatted.contains("Function"), "{formatted}");

    // A class value is a constructor function at runtime, so the `function`
    // tag retains it and strips the number.
    let class_offset = SOURCE.find("c;").expect("class reference must exist");
    let narrowed = normalize_type(&db, module, expression_ty_at(class_offset));
    assert!(!contains_inferred_number(&db, narrowed));
    let formatted = format_inferred_type(&db, narrowed);
    assert!(formatted.contains("Service"), "{formatted}");
}
