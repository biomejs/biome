use std::collections::BTreeMap;

use super::*;

/// A self-contained local type table for selected declarations and their dependencies.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredDeclarations {
    types: Box<[LoweredTypeData]>,
    interfaces: BTreeMap<String, usize>,
}

impl LoweredDeclarations {
    /// Type data in the order addressed by `LoweredTypeReference::Local`.
    pub fn types(&self) -> &[LoweredTypeData] {
        &self.types
    }

    /// Returns the local reference for a converted interface, including dependencies.
    pub fn interface_reference(&self, name: &str) -> Option<LoweredTypeReference> {
        self.interfaces
            .get(name)
            .copied()
            .map(LoweredTypeReference::Local)
    }
}

/// Converts selected interfaces and all interfaces referenced by their members or bases.
///
/// Names are looked up in the manifest's global scope. Merged declarations retain source
/// order. Recursive references share local table entries. Unsupported syntax in selected
/// declarations or their dependencies, and unresolved references, return errors.
/// No global IDs or runtime name registrations are allocated.
pub fn lower_interfaces(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    names: &[&str],
) -> Result<LoweredDeclarations> {
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(source_files),
        interfaces: BTreeMap::new(),
        pending: Vec::new(),
        types: Vec::new(),
    };
    for name in names {
        lowerer.named_reference(name)?;
    }
    let mut next = 0;
    while next < lowerer.pending.len() {
        let (name, index) = lowerer.pending[next].clone();
        let interface = lowerer
            .lower_interface(&name)
            .with_context(|| format!("while lowering interface {name}"))?;
        lowerer.types[index] = Some(LoweredTypeData::Interface(interface));
        next += 1;
    }
    Ok(LoweredDeclarations {
        types: lowerer
            .types
            .into_iter()
            .collect::<Option<Box<[_]>>>()
            .context("unfilled declaration type table entry")?,
        interfaces: lowerer.interfaces,
    })
}

struct DeclarationLowerer<'a> {
    manifest: &'a GlobalManifest,
    sources: ParsedSourceCache<'a>,
    interfaces: BTreeMap<String, usize>,
    pending: Vec<(String, usize)>,
    types: Vec<Option<LoweredTypeData>>,
}

impl DeclarationLowerer<'_> {
    fn named_reference(&mut self, name: &str) -> Result<LoweredTypeReference> {
        if let Some(index) = self.interfaces.get(name) {
            return Ok(LoweredTypeReference::Local(*index));
        }
        let group = self
            .manifest
            .global_group(name)
            .with_context(|| format!("unresolved type reference {name}"))?;
        if !group
            .declarations()
            .iter()
            .any(|record| record.kind == DeclarationKind::Interface)
        {
            bail!("unsupported declaration for type reference {name}: expected an interface");
        }
        let index = self.types.len();
        self.types.push(None);
        self.interfaces.insert(name.to_owned(), index);
        self.pending.push((name.to_owned(), index));
        Ok(LoweredTypeReference::Local(index))
    }

    fn register(&mut self, data: LoweredTypeData) -> LoweredTypeReference {
        let index = self.types.len();
        self.types.push(Some(data));
        LoweredTypeReference::Local(index)
    }

    fn lower_interface(&mut self, name: &str) -> Result<LoweredInterface> {
        let group = self
            .manifest
            .global_group(name)
            .with_context(|| format!("unresolved type reference {name}"))?;
        let mut extends = Vec::new();
        let mut members = Vec::new();
        for record in group.declarations() {
            match record.kind {
                DeclarationKind::Interface => {}
                DeclarationKind::TypeAlias => {
                    bail!("unsupported type alias merged with interface {name}")
                }
                _ => continue,
            }
            let declaration = self
                .sources
                .find_interface_declaration(record)?
                .with_context(|| format!("missing interface declaration {name}"))?;
            if declaration.type_parameters().is_some() {
                bail!("unsupported type parameters on interface {name}");
            }
            if let Some(clause) = declaration.extends_clause() {
                for base in clause.types() {
                    extends.push(self.lower_reference(&AnyTsType::TsReferenceType(base?))?);
                }
            }
            for member in declaration.members() {
                let member = self.lower_member(member)?;
                if members
                    .iter()
                    .any(|previous: &LoweredTypeMember| previous.name == member.name)
                {
                    bail!("unsupported duplicate member {name}.{}", member.name);
                }
                members.push(member);
            }
        }
        Ok(LoweredInterface {
            name: Text::from(name.to_owned()),
            extends: extends.into_boxed_slice(),
            members: members.into_boxed_slice(),
        })
    }

    fn lower_member(&mut self, member: AnyTsTypeMember) -> Result<LoweredTypeMember> {
        match member {
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                let name = lower_object_member_name(property.name()?)?;
                let ty = property
                    .type_annotation()
                    .with_context(|| format!("property {name} is missing a type annotation"))?
                    .ty()?;
                let type_reference = self
                    .lower_reference(&ty)
                    .with_context(|| format!("in property {name}"))?;
                Ok(LoweredTypeMember {
                    name,
                    kind: LoweredMemberKind::Named {
                        optional: property.optional_token().is_some(),
                    },
                    type_reference,
                })
            }
            AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                let name = lower_object_member_name(method.name()?)?;
                if method.type_parameters().is_some() {
                    bail!("unsupported type parameters on method {name}");
                }
                let function = self
                    .lower_function(
                        Some(name.clone()),
                        method.parameters()?,
                        method
                            .return_type_annotation()
                            .with_context(|| format!("method {name} is missing a return type"))?
                            .ty()?,
                    )
                    .with_context(|| format!("in method {name}"))?;
                Ok(LoweredTypeMember {
                    name,
                    kind: LoweredMemberKind::Named {
                        optional: method.optional_token().is_some(),
                    },
                    type_reference: self.register(LoweredTypeData::Function(function)),
                })
            }
            _ => bail!("unsupported interface member: {:?}", member.syntax().kind()),
        }
    }

    fn lower_function(
        &mut self,
        name: Option<Text>,
        parameters: JsParameters,
        return_type: AnyTsReturnType,
    ) -> Result<LoweredFunction> {
        let parameters = lower_parameters_with(parameters, &mut |ty| self.lower_reference(ty))?;
        let return_type = self.lower_reference(&regular_return_type(return_type, "function")?)?;
        Ok(LoweredFunction {
            is_async: false,
            type_parameters: Box::default(),
            name,
            parameters,
            return_type,
        })
    }

    fn lower_reference(&mut self, ty: &AnyTsType) -> Result<LoweredTypeReference> {
        if let Some(reference) = lower_primitive_reference(ty) {
            return Ok(reference);
        }
        match ty {
            AnyTsType::TsBooleanType(_) => Ok(self.register(LoweredTypeData::Boolean)),
            AnyTsType::TsNullLiteralType(_) => Ok(self.register(LoweredTypeData::Null)),
            AnyTsType::TsReferenceType(reference) => {
                if reference.type_arguments().is_some() {
                    bail!("unsupported type arguments in type reference");
                }
                let biome_js_syntax::AnyTsName::JsReferenceIdentifier(name) = reference.name()?
                else {
                    bail!("unsupported qualified type reference");
                };
                self.named_reference(name.value_token()?.text_trimmed())
            }
            AnyTsType::TsParenthesizedType(parenthesized) => {
                self.lower_reference(&parenthesized.ty()?)
            }
            AnyTsType::TsUnionType(union) => {
                let types = union
                    .types()
                    .into_iter()
                    .map(|ty| self.lower_reference(&ty?))
                    .collect::<Result<Box<[_]>>>()?;
                Ok(self.register(LoweredTypeData::Union(types)))
            }
            AnyTsType::TsStringLiteralType(literal) => Ok(self.register(
                LoweredTypeData::StringLiteral(Text::from(literal.inner_string_text()?)),
            )),
            AnyTsType::TsFunctionType(function) => {
                if function.type_parameters().is_some() {
                    bail!("unsupported function type parameters");
                }
                let function =
                    self.lower_function(None, function.parameters()?, function.return_type()?)?;
                Ok(self.register(LoweredTypeData::Function(function)))
            }
            _ => bail!("unsupported type syntax: {:?}", ty.syntax().kind()),
        }
    }
}
