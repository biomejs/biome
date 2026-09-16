use super::*;

use biome_js_type_info::TypeOperator;
use biome_js_type_info::resolved::InferredLiteralValue as InferredLiteral;
use biome_module_graph::{BindingTypeInput, infer_binding_type};
use biome_rowan::TextSize;

fn normalized_function_parameter_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    inferred: &InferredModuleTypes<'db>,
    function_name: &str,
) -> InferredTypeData<'db> {
    let function_ty = inferred_binding_ty_by_name(db, module, inferred, function_name)
        .unwrap_or_else(|| panic!("{function_name} binding type must be inferred"));
    let function = inferred
        .resolve_type(db, function_ty)
        .callable_function(db)
        .unwrap_or_else(|| panic!("{function_name} must be inferred as a function"));
    let parameter = function
        .parameters(db)
        .first()
        .unwrap_or_else(|| panic!("{function_name} must have a parameter"));

    normalize_type(db, module, parameter.ty())
}

fn normalized_binding_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    inferred: &InferredModuleTypes<'db>,
    binding_name: &str,
) -> InferredTypeData<'db> {
    let ty = inferred_binding_ty_by_name(db, module, inferred, binding_name)
        .unwrap_or_else(|| panic!("{binding_name} binding type must be inferred"));
    normalize_type(db, module, inferred.resolve_type(db, ty))
}

fn assert_indeterminate_keyof<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>, name: &str) {
    let mut leaf = ty;
    for _ in 0..16 {
        match leaf {
            InferredTypeData::Unknown => return,
            InferredTypeData::TypeOperator(operator)
                if operator.operator(db) == TypeOperator::Keyof =>
            {
                return;
            }
            InferredTypeData::InstanceOf(instance) => {
                leaf = instance.ty(db);
            }
            _ => break,
        }
    }
    panic!("{name}: incomplete keyof result must remain indeterminate, got {ty:?}");
}

fn key_variant_labels<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> Vec<String> {
    let mut labels = match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .flat_map(|ty| key_variant_labels(db, *ty))
            .collect(),
        InferredTypeData::Literal(literal) => match literal.literal(db) {
            InferredLiteral::String(value) => vec![format!("string:{}", value.as_str())],
            InferredLiteral::Number(value) => vec![format!(
                "number:{}",
                value
                    .to_property_key()
                    .expect("numeric literal must have a property key")
            )],
            value => panic!("unexpected literal key variant: {value:?}"),
        },
        InferredTypeData::Number => vec!["number".into()],
        InferredTypeData::String => vec!["string".into()],
        InferredTypeData::Symbol => vec!["symbol".into()],
        other => panic!("unexpected keyof result: {other:?}"),
    };
    labels.sort();
    labels.dedup();
    labels
}

fn assert_key_variants<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>, expected: &[&str]) {
    assert_eq!(
        key_variant_labels(db, ty),
        expected
            .iter()
            .map(|key| (*key).into())
            .collect::<Vec<String>>()
    );
}

fn assert_numeric_literal<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>, expected: &str) {
    let InferredTypeData::Literal(literal) = ty else {
        panic!("expected numeric literal {expected}, got {ty:?}");
    };
    let InferredLiteral::Number(value) = literal.literal(db) else {
        panic!("expected numeric literal {expected}, got {literal:?}");
    };
    assert_eq!(value.as_str(), expected);
}

fn assert_broad_value_variants<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    expected: &[&str],
) {
    let InferredTypeData::Union(union) = ty else {
        panic!("expected a broad value union, got {ty:?}");
    };
    let variants = union.types(db);
    assert_eq!(variants.len(), expected.len());
    for expected in expected {
        assert!(
            variants.iter().any(|ty| match *expected {
                "boolean" => is_inferred_boolean(db, *ty),
                "number" => is_inferred_number(db, *ty),
                "string" => is_inferred_string(db, *ty),
                _ => panic!("unexpected broad value variant {expected}"),
            }),
            "missing broad value variant {expected} in {variants:?}"
        );
    }
}

#[test]
fn test_normalize_type_evaluates_keyof_shapes() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Fleet = {
                starDestroyers: number;
                tieFighters: number;
                shuttles: number;
            };
            type Identity<T> = T;
            const fleetValue = { starDestroyers: 1, tieFighters: 2 };

            type Common = keyof (
                { left: number; shared: string }
                | { right: number; shared: string }
            );
            type Combined = keyof ({ left: number } & { right: number });

            interface Base { inherited: number }
            interface Derived extends Base { own?: string }

            type Recursive = { next?: Recursive; value: number };
            type Numeric = { 1: unknown };
            type Quoted = { "1": unknown };
            type EscapedIdentifier = { \u0061: unknown };
            type Escaped = {
                "a\"b": unknown;
                "a\\b": unknown;
                "a\nb": unknown;
            };
            type NumberIndexed = { [key: number]: unknown };
            type StringIndexed = { [key: string]: unknown };
            type SymbolIndexed = { [key: symbol]: unknown };

            export function fleet(key: keyof Fleet): keyof Fleet { return key; }
            export function identity(key: keyof Identity<Fleet>): keyof Identity<Fleet> { return key; }
            export function valueKeys(key: keyof typeof fleetValue) { return key; }
            export function common(key: Common): Common { return key; }
            export function combined(key: Combined): Combined { return key; }
            export function derived(key: keyof Derived): keyof Derived { return key; }
            export function recursive(key: keyof Recursive): keyof Recursive { return key; }
            export function numeric(key: keyof Numeric): keyof Numeric { return key; }
            export function quoted(key: keyof Quoted): keyof Quoted { return key; }
            export function escapedIdentifier(
                key: keyof EscapedIdentifier,
            ): keyof EscapedIdentifier {
                return key;
            }
            export function escaped(key: keyof Escaped): keyof Escaped { return key; }
            export function numberIndex(key: keyof NumberIndexed): keyof NumberIndexed { return key; }
            export function stringIndex(key: keyof StringIndexed): keyof StringIndexed { return key; }
            export function symbolIndex(key: keyof SymbolIndexed): keyof SymbolIndexed { return key; }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "fleet"),
        &[
            "string:shuttles",
            "string:starDestroyers",
            "string:tieFighters",
        ],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "identity"),
        &[
            "string:shuttles",
            "string:starDestroyers",
            "string:tieFighters",
        ],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "valueKeys"),
        &["string:starDestroyers", "string:tieFighters"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "common"),
        &["string:shared"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "combined"),
        &["string:left", "string:right"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "derived"),
        &["string:inherited", "string:own"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "recursive"),
        &["string:next", "string:value"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "numeric"),
        &["number:1"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "quoted"),
        &["string:1"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "escapedIdentifier"),
        &["string:a"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "escaped"),
        &["string:a\\\"b", "string:a\\\\b", "string:a\\nb"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "numberIndex"),
        &["number"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "stringIndex"),
        &["number", "string"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "symbolIndex"),
        &["symbol"],
    );
}

#[test]
fn test_identifier_escapes_preserve_key_and_value_lookup() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            const escapedObject = { \u0061: 1, "\\u0061": 2 };
            const \u0063 = 3;
            const shorthandObject = { \u0063 };

            class EscapedConstructor {
                constructor(public \u0062: number) {}
            }
            const escapedConstructor = new EscapedConstructor(4);

            class StaticComputed {
                static ["known"] = 1;
                static [1]() {
                    return "numeric";
                }
                static [(`template`)] = 3;
            }
            declare function dynamicStaticKey(): string;
            class DynamicStatic {
                static [dynamicStaticKey()] = 0;
            }
            declare function tag(value: TemplateStringsArray): string;
            class TaggedStatic {
                static [(tag`known`)] = 1;
            }

            export function objectKeys(
                key: keyof typeof escapedObject,
            ): keyof typeof escapedObject {
                return key;
            }
            export function shorthandKeys(
                key: keyof typeof shorthandObject,
            ): keyof typeof shorthandObject {
                return key;
            }
            export function constructorKeys(
                key: keyof EscapedConstructor,
            ): keyof EscapedConstructor {
                return key;
            }
            export function staticComputedKeys(
                key: keyof typeof StaticComputed,
            ): keyof typeof StaticComputed {
                return key;
            }
            export function dynamicStaticKeys(
                key: keyof typeof DynamicStatic,
            ): keyof typeof DynamicStatic {
                return key;
            }
            export function taggedStaticKeys(
                key: keyof typeof TaggedStatic,
            ): keyof typeof TaggedStatic {
                return key;
            }

            export const escapedDot = escapedObject.\u0061;
            export const ordinaryDot = escapedObject.a;
            export const backslashKey = escapedObject["\\u0061"];
            export const shorthandValue = shorthandObject.c;
            export const constructorValue = escapedConstructor.b;
            export const staticKnownValue = StaticComputed["known"];
            export const staticNumericMethodValue = StaticComputed["1"]();
            export const staticTemplateValue = StaticComputed["template"];
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "objectKeys"),
        &["string:\\\\u0061", "string:a"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "shorthandKeys"),
        &["string:c"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "constructorKeys"),
        &["string:b"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "staticComputedKeys"),
        &[
            "number:1",
            "string:known",
            "string:prototype",
            "string:template",
        ],
    );
    assert_indeterminate_keyof(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "dynamicStaticKeys"),
        "dynamicStaticKeys",
    );
    assert_indeterminate_keyof(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "taggedStaticKeys"),
        "taggedStaticKeys",
    );
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "escapedDot"),
        "1",
    );
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "ordinaryDot"),
        "1",
    );
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "backslashKey"),
        "2",
    );
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "shorthandValue"),
        "3",
    );
    assert_eq!(
        normalized_binding_type(&db, module, inferred, "constructorValue"),
        InferredTypeData::Number
    );
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "staticKnownValue"),
        "1",
    );
    assert!(is_inferred_string(
        &db,
        normalized_binding_type(&db, module, inferred, "staticNumericMethodValue")
    ));
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "staticTemplateValue"),
        "3",
    );
}

#[test]
fn test_normalize_type_follows_keyof_aliases_and_imports() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/shapes.ts".into(),
        r#"
            export interface Fleet {
                starDestroyers: number;
                tieFighters: number;
            }

            export type FleetKeys = keyof Fleet;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Fleet, FleetKeys } from "./shapes.ts";

            type LocalFleet = Fleet;

            export function direct(key: keyof LocalFleet): keyof LocalFleet { return key; }
            export function aliased(key: FleetKeys): FleetKeys { return key; }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/shapes.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for function_name in ["direct", "aliased"] {
        assert_key_variants(
            &db,
            normalized_function_parameter_type(&db, module, inferred, function_name),
            &["string:starDestroyers", "string:tieFighters"],
        );
    }
}

#[test]
fn test_normalize_type_evaluates_keyof_domains_and_computed_keys() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function anyKey(key: keyof any): keyof any { return key; }
            export function neverKey(key: keyof never): keyof never { return key; }
            export function unknownKey(key: keyof unknown): keyof unknown { return key; }

            type Computed = { ["computed"]: unknown; [2]: unknown };
            export function computed(key: keyof Computed): keyof Computed { return key; }

            class Ship {
                static registry: number;
                name: string;
                constructor() {}
                static {}
            }
            const shipInstance = new Ship();
            export function instance(key: keyof Ship): keyof Ship { return key; }
            export function staticSide(key: keyof typeof Ship): keyof typeof Ship { return key; }
            export function instanceValue(key: keyof typeof shipInstance) { return key; }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for function_name in ["anyKey", "neverKey"] {
        assert_key_variants(
            &db,
            normalized_function_parameter_type(&db, module, inferred, function_name),
            &["number", "string", "symbol"],
        );
    }
    assert_eq!(
        normalized_function_parameter_type(&db, module, inferred, "unknownKey"),
        InferredTypeData::NeverKeyword
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "computed"),
        &["number:2", "string:computed"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "instance"),
        &["string:name"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "staticSide"),
        &["string:prototype", "string:registry"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "instanceValue"),
        &["string:name"],
    );
}

#[test]
fn test_normalize_type_preserves_aliased_any_and_never_domains() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export type AnyAlias = any;
            export type NestedAny = AnyAlias;
            export type NeverAlias = never;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { AnyAlias, NestedAny, NeverAlias } from "./types.ts";

            type Shape = { present: number };
            type LocalAny = any;
            type LocalNestedAny = LocalAny;

            export function localFirst(key: keyof (LocalAny | Shape)) { return key; }
            export function localSecond(key: keyof (Shape | LocalNestedAny)) { return key; }
            export function importedFirst(key: keyof (AnyAlias | Shape)) { return key; }
            export function importedSecond(key: keyof (Shape | NestedAny)) { return key; }
            export function intersection(key: keyof (AnyAlias & Shape)) { return key; }
            export function neverUnion(key: keyof (NeverAlias | Shape)) { return key; }
            export function neverIntersection(key: keyof (NeverAlias & Shape)) { return key; }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for function_name in [
        "localFirst",
        "localSecond",
        "importedFirst",
        "importedSecond",
        "intersection",
    ] {
        assert_key_variants(
            &db,
            normalized_function_parameter_type(&db, module, inferred, function_name),
            &["number", "string", "symbol"],
        );
    }
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "neverUnion"),
        &["string:present"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "neverIntersection"),
        &["number", "string", "symbol"],
    );
}

#[test]
fn test_static_getter_value_and_keyof_projection_remain_consistent() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            class Box {
                static get value(): number {
                    return 42;
                }
            }

            class NumericBox {
                static 1 = "known";
            }

            type NumericGetter = {
                get 1(): string;
            };
            declare const numericGetter: NumericGetter;
            const numericPropertyKeys = { [0.5]: 1, [1e-7]: 2, [1e21]: 3 };

            export function staticKeys(key: keyof typeof Box): keyof typeof Box {
                return key;
            }
            export function numericStaticKeys(
                key: keyof typeof NumericBox,
            ): keyof typeof NumericBox {
                return key;
            }

            export const staticValue = Box.value;
            export const staticNumericValue = NumericBox["1"];
            export const numericValue = numericGetter["1"];
            export const halfNumericValue = numericPropertyKeys["0.5"];
            export const tinyNumericValue = numericPropertyKeys["1e-7"];
            export const hugeNumericValue = numericPropertyKeys["1e+21"];
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "staticKeys"),
        &["string:prototype", "string:value"],
    );

    assert_eq!(
        normalized_binding_type(&db, module, inferred, "staticValue"),
        InferredTypeData::Number
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "numericStaticKeys"),
        &["number:1", "string:prototype"],
    );
    assert!(is_inferred_string(
        &db,
        normalized_binding_type(&db, module, inferred, "staticNumericValue")
    ));
    let numeric_value = normalized_binding_type(&db, module, inferred, "numericValue");
    let InferredTypeData::Union(numeric_value) = numeric_value else {
        panic!("numericValue must preserve getter optionality, got {numeric_value:?}");
    };
    assert_eq!(numeric_value.types(&db).len(), 2);
    assert!(
        numeric_value
            .types(&db)
            .contains(&InferredTypeData::Undefined)
    );
    let getter = numeric_value
        .types(&db)
        .iter()
        .find_map(|ty| match ty {
            InferredTypeData::Function(function) => Some(function),
            _ => None,
        })
        .expect("numericValue must preserve its getter function payload");
    let InferredReturnType::Type(return_type) = getter.return_type(&db) else {
        panic!("numeric getter payload must have a type return");
    };
    assert!(is_inferred_string(&db, *return_type));
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "halfNumericValue"),
        "1",
    );
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "tinyNumericValue"),
        "2",
    );
    assert_numeric_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "hugeNumericValue"),
        "3",
    );
}

#[test]
fn test_numeric_getter_keys_and_static_getter_lookup() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type NumericObject = {
                get 1(): string;
            };
            interface NumericInterface {
                get 2(): boolean;
            }
            class NumericClass {
                get 3(): boolean {
                    return true;
                }
                static get 4(): string {
                    return "numeric";
                }
                static get ["known"](): string {
                    return "known";
                }
            }
            declare function dynamicKey(): string;
            class DynamicClass {
                static get [dynamicKey()](): string {
                    return "dynamic";
                }
            }

            declare const numericObject: NumericObject;
            declare const numericInterface: NumericInterface;
            const numericClass = new NumericClass();

            export function objectKeys(key: keyof NumericObject): keyof NumericObject {
                return key;
            }
            export function interfaceKeys(
                key: keyof NumericInterface,
            ): keyof NumericInterface {
                return key;
            }
            export function instanceKeys(key: keyof NumericClass): keyof NumericClass {
                return key;
            }
            export function staticKeys(
                key: keyof typeof NumericClass,
            ): keyof typeof NumericClass {
                return key;
            }
            export function dynamicStaticKeys(
                key: keyof typeof DynamicClass,
            ): keyof typeof DynamicClass {
                return key;
            }

            export const staticNumericGetterValue = NumericClass["4"];
            export const staticKnownGetterValue = NumericClass["known"];
            export const objectNumericGetterValue = numericObject["1"];
            export const interfaceNumericGetterValue = numericInterface["2"];
            export const instanceNumericGetterValue = numericClass["3"];
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "objectKeys"),
        &["number:1"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "interfaceKeys"),
        &["number:2"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "instanceKeys"),
        &["number:3"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "staticKeys"),
        &["number:4", "string:known", "string:prototype"],
    );
    assert_indeterminate_keyof(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "dynamicStaticKeys"),
        "dynamicStaticKeys",
    );
    assert!(is_inferred_string(
        &db,
        normalized_binding_type(&db, module, inferred, "staticNumericGetterValue")
    ));
    assert!(is_inferred_string(
        &db,
        normalized_binding_type(&db, module, inferred, "staticKnownGetterValue")
    ));
    assert!(is_inferred_boolean(
        &db,
        normalized_binding_type(&db, module, inferred, "instanceNumericGetterValue")
    ));
}

#[test]
fn test_normalize_type_evaluates_keyof_generic_alias_and_call_substitution() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Fleet = { starDestroyers: number; tieFighters: number };
            type Identity<T> = T;

            export function aliased(key: keyof Identity<Fleet>): keyof Identity<Fleet> {
                return key;
            }

            export function keys<T>(object: T): keyof T {
                return undefined as keyof T;
            }

            export const concreteKeys = keys({ starDestroyers: 1, tieFighters: 2 });
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "aliased"),
        &["string:starDestroyers", "string:tieFighters"],
    );
    assert_key_variants(
        &db,
        normalized_binding_type(&db, module, inferred, "concreteKeys"),
        &["string:starDestroyers", "string:tieFighters"],
    );
}

#[test]
fn test_normalize_type_intersects_keyof_index_domains() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type StringIndexed = { [key: string]: unknown };
            type Fixed = { a: number; b: number };

            export function overlap(key: keyof (StringIndexed | Fixed)):
                keyof (StringIndexed | Fixed) {
                return key;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "overlap"),
        &["string:a", "string:b"],
    );
}

#[test]
fn test_normalize_type_keeps_incomplete_keyof_indeterminate() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function generic<T>(key: keyof T): keyof T { return key; }
            export function incomplete(key: keyof MissingType): keyof MissingType { return key; }
            declare function dynamicKey(): string;
            type Dynamic = { [dynamicKey()]: unknown };
            export function dynamic(key: keyof Dynamic): keyof Dynamic { return key; }
            type UnsupportedKey = { "\ud800": unknown };
            export function unsupported(key: keyof UnsupportedKey): keyof UnsupportedKey {
                return key;
            }
            type LargeNumericUnion = keyof (
                { 0x10000000000000000: unknown }
                | { 18446744073709551616: unknown }
            );
            export function largeNumeric(key: LargeNumericUnion): LargeNumericUnion {
                return key;
            }
            interface CycleA extends CycleB { left: string }
            interface CycleB extends CycleA { right: string }
            export function cycle(key: keyof CycleA): keyof CycleA { return key; }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for function_name in [
        "generic",
        "incomplete",
        "dynamic",
        "unsupported",
        "largeNumeric",
        "cycle",
    ] {
        assert_indeterminate_keyof(
            &db,
            normalized_function_parameter_type(&db, module, inferred, function_name),
            function_name,
        );
    }
}

#[test]
fn test_keyof_lookup_reuses_unrelated_edits_and_invalidates_key_edits() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/shapes.ts".into(),
        "export interface Fleet { starDestroyers: number; tieFighters: number; }",
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Fleet } from "./shapes.ts";
            export const key: keyof Fleet = "starDestroyers";
        "#,
    );
    fs.insert("/src/unrelated.ts".into(), "export const unrelated = 1;");

    let mut db = build_js_test_module_db(
        &fs,
        &["/src/shapes.ts", "/src/index.ts", "/src/unrelated.ts"],
        true,
    );
    let shapes = db
        .module_for_path(Utf8Path::new("/src/shapes.ts"))
        .expect("shapes module must exist");
    let index = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let unrelated = db
        .module_for_path(Utf8Path::new("/src/unrelated.ts"))
        .expect("unrelated module must exist");
    let key_range = binding_range_by_name(&db, index, "key");

    {
        db.clear_salsa_events();
        let input = BindingTypeInput::new(&db, index, key_range);
        let raw = infer_binding_type(&db, input).expect("key type must be inferred");
        let normalized_input = NormalizeTypeInput::new(&db, index, raw);
        assert_key_variants(
            &db,
            normalize_type_query(&db, normalized_input),
            &["string:starDestroyers", "string:tieFighters"],
        );
        let events = db.take_salsa_events();
        assert_function_query_was_run(&db, infer_binding_type, input, &events);
        assert_function_query_was_not_run(&db, infer_module_types, index, &events);
        assert_function_query_was_not_run(&db, infer_module_types, shapes, &events);
    }

    fs.insert(
        "/src/unrelated.ts".into(),
        "export const unrelated = 'changed';",
    );
    let unrelated_kind = resolve_js_module_kind_for_test(&fs, "/src/unrelated.ts", true);
    salsa::Setter::to(unrelated.set_kind(&mut db), unrelated_kind);

    {
        db.clear_salsa_events();
        let input = BindingTypeInput::new(&db, index, key_range);
        let raw = infer_binding_type(&db, input).expect("cached key type must be inferred");
        let normalized_input = NormalizeTypeInput::new(&db, index, raw);
        assert_key_variants(
            &db,
            normalize_type_query(&db, normalized_input),
            &["string:starDestroyers", "string:tieFighters"],
        );
        let events = db.take_salsa_events();
        assert_function_query_was_not_run(&db, infer_binding_type, input, &events);
        assert_function_query_was_not_run(&db, normalize_type_query, normalized_input, &events);
        assert_function_query_was_not_run(&db, infer_module_types, index, &events);
    }

    fs.insert(
        "/src/shapes.ts".into(),
        "export interface Fleet { starDestroyers: number; tieFighters: number; shuttles: number; }",
    );
    let shapes_kind = resolve_js_module_kind_for_test(&fs, "/src/shapes.ts", true);
    salsa::Setter::to(shapes.set_kind(&mut db), shapes_kind);

    db.clear_salsa_events();
    let input = BindingTypeInput::new(&db, index, key_range);
    let raw = infer_binding_type(&db, input).expect("updated key type must be inferred");
    let normalized_input = NormalizeTypeInput::new(&db, index, raw);
    assert_key_variants(
        &db,
        normalize_type_query(&db, normalized_input),
        &[
            "string:shuttles",
            "string:starDestroyers",
            "string:tieFighters",
        ],
    );
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, infer_binding_type, input, &events);
    assert_function_query_was_run(&db, normalize_type_query, normalized_input, &events);
    assert_function_query_was_not_run(&db, infer_module_types, index, &events);
    assert_function_query_was_not_run(&db, infer_module_types, shapes, &events);
}

#[test]
fn test_numeric_optional_key_preserves_required_and_partial_value_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Optional = { 1?: string };
            type RequiredOptional = Required<Optional>;
            type PartialRequiredOptional = Partial<RequiredOptional>;
            type OptionalComputedString = { ["a"]?: string };

            const value = { 1: "known" } as const;
            type ConstNumeric = { readonly 1: "known" };
            type ConstRoundTrip = Required<Partial<ConstNumeric>>;

            const computed = { [1]: "computed" };
            class OptionalNumericClass { 1?: string }

            declare const original: Optional;
            declare const required: RequiredOptional;
            declare const partial: PartialRequiredOptional;
            declare const optionalComputedString: OptionalComputedString;
            declare const constRoundTrip: ConstRoundTrip;
            declare const optionalNumericClass: OptionalNumericClass;

            export const originalValue = original["1"];
            export const requiredValue = required["1"];
            export const partialValue = partial["1"];
            export const optionalComputedStringValue = optionalComputedString["a"];
            export const constValue = value["1"];
            export const constRoundTripValue = constRoundTrip["1"];
            export const computedValue = computed["1"];
            export const optionalNumericClassValue = optionalNumericClass["1"];

            export function originalKeys(key: keyof Optional): keyof Optional {
                return key;
            }
            export function requiredKeys(
                key: keyof RequiredOptional,
            ): keyof RequiredOptional {
                return key;
            }
            export function partialKeys(
                key: keyof PartialRequiredOptional,
            ): keyof PartialRequiredOptional {
                return key;
            }
            export function optionalComputedStringKeys(
                key: keyof OptionalComputedString,
            ): keyof OptionalComputedString {
                return key;
            }
            export function constRoundTripKeys(
                key: keyof ConstRoundTrip,
            ): keyof ConstRoundTrip {
                return key;
            }
            export function computedKeys(key: keyof typeof computed): keyof typeof computed {
                return key;
            }
            export function optionalNumericClassKeys(
                key: keyof OptionalNumericClass,
            ): keyof OptionalNumericClass {
                return key;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let original_value = normalized_binding_type(&db, module, inferred, "originalValue");
    let InferredTypeData::Union(original_value) = original_value else {
        panic!("optional numeric lookup must be a String | Undefined union");
    };
    assert_eq!(original_value.types(&db).len(), 2);
    assert!(
        original_value
            .types(&db)
            .contains(&InferredTypeData::String)
    );
    assert!(
        original_value
            .types(&db)
            .contains(&InferredTypeData::Undefined)
    );

    assert_eq!(
        normalized_binding_type(&db, module, inferred, "requiredValue"),
        InferredTypeData::String
    );

    let partial_value = normalized_binding_type(&db, module, inferred, "partialValue");
    let InferredTypeData::Union(partial_value) = partial_value else {
        panic!("partial numeric lookup must be a String | Undefined union");
    };
    assert_eq!(partial_value.types(&db).len(), 2);
    assert!(partial_value.types(&db).contains(&InferredTypeData::String));
    assert!(
        partial_value
            .types(&db)
            .contains(&InferredTypeData::Undefined)
    );

    assert!(is_inferred_string_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "constRoundTripValue"),
        "known"
    ));
    assert!(is_inferred_string_literal(
        &db,
        normalized_binding_type(&db, module, inferred, "constValue"),
        "known"
    ));
    assert!(is_inferred_string(
        &db,
        normalized_binding_type(&db, module, inferred, "computedValue")
    ));

    let optional_computed_string_value =
        normalized_binding_type(&db, module, inferred, "optionalComputedStringValue");
    let InferredTypeData::Union(optional_computed_string_value) = optional_computed_string_value
    else {
        panic!("optional computed string lookup must be a String | Undefined union");
    };
    assert_eq!(optional_computed_string_value.types(&db).len(), 2);
    assert!(
        optional_computed_string_value
            .types(&db)
            .contains(&InferredTypeData::String)
    );
    assert!(
        optional_computed_string_value
            .types(&db)
            .contains(&InferredTypeData::Undefined)
    );

    let optional_numeric_class_value =
        normalized_binding_type(&db, module, inferred, "optionalNumericClassValue");
    let InferredTypeData::Union(optional_numeric_class_value) = optional_numeric_class_value else {
        panic!("optional numeric class lookup must be a String | Undefined union");
    };
    assert_eq!(optional_numeric_class_value.types(&db).len(), 2);
    assert!(
        optional_numeric_class_value
            .types(&db)
            .contains(&InferredTypeData::String)
    );
    assert!(
        optional_numeric_class_value
            .types(&db)
            .contains(&InferredTypeData::Undefined)
    );

    for function_name in [
        "originalKeys",
        "requiredKeys",
        "partialKeys",
        "optionalComputedStringKeys",
        "constRoundTripKeys",
        "computedKeys",
        "optionalNumericClassKeys",
    ] {
        assert_key_variants(
            &db,
            normalized_function_parameter_type(&db, module, inferred, function_name),
            if function_name == "optionalComputedStringKeys" {
                &["string:a"]
            } else {
                &["number:1"]
            },
        );
    }
}

#[test]
fn test_keyof_reuses_deferred_aliases_after_shortcuts() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type A = any;
            type U = A | { a: unknown };
            type V = U;
            type W = U;

            type N = never;
            type NU = N & { n: unknown };
            type NV = NU;
            type NW = NU;

            export function anyAliasReentry(key: keyof (V & W)): keyof (V & W) {
                return key;
            }
            export function neverAliasReentry(key: keyof (NV & NW)): keyof (NV & NW) {
                return key;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for function_name in ["anyAliasReentry", "neverAliasReentry"] {
        assert_key_variants(
            &db,
            normalized_function_parameter_type(&db, module, inferred, function_name),
            &["number", "string", "symbol"],
        );
    }
}

#[test]
fn test_keyof_intersection_preserves_numeric_and_string_key_identity() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type NumericKey = { 1: number };
            type StringKey = { "1": string };
            type AliasedNumericFirst = NumericKey & StringKey;
            type AliasedStringFirst = StringKey & NumericKey;
            type RepeatedNumeric = { 1: number } & { 0x1: string } & { 1: boolean };
            type RepeatedMixed = { 1: number } & { "1": string } & { 1: boolean };

            declare const direct: { 1: number } & { "1": string };
            declare const repeated: { 1: number } & { 0x1: string } & { 1: boolean };
            declare const repeatedMixed: { 1: number } & { "1": string } & { 1: boolean };

            export function directNumericFirst(
                key: keyof ({ 1: number } & { "1": string }),
            ): keyof ({ 1: number } & { "1": string }) {
                return key;
            }
            export function directStringFirst(
                key: keyof ({ "1": string } & { 1: number }),
            ): keyof ({ "1": string } & { 1: number }) {
                return key;
            }
            export function aliasedNumericFirst(
                key: keyof AliasedNumericFirst,
            ): keyof AliasedNumericFirst {
                return key;
            }
            export function aliasedStringFirst(
                key: keyof AliasedStringFirst,
            ): keyof AliasedStringFirst {
                return key;
            }
            export function repeatedNumeric(
                key: keyof ({ 1: number } & { 0x1: string } & { 1: boolean }),
            ): keyof ({ 1: number } & { 0x1: string } & { 1: boolean }) {
                return key;
            }
            export function repeatedMixedKeys(
                key: keyof ({ 1: number } & { "1": string } & { 1: boolean }),
            ): keyof ({ 1: number } & { "1": string } & { 1: boolean }) {
                return key;
            }

            export const directValue = direct["1"];
            export const repeatedValue = repeated["1"];
            export const repeatedMixedValue = repeatedMixed["1"];
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for function_name in [
        "directNumericFirst",
        "directStringFirst",
        "aliasedNumericFirst",
        "aliasedStringFirst",
    ] {
        assert_key_variants(
            &db,
            normalized_function_parameter_type(&db, module, inferred, function_name),
            &["number:1", "string:1"],
        );
    }
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "repeatedNumeric"),
        &["number:1"],
    );
    assert_key_variants(
        &db,
        normalized_function_parameter_type(&db, module, inferred, "repeatedMixedKeys"),
        &["number:1", "string:1"],
    );

    let direct_value = normalized_binding_type(&db, module, inferred, "directValue");
    assert_broad_value_variants(&db, direct_value, &["number", "string"]);

    let repeated_value = normalized_binding_type(&db, module, inferred, "repeatedValue");
    assert_broad_value_variants(&db, repeated_value, &["boolean", "number", "string"]);

    let repeated_mixed_value = normalized_binding_type(&db, module, inferred, "repeatedMixedValue");
    assert_broad_value_variants(&db, repeated_mixed_value, &["boolean", "number", "string"]);
}

#[test]
fn test_normalized_expression_request_preserves_outer_keyof_alias_key_identity() {
    let source = r#"
        type NumericFirst = keyof ({ 1: number } & { "1": string });
        type StringFirst = keyof ({ "1": string } & { 1: number });
        type NumericStringNumeric = keyof (
            { 1: number } & { "1": string } & { 1: boolean }
        );
        type NumericStringString = keyof (
            { 1: number } & { "1": string } & { "1": boolean }
        );

        function numericFirst(key: NumericFirst) {
            switch (key) {
                case 1:
                case "1":
                    return;
            }
        }
        function stringFirst(key: StringFirst) {
            switch (key) {
                case 1:
                case "1":
                    return;
            }
        }
        function numericStringNumeric(key: NumericStringNumeric) {
            switch (key) {
                case 1:
                case "1":
                    return;
            }
        }
        function numericStringString(key: NumericStringString) {
            switch (key) {
                case 1:
                case "1":
                    return;
            }
        }
    "#;
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), source);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for function_name in [
        "numericFirst",
        "stringFirst",
        "numericStringNumeric",
        "numericStringString",
    ] {
        let function_start = source
            .find(&format!("function {function_name}"))
            .expect("function must exist");
        let key_start = function_start
            + source[function_start..]
                .find("switch (key)")
                .expect("switch expression must exist")
            + "switch (".len();
        let expression = TextRange::new(
            TextSize::from(key_start as u32),
            TextSize::from((key_start + "key".len()) as u32),
        );
        db.clear_salsa_events();
        let ty = biome_module_graph::type_inference::execute_type_inference_request(
            &db,
            biome_module_graph::type_inference::TypeInferenceCaller::new("test", "outerKeyofAlias"),
            biome_module_graph::type_inference::NormalizedExpressionTypeRequest::new(
                module, expression,
            ),
        )
        .expect("expression type must be inferred");
        assert_key_variants(&db, ty, &["number:1", "string:1"]);
        let events = db.take_salsa_events();
        assert_function_query_was_not_run(&db, infer_module_types, module, &events);
    }
}
