use std::collections::{BTreeMap, BTreeSet};

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
///
/// Member types support primitive keywords, boolean/number/bigint/string literals,
/// global interface references, arrays, parentheses, unions, and nongeneric function types.
/// Type aliases, type arguments, qualified references, object and template literal types,
/// and type operators such as `unique symbol` are excluded.
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
        class_scope: None,
        declaration_parameters: BTreeMap::new(),
        unbound_parameters: BTreeSet::new(),
        predefined_declarations: false,
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
    class_scope: Option<ClassScope>,
    declaration_parameters: BTreeMap<Text, LoweredTypeReference>,
    // Local constraints must precede their users in the runtime table; unbound names
    // must not fall through to global lookup while those constraints are lowered.
    unbound_parameters: BTreeSet<Text>,
    predefined_declarations: bool,
}

struct ClassScope {
    name: Text,
    reference: &'static str,
    parameters: BTreeMap<Text, LoweredTypeReference>,
}

impl DeclarationLowerer<'_> {
    fn named_reference(&mut self, name: &str) -> Result<LoweredTypeReference> {
        if let Some(reference) = self.declaration_parameters.get(name) {
            return Ok(reference.clone());
        }
        if self.predefined_declarations {
            self.manifest
                .global_group(name)
                .with_context(|| format!("missing declaration dependency {name}"))?;
            return ITERATOR_DECLARATIONS
                .iter()
                .find(|(declared, _, _)| *declared == name)
                .map(|(_, _, reference)| LoweredTypeReference::Predefined(reference))
                .with_context(|| format!("unsupported declaration dependency {name}"));
        }
        if let Some(scope) = &self.class_scope {
            return scope
                .parameters
                .get(name)
                .cloned()
                .with_context(|| format!("unsupported class member type reference {name}"));
        }
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

    /// Reuses structurally equal entries without changing existing local indices.
    fn register(&mut self, data: LoweredTypeData) -> LoweredTypeReference {
        if let Some(index) = self.types.iter().position(|ty| ty.as_ref() == Some(&data)) {
            return LoweredTypeReference::Local(index);
        }
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
            if declaration.type_parameters().is_some() && !self.predefined_declarations {
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
            type_parameters: Box::default(),
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
        if let Some(data) = lower_scalar_type(ty)? {
            return Ok(self.register(data));
        }
        match ty {
            AnyTsType::TsArrayType(array) => {
                let element = self.lower_reference(&array.element_type()?)?;
                Ok(self.register(LoweredTypeData::InstanceOf {
                    ty: LoweredTypeReference::Predefined("GLOBAL_ARRAY_ID"),
                    type_parameters: Box::new([element]),
                }))
            }
            AnyTsType::TsThisType(_) if self.class_scope.is_some() => {
                Ok(self.register(LoweredTypeData::ThisKeyword))
            }
            AnyTsType::TsReferenceType(reference) => {
                let biome_js_syntax::AnyTsName::JsReferenceIdentifier(name) = reference.name()?
                else {
                    bail!("unsupported qualified type reference");
                };
                let name = name.value_token()?;
                let name = name.text_trimmed();
                if self.unbound_parameters.contains(name) {
                    bail!("type parameter {name} requires a forward or recursive local reference");
                }
                if name == "WeakKey"
                    && reference.type_arguments().is_none()
                    && !self.declaration_parameters.contains_key(name)
                    && !self
                        .class_scope
                        .as_ref()
                        .is_some_and(|scope| scope.parameters.contains_key(name))
                    && (self.class_scope.is_some() || self.predefined_declarations)
                {
                    // TODO: Derive WeakKey from WeakKeyTypes[keyof WeakKeyTypes] in lib.es5.d.ts.
                    // This requires alias dependencies, object, keyof, and indexed-access lowering,
                    // including merged WeakKeyTypes members from the selected library profile.
                    return Ok(self.register(LoweredTypeData::ObjectKeyword));
                }
                if let Some(scope) = &self.class_scope
                    && scope.name == name
                    && !scope.parameters.contains_key(name)
                {
                    let ty = LoweredTypeReference::Predefined(scope.reference);
                    let type_parameters = reference
                        .type_arguments()
                        .map(|arguments| {
                            arguments
                                .ts_type_argument_list()
                                .into_iter()
                                .map(|ty| self.lower_reference(&ty?))
                                .collect::<Result<Box<[_]>>>()
                        })
                        .transpose()?
                        .unwrap_or_default();
                    return Ok(self.register(LoweredTypeData::InstanceOf {
                        ty,
                        type_parameters,
                    }));
                }
                if self.predefined_declarations && !self.declaration_parameters.contains_key(name) {
                    let ty = self.named_reference(name)?;
                    let type_parameters = reference
                        .type_arguments()
                        .map(|arguments| {
                            arguments
                                .ts_type_argument_list()
                                .into_iter()
                                .map(|ty| self.lower_reference(&ty?))
                                .collect::<Result<Box<[_]>>>()
                        })
                        .transpose()?
                        .unwrap_or_default();
                    return Ok(self.register(LoweredTypeData::InstanceOf {
                        ty,
                        type_parameters,
                    }));
                }
                if reference.type_arguments().is_some() {
                    bail!("unsupported type arguments in type reference");
                }
                self.named_reference(name)
            }
            AnyTsType::TsTupleType(tuple) if self.predefined_declarations => {
                let elements = tuple
                    .elements()
                    .into_iter()
                    .map(|element| {
                        let biome_js_syntax::AnyTsTupleTypeElement::AnyTsType(ty) = element? else {
                            bail!("named, optional, and rest tuple elements are not supported");
                        };
                        self.lower_reference(&ty)
                    })
                    .collect::<Result<Box<[_]>>>()?;
                Ok(self.register(LoweredTypeData::Tuple(elements)))
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

fn lower_scalar_type(ty: &AnyTsType) -> Result<Option<LoweredTypeData>> {
    let data = match ty {
        AnyTsType::TsAnyType(_) => LoweredTypeData::AnyKeyword,
        AnyTsType::TsBigintType(_) => LoweredTypeData::BigInt,
        AnyTsType::TsBooleanType(_) => LoweredTypeData::Boolean,
        AnyTsType::TsNeverType(_) => LoweredTypeData::NeverKeyword,
        AnyTsType::TsNullLiteralType(_) => LoweredTypeData::Null,
        AnyTsType::TsSymbolType(_) => LoweredTypeData::Symbol,
        AnyTsType::TsUndefinedType(_) => LoweredTypeData::Undefined,
        AnyTsType::TsUnknownType(_) => LoweredTypeData::UnknownKeyword,
        AnyTsType::TsBooleanLiteralType(literal) => {
            LoweredTypeData::BooleanLiteral(literal.literal()?.kind() == T![true])
        }
        AnyTsType::TsNumberLiteralType(literal) => LoweredTypeData::NumberLiteral(
            signed_literal_text(literal.minus_token().is_some(), literal.literal_token()?),
        ),
        AnyTsType::TsBigintLiteralType(literal) => LoweredTypeData::BigIntLiteral(
            signed_literal_text(literal.minus_token().is_some(), literal.literal_token()?),
        ),
        AnyTsType::TsStringLiteralType(literal) => {
            LoweredTypeData::StringLiteral(Text::from(literal.inner_string_text()?))
        }
        _ => return Ok(None),
    };
    Ok(Some(data))
}

fn signed_literal_text(negative: bool, token: biome_js_syntax::JsSyntaxToken) -> Text {
    if negative {
        Text::from(format!("-{}", token.text_trimmed()))
    } else {
        Text::from(token.token_text_trimmed())
    }
}

/// Lowers named instance properties and nongeneric methods with declaration-derived
/// generic parameters. Constraints use supported member types and earlier type parameters;
/// the first interface supplies constraints for merged declarations. Defaults,
/// value-side declarations, computed members, methods returning `MapIterator` or
/// `SetIterator`, and methods referencing Intl types
/// are excluded. References to the
/// enclosing class may carry type arguments. Other external references and unsupported
/// member shapes are errors.
/// `this` remains a keyword; lowering does not bind it to a call receiver.
/// Existing class members retain their projections; their declarations are not lowered again.
/// Local types are registered after their dependencies for runtime conversion in one pass.
pub(super) fn lower_class_members(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    class: &mut LoweredClass,
    class_reference: &'static str,
) -> Result<Box<[LoweredTypeData]>> {
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(source_files),
        interfaces: BTreeMap::new(),
        pending: Vec::new(),
        types: Vec::new(),
        declaration_parameters: BTreeMap::new(),
        unbound_parameters: BTreeSet::new(),
        predefined_declarations: false,
        class_scope: Some(ClassScope {
            name: class.name.clone(),
            reference: class_reference,
            parameters: BTreeMap::new(),
        }),
    };
    let group = manifest
        .global_group(class.name())
        .context("missing class declaration group")?;
    let mut members = class.members.to_vec();
    let mut class_parameters = None;
    for record in group.declarations() {
        match record.kind {
            DeclarationKind::Interface => {}
            DeclarationKind::TypeAlias => {
                bail!("unsupported type alias merged with class {}", class.name())
            }
            _ => continue,
        }
        let declaration = lowerer
            .sources
            .find_interface_declaration(record)?
            .context("missing class interface")?;
        if declaration.extends_clause().is_some() {
            bail!("unsupported extends clause on class {}", class.name());
        }
        let names = declaration
            .type_parameters()
            .map(|parameters| {
                parameters
                    .items()
                    .into_iter()
                    .map(|parameter| {
                        Ok(Text::from(
                            parameter?.name()?.ident_token()?.token_text_trimmed(),
                        ))
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();
        if class_parameters.is_none() {
            lowerer.unbound_parameters = names.iter().cloned().collect();
            let mut references = Vec::new();
            if let Some(parameters) = declaration.type_parameters() {
                for parameter in parameters.items() {
                    let parameter = parameter?;
                    let name = Text::from(parameter.name()?.ident_token()?.token_text_trimmed());
                    let constraint = parameter
                        .constraint()
                        .map(|constraint| lowerer.lower_reference(&constraint.ty()?))
                        .transpose()
                        .with_context(|| format!("in constraint of {}.{name}", class.name()))?;
                    let reference = lowerer.register(LoweredTypeData::GenericParameter {
                        name: name.clone(),
                        constraint,
                        default: None,
                    });
                    lowerer.unbound_parameters.remove(&name);
                    if let Some(scope) = &mut lowerer.class_scope {
                        scope.parameters.insert(name, reference.clone());
                    }
                    references.push(reference);
                }
            }
            class_parameters = Some(references.into_boxed_slice());
        }
        let references = class_parameters
            .as_ref()
            .context("missing class parameters")?;
        if names.len() != references.len() {
            bail!(
                "inconsistent type parameter count across merged class {} declarations",
                class.name()
            );
        }
        if let Some(scope) = &mut lowerer.class_scope {
            scope.parameters = names.into_iter().zip(references.iter().cloned()).collect();
        }
        for member in declaration.members() {
            if !supports_class_member(&member, class)? {
                continue;
            }
            let member = lowerer.lower_member(member).with_context(|| {
                format!("in {} from {}", class.name(), record.file_repo_relative)
            })?;
            if members.iter().any(|previous| previous.name == member.name) {
                bail!(
                    "unsupported duplicate member {}.{}",
                    class.name(),
                    member.name
                );
            }
            members.push(member);
        }
    }
    class.type_parameters =
        class_parameters.context("class must include an interface declaration")?;
    class.members = members.into_boxed_slice();
    lowerer
        .types
        .into_iter()
        .collect::<Option<Box<[_]>>>()
        .context("unfilled class member type")
}

fn supports_class_member(member: &AnyTsTypeMember, class: &LoweredClass) -> Result<bool> {
    let name = match member {
        AnyTsTypeMember::TsPropertySignatureTypeMember(property) => property.name()?,
        AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
            if method_uses_intl_types(method)? {
                return Ok(false);
            }
            if let Some(annotation) = method.return_type_annotation()
                && let AnyTsReturnType::AnyTsType(AnyTsType::TsReferenceType(reference)) =
                    annotation.ty()?
                && reference.type_arguments().is_some()
                && let biome_js_syntax::AnyTsName::JsReferenceIdentifier(name) = reference.name()?
                && matches!(
                    name.value_token()?.text_trimmed(),
                    "MapIterator" | "SetIterator"
                )
            {
                return Ok(false);
            }
            method.name()?
        }
        _ => bail!("unsupported class member: {:?}", member.syntax().kind()),
    };
    match name {
        AnyJsObjectMemberName::JsComputedMemberName(_) => Ok(false),
        name => Ok(class
            .member(lower_object_member_name(name)?.text())
            .is_none()),
    }
}

/// Lowers selected constructor-interface members as static members and call signatures of a class.
/// The caller selects members and supplies predefined identities for unique symbols.
/// Other unique symbols use the runtime's symbol type. Function signatures use
/// the same translation as instance methods.
pub(super) fn lower_constructor_members(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    records: &[DeclarationRecord],
    class: &mut LoweredClass,
    class_reference: &'static str,
    select_member: fn(&AnyTsTypeMember) -> Result<bool>,
    predefined_symbols: &[(&str, &'static str)],
) -> Result<Box<[LoweredTypeData]>> {
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(source_files),
        interfaces: BTreeMap::new(),
        pending: Vec::new(),
        types: Vec::new(),
        class_scope: Some(ClassScope {
            name: class.name.clone(),
            reference: class_reference,
            parameters: BTreeMap::new(),
        }),
        declaration_parameters: BTreeMap::new(),
        unbound_parameters: BTreeSet::new(),
        predefined_declarations: false,
    };
    let mut members = Vec::new();
    for record in records {
        let declaration = lowerer
            .sources
            .find_interface_declaration(record)?
            .context("missing constructor interface")?;
        if declaration.extends_clause().is_some() || declaration.type_parameters().is_some() {
            bail!(
                "constructor bases and type parameters are not supported for {}",
                class.name()
            );
        }
        for member in declaration.members() {
            if !select_member(&member)? {
                continue;
            }
            let mut member = match member {
                AnyTsTypeMember::TsCallSignatureTypeMember(signature) => {
                    if signature.type_parameters().is_some() {
                        bail!("generic call signatures are not supported");
                    }
                    let function = lowerer
                        .lower_function(
                            None,
                            signature.parameters()?,
                            signature
                                .return_type_annotation()
                                .context("call signature is missing a return type")?
                                .ty()?,
                        )
                        .with_context(|| {
                            format!(
                                "in {} call signature from {}",
                                class.name(),
                                record.file_repo_relative
                            )
                        })?;
                    LoweredTypeMember {
                        name: Text::default(),
                        kind: LoweredMemberKind::CallSignature,
                        type_reference: lowerer.register(LoweredTypeData::Function(function)),
                    }
                }
                AnyTsTypeMember::TsPropertySignatureTypeMember(property)
                    if is_unique_symbol_property(&property)? =>
                {
                    let name = lower_object_member_name(property.name()?)?;
                    let type_reference = predefined_symbols
                        .iter()
                        .find(|(member_name, _)| *member_name == name.text())
                        .map_or_else(
                            || lowerer.register(LoweredTypeData::Symbol),
                            |(_, reference)| LoweredTypeReference::Predefined(reference),
                        );
                    LoweredTypeMember {
                        name,
                        kind: LoweredMemberKind::Named {
                            optional: property.optional_token().is_some(),
                        },
                        type_reference,
                    }
                }
                member => lowerer.lower_member(member).with_context(|| {
                    format!(
                        "in {} statics from {}",
                        class.name(),
                        record.file_repo_relative
                    )
                })?,
            };
            if member.kind == LoweredMemberKind::CallSignature {
                members.push(member);
                continue;
            }
            // NamedStatic cannot represent an optional member without losing undefined.
            if member.kind != (LoweredMemberKind::Named { optional: false }) {
                bail!(
                    "optional static member {}.{} is not supported",
                    class.name(),
                    member.name
                );
            }
            member.kind = LoweredMemberKind::NamedStatic;
            if members.iter().any(|previous: &LoweredTypeMember| {
                previous.kind == member.kind && previous.name == member.name
            }) {
                bail!(
                    "duplicate static member {}.{} cannot be represented by one member",
                    class.name(),
                    member.name
                );
            }
            members.push(member);
        }
    }
    class.members = members.into_boxed_slice();
    lowerer
        .types
        .into_iter()
        .collect::<Option<Box<[_]>>>()
        .context("unfilled constructor member type")
}

fn method_uses_intl_types(method: &TsMethodSignatureTypeMember) -> Result<bool> {
    for name in method
        .syntax()
        .descendants()
        .filter_map(biome_js_syntax::TsQualifiedName::cast)
    {
        if let biome_js_syntax::AnyTsName::JsReferenceIdentifier(root) = name.left()?
            && root.value_token()?.text_trimmed() == "Intl"
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Named protocol declarations with stable runtime identities. Member selection is syntax-driven.
pub(in crate::generate_global_types) const ITERATOR_DECLARATIONS: &[(&str, &str, &str)] = &[
    (
        "IteratorYieldResult",
        "ITERATOR_YIELD_RESULT_ID_GLOBAL_TYPE_ID",
        "GLOBAL_ITERATOR_YIELD_RESULT_ID",
    ),
    (
        "IteratorReturnResult",
        "ITERATOR_RETURN_RESULT_ID_GLOBAL_TYPE_ID",
        "GLOBAL_ITERATOR_RETURN_RESULT_ID",
    ),
    (
        "IteratorResult",
        "ITERATOR_RESULT_ID_GLOBAL_TYPE_ID",
        "GLOBAL_ITERATOR_RESULT_ID",
    ),
    (
        "Iterator",
        "ITERATOR_ID_GLOBAL_TYPE_ID",
        "GLOBAL_ITERATOR_ID",
    ),
];

/// Lowers the synchronous iterator protocol, including its result dependencies.
/// Constraints use supported member types and earlier type parameters.
/// Merged declarations and dependencies outside this selection are errors.
/// Computed members are excluded. Tuples support required unnamed elements.
pub(super) fn lower_iterator_globals(
    manifest: &GlobalManifest,
    sources: &[DiscoveredFile],
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(sources),
        interfaces: BTreeMap::new(),
        pending: Vec::new(),
        types: Vec::new(),
        class_scope: None,
        declaration_parameters: BTreeMap::new(),
        unbound_parameters: BTreeSet::new(),
        predefined_declarations: true,
    };
    for &(name, id_constant, _) in ITERATOR_DECLARATIONS {
        let Some(group) = manifest.global_group(name) else {
            continue;
        };
        let [record] = group.declarations() else {
            bail!("merged protocol declarations are not supported: {name}");
        };
        let module = lowerer.sources.module_for(record)?;
        let node = module
            .syntax()
            .descendants()
            .find(|node| {
                node.kind() == record.syntax_kind && node.text_trimmed_range() == record.text_range
            })
            .with_context(|| format!("missing declaration {name}"))?;
        let (parameters, alias) =
            if let Some(interface) = TsInterfaceDeclaration::cast(node.clone()) {
                (interface.type_parameters(), None)
            } else if let Some(alias) = biome_js_syntax::TsTypeAliasDeclaration::cast(node) {
                (alias.type_parameters(), Some(alias.ty()?))
            } else {
                bail!("unsupported protocol declaration {name}");
            };
        lowerer.declaration_parameters.clear();
        let mut type_parameters = Vec::new();
        if let Some(parameters) = parameters {
            lowerer.unbound_parameters = parameters
                .items()
                .into_iter()
                .map(|parameter| {
                    Ok(Text::from(
                        parameter?.name()?.ident_token()?.token_text_trimmed(),
                    ))
                })
                .collect::<Result<_>>()?;
            for parameter in parameters.items() {
                let parameter = parameter?;
                if !parameter.modifiers().is_empty() {
                    bail!("unsupported modified protocol type parameter");
                }
                let name = Text::from(parameter.name()?.ident_token()?.token_text_trimmed());
                let constraint = parameter
                    .constraint()
                    .map(|constraint| lowerer.lower_reference(&constraint.ty()?))
                    .transpose()
                    .with_context(|| format!("in constraint of {name}"))?;
                let default = parameter
                    .default()
                    .map(|default| lowerer.lower_reference(&default.ty()?))
                    .transpose()?;
                let reference = lowerer.register(LoweredTypeData::GenericParameter {
                    name: name.clone(),
                    constraint,
                    default,
                });
                lowerer.unbound_parameters.remove(&name);
                if lowerer
                    .declaration_parameters
                    .insert(name, reference.clone())
                    .is_some()
                {
                    bail!("duplicate protocol type parameter");
                }
                type_parameters.push(reference);
            }
        }
        let type_parameters = type_parameters.into_boxed_slice();
        let data = if let Some(alias) = alias {
            let ty = lowerer.lower_reference(&alias)?;
            LoweredTypeData::InstanceOf {
                ty,
                type_parameters,
            }
        } else {
            let mut interface = lowerer.lower_interface(name)?;
            interface.type_parameters = type_parameters;
            LoweredTypeData::Interface(interface)
        };
        let local_types = std::mem::take(&mut lowerer.types)
            .into_iter()
            .collect::<Option<Box<[_]>>>()
            .context("unfilled protocol type")?;
        globals.push(LoweredGlobal {
            local_types,
            name: Text::from(record.declared_name.clone()),
            id_constant,
            data,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_global_types::{
        collect::collect, emit::render_local_types, manifest::build_global_manifest,
        source::CanonicalPath,
    };

    #[test]
    fn array_types_preserve_elements_and_share_references() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for element in ["boolean", "string", "number", "T", "(boolean | T)"] {
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(
                    root,
                    "tests/fixtures/global-types/lowering.interfaces.d.ts",
                )?,
                repo_relative: "arrays.d.ts".to_owned(),
                bytes: format!(
                    "
                    interface Owner<T> {{
                        element: {element};
                        values: {element}[];
                        repeated: {element}[];
                        nested: {element}[][];
                        callback: (values: {element}[]) => {element}[][];
                    }}
                "
                )
                .into_bytes(),
            };
            let manifest = build_global_manifest(collect(&file).records);
            let mut class = LoweredClass {
                name: Text::from("Owner"),
                type_parameters: Box::default(),
                members: Box::default(),
            };
            let types =
                lower_class_members(&manifest, &[file], &mut class, "GLOBAL_TEST_OWNER_ID")?;
            let member_type = |name| class.member(name).unwrap().type_reference();
            let local_index = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                *index
            };
            for (array, element) in [("values", "element"), ("nested", "values")] {
                let index = local_index(member_type(array));
                let LoweredTypeData::InstanceOf {
                    ty,
                    type_parameters,
                } = &types[index]
                else {
                    panic!("expected array instance")
                };
                assert_eq!(ty, &LoweredTypeReference::Predefined("GLOBAL_ARRAY_ID"));
                assert_eq!(
                    type_parameters.as_ref(),
                    std::slice::from_ref(member_type(element))
                );
                if let LoweredTypeReference::Local(element_index) = member_type(element) {
                    assert!(*element_index < index);
                }
            }
            assert_eq!(member_type("values"), member_type("repeated"));
            let LoweredTypeData::Function(callback) = &types[local_index(member_type("callback"))]
            else {
                panic!("expected callback")
            };
            assert_eq!(
                callback.parameters()[0].type_reference(),
                member_type("values")
            );
            assert_eq!(callback.return_type(), member_type("nested"));
        }
        Ok(())
    }

    #[test]
    fn repeated_local_types_share_references() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(
                root,
                "tests/fixtures/global-types/lowering.interfaces.d.ts",
            )?,
            repo_relative: "repeated.d.ts".to_owned(),
            bytes: b"
                interface Node {
                    first: symbol;
                    second: symbol;
                    maybe: symbol | undefined;
                    again: symbol | undefined;
                    callback: (value: symbol) => symbol | undefined;
                    repeatedCallback: (value: symbol) => symbol | undefined;
                    renamedParameter: (other: symbol) => symbol | undefined;
                    next: Node;
                }
            "
            .to_vec(),
        };
        let manifest = build_global_manifest(collect(&file).records);
        let lowered = lower_interfaces(&manifest, &[file], &["Node"])?;
        let node_reference = lowered.interface_reference("Node").unwrap();
        let LoweredTypeReference::Local(index) = node_reference else {
            panic!("expected local interface")
        };
        let LoweredTypeData::Interface(node) = &lowered.types()[index] else {
            panic!("expected interface")
        };
        let member_type = |name| node.member(name).unwrap().type_reference();
        for (first, repeated) in [
            ("first", "second"),
            ("maybe", "again"),
            ("callback", "repeatedCallback"),
        ] {
            assert_eq!(member_type(first), member_type(repeated));
        }
        assert_ne!(member_type("callback"), member_type("renamedParameter"));
        assert_eq!(member_type("next"), &node_reference);
        let LoweredTypeReference::Local(callback_index) = member_type("callback") else {
            panic!("expected local function")
        };
        let LoweredTypeData::Function(callback) = &lowered.types()[*callback_index] else {
            panic!("expected function")
        };
        assert_eq!(
            callback.parameters()[0].type_reference(),
            member_type("first")
        );
        assert_eq!(callback.return_type(), member_type("maybe"));
        for dependency in [
            callback.parameters()[0].type_reference(),
            callback.return_type(),
        ] {
            let LoweredTypeReference::Local(index) = dependency else {
                panic!("expected local dependency")
            };
            assert!(index < callback_index);
        }
        for (index, ty) in lowered.types().iter().enumerate() {
            assert!(
                !lowered.types()[..index].contains(ty),
                "duplicate type {ty:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn constructor_members_translate_without_symbol_selection() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for (owner, constructor, method, scalar, expected) in [
            (
                "First",
                "Factory",
                "create",
                "boolean",
                LoweredTypeData::Boolean,
            ),
            (
                "Second",
                "Registry",
                "lookup",
                "bigint",
                LoweredTypeData::BigInt,
            ),
        ] {
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(
                    root,
                    "tests/fixtures/global-types/lowering.interfaces.d.ts",
                )?,
                repo_relative: "statics.d.ts".to_owned(),
                bytes: format!(
                    "
                    declare var {owner}: {constructor};
                    interface {constructor} {{ readonly key: unique symbol; }}
                    interface {constructor} {{
                        readonly stableKey: unique symbol;
                        {method}(input?: {scalar}): {scalar};
                        (input?: {scalar}): {scalar} | undefined;
                    }}
                "
                )
                .into_bytes(),
            };
            let files = [file];
            let manifest = build_global_manifest(collect(&files[0]).records);
            let mut cache = ParsedSourceCache::new(&files);
            let constructor = resolve_constructor_name(
                owner,
                manifest.global_group(owner).unwrap().declarations(),
                &mut cache,
            )?;
            let mut class = LoweredClass {
                name: Text::from(owner),
                type_parameters: Box::default(),
                members: Box::default(),
            };
            let types = lower_constructor_members(
                &manifest,
                cache.source_files,
                manifest
                    .global_group(constructor.text())
                    .unwrap()
                    .declarations(),
                &mut class,
                "GLOBAL_TEST_OWNER_ID",
                |_| Ok(true),
                &[("stableKey", "GLOBAL_TEST_KEY_ID")],
            )?;
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                &types[*index]
            };
            assert!(
                class
                    .members()
                    .iter()
                    .filter(|member| member.kind() != &LoweredMemberKind::CallSignature)
                    .all(|member| member.kind() == &LoweredMemberKind::NamedStatic)
            );
            assert_eq!(
                local(class.member("key").unwrap().type_reference()),
                &LoweredTypeData::Symbol
            );
            assert_eq!(
                class.member("stableKey").unwrap().type_reference(),
                &LoweredTypeReference::Predefined("GLOBAL_TEST_KEY_ID")
            );
            let LoweredTypeData::Function(function) =
                local(class.member(method).unwrap().type_reference())
            else {
                panic!("expected function")
            };
            assert!(function.parameters()[0].is_optional());
            assert_eq!(local(function.parameters()[0].type_reference()), &expected);
            assert_eq!(local(function.return_type()), &expected);
            let call = class
                .members()
                .iter()
                .find(|member| member.kind() == &LoweredMemberKind::CallSignature)
                .expect("expected call signature");
            let LoweredTypeData::Function(function) = local(call.type_reference()) else {
                panic!("expected function")
            };
            assert_eq!(function.name(), None);
            assert!(function.parameters()[0].is_optional());
            assert_eq!(local(function.parameters()[0].type_reference()), &expected);
            let LoweredTypeData::Union(types) = local(function.return_type()) else {
                panic!("expected union return type")
            };
            assert_eq!(local(&types[0]), &expected);
            assert_eq!(local(&types[1]), &LoweredTypeData::Undefined);
        }
        Ok(())
    }

    #[test]
    fn supporting_arrays_are_scoped_to_each_declaration() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(root, "tests/fixtures/global-types/lowering.interfaces.d.ts")?,
            repo_relative: "owners.d.ts".to_owned(),
            bytes: b"interface First { value: true; } interface Second { value: true; } interface Empty {}".to_vec(),
        };
        let manifest = build_global_manifest(collect(&file).records);
        let mut globals = Vec::new();
        for (name, id_constant) in [
            ("First", "FIRST_ID_GLOBAL_TYPE_ID"),
            ("Second", "SECOND_ID_GLOBAL_TYPE_ID"),
            ("Empty", "EMPTY_ID_GLOBAL_TYPE_ID"),
        ] {
            let mut class = LoweredClass {
                name: Text::from(name),
                type_parameters: Box::default(),
                members: Box::default(),
            };
            let local_types = lower_class_members(
                &manifest,
                std::slice::from_ref(&file),
                &mut class,
                id_constant,
            )?;
            if let Some(member) = class.members().first() {
                assert_eq!(member.type_reference(), &LoweredTypeReference::Local(0));
            }
            globals.push(LoweredGlobal {
                name: class.name.clone(),
                id_constant,
                data: LoweredTypeData::Class(class),
                local_types,
            });
        }
        let first = render_local_types(&globals[..1]);
        let combined = render_local_types(&globals);
        let statics = |source: &str| -> Result<Vec<syn::ItemStatic>> {
            Ok(syn::parse_file(source)?
                .items
                .into_iter()
                .filter_map(|item| {
                    if let syn::Item::Static(item) = item {
                        Some(item)
                    } else {
                        None
                    }
                })
                .collect())
        };
        let first = statics(&first)?;
        let combined = statics(&combined)?;
        assert_eq!(
            combined.len(),
            globals
                .iter()
                .filter(|global| !global.local_types().is_empty())
                .count()
        );
        let render_static = |item: &syn::ItemStatic| {
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: Vec::new(),
                items: vec![syn::Item::Static(item.clone())],
            })
        };
        assert_eq!(
            render_static(&first[0]),
            render_static(&combined[0]),
            "another owner must not grow the first owner's initializer"
        );
        assert_ne!(combined[0].ident, combined[1].ident);
        Ok(())
    }
}
