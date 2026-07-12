use std::collections::BTreeMap;

use biome_db::ParsedSource;
use biome_db::testing::{Events, assert_function_query_was_not_run, assert_function_query_was_run};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_type_info::{
    InferredType, TypeResolverLevel, format_inferred_type,
    resolved::{
        InferredCallArgumentType, InferredClass, InferredFunction, InferredFunctionParameter,
        InferredGenericTypeParameter, InferredInterface, InferredLiteralValue,
        InferredLocalTypeHandle, InferredLocalTypeId, InferredMergedReference, InferredModuleKey,
        InferredNamedFunctionParameter, InferredObject, InferredReturnType, InferredTypeData,
        InferredTypeInstance, InferredTypeMember, InferredTypeMemberKind, InferredTypeofType,
        InferredUnion,
    },
};
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_languages::JsFileSource;
use biome_module_graph::{
    CallArgumentTypeInput, InferredModuleTypes, JsExport, JsOwnExport, ModuleDb,
    ModuleGraphGeneration, ModuleInfo, ModuleInfoKind, NormalizeTypeInput, PathInfoCache,
    infer_call_expression_type as infer_call_expression_type_query,
    infer_constructor_argument_type, infer_module_types, normalize_type as normalize_type_query,
    resolve_js_module,
};
use biome_package::{Dependencies, PackageJson};
use biome_project_layout::ProjectLayout;
use biome_rowan::{AstNode, Text, TextRange};
use biome_test_utils::get_added_js_paths;
use camino::{Utf8Path, Utf8PathBuf};
use salsa::Storage;
use salsa::plumbing::{AsId, FromId};

#[salsa::db]
struct TestModuleDb {
    modules: BTreeMap<Utf8PathBuf, ModuleInfo>,
    events: Events,
    storage: Storage<Self>,
}

#[salsa::input]
struct GenericReplacementBudget {
    steps: usize,
}

#[salsa::input]
#[derive(Debug)]
struct NormalizationBudget {
    module: ModuleInfo,
    distinct_types: usize,
}

#[salsa::input]
#[derive(Debug)]
struct MemberLookupBudget {
    module: ModuleInfo,
    steps: usize,
}

#[salsa::input]
#[derive(Debug)]
struct IndeterminateCallSignature {
    module: ModuleInfo,
    is_indeterminate: bool,
}

#[salsa::tracked]
fn infer_generic_call_for_budget<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    budget: GenericReplacementBudget,
) -> InferredTypeData<'db> {
    let (callee, argument) = generic_call_types(db, budget.steps(db));
    infer_call_expression_type(db, module, callee, Vec::from([argument]))
}

#[salsa::tracked]
fn normalize_type_for_budget<'db>(
    db: &'db dyn ModuleDb,
    budget: NormalizationBudget,
) -> InferredTypeData<'db> {
    let ty = inferred_typeof_chain(db, budget.distinct_types(db), InferredTypeData::String);
    normalize_type(db, budget.module(db), ty)
}

#[salsa::tracked]
fn lookup_member_for_budget<'db>(
    db: &'db dyn ModuleDb,
    budget: MemberLookupBudget,
) -> Option<InferredTypeData<'db>> {
    let module = budget.module(db);
    let leaf = InferredTypeData::Interface(InferredInterface::new(
        db,
        Box::default(),
        Box::default(),
        Vec::from([InferredTypeMember {
            kind: InferredTypeMemberKind::Named(Text::new_static("target")),
            ty: InferredTypeData::Number,
        }])
        .into_boxed_slice(),
        Text::new_static("Known"),
    ));
    local_alias_member_lookup(db, module, budget.steps(db), leaf)
}

#[salsa::tracked]
fn resolve_local_for_budget<'db>(
    db: &'db dyn ModuleDb,
    budget: MemberLookupBudget,
) -> InferredTypeData<'db> {
    let leaf = if budget.steps(db).is_multiple_of(2) {
        InferredTypeData::String
    } else {
        InferredTypeData::Number
    };
    let (inferred, local) = local_alias_types(db, budget.module(db), budget.steps(db), leaf);
    inferred.resolve_type(db, local)
}

#[salsa::tracked]
fn lookup_inherited_member_for_budget<'db>(
    db: &'db dyn ModuleDb,
    budget: MemberLookupBudget,
) -> Option<InferredTypeData<'db>> {
    let module = budget.module(db);
    let mut ty = InferredTypeData::Interface(InferredInterface::new(
        db,
        Box::default(),
        Box::default(),
        Vec::from([InferredTypeMember {
            kind: InferredTypeMemberKind::Named(Text::new_static("target")),
            ty: InferredTypeData::Number,
        }])
        .into_boxed_slice(),
        Text::new_static("Known"),
    ));
    for _ in 1..budget.steps(db) {
        ty = InferredTypeData::Interface(InferredInterface::new(
            db,
            Box::default(),
            Vec::from([ty]).into_boxed_slice(),
            Box::default(),
            Text::new_static("Derived"),
        ));
    }
    let inferred = InferredModuleTypes {
        module_key: InferredModuleKey::new(module.as_id()),
        named_type_ids: Box::default(),
        types: Box::default(),
        expressions: Default::default(),
        binding_type_data: Default::default(),
    };

    inferred.find_member_type(db, ty, "target")
}

#[salsa::tracked]
fn infer_call_with_indeterminate_signature<'db>(
    db: &'db dyn ModuleDb,
    input: IndeterminateCallSignature,
) -> InferredTypeData<'db> {
    let function = InferredTypeData::Function(InferredFunction::new(
        db,
        Box::default(),
        Box::default(),
        InferredReturnType::Type(InferredTypeData::Number),
        false,
        None,
    ));
    let second = if input.is_indeterminate(db) {
        InferredTypeData::Unknown
    } else {
        function
    };
    let callee = InferredTypeData::Object(InferredObject::new(
        db,
        None,
        Vec::from([
            InferredTypeMember {
                kind: InferredTypeMemberKind::CallSignature,
                ty: function,
            },
            InferredTypeMember {
                kind: InferredTypeMemberKind::CallSignature,
                ty: second,
            },
        ])
        .into_boxed_slice(),
    ));

    infer_call_expression_type(db, input.module(db), callee, Vec::new())
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

    let failed = inferred_function_return_ty_by_name(&db, module, &inferred, "failed")
        .expect("failed return type must be inferred");
    let failed = normalize_type(&db, module, failed);
    assert_eq!(failed, InferredTypeData::Unknown);
    assert!(!InferredType::new(&db, failed).is_inferred());

    let explicit = inferred_function_return_ty_by_name(&db, module, &inferred, "explicit")
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
    let result = inferred_binding_ty_by_name(&db, module, &inferred, "result")
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
    let result = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");
    assert!(is_inferred_string(&db, normalize_type(&db, module, result)));
}

#[test]
fn test_namespace_import_preserves_members_above_export_step_limit() {
    let fs = MemoryFileSystem::default();
    let exports = (0..1025)
        .map(|index| format!("export const member{index} = {index};\n"))
        .collect::<String>();
    fs.insert("/src/source.ts".into(), exports);
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import * as source from "./source";
            export const first = source.member0;
            export const last = source.member1024;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for name in ["first", "last"] {
        let ty = inferred_binding_ty_by_name(&db, module, &inferred, name)
            .expect("namespace member type must be inferred");
        assert!(is_inferred_number(&db, normalize_type(&db, module, ty)));
    }
}

fn insert_blanket_reexport_chain(
    fs: &MemoryFileSystem,
    reexport_count: usize,
    leaf_reexport: Option<usize>,
) -> Vec<String> {
    let mut paths = Vec::with_capacity(reexport_count + 1);
    for index in 0..reexport_count {
        let path = format!("/src/module{index}.ts");
        fs.insert(
            path.clone().into(),
            format!("export * from \"./module{}.ts\";", index + 1),
        );
        paths.push(path);
    }

    let leaf_path = format!("/src/module{reexport_count}.ts");
    let leaf_reexport = leaf_reexport.map_or_else(String::new, |index| {
        format!("export * from \"./module{index}.ts\";")
    });
    fs.insert(
        leaf_path.clone().into(),
        format!("export const leaf = 1;{leaf_reexport}"),
    );
    paths.push(leaf_path);
    paths
}

fn insert_namespace_import(fs: &MemoryFileSystem, path: &str, start: usize) {
    fs.insert(
        path.into(),
        format!("import * as namespace from \"./module{start}.ts\"; export {{ namespace }};"),
    );
}

#[test]
fn test_namespace_import_blanket_reexport_step_boundaries_and_cycle() {
    const LIMIT: usize = 1024;

    let fs = MemoryFileSystem::default();
    let mut paths = insert_blanket_reexport_chain(&fs, LIMIT + 1, Some(LIMIT + 1));
    for (path, start) in [
        ("/src/under.ts", 2),
        ("/src/exact.ts", 1),
        ("/src/over.ts", 0),
    ] {
        insert_namespace_import(&fs, path, start);
        paths.push(path.to_string());
    }
    let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let db = build_js_test_module_db(&fs, &path_refs, true);

    for path in ["/src/under.ts", "/src/exact.ts"] {
        let module = db
            .module_for_path(Utf8Path::new(path))
            .expect("importer module must exist");
        let inferred = infer_module_types(&db, module).expect("types must be inferred");
        let namespace_ty = inferred_binding_ty_by_name(&db, module, &inferred, "namespace")
            .expect("namespace binding must exist");
        let InferredTypeData::Namespace(namespace) = namespace_ty else {
            panic!("chain at or below the limit must produce a namespace");
        };
        assert!(
            namespace
                .members(&db)
                .iter()
                .any(|member| { member.kind.name().is_some_and(|name| name.text() == "leaf") })
        );
    }

    let exact_module = db
        .module_for_path(Utf8Path::new("/src/exact.ts"))
        .expect("exact-limit importer must exist");
    let first = infer_module_types(&db, exact_module).expect("types must be inferred");
    let second = infer_module_types(&db, exact_module).expect("types must be cached");
    assert!(std::sync::Arc::ptr_eq(&first, &second));
    let first_ty = inferred_binding_ty_by_name(&db, exact_module, &first, "namespace")
        .expect("namespace binding must exist");
    let second_ty = inferred_binding_ty_by_name(&db, exact_module, &second, "namespace")
        .expect("namespace binding must remain present");
    let (
        InferredTypeData::Namespace(first_namespace),
        InferredTypeData::Namespace(second_namespace),
    ) = (first_ty, second_ty)
    else {
        panic!("repeated query must preserve the namespace");
    };
    assert_eq!(first_namespace.as_id(), second_namespace.as_id());

    let over_module = db
        .module_for_path(Utf8Path::new("/src/over.ts"))
        .expect("over-limit importer must exist");
    let inferred = infer_module_types(&db, over_module).expect("types must be inferred");
    let namespace_ty = inferred_binding_ty_by_name(&db, over_module, &inferred, "namespace")
        .expect("namespace binding must exist");
    assert_eq!(namespace_ty, InferredTypeData::Unknown);
}

#[test]
fn test_namespace_import_invalidates_over_budget_unknown_after_chain_replacement() {
    const LIMIT: usize = 1024;

    let fs = MemoryFileSystem::default();
    let mut paths = insert_blanket_reexport_chain(&fs, LIMIT + 1, None);
    insert_namespace_import(&fs, "/src/index.ts", 0);
    paths.push("/src/index.ts".to_string());
    let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let mut db = build_js_test_module_db(&fs, &path_refs, true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");

    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let namespace_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "namespace")
        .expect("namespace binding must exist");
    assert_eq!(namespace_ty, InferredTypeData::Unknown);
    drop(inferred);

    let replacement_path = format!("/src/module{}.ts", LIMIT - 1);
    fs.insert(replacement_path.clone().into(), "export const leaf = 1;");
    let replacement_module = db
        .module_for_path(Utf8Path::new(&replacement_path))
        .expect("replacement module must exist");
    let replacement_kind = resolve_js_module_kind_for_test(&fs, &replacement_path, true);
    salsa::Setter::to(replacement_module.set_kind(&mut db), replacement_kind);

    let inferred = infer_module_types(&db, index_module).expect("types must be invalidated");
    let namespace_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "namespace")
        .expect("namespace binding must exist");
    let InferredTypeData::Namespace(namespace) = namespace_ty else {
        panic!("shortened chain must produce a complete namespace");
    };
    assert!(
        namespace
            .members(&db)
            .iter()
            .any(|member| { member.kind.name().is_some_and(|name| name.text() == "leaf") })
    );
}

#[test]
fn test_namespace_import_discards_known_members_when_blanket_path_is_unresolved() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        "export const known = 1; export * from './missing';",
    );
    fs.insert(
        "/src/index.ts".into(),
        "import * as namespace from './source'; export { namespace };",
    );
    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_eq!(
        inferred_binding_ty_by_name(&db, module, &inferred, "namespace"),
        Some(InferredTypeData::Unknown)
    );
}

#[test]
fn test_namespace_import_discards_known_members_when_blanket_module_cannot_infer() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        "export const known = 1; export * from './unavailable';",
    );
    fs.insert("/src/unavailable.ts".into(), "export const hidden = 2;");
    fs.insert(
        "/src/index.ts".into(),
        "import * as namespace from './source'; export { namespace };",
    );
    let mut db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let unavailable = ModuleInfo::new(
        &db,
        Utf8PathBuf::from("/src/unavailable.ts"),
        resolve_js_module_kind_for_test(&fs, "/src/unavailable.ts", false),
    );
    db.insert_module(Utf8PathBuf::from("/src/unavailable.ts"), unavailable);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    assert_eq!(
        inferred_binding_ty_by_name(&db, module, &inferred, "namespace"),
        Some(InferredTypeData::Unknown)
    );
}

impl TestModuleDb {
    fn new() -> Self {
        let events = Events::default();
        let db = Self {
            modules: BTreeMap::new(),
            storage: salsa::Storage::new(Some(Box::new({
                let events = events.clone();
                move |event| {
                    events.0.lock().unwrap().push(event);
                }
            }))),
            events,
        };
        ModuleGraphGeneration::new(&db, 0);
        db
    }

    fn insert_module(&mut self, path: Utf8PathBuf, module: ModuleInfo) {
        self.modules.insert(path, module);
        self.bump_module_graph_generation();
    }

    fn remove_module(&mut self, path: &Utf8Path) {
        if self.modules.remove(path).is_some() {
            self.bump_module_graph_generation();
        }
    }

    fn bump_module_graph_generation(&mut self) {
        let generation = ModuleGraphGeneration::get(self);
        let next = generation.value(self).wrapping_add(1);
        salsa::Setter::to(generation.set_value(self), next);
    }

    fn take_salsa_events(&mut self) -> Vec<salsa::Event> {
        std::mem::take(&mut *self.events.0.lock().unwrap())
    }

    fn clear_salsa_events(&mut self) {
        self.take_salsa_events();
    }
}

#[salsa::db]
impl salsa::Database for TestModuleDb {}

#[salsa::db]
impl biome_db::Db for TestModuleDb {
    fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<ParsedSource> {
        None
    }
}

#[salsa::db]
impl biome_module_graph::TypeDb for TestModuleDb {
    fn local_type_name(
        &self,
        module_key: InferredModuleKey,
        type_id: InferredLocalTypeId,
    ) -> Option<Text> {
        let module = ModuleInfo::from_id(module_key.as_id());
        let current = self.module_for_path(module.path(self))?;
        if InferredModuleKey::new(current.as_id()) != module_key {
            return None;
        }

        let ModuleInfoKind::Js(info) = current.kind(self) else {
            return None;
        };
        info.local_type_name(type_id)
    }
}

#[salsa::db]
impl ModuleDb for TestModuleDb {
    fn module_graph_generation(&self) -> u64 {
        ModuleGraphGeneration::get(self).value(self)
    }

    fn module_for_path(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        let _ = self.module_graph_generation();
        self.modules.get(path).copied()
    }

    fn for_each_module(&self, f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind)) {
        let _ = self.module_graph_generation();
        for (path, module) in &self.modules {
            f(path, &module.kind(self));
        }
    }
}

fn is_inferred_instance_of<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    inner: InferredTypeData<'db>,
) -> bool {
    matches!(ty, InferredTypeData::InstanceOf(instance) if instance.ty(db) == inner)
}

fn is_inferred_string<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    ty == InferredTypeData::String
        || is_inferred_instance_of(db, ty, InferredTypeData::String)
        || matches!(ty, InferredTypeData::Literal(literal) if matches!(literal.literal(db), InferredLiteralValue::String(_)))
}

fn is_inferred_number<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    ty == InferredTypeData::Number
        || is_inferred_instance_of(db, ty, InferredTypeData::Number)
        || matches!(ty, InferredTypeData::Literal(literal) if matches!(literal.literal(db), InferredLiteralValue::Number(_)))
}

fn is_inferred_boolean<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    ty == InferredTypeData::Boolean
        || is_inferred_instance_of(db, ty, InferredTypeData::Boolean)
        || matches!(ty, InferredTypeData::Literal(literal) if matches!(literal.literal(db), InferredLiteralValue::Boolean(_)))
}

fn is_inferred_array_of_promises<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    let InferredTypeData::InstanceOf(instance) = ty else {
        return false;
    };

    instance.ty(db).is_array_class(db)
        && instance
            .type_parameters(db)
            .first()
            .is_some_and(|ty| is_inferred_promise_instance(db, *ty))
}

fn is_inferred_promise_instance<'db>(db: &'db dyn ModuleDb, mut ty: InferredTypeData<'db>) -> bool {
    while let InferredTypeData::InstanceOf(instance) = ty {
        ty = instance.ty(db);
        if ty.is_promise_class(db) {
            return true;
        }
    }

    false
}

fn is_inferred_string_literal<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    value: &str,
) -> bool {
    matches!(
        ty,
        InferredTypeData::Literal(literal)
            if matches!(literal.literal(db), InferredLiteralValue::String(string) if string.as_str() == value)
    )
}

fn is_inferred_number_literal<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    value: &str,
) -> bool {
    matches!(
        ty,
        InferredTypeData::Literal(literal)
            if matches!(literal.literal(db), InferredLiteralValue::Number(number) if number.as_str() == value)
    )
}

fn contains_inferred_string_literal<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    value: &str,
) -> bool {
    if is_inferred_string_literal(db, ty, value) {
        return true;
    }

    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| contains_inferred_string_literal(db, *ty, value)),
        _ => false,
    }
}

fn contains_inferred_number_literal<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    value: &str,
) -> bool {
    if is_inferred_number_literal(db, ty, value) {
        return true;
    }

    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| contains_inferred_number_literal(db, *ty, value)),
        _ => false,
    }
}

fn contains_inferred_string<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    if is_inferred_string(db, ty) {
        return true;
    }

    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| contains_inferred_string(db, *ty)),
        _ => false,
    }
}

fn contains_inferred_number<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    if is_inferred_number(db, ty) {
        return true;
    }

    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| contains_inferred_number(db, *ty)),
        _ => false,
    }
}

fn contains_inferred_boolean<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    if is_inferred_boolean(db, ty) {
        return true;
    }

    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| contains_inferred_boolean(db, *ty)),
        _ => false,
    }
}

fn contains_inferred_undefined<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    if ty == InferredTypeData::Undefined {
        return true;
    }

    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| contains_inferred_undefined(db, *ty)),
        _ => false,
    }
}

fn contains_inferred_null<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    if ty == InferredTypeData::Null {
        return true;
    }

    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| contains_inferred_null(db, *ty)),
        _ => false,
    }
}

fn contains_inferred_instance<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    match ty {
        InferredTypeData::InstanceOf(_) => true,
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| contains_inferred_instance(db, *ty)),
        _ => false,
    }
}

fn assert_inferred_function_returns_number<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) {
    let InferredTypeData::Function(function) = ty else {
        panic!("type must be inferred as a function");
    };
    let InferredReturnType::Type(return_ty) = function.return_type(db) else {
        panic!("function return type must be inferred as a type");
    };

    assert!(is_inferred_number(db, *return_ty));
}

fn object_member_ty_by_name<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    member_name: &str,
) -> Option<(InferredTypeMemberKind<'db>, InferredTypeData<'db>)> {
    let object = match ty {
        InferredTypeData::Object(object) => object,
        InferredTypeData::InstanceOf(instance) => match instance.ty(db) {
            InferredTypeData::Object(object) => object,
            _ => return None,
        },
        _ => return None,
    };

    object.members(db).iter().find_map(|member| {
        member
            .kind
            .name()
            .is_some_and(|name| name.text() == member_name)
            .then(|| (member.kind.clone(), member.ty))
    })
}

fn inferred_binding_ty_by_name<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    inferred: &InferredModuleTypes<'db>,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        return None;
    };
    let binding = info.semantic_model.all_bindings().find(|binding| {
        binding
            .tree()
            .name_token()
            .is_ok_and(|token| token.text_trimmed() == name)
    })?;

    inferred
        .binding_type_data
        .get(&binding.syntax().text_trimmed_range())
        .map(|data| data.ty)
}

fn inferred_function_return_ty_by_name<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    inferred: &InferredModuleTypes<'db>,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    let binding_ty = inferred_binding_ty_by_name(db, module, inferred, name)?;
    let function = inferred
        .resolve_type(db, binding_ty)
        .callable_function(db)?;
    let InferredReturnType::Type(return_ty) = function.return_type(db) else {
        return None;
    };

    Some(inferred.resolve_type(db, *return_ty))
}

#[salsa::tracked]
fn inferred_expression_count(db: &dyn ModuleDb, module: ModuleInfo) -> usize {
    infer_module_types(db, module).map_or(0, |inferred| inferred.expressions.len())
}

fn is_inferred_promise_with_type_parameter<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    predicate: impl Fn(InferredTypeData<'db>) -> bool,
) -> bool {
    let InferredTypeData::InstanceOf(instance) = ty else {
        return false;
    };

    ty.is_promise_instance(db) == Some(true)
        && instance.type_parameters(db).iter().any(|ty| predicate(*ty))
}

fn assert_inferred_function_returns_string<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) {
    let InferredTypeData::Function(function) = ty else {
        panic!("type must be inferred as a function");
    };
    let InferredReturnType::Type(return_ty) = function.return_type(db) else {
        panic!("function return type must be inferred as a type");
    };

    assert!(is_inferred_string(db, *return_ty));
}

fn local_type_id_of_instance<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<usize> {
    let InferredTypeData::InstanceOf(instance) = ty else {
        return None;
    };
    let InferredTypeData::Local(local) = instance.ty(db) else {
        return None;
    };

    Some(local.type_id(db).index())
}

fn infer_call_expression_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    callee: InferredTypeData<'db>,
    args: Vec<InferredTypeData<'db>>,
) -> InferredTypeData<'db> {
    infer_call_expression_type_query(db, module, callee, &args)
}

fn nested_instance_type<'db>(
    db: &'db dyn ModuleDb,
    depth: usize,
    leaf: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    (0..depth).fold(leaf, |ty, _| {
        InferredTypeData::InstanceOf(InferredTypeInstance::new(
            db,
            ty,
            Vec::new().into_boxed_slice(),
        ))
    })
}

fn inferred_typeof_chain<'db>(
    db: &'db dyn ModuleDb,
    distinct_types: usize,
    leaf: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    assert!(distinct_types > 0);
    (1..distinct_types).fold(leaf, |ty, _| {
        InferredTypeData::TypeofType(InferredTypeofType::new(db, ty))
    })
}

fn substitution_generic<'db>(db: &'db dyn ModuleDb) -> InferredTypeData<'db> {
    InferredTypeData::Generic(InferredGenericTypeParameter::new(
        db,
        None,
        None,
        Text::new_static("T"),
    ))
}

fn generic_interface_instance_with_member<'db>(
    db: &'db dyn ModuleDb,
    distinct_types: usize,
) -> InferredTypeData<'db> {
    let generic = substitution_generic(db);
    let interface = InferredTypeData::Interface(InferredInterface::new(
        db,
        Vec::from([generic]).into_boxed_slice(),
        Box::default(),
        Vec::from([InferredTypeMember {
            kind: InferredTypeMemberKind::Named(Text::new_static("target")),
            ty: inferred_typeof_chain(db, distinct_types, generic),
        }])
        .into_boxed_slice(),
        Text::new_static("Container"),
    ));
    InferredTypeData::InstanceOf(InferredTypeInstance::new(
        db,
        interface,
        Vec::from([InferredTypeData::Number]).into_boxed_slice(),
    ))
}

fn local_alias_member_lookup<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    steps: usize,
    leaf: InferredTypeData<'db>,
) -> Option<InferredTypeData<'db>> {
    let (inferred, ty) = local_alias_types(db, module, steps, leaf);
    inferred.find_member_type(db, ty, "target")
}

fn local_alias_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    steps: usize,
    leaf: InferredTypeData<'db>,
) -> (InferredModuleTypes<'db>, InferredTypeData<'db>) {
    assert!(steps >= 1);
    let module_key = InferredModuleKey::new(module.as_id());
    let mut types = Vec::with_capacity(steps);
    for index in 0..steps {
        let ty = if index + 1 == steps {
            leaf
        } else {
            InferredTypeData::Local(InferredLocalTypeHandle::new(
                db,
                module_key,
                InferredLocalTypeId::new(index + 1),
            ))
        };
        types.push(ty);
    }
    let inferred = InferredModuleTypes {
        module_key,
        named_type_ids: Box::default(),
        types: types.into_boxed_slice(),
        expressions: Default::default(),
        binding_type_data: Default::default(),
    };
    let ty = InferredTypeData::Local(InferredLocalTypeHandle::new(
        db,
        module_key,
        InferredLocalTypeId::new(0),
    ));
    (inferred, ty)
}

fn generic_replacement_chain<'db>(
    db: &'db dyn ModuleDb,
    steps: usize,
    leaf: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    assert!(steps >= 2);
    let wrapper = InferredTypeData::Class(InferredClass::new(
        db,
        Box::default(),
        None,
        Box::default(),
        Box::default(),
        Some(Text::new_static("Wrapper")),
    ));
    (0..steps - 2).fold(leaf, |ty, _| {
        InferredTypeData::instance_of(db, wrapper, Box::new([ty]))
    })
}

fn generic_callback<'db>(
    db: &'db dyn ModuleDb,
    return_ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    InferredTypeData::Function(InferredFunction::new(
        db,
        Box::default(),
        Box::default(),
        InferredReturnType::Type(return_ty),
        false,
        None,
    ))
}

fn generic_call_types<'db>(
    db: &'db dyn ModuleDb,
    steps: usize,
) -> (InferredTypeData<'db>, InferredTypeData<'db>) {
    let generic = InferredTypeData::Generic(InferredGenericTypeParameter::new(
        db,
        None,
        None,
        Text::new_static("T"),
    ));
    let parameter_ty = generic_callback(db, generic_replacement_chain(db, steps, generic));
    let callee = InferredTypeData::Function(InferredFunction::new(
        db,
        Vec::from([generic]).into_boxed_slice(),
        Vec::from([InferredFunctionParameter::Named(
            InferredNamedFunctionParameter {
                name: Text::new_static("callback"),
                ty: parameter_ty,
                is_optional: false,
                is_rest: false,
            },
        )])
        .into_boxed_slice(),
        InferredReturnType::Type(generic),
        false,
        Some(Text::new_static("run")),
    ));
    let argument = generic_callback(
        db,
        generic_replacement_chain(db, steps, InferredTypeData::Number),
    );
    (callee, argument)
}

fn normalize_type<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    normalize_type_query(db, NormalizeTypeInput::new(db, module, ty))
}

fn interface_member_ty<'db>(
    db: &'db dyn ModuleDb,
    interface: InferredInterface<'db>,
    member_name: &str,
) -> Option<InferredTypeData<'db>> {
    interface.members(db).iter().find_map(|member| {
        matches!(
            &member.kind,
            InferredTypeMemberKind::Named(name) if name.text() == member_name,
        )
        .then_some(member.ty)
    })
}

fn inferred_overload_ty_by_name<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    inferred: &InferredModuleTypes<'db>,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        return None;
    };

    info.semantic_model
        .all_bindings()
        .filter(|binding| {
            binding
                .tree()
                .name_token()
                .is_ok_and(|token| token.text_trimmed() == name)
        })
        .filter_map(|binding| {
            inferred
                .binding_type_data
                .get(&binding.syntax().text_trimmed_range())
                .map(|data| inferred.resolve_type(db, data.ty))
        })
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Object(object)
                    if object
                        .members(db)
                        .iter()
                        .filter(|member| member.kind.is_call_signature())
                        .count()
                        >= 2
            )
        })
}

fn assert_inferred_type_snapshot(test_name: &str, db: &dyn ModuleDb, fs: &MemoryFileSystem) {
    let mut content = String::new();
    let files = source_files_from_memory_fs(fs);
    for (file_name, source_code) in &files {
        let file_name = Utf8PathBuf::from(file_name.as_str());
        write_source_file(&mut content, &file_name, source_code);

        let Some(module) = db.module_for_path(file_name.as_path()) else {
            continue;
        };
        let Some(inferred) = infer_module_types(db, module) else {
            continue;
        };
        write_inferred_type_rows(&mut content, db, module, &inferred, source_code);
    }

    insta::with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(test_name, content);
    });
}

struct InferredTypeSnapshotRow {
    range: TextRange,
    text: String,
}

fn write_inferred_type_rows<'db>(
    content: &mut String,
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    inferred: &InferredModuleTypes<'db>,
    source_code: &str,
) {
    let ModuleInfoKind::Js(info) = module.kind(db) else {
        return;
    };

    let mut rows = Vec::new();
    for range in info.raw_binding_types.keys() {
        let Some(data) = inferred.binding_type_data.get(range) else {
            continue;
        };
        let binding_name = info
            .semantic_model
            .as_binding_by_range(*range)
            .and_then(|binding| binding.tree().name_token().ok())
            .map_or_else(
                || "<unknown>".to_string(),
                |token| token.text_trimmed().to_string(),
            );
        rows.push(InferredTypeSnapshotRow {
            range: *range,
            text: inferred_type_snapshot_row(
                format!(
                    "Binding {binding_name} {:?}",
                    source_snippet(source_code, *range)
                ),
                format_inferred_type(db, inferred.resolve_type(db, data.ty)),
            ),
        });
    }

    for (range, ty) in &inferred.expressions {
        rows.push(InferredTypeSnapshotRow {
            range: *range,
            text: inferred_type_snapshot_row(
                format!("Expression {:?}", source_snippet(source_code, *range)),
                format_inferred_type(db, inferred.resolve_type(db, *ty)),
            ),
        });
    }

    if rows.is_empty() {
        return;
    }

    rows.sort_by(|left, right| {
        left.range
            .start()
            .cmp(&right.range.start())
            .then_with(|| left.range.end().cmp(&right.range.end()))
            .then_with(|| left.text.cmp(&right.text))
    });

    content.push_str("\n\n## Inferred types\n\n```");
    for row in rows {
        content.push('\n');
        content.push_str(&row.text);
        content.push('\n');
    }
    content.push_str("```\n");
}

fn inferred_type_snapshot_row(label: String, formatted_ty: String) -> String {
    if !formatted_ty
        .lines()
        .skip(1)
        .any(|line| line.starts_with("| ") || line.starts_with("& "))
    {
        return format!("{label} => {formatted_ty}");
    }

    let indented_ty = formatted_ty
        .lines()
        .map(|line| format!("  {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{label} =>\n{indented_ty}")
}

fn source_snippet(source_code: &str, range: TextRange) -> String {
    let start = usize::from(range.start());
    let end = usize::from(range.end());
    source_code
        .get(start..end)
        .unwrap_or("<invalid range>")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn source_files_from_memory_fs(fs: &MemoryFileSystem) -> BTreeMap<String, String> {
    fs.files
        .read()
        .iter()
        .map(|(file, entry)| {
            let content = entry.lock();
            let content = String::from_utf8_lossy(content.as_slice()).into_owned();
            (file.as_str().to_string(), content)
        })
        .collect()
}

fn write_source_file(content: &mut String, file_name: &Utf8PathBuf, source_code: &str) {
    let extension = file_name.extension().unwrap_or_default();

    content.push_str("\n# `");
    content.push_str(file_name.as_str());
    content.push_str("`\n\n## Source\n\n");
    content.push_str("```");
    content.push_str(extension);
    content.push('\n');

    if let Ok(file_source) = JsFileSource::try_from(file_name.as_path()) {
        let tree = parse(source_code, file_source, JsParserOptions::default());
        let formatted = format_node(JsFormatOptions::default(), tree.tree().syntax(), Vec::new())
            .unwrap()
            .print()
            .unwrap();
        content.push_str(formatted.as_code().trim());
    } else {
        content.push_str(source_code.trim());
    }

    content.push_str("\n```");
}

fn resolve_js_module_kind_for_test(
    fs: &MemoryFileSystem,
    path: &str,
    infer_types: bool,
) -> ModuleInfoKind {
    resolve_js_module_kind_with_layout(fs, &ProjectLayout::default(), path, infer_types)
}

fn resolve_js_module_kind_with_layout(
    fs: &MemoryFileSystem,
    project_layout: &ProjectLayout,
    path: &str,
    infer_types: bool,
) -> ModuleInfoKind {
    let paths = [BiomePath::new(path)];
    let mut added_paths = get_added_js_paths(fs, &paths);
    let (path, root, semantic_model) = added_paths.pop().expect("module must parse");
    let (module_info, _, _) = resolve_js_module(
        root,
        path,
        fs,
        project_layout,
        semantic_model,
        &PathInfoCache::default(),
        infer_types,
    );

    ModuleInfoKind::Js(module_info)
}

fn build_js_test_module_db(
    fs: &MemoryFileSystem,
    paths: &[&str],
    infer_types: bool,
) -> TestModuleDb {
    build_js_test_module_db_with_layout(fs, &ProjectLayout::default(), paths, infer_types)
}

fn build_js_test_module_db_with_layout(
    fs: &MemoryFileSystem,
    project_layout: &ProjectLayout,
    paths: &[&str],
    infer_types: bool,
) -> TestModuleDb {
    let mut db = TestModuleDb::new();
    for path in paths {
        let module_info = ModuleInfo::new(
            &db,
            Utf8PathBuf::from(*path),
            resolve_js_module_kind_with_layout(fs, project_layout, path, infer_types),
        );
        db.insert_module(Utf8PathBuf::from(*path), module_info);
    }
    db
}

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

    let map_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readMap")
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

    let set_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readSet")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readWeakMap")
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
        let ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, function_name)
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
        let ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, function_name)
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

    let record_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readRecord")
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

    let pick_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readPick")
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

    let omit_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readOmit")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readPartial")
            .expect("readPartial return type must be inferred");
    let (partial_name_kind, partial_name_ty) =
        object_member_ty_by_name(&db, partial_ty, "name").expect("Partial<Source> must keep name");
    assert!(partial_name_kind.is_optional());
    assert!(contains_inferred_string(&db, partial_name_ty));
    assert!(contains_inferred_undefined(&db, partial_name_ty));

    let required_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readRequired")
            .expect("readRequired return type must be inferred");
    let (required_optional_kind, required_optional_ty) =
        object_member_ty_by_name(&db, required_ty, "optional")
            .expect("Required<Source> must keep optional");
    assert!(!required_optional_kind.is_optional());
    assert!(is_inferred_string(&db, required_optional_ty));
    assert!(!contains_inferred_undefined(&db, required_optional_ty));

    let readonly_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readReadonly")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readRightNever")
            .expect("readRightNever return type must be inferred");
    let right_name_ty = inferred
        .find_member_type(&db, right_never_ty, "name")
        .expect("Named & never must preserve Named members for legacy parity");
    assert!(is_inferred_string(&db, right_name_ty));

    let left_never_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readLeftNever")
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

    let union_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readUnion")
        .map(|ty| normalize_type(&db, index_module, ty))
        .expect("readUnion return type must be inferred");
    assert!(contains_inferred_string(&db, union_ty));
    assert!(contains_inferred_number(&db, union_ty));

    let intersection_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readIntersection")
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
fn test_infer_module_types_resolves_namespace_import_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        r#"
            export function alpha(): number {
                return 1;
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import * as source from "./source.ts";

            export { source };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/source.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let source_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "source")
        .expect("source import type must be inferred");
    let InferredTypeData::Namespace(_) = source_ty else {
        panic!("namespace import must infer a namespace, got {source_ty:?}");
    };

    let alpha_ty = inferred
        .find_member_type(&db, source_ty, "alpha")
        .expect("namespace import must expose source.alpha");
    assert_inferred_function_returns_number(&db, alpha_ty);

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_namespace_import_members",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_resolves_namespace_reexport_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/source.ts".into(),
        r#"
            export function alpha(): number {
                return 1;
            }
        "#,
    );
    fs.insert(
        "/src/barrel.ts".into(),
        r#"export * as MyNs from "./source.ts";"#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { MyNs } from "./barrel.ts";

            export { MyNs };
        "#,
    );

    let db = build_js_test_module_db(
        &fs,
        &["/src/source.ts", "/src/barrel.ts", "/src/index.ts"],
        true,
    );
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let namespace_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "MyNs")
        .expect("MyNs import type must be inferred");
    let InferredTypeData::Namespace(_) = namespace_ty else {
        panic!("namespace reexport import must infer a namespace, got {namespace_ty:?}");
    };

    let alpha_ty = inferred
        .find_member_type(&db, namespace_ty, "alpha")
        .expect("namespace reexport must expose source.alpha");
    assert_inferred_function_returns_number(&db, alpha_ty);

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_namespace_reexport_members",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_warms_blanket_reexports() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/leaf.ts".into(),
        r#"
            export type Source = {
                name: string;
            };
        "#,
    );
    fs.insert(
        "/src/mid.ts".into(),
        r#"
            export * from "./leaf.ts";
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Source } from "./mid.ts";

            export function read(value: Source): Source {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/leaf.ts", "/src/mid.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");

    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let return_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "read")
        .expect("read return type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, return_ty, "name")
        .expect("re-exported Source must expose name");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_handles_deep_import_chains() {
    const MODULE_COUNT: usize = 512;

    let fs = MemoryFileSystem::default();
    let paths = (0..MODULE_COUNT)
        .map(|index| format!("/src/module_{index}.ts"))
        .collect::<Vec<_>>();

    for (index, path) in paths.iter().enumerate() {
        let source = if index + 1 == MODULE_COUNT {
            r#"export const value = "leaf";"#.to_string()
        } else {
            format!(
                r#"
                    import {{ value as next }} from "./module_{}.ts";
                    export const value = next;
                "#,
                index + 1
            )
        };
        fs.insert(path.clone().into(), source);
    }

    let path_refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let db = build_js_test_module_db(&fs, &path_refs, true);
    let root = db
        .module_for_path(Utf8Path::new("/src/module_0.ts"))
        .expect("root module must exist");
    infer_module_types(&db, root).expect("types must be inferred");

    let inferred = infer_module_types(&db, root).expect("warmed types must be available");
    let value_ty = inferred_binding_ty_by_name(&db, root, &inferred, "value")
        .expect("value type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, value_ty),
        "leaf"
    ));
}

#[test]
fn test_infer_module_types_preserves_local_types_in_import_cycles() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import { importedB } from "./b";
            import { sideC } from "./c";
            export const localA = "a";
            export const fromB = importedB;
            export const fromC = sideC;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { localA } from "./a";
            export const localB = 1;
            export const importedB = localA;
        "#,
    );
    fs.insert(
        "/src/c.ts".into(),
        r#"
            export const sideC = "c";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts", "/src/c.ts"], true);
    let a_module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let b_module = db
        .module_for_path(Utf8Path::new("/src/b.ts"))
        .expect("b module must exist");
    let a_types = infer_module_types(&db, a_module).expect("a types must be inferred");
    let b_types = infer_module_types(&db, b_module).expect("b types must be inferred");

    let local_a = inferred_binding_ty_by_name(&db, a_module, &a_types, "localA")
        .expect("localA type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        a_types.resolve_type(&db, local_a),
        "a"
    ));

    let local_b = inferred_binding_ty_by_name(&db, b_module, &b_types, "localB")
        .expect("localB type must be inferred");
    assert!(is_inferred_number_literal(
        &db,
        b_types.resolve_type(&db, local_b),
        "1"
    ));

    let from_b = inferred_binding_ty_by_name(&db, a_module, &a_types, "fromB")
        .expect("fromB type must be present");
    assert_eq!(a_types.resolve_type(&db, from_b), InferredTypeData::Unknown);

    let from_c = inferred_binding_ty_by_name(&db, a_module, &a_types, "fromC")
        .expect("fromC type must be present");
    assert!(is_inferred_string_literal(
        &db,
        a_types.resolve_type(&db, from_c),
        "c"
    ));

    let imported_b = inferred_binding_ty_by_name(&db, b_module, &b_types, "importedB")
        .expect("importedB type must be present");
    assert_eq!(
        b_types.resolve_type(&db, imported_b),
        InferredTypeData::Unknown
    );
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readLocalBox")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readImportedBox")
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
fn test_infer_module_types_substitutes_generic_members_through_lookup_traversal() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface InterfaceBase<T> {
                value: T;
            }

            interface InterfaceBox<T> extends InterfaceBase<T> {}

            class ClassBase<T> {
                value: T;
            }

            class ClassBox<T> extends ClassBase<T> {}

            interface UnionBox<T> {
                value: T;
            }

            export function readInterfaceBox(
                value: InterfaceBox<number>,
            ): InterfaceBox<number> {
                return value;
            }

            export function readClassBox(value: ClassBox<string>): ClassBox<string> {
                return value;
            }

            export function readUnionBox(
                value: UnionBox<string> | UnionBox<number>,
            ): UnionBox<string> | UnionBox<number> {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let interface_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readInterfaceBox")
            .expect("readInterfaceBox return type must be inferred");
    let interface_value_ty = inferred
        .find_member_type(&db, interface_box_ty, "value")
        .expect("InterfaceBox<number>.value must be inferred through extends");
    assert!(is_inferred_number(&db, interface_value_ty));

    let class_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readClassBox")
            .expect("readClassBox return type must be inferred");
    let class_value_ty = inferred
        .find_member_type(&db, class_box_ty, "value")
        .expect("ClassBox<string>.value must be inferred through extends");
    assert!(is_inferred_string(&db, class_value_ty));

    let union_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readUnionBox")
            .expect("readUnionBox return type must be inferred");
    let union_value_ty = inferred
        .find_member_type(&db, union_box_ty, "value")
        .expect("UnionBox<string> | UnionBox<number> value must be inferred");
    assert!(contains_inferred_string(&db, union_value_ty));
    assert!(contains_inferred_number(&db, union_value_ty));
}

#[test]
fn test_infer_module_types_resolves_imported_exported_type() {
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

            export const value: Foo = "value";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let types_module = db
        .module_for_path(Utf8Path::new("/src/types.ts"))
        .expect("module must exist");
    let inferred_types = infer_module_types(&db, types_module).expect("types must be inferred");
    let ModuleInfoKind::Js(types_info) = types_module.kind(&db) else {
        panic!("module must be JavaScript");
    };
    assert_eq!(inferred_types.types.len(), types_info.raw_types.len());

    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    assert!(
        inferred
            .binding_type_data
            .values()
            .any(|data| data.ty == InferredTypeData::String)
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

    let derived_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "derived")
        .expect("derived binding type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived instance must inherit Base.name");
    assert!(is_inferred_string(&db, name_ty));

    let derived_class_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "Derived")
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

    let derived_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "derived")
        .expect("derived binding type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived interface must inherit Base.name");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_local_alias_resolution_and_member_lookup_exceed_old_limit_and_memoize() {
    const LIMIT: usize = 1024;

    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        "interface Known { target: number } declare const known: Known;",
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for steps in [LIMIT - 1, LIMIT, LIMIT + 1] {
        let budget = MemberLookupBudget::new(&db, module, steps);
        let result = lookup_member_for_budget(&db, budget);
        assert_eq!(
            result,
            Some(InferredTypeData::Number),
            "lookup steps {steps}"
        );

        let resolved = resolve_local_for_budget(&db, budget);
        let expected = if steps.is_multiple_of(2) {
            InferredTypeData::String
        } else {
            InferredTypeData::Number
        };
        assert_eq!(resolved, expected, "resolution steps {steps}");

        if steps == LIMIT + 1 {
            db.events.0.lock().unwrap().clear();
            assert_eq!(lookup_member_for_budget(&db, budget), result);
            assert_eq!(resolve_local_for_budget(&db, budget), resolved);
            let events = std::mem::take(&mut *db.events.0.lock().unwrap());
            assert_function_query_was_not_run(&db, lookup_member_for_budget, budget, &events);
            assert_function_query_was_not_run(&db, resolve_local_for_budget, budget, &events);
        }
    }
}

#[test]
fn test_member_lookup_interface_inheritance_boundaries() {
    const LIMIT: usize = 1024;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for steps in [LIMIT - 1, LIMIT, LIMIT + 1] {
        let budget = MemberLookupBudget::new(&db, module, steps);
        let result = lookup_inherited_member_for_budget(&db, budget);
        let expected = if steps <= LIMIT {
            Some(InferredTypeData::Number)
        } else {
            Some(InferredTypeData::Unknown)
        };
        assert_eq!(result, expected, "interface lookup steps {steps}");
    }
}

#[test]
fn test_local_alias_resolution_invalidates_after_budget_replacement() {
    const LIMIT: usize = 1024;

    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        "interface Known { target: number } declare const known: Known;",
    );
    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let budget = MemberLookupBudget::new(&db, module, LIMIT + 1);

    assert_eq!(
        lookup_member_for_budget(&db, budget),
        Some(InferredTypeData::Number)
    );
    assert_eq!(
        resolve_local_for_budget(&db, budget),
        InferredTypeData::Number
    );
    db.events.0.lock().unwrap().clear();
    salsa::Setter::to(budget.set_steps(&mut db), LIMIT);
    assert_eq!(
        lookup_member_for_budget(&db, budget),
        Some(InferredTypeData::Number)
    );
    assert_eq!(
        resolve_local_for_budget(&db, budget),
        InferredTypeData::String
    );
    let events = std::mem::take(&mut *db.events.0.lock().unwrap());
    assert_function_query_was_run(&db, lookup_member_for_budget, budget, &events);
    assert_function_query_was_run(&db, resolve_local_for_budget, budget, &events);
}

#[test]
fn test_member_lookup_distinguishes_absence_from_uncertainty_and_completed_cycles() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Known { target: number }
            interface ExplicitUnknown { target: unknown }
            interface CycleA extends CycleB {}
            interface CycleB extends CycleA {}
            declare const known: Known;
            declare const explicitUnknown: ExplicitUnknown;
            declare const cycle: CycleA;
        "#,
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let known = inferred_binding_ty_by_name(&db, module, &inferred, "known")
        .expect("known binding must exist");
    let explicit_unknown = inferred_binding_ty_by_name(&db, module, &inferred, "explicitUnknown")
        .expect("explicitUnknown binding must exist");
    let cycle = inferred_binding_ty_by_name(&db, module, &inferred, "cycle")
        .expect("cycle binding must exist");

    for ty in [
        InferredTypeData::Unknown,
        InferredTypeData::UnknownKeyword,
        InferredTypeData::AnyKeyword,
        explicit_unknown,
    ] {
        assert_eq!(
            inferred.find_member_type(&db, ty, "target"),
            Some(InferredTypeData::Unknown)
        );
    }

    for variants in [
        [known, InferredTypeData::Unknown],
        [InferredTypeData::Unknown, known],
    ] {
        let partially_known = InferredTypeData::Union(InferredUnion::new(
            &db,
            Vec::from(variants).into_boxed_slice(),
        ));
        assert_eq!(
            inferred.find_member_type(&db, partially_known, "target"),
            Some(InferredTypeData::Unknown)
        );
    }

    assert_eq!(inferred.find_member_type(&db, cycle, "target"), None);
    let known_with_cycle = InferredTypeData::Union(InferredUnion::new(
        &db,
        Vec::from([known, cycle]).into_boxed_slice(),
    ));
    assert_eq!(
        inferred.find_member_type(&db, known_with_cycle, "target"),
        Some(InferredTypeData::Number)
    );

    let module_key = InferredModuleKey::new(module.as_id());
    let local_cycle = InferredModuleTypes {
        module_key,
        named_type_ids: Box::default(),
        types: Vec::from([
            InferredTypeData::Local(InferredLocalTypeHandle::new(
                &db,
                module_key,
                InferredLocalTypeId::new(1),
            )),
            InferredTypeData::Local(InferredLocalTypeHandle::new(
                &db,
                module_key,
                InferredLocalTypeId::new(0),
            )),
        ])
        .into_boxed_slice(),
        expressions: Default::default(),
        binding_type_data: Default::default(),
    };
    let local = InferredTypeData::Local(InferredLocalTypeHandle::new(
        &db,
        module_key,
        InferredLocalTypeId::new(0),
    ));
    assert_eq!(
        local_cycle.find_member_type(&db, local, "target"),
        Some(InferredTypeData::Unknown)
    );
    assert_eq!(
        local_cycle.resolve_type(&db, local),
        InferredTypeData::Unknown
    );

    let self_cycle = InferredModuleTypes {
        module_key,
        named_type_ids: Box::default(),
        types: Vec::from([local]).into_boxed_slice(),
        expressions: Default::default(),
        binding_type_data: Default::default(),
    };
    assert_eq!(
        self_cycle.resolve_type(&db, local),
        InferredTypeData::Unknown
    );
    assert_eq!(
        self_cycle.find_member_type(&db, local, "target"),
        Some(InferredTypeData::Unknown)
    );
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
    let combined_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "combined")
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
    let item_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "item")
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
    let read_name_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readName")
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
    let dictionary_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "dictionary")
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
            declare const key: string;

            export const object = {
                ["name"]: "object",
            };

            export const broad = {
                [key]: 1,
            };

            export const disposable = {
                [Symbol.dispose]() {},
                [Symbol.asyncDispose]() {},
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let object_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "object")
        .expect("object binding type must be inferred");

    let name_ty = inferred
        .find_member_type(&db, object_ty, "name")
        .expect("computed string literal member must match its literal name");
    assert!(is_inferred_string(&db, name_ty));

    assert!(inferred.find_member_type(&db, object_ty, "other").is_none());

    let broad_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "broad")
        .expect("broad binding type must be inferred");
    let arbitrary_ty = inferred
        .find_member_type(&db, broad_ty, "anything")
        .expect("computed string member must match arbitrary string names");
    assert!(is_inferred_number(&db, arbitrary_ty));

    let disposable_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "disposable")
        .expect("disposable binding type must be inferred");
    let disposable_ty = InferredType::new(&db, inferred.resolve_type(&db, disposable_ty));
    assert!(disposable_ty.is_disposable());
    assert!(disposable_ty.is_async_disposable());
}

#[test]
fn test_infer_module_types_preserves_const_asserted_object_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"export const object = { value: "x" as const };"#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let object_ty = inferred_binding_ty_by_name(&db, module, &inferred, "object")
        .expect("object binding type must be inferred");
    let InferredTypeData::Object(object) = inferred.resolve_type(&db, object_ty) else {
        panic!("object binding must resolve to an object");
    };
    let value = object
        .members(&db)
        .iter()
        .find(|member| {
            member
                .kind
                .name()
                .is_some_and(|name| name.text() == "value")
        })
        .expect("value member must exist");

    assert!(value.kind.is_const_asserted());
    assert!(is_inferred_string_literal(&db, value.ty, "x"));
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
fn test_infer_module_types_resolves_destructured_intersection_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Config {
                mutate: () => Promise<void>;
            }

            interface Other {
                value: string;
            }

            type Full = Config & Other;
            declare function config(): Full;
            export const { mutate } = config();
            export const result = mutate();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    let mutate_ty = inferred_binding_ty_by_name(&db, module, &inferred, "mutate")
        .expect("mutate binding type must be inferred");
    assert!(
        inferred
            .resolve_type(&db, mutate_ty)
            .callable_function(&db)
            .is_some()
    );

    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result binding type must be inferred");
    assert!(is_inferred_promise_instance(
        &db,
        inferred.resolve_type(&db, result_ty)
    ));
}

#[test]
fn test_infer_module_types_resolves_class_getter_properties() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            class Value {
                get text() {
                    return "value";
                }
            }

            export const result = new Value().text;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");

    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, result),
        "value"
    ));
}

#[test]
fn test_infer_module_types_widens_assignment_values() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export let boolean = false;
            function setBoolean() {
                boolean = true;
            }

            export let mixed = undefined;
            function setString() {
                mixed = "value";
            }
            function setNumber() {
                mixed = 123;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let boolean = inferred_binding_ty_by_name(&db, module, &inferred, "boolean")
        .expect("boolean type must be inferred");
    let mixed = inferred_binding_ty_by_name(&db, module, &inferred, "mixed")
        .expect("mixed type must be inferred");

    assert!(contains_inferred_boolean(
        &db,
        inferred.resolve_type(&db, boolean)
    ));
    let mixed = inferred.resolve_type(&db, mixed);
    assert!(contains_inferred_string_literal(&db, mixed, "value"));
    assert!(contains_inferred_number_literal(&db, mixed, "123"));
    assert!(contains_inferred_undefined(&db, mixed));
}

#[test]
fn test_infer_module_types_calls_through_function_type_aliases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type AsyncCallback = () => Promise<void>;
            declare const callback: AsyncCallback;
            export const result = callback();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let callback_ty = inferred_binding_ty_by_name(&db, module, &inferred, "callback")
        .expect("callback type must be inferred");
    let public_call_ty = infer_call_expression_type(&db, module, callback_ty, Vec::new());
    assert!(is_inferred_promise_instance(&db, public_call_ty));

    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");
    assert!(is_inferred_promise_instance(
        &db,
        inferred.resolve_type(&db, result_ty)
    ));
}

#[test]
fn test_infer_module_types_calls_generic_function_type_aliases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Factory<T> = () => Promise<T>;
            declare const makeString: Factory<string>;
            export const result = makeString();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");
    let result_ty = inferred.resolve_type(&db, result_ty);

    assert!(is_inferred_promise_with_type_parameter(
        &db,
        result_ty,
        |ty| is_inferred_string(&db, ty)
    ));
}

#[test]
fn test_infer_module_types_calls_generic_callable_interfaces() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Factory<T> {
                (): Promise<T>;
            }

            declare const makeString: Factory<string>;
            export const result = makeString();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");
    let result_ty = inferred.resolve_type(&db, result_ty);

    assert!(is_inferred_promise_with_type_parameter(
        &db,
        result_ty,
        |ty| is_inferred_string(&db, ty)
    ));
}

#[test]
fn test_infer_module_types_calls_nested_generic_callable_aliases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Callable<T> {
                (value: T): T;
            }

            type First<T> = Callable<T>;
            type Second<T> = First<T>;
            declare const call: Second<string>;
            export const result = call("value");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");

    let result_ty = inferred.resolve_type(&db, result_ty);
    assert!(
        is_inferred_string(&db, result_ty),
        "nested callable alias must return string, got {}",
        format_inferred_type(&db, result_ty)
    );
}

#[test]
fn test_infer_module_types_calls_nested_zero_argument_generic_callable_aliases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            interface Factory<T> {
                (): T;
            }

            type First<T> = Factory<T>;
            type Second<T> = First<T>;
            declare const makePromise: Second<Promise<void>>;
            export const result = makePromise();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");
    let result_ty = inferred.resolve_type(&db, result_ty);

    assert!(
        is_inferred_promise_instance(&db, result_ty),
        "nested zero-argument callable alias must return Promise<void>, got {}",
        format_inferred_type(&db, result_ty)
    );
}

#[test]
fn test_infer_module_types_calls_swr_style_mutator_interface() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type ScopedMutator<Data = any, T = Data> = (
                key: string,
                data?: T | Promise<T>,
            ) => Promise<T | undefined>;

            interface InternalConfiguration {
                mutate: ScopedMutator;
            }

            interface PublicConfiguration {
                errorRetryInterval: number;
            }

            type FullConfiguration = InternalConfiguration & PublicConfiguration;
            declare const useSWRConfig: () => FullConfiguration;
            const { mutate } = useSWRConfig();
            export const mutateResult = mutate("key");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let mutate_ty = inferred_binding_ty_by_name(&db, module, &inferred, "mutate")
        .expect("mutate type must be inferred");
    assert!(
        normalize_type(&db, module, mutate_ty)
            .callable_function(&db)
            .is_some()
    );

    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "mutateResult")
        .expect("mutateResult type must be inferred");
    assert!(is_inferred_promise_instance(
        &db,
        inferred.resolve_type(&db, result_ty)
    ));
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

    let source_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readSource")
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
fn test_normalize_type_reports_structural_step_boundaries() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for distinct_types in [1023, 1024, 1025] {
        let ty = inferred_typeof_chain(&db, distinct_types, InferredTypeData::String);
        let expected = if distinct_types <= 1024 {
            InferredTypeData::String
        } else {
            InferredTypeData::Unknown
        };
        assert_eq!(
            normalize_type(&db, module, ty),
            expected,
            "distinct types {distinct_types}"
        );
    }
}

#[test]
fn test_normalize_type_exhaustion_is_cached_and_invalidates_to_known() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let budget = NormalizationBudget::new(&db, module, 1025);

    assert_eq!(
        normalize_type_for_budget(&db, budget),
        InferredTypeData::Unknown
    );
    db.events.0.lock().unwrap().clear();
    assert_eq!(
        normalize_type_for_budget(&db, budget),
        InferredTypeData::Unknown
    );
    let events = std::mem::take(&mut *db.events.0.lock().unwrap());
    assert_function_query_was_not_run(&db, normalize_type_for_budget, budget, &events);

    salsa::Setter::to(budget.set_distinct_types(&mut db), 1024);
    assert_eq!(
        normalize_type_for_budget(&db, budget),
        InferredTypeData::String
    );
}

#[test]
fn test_normalize_type_resolves_local_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Result = Promise<void>;
            const callback = (): Result => Promise.resolve();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let callback_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "callback")
        .expect("callback binding type must be inferred");
    let normalized_ty = normalize_type(&db, index_module, callback_ty);
    let function = normalized_ty
        .callable_function(&db)
        .expect("callback must normalize to a function");
    assert_eq!(
        InferredType::new(&db, normalized_ty).function_returns_promise(),
        Some(true)
    );
    let InferredReturnType::Type(return_ty) = function.return_type(&db) else {
        panic!("callback return type must be inferred as a type");
    };

    assert!(
        return_ty.is_promise_instance(&db) == Some(true),
        "callback return type must normalize to Promise, got {return_ty:?}"
    );
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

    let tree_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readTree")
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
fn test_normalize_type_preserves_recursive_array_local_edge() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Tree = number | Tree[];

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

    let tree_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readTree")
        .expect("readTree return type must be inferred");
    let tree_index = local_type_id_of_instance(&db, tree_ty)
        .expect("readTree must return an instance of the local Tree type");
    let normalized_ty = normalize_type(&db, index_module, tree_ty);

    let InferredTypeData::Union(union) = normalized_ty else {
        panic!("recursive Tree type must normalize to a union, got {normalized_ty:?}");
    };
    let normalized_tree = format_inferred_type(&db, normalized_ty);
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
                    if instance.ty(&db).is_array_class(&db)
                    && instance.type_parameters(&db).iter().any(|parameter| {
                        matches!(
                            parameter,
                            InferredTypeData::Local(local)
                                if local.type_id(&db).index() == tree_index
                        )
                        || local_type_id_of_instance(&db, *parameter) == Some(tree_index)
                    })
            )
        }),
        "recursive Tree union must keep the recursive Array local edge: {normalized_tree}"
    );

    assert_inferred_type_snapshot(
        "test_normalize_type_preserves_recursive_array_local_edge",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_resolves_anonymous_default_class_export() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export default class {
                name: string;
            }
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
    let default_ty = match js_info
        .exports
        .get("default")
        .and_then(JsExport::as_own_export)
    {
        Some(JsOwnExport::Type(resolved_id)) => inferred
            .types
            .get(resolved_id.index())
            .copied()
            .expect("default export type must be inferred"),
        _ => panic!("default export must have a type"),
    };

    assert!(!js_info.raw_types.is_empty());
    assert!(inferred.named_type_ids.is_empty());

    let name_ty = inferred
        .find_member_type(&db, default_ty, "name")
        .expect("anonymous default class member must be inferred");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_anonymous_default_function_export() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export default function(): string {
                return "value";
            }
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
    let default_ty = match js_info
        .exports
        .get("default")
        .and_then(JsExport::as_own_export)
    {
        Some(JsOwnExport::Type(resolved_id)) => inferred
            .types
            .get(resolved_id.index())
            .copied()
            .expect("default export type must be inferred"),
        _ => panic!("default export must have a type"),
    };

    assert!(!js_info.raw_types.is_empty());
    assert!(inferred.named_type_ids.is_empty());
    assert_inferred_function_returns_string(&db, inferred.resolve_type(&db, default_ty));
}

#[test]
fn test_infer_module_types_resolves_imported_anonymous_default_class_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export default class {
                name: string;
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import Base from "./base.ts";

            export const value: Base = new Base();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "value")
        .expect("value binding type must be inferred");

    let name_ty = inferred
        .find_member_type(&db, value_ty, "name")
        .expect("imported anonymous default class member must be inferred");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_imported_anonymous_default_function() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export default function(): string {
                return "value";
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import readValue from "./base.ts";

            export { readValue };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readValue")
        .expect("readValue binding type must be inferred");

    assert_inferred_function_returns_string(&db, inferred.resolve_type(&db, read_value_ty));
}

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

    let value_type_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "valueType")
        .expect("valueType binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, value_type_ty),
        "number"
    ));

    let function_type_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "functionType")
            .expect("functionType binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, function_type_ty),
        "function"
    ));

    let unknown_type_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "unknownType")
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

    let numeric_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "numeric")
        .expect("numeric binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, numeric_ty)
    ));

    let textual_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "textual")
        .expect("textual binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, textual_ty)
    ));

    let unknown_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "unknown")
        .expect("unknown binding type must be inferred");
    assert_eq!(
        inferred.resolve_type(&db, unknown_ty),
        InferredTypeData::Unknown
    );

    let negative_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "negative")
        .expect("negative binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, negative_ty)
    ));

    let inverted_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "inverted")
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

    let tuple_first_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "tupleFirst")
        .expect("tupleFirst binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, tuple_first_ty)
    ));

    let array_first_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "arrayFirst")
        .expect("arrayFirst binding type must be inferred");
    let array_first_ty = inferred.resolve_type(&db, array_first_ty);
    assert!(contains_inferred_number(&db, array_first_ty));
    assert!(contains_inferred_undefined(&db, array_first_ty));

    let destructured_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "destructured")
        .expect("destructured binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, destructured_ty)
    ));

    let item_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "item")
        .expect("item binding type must be inferred");
    assert!(is_inferred_number(&db, inferred.resolve_type(&db, item_ty)));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_array_element_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_await_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const promised: Promise<string> = Promise.resolve("value");

            export async function consume() {
                const awaited = await promised;
                const primitive = await 1;
                return awaited;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let promised_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "promised")
        .expect("promised binding type must be inferred");
    let promised_ty = inferred.resolve_type(&db, promised_ty);
    assert!(
        is_inferred_promise_with_type_parameter(&db, promised_ty, |ty| is_inferred_string(&db, ty)),
        "expected promised to be Promise<string>, got {}",
        format_inferred_type(&db, promised_ty)
    );

    let awaited_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "awaited")
        .expect("awaited binding type must be inferred");
    let awaited_ty = inferred.resolve_type(&db, awaited_ty);
    assert!(
        is_inferred_string(&db, awaited_ty),
        "expected awaited to be string, got {}",
        format_inferred_type(&db, awaited_ty)
    );

    let primitive_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "primitive")
        .expect("primitive binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, primitive_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_await_expressions_on_build",
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

    let object_name_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "objectName")
        .expect("objectName binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, object_name_ty)
    ));

    let static_label_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "staticLabel")
        .expect("staticLabel binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, static_label_ty)
    ));

    let member_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "memberValue")
        .expect("memberValue binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, member_value_ty)
    ));

    let optional_value_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "optionalValue")
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

    let value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "text")
        .expect("text binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, value_ty)
    ));

    let numeric_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "numeric")
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

    let text_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "text")
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

    let box_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "box")
        .expect("box binding type must be inferred");
    assert!(matches!(
        inferred.resolve_type(&db, box_ty),
        InferredTypeData::InstanceOf(_)
    ));

    let value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "boxValue")
        .expect("boxValue binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, value_ty)
    ));

    let count_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "count")
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
        inferred_binding_ty_by_name(&db, index_module, &inferred, "explicitValue")
            .expect("explicitValue binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, explicit_value_ty)
    ));

    let inferred_value_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "inferredValue")
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

    let direct_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "directValue")
        .expect("directValue binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, direct_value_ty)
    ));

    let array_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "arrayValue")
        .expect("arrayValue binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, array_value_ty)
    ));

    let callback_value_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "callbackValue")
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
fn test_infer_module_types_resolves_promise_member_chain() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Parent {
                async returnsPromise(): Promise<string> {
                    return "value";
                }
            }

            export class Child extends Parent {}

            export const direct = new Child().returnsPromise();
            export const then = direct.then(() => {});
            export const finalResult = then.finally(() => {});
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    for name in ["direct", "then", "finalResult"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, &inferred, name)
            .expect("binding type must be inferred");
        let ty = inferred.resolve_type(&db, ty);
        assert!(
            is_inferred_promise_instance(&db, ty),
            "{name} must be a Promise, got {}",
            format_inferred_type(&db, ty)
        );
    }

    assert_inferred_type_snapshot(
        "test_infer_module_types_resolves_promise_member_chain",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_preserves_floating_promise_shapes() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type Cheating<T extends 1> = T extends 1 ? Promise<string> : Promise<string>;

            async function promiseLike(): Cheating<1> {
                return "value";
            }

            const sneakyObject = {
                get something() {
                    return new Promise((_, reject) => reject("value"));
                },
            };

            function wrapper<F extends (...args: any) => any>(fn: F): F {
                return fn;
            }

            async function doWork(): Promise<void> {}

            export const mappedAsync = [1, 2, 3].map(async (value) => value + 1);
            export const mappedPromise = [1, 2, 3].map((value) => Promise.resolve(value + 1));
            export const conditional = promiseLike();
            export const getter = sneakyObject.something;
            export const wrapped = wrapper(doWork)();
            export const maybeDoWork: typeof doWork | undefined = doWork;
            export const optional = maybeDoWork?.();
            export const globalChain = globalThis.Promise.reject("value").finally();

            await new Promise((resolve) => resolve("value"));
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    for name in ["mappedAsync", "mappedPromise"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, &inferred, name)
            .expect("array binding type must be inferred");
        let ty = normalize_type(&db, index_module, ty);
        assert!(
            is_inferred_array_of_promises(&db, ty),
            "{name} must be an array of Promises, got {}",
            format_inferred_type(&db, ty)
        );
    }

    for name in ["conditional", "getter", "wrapped"] {
        let ty = inferred_binding_ty_by_name(&db, index_module, &inferred, name)
            .expect("Promise binding type must be inferred");
        let ty = normalize_type(&db, index_module, ty);
        assert!(
            is_inferred_promise_instance(&db, ty),
            "{name} must be a Promise, got {}",
            format_inferred_type(&db, ty)
        );
    }
    let global_chain = inferred_binding_ty_by_name(&db, index_module, &inferred, "globalChain")
        .expect("globalChain binding type must be inferred");
    let global_chain = normalize_type(&db, index_module, global_chain);
    assert!(is_inferred_promise_instance(&db, global_chain));

    let optional = inferred_binding_ty_by_name(&db, index_module, &inferred, "optional")
        .expect("optional binding type must be inferred");
    let optional = normalize_type(&db, index_module, optional);
    let InferredTypeData::Union(optional) = optional else {
        panic!("optional call must preserve a Promise | undefined union, got {optional:?}");
    };
    assert!(
        optional
            .types(&db)
            .iter()
            .any(|ty| is_inferred_promise_instance(&db, *ty))
    );
    assert!(optional.types(&db).contains(&InferredTypeData::Undefined));

    let maybe_do_work = inferred_binding_ty_by_name(&db, index_module, &inferred, "maybeDoWork")
        .expect("maybeDoWork binding type must be inferred");
    let optional_call = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, maybe_do_work),
        Vec::new(),
    );
    let InferredTypeData::Union(optional_call) = optional_call else {
        panic!(
            "optional call query must preserve Promise | undefined, got {}",
            format_inferred_type(&db, optional_call)
        );
    };
    assert!(
        optional_call
            .types(&db)
            .iter()
            .any(|ty| is_inferred_promise_instance(&db, *ty))
    );
    assert!(
        optional_call
            .types(&db)
            .contains(&InferredTypeData::Undefined)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_preserves_floating_promise_shapes",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_await_promise_like_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class StringPromise extends Promise<string> {}

            export interface PromiseLike<T> {
                then(resolve: (value: T) => void): void;
            }

            export async function consume(
                subclass: StringPromise,
                like: PromiseLike<number>,
            ) {
                const awaitedSubclass = await subclass;
                const awaitedLike = await like;
                return awaitedSubclass;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let awaited_subclass_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "awaitedSubclass")
            .expect("awaitedSubclass binding type must be inferred");
    let awaited_subclass_ty = inferred.resolve_type(&db, awaited_subclass_ty);
    assert!(
        is_inferred_string(&db, awaited_subclass_ty),
        "awaitedSubclass must be string, got {}",
        format_inferred_type(&db, awaited_subclass_ty)
    );

    let awaited_like_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "awaitedLike")
        .expect("awaitedLike binding type must be inferred");
    let awaited_like_ty = inferred.resolve_type(&db, awaited_like_ty);
    assert!(
        is_inferred_number(&db, awaited_like_ty),
        "awaitedLike must be number, got {}",
        format_inferred_type(&db, awaited_like_ty)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_await_promise_like_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_evaluates_await_union_expressions_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export async function consume(value: Promise<Promise<string>> | number | undefined) {
                const awaited = await value;
                return awaited;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let awaited_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "awaited")
        .expect("awaited binding type must be inferred");
    let awaited_ty = inferred.resolve_type(&db, awaited_ty);
    assert!(contains_inferred_string(&db, awaited_ty));
    assert!(contains_inferred_number(&db, awaited_ty));
    assert!(contains_inferred_undefined(&db, awaited_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_evaluates_await_union_expressions_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_await_promise_resolution_boundaries_and_unresolved_paths() {
    let fs = MemoryFileSystem::default();
    let mut source = String::from(
        r#"
            interface PromiseLike<T> {
                then(resolve: (value: T) => void): void;
            }
        "#,
    );
    for index in 1..=64 {
        let promise_base = if index == 1 {
            "Promise<number>".to_string()
        } else {
            format!("PromiseChain{}", index - 1)
        };
        source.push_str(&format!(
            "class PromiseChain{index} extends {promise_base} {{}}\n"
        ));

        let like_base = if index == 1 {
            "PromiseLike<number>".to_string()
        } else {
            format!("PromiseLikeChain{}", index - 1)
        };
        source.push_str(&format!(
            "interface PromiseLikeChain{index} extends {like_base} {{}}\n"
        ));
    }
    source.push_str(
        r#"
            declare const promise63: PromiseChain62;
            declare const promise64: PromiseChain63;
            declare const promise65: PromiseChain64;
            declare const like63: PromiseLikeChain62;
            declare const like64: PromiseLikeChain63;
            declare const like65: PromiseLikeChain64;
            declare const unresolved: unknown;
            declare const complete: string;

            export const awaitedPromise63 = await promise63;
            export const awaitedPromise64 = await promise64;
            export const awaitedPromise65 = await promise65;
            export const awaitedLike63 = await like63;
            export const awaitedLike64 = await like64;
            export const awaitedLike65 = await like65;
            export const awaitedUnresolved = await unresolved;
            export const awaitedComplete = await complete;
        "#,
    );
    fs.insert("/src/index.ts".into(), source);

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for name in [
        "awaitedPromise63",
        "awaitedPromise64",
        "awaitedLike63",
        "awaitedLike64",
    ] {
        let ty = inferred_binding_ty_by_name(&db, module, &inferred, name)
            .expect("awaited binding type must be inferred");
        assert!(
            is_inferred_number(&db, inferred.resolve_type(&db, ty)),
            "{name} must resolve to number, got {}",
            format_inferred_type(&db, inferred.resolve_type(&db, ty))
        );
    }
    for name in ["awaitedPromise65", "awaitedLike65", "awaitedUnresolved"] {
        let ty = inferred_binding_ty_by_name(&db, module, &inferred, name)
            .expect("awaited binding type must be inferred");
        assert_eq!(
            inferred.resolve_type(&db, ty),
            InferredTypeData::Unknown,
            "{name} must discard partial resolution"
        );
    }
    let complete = inferred_binding_ty_by_name(&db, module, &inferred, "awaitedComplete")
        .expect("awaited binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, complete)
    ));
}

#[test]
fn test_infer_module_types_selects_call_overloads_by_parameter_types_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function reader(value: string): string;
            export function reader(value: number): number;
            export function reader(left: number, right: number): boolean;
            export function reader(..._args: [string] | [number] | [number, number]) {
                return undefined as string | number | boolean;
            }

            export const textual = reader("value");
            export const numeric = reader(1);
            export const args = [1, 2];
            export const spread = reader(...args);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let textual_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "textual")
        .expect("textual binding type must be inferred");
    let textual_ty = inferred.resolve_type(&db, textual_ty);
    assert!(
        is_inferred_string(&db, textual_ty),
        "textual must be string, got {}",
        format_inferred_type(&db, textual_ty)
    );

    let numeric_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "numeric")
        .expect("numeric binding type must be inferred");
    let numeric_ty = inferred.resolve_type(&db, numeric_ty);
    assert!(
        is_inferred_number(&db, numeric_ty),
        "numeric must be number, got {}",
        format_inferred_type(&db, numeric_ty)
    );

    let spread_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "spread")
        .expect("spread binding type must be inferred");
    let spread_ty = inferred.resolve_type(&db, spread_ty);
    assert!(
        is_inferred_boolean(&db, spread_ty),
        "spread must be boolean, got {}",
        format_inferred_type(&db, spread_ty)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_by_parameter_types_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_by_nominal_class_parameters_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Foo {}
            export class Bar {}

            export function select(value: Foo): string;
            export function select(value: Bar): number;
            export function select(_value: Foo | Bar): string | number {
                return undefined as string | number;
            }

            export const selected = select(new Bar());
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let selected_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "selected")
        .expect("selected binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, selected_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_by_nominal_class_parameters_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_for_subclass_arguments_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Base {}
            export class Derived extends Base {}
            export class Other {}

            export function select(value: Other): number;
            export function select(value: Base): string;
            export function select(_value: Other | Base): string | number {
                return undefined as string | number;
            }

            export const selected = select(new Derived());
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let selected_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "selected")
        .expect("selected binding type must be inferred");
    assert!(is_inferred_string(
        &db,
        inferred.resolve_type(&db, selected_ty)
    ));

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_for_subclass_arguments_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_selects_call_overloads_for_array_and_optional_tuple_spreads_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readArray(value: string): string;
            export function readArray(left: number, right: number): boolean;
            export function readArray(..._args: [string] | [number, number]) {
                return undefined as string | boolean;
            }

            export const numbers: number[] = [1, 2];
            export const arraySpread = readArray(...numbers);

            export function readTuple(value: string): string;
            export function readTuple(value: string, count: number): number;
            export function readTuple(..._args: [string, number?]) {
                return undefined as string | number;
            }

            export const optionalTuple: [string, number?] = ["value"];
            export const optionalTupleSpread = readTuple(...optionalTuple);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let array_spread_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "arraySpread")
        .expect("arraySpread binding type must be inferred");
    assert!(is_inferred_boolean(
        &db,
        inferred.resolve_type(&db, array_spread_ty)
    ));

    let optional_tuple_spread_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "optionalTupleSpread")
            .expect("optionalTupleSpread binding type must be inferred");
    let optional_tuple_spread_ty = inferred.resolve_type(&db, optional_tuple_spread_ty);
    assert!(
        is_inferred_string(&db, optional_tuple_spread_ty),
        "optionalTupleSpread must be string, got {}",
        format_inferred_type(&db, optional_tuple_spread_ty)
    );

    assert_inferred_type_snapshot(
        "test_infer_module_types_selects_call_overloads_for_array_and_optional_tuple_spreads_on_build",
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

    let choice_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "choice")
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

    let and_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "andValue")
        .expect("andValue binding type must be inferred");
    let and_value_ty = inferred.resolve_type(&db, and_value_ty);
    assert!(contains_inferred_string_literal(&db, and_value_ty, ""));
    assert!(contains_inferred_number(&db, and_value_ty));
    assert!(contains_inferred_undefined(&db, and_value_ty));

    let or_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "orValue")
        .expect("orValue binding type must be inferred");
    let or_value_ty = inferred.resolve_type(&db, or_value_ty);
    assert!(contains_inferred_string(&db, or_value_ty));
    assert!(contains_inferred_number(&db, or_value_ty));
    assert!(!contains_inferred_undefined(&db, or_value_ty));

    let nullish_value_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "nullishValue")
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

    let truthy_and_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "truthyAnd")
        .expect("truthyAnd binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, truthy_and_ty)
    ));

    let falsy_and_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "falsyAnd")
        .expect("falsyAnd binding type must be inferred");
    assert!(is_inferred_number_literal(
        &db,
        inferred.resolve_type(&db, falsy_and_ty),
        "0"
    ));

    let nullish_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "nullish")
        .expect("nullish binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, nullish_ty)
    ));

    let non_nullish_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "nonNullish")
        .expect("nonNullish binding type must be inferred");
    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, non_nullish_ty),
        "s"
    ));

    let conditional_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "conditional")
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
        inferred_binding_ty_by_name(&db, index_module, &inferred, "nullishResult")
            .expect("nullishResult binding type must be inferred");
    let nullish_result_ty = inferred.resolve_type(&db, nullish_result_ty);
    assert!(contains_inferred_string(&db, nullish_result_ty));
    assert!(!contains_inferred_null(&db, nullish_result_ty));

    let and_result_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "andResult")
        .expect("andResult binding type must be inferred");
    let and_result_ty = inferred.resolve_type(&db, and_result_ty);
    assert!(contains_inferred_number_literal(&db, and_result_ty, "0"));
    assert!(contains_inferred_boolean(&db, and_result_ty));

    let nullish_box_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "nullishBox")
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

    let own_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "own")
        .expect("own binding type must be inferred");
    assert!(is_inferred_number(&db, inferred.resolve_type(&db, own_ty)));

    let inherited_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "inherited")
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

    let a_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "a")
        .expect("a binding type must be inferred");
    assert!(is_inferred_string(&db, inferred.resolve_type(&db, a_ty)));

    let b_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "b")
        .expect("b binding type must be inferred");
    let b_ty = inferred.resolve_type(&db, b_ty);
    assert!(contains_inferred_number(&db, b_ty));
    assert!(contains_inferred_undefined(&db, b_ty));

    let rest_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "rest")
        .expect("rest binding type must be inferred");
    let rest_ty = inferred.resolve_type(&db, rest_ty);
    assert!(object_member_ty_by_name(&db, rest_ty, "a").is_none());
    assert!(object_member_ty_by_name(&db, rest_ty, "b").is_none());
    let (_, rest_c_ty) =
        object_member_ty_by_name(&db, rest_ty, "c").expect("rest must retain the c member");
    assert!(contains_inferred_boolean(&db, rest_c_ty));

    let head_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "head")
        .expect("head binding type must be inferred");
    let head_ty = inferred.resolve_type(&db, head_ty);
    assert!(contains_inferred_number(&db, head_ty));
    assert!(contains_inferred_undefined(&db, head_ty));

    let tail_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "tail")
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

    let a_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "a")
        .expect("a binding type must be inferred");
    assert!(is_inferred_number(&db, inferred.resolve_type(&db, a_ty)));

    let static_rest_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "staticRest")
        .expect("staticRest binding type must be inferred");
    let static_rest_ty = inferred.resolve_type(&db, static_rest_ty);
    assert!(object_member_ty_by_name(&db, static_rest_ty, "a").is_none());
    let (_, keep_ty) = object_member_ty_by_name(&db, static_rest_ty, "keep")
        .expect("static rest must retain keep");
    assert!(contains_inferred_boolean(&db, keep_ty));

    let inherited_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "inherited")
        .expect("inherited binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, inherited_ty)
    ));

    let instance_rest_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "instanceRest")
            .expect("instanceRest binding type must be inferred");
    let instance_rest_ty = inferred.resolve_type(&db, instance_rest_ty);
    assert!(object_member_ty_by_name(&db, instance_rest_ty, "inherited").is_none());
    let (_, own_ty) = object_member_ty_by_name(&db, instance_rest_ty, "own")
        .expect("instance rest must retain own");
    assert!(contains_inferred_string(&db, own_ty));

    let tuple_rest_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "tupleRest")
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

    let arrow_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "arrowValue")
        .expect("arrowValue binding type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, arrow_value_ty)
    ));

    let plain_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "plainValue")
        .expect("plainValue binding type must be inferred");
    assert_eq!(
        inferred.resolve_type(&db, plain_value_ty),
        InferredTypeData::Unknown
    );

    let no_parent_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "noParent")
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
    let direct = inferred_binding_ty_by_name(&db, module, &inferred, "direct")
        .expect("direct binding type must be inferred");
    let direct = normalize_type(&db, module, direct);
    assert!(
        is_inferred_promise_instance(&db, direct),
        "expected direct Promise, got {}",
        format_inferred_type(&db, direct)
    );
    let method = inferred_binding_ty_by_name(&db, module, &inferred, "method")
        .expect("method binding type must be inferred");
    let method = normalize_type(&db, module, method);
    let method_details = match method {
        InferredTypeData::Function(function) => match function.return_type(&db) {
            InferredReturnType::Type(ty) => format_inferred_type(&db, *ty),
            return_type => format!("{return_type:?}"),
        },
        _ => format_inferred_type(&db, method),
    };
    assert!(
        matches!(
            method,
            InferredTypeData::Function(function)
                if matches!(function.return_type(&db), InferredReturnType::Type(ty) if is_inferred_promise_instance(&db, *ty))
        ),
        "expected method returning Promise, got {method_details}"
    );
    let result = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result binding type must be inferred");

    let result = normalize_type(&db, module, result);
    let result_details = match result {
        InferredTypeData::InstanceOf(instance) => format!(
            "target={}, parameters={:?}",
            format_inferred_type(&db, instance.ty(&db)),
            instance
                .type_parameters(&db)
                .iter()
                .map(|ty| format_inferred_type(&db, *ty))
                .collect::<Vec<_>>()
        ),
        _ => format_inferred_type(&db, result),
    };
    assert!(
        is_inferred_promise_instance(&db, result),
        "expected Promise, got {result_details}"
    );
}

#[test]
fn test_infer_call_expression_type_resolves_imported_default_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export default function(): string {
                return "value";
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import readValue from "./base.ts";

            export const value = readValue();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readValue")
        .expect("readValue binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, read_value_ty),
        Vec::new(),
    );

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_resolves_annotated_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const readValue: () => string = () => "value";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readValue")
        .expect("readValue binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, read_value_ty),
        Vec::new(),
    );

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_resolves_callable_interface_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Reader {
                (): string;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Interface(interface)
                    if interface.name(&db).text() == "Reader"
                        && interface
                            .members(&db)
                            .iter()
                            .any(|member| member.kind.is_call_signature())
            )
        })
        .expect("Reader interface type must be inferred");
    let call_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_selects_callable_interface_overload_by_arity() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Reader {
                (): string;
                (value: number): number;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Interface(interface)
                    if interface.name(&db).text() == "Reader"
                        && interface
                            .members(&db)
                            .iter()
                            .filter(|member| member.kind.is_call_signature())
                            .count()
                            == 2
            )
        })
        .expect("Reader interface type must be inferred");

    let zero_arg_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());
    assert!(is_inferred_string(&db, zero_arg_ty));

    let one_arg_ty = infer_call_expression_type(
        &db,
        index_module,
        reader_ty,
        Vec::from([InferredTypeData::Number]),
    );
    assert!(is_inferred_number(&db, one_arg_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_callable_interface_overload_by_arity",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_selects_callable_interface_overload_with_optional_parameter_by_arity()
 {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Reader {
                (): string;
                (value?: number): number;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Interface(interface)
                    if interface.name(&db).text() == "Reader"
                        && interface
                            .members(&db)
                            .iter()
                            .filter(|member| member.kind.is_call_signature())
                            .count()
                            == 2
            )
        })
        .expect("Reader interface type must be inferred");

    let zero_arg_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());
    assert!(is_inferred_string(&db, zero_arg_ty));

    let one_arg_ty = infer_call_expression_type(
        &db,
        index_module,
        reader_ty,
        Vec::from([InferredTypeData::Number]),
    );
    assert!(is_inferred_number(&db, one_arg_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_callable_interface_overload_with_optional_parameter_by_arity",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_selects_callable_interface_overload_with_rest_parameter_by_arity()
 {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export interface Reader {
                (): string;
                (...values: number[]): number;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Interface(interface)
                    if interface.name(&db).text() == "Reader"
                        && interface
                            .members(&db)
                            .iter()
                            .filter(|member| member.kind.is_call_signature())
                            .count()
                            == 2
            )
        })
        .expect("Reader interface type must be inferred");

    let zero_arg_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());
    assert!(is_inferred_string(&db, zero_arg_ty));

    let many_arg_ty = infer_call_expression_type(
        &db,
        index_module,
        reader_ty,
        Vec::from([InferredTypeData::Number, InferredTypeData::Number]),
    );
    assert!(is_inferred_number(&db, many_arg_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_callable_interface_overload_with_rest_parameter_by_arity",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_resolves_callable_object_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export type Reader = {
                (): string;
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Object(object)
                    if object
                        .members(&db)
                        .iter()
                        .any(|member| member.kind.is_call_signature())
            )
        })
        .expect("Reader object type must be inferred");
    let call_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_selects_callable_object_overload_by_arity() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export type Reader = {
                (): string;
                (value: number): number;
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| {
            matches!(
                ty,
                InferredTypeData::Object(object)
                    if object
                        .members(&db)
                        .iter()
                        .filter(|member| member.kind.is_call_signature())
                        .count()
                        == 2
            )
        })
        .expect("Reader object type must be inferred");

    let zero_arg_ty = infer_call_expression_type(&db, index_module, reader_ty, Vec::new());
    assert!(is_inferred_string(&db, zero_arg_ty));

    let one_arg_ty = infer_call_expression_type(
        &db,
        index_module,
        reader_ty,
        Vec::from([InferredTypeData::Number]),
    );
    assert!(is_inferred_number(&db, one_arg_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_callable_object_overload_by_arity",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_reports_sequence_budget_uncertainty() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function select(...values: number[]): string;
            export function select(...values: any[]): number;
            export function select(..._values: any[]): string | number {
                return 0;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");

    for (argument_count, expected) in [
        (1023, InferredTypeData::String),
        (1024, InferredTypeData::Unknown),
        (1025, InferredTypeData::Unknown),
    ] {
        let call_ty = infer_call_expression_type(
            &db,
            module,
            select_ty,
            vec![InferredTypeData::Number; argument_count],
        );
        assert_eq!(call_ty, expected, "argument count {argument_count}");
    }
}

#[test]
fn test_infer_call_expression_type_reports_pair_budget_uncertainty() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function select(value: string): string;
            export function select(value: any): number;
            export function select(_value: any): string | number {
                return 0;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");

    for (depth, expected) in [
        (1023, InferredTypeData::Number),
        (1024, InferredTypeData::Unknown),
        (1025, InferredTypeData::Unknown),
    ] {
        let arg_ty = nested_instance_type(&db, depth, InferredTypeData::Number);
        let call_ty = infer_call_expression_type(&db, module, select_ty, vec![arg_ty]);
        assert_eq!(call_ty, expected, "instance depth {depth}");
    }
}

#[test]
fn test_infer_call_expression_type_reports_extends_budget_uncertainty() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Base {}
            export function select(value: Base): string;
            export function select(value: any): number;
            export function select(_value: any): string | number {
                return 0;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");
    let base_ty = inferred_binding_ty_by_name(&db, module, &inferred, "Base")
        .expect("Base type must be inferred");
    let mut class_ty = normalize_type(&db, module, base_ty);
    let mut arguments = Vec::new();
    for depth in 1..=1025 {
        class_ty = InferredTypeData::Class(InferredClass::new(
            &db,
            Vec::new().into_boxed_slice(),
            Some(class_ty),
            Vec::new().into_boxed_slice(),
            Vec::new().into_boxed_slice(),
            Some(Text::new_static("Derived")),
        ));
        if depth >= 1023 {
            arguments.push((
                depth,
                InferredTypeData::instance_of(&db, class_ty, Box::new([])),
            ));
        }
    }

    for ((depth, arg_ty), expected) in arguments.into_iter().zip([
        InferredTypeData::String,
        InferredTypeData::String,
        InferredTypeData::Unknown,
    ]) {
        let call_ty = infer_call_expression_type(&db, module, select_ty, vec![arg_ty]);
        assert_eq!(call_ty, expected, "extends depth {depth}");
    }
}

#[test]
fn test_infer_call_expression_type_keeps_earlier_definite_match() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function select(value: number): boolean;
            export function select(value: string): string;
            export function select(_value: number | string): boolean | string {
                return true;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");
    let call_ty =
        infer_call_expression_type(&db, module, select_ty, vec![InferredTypeData::Number]);

    assert_eq!(call_ty, InferredTypeData::Boolean);
}

#[test]
fn test_infer_call_expression_type_blocks_later_distinct_return_after_uncertainty() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function select(value: string): string;
            export function select(value: any): number;
            export function select(_value: any): string | number {
                return 0;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");
    let arg_ty = nested_instance_type(&db, 1024, InferredTypeData::Number);
    let call_ty = infer_call_expression_type(&db, module, select_ty, vec![arg_ty]);

    assert_eq!(call_ty, InferredTypeData::Unknown);
}

#[test]
fn test_infer_call_expression_type_treats_internal_unknown_argument_as_uncertain() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function select(value: string): Promise<string>;
            export function select(value: unknown): number;
            export function select(_value: unknown): Promise<string> | number {
                return 0;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");

    assert_eq!(
        infer_call_expression_type(&db, module, select_ty, vec![InferredTypeData::Unknown],),
        InferredTypeData::Unknown
    );
}

#[test]
fn test_infer_call_expression_type_treats_distinct_global_handles_as_uncertain() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function select(value: Array<unknown>): string;
            export function select(value: unknown): number;
            export function select(_value: unknown): string | number {
                return 0;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");
    let promise = InferredTypeData::instance_of(
        &db,
        InferredTypeData::promise_class(&db),
        Box::new([InferredTypeData::Number]),
    );

    assert_eq!(
        infer_call_expression_type(&db, module, select_ty, vec![promise]),
        InferredTypeData::Unknown
    );
}

#[test]
fn test_infer_call_expression_type_requires_every_argument_union_variant() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function read(value: string): Promise<void>;
            export function read(value: string | number): number;
            export function read(_value: string | number): Promise<void> | number {
                return 0;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let read_ty = inferred_overload_ty_by_name(&db, module, &inferred, "read")
        .expect("read overload type must be inferred");
    let argument = InferredTypeData::Union(InferredUnion::new(
        &db,
        Vec::from([InferredTypeData::String, InferredTypeData::Number]).into_boxed_slice(),
    ));
    let result = infer_call_expression_type(&db, module, read_ty, vec![argument]);

    assert!(
        is_inferred_number(&db, normalize_type(&db, module, result)),
        "expected number, got {result:?}"
    );
}

#[test]
fn test_infer_call_expression_type_checks_generic_parameter_constraints() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function read<T extends string>(value: T): Promise<void>;
            export function read(value: number): number;
            export function read(_value: string | number): Promise<void> | number {
                return 0;
            }
            export const result = read(1);
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result binding type must be inferred");

    assert!(is_inferred_number(&db, normalize_type(&db, module, result)));
}

#[test]
fn test_infer_constructor_argument_type_selects_overload_by_arity() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export class Consumer {
                constructor(callback: () => void);
                constructor(callback: () => Promise<void>, marker: number);
                constructor(_callback: (() => void) | (() => Promise<void>), _marker?: number) {}
            }
            export const callback = async () => {};
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let consumer = inferred_binding_ty_by_name(&db, module, &inferred, "Consumer")
        .expect("Consumer binding type must be inferred");
    let resolved_consumer = inferred.resolve_type(&db, consumer);
    let InferredTypeData::Class(class) = resolved_consumer else {
        panic!("expected class, got {resolved_consumer:?}");
    };
    assert_eq!(class.members(&db).len(), 3, "{:?}", class.members(&db));
    let callback = inferred_binding_ty_by_name(&db, module, &inferred, "callback")
        .expect("callback binding type must be inferred");
    let input = CallArgumentTypeInput::new(
        &db,
        resolved_consumer,
        Vec::from([InferredCallArgumentType::Argument(
            inferred.resolve_type(&db, callback),
        )])
        .into_boxed_slice(),
        0,
    );
    let expected = infer_constructor_argument_type(&db, input)
        .expect("constructor expected type must be inferred");

    assert!(
        InferredType::new(&db, expected).function_returns_void(),
        "expected void callback, got {expected:?}"
    );
}

#[test]
fn test_infer_call_expression_type_rejects_over_budget_union_frontier() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function select(value: string): string;
            export function select(value: unknown): number;
            export function select(_value: unknown): string | number {
                return 0;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");
    let wide_union = InferredTypeData::Union(InferredUnion::new(
        &db,
        (0..1025)
            .map(|index| {
                if index % 2 == 0 {
                    InferredTypeData::String
                } else {
                    InferredTypeData::Number
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    ));

    assert_eq!(
        infer_call_expression_type(&db, module, select_ty, vec![wide_union]),
        InferredTypeData::Unknown
    );
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
    let result = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result binding type must be inferred");

    assert!(is_inferred_promise_instance(
        &db,
        normalize_type(&db, module, result)
    ));
}

#[test]
fn test_member_lookup_expands_global_instance_target() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        "export const promise: Promise<number> = Promise.resolve(1);".to_string(),
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let promise = inferred_binding_ty_by_name(&db, module, &inferred, "promise")
        .expect("promise binding type must be inferred");

    assert!(
        inferred
            .find_member_type(&db, inferred.resolve_type(&db, promise), "then")
            .is_some()
    );
}

#[test]
fn test_member_lookup_rejects_over_budget_union_frontier() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let wide_union = InferredTypeData::Union(InferredUnion::new(
        &db,
        vec![InferredTypeData::Number; 1025].into_boxed_slice(),
    ));

    assert_eq!(
        inferred.find_member_type(&db, wide_union, "missing"),
        Some(InferredTypeData::Unknown)
    );
}

#[test]
fn test_infer_call_expression_type_rejects_partial_call_signature_set_repeatedly() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let input = IndeterminateCallSignature::new(&db, module, true);

    assert_eq!(
        infer_call_with_indeterminate_signature(&db, input),
        InferredTypeData::Unknown
    );
    db.events.0.lock().unwrap().clear();
    assert_eq!(
        infer_call_with_indeterminate_signature(&db, input),
        InferredTypeData::Unknown
    );
    let events = std::mem::take(&mut *db.events.0.lock().unwrap());
    assert_function_query_was_not_run(&db, infer_call_with_indeterminate_signature, input, &events);
}

#[test]
fn test_infer_call_expression_type_invalidates_indeterminate_call_signature() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let input = IndeterminateCallSignature::new(&db, module, true);

    assert_eq!(
        infer_call_with_indeterminate_signature(&db, input),
        InferredTypeData::Unknown
    );
    salsa::Setter::to(input.set_is_indeterminate(&mut db), false);
    assert_eq!(
        infer_call_with_indeterminate_signature(&db, input),
        InferredTypeData::Number
    );
}

#[test]
fn test_infer_call_expression_type_bypasses_matching_for_single_callable_object_signature() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export type Reader = {
                (value: string): string;
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let reader_ty = inferred
        .types
        .iter()
        .copied()
        .find(|ty| matches!(ty, InferredTypeData::Object(object) if object.members(&db).iter().any(|member| member.kind.is_call_signature())))
        .expect("Reader object type must be inferred");
    let arg_ty = nested_instance_type(&db, 1025, InferredTypeData::Number);
    let call_ty = infer_call_expression_type(&db, module, reader_ty, vec![arg_ty]);

    assert_eq!(call_ty, InferredTypeData::String);
}

#[test]
fn test_infer_call_expression_type_bypasses_matching_for_direct_function() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function read(value: string): string {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let read_ty = inferred_binding_ty_by_name(&db, module, &inferred, "read")
        .expect("read function type must be inferred");
    let arg_ty = nested_instance_type(&db, 1025, InferredTypeData::Number);
    let call_ty = infer_call_expression_type(
        &db,
        module,
        inferred.resolve_type(&db, read_ty),
        vec![arg_ty],
    );

    assert_eq!(call_ty, InferredTypeData::String);
}

#[test]
fn test_generic_replacement_budget_boundaries_for_call_query() {
    const LIMIT: usize = 64;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    for steps in [LIMIT - 1, LIMIT, LIMIT + 1] {
        let (callee, argument) = generic_call_types(&db, steps);
        let result = infer_call_expression_type_query(&db, module, callee, &[argument]);
        let expected = if steps <= LIMIT {
            InferredTypeData::Number
        } else {
            InferredTypeData::Unknown
        };
        assert_eq!(result, expected, "replacement steps {steps}");
    }
}

#[test]
fn test_generic_replacement_exhaustion_invalidates_after_input_replacement() {
    const LIMIT: usize = 64;

    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let budget = GenericReplacementBudget::new(&db, LIMIT + 1);
    assert_eq!(
        infer_generic_call_for_budget(&db, module, budget),
        InferredTypeData::Unknown
    );

    salsa::Setter::to(budget.set_steps(&mut db), LIMIT - 1);
    assert_eq!(
        infer_generic_call_for_budget(&db, module, budget),
        InferredTypeData::Number
    );
}

#[test]
fn test_member_substitution_reports_step_boundaries_without_partial_results() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export {};".to_string());
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for distinct_types in [1023, 1024, 1025] {
        let instance = generic_interface_instance_with_member(&db, distinct_types);
        let result = inferred
            .find_member_type(&db, instance, "target")
            .expect("the member must be present");
        let expected = if distinct_types <= 1024 {
            InferredTypeData::Number
        } else {
            InferredTypeData::Unknown
        };
        assert_eq!(
            normalize_type(&db, module, result),
            expected,
            "distinct types {distinct_types}"
        );
    }
}

#[test]
fn test_infer_call_expression_type_selects_function_declaration_overload_by_callback_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function bestEffort<T>(cb: () => Promise<T>): Promise<T | undefined>;
            export function bestEffort<T>(cb: () => T): T | undefined;
            export function bestEffort<T>(cb: (() => T) | (() => Promise<T>)) {
                return cb();
            }

            export function readPromise(): Promise<string> {
                return Promise.resolve("value");
            }

            export function readString(): string {
                return "value";
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let best_effort_ty = inferred_overload_ty_by_name(&db, index_module, &inferred, "bestEffort")
        .expect("bestEffort overload type must be inferred");
    let read_promise_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readPromise")
        .expect("readPromise binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readString")
        .expect("readString binding type must be inferred");

    let promise_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_promise_ty)]),
    );
    assert!(
        promise_result_ty.is_promise_instance(&db) == Some(true),
        "promise callback overload must return a Promise, got {promise_result_ty:?}",
    );
    let InferredTypeData::InstanceOf(instance) = promise_result_ty else {
        panic!("promise callback overload must return a Promise instance");
    };
    assert!(
        instance
            .type_parameters(&db)
            .iter()
            .any(|ty| contains_inferred_string(&db, *ty)),
        "promise callback overload must substitute the callback return type"
    );

    let sync_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_string_ty)]),
    );
    assert_ne!(sync_result_ty, InferredTypeData::Unknown);
    assert!(
        sync_result_ty.is_promise_instance(&db) == Some(false),
        "sync callback overload must not return a Promise, got {sync_result_ty:?}",
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_function_declaration_overload_by_callback_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_selects_imported_function_declaration_overload_by_callback_return_type()
 {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/functions.ts".into(),
        r#"
            export function bestEffort<T>(cb: () => Promise<T>): Promise<T | undefined>;
            export function bestEffort<T>(cb: () => T): T | undefined;
            export function bestEffort<T>(cb: (() => T) | (() => Promise<T>)) {
                return cb();
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { bestEffort } from "./functions.ts";

            export function readPromise(): Promise<string> {
                return Promise.resolve("value");
            }

            export function readString(): string {
                return "value";
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/functions.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let best_effort_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "bestEffort")
        .expect("bestEffort import type must be inferred");
    let read_promise_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readPromise")
        .expect("readPromise binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readString")
        .expect("readString binding type must be inferred");
    let best_effort_ty = inferred.resolve_type(&db, best_effort_ty);

    let promise_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_promise_ty)]),
    );
    assert!(
        promise_result_ty.is_promise_instance(&db) == Some(true),
        "promise callback overload must return a Promise, got {promise_result_ty:?}",
    );
    let InferredTypeData::InstanceOf(instance) = promise_result_ty else {
        panic!("promise callback overload must return a Promise instance");
    };
    assert!(
        instance
            .type_parameters(&db)
            .iter()
            .any(|ty| contains_inferred_string(&db, *ty)),
        "promise callback overload must substitute the callback return type"
    );

    let sync_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_string_ty)]),
    );
    assert_ne!(sync_result_ty, InferredTypeData::Unknown);
    assert!(
        sync_result_ty.is_promise_instance(&db) == Some(false),
        "sync callback overload must not return a Promise, got {sync_result_ty:?}",
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_selects_imported_function_declaration_overload_by_callback_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_direct_generic_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function identity<T>(value: T): T {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let identity_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "identity")
        .expect("identity binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, identity_ty),
        Vec::from([InferredTypeData::Number]),
    );

    assert!(is_inferred_number(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_deduplicates_substituted_union_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function maybeString<T>(value: T): T | string {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let maybe_string_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "maybeString")
        .expect("maybeString binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, maybe_string_ty),
        Vec::from([InferredTypeData::String]),
    );

    assert!(is_inferred_string(&db, call_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_deduplicates_substituted_union_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_callback_generic_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function run<T>(cb: () => T): T {
                return cb();
            }

            export function readString(): string {
                return "value";
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let run_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "run")
        .expect("run binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readString")
        .expect("readString binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, run_ty),
        Vec::from([inferred.resolve_type(&db, read_string_ty)]),
    );

    assert!(is_inferred_string(&db, call_ty));
}

#[test]
fn test_infer_call_expression_type_substitutes_nested_generic_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function wrap<T>(value: T): Promise<T> {
                return Promise.resolve(value);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let wrap_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "wrap")
        .expect("wrap binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, wrap_ty),
        Vec::from([InferredTypeData::Number]),
    );
    let InferredTypeData::InstanceOf(instance) = call_ty else {
        panic!("wrap must return a Promise instance, got {call_ty:?}");
    };

    assert_eq!(call_ty.is_promise_instance(&db), Some(true));
    assert!(
        instance
            .type_parameters(&db)
            .iter()
            .any(|ty| is_inferred_number(&db, *ty))
    );
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_nested_generic_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_in_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function wrap<T>(value: T): () => T {
                return () => value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let wrap_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "wrap")
        .expect("wrap binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, wrap_ty),
        Vec::from([InferredTypeData::Number]),
    );

    let InferredTypeData::Function(function) = call_ty else {
        panic!("wrap must return a function, got {call_ty:?}");
    };
    let InferredReturnType::Type(return_ty) = function.return_type(&db) else {
        panic!("nested function return type must be inferred as a type");
    };
    assert!(
        is_inferred_number(&db, *return_ty),
        "nested function return type must substitute T with number, got {return_ty:?}"
    );
}

#[test]
fn test_infer_call_expression_type_preserves_shadowed_nested_function_generic() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function makeIdentity<T>(value: T): <T>(value: T) => T {
                return value => value;
            }

            const identity = makeIdentity(1);
            export const result = identity("value");
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");

    let result_ty = inferred.resolve_type(&db, result_ty);
    assert!(
        is_inferred_string_literal(&db, result_ty, "value"),
        "shadowed nested generic must return the argument literal, got {}",
        format_inferred_type(&db, result_ty)
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
    let result_ty = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");

    let result_ty = inferred.resolve_type(&db, result_ty);
    assert!(
        is_inferred_string_literal(&db, result_ty, "value"),
        "shadowed method generic must return the argument literal, got {}",
        format_inferred_type(&db, result_ty)
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_inside_promise_union_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function boxed<T>(value: T): Promise<T | number> {
                return Promise.resolve(value as T | number);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let boxed_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "boxed")
        .expect("boxed binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, boxed_ty),
        Vec::from([InferredTypeData::String]),
    );

    assert!(is_inferred_promise_with_type_parameter(
        &db,
        call_ty,
        |ty| { contains_inferred_string(&db, ty) && contains_inferred_number(&db, ty) }
    ));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_inside_promise_union_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_multiple_generics_inside_union_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function pair<T, U>(left: T, right: U): Promise<T | U> {
                return Promise.resolve(left as T | U);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let pair_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "pair")
        .expect("pair binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, pair_ty),
        Vec::from([InferredTypeData::String, InferredTypeData::Number]),
    );

    assert!(is_inferred_promise_with_type_parameter(
        &db,
        call_ty,
        |ty| { contains_inferred_string(&db, ty) && contains_inferred_number(&db, ty) }
    ));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_multiple_generics_inside_union_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_inside_union_with_promise_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function maybePromise<T>(value: T): T | Promise<T> {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let maybe_promise_ty =
        inferred_binding_ty_by_name(&db, index_module, &inferred, "maybePromise")
            .expect("maybePromise binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, maybe_promise_ty),
        Vec::from([InferredTypeData::String]),
    );
    let InferredTypeData::Union(union) = call_ty else {
        panic!("maybePromise must return a union, got {call_ty:?}");
    };

    assert!(
        union
            .types(&db)
            .iter()
            .any(|ty| contains_inferred_string(&db, *ty))
    );
    assert!(union.types(&db).iter().any(|ty| {
        is_inferred_promise_with_type_parameter(&db, *ty, |ty| contains_inferred_string(&db, ty))
    }));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_inside_union_with_promise_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_inside_intersection_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            type WithName = {
                name: string;
            };

            export function withName<T>(value: T): T & WithName {
                return value as T & WithName;
            }

            export function readValue(): { value: number } {
                return { value: 1 };
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let with_name_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "withName")
        .expect("withName binding type must be inferred");
    let read_value_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readValue")
            .expect("readValue return type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, with_name_ty),
        Vec::from([read_value_ty]),
    );
    let InferredTypeData::Object(object) = call_ty else {
        panic!("withName call must return a normalized object, got {call_ty:?}");
    };
    assert_eq!(object.members(&db).len(), 2);

    let name_ty = inferred
        .find_member_type(&db, call_ty, "name")
        .expect("normalized call return must expose WithName.name");
    assert!(is_inferred_string(&db, name_ty));

    let value_ty = inferred
        .find_member_type(&db, call_ty, "value")
        .expect("normalized call return must expose readValue.value");
    assert!(is_inferred_number(&db, value_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_inside_intersection_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_substitutes_generic_from_callback_promise_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function unwrap<T>(cb: () => Promise<T>): T {
                return undefined as T;
            }

            export function readNumber(): Promise<number> {
                return Promise.resolve(1);
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let unwrap_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "unwrap")
        .expect("unwrap binding type must be inferred");
    let read_number_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readNumber")
        .expect("readNumber binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        index_module,
        inferred.resolve_type(&db, unwrap_ty),
        Vec::from([inferred.resolve_type(&db, read_number_ty)]),
    );

    assert!(is_inferred_number(&db, call_ty));
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_substitutes_generic_from_callback_promise_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_suppresses_indeterminate_callback_promise_selection() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function select(callback: () => Promise<void>): number;
            export function select(callback: () => void): string;
            export function select(_callback: () => unknown): number | string {
                return 1;
            }

            export function unknownCallback(): unknown {
                return undefined;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let select_ty = inferred_overload_ty_by_name(&db, module, &inferred, "select")
        .expect("select overload type must be inferred");
    let callback_ty = inferred_binding_ty_by_name(&db, module, &inferred, "unknownCallback")
        .expect("callback type must be inferred");

    assert_eq!(
        infer_call_expression_type(
            &db,
            module,
            select_ty,
            Vec::from([inferred.resolve_type(&db, callback_ty)]),
        ),
        InferredTypeData::Unknown
    );
}

#[test]
fn test_infer_call_expression_type_resolves_union_function_return_type() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readString(): string {
                return "value";
            }

            export function readNumber(): number {
                return 1;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readString")
        .expect("readString binding type must be inferred");
    let read_number_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "readNumber")
        .expect("readNumber binding type must be inferred");
    let callee_ty = InferredTypeData::Union(InferredUnion::new(
        &db,
        Vec::from([
            inferred.resolve_type(&db, read_string_ty),
            inferred.resolve_type(&db, read_number_ty),
        ])
        .into_boxed_slice(),
    ));
    let call_ty = infer_call_expression_type(&db, index_module, callee_ty, Vec::new());
    let InferredTypeData::Union(union) = call_ty else {
        panic!("union function call must return a union, got {call_ty:?} from {callee_ty:?}");
    };

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
    assert_inferred_type_snapshot(
        "test_infer_call_expression_type_resolves_union_function_return_type",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_call_expression_type_poisoned_by_indeterminate_union_variants_in_both_orders() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        "export function known(): number { return 1; }".to_string(),
    );
    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let known = inferred_binding_ty_by_name(&db, module, &inferred, "known")
        .expect("known function type must be inferred");
    let known = inferred.resolve_type(&db, known);

    for indeterminate in [InferredTypeData::Unknown, InferredTypeData::Number] {
        for variants in [[known, indeterminate], [indeterminate, known]] {
            let callee = InferredTypeData::Union(InferredUnion::new(
                &db,
                Vec::from(variants).into_boxed_slice(),
            ));
            assert_eq!(
                infer_call_expression_type_query(&db, module, callee, &[]),
                InferredTypeData::Unknown
            );
            assert_eq!(
                infer_call_expression_type_query(&db, module, callee, &[]),
                InferredTypeData::Unknown
            );
        }
    }
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

    let value_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readValue")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readBoolean")
            .expect("readBoolean return type must be inferred");
    assert_eq!(boolean_ty, InferredTypeData::Boolean);

    let string_literal_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readStringLiteral")
            .expect("readStringLiteral return type must be inferred");
    assert_eq!(string_literal_ty, InferredTypeData::String);

    let number_literal_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readNumberLiteral")
            .expect("readNumberLiteral return type must be inferred");
    assert_eq!(number_literal_ty, InferredTypeData::Number);

    let bigint_literal_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readBigIntLiteral")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readCombined")
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

    let string_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readString")
        .expect("readString return type must be inferred");
    assert_eq!(string_ty, InferredTypeData::String);

    let never_ty = inferred_function_return_ty_by_name(&db, index_module, &inferred, "readNever")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readCombined")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readValue")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readFunction")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readCallableObject")
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
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readPrimitive")
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
fn test_infer_module_types_merges_class_instance_intersections_on_build() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export function readPromiseObject(
                value: Promise<string> & { value: number },
            ): Promise<string> & { value: number } {
                return value;
            }
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let promise_object_ty =
        inferred_function_return_ty_by_name(&db, index_module, &inferred, "readPromiseObject")
            .expect("readPromiseObject return type must be inferred");
    let promise_object_ty = normalize_type(&db, index_module, promise_object_ty);
    let InferredTypeData::Intersection(intersection) = promise_object_ty else {
        panic!("readPromiseObject must preserve the intersection, got {promise_object_ty:?}");
    };
    assert!(
        intersection
            .types(&db)
            .iter()
            .any(|ty| is_inferred_promise_instance(&db, *ty))
    );

    let value_ty = inferred
        .find_member_type(&db, promise_object_ty, "value")
        .expect("merged class instance must expose value");
    assert!(is_inferred_number(&db, value_ty));

    assert_inferred_type_snapshot(
        "test_infer_module_types_merges_class_instance_intersections_on_build",
        &db,
        &fs,
    );
}

#[test]
fn test_infer_module_types_resolves_imported_local_handle_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/types.ts".into(),
        r#"
            export class Foo {
                name: string;

                static create(): Foo {
                    return new Foo();
                }
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { Foo } from "./types.ts";

            export const value: Foo = Foo.create();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/types.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let foo_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "Foo")
        .expect("Foo import type must be inferred");

    let create_ty = inferred
        .find_member_type(&db, foo_ty, "create")
        .expect("Foo.create must be found through the imported local handle");
    let InferredTypeData::Function(create_function) = create_ty else {
        panic!("Foo.create must be a function");
    };
    let InferredReturnType::Type(return_ty) = create_function.return_type(&db) else {
        panic!("Foo.create return type must be a type");
    };

    let name_ty = inferred
        .find_member_type(&db, *return_ty, "name")
        .expect("Foo.create().name must be found through the imported local handle");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_resolves_imported_inherited_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export class Base {
                name: string;

                static label(): string {
                    return "base";
                }
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { Base } from "./base.ts";

            export class Derived extends Base {
                value: number;
            }

            export const derived: Derived = new Derived();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let derived_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "derived")
        .expect("derived binding type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived instance must inherit imported Base.name");
    assert!(is_inferred_string(&db, name_ty));

    let derived_class_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "Derived")
        .expect("Derived class type must be inferred");
    let label_ty = inferred
        .find_member_type(&db, derived_class_ty, "label")
        .expect("Derived class must inherit imported Base.label");
    assert!(matches!(label_ty, InferredTypeData::Function(_)));
}

#[test]
fn test_infer_module_types_resolves_imported_interface_extends_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"
            export interface Base {
                name: string;
            }
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import type { Base } from "./base.ts";

            export interface Derived extends Base {
                value: number;
            }

            export const derived: Derived = {
                name: "derived",
                value: 1,
            };
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/base.ts", "/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let derived_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "derived")
        .expect("derived binding type must be inferred");

    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived interface must inherit imported Base.name");
    assert!(is_inferred_string(&db, name_ty));
}

#[test]
fn test_infer_module_types_is_memoized() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const value: string = "value";
        "#,
    );

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");

    let first = infer_module_types(&db, index_module).expect("types must be inferred");
    let first_ptr = std::sync::Arc::as_ptr(&first) as usize;
    drop(first);
    db.clear_salsa_events();
    let second = infer_module_types(&db, index_module).expect("types must be inferred");
    let second_ptr = std::sync::Arc::as_ptr(&second) as usize;
    drop(second);
    let events = db.take_salsa_events();

    assert_eq!(first_ptr, second_ptr);
    assert_function_query_was_not_run(&db, infer_module_types, index_module, &events);
}

#[test]
fn test_infer_module_types_invalidates_changed_dependencies_only() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/base.ts".into(),
        r#"export const value: string = "value";"#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { value } from "./base.ts";
            export const result = value;
        "#,
    );
    fs.insert(
        "/src/unrelated.ts".into(),
        r#"export const unrelated = true;"#,
    );

    let mut db = build_js_test_module_db(
        &fs,
        &["/src/base.ts", "/src/index.ts", "/src/unrelated.ts"],
        true,
    );
    let base_module = db
        .module_for_path(Utf8Path::new("/src/base.ts"))
        .expect("base module must exist");
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");
    let unrelated_module = db
        .module_for_path(Utf8Path::new("/src/unrelated.ts"))
        .expect("unrelated module must exist");
    let _ = infer_module_types(&db, index_module);

    fs.insert("/src/base.ts".into(), r#"export const value: number = 1;"#);
    let base_kind = resolve_js_module_kind_for_test(&fs, "/src/base.ts", true);
    salsa::Setter::to(base_module.set_kind(&mut db), base_kind);
    db.clear_salsa_events();

    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "result")
        .expect("result type must be inferred");
    assert!(is_inferred_number(
        &db,
        inferred.resolve_type(&db, result_ty)
    ));
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, infer_module_types, index_module, &events);

    fs.insert(
        "/src/unrelated.ts".into(),
        r#"export const unrelated = false;"#,
    );
    let unrelated_kind = resolve_js_module_kind_for_test(&fs, "/src/unrelated.ts", true);
    salsa::Setter::to(unrelated_module.set_kind(&mut db), unrelated_kind);
    db.clear_salsa_events();
    let _ = infer_module_types(&db, index_module);
    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, infer_module_types, index_module, &events);
}

#[test]
fn test_infer_module_types_invalidates_when_imported_modules_are_added_or_removed() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import { value } from "./b.ts";
            export const result = value;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"export const value: string = "value";"#,
    );

    let mut db = build_js_test_module_db(&fs, &["/src/a.ts"], true);
    let a_module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");

    let inferred = infer_module_types(&db, a_module).expect("a types must be inferred");
    let result = inferred_binding_ty_by_name(&db, a_module, &inferred, "result")
        .expect("result type must be present");
    assert_eq!(
        inferred.resolve_type(&db, result),
        InferredTypeData::Unknown
    );
    drop(inferred);

    let b_path = Utf8PathBuf::from("/src/b.ts");
    let b_kind = resolve_js_module_kind_for_test(&fs, b_path.as_str(), true);
    let b_module = ModuleInfo::new(&db, b_path.clone(), b_kind);
    db.insert_module(b_path.clone(), b_module);

    let inferred = infer_module_types(&db, a_module).expect("a types must be inferred");
    let result = inferred_binding_ty_by_name(&db, a_module, &inferred, "result")
        .expect("result type must be present");
    assert!(is_inferred_string(&db, inferred.resolve_type(&db, result)));
    drop(inferred);

    db.remove_module(&b_path);

    let inferred = infer_module_types(&db, a_module).expect("a types must be inferred");
    let result = inferred_binding_ty_by_name(&db, a_module, &inferred, "result")
        .expect("result type must be present");
    assert_eq!(
        inferred.resolve_type(&db, result),
        InferredTypeData::Unknown
    );
}

#[test]
fn test_infer_module_types_backdates_equal_output() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const value: string = "value";
        "#,
    );

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let expression_count = inferred_expression_count(&db, index_module);
    assert!(expression_count > 0);

    fs.insert(
        "/src/index.ts".into(),
        r#"
            export const value: string = "value";
            // This changes the module input, but not the inferred types.
        "#,
    );
    let module_kind = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(index_module.set_kind(&mut db), module_kind);

    db.clear_salsa_events();
    assert_eq!(
        inferred_expression_count(&db, index_module),
        expression_count
    );
    let events = db.take_salsa_events();

    assert_function_query_was_run(&db, infer_module_types, index_module, &events);
    assert_function_query_was_not_run(&db, inferred_expression_count, index_module, &events);
}

#[test]
fn test_infer_module_types_updates_replaced_range_map_keys() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/index.ts".into(),
        r#"export const value: string = "value";"#,
    );

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let first = infer_module_types(&db, index_module).expect("types must be inferred");
    let mut first_expression_ranges = first.expressions.keys().copied().collect::<Vec<_>>();
    let mut first_binding_ranges = first.binding_type_data.keys().copied().collect::<Vec<_>>();
    first_expression_ranges.sort_unstable();
    first_binding_ranges.sort_unstable();
    drop(first);

    fs.insert(
        "/src/index.ts".into(),
        r#"// Shift every syntax range.
export const value: number = 1;"#,
    );
    let replacement = resolve_js_module_kind_for_test(&fs, "/src/index.ts", true);
    salsa::Setter::to(index_module.set_kind(&mut db), replacement);

    let updated = infer_module_types(&db, index_module).expect("updated types must be inferred");
    let mut updated_expression_ranges = updated.expressions.keys().copied().collect::<Vec<_>>();
    let mut updated_binding_ranges = updated
        .binding_type_data
        .keys()
        .copied()
        .collect::<Vec<_>>();
    updated_expression_ranges.sort_unstable();
    updated_binding_ranges.sort_unstable();
    assert_ne!(first_expression_ranges, updated_expression_ranges);
    assert_ne!(first_binding_ranges, updated_binding_ranges);

    let value = inferred_binding_ty_by_name(&db, index_module, &updated, "value")
        .expect("updated value type must be inferred");
    assert!(is_inferred_number(&db, updated.resolve_type(&db, value)));
}

#[test]
fn test_infer_module_types_resolves_react_export_equals_namespace() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/node_modules/@types/react/index.d.ts".into(),
        include_bytes!("../../biome_resolver/tests/fixtures/resolver_cases_5/node_modules/@types/react/index.d.ts")
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"import { useCallback } from "react";

        const fn = useCallback(async () => {});
        const promise = fn();
        "#,
    );

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest(
        "/".into(),
        PackageJson::new("frontend")
            .with_version("0.0.0")
            .with_dependencies(Dependencies(Box::new([("react".into(), "19.0.0".into())]))),
    );

    let tsconfig_json = parse_json(r#"{}"#, JsonParserOptions::default());
    project_layout
        .insert_serialized_tsconfig("/".into(), &tsconfig_json.syntax().as_send().unwrap());

    let db = build_js_test_module_db_with_layout(
        &fs,
        &project_layout,
        &["/node_modules/@types/react/index.d.ts", "/src/index.ts"],
        true,
    );
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let react_module = db
        .module_for_path(Utf8Path::new("/node_modules/@types/react/index.d.ts"))
        .expect("React module must exist");
    let ModuleInfoKind::Js(react_info) = react_module.kind(&db) else {
        panic!("React module must be JavaScript");
    };
    let react_types = infer_module_types(&db, react_module).expect("React types must be inferred");
    for export in react_info.exports.values() {
        let Some(JsOwnExport::Type(resolved_id)) = export.as_own_export() else {
            continue;
        };
        if resolved_id.level() == TypeResolverLevel::Thin {
            assert!(
                react_types.types.get(resolved_id.index()).is_some(),
                "raw export type ID must index the raw Salsa type table"
            );
        }
    }
    let use_callback_export = react_info
        .exports
        .get("useCallback")
        .and_then(JsExport::as_own_export)
        .expect("React must export useCallback");
    let use_callback_source_ty = match use_callback_export {
        JsOwnExport::Binding(range) => react_types
            .binding_type_data
            .get(range)
            .map(|data| data.ty)
            .expect("useCallback source binding type must be inferred"),
        JsOwnExport::Type(use_callback_id) => react_types
            .types
            .get(use_callback_id.index())
            .copied()
            .expect("useCallback raw export ID must be in bounds"),
        JsOwnExport::Namespace(_) => panic!("useCallback must reference its raw source type"),
    };
    assert!(
        react_types
            .resolve_type(&db, use_callback_source_ty)
            .callable_function(&db)
            .is_some()
    );
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");

    let use_callback_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "useCallback")
        .expect("useCallback binding type must be inferred");
    let use_callback_ty = inferred.resolve_type(&db, use_callback_ty);
    assert!(use_callback_ty.callable_function(&db).is_some());

    let promise_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "promise")
        .expect("promise binding type must be inferred");
    let promise_ty = inferred.resolve_type(&db, promise_ty);
    assert_eq!(promise_ty.is_promise_instance(&db), Some(true));
}

#[test]
fn test_infer_module_types_resolves_redis_commander_types() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/RedisCommander.d.ts".into(),
        include_bytes!("../benches/RedisCommander.d.ts"),
    );
    fs.insert(
        "/index.ts".into(),
        r#"import RedisCommander from "./RedisCommander.d.ts";
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/RedisCommander.d.ts", "/index.ts"], true);
    let commander_module = db
        .module_for_path(Utf8Path::new("/RedisCommander.d.ts"))
        .expect("module must exist");
    let commander_inferred =
        infer_module_types(&db, commander_module).expect("types must be inferred");
    assert!(!commander_inferred.types.is_empty());

    let index_module = db
        .module_for_path(Utf8Path::new("/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, index_module).expect("types must be inferred");
    let commander_ty = inferred_binding_ty_by_name(&db, index_module, &inferred, "RedisCommander")
        .expect("RedisCommander binding type must be inferred");
    let commander_ty = inferred.resolve_type(&db, commander_ty);
    assert_ne!(commander_ty, InferredTypeData::Unknown);
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
        let ty = inferred_binding_ty_by_name(&db, module, &inferred, name)
            .expect("binding type must be inferred");
        let ty = inferred.resolve_type(&db, ty);
        assert!(
            is_inferred_array_of_promises(&db, ty),
            "{name} must be inferred as an array of promises, got {}",
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
    let result = inferred_binding_ty_by_name(&db, module, &inferred, "result")
        .expect("result type must be inferred");
    let result = inferred.resolve_type(&db, result);

    assert!(
        is_inferred_promise_instance(&db, result),
        "result must be inferred as a promise, got {result:?}"
    );
}

#[test]
fn test_infer_module_types_preserves_lexical_this_in_object_members() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/index.ts".into(),
        r#"
            const api = {
                value: "value",
                topLevel: this.value,
                get getter() {
                    return this.value;
                },
                method() {
                    return this.value;
                },
                nestedArrow() {
                    const read = () => this.value;
                    return read();
                },
                nestedObject() {
                    const inner = { value: this.value };
                    return inner.value;
                },
                arrow: () => this.value,
            };

            export const getterResult = api.getter;
            export const methodResult = api.method();
            export const nestedArrowResult = api.nestedArrow();
            export const nestedObjectResult = api.nestedObject();
            export const topLevelResult = api.topLevel;
            export const arrowResult = api.arrow();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");

    for name in [
        "getterResult",
        "methodResult",
        "nestedArrowResult",
        "nestedObjectResult",
    ] {
        let ty = inferred_binding_ty_by_name(&db, module, &inferred, name)
            .expect("binding type must be inferred");
        let ty = normalize_type(&db, module, ty);
        assert!(
            is_inferred_string_literal(&db, ty, "value"),
            "{name} must be the object member value, got {}",
            format_inferred_type(&db, ty),
        );
    }

    for name in ["topLevelResult", "arrowResult"] {
        let ty = inferred_binding_ty_by_name(&db, module, &inferred, name)
            .expect("binding type must be inferred");
        let ty = normalize_type(&db, module, ty);
        assert!(
            !is_inferred_string_literal(&db, ty, "value"),
            "{name} must not bind this to the object, got {}",
            format_inferred_type(&db, ty),
        );
    }
}
