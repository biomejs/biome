use std::collections::BTreeMap;

use biome_db::ParsedSource;
use biome_db::testing::{
    Events, assert_function_query_was_not_run, assert_function_query_was_run,
    function_query_will_execute_count_by_name, function_query_will_execute_position,
};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_type_info::{
    InferredType, format_inferred_type,
    resolved::{
        InferredCallArgumentType, InferredFunctionParameter, InferredInterface,
        InferredLiteralValue as InferredLiteral, InferredLocalTypeId, InferredMergedReference,
        InferredModuleKey, InferredObject, InferredReturnType, InferredTypeData,
        InferredTypeMemberKind, InferredTypeofType, InferredUnion,
    },
};
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_module_graph::{
    CallArgumentTypeInput, CallExpressionTypeInput, InferredModuleTypes, JsExport, JsOwnExport,
    ModuleDb, ModuleGraphGeneration, ModuleInfo, ModuleInfoKind, NormalizeTypeInput, PathInfoCache,
    find_value_member_type, infer_call_argument_type,
    infer_call_expression_type as infer_call_expression_type_query,
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

#[path = "spec_tests/callback_parameters.test.rs"]
mod callback_parameters;
#[path = "spec_tests/calls.test.rs"]
mod calls;
#[path = "spec_tests/css_classes.test.rs"]
mod css_classes;
#[path = "spec_tests/css_properties.test.rs"]
mod css_properties;
#[path = "spec_tests/cycles.test.rs"]
mod cycles;
#[path = "spec_tests/database.test.rs"]
mod database;
#[path = "spec_tests/expected_argument_inference.test.rs"]
mod expected_argument_inference;
#[path = "spec_tests/expressions.test.rs"]
mod expressions;
#[path = "spec_tests/globals.test.rs"]
mod globals;
#[path = "spec_tests/html_classes.test.rs"]
mod html_classes;
#[path = "spec_tests/html_components.test.rs"]
mod html_components;
#[path = "spec_tests/imports.test.rs"]
mod imports;
#[path = "spec_tests/intersections.test.rs"]
mod intersections;
#[path = "spec_tests/js_doc.test.rs"]
mod js_doc;
#[path = "spec_tests/module_resolution.test.rs"]
mod module_resolution;
#[path = "spec_tests/normalization.test.rs"]
mod normalization;
#[path = "spec_tests/overloads.test.rs"]
mod overloads;
#[path = "spec_tests/promises.test.rs"]
mod promises;
#[path = "spec_tests/queries.test.rs"]
mod queries;
#[path = "spec_tests/requests.test.rs"]
mod requests;
#[path = "spec_tests/substitutions.test.rs"]
mod substitutions;
#[path = "spec_tests/types.test.rs"]
mod types;

#[path = "spec_tests/support.rs"]
mod support;

#[path = "snap/mod.rs"]
mod snap;

#[salsa::db]
struct TestModuleDb {
    modules: BTreeMap<Utf8PathBuf, ModuleInfo>,
    events: Events,
    storage: Storage<Self>,
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

    fn take_salsa_events(&self) -> Vec<salsa::Event> {
        std::mem::take(&mut *self.events.0.lock().unwrap())
    }

    fn clear_salsa_events(&self) {
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
impl LanguageDb for TestModuleDb {
    fn source_from_index(&self, _index: usize) -> Option<DocumentFileSource> {
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

    fn for_each_module(&self, f: &mut dyn FnMut(ModuleInfo)) {
        for module in self.modules.values() {
            f(*module);
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

fn binding_range_by_name(db: &dyn ModuleDb, module: ModuleInfo, name: &str) -> TextRange {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        panic!("module must contain JavaScript information");
    };
    js_info
        .semantic_model
        .all_bindings()
        .find(|binding| {
            binding
                .tree()
                .name_token()
                .is_ok_and(|token| token.text_trimmed() == name)
        })
        .unwrap_or_else(|| panic!("{name} binding must exist"))
        .syntax()
        .text_trimmed_range()
}
