#![cfg(feature = "global_types")]

use std::{fs, path::Path};

use anyhow::{Context, Result};
use xtask_codegen::generate_global_types::{
    collect::collect,
    lower::{
        LoweredDeclarations, LoweredFunctionParameterBinding, LoweredMemberKind, LoweredTypeData,
        LoweredTypeReference, lower_interfaces,
    },
    manifest::build_global_manifest,
    render_declarations,
    source::{CanonicalPath, DiscoveredFile},
};

fn fixture(name: &str) -> Result<DiscoveredFile> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/global-types");
    Ok(DiscoveredFile {
        path: CanonicalPath::from_within(&root, name)?,
        repo_relative: name.to_owned(),
        bytes: fs::read(root.join(name))?,
    })
}

fn lower(files: &[DiscoveredFile], names: &[&str]) -> Result<LoweredDeclarations> {
    let manifest = build_global_manifest(
        files
            .iter()
            .flat_map(|file| collect(file).records)
            .collect(),
    );
    lower_interfaces(&manifest, files, names)
}

fn local<'a>(
    table: &'a LoweredDeclarations,
    reference: &LoweredTypeReference,
) -> &'a LoweredTypeData {
    let LoweredTypeReference::Local(index) = reference else {
        panic!("expected local reference, got {reference:?}");
    };
    &table.types()[*index]
}

#[test]
fn declaration_interfaces_preserve_types_and_signatures() -> Result<()> {
    let files = [fixture("lowering.interfaces.d.ts")?];
    let table = lower(&files, &["Catalog"])?;
    let catalog_ref = table
        .interface_reference("Catalog")
        .context("missing Catalog")?;
    let item_ref = table.interface_reference("Item").context("missing Item")?;
    let LoweredTypeData::Interface(catalog) = local(&table, &catalog_ref) else {
        panic!("expected interface")
    };
    assert_eq!(catalog.name(), "Catalog");
    assert_eq!(
        catalog.extends(),
        [
            table.interface_reference("Named").unwrap(),
            table.interface_reference("Tagged").unwrap()
        ]
    );
    assert_eq!(catalog.member("item").unwrap().type_reference(), &item_ref);
    assert_eq!(
        catalog.member("selected").unwrap().kind(),
        &LoweredMemberKind::Named { optional: true }
    );
    assert_nullable(
        &table,
        catalog.member("selected").unwrap().type_reference(),
        &item_ref,
    );
    assert_eq!(
        local(&table, catalog.member("empty").unwrap().type_reference()),
        &LoweredTypeData::Null
    );
    assert_eq!(
        catalog.member("count").unwrap().type_reference(),
        &LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID")
    );
    assert_eq!(
        local(&table, catalog.member("label").unwrap().type_reference()),
        &LoweredTypeData::StringLiteral("ready".into())
    );
    let LoweredTypeData::Union(state) =
        local(&table, catalog.member("state").unwrap().type_reference())
    else {
        panic!("expected union")
    };
    assert_eq!(local(&table, &state[0]), &LoweredTypeData::Boolean);
    assert_eq!(
        local(&table, &state[1]),
        &LoweredTypeData::StringLiteral("pending".into())
    );
    let method = catalog.member("find").unwrap();
    assert_eq!(method.kind(), &LoweredMemberKind::Named { optional: true });
    let LoweredTypeData::Function(function) = local(&table, method.type_reference()) else {
        panic!("expected function")
    };
    assert_eq!(function.name(), Some("find"));
    assert_eq!(function.parameters().len(), 2);
    assert_eq!(
        function.parameters()[0].binding(),
        &LoweredFunctionParameterBinding::Named("candidate".into())
    );
    assert!(!function.parameters()[0].is_optional());
    assert!(!function.parameters()[0].is_rest());
    assert!(function.parameters()[1].is_optional());
    assert_nullable(&table, function.parameters()[0].type_reference(), &item_ref);
    assert_nullable(&table, function.return_type(), &item_ref);
    let LoweredTypeData::Union(parameter_state) =
        local(&table, function.parameters()[1].type_reference())
    else {
        panic!("expected union")
    };
    assert_eq!(local(&table, &parameter_state[0]), local(&table, &state[0]));
    assert_eq!(local(&table, &parameter_state[1]), local(&table, &state[1]));
    let LoweredTypeData::Function(update) =
        local(&table, catalog.member("update").unwrap().type_reference())
    else {
        panic!("expected function")
    };
    assert_eq!(update.name(), None);
    assert_eq!(
        update.return_type(),
        &LoweredTypeReference::Predefined("GLOBAL_VOID_ID")
    );
    assert_eq!(update.parameters()[0].type_reference(), &item_ref);
    assert!(update.parameters()[1].is_optional());
    let LoweredTypeData::Interface(item) = local(&table, &item_ref) else {
        panic!("expected interface")
    };
    assert_eq!(item.member("owner").unwrap().type_reference(), &catalog_ref);
    assert!(table.interface_reference("Unselected").is_none());
    Ok(())
}

#[test]
fn declaration_output_is_deterministic_and_follows_source_edits() -> Result<()> {
    let mut files = [fixture("lowering.interfaces.d.ts")?];
    let original = lower(&files, &["Catalog"])?;
    let output = render_declarations(&original);
    assert_eq!(original, lower(&files, &["Catalog"])?);
    assert_eq!(output, render_declarations(&lower(&files, &["Catalog"])?));
    syn::parse_str::<syn::Expr>(&output)?;
    let formatted = prettyplease::unparse(&syn::parse_file(&format!(
        "fn types() -> Box<[crate::TypeData]> {{ {output} }}"
    ))?);
    let expected = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/global-types/lowering.interfaces.rs"),
    )?;
    assert_eq!(formatted, expected);
    files[0].bytes = String::from_utf8(files[0].bytes.clone())?
        .replace("selected?: Item | null", "selected?: boolean | \"changed\"")
        .into_bytes();
    let changed = lower(&files, &["Catalog"])?;
    assert_ne!(original, changed);
    assert_ne!(output, render_declarations(&changed));
    assert!(render_declarations(&changed).contains("\"changed\""));
    Ok(())
}

#[test]
fn declaration_errors_report_unresolved_references() -> Result<()> {
    let files = [fixture("lowering.errors.d.ts")?];
    for (name, expected) in [
        ("MissingProperty", "unresolved type reference Missing"),
        ("MissingBase", "unresolved type reference Missing"),
        ("MissingParameter", "unresolved type reference Missing"),
        ("MissingReturn", "unresolved type reference Missing"),
        ("Absent", "unresolved type reference Absent"),
    ] {
        let error = lower(&files, &[name]).expect_err(name);
        assert!(format!("{error:#}").contains(expected), "{name}: {error:#}");
    }
    Ok(())
}

#[test]
fn declaration_references_cross_source_files() -> Result<()> {
    let mut first = fixture("lowering.interfaces.d.ts")?;
    let mut second = fixture("lowering.errors.d.ts")?;
    first.bytes = b"interface Entry extends Base { value: (Detail | null); }".to_vec();
    second.bytes = b"interface Detail { entry: Entry; } interface Base { name: string; }".to_vec();
    let table = lower(&[first, second], &["Entry", "Entry"])?;
    assert!(table.interface_reference("Detail").is_some());
    assert!(table.interface_reference("Base").is_some());
    assert_eq!(table.types().len(), 5);
    Ok(())
}

#[test]
fn declaration_parser_errors_are_not_silently_lowered() -> Result<()> {
    let mut file = fixture("lowering.interfaces.d.ts")?;
    file.bytes = b"interface Broken { value: string | ; }".to_vec();
    let error = lower(&[file], &["Broken"]).expect_err("invalid declaration must fail");
    assert!(
        format!("{error:#}").contains("parser diagnostics"),
        "{error:#}"
    );
    Ok(())
}

fn assert_nullable(
    table: &LoweredDeclarations,
    reference: &LoweredTypeReference,
    item: &LoweredTypeReference,
) {
    let LoweredTypeData::Union(types) = local(table, reference) else {
        panic!("expected union")
    };
    assert_eq!(types.len(), 2);
    assert_eq!(&types[0], item);
    assert_eq!(local(table, &types[1]), &LoweredTypeData::Null);
}
