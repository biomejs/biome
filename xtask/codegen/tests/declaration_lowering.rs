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
fn indexed_access_preserves_nested_operands() -> Result<()> {
    let mut file = fixture("lowering.interfaces.d.ts")?;
    file.bytes = b"interface Access {
        object: Target;
        key: 'items';
        items: Target['items'];
        repeated: (Target)['items'];
        nested: Target['items'][number];
        values: Target[keyof Target];
        pair: [string, number];
        keys: 0 | 1;
        element: [string, number][0 | 1];
    }
    interface Target { items: string[]; }"
        .to_vec();
    let table = lower(&[file], &["Access"])?;
    let LoweredTypeData::Interface(interface) =
        local(&table, &table.interface_reference("Access").unwrap())
    else {
        panic!("expected interface")
    };
    let member = |name| interface.member(name).unwrap().type_reference().clone();
    for (name, object, index) in [
        ("items", member("object"), member("key")),
        (
            "nested",
            member("items"),
            LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID"),
        ),
        ("element", member("pair"), member("keys")),
    ] {
        assert_eq!(
            local(&table, &member(name)),
            &LoweredTypeData::IndexedAccess { object, index },
        );
    }
    assert_eq!(member("items"), member("repeated"));
    let LoweredTypeData::IndexedAccess { object, index } = local(&table, &member("values")) else {
        panic!("expected indexed access")
    };
    assert_eq!(object, &table.interface_reference("Target").unwrap());
    assert_eq!(
        local(&table, index),
        &LoweredTypeData::Keyof(object.clone())
    );
    Ok(())
}

#[test]
fn indexed_access_preserves_signature_parameters() -> Result<()> {
    let mut file = fixture("lowering.interfaces.d.ts")?;
    file.bytes = b"interface Access {
        get<T, K extends keyof T, V extends T[K] = T[K]>(value: T[K]): T[K];
    }"
    .to_vec();
    let table = lower(&[file], &["Access"])?;
    let LoweredTypeData::Interface(interface) =
        local(&table, &table.interface_reference("Access").unwrap())
    else {
        panic!("expected interface")
    };
    let LoweredTypeData::Function(function) =
        local(&table, interface.member("get").unwrap().type_reference())
    else {
        panic!("expected function")
    };
    let expected = LoweredTypeData::IndexedAccess {
        object: function.type_parameters()[0].clone(),
        index: function.type_parameters()[1].clone(),
    };
    assert_eq!(local(&table, function.return_type()), &expected);
    assert_eq!(
        function.parameters()[0].type_reference(),
        function.return_type()
    );
    let LoweredTypeData::GenericParameter {
        constraint,
        default,
        ..
    } = local(&table, &function.type_parameters()[2])
    else {
        panic!("expected generic parameter")
    };
    assert_eq!(constraint.as_ref(), Some(function.return_type()));
    assert_eq!(default, constraint);
    for (position, ty) in table.types().iter().enumerate() {
        if let LoweredTypeData::IndexedAccess { object, index } = ty {
            for operand in [object, index] {
                let LoweredTypeReference::Local(operand) = operand else {
                    panic!("expected local operand")
                };
                assert!(*operand < position);
            }
        }
    }
    Ok(())
}

#[test]
fn keyof_preserves_operands_in_signatures() -> Result<()> {
    let mut file = fixture("lowering.interfaces.d.ts")?;
    file.bytes = b"interface Keys {
        named: keyof Target;
        key<T, K extends keyof T = keyof T>(value: keyof (T)): keyof T;
    }
    interface Target { id: boolean; }"
        .to_vec();
    let table = lower(&[file], &["Keys"])?;
    let LoweredTypeData::Interface(interface) =
        local(&table, &table.interface_reference("Keys").unwrap())
    else {
        panic!("expected interface")
    };
    assert_eq!(
        local(&table, interface.member("named").unwrap().type_reference()),
        &LoweredTypeData::Keyof(table.interface_reference("Target").unwrap()),
    );
    let LoweredTypeData::Function(function) =
        local(&table, interface.member("key").unwrap().type_reference())
    else {
        panic!("expected function")
    };
    let expected = LoweredTypeData::Keyof(function.type_parameters()[0].clone());
    assert_eq!(
        local(&table, function.parameters()[0].type_reference()),
        &expected
    );
    assert_eq!(local(&table, function.return_type()), &expected);
    let LoweredTypeData::GenericParameter {
        constraint,
        default,
        ..
    } = local(&table, &function.type_parameters()[1])
    else {
        panic!("expected generic parameter")
    };
    for reference in [constraint, default] {
        assert_eq!(local(&table, reference.as_ref().unwrap()), &expected);
    }
    Ok(())
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
        ("object", LoweredTypeData::ObjectKeyword),
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
fn symbol_static_members_follow_merged_constructor_declarations() -> Result<()> {
    use xtask_codegen::generate_global_types::{
        compare::compare_lowered_globals, lower::lower_global_types,
    };

    for (constructor, property, scalar, expected) in [
        (
            "SymbolConstructor",
            "customKey",
            "boolean",
            LoweredTypeData::Boolean,
        ),
        ("Registry", "anotherKey", "bigint", LoweredTypeData::BigInt),
    ] {
        let mut file = fixture("manifest.disposables.d.ts")?;
        file.bytes = String::from_utf8(file.bytes)?
            .replace("SymbolConstructor", constructor)
            .into_bytes();
        let mut extension = fixture("lowering.interfaces.d.ts")?;
        extension.bytes = format!(
            r#"
            interface {constructor} {{ readonly {property}: unique symbol; }}
            interface {constructor} {{
                for(input?: {scalar}): {scalar} | undefined;
                keyFor(input: {scalar}): {scalar};
            }}
        "#
        )
        .into_bytes();
        let files = [file, extension];
        let manifest = build_global_manifest(
            files
                .iter()
                .flat_map(|file| collect(file).records)
                .collect(),
        );
        let lowered = lower_global_types(&manifest, &files)?;
        compare_lowered_globals(&lowered)?;
        let global = lowered.global("Symbol").unwrap();
        let LoweredTypeData::Class(class) = global.data() else {
            panic!("expected class")
        };
        let local = |reference: &LoweredTypeReference| {
            let LoweredTypeReference::Local(index) = reference else {
                panic!("expected local type")
            };
            &global.local_types()[*index]
        };
        let property = class.member(property).unwrap();
        assert_eq!(property.kind(), &LoweredMemberKind::NamedStatic);
        assert_eq!(local(property.type_reference()), &LoweredTypeData::Symbol);
        for name in ["for", "keyFor"] {
            let member = class.member(name).unwrap();
            assert_eq!(member.kind(), &LoweredMemberKind::NamedStatic);
            let LoweredTypeData::Function(function) = local(member.type_reference()) else {
                panic!("expected function")
            };
            let parameter = &function.parameters()[0];
            assert_eq!(local(parameter.type_reference()), &expected);
            assert_eq!(parameter.is_optional(), name == "for");
            if name == "for" {
                let LoweredTypeData::Union(types) = local(function.return_type()) else {
                    panic!("expected union")
                };
                assert_eq!(local(&types[0]), &expected);
                assert_eq!(local(&types[1]), &LoweredTypeData::Undefined);
            } else {
                assert_eq!(local(function.return_type()), &expected);
            }
        }
    }
    Ok(())
}

#[test]
fn generic_constraints_translate_types_and_parameter_references() -> Result<()> {
    use xtask_codegen::generate_global_types::lower::lower_global_types;

    for declaration in [
        "interface WeakMap",
        "interface Iterator",
        "type IteratorResult",
    ] {
        for (source, expected) in [
            ("boolean", LoweredTypeData::Boolean),
            ("'bound'", LoweredTypeData::StringLiteral("bound".into())),
            ("WeakKey", LoweredTypeData::ObjectKeyword),
            ("object", LoweredTypeData::ObjectKeyword),
        ] {
            let mut file = fixture("lowering.interfaces.d.ts")?;
            let body = if declaration.starts_with("type") {
                "= B;"
            } else {
                "{}"
            };
            file.bytes =
                format!("{declaration}<A extends ({source}), B extends A, C> {body}").into_bytes();
            let manifest = build_global_manifest(collect(&file).records);
            let lowered = lower_global_types(&manifest, &[file])?;
            let global = &lowered.globals()[0];
            let parameters = match global.data() {
                LoweredTypeData::Class(class) => class.type_parameters(),
                LoweredTypeData::Interface(interface) => interface.type_parameters(),
                LoweredTypeData::InstanceOf {
                    type_parameters, ..
                } => type_parameters,
                _ => panic!("expected generic declaration"),
            };
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                &global.local_types()[*index]
            };
            let constraint = |reference: &LoweredTypeReference| {
                let LoweredTypeData::GenericParameter { constraint, .. } = local(reference) else {
                    panic!("expected generic parameter")
                };
                constraint.as_ref()
            };
            let bound = constraint(&parameters[0]).expect("constraint must be preserved");
            assert_eq!(local(bound), &expected);
            assert_eq!(constraint(&parameters[1]), Some(&parameters[0]));
            assert_eq!(constraint(&parameters[2]), None);
        }
    }
    Ok(())
}

#[test]
fn generic_constraints_resolve_shadowed_names() -> Result<()> {
    use xtask_codegen::generate_global_types::lower::lower_global_types;

    for name in ["WeakMap", "Iterator"] {
        let mut file = fixture("lowering.interfaces.d.ts")?;
        file.bytes =
            format!("interface {name}<WeakKey extends boolean, Value extends WeakKey> {{}}")
                .into_bytes();
        let manifest = build_global_manifest(collect(&file).records);
        let lowered = lower_global_types(&manifest, &[file])?;
        let global = lowered.global(name).unwrap();
        let parameters = match global.data() {
            LoweredTypeData::Class(class) => class.type_parameters(),
            LoweredTypeData::Interface(interface) => interface.type_parameters(),
            _ => panic!("expected generic declaration"),
        };
        let LoweredTypeReference::Local(index) = parameters[1] else {
            panic!("expected local parameter")
        };
        let LoweredTypeData::GenericParameter { constraint, .. } = &global.local_types()[index]
        else {
            panic!("expected generic parameter")
        };
        assert_eq!(constraint.as_ref(), Some(&parameters[0]));
    }
    Ok(())
}

#[test]
fn unresolved_constraints_report_the_parameter() -> Result<()> {
    use xtask_codegen::generate_global_types::lower::lower_global_types;

    for name in ["WeakMap", "Iterator"] {
        let mut file = fixture("lowering.interfaces.d.ts")?;
        file.bytes = format!("interface {name}<Value extends Missing> {{}}").into_bytes();
        let manifest = build_global_manifest(collect(&file).records);
        let error =
            lower_global_types(&manifest, &[file]).expect_err("unresolved constraint must fail");
        let message = format!("{error:#}");
        assert!(
            message.contains("constraint")
                && message.contains("Value")
                && message.contains("Missing"),
            "{message}"
        );
    }
    Ok(())
}

#[test]
fn generic_constraints_preserve_unions_and_defaults() -> Result<()> {
    use xtask_codegen::generate_global_types::lower::lower_global_types;

    for name in ["Iterator", "WeakMap"] {
        let mut file = fixture("lowering.interfaces.d.ts")?;
        file.bytes =
            format!("interface {name}<A extends string | number = string, B extends A = A> {{}}")
                .into_bytes();
        let manifest = build_global_manifest(collect(&file).records);
        let lowered = lower_global_types(&manifest, &[file])?;
        let global = lowered.global(name).unwrap();
        let parameters = match global.data() {
            LoweredTypeData::Interface(interface) => interface.type_parameters(),
            LoweredTypeData::Class(class) => class.type_parameters(),
            _ => panic!("expected generic declaration"),
        };
        let local = |reference: &LoweredTypeReference| {
            let LoweredTypeReference::Local(index) = reference else {
                panic!("expected local type")
            };
            &global.local_types()[*index]
        };
        let LoweredTypeData::GenericParameter {
            constraint: Some(constraint),
            default,
            ..
        } = local(&parameters[0])
        else {
            panic!("expected constrained parameter")
        };
        let LoweredTypeData::Union(types) = local(constraint) else {
            panic!("expected union constraint")
        };
        assert_eq!(
            types.as_ref(),
            [
                LoweredTypeReference::Predefined("GLOBAL_STRING_ID"),
                LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID")
            ]
        );
        assert_eq!(
            default,
            &Some(LoweredTypeReference::Predefined("GLOBAL_STRING_ID"))
        );
        let LoweredTypeData::GenericParameter {
            constraint,
            default,
            ..
        } = local(&parameters[1])
        else {
            panic!("expected constrained parameter")
        };
        assert_eq!(constraint.as_ref(), Some(&parameters[0]));
        assert_eq!(default, constraint);
    }
    Ok(())
}

#[test]
fn iterator_declarations_translate_generics_aliases_and_rest_tuples() -> Result<()> {
    use xtask_codegen::generate_global_types::lower::lower_global_types;
    for method in ["advance", "step"] {
        let mut file = fixture("lowering.interfaces.d.ts")?;
        file.bytes = format!(r#"
            interface IteratorYieldResult<Value> {{ done?: false; value: Value; }}
            interface IteratorReturnResult<End> {{ done: true; value: End; }}
            type IteratorResult<Value, End = boolean> = IteratorYieldResult<Value> | IteratorReturnResult<End>;
            interface Iterator<Value, End = boolean, Input = unknown> {{
                {method}(...[input]: [] | [Input]): IteratorResult<Value, End>;
                finish?(value?: End): IteratorResult<Value, End>;
            }}
        "#).into_bytes();
        let manifest = build_global_manifest(collect(&file).records);
        let lowered = lower_global_types(&manifest, &[file])?;
        let local = |reference: &LoweredTypeReference| {
            let LoweredTypeReference::Local(index) = reference else {
                panic!("expected supporting type")
            };
            &lowered.global("Iterator").unwrap().local_types()[*index]
        };
        let LoweredTypeData::Interface(iterator) = lowered.global("Iterator").unwrap().data()
        else {
            panic!("expected interface")
        };
        let LoweredTypeData::Function(function) =
            local(iterator.member(method).unwrap().type_reference())
        else {
            panic!("expected function")
        };
        let [parameter] = function.parameters() else {
            panic!("expected a rest parameter")
        };
        assert!(parameter.is_rest());
        assert_eq!(
            parameter.binding(),
            &LoweredFunctionParameterBinding::Pattern
        );
        let LoweredTypeData::Union(alternatives) = local(parameter.type_reference()) else {
            panic!("expected tuple union")
        };
        assert_eq!(
            local(&alternatives[0]),
            &LoweredTypeData::Tuple(Box::default())
        );
        assert_eq!(
            local(&alternatives[1]),
            &LoweredTypeData::Tuple(Box::new([iterator.type_parameters()[2].clone()]))
        );
        let LoweredTypeData::InstanceOf {
            ty,
            type_parameters,
        } = local(function.return_type())
        else {
            panic!("expected result application")
        };
        assert_eq!(
            ty,
            &LoweredTypeReference::Predefined("GLOBAL_ITERATOR_RESULT_ID")
        );
        assert_eq!(type_parameters.as_ref(), &iterator.type_parameters()[..2]);
        let local = |reference: &LoweredTypeReference| {
            let LoweredTypeReference::Local(index) = reference else {
                panic!("expected supporting type")
            };
            &lowered.global("IteratorResult").unwrap().local_types()[*index]
        };
        let LoweredTypeData::InstanceOf {
            ty,
            type_parameters,
        } = lowered.global("IteratorResult").unwrap().data()
        else {
            panic!("expected alias declaration")
        };
        let LoweredTypeData::GenericParameter {
            default: Some(default),
            ..
        } = local(&type_parameters[1])
        else {
            panic!("expected default")
        };
        assert_eq!(local(default), &LoweredTypeData::Boolean);
        let LoweredTypeData::Union(results) = local(ty) else {
            panic!("expected alias union")
        };
        for (result, parameter) in results.iter().zip(type_parameters) {
            let LoweredTypeData::InstanceOf {
                type_parameters, ..
            } = local(result)
            else {
                panic!("expected result interface")
            };
            assert_eq!(type_parameters.as_ref(), std::slice::from_ref(parameter));
        }
    }
    Ok(())
}

#[test]
fn regexp_signatures_follow_declarations() -> Result<()> {
    use xtask_codegen::generate_global_types::lower::lower_global_types;
    let mut file = fixture("manifest.disposables.d.ts")?;
    file.bytes = String::from_utf8(file.bytes)?
        .replace(
            "interface RegExpExecArray {}",
            "interface RegExpExecArray extends Array<number> {}",
        )
        .replace(
            "exec(string: string): RegExpExecArray | null;",
            "exec<T>(value?: T): T;",
        )
        .into_bytes();
    file.bytes.extend_from_slice(
        b"
        declare var RegExp: Factory;
        interface Factory {
            new<T>(value?: T): RegExp;
            <T>(value?: T): RegExp;
            new(value: RegExp, flag?: boolean): RegExp;
            (value: RegExp, flag?: boolean): RegExp;
        }
    ",
    );
    let manifest = build_global_manifest(collect(&file).records);
    let lowered = lower_global_types(&manifest, &[file])?;
    let result = lowered.global("RegExpExecArray").unwrap();
    let LoweredTypeData::Interface(interface) = result.data() else {
        panic!("expected interface")
    };
    let LoweredTypeReference::Local(index) = &interface.extends()[0] else {
        panic!("expected applied base")
    };
    let LoweredTypeData::InstanceOf {
        ty,
        type_parameters,
    } = &result.local_types()[*index]
    else {
        panic!("expected applied base")
    };
    assert_eq!(ty, &LoweredTypeReference::Predefined("GLOBAL_ARRAY_ID"));
    assert_eq!(
        type_parameters.as_ref(),
        &[LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID")]
    );
    let LoweredTypeData::Function(exec) = lowered.global("RegExp.exec").unwrap().data() else {
        panic!("expected exec function")
    };
    assert_eq!(
        exec.parameters()[0].type_reference(),
        &exec.type_parameters()[0]
    );
    assert_eq!(exec.return_type(), &exec.type_parameters()[0]);
    assert!(exec.parameters()[0].is_optional());
    let regexp = lowered.global("RegExp").unwrap();
    let LoweredTypeData::Class(class) = regexp.data() else {
        panic!("expected class")
    };
    let mut calls = Vec::new();
    let mut constructors = Vec::new();
    for member in class.members() {
        let LoweredTypeReference::Local(index) = member.type_reference() else {
            continue;
        };
        match &regexp.local_types()[*index] {
            LoweredTypeData::Function(function)
                if member.kind() == &LoweredMemberKind::CallSignature =>
            {
                calls.push((function.parameters(), function.return_type()));
            }
            LoweredTypeData::Constructor(constructor) => {
                constructors.push((constructor.parameters(), constructor.return_type().unwrap()));
            }
            _ => {}
        }
    }
    assert_eq!(calls, constructors);
    assert_eq!(calls.len(), 2, "both distinct overloads must survive");
    for (parameters, result) in calls {
        assert!(parameters.last().unwrap().is_optional());
        let LoweredTypeReference::Local(index) = result else {
            panic!("expected self instance")
        };
        let LoweredTypeData::InstanceOf { ty, .. } = &regexp.local_types()[*index] else {
            panic!("expected self instance")
        };
        assert_eq!(ty, &LoweredTypeReference::Predefined("GLOBAL_REGEXP_ID"));
    }
    Ok(())
}
