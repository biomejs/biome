use super::*;
use biome_js_syntax::TsDeclareFunctionDeclaration;
use biome_string_case::Case;

/// Discovers top-level function groups in declaration order and derives IDs from their names.
pub(in crate::generate_global_types::lower) fn lower_function_globals(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
) -> Result<Vec<LoweredGlobal>> {
    manifest
        .groups_in_scope(&crate::generate_global_types::collect::ScopePath::Global)
        .filter_map(|group| {
            group
                .declarations()
                .iter()
                .find(|record| record.kind == DeclarationKind::DeclareFunction)
                .map(|record| record.declared_name.text())
        })
        .map(|name| {
            lower_function_global(
                manifest,
                source_files,
                name,
                Text::from(format!(
                    "{}_ID_GLOBAL_TYPE_ID",
                    Case::Constant.convert(name)
                )),
            )
        })
        .collect()
}

/// Lowers a function declaration group, retaining overloads in source order.
/// Signatures share a local type table; generic parameters remain signature-scoped.
fn lower_function_global(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    name: &str,
    id_constant: Text,
) -> Result<LoweredGlobal> {
    let group = manifest
        .global_group(name)
        .with_context(|| format!("missing function declaration group {name}"))?;
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(source_files),
        namespace_scope: Vec::new(),
        scoped_types: BTreeMap::new(),
        active_declarations: BTreeSet::new(),
        interfaces: BTreeMap::new(),
        pending: Vec::new(),
        types: Vec::new(),
        class_scope: None,
        declaration_parameters: BTreeMap::new(),
        unbound_parameters: BTreeSet::new(),
        predefined_declarations: true,
    };
    let mut functions = Vec::new();
    for record in group.declarations() {
        if record.kind != DeclarationKind::DeclareFunction {
            bail!("expected function declaration for {name}");
        }
        let module = lowerer.sources.module_for(record)?;
        let declaration = module
            .syntax()
            .descendants()
            .find(|node| {
                node.kind() == record.syntax_kind && node.text_trimmed_range() == record.text_range
            })
            .and_then(TsDeclareFunctionDeclaration::cast)
            .with_context(|| format!("missing function declaration {name}"))?;
        let mut function = lowerer
            .lower_signature(
                Some(Text::from(record.declared_name.clone())),
                declaration.type_parameters(),
                declaration.parameters()?,
                declaration
                    .return_type_annotation()
                    .with_context(|| format!("function {name} is missing a return type"))?
                    .ty()?,
            )
            .with_context(|| format!("in function {name} from {}", record.file_repo_relative))?;
        function.is_async = declaration.async_token().is_some();
        functions.push(function);
    }
    let data = if functions.len() == 1 {
        LoweredTypeData::Function(functions.pop().unwrap())
    } else {
        LoweredTypeData::Object(
            functions
                .into_iter()
                .map(|function| LoweredTypeMember {
                    name: Text::default(),
                    kind: LoweredMemberKind::CallSignature,
                    type_reference: lowerer.register(LoweredTypeData::Function(function)),
                })
                .collect(),
        )
    };
    Ok(LoweredGlobal {
        name: Text::from(name.to_owned()),
        id_constant,
        data,
        local_types: lowerer
            .types
            .into_iter()
            .collect::<Option<Box<[_]>>>()
            .context("unfilled function type")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_global_types::{
        collect::collect, manifest::build_global_manifest, source::CanonicalPath,
    };

    #[test]
    fn function_declarations_preserve_generic_overloads_and_parameters() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for name in ["convertValue", "mapValues"] {
            let mut files = Vec::new();
            for (index, signature) in [
                "<T extends string = string>(value?: T): T",
                "<T extends boolean>(map: (value: T) => T, ...values: T[]): T[]",
            ]
            .iter()
            .enumerate()
            {
                let other = if index == 0 {
                    "declare function readFlag(input: number): boolean;"
                } else {
                    ""
                };
                files.push(DiscoveredFile {
                    path: CanonicalPath::from_within(
                        root,
                        "tests/fixtures/global-types/lowering.interfaces.d.ts",
                    )?,
                    repo_relative: format!("functions{index}.d.ts"),
                    bytes: format!("declare function {name}{signature}; {other}").into_bytes(),
                });
            }
            let manifest = build_global_manifest(
                files
                    .iter()
                    .flat_map(|file| collect(file).records)
                    .collect(),
            );
            let lowered = lower_global_types(&manifest, &files)?;
            let global = lowered.global(name).expect("expected lowered function");
            assert_eq!(lowered.globals().len(), 2);
            let other = lowered
                .global("readFlag")
                .expect("expected second function");
            let LoweredTypeData::Function(function) = other.data() else {
                panic!("expected single signature")
            };
            assert_eq!(function.name(), Some("readFlag"));
            assert_eq!(function.parameters().len(), 1);
            assert_eq!(
                function.parameters()[0].type_reference(),
                &LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID")
            );
            let LoweredTypeReference::Local(index) = function.return_type() else {
                panic!("expected local return type")
            };
            assert_eq!(other.local_types()[*index], LoweredTypeData::Boolean);
            let LoweredTypeData::Object(signatures) = global.data() else {
                panic!("expected overloads")
            };
            assert_eq!(signatures.len(), 2);
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                &global.local_types()[*index]
            };
            let mut generics = Vec::new();
            for (index, signature) in signatures.iter().enumerate() {
                assert_eq!(signature.kind(), &LoweredMemberKind::CallSignature);
                let LoweredTypeData::Function(function) = local(signature.type_reference()) else {
                    panic!("expected function")
                };
                assert_eq!(function.name(), Some(name));
                let [generic] = function.type_parameters() else {
                    panic!("expected one generic parameter")
                };
                generics.push(generic);
                let LoweredTypeData::GenericParameter {
                    constraint,
                    default,
                    ..
                } = local(generic)
                else {
                    panic!("expected generic parameter")
                };
                if index == 0 {
                    assert_eq!(
                        constraint,
                        &Some(LoweredTypeReference::Predefined("GLOBAL_STRING_ID"))
                    );
                    assert_eq!(default, constraint);
                    assert_eq!(function.parameters().len(), 1);
                    let parameter = &function.parameters()[0];
                    assert_eq!(
                        parameter.binding(),
                        &LoweredFunctionParameterBinding::Named(Text::from("value"))
                    );
                    assert!(parameter.is_optional());
                    assert!(!parameter.is_rest());
                    assert_eq!(parameter.type_reference(), generic);
                    assert_eq!(function.return_type(), generic);
                } else {
                    assert_eq!(
                        local(constraint.as_ref().unwrap()),
                        &LoweredTypeData::Boolean
                    );
                    assert_eq!(default, &None);
                    assert_eq!(function.parameters().len(), 2);
                    let LoweredTypeData::Function(callback) =
                        local(function.parameters()[0].type_reference())
                    else {
                        panic!("expected callback")
                    };
                    assert_eq!(callback.parameters()[0].type_reference(), generic);
                    assert_eq!(callback.return_type(), generic);
                    let rest = &function.parameters()[1];
                    assert!(rest.is_rest());
                    assert!(!rest.is_optional());
                    assert_eq!(rest.type_reference(), function.return_type());
                    let LoweredTypeData::InstanceOf {
                        ty,
                        type_parameters,
                    } = local(function.return_type())
                    else {
                        panic!("expected array")
                    };
                    assert_eq!(ty, &LoweredTypeReference::Predefined("GLOBAL_ARRAY_ID"));
                    assert_eq!(type_parameters.as_ref(), std::slice::from_ref(generic));
                }
            }
            assert_ne!(generics[0], generics[1]);
        }
        Ok(())
    }
}
