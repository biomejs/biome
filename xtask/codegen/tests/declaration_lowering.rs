#![cfg(feature = "global_types")]

use std::{fs, path::Path};

use anyhow::{Context, Result};
use xtask_codegen::generate_global_types::{
    SourcePin,
    collect::collect,
    compare::compare_lowered_globals,
    lower::{
        LoweredFunctionParameterBinding, LoweredGlobal, LoweredGlobalTypes, LoweredInterface,
        LoweredMemberKind, LoweredTypeData, LoweredTypeReference, lower_global_types,
    },
    manifest::build_global_manifest,
    render_global_types,
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

/// Builds a declaration file with `source` as its content.
fn source(name: &str, source: impl Into<Vec<u8>>) -> Result<DiscoveredFile> {
    let mut file = fixture("lowering.interfaces.d.ts")?;
    name.clone_into(&mut file.repo_relative);
    file.bytes = source.into();
    Ok(file)
}

fn lower(files: &[DiscoveredFile]) -> Result<LoweredGlobalTypes> {
    let manifest = build_global_manifest(
        files
            .iter()
            .flat_map(|file| collect(file).records)
            .collect(),
    );
    lower_global_types(&manifest, files)
}

fn global<'a>(lowered: &'a LoweredGlobalTypes, name: &str) -> &'a LoweredGlobal {
    lowered
        .global(name)
        .unwrap_or_else(|| panic!("missing global {name}"))
}

fn interface<'a>(lowered: &'a LoweredGlobalTypes, name: &str) -> &'a LoweredInterface {
    let LoweredTypeData::Interface(interface) = global(lowered, name).data() else {
        panic!("expected {name} to be an interface")
    };
    interface
}

fn local<'a>(global: &'a LoweredGlobal, reference: &LoweredTypeReference) -> &'a LoweredTypeData {
    let LoweredTypeReference::Local(index) = reference else {
        panic!("expected local reference, got {reference:?}");
    };
    &global.local_types()[*index]
}

fn generated(constant: &str) -> LoweredTypeReference {
    LoweredTypeReference::Global(biome_rowan::Text::from(constant.to_owned()))
}

/// Asserts that `reference` is an unparameterized instance of the named global.
fn assert_instance_of(global: &LoweredGlobal, reference: &LoweredTypeReference, constant: &str) {
    assert_eq!(
        local(global, reference),
        &LoweredTypeData::InstanceOf {
            ty: generated(constant),
            type_parameters: Box::default(),
        }
    );
}

#[test]
fn every_declaration_is_lowered_and_named() -> Result<()> {
    let lowered = lower(&[source(
        "weak.d.ts",
        "interface WeakKeyTypes { object: object; }
        interface WeakKeyTypes { symbol: symbol; }
        type WeakKey = WeakKeyTypes[keyof WeakKeyTypes];
        interface Unused { name: string; }",
    )?])?;
    assert!(lowered.gaps().is_empty(), "{:?}", lowered.gaps());

    let types = interface(&lowered, "WeakKeyTypes");
    assert_eq!(
        types.members().len(),
        2,
        "merged declarations share one global"
    );
    let weak_key = global(&lowered, "WeakKey");
    let LoweredTypeData::Reference(reference) = weak_key.data() else {
        panic!("expected an alias")
    };
    let LoweredTypeData::IndexedAccess { object, index } = local(weak_key, reference) else {
        panic!("expected indexed access")
    };
    assert_instance_of(weak_key, object, "GLOBAL_WEAK_KEY_TYPES_ID");
    assert_eq!(
        local(weak_key, index),
        &LoweredTypeData::Keyof(object.clone())
    );

    for name in ["WeakKeyTypes", "WeakKey", "Unused"] {
        let roles = global(&lowered, name).roles();
        assert!(roles.type_name && !roles.value_name, "{name}");
    }
    Ok(())
}

#[test]
fn indexed_access_preserves_nested_operands() -> Result<()> {
    let lowered = lower(&[source(
        "access.d.ts",
        "interface Access {
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
        interface Target { items: string[]; }",
    )?])?;
    let access = global(&lowered, "Access");
    let interface = interface(&lowered, "Access");
    let member = |name| interface.member(name).unwrap().type_reference().clone();
    assert_instance_of(access, &member("object"), "GLOBAL_TARGET_ID");
    for (name, object, index) in [
        ("items", member("object"), member("key")),
        (
            "nested",
            member("items"),
            LoweredTypeReference::Predefined("GLOBAL_NUMBER_KEYWORD_ID"),
        ),
        ("element", member("pair"), member("keys")),
    ] {
        assert_eq!(
            local(access, &member(name)),
            &LoweredTypeData::IndexedAccess { object, index },
        );
    }
    assert_eq!(member("items"), member("repeated"));
    let LoweredTypeData::IndexedAccess { object, index } = local(access, &member("values")) else {
        panic!("expected indexed access")
    };
    assert_eq!(object, &member("object"));
    assert_eq!(
        local(access, index),
        &LoweredTypeData::Keyof(object.clone())
    );
    Ok(())
}

#[test]
fn indexed_access_preserves_signature_parameters() -> Result<()> {
    let lowered = lower(&[source(
        "access.d.ts",
        "interface Access {
            get<T, K extends keyof T, V extends T[K] = T[K]>(value: T[K]): T[K];
        }",
    )?])?;
    let access = global(&lowered, "Access");
    let interface = interface(&lowered, "Access");
    let LoweredTypeData::Function(function) =
        local(access, interface.member("get").unwrap().type_reference())
    else {
        panic!("expected function")
    };
    let expected = LoweredTypeData::IndexedAccess {
        object: function.type_parameters()[0].clone(),
        index: function.type_parameters()[1].clone(),
    };
    assert_eq!(local(access, function.return_type()), &expected);
    assert_eq!(
        function.parameters()[0].type_reference(),
        function.return_type()
    );
    let LoweredTypeData::GenericParameter {
        constraint,
        default,
        ..
    } = local(access, &function.type_parameters()[2])
    else {
        panic!("expected generic parameter")
    };
    assert_eq!(constraint.as_ref(), Some(function.return_type()));
    assert_eq!(default, constraint);
    for (position, ty) in access.local_types().iter().enumerate() {
        if let LoweredTypeData::IndexedAccess { object, index } = ty {
            for operand in [object, index] {
                let LoweredTypeReference::Local(operand) = operand else {
                    panic!("expected local operand")
                };
                assert!(*operand < position, "local types follow their dependencies");
            }
        }
    }
    Ok(())
}

#[test]
fn keyof_preserves_operands_in_signatures() -> Result<()> {
    let lowered = lower(&[source(
        "keys.d.ts",
        "interface Keys {
            named: keyof Target;
            key<T, K extends keyof T = keyof T>(value: keyof (T)): keyof T;
        }
        interface Target { id: boolean; }",
    )?])?;
    let keys = global(&lowered, "Keys");
    let interface = interface(&lowered, "Keys");
    let LoweredTypeData::Keyof(target) =
        local(keys, interface.member("named").unwrap().type_reference())
    else {
        panic!("expected keyof")
    };
    assert_instance_of(keys, target, "GLOBAL_TARGET_ID");
    let LoweredTypeData::Function(function) =
        local(keys, interface.member("key").unwrap().type_reference())
    else {
        panic!("expected function")
    };
    let expected = LoweredTypeData::Keyof(function.type_parameters()[0].clone());
    assert_eq!(
        local(keys, function.parameters()[0].type_reference()),
        &expected
    );
    assert_eq!(local(keys, function.return_type()), &expected);
    let LoweredTypeData::GenericParameter {
        constraint,
        default,
        ..
    } = local(keys, &function.type_parameters()[1])
    else {
        panic!("expected generic parameter")
    };
    for reference in [constraint, default] {
        assert_eq!(local(keys, reference.as_ref().unwrap()), &expected);
    }
    Ok(())
}

#[test]
fn declaration_interfaces_preserve_types_and_signatures() -> Result<()> {
    let lowered = lower(&[fixture("lowering.interfaces.d.ts")?])?;
    let catalog_global = global(&lowered, "Catalog");
    let catalog = interface(&lowered, "Catalog");
    assert_eq!(catalog.name(), "Catalog");
    assert_eq!(catalog.extends().len(), 2);
    assert_instance_of(catalog_global, &catalog.extends()[0], "GLOBAL_NAMED_ID");
    assert_instance_of(catalog_global, &catalog.extends()[1], "GLOBAL_TAGGED_ID");
    let item_ref = catalog.member("item").unwrap().type_reference();
    assert_instance_of(catalog_global, item_ref, "GLOBAL_ITEM_ID");
    assert_eq!(
        catalog.member("selected").unwrap().kind(),
        &LoweredMemberKind::Named { optional: true }
    );
    let assert_nullable = |reference: &LoweredTypeReference| {
        let LoweredTypeData::Union(types) = local(catalog_global, reference) else {
            panic!("expected union")
        };
        assert_eq!(types.as_ref()[0], *item_ref);
        assert_eq!(local(catalog_global, &types[1]), &LoweredTypeData::Null);
    };
    assert_nullable(catalog.member("selected").unwrap().type_reference());
    assert_eq!(
        local(
            catalog_global,
            catalog.member("empty").unwrap().type_reference()
        ),
        &LoweredTypeData::Null
    );
    assert_eq!(
        catalog.member("count").unwrap().type_reference(),
        &LoweredTypeReference::Predefined("GLOBAL_NUMBER_KEYWORD_ID"),
        "members of merged declarations are included"
    );
    assert_eq!(
        local(
            catalog_global,
            catalog.member("label").unwrap().type_reference()
        ),
        &LoweredTypeData::StringLiteral("ready".into())
    );
    let LoweredTypeData::Union(state) = local(
        catalog_global,
        catalog.member("state").unwrap().type_reference(),
    ) else {
        panic!("expected union")
    };
    assert_eq!(local(catalog_global, &state[0]), &LoweredTypeData::Boolean);
    assert_eq!(
        local(catalog_global, &state[1]),
        &LoweredTypeData::StringLiteral("pending".into())
    );
    let method = catalog.member("find").unwrap();
    assert_eq!(method.kind(), &LoweredMemberKind::Named { optional: true });
    let LoweredTypeData::Function(function) = local(catalog_global, method.type_reference()) else {
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
    assert_nullable(function.parameters()[0].type_reference());
    assert_nullable(function.return_type());
    assert_eq!(
        function.parameters()[1].type_reference(),
        &state_reference(catalog)
    );
    let LoweredTypeData::Function(update) = local(
        catalog_global,
        catalog.member("update").unwrap().type_reference(),
    ) else {
        panic!("expected function")
    };
    assert_eq!(update.name(), None);
    assert_eq!(
        update.return_type(),
        &LoweredTypeReference::Predefined("GLOBAL_VOID_ID")
    );
    assert_eq!(update.parameters()[0].type_reference(), item_ref);
    assert!(update.parameters()[1].is_optional());

    let item_global = global(&lowered, "Item");
    let item = interface(&lowered, "Item");
    assert_instance_of(
        item_global,
        item.member("owner").unwrap().type_reference(),
        "GLOBAL_CATALOG_ID",
    );
    assert!(
        lowered.global("Unselected").is_some(),
        "declarations are lowered without being referenced"
    );
    Ok(())
}

fn state_reference(catalog: &LoweredInterface) -> LoweredTypeReference {
    catalog.member("state").unwrap().type_reference().clone()
}

#[test]
fn declaration_output_is_deterministic_and_follows_source_edits() -> Result<()> {
    let pin = SourcePin::new("v0.0.0", "0000000000000000000000000000000000000000");
    let mut files = [fixture("lowering.interfaces.d.ts")?];
    let original = lower(&files)?;
    let output = render_global_types(&pin, &original);
    assert_eq!(original, lower(&files)?);
    assert_eq!(output, render_global_types(&pin, &lower(&files)?));
    let formatted = prettyplease::unparse(&syn::parse_file(&output)?);
    let expected_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/global-types/lowering.interfaces.rs");
    if std::env::var_os("UPDATE_EXPECT").is_some() {
        fs::write(&expected_path, &formatted)?;
    }
    assert_eq!(formatted, fs::read_to_string(&expected_path)?);
    files[0].bytes = String::from_utf8(files[0].bytes.clone())?
        .replace("selected?: Item | null", "selected?: boolean | \"changed\"")
        .into_bytes();
    let changed = lower(&files)?;
    assert_ne!(original, changed);
    let changed_output = render_global_types(&pin, &changed);
    assert_ne!(output, changed_output);
    assert!(changed_output.contains("\"changed\""));
    Ok(())
}

#[test]
fn unresolved_references_lower_to_unknown_and_are_reported() -> Result<()> {
    let lowered = lower(&[fixture("lowering.errors.d.ts")?])?;
    let unknown = LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID");
    for owner in [
        "MissingProperty",
        "MissingBase",
        "MissingParameter",
        "MissingReturn",
    ] {
        assert!(
            lowered.gaps().iter().any(|gap| gap.owner == owner
                && gap.detail.contains("unresolved type reference Missing")),
            "{owner}: {:?}",
            lowered.gaps()
        );
    }
    let property = global(&lowered, "MissingProperty");
    let LoweredTypeData::Interface(interface) = property.data() else {
        panic!("expected interface")
    };
    assert_eq!(
        interface.member("value").unwrap().type_reference(),
        &unknown
    );
    let LoweredTypeData::Interface(base) = global(&lowered, "MissingBase").data() else {
        panic!("expected interface")
    };
    assert_eq!(base.extends(), [unknown]);
    Ok(())
}

#[test]
fn declaration_references_cross_source_files() -> Result<()> {
    let lowered = lower(&[
        source(
            "entry.d.ts",
            "interface Entry extends Base { value: (Detail | null); }",
        )?,
        source(
            "detail.d.ts",
            "interface Detail { entry: Entry; } interface Base { name: string; }",
        )?,
    ])?;
    assert!(lowered.gaps().is_empty(), "{:?}", lowered.gaps());
    let entry = global(&lowered, "Entry");
    let interface = interface(&lowered, "Entry");
    assert_instance_of(entry, &interface.extends()[0], "GLOBAL_BASE_ID");
    let LoweredTypeData::Union(value) =
        local(entry, interface.member("value").unwrap().type_reference())
    else {
        panic!("expected union")
    };
    assert_instance_of(entry, &value[0], "GLOBAL_DETAIL_ID");
    let detail = global(&lowered, "Detail");
    assert_instance_of(
        detail,
        self::interface(&lowered, "Detail")
            .member("entry")
            .unwrap()
            .type_reference(),
        "GLOBAL_ENTRY_ID",
    );
    Ok(())
}

#[test]
fn declaration_parser_errors_are_not_silently_lowered() -> Result<()> {
    let error = lower(&[source(
        "broken.d.ts",
        "interface Broken { value: string | ; }",
    )?])
    .expect_err("invalid declaration must fail");
    assert!(
        format!("{error:#}").contains("parser diagnostics"),
        "{error:#}"
    );
    Ok(())
}

#[test]
fn declaration_scalar_types_translate_in_every_type_position() -> Result<()> {
    for (source_type, expected) in [
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
        let lowered = lower(&[source(
            "scalars.d.ts",
            format!(
                "interface Example {{ value?: ({source_type}); method(input: {source_type}): {source_type}; callback: (input: {source_type}) => {source_type}; alternatives: {source_type} | string; }}"
            ),
        )?])
        .with_context(|| source_type)?;
        let example = global(&lowered, "Example");
        let interface = interface(&lowered, "Example");
        assert_eq!(
            local(example, interface.member("value").unwrap().type_reference()),
            &expected,
            "{source_type}"
        );
        for name in ["method", "callback"] {
            let LoweredTypeData::Function(function) =
                local(example, interface.member(name).unwrap().type_reference())
            else {
                panic!("expected function")
            };
            assert_eq!(
                local(example, function.parameters()[0].type_reference()),
                &expected,
                "{source_type}"
            );
            assert_eq!(
                local(example, function.return_type()),
                &expected,
                "{source_type}"
            );
        }
        let LoweredTypeData::Union(union) = local(
            example,
            interface.member("alternatives").unwrap().type_reference(),
        ) else {
            panic!("expected union")
        };
        assert_eq!(local(example, &union[0]), &expected, "{source_type}");
    }
    Ok(())
}

#[test]
fn symbol_static_members_follow_merged_constructor_declarations() -> Result<()> {
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
        let extension = source(
            "extension.d.ts",
            format!(
                r#"
                interface {constructor} {{ readonly {property}: unique symbol; }}
                interface {constructor} {{
                    for(input?: {scalar}): {scalar} | undefined;
                    keyFor(input: {scalar}): {scalar};
                }}
            "#
            ),
        )?;
        let lowered = lower(&[file, extension])?;
        compare_lowered_globals(&lowered)?;
        let symbol = global(&lowered, "Symbol");
        let LoweredTypeData::Class(class) = symbol.data() else {
            panic!("expected class")
        };
        let member = class.member(property).unwrap();
        assert_eq!(member.kind(), &LoweredMemberKind::NamedStatic);
        let constant = format!(
            "GLOBAL_SYMBOL_{}_ID",
            biome_string_case::Case::Constant.convert(property)
        );
        assert_eq!(member.type_reference(), &generated(&constant));
        assert_eq!(
            global(&lowered, &format!("Symbol.{property}")).data(),
            &LoweredTypeData::Symbol
        );
        for name in ["for", "keyFor"] {
            let member = class.member(name).unwrap();
            assert_eq!(member.kind(), &LoweredMemberKind::NamedStatic);
            let LoweredTypeData::Function(function) = local(symbol, member.type_reference()) else {
                panic!("expected function")
            };
            let parameter = &function.parameters()[0];
            assert_eq!(local(symbol, parameter.type_reference()), &expected);
            assert_eq!(parameter.is_optional(), name == "for");
            if name == "for" {
                let LoweredTypeData::Union(types) = local(symbol, function.return_type()) else {
                    panic!("expected union")
                };
                assert_eq!(local(symbol, &types[0]), &expected);
                assert_eq!(local(symbol, &types[1]), &LoweredTypeData::Undefined);
            } else {
                assert_eq!(local(symbol, function.return_type()), &expected);
            }
        }
    }
    Ok(())
}

#[test]
fn computed_keys_use_unique_symbol_identities() -> Result<()> {
    let lowered = lower(&[source(
        "keys.d.ts",
        "declare var Symbol: SymbolConstructor;
        interface SymbolConstructor { readonly iterator: unique symbol; readonly plain: symbol; }
        interface Iterator<T> { value: T; }
        interface Iterable<T> { [Symbol.iterator](): Iterator<T>; }
        interface Owner { [Symbol.plain](): void; ['literal']: string; }",
    )?])?;
    let iterable = global(&lowered, "Iterable");
    let LoweredTypeData::Interface(interface) = iterable.data() else {
        panic!("expected interface")
    };
    let [member] = interface.members() else {
        panic!("expected one member")
    };
    assert_eq!(
        member.kind(),
        &LoweredMemberKind::ComputedValue {
            key_reference: generated("GLOBAL_SYMBOL_ITERATOR_ID"),
        }
    );
    let LoweredTypeData::Function(function) = local(iterable, member.type_reference()) else {
        panic!("expected method")
    };
    let LoweredTypeData::InstanceOf {
        ty,
        type_parameters,
    } = local(iterable, function.return_type())
    else {
        panic!("expected generic return")
    };
    assert_eq!(ty, &generated("GLOBAL_ITERATOR_ID"));
    assert_eq!(type_parameters.as_ref(), interface.type_parameters());

    // `Symbol.plain` is not a unique symbol, so its computed member is skipped.
    let owner = interface_members(&lowered, "Owner");
    assert_eq!(owner.len(), 1);
    assert!(lowered.gaps().iter().any(|gap| {
        gap.owner == "Owner"
            && gap
                .detail
                .contains("unsupported computed member key [Symbol.plain]")
    }));
    Ok(())
}

fn interface_members(lowered: &LoweredGlobalTypes, name: &str) -> Vec<String> {
    interface(lowered, name)
        .members()
        .iter()
        .map(|member| member.name().to_owned())
        .collect()
}

#[test]
fn classes_merge_instance_and_constructor_interfaces() -> Result<()> {
    for owner in ["Map", "Set", "WeakMap", "Date"] {
        let lowered = lower(&[source(
            "constructors.d.ts",
            format!(
                "interface {owner}<T> {{ current: T; }}
                declare var {owner}: Factory;
                interface Factory {{ new<U>(value: U): {owner}<U>; readonly prototype: {owner}<any>; }}
                interface Factory {{ new<U>(value: readonly U[]): {owner}<U>; create(): Factory; }}"
            ),
        )?])?;
        let global = global(&lowered, owner);
        let roles = global.roles();
        assert!(
            roles.type_name && roles.value_name,
            "classes serve both roles"
        );
        let LoweredTypeData::Class(class) = global.data() else {
            panic!("expected class")
        };
        assert_eq!(
            class.member("current").unwrap().type_reference(),
            &class.type_parameters()[0]
        );
        assert_eq!(
            class.member("prototype").unwrap().kind(),
            &LoweredMemberKind::NamedStatic
        );
        let constructors = class
            .members()
            .iter()
            .filter(|member| member.kind() == &LoweredMemberKind::Constructor)
            .collect::<Vec<_>>();
        assert_eq!(constructors.len(), 2);
        let reference = format!(
            "GLOBAL_{}_ID",
            biome_string_case::Case::Constant.convert(owner)
        );
        for member in constructors {
            let LoweredTypeData::Constructor(constructor) = local(global, member.type_reference())
            else {
                panic!("expected constructor")
            };
            let LoweredTypeData::InstanceOf {
                ty,
                type_parameters,
            } = local(global, constructor.return_type().unwrap())
            else {
                panic!("expected owner instance")
            };
            assert_eq!(ty, &generated(&reference));
            assert_eq!(type_parameters.as_ref(), constructor.type_parameters());
        }
        // The constructor interface is the type of the class value.
        let LoweredTypeData::Function(create) =
            local(global, class.member("create").unwrap().type_reference())
        else {
            panic!("expected static method")
        };
        assert_eq!(create.return_type(), &generated(&reference));
        for (index, ty) in global.local_types().iter().enumerate() {
            assert!(!global.local_types()[..index].contains(ty));
        }
        assert!(
            lowered.global("Factory").is_some(),
            "the constructor interface keeps its own global"
        );
    }
    Ok(())
}

#[test]
fn class_bases_extend_other_classes() -> Result<()> {
    let lowered = lower(&[source(
        "errors.d.ts",
        "interface Failure { message: string; }
        interface FailureConstructor { new(message?: string): Failure; }
        declare var Failure: FailureConstructor;
        interface Labeled { label: string; }
        interface RangeFailure extends Failure, Labeled {}
        interface RangeFailureConstructor extends FailureConstructor { new(message?: string): RangeFailure; }
        declare var RangeFailure: RangeFailureConstructor;",
    )?])?;
    let range = global(&lowered, "RangeFailure");
    let LoweredTypeData::Class(class) = range.data() else {
        panic!("expected class")
    };
    assert_eq!(class.extends(), Some(&generated("GLOBAL_FAILURE_ID")));
    let [labeled] = class.implements() else {
        panic!("expected one interface")
    };
    assert_instance_of(range, labeled, "GLOBAL_LABELED_ID");
    assert!(lowered.gaps().iter().any(|gap| {
        gap.owner == "RangeFailure"
            && gap
                .detail
                .contains("static members inherited by RangeFailureConstructor")
    }));
    Ok(())
}

#[test]
fn self_typed_values_keep_separate_type_and_value_globals() -> Result<()> {
    let lowered = lower(&[source(
        "math.d.ts",
        "interface Calculator { abs(x: number): number; }
        declare var Calculator: Calculator;",
    )?])?;
    let values = lowered
        .globals()
        .iter()
        .filter(|global| global.name() == "Calculator")
        .collect::<Vec<_>>();
    let [ty, value] = values.as_slice() else {
        panic!("expected type and value globals")
    };
    assert!(ty.roles().type_name && !ty.roles().value_name);
    assert!(!value.roles().type_name && value.roles().value_name);
    assert_eq!(value.id_constant(), "CALCULATOR_VALUE_ID_GLOBAL_TYPE_ID");
    let LoweredTypeData::Reference(reference) = value.data() else {
        panic!("expected the variable's type")
    };
    assert_instance_of(value, reference, "GLOBAL_CALCULATOR_ID");
    Ok(())
}

#[test]
fn function_declarations_preserve_generic_overloads_and_parameters() -> Result<()> {
    let lowered = lower(&[
        source(
            "functions0.d.ts",
            "declare function convertValue<T extends string = string>(value?: T): T;
            declare function readFlag(input: number): boolean;",
        )?,
        source(
            "functions1.d.ts",
            "declare function convertValue<T extends boolean>(map: (value: T) => T, ...values: T[]): T[];",
        )?,
    ])?;
    assert_eq!(lowered.globals().len(), 2);
    let read_flag = global(&lowered, "readFlag");
    assert!(read_flag.roles().value_name && !read_flag.roles().type_name);
    let LoweredTypeData::Function(function) = read_flag.data() else {
        panic!("expected single signature")
    };
    assert_eq!(function.name(), Some("readFlag"));
    assert_eq!(
        function.parameters()[0].type_reference(),
        &LoweredTypeReference::Predefined("GLOBAL_NUMBER_KEYWORD_ID")
    );
    assert_eq!(
        local(read_flag, function.return_type()),
        &LoweredTypeData::Boolean
    );

    let convert = global(&lowered, "convertValue");
    let LoweredTypeData::Object(signatures) = convert.data() else {
        panic!("expected overloads")
    };
    assert_eq!(signatures.len(), 2);
    let mut generics = Vec::new();
    for (index, signature) in signatures.iter().enumerate() {
        assert_eq!(signature.kind(), &LoweredMemberKind::CallSignature);
        let LoweredTypeData::Function(function) = local(convert, signature.type_reference()) else {
            panic!("expected function")
        };
        let [generic] = function.type_parameters() else {
            panic!("expected one generic parameter")
        };
        generics.push(generic);
        let LoweredTypeData::GenericParameter {
            constraint,
            default,
            ..
        } = local(convert, generic)
        else {
            panic!("expected generic parameter")
        };
        if index == 0 {
            assert_eq!(
                constraint,
                &Some(LoweredTypeReference::Predefined("GLOBAL_STRING_KEYWORD_ID"))
            );
            assert_eq!(default, constraint);
            assert!(function.parameters()[0].is_optional());
            assert_eq!(function.return_type(), generic);
        } else {
            assert_eq!(
                local(convert, constraint.as_ref().unwrap()),
                &LoweredTypeData::Boolean
            );
            let rest = &function.parameters()[1];
            assert!(rest.is_rest());
            assert_eq!(rest.type_reference(), function.return_type());
            let LoweredTypeData::InstanceOf {
                ty,
                type_parameters,
            } = local(convert, function.return_type())
            else {
                panic!("expected array")
            };
            assert_eq!(ty, &generated("GLOBAL_ARRAY_ID"));
            assert_eq!(type_parameters.as_ref(), std::slice::from_ref(generic));
        }
    }
    assert_ne!(generics[0], generics[1]);
    Ok(())
}

#[test]
fn namespaces_expose_values_and_qualified_types() -> Result<()> {
    let lowered = lower(&[source(
        "namespace.d.ts",
        "interface Value { unrelated: number; }
        declare namespace Formats {
            interface Value { own: string; }
            interface Options { value: Value; outer: Other.Value; }
            interface Formatter { format(options: Options): string; }
            interface FormatterConstructor { new(options?: Options): Formatter; }
            var Formatter: FormatterConstructor;
            var version: string;
            function create(): Formatter;
        }
        declare namespace Other { interface Value { enabled: boolean; } }",
    )?])?;
    assert!(lowered.gaps().is_empty(), "{:?}", lowered.gaps());

    let formats = global(&lowered, "Formats");
    assert!(formats.roles().value_name && !formats.roles().type_name);
    let LoweredTypeData::Object(members) = formats.data() else {
        panic!("expected namespace object")
    };
    let members = members
        .iter()
        .map(|member| (member.name(), member.type_reference().clone()))
        .collect::<Vec<_>>();
    assert_eq!(
        members,
        [
            ("Formatter", generated("GLOBAL_FORMATS_FORMATTER_ID")),
            ("version", generated("GLOBAL_FORMATS_VERSION_ID")),
            ("create", generated("GLOBAL_FORMATS_CREATE_ID")),
        ]
    );

    let options = global(&lowered, "Formats.Options");
    assert!(options.roles().type_name);
    let LoweredTypeData::Interface(interface) = options.data() else {
        panic!("expected interface")
    };
    // Names resolve in the enclosing namespace before the global scope.
    assert_instance_of(
        options,
        interface.member("value").unwrap().type_reference(),
        "GLOBAL_FORMATS_VALUE_ID",
    );
    assert_instance_of(
        options,
        interface.member("outer").unwrap().type_reference(),
        "GLOBAL_OTHER_VALUE_ID",
    );
    let LoweredTypeData::Class(formatter) = global(&lowered, "Formats.Formatter").data() else {
        panic!("expected class")
    };
    assert_eq!(formatter.name(), "Formats.Formatter");
    Ok(())
}

#[test]
fn generic_constraints_translate_types_and_parameter_references() -> Result<()> {
    for declaration in [
        "interface WeakMap",
        "interface Iterator",
        "type IteratorResult",
    ] {
        for (source_type, expected) in [
            ("boolean", LoweredTypeData::Boolean),
            ("'bound'", LoweredTypeData::StringLiteral("bound".into())),
            ("object", LoweredTypeData::ObjectKeyword),
        ] {
            let body = if declaration.starts_with("type") {
                "= B;"
            } else {
                "{}"
            };
            let lowered = lower(&[source(
                "generics.d.ts",
                format!("{declaration}<A extends ({source_type}), B extends A, C> {body}"),
            )?])?;
            let global = &lowered.globals()[0];
            let parameters = match global.data() {
                LoweredTypeData::Interface(interface) => interface.type_parameters(),
                LoweredTypeData::InstanceOf {
                    type_parameters, ..
                } => type_parameters,
                _ => panic!("expected generic declaration"),
            };
            let constraint = |reference: &LoweredTypeReference| {
                let LoweredTypeData::GenericParameter { constraint, .. } = local(global, reference)
                else {
                    panic!("expected generic parameter")
                };
                constraint.clone()
            };
            let bound = constraint(&parameters[0]).expect("constraint must be preserved");
            assert_eq!(local(global, &bound), &expected);
            assert_eq!(constraint(&parameters[1]), Some(parameters[0].clone()));
            assert_eq!(constraint(&parameters[2]), None);
        }
    }
    Ok(())
}

#[test]
fn generic_constraints_resolve_shadowed_names() -> Result<()> {
    let lowered = lower(&[source(
        "shadowed.d.ts",
        "interface WeakKey {}
        interface Owner<WeakKey extends boolean, Value extends WeakKey> {}",
    )?])?;
    let owner = global(&lowered, "Owner");
    let parameters = interface(&lowered, "Owner").type_parameters();
    let LoweredTypeData::GenericParameter { constraint, .. } = local(owner, &parameters[1]) else {
        panic!("expected generic parameter")
    };
    assert_eq!(constraint.as_ref(), Some(&parameters[0]));
    Ok(())
}

#[test]
fn constraints_on_later_parameters_lower_to_unknown() -> Result<()> {
    let lowered = lower(&[source(
        "forward.d.ts",
        "interface Owner<A extends B, B> {}",
    )?])?;
    let owner = global(&lowered, "Owner");
    let parameters = interface(&lowered, "Owner").type_parameters();
    let LoweredTypeData::GenericParameter { constraint, .. } = local(owner, &parameters[0]) else {
        panic!("expected generic parameter")
    };
    // Local types must follow their dependencies, so `A` cannot refer to `B`.
    assert_eq!(
        constraint.as_ref(),
        Some(&LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID"))
    );
    Ok(())
}

#[test]
fn generic_constraints_preserve_unions_and_defaults() -> Result<()> {
    let lowered = lower(&[source(
        "defaults.d.ts",
        "interface Owner<A extends string | number = string, B extends A = A> {}",
    )?])?;
    let owner = global(&lowered, "Owner");
    let parameters = interface(&lowered, "Owner").type_parameters();
    let LoweredTypeData::GenericParameter {
        constraint: Some(constraint),
        default,
        ..
    } = local(owner, &parameters[0])
    else {
        panic!("expected constrained parameter")
    };
    let LoweredTypeData::Union(types) = local(owner, constraint) else {
        panic!("expected union constraint")
    };
    assert_eq!(
        types.as_ref(),
        [
            LoweredTypeReference::Predefined("GLOBAL_STRING_KEYWORD_ID"),
            LoweredTypeReference::Predefined("GLOBAL_NUMBER_KEYWORD_ID")
        ]
    );
    assert_eq!(
        default,
        &Some(LoweredTypeReference::Predefined("GLOBAL_STRING_KEYWORD_ID"))
    );
    let LoweredTypeData::GenericParameter {
        constraint,
        default,
        ..
    } = local(owner, &parameters[1])
    else {
        panic!("expected constrained parameter")
    };
    assert_eq!(constraint.as_ref(), Some(&parameters[0]));
    assert_eq!(default, constraint);
    Ok(())
}

#[test]
fn iterator_declarations_translate_generics_aliases_and_rest_tuples() -> Result<()> {
    for method in ["advance", "step"] {
        let lowered = lower(&[source(
            "iterators.d.ts",
            format!(
                r#"
                interface IteratorYieldResult<Value> {{ done?: false; value: Value; }}
                interface IteratorReturnResult<End> {{ done: true; value: End; }}
                type IteratorResult<Value, End = boolean> = IteratorYieldResult<Value> | IteratorReturnResult<End>;
                interface Iterator<Value, End = boolean, Input = unknown> {{
                    {method}(...[input]: [] | [Input]): IteratorResult<Value, End>;
                    finish?(value?: End): IteratorResult<Value, End>;
                }}
            "#
            ),
        )?])?;
        let iterator_global = global(&lowered, "Iterator");
        let iterator = interface(&lowered, "Iterator");
        let LoweredTypeData::Function(function) = local(
            iterator_global,
            iterator.member(method).unwrap().type_reference(),
        ) else {
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
        let LoweredTypeData::Union(alternatives) =
            local(iterator_global, parameter.type_reference())
        else {
            panic!("expected tuple union")
        };
        assert_eq!(
            local(iterator_global, &alternatives[0]),
            &LoweredTypeData::Tuple(Box::default())
        );
        let LoweredTypeData::Tuple(elements) = local(iterator_global, &alternatives[1]) else {
            panic!("expected tuple")
        };
        let [element] = elements.as_ref() else {
            panic!("expected one element")
        };
        assert_eq!(element.ty(), &iterator.type_parameters()[2]);
        assert!(!element.is_optional() && !element.is_rest());
        let LoweredTypeData::InstanceOf {
            ty,
            type_parameters,
        } = local(iterator_global, function.return_type())
        else {
            panic!("expected result application")
        };
        assert_eq!(ty, &generated("GLOBAL_ITERATOR_RESULT_ID"));
        assert_eq!(type_parameters.as_ref(), &iterator.type_parameters()[..2]);

        let result = global(&lowered, "IteratorResult");
        let LoweredTypeData::InstanceOf {
            ty,
            type_parameters,
        } = result.data()
        else {
            panic!("expected alias declaration")
        };
        let LoweredTypeData::GenericParameter {
            default: Some(default),
            ..
        } = local(result, &type_parameters[1])
        else {
            panic!("expected default")
        };
        assert_eq!(local(result, default), &LoweredTypeData::Boolean);
        let LoweredTypeData::Union(results) = local(result, ty) else {
            panic!("expected alias union")
        };
        for (result_type, parameter) in results.iter().zip(type_parameters) {
            let LoweredTypeData::InstanceOf {
                type_parameters, ..
            } = local(result, result_type)
            else {
                panic!("expected result interface")
            };
            assert_eq!(type_parameters.as_ref(), std::slice::from_ref(parameter));
        }
    }
    Ok(())
}

#[test]
fn tuples_keep_labels_optional_and_rest_elements() -> Result<()> {
    let lowered = lower(&[source(
        "tuples.d.ts",
        "interface Owner { value: [first: string, second?: number, ...rest: boolean[]]; frozen: readonly [string?]; }",
    )?])?;
    let owner = global(&lowered, "Owner");
    let interface = interface(&lowered, "Owner");
    let LoweredTypeData::Tuple(elements) =
        local(owner, interface.member("value").unwrap().type_reference())
    else {
        panic!("expected tuple")
    };
    let shapes = elements
        .iter()
        .map(|element| (element.name(), element.is_optional(), element.is_rest()))
        .collect::<Vec<_>>();
    assert_eq!(
        shapes,
        [
            (Some("first"), false, false),
            (Some("second"), true, false),
            (Some("rest"), false, true),
        ]
    );
    let LoweredTypeData::Readonly(tuple) =
        local(owner, interface.member("frozen").unwrap().type_reference())
    else {
        panic!("expected readonly tuple")
    };
    let LoweredTypeData::Tuple(elements) = local(owner, tuple) else {
        panic!("expected tuple")
    };
    assert!(elements[0].is_optional());
    Ok(())
}

#[test]
fn unsupported_syntax_lowers_to_unknown() -> Result<()> {
    let lowered = lower(&[source(
        "unsupported.d.ts",
        "type Partial<T> = { [P in keyof T]?: T[P] };
        interface Owner { label: `prefix-${string}`; }",
    )?])?;
    let partial = global(&lowered, "Partial");
    let LoweredTypeData::InstanceOf { ty, .. } = partial.data() else {
        panic!("expected generic alias")
    };
    assert_eq!(ty, &LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID"));
    let details = lowered
        .gaps()
        .iter()
        .map(|gap| (gap.owner.as_str(), gap.detail.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        details,
        [
            ("Partial", "unsupported type syntax TS_MAPPED_TYPE"),
            ("Owner", "unsupported type syntax TS_TEMPLATE_LITERAL_TYPE"),
        ]
    );
    Ok(())
}

#[test]
fn regexp_signatures_follow_declarations() -> Result<()> {
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
    let lowered = lower(&[file])?;
    let result = global(&lowered, "RegExpExecArray");
    let LoweredTypeData::Interface(interface) = result.data() else {
        panic!("expected interface")
    };
    let LoweredTypeData::InstanceOf {
        ty,
        type_parameters,
    } = local(result, &interface.extends()[0])
    else {
        panic!("expected applied base")
    };
    assert_eq!(ty, &generated("GLOBAL_ARRAY_ID"));
    assert_eq!(
        type_parameters.as_ref(),
        &[LoweredTypeReference::Predefined("GLOBAL_NUMBER_KEYWORD_ID")]
    );
    let regexp = global(&lowered, "RegExp");
    let LoweredTypeData::Class(class) = regexp.data() else {
        panic!("expected class")
    };
    let LoweredTypeData::Function(exec) =
        local(regexp, class.member("exec").unwrap().type_reference())
    else {
        panic!("expected exec function")
    };
    assert_eq!(
        exec.parameters()[0].type_reference(),
        &exec.type_parameters()[0]
    );
    assert_eq!(exec.return_type(), &exec.type_parameters()[0]);
    assert!(exec.parameters()[0].is_optional());
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
        assert_instance_of(regexp, result, "GLOBAL_REG_EXP_ID");
    }
    Ok(())
}
