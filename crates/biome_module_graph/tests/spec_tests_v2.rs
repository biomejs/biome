use std::collections::BTreeMap;

use biome_db::ParsedSource;
use biome_db::testing::{
    Events, assert_function_query_was_not_run, assert_function_query_was_run,
    function_query_will_execute_position,
};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_type_info::{
    InferredType, format_inferred_type,
    interned_types::{
        CallArgumentType as InferredCallArgumentType,
        FunctionParameter as InferredFunctionParameter, InternedInterface as InferredInterface,
        InternedMergedReference as InferredMergedReference,
        InternedTypeofType as InferredTypeofType, InternedUnion as InferredUnion,
        Literal as InferredLiteral, LocalTypeId as InferredLocalTypeId,
        ModuleKey as InferredModuleKey, ReturnType as InferredReturnType,
        TypeData as InferredTypeData, TypeMemberKind as InferredTypeMemberKind,
    },
};
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_languages::JsFileSource;
use biome_module_graph::{
    CallArgumentTypeInput, CallExpressionTypeInput, InferredModuleTypes, JsExport, JsOwnExport,
    ModuleDb, ModuleInfo, ModuleInfoKind, NormalizeTypeInput, PathInfoCache,
    infer_call_argument_type, infer_call_expression_type as infer_call_expression_type_query,
    infer_constructor_argument_type, infer_module_types, infer_module_types_bottom_up,
    module_for_key, normalize_type as normalize_type_query, resolve_js_module,
};
use biome_package::{Dependencies, PackageJson};
use biome_project_layout::ProjectLayout;
use biome_rowan::{AstNode, Text, TextRange};
use biome_test_utils::get_added_js_paths;
use camino::{Utf8Path, Utf8PathBuf};
use salsa::Storage;
use salsa::plumbing::{AsId, FromId};

#[path = "spec_tests_v2/expected_argument_inference.test.rs"]
mod expected_argument_inference;
#[path = "spec_tests_v2/promises.test.rs"]
mod promises;

#[salsa::db]
struct TestModuleDb {
    modules: BTreeMap<Utf8PathBuf, ModuleInfo>,
    events: Events,
    storage: Storage<Self>,
}

#[test]
fn test_module_keys_reject_stale_handles() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.ts".into(), "export const value = 1;");

    let mut db = build_js_test_module_db(&fs, &["/src/index.ts"], true);
    let path = Utf8PathBuf::from("/src/index.ts");
    let original = db.module_for_path(&path).expect("module must exist");
    let replacement = ModuleInfo::new(&db, path.clone(), original.kind(&db).clone());
    db.modules.insert(path, replacement);

    assert!(
        module_for_key(&db, InferredModuleKey::new(original.as_id())).is_none(),
        "stale module handles must be rejected"
    );
    assert_eq!(
        module_for_key(&db, InferredModuleKey::new(replacement.as_id())),
        Some(replacement)
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
        let ty = inferred_binding_ty_by_name(&db, module, inferred, name)
            .expect("namespace member type must be inferred");
        assert!(is_inferred_number(&db, normalize_type(&db, module, ty)));
    }
}

impl TestModuleDb {
    fn new() -> Self {
        let events = Events::default();
        Self {
            modules: BTreeMap::new(),
            storage: salsa::Storage::new(Some(Box::new({
                let events = events.clone();
                move |event| {
                    events.0.lock().unwrap().push(event);
                }
            }))),
            events,
        }
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
    fn module_for_path(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        self.modules.get(path).copied()
    }

    fn for_each_module(&self, f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind)) {
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
        || matches!(ty, InferredTypeData::Literal(literal) if matches!(literal.literal(db), InferredLiteral::String(_)))
}

fn is_inferred_number<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    ty == InferredTypeData::Number
        || is_inferred_instance_of(db, ty, InferredTypeData::Number)
        || matches!(ty, InferredTypeData::Literal(literal) if matches!(literal.literal(db), InferredLiteral::Number(_)))
}

fn is_inferred_boolean<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    ty == InferredTypeData::Boolean
        || is_inferred_instance_of(db, ty, InferredTypeData::Boolean)
        || matches!(ty, InferredTypeData::Literal(literal) if matches!(literal.literal(db), InferredLiteral::Boolean(_)))
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
            if matches!(literal.literal(db), InferredLiteral::String(string) if string.as_str() == value)
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
            if matches!(literal.literal(db), InferredLiteral::Number(number) if number.as_str() == value)
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

    ty.is_promise_instance(db) && instance.type_parameters(db).iter().any(|ty| predicate(*ty))
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
    infer_call_expression_type_query(
        db,
        CallExpressionTypeInput::new(db, module, callee, args.into_boxed_slice()),
    )
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
        write_inferred_type_rows(&mut content, db, module, inferred, source_code);
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
        db.modules.insert(Utf8PathBuf::from(*path), module_info);
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

    let map_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "readMap")
        .expect("readMap return type must be inferred");
    let InferredTypeData::InstanceOf(map_instance) = map_ty else {
        panic!("readMap must return a Map instance, got {map_ty:?}");
    };
    let InferredTypeData::Class(map_class) = map_instance.ty(&db) else {
        panic!("readMap must return a class instance");
    };
    assert_eq!(map_class.name(&db).as_ref().map(Text::text), Some("Map"));
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
    let InferredTypeData::Class(set_class) = set_instance.ty(&db) else {
        panic!("readSet must return a class instance");
    };
    assert_eq!(set_class.name(&db).as_ref().map(Text::text), Some("Set"));
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
    let InferredTypeData::Class(weak_map_class) = weak_map_instance.ty(&db) else {
        panic!("readWeakMap must return a class instance");
    };
    assert_eq!(
        weak_map_class.name(&db).as_ref().map(Text::text),
        Some("WeakMap")
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
    let source_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "source")
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
    let namespace_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "MyNs")
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
fn test_infer_module_types_bottom_up_warms_blanket_reexports() {
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

    let mut db =
        build_js_test_module_db(&fs, &["/src/leaf.ts", "/src/mid.ts", "/src/index.ts"], true);
    let leaf_module = db
        .module_for_path(Utf8Path::new("/src/leaf.ts"))
        .expect("leaf module must exist");
    let mid_module = db
        .module_for_path(Utf8Path::new("/src/mid.ts"))
        .expect("mid module must exist");
    let index_module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("index module must exist");

    db.clear_salsa_events();
    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");
    let return_ty = inferred_function_return_ty_by_name(&db, index_module, inferred, "read")
        .expect("read return type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, return_ty, "name")
        .expect("re-exported Source must expose name");
    assert!(is_inferred_string(&db, name_ty));

    let events = db.take_salsa_events();
    let leaf_position =
        function_query_will_execute_position(&db, infer_module_types, leaf_module, &events)
            .expect("leaf inference must run");
    let mid_position =
        function_query_will_execute_position(&db, infer_module_types, mid_module, &events)
            .expect("mid inference must run");
    let index_position =
        function_query_will_execute_position(&db, infer_module_types, index_module, &events)
            .expect("index inference must run");
    assert!(
        leaf_position < mid_position && mid_position < index_position,
        "bottom-up inference must warm blanket re-export dependencies before their importers"
    );
}

#[test]
fn test_infer_module_types_bottom_up_returns_none_for_import_cycles() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/a.ts".into(),
        r#"
            import { b } from "./b.ts";
            export const a = b;
        "#,
    );
    fs.insert(
        "/src/b.ts".into(),
        r#"
            import { a } from "./a.ts";
            export const b = a;
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/a.ts", "/src/b.ts"], true);
    let a_module = db
        .module_for_path(Utf8Path::new("/src/a.ts"))
        .expect("a module must exist");
    let b_module = db
        .module_for_path(Utf8Path::new("/src/b.ts"))
        .expect("b module must exist");

    assert!(
        infer_module_types_bottom_up(&db, a_module).is_none(),
        "the import cycle must use the documented no-result fallback"
    );
    assert!(
        infer_module_types_bottom_up(&db, b_module).is_none(),
        "cycle recovery must remain stable from either entry module"
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
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readInterfaceBox")
            .expect("readInterfaceBox return type must be inferred");
    let interface_value_ty = inferred
        .find_member_type(&db, interface_box_ty, "value")
        .expect("InterfaceBox<number>.value must be inferred through extends");
    assert!(is_inferred_number(&db, interface_value_ty));

    let class_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readClassBox")
            .expect("readClassBox return type must be inferred");
    let class_value_ty = inferred
        .find_member_type(&db, class_box_ty, "value")
        .expect("ClassBox<string>.value must be inferred through extends");
    assert!(is_inferred_string(&db, class_value_ty));

    let union_box_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readUnionBox")
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
                    if matches!(
                        instance.ty(&db),
                        InferredTypeData::Class(class)
                            if class
                                .name(&db)
                                .as_ref()
                                .is_some_and(|name| name.text() == "Array")
                    )
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
    let value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "value")
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
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readValue")
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

    let textual_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "textual")
        .expect("textual binding type must be inferred");
    let textual_ty = inferred.resolve_type(&db, textual_ty);
    assert!(
        is_inferred_string(&db, textual_ty),
        "textual must be string, got {}",
        format_inferred_type(&db, textual_ty)
    );

    let numeric_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "numeric")
        .expect("numeric binding type must be inferred");
    let numeric_ty = inferred.resolve_type(&db, numeric_ty);
    assert!(
        is_inferred_number(&db, numeric_ty),
        "numeric must be number, got {}",
        format_inferred_type(&db, numeric_ty)
    );

    let spread_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "spread")
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

    let selected_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "selected")
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

    let selected_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "selected")
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

    let array_spread_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "arraySpread")
        .expect("arraySpread binding type must be inferred");
    assert!(is_inferred_boolean(
        &db,
        inferred.resolve_type(&db, array_spread_ty)
    ));

    let optional_tuple_spread_ty =
        inferred_binding_ty_by_name(&db, index_module, inferred, "optionalTupleSpread")
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
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readValue")
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
    let read_value_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readValue")
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
    let best_effort_ty = inferred_overload_ty_by_name(&db, index_module, inferred, "bestEffort")
        .expect("bestEffort overload type must be inferred");
    let read_promise_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readPromise")
        .expect("readPromise binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readString")
        .expect("readString binding type must be inferred");

    let promise_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_promise_ty)]),
    );
    assert!(
        promise_result_ty.is_promise_instance(&db),
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
        !sync_result_ty.is_promise_instance(&db),
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
    let best_effort_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "bestEffort")
        .expect("bestEffort import type must be inferred");
    let read_promise_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readPromise")
        .expect("readPromise binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readString")
        .expect("readString binding type must be inferred");
    let best_effort_ty = inferred.resolve_type(&db, best_effort_ty);

    let promise_result_ty = infer_call_expression_type(
        &db,
        index_module,
        best_effort_ty,
        Vec::from([inferred.resolve_type(&db, read_promise_ty)]),
    );
    assert!(
        promise_result_ty.is_promise_instance(&db),
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
        !sync_result_ty.is_promise_instance(&db),
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
    let identity_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "identity")
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
    let maybe_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "maybeString")
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
    let run_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "run")
        .expect("run binding type must be inferred");
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readString")
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
    let wrap_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "wrap")
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

    assert!(call_ty.is_promise_instance(&db));
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
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");

    let result_ty = inferred.resolve_type(&db, result_ty);
    assert!(
        is_inferred_promise_with_type_parameter(&db, result_ty, |ty| is_inferred_string(&db, ty)),
        "generic alias must return Promise<string>, got {}",
        format_inferred_type(&db, result_ty)
    );
}

#[test]
fn test_infer_module_types_calls_imported_generic_function_type_aliases() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/src/factory.ts".into(),
        r#"
            type Factory<T> = () => Promise<T>;
            export declare const makeString: Factory<string>;
        "#,
    );
    fs.insert(
        "/src/index.ts".into(),
        r#"
            import { makeString } from "./factory";
            export const result = makeString();
        "#,
    );

    let db = build_js_test_module_db(&fs, &["/src/factory.ts", "/src/index.ts"], true);
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");
    let result_ty = inferred.resolve_type(&db, result_ty);

    assert!(is_inferred_promise_with_type_parameter(
        &db,
        result_ty,
        |ty| is_inferred_string(&db, ty)
    ));
    assert_inferred_type_snapshot(
        "test_infer_module_types_calls_imported_generic_function_type_aliases",
        &db,
        &fs,
    );
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
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");

    let result_ty = inferred.resolve_type(&db, result_ty);
    assert!(
        is_inferred_string(&db, result_ty),
        "nested generic alias must return string, got {}",
        format_inferred_type(&db, result_ty)
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
    let module = db
        .module_for_path(Utf8Path::new("/src/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types(&db, module).expect("types must be inferred");
    let wrap_ty = inferred_binding_ty_by_name(&db, module, inferred, "wrap")
        .expect("wrap binding type must be inferred");
    let call_ty = infer_call_expression_type(
        &db,
        module,
        inferred.resolve_type(&db, wrap_ty),
        Vec::from([InferredTypeData::Number]),
    );
    let InferredTypeData::Function(function) = call_ty else {
        panic!("wrap must return a function, got {call_ty:?}");
    };
    let InferredReturnType::Type(return_ty) = function.return_type(&db) else {
        panic!("nested function return type must be inferred");
    };

    assert!(is_inferred_number(&db, *return_ty));
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
    let result_ty = inferred_binding_ty_by_name(&db, module, inferred, "result")
        .expect("result type must be inferred");

    assert!(is_inferred_string_literal(
        &db,
        inferred.resolve_type(&db, result_ty),
        "value"
    ));
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
    let pair_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "pair")
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
    let with_name_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "withName")
        .expect("withName binding type must be inferred");
    let read_value_ty =
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readValue")
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
    let read_string_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readString")
        .expect("readString binding type must be inferred");
    let read_number_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "readNumber")
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
        inferred_function_return_ty_by_name(&db, index_module, inferred, "readPromiseObject")
            .expect("readPromiseObject return type must be inferred");
    let InferredTypeData::InstanceOf(instance) = promise_object_ty else {
        panic!("readPromiseObject must return a merged class instance, got {promise_object_ty:?}");
    };
    assert!(matches!(instance.ty(&db), InferredTypeData::Class(_)));

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
    let foo_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "Foo")
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

    let derived_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "derived")
        .expect("derived binding type must be inferred");
    let name_ty = inferred
        .find_member_type(&db, derived_ty, "name")
        .expect("Derived instance must inherit imported Base.name");
    assert!(is_inferred_string(&db, name_ty));

    let derived_class_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "Derived")
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
    let derived_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "derived")
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

    let _ = infer_module_types(&db, index_module);
    db.clear_salsa_events();
    let _ = infer_module_types(&db, index_module);
    let events = db.take_salsa_events();

    assert_function_query_was_not_run(&db, infer_module_types, index_module, &events);
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
fn test_infer_module_types_documents_react_export_equals_gap() {
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
    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");

    // React types are exposed through `export = React`, which the new engine
    // does not resolve yet. Once it does, these assertions must be upgraded to
    // expect a callable `useCallback` and a `Promise` instance for `promise`.
    let use_callback_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "useCallback")
        .expect("useCallback binding type must be inferred");
    let use_callback_ty = inferred.resolve_type(&db, use_callback_ty);
    assert!(use_callback_ty.callable_function(&db).is_none());

    let promise_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "promise")
        .expect("promise binding type must be inferred");
    let promise_ty = inferred.resolve_type(&db, promise_ty);
    assert!(!promise_ty.is_promise_instance(&db));
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
        infer_module_types_bottom_up(&db, commander_module).expect("types must be inferred");
    assert!(!commander_inferred.types.is_empty());

    let index_module = db
        .module_for_path(Utf8Path::new("/index.ts"))
        .expect("module must exist");
    let inferred = infer_module_types_bottom_up(&db, index_module).expect("types must be inferred");
    let commander_ty = inferred_binding_ty_by_name(&db, index_module, inferred, "RedisCommander")
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
