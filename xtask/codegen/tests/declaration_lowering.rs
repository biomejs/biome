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

#[test]
fn declaration_scalar_types_translate_in_every_type_position() -> Result<()> {
    for (source, expected) in [
        ("any", LoweredTypeData::AnyKeyword),
        ("unknown", LoweredTypeData::UnknownKeyword),
        ("never", LoweredTypeData::NeverKeyword),
        ("undefined", LoweredTypeData::Undefined),
        ("bigint", LoweredTypeData::BigInt),
        ("symbol", LoweredTypeData::Symbol),
        ("boolean", LoweredTypeData::Boolean),
        ("null", LoweredTypeData::Null),
        ("true", LoweredTypeData::BooleanLiteral(true)),
        ("false", LoweredTypeData::BooleanLiteral(false)),
        ("0", LoweredTypeData::NumberLiteral("0".into())),
        ("-0", LoweredTypeData::NumberLiteral("-0".into())),
        (
            "1_000.25",
            LoweredTypeData::NumberLiteral("1_000.25".into()),
        ),
        ("1e-3", LoweredTypeData::NumberLiteral("1e-3".into())),
        ("0xff", LoweredTypeData::NumberLiteral("0xff".into())),
        ("0b10", LoweredTypeData::NumberLiteral("0b10".into())),
        ("0o10", LoweredTypeData::NumberLiteral("0o10".into())),
        (
            "- /* trivia */ 0xFF",
            LoweredTypeData::NumberLiteral("-0xFF".into()),
        ),
        ("123n", LoweredTypeData::BigIntLiteral("123n".into())),
        ("0n", LoweredTypeData::BigIntLiteral("0n".into())),
        ("-0n", LoweredTypeData::BigIntLiteral("-0n".into())),
        (
            "9_007_199_254_740_993n",
            LoweredTypeData::BigIntLiteral("9_007_199_254_740_993n".into()),
        ),
        ("0b10n", LoweredTypeData::BigIntLiteral("0b10n".into())),
        ("0o10n", LoweredTypeData::BigIntLiteral("0o10n".into())),
        (
            "- /* trivia */ 0xFFn",
            LoweredTypeData::BigIntLiteral("-0xFFn".into()),
        ),
        ("'ready'", LoweredTypeData::StringLiteral("ready".into())),
    ] {
        let mut file = fixture("lowering.interfaces.d.ts")?;
        file.bytes = format!(
            "interface Example {{ value?: ({source}); method(input: {source}): {source}; callback: (input: {source}) => {source}; alternatives: {source} | string; }}"
        ).into_bytes();
        let table = lower(&[file], &["Example"]).with_context(|| source)?;
        let reference = table.interface_reference("Example").unwrap();
        let LoweredTypeData::Interface(interface) = local(&table, &reference) else {
            panic!("expected interface")
        };
        assert_eq!(
            local(&table, interface.member("value").unwrap().type_reference()),
            &expected,
            "{source}"
        );
        for name in ["method", "callback"] {
            let LoweredTypeData::Function(function) =
                local(&table, interface.member(name).unwrap().type_reference())
            else {
                panic!("expected function")
            };
            assert_eq!(
                local(&table, function.parameters()[0].type_reference()),
                &expected,
                "{source}"
            );
            assert_eq!(local(&table, function.return_type()), &expected, "{source}");
        }
        let LoweredTypeData::Union(union) = local(
            &table,
            interface.member("alternatives").unwrap().type_reference(),
        ) else {
            panic!("expected union")
        };
        assert_eq!(local(&table, &union[0]), &expected, "{source}");
        syn::parse_str::<syn::Expr>(&render_declarations(&table))?;
    }
    Ok(())
}

#[test]
fn declaration_scalar_support_does_not_accept_type_operators_or_objects() -> Result<()> {
    for source in [
        "unique symbol",
        "keyof symbol",
        "object",
        "{ value: bigint }",
        "bigint[]",
        "`text`",
    ] {
        let mut file = fixture("lowering.interfaces.d.ts")?;
        file.bytes = format!("interface Example {{ value: {source}; }}").into_bytes();
        let error = lower(&[file], &["Example"]).expect_err(source);
        assert!(
            format!("{error:#}").contains("unsupported type syntax"),
            "{source}: {error:#}"
        );
    }
    Ok(())
}

#[test]
fn declaration_scalar_runtime_fixture_matches_emission() -> Result<()> {
    let files = [fixture("lowering.scalars.d.ts")?];
    let table = lower(&files, &["Scalars"])?;
    let output = render_declarations(&table);
    let formatted = xtask_glue::reformat_without_preamble(format!(
        "fn scalar_types() -> Box<[crate::TypeData]> {{ {output} }}"
    ))?;
    let formatted = format!(
        "// Generated from xtask/codegen/tests/fixtures/global-types/lowering.scalars.d.ts.\n\
         // Regenerate with BIOME_GLOBAL_TYPES_UPDATE_FIXTURES=1 cargo test -p xtask_codegen --features global_types --test declaration_lowering declaration_scalar_runtime_fixture_matches_emission\n\
         {formatted}"
    );
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/biome_js_type_info/src/generated/scalar_test_types.rs");
    if std::env::var_os("BIOME_GLOBAL_TYPES_UPDATE_FIXTURES").is_some() {
        fs::write(&path, &formatted)?;
    }
    assert_eq!(formatted, fs::read_to_string(path)?);
    Ok(())
}

#[test]
fn class_members_use_declaration_names_and_generic_positions() -> Result<()> {
    use xtask_codegen::generate_global_types::lower::lower_global_types;

    for (key, value, member_name) in [("K", "V", "lookup"), ("Key", "Value", "renamed")] {
        let mut file = fixture("lowering.interfaces.d.ts")?;
        file.bytes = format!(
            "interface WeakMap<{key} extends MissingConstraint, {value}> {{ {member_name}(key: {key}): {value} | undefined; }}
             interface WeakMap<{key} extends MissingConstraint, {value}> {{ extra?: boolean; chain(): this; readonly [Symbol.toStringTag]: Unsupported; }}
             declare var WeakMap: UnsupportedConstructor;"
        ).into_bytes();
        let manifest = build_global_manifest(collect(&file).records);
        let lowered = lower_global_types(&manifest, &[file])?;
        let LoweredTypeData::Class(class) = lowered.global("WeakMap").unwrap().data() else {
            panic!("expected class")
        };
        let local = |reference: &LoweredTypeReference| {
            let LoweredTypeReference::Local(index) = reference else {
                panic!("expected local reference")
            };
            &lowered.local_types()[*index]
        };
        let LoweredTypeData::Function(method) =
            local(class.member(member_name).unwrap().type_reference())
        else {
            panic!("expected method")
        };
        assert_eq!(
            method.parameters()[0].type_reference(),
            &class.type_parameters()[0]
        );
        let LoweredTypeData::Union(types) = local(method.return_type()) else {
            panic!("expected union")
        };
        assert_eq!(&types[0], &class.type_parameters()[1]);
        assert_eq!(local(&types[1]), &LoweredTypeData::Undefined);
        let property = class.member("extra").unwrap();
        assert_eq!(
            property.kind(),
            &LoweredMemberKind::Named { optional: true }
        );
        assert_eq!(local(property.type_reference()), &LoweredTypeData::Boolean);
        let LoweredTypeData::Function(chain) =
            local(class.member("chain").unwrap().type_reference())
        else {
            panic!("expected method")
        };
        assert_eq!(local(chain.return_type()), &LoweredTypeData::ThisKeyword);
        assert!(
            class
                .members()
                .iter()
                .all(|member| matches!(member.kind(), LoweredMemberKind::Named { .. }))
        );
    }
    Ok(())
}

#[test]
fn class_members_reject_unsupported_named_shapes() -> Result<()> {
    use xtask_codegen::generate_global_types::lower::lower_global_types;

    for (member, expected) in [
        ("method<T>(value: T): T;", "unsupported type parameters"),
        ("value: Missing;", "unsupported class member type reference"),
        ("value: K<string>;", "unsupported type arguments"),
        ("method(): V; method(): K;", "unsupported duplicate member"),
        ("[key: string]: V;", "unsupported class member"),
    ] {
        let mut file = fixture("lowering.interfaces.d.ts")?;
        file.bytes = format!("interface WeakMap<K, V> {{ {member} }}").into_bytes();
        let manifest = build_global_manifest(collect(&file).records);
        let error = lower_global_types(&manifest, &[file]).expect_err(member);
        assert!(
            format!("{error:#}").contains(expected),
            "{member}: {error:#}"
        );
    }
    Ok(())
}
