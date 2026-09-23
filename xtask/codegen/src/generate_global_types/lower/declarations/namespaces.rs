use super::*;
use crate::generate_global_types::collect::ScopePath;
use biome_js_syntax::{AnyTsName, TsReferenceType, TsTypeAliasDeclaration};

/// Lowers namespace variables and their interface/alias dependencies into one local table.
/// Dependencies precede their users; recursive declarations return an error.
pub(in crate::generate_global_types::lower) fn lower_namespace(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    path: &[&str],
    id_constant: &'static str,
) -> Result<Option<LoweredGlobal>> {
    let scope = ScopePath::Namespace(
        path.iter()
            .map(|part| Text::from((*part).to_owned()))
            .collect(),
    );
    let groups = manifest.groups_in_scope(&scope).collect::<Vec<_>>();
    if groups.is_empty() {
        return Ok(None);
    }
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(source_files),
        namespace_scope: path.iter().map(|part| (*part).to_owned()).collect(),
        scoped_types: BTreeMap::new(),
        active_declarations: BTreeSet::new(),
        interfaces: BTreeMap::new(),
        pending: Vec::new(),
        types: Vec::new(),
        class_scope: None,
        declaration_parameters: BTreeMap::new(),
        unbound_parameters: BTreeSet::new(),
        predefined_declarations: false,
    };
    let mut members = Vec::new();
    for group in groups {
        for record in group.declarations() {
            if !matches!(record.kind, DeclarationKind::VariableDeclarator { .. }) {
                if matches!(
                    record.kind,
                    DeclarationKind::DeclareFunction | DeclarationKind::ImportEquals
                ) {
                    bail!(
                        "unsupported namespace value declaration {}",
                        record.declared_name
                    );
                }
                continue;
            }
            let declaration = lowerer
                .sources
                .find_variable_declarator(record)?
                .context("missing namespace variable")?;
            let Some(AnyTsVariableAnnotation::TsTypeAnnotation(annotation)) =
                declaration.variable_annotation()
            else {
                bail!("namespace variable requires a type annotation");
            };
            let reference = lowerer.lower_reference(&annotation.ty()?)?;
            let type_reference =
                lowerer.lower_variable_type(record.declared_name.text(), reference)?;
            let member = LoweredTypeMember {
                name: Text::from(record.declared_name.clone()),
                kind: LoweredMemberKind::Named { optional: false },
                type_reference,
            };
            if let Some(previous) = members
                .iter()
                .find(|previous: &&LoweredTypeMember| previous.name == member.name)
            {
                if previous != &member {
                    bail!("conflicting namespace variable {}", member.name);
                }
            } else {
                members.push(member);
            }
        }
    }
    Ok(Some(LoweredGlobal {
        name: Text::from(path.join(".")),
        id_constant: id_constant.into(),
        data: LoweredTypeData::Object(members.into_boxed_slice()),
        local_types: lowerer
            .types
            .into_iter()
            .collect::<Option<Box<[_]>>>()
            .context("unfilled namespace type")?,
    }))
}

impl DeclarationLowerer<'_> {
    fn lower_variable_type(
        &mut self,
        name: &str,
        reference: LoweredTypeReference,
    ) -> Result<LoweredTypeReference> {
        let LoweredTypeReference::Local(index) = &reference else {
            return Ok(reference);
        };
        let Some(Some(LoweredTypeData::Interface(interface))) = self.types.get(*index) else {
            return Ok(reference);
        };
        if !interface
            .members
            .iter()
            .any(|member| member.kind == LoweredMemberKind::Constructor)
        {
            return Ok(reference);
        }
        if !interface.extends.is_empty() || !interface.type_parameters.is_empty() {
            bail!("constructor variable {name} requires a non-generic interface without bases");
        }
        let mut members = interface.members.clone();
        for member in &mut members {
            member.kind = match &member.kind {
                LoweredMemberKind::Named { optional: false } => LoweredMemberKind::NamedStatic,
                LoweredMemberKind::ComputedValue { key_reference } => {
                    LoweredMemberKind::ComputedStatic {
                        key_reference: key_reference.clone(),
                    }
                }
                LoweredMemberKind::Constructor | LoweredMemberKind::CallSignature => {
                    member.kind.clone()
                }
                _ => bail!("unsupported constructor variable member {}", member.name),
            };
        }
        Ok(self.register(LoweredTypeData::Class(LoweredClass {
            name: Text::from(format!("{}.{name}", self.namespace_scope.join("."))),
            type_parameters: Box::default(),
            members,
        })))
    }

    pub(super) fn declaration_scope(&self) -> ScopePath {
        if self.namespace_scope.is_empty() {
            ScopePath::Global
        } else {
            ScopePath::Namespace(
                self.namespace_scope
                    .iter()
                    .cloned()
                    .map(Text::from)
                    .collect(),
            )
        }
    }

    pub(super) fn lower_scoped_reference(
        &mut self,
        reference: &TsReferenceType,
    ) -> Result<Option<LoweredTypeReference>> {
        let mut path = type_name_path(reference.name()?)?;
        let name = path.pop().context("empty type name")?;
        if path.is_empty()
            && (self.namespace_scope.is_empty()
                || self.declaration_parameters.contains_key(name.as_str())
                || self.unbound_parameters.contains(name.as_str()))
        {
            return Ok(None);
        }
        if path.is_empty() {
            path.clone_from(&self.namespace_scope);
        }
        let scope = ScopePath::Namespace(path.iter().cloned().map(Text::from).collect());
        let Some(group) = self.manifest.group(&scope, &name) else {
            if matches!(reference.name()?, AnyTsName::JsReferenceIdentifier(_)) {
                return Ok(None);
            }
            bail!(
                "unresolved qualified type reference {}.{}",
                path.join("."),
                name
            );
        };
        if reference.type_arguments().is_some() {
            bail!("namespace declaration type arguments are not supported");
        }
        let key = format!("{}.{}", path.join("."), name);
        if let Some(ty) = self.scoped_types.get(&key) {
            return Ok(Some(ty.clone()));
        }
        if !self.active_declarations.insert(key.clone()) {
            bail!("recursive declaration {key} requires a forward local reference");
        }
        let outer_scope = std::mem::replace(&mut self.namespace_scope, path);
        let outer_class = self.class_scope.take();
        let outer_parameters = std::mem::take(&mut self.declaration_parameters);
        let outer_unbound = std::mem::take(&mut self.unbound_parameters);
        let result = (|| {
            let aliases = group
                .declarations()
                .iter()
                .filter(|record| record.kind == DeclarationKind::TypeAlias)
                .collect::<Vec<_>>();
            if !aliases.is_empty() {
                if aliases.len() != 1
                    || group
                        .declarations()
                        .iter()
                        .any(|record| record.kind == DeclarationKind::Interface)
                {
                    bail!("conflicting alias declaration {key}");
                }
                let record = aliases[0];
                let module = self.sources.module_for(record)?;
                let alias = module
                    .syntax()
                    .descendants()
                    .find(|node| {
                        node.kind() == record.syntax_kind
                            && node.text_trimmed_range() == record.text_range
                    })
                    .and_then(TsTypeAliasDeclaration::cast)
                    .context("missing type alias")?;
                if alias.type_parameters().is_some() {
                    bail!("namespace alias type parameters are not supported");
                }
                self.lower_reference(&alias.ty()?)
            } else {
                if !group.has_role(GlobalDeclarationRole::Type) {
                    bail!("expected type declaration {key}");
                }
                let mut interface = self.lower_interface(&name)?;
                interface.name = Text::from(key.clone());
                Ok(self.register(LoweredTypeData::Interface(interface)))
            }
        })();
        self.namespace_scope = outer_scope;
        self.class_scope = outer_class;
        self.declaration_parameters = outer_parameters;
        self.unbound_parameters = outer_unbound;
        self.active_declarations.remove(&key);
        let ty = result.with_context(|| format!("in declaration {key}"))?;
        self.scoped_types.insert(key, ty.clone());
        Ok(Some(ty))
    }
}

fn type_name_path(name: AnyTsName) -> Result<Vec<String>> {
    match name {
        AnyTsName::JsReferenceIdentifier(name) => {
            Ok(vec![name.value_token()?.text_trimmed().to_owned()])
        }
        AnyTsName::TsQualifiedName(name) => {
            let mut path = type_name_path(name.left()?)?;
            path.push(name.right()?.value_token()?.text_trimmed().to_owned());
            Ok(path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_global_types::{
        collect::collect, manifest::build_global_manifest, source::CanonicalPath,
    };

    #[test]
    fn namespace_variables_lower_merged_interfaces_and_scoped_aliases() -> Result<()> {
        for name in ["First", "Second"] {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(
                    root,
                    "tests/fixtures/global-types/lowering.interfaces.d.ts",
                )?,
                repo_relative: "namespace.d.ts".to_owned(),
                bytes: format!(
                    r#"
                    interface Value {{ unrelated: number; }}
                    declare namespace Other {{ interface Value {{ enabled: boolean; }} }}
                    declare namespace {name} {{
                        interface Base {{ label: string; }}
                        interface Value extends Base {{ own: string; }}
                        type Result = Value;
                        interface Registry {{ text: never; number: never; }}
                        type Key = keyof Registry;
                        type Choice = {{}} extends Registry ? boolean : Key;
                        interface Value {{ choice: Choice; }}
                        interface Factory {{ new(): Result; (): Result; }}
                        var create: Factory;
                    }}
                    declare namespace {name} {{
                        interface Value {{ foreign: Other.Value; }}
                        var repeated: Factory;
                    }}
                "#
                )
                .into_bytes(),
            };
            let manifest = build_global_manifest(collect(&file).records);
            let global =
                lower_namespace(&manifest, &[file], &[name], "TEST_ID_GLOBAL_TYPE_ID")?.unwrap();
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local reference")
                };
                &global.local_types()[*index]
            };
            let LoweredTypeData::Object(members) = global.data() else {
                panic!("expected namespace object")
            };
            assert_eq!(members.len(), 2);
            let LoweredTypeData::Class(repeated) = local(members[1].type_reference()) else {
                panic!("expected repeated factory")
            };
            let LoweredTypeData::Class(factory) = local(members[0].type_reference()) else {
                panic!("expected factory")
            };
            assert_eq!(factory.members(), repeated.members());
            let LoweredTypeData::Constructor(constructor) =
                local(factory.members()[0].type_reference())
            else {
                panic!("expected constructor")
            };
            let LoweredTypeData::Function(call) = local(factory.members()[1].type_reference())
            else {
                panic!("expected call")
            };
            assert_eq!(constructor.return_type(), Some(call.return_type()));
            let LoweredTypeData::Interface(value) = local(call.return_type()) else {
                panic!("expected value")
            };
            assert_eq!(value.name(), format!("{name}.Value"));
            assert_eq!(value.members().len(), 3);
            let LoweredTypeData::Union(branches) =
                local(value.member("choice").unwrap().type_reference())
            else {
                panic!("expected conditional branches")
            };
            assert_eq!(branches.len(), 2);
            assert_eq!(local(&branches[0]), &LoweredTypeData::Boolean);
            let LoweredTypeData::Keyof(registry) = local(&branches[1]) else {
                panic!("expected alias to keyof registry")
            };
            let LoweredTypeData::Interface(registry) = local(registry) else {
                panic!("expected registry")
            };
            assert_eq!(registry.members().len(), 2);
            for member in registry.members() {
                assert_eq!(
                    local(member.type_reference()),
                    &LoweredTypeData::NeverKeyword
                );
            }
            assert_eq!(
                value.member("own").unwrap().type_reference(),
                &LoweredTypeReference::Predefined("GLOBAL_STRING_ID")
            );
            let LoweredTypeData::Interface(base) = local(&value.extends()[0]) else {
                panic!("expected base")
            };
            assert_eq!(base.name(), format!("{name}.Base"));
            let LoweredTypeData::Interface(foreign) =
                local(value.member("foreign").unwrap().type_reference())
            else {
                panic!("expected foreign interface")
            };
            assert_eq!(foreign.name(), "Other.Value");
            assert_eq!(
                local(foreign.member("enabled").unwrap().type_reference()),
                &LoweredTypeData::Boolean
            );
        }
        Ok(())
    }
}
