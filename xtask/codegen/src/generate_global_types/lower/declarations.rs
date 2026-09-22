mod functions;
mod namespaces;

pub(super) use functions::lower_function_globals;
pub(super) use namespaces::lower_namespace;

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
/// global interface references, arrays, required unnamed tuple elements, readonly arrays/tuples,
/// `keyof`, indexed access, parentheses, unions, and function types.
/// Methods, call/construct signatures, and function types may declare type parameters with constraints
/// and defaults using supported types and earlier parameters. Bindings are signature-local.
/// Numeric and string index signatures preserve their key and value types.
/// Computed properties and methods support string/number literals and declared predefined Symbol keys.
/// Optional computed members include undefined in their value type.
/// References to selected predefined types preserve type arguments and require declarations.
/// Namespace interface and alias references and conditional branch unions are supported.
/// Type arguments on other references, global type aliases, object and template literal types,
/// and other type operators such as `unique symbol` are excluded.
pub fn lower_interfaces(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    names: &[&str],
) -> Result<LoweredDeclarations> {
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
    namespace_scope: Vec<String>,
    scoped_types: BTreeMap<String, LoweredTypeReference>,
    active_declarations: BTreeSet<String>,
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
        if let Some(reference) = self
            .class_scope
            .as_ref()
            .and_then(|scope| scope.parameters.get(name))
        {
            return Ok(reference.clone());
        }
        if self.predefined_declarations {
            return self.predefined_reference(name);
        }
        if self.class_scope.is_some() {
            bail!("unsupported class member type reference {name}");
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

    fn predefined_reference(&self, name: &str) -> Result<LoweredTypeReference> {
        self.manifest
            .global_group(name)
            .with_context(|| format!("missing declaration dependency {name}"))?;
        predefined_type_reference(name)
            .map(LoweredTypeReference::Predefined)
            .with_context(|| format!("unsupported declaration dependency {name}"))
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
            .group(&self.declaration_scope(), name)
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
            if declaration.type_parameters().is_some()
                && (!self.predefined_declarations || !self.namespace_scope.is_empty())
            {
                bail!("unsupported type parameters on interface {name}");
            }
            if let Some(clause) = declaration.extends_clause() {
                for base in clause.types() {
                    extends.push(self.lower_reference(&AnyTsType::TsReferenceType(base?))?);
                }
            }
            for member in declaration.members() {
                let member = self.lower_member(member)?;
                if members.iter().any(|previous: &LoweredTypeMember| {
                    previous.name == member.name
                        && !matches!(
                            previous.kind,
                            LoweredMemberKind::CallSignature | LoweredMemberKind::Constructor
                        )
                        && !matches!(
                            member.kind,
                            LoweredMemberKind::CallSignature | LoweredMemberKind::Constructor
                        )
                }) {
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
            AnyTsTypeMember::TsIndexSignatureTypeMember(signature) => {
                let key = signature.parameter()?.type_annotation()?.ty()?;
                if !matches!(key, AnyTsType::TsNumberType(_) | AnyTsType::TsStringType(_)) {
                    bail!("unsupported index signature key type");
                }
                let key_reference = self.lower_reference(&key)?;
                let type_reference = self.lower_reference(&signature.type_annotation()?.ty()?)?;
                Ok(LoweredTypeMember {
                    name: Text::default(),
                    kind: LoweredMemberKind::IndexSignature { key_reference },
                    type_reference,
                })
            }
            AnyTsTypeMember::TsConstructSignatureTypeMember(signature) => {
                let function = self
                    .lower_signature(
                        None,
                        signature.type_parameters(),
                        signature.parameters()?,
                        AnyTsReturnType::AnyTsType(
                            signature
                                .type_annotation()
                                .context("construct signature is missing a return type")?
                                .ty()?,
                        ),
                    )
                    .context("in construct signature")?;
                Ok(LoweredTypeMember {
                    name: Text::from("constructor"),
                    kind: LoweredMemberKind::Constructor,
                    type_reference: self.register(LoweredTypeData::Constructor(
                        LoweredConstructor {
                            type_parameters: function.type_parameters,
                            parameters: function.parameters,
                            return_type: Some(function.return_type),
                        },
                    )),
                })
            }
            AnyTsTypeMember::TsCallSignatureTypeMember(signature) => {
                let function = self
                    .lower_signature(
                        None,
                        signature.type_parameters(),
                        signature.parameters()?,
                        signature
                            .return_type_annotation()
                            .context("call signature is missing a return type")?
                            .ty()?,
                    )
                    .context("in call signature")?;
                Ok(LoweredTypeMember {
                    name: Text::default(),
                    kind: LoweredMemberKind::CallSignature,
                    type_reference: self.register(LoweredTypeData::Function(function)),
                })
            }
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                let optional = property.optional_token().is_some();
                let (name, kind) = self.lower_member_name(property.name()?, optional)?;
                let ty = property
                    .type_annotation()
                    .with_context(|| format!("property {name} is missing a type annotation"))?
                    .ty()?;
                let type_reference = self
                    .lower_reference(&ty)
                    .with_context(|| format!("in property {name}"))?;
                Ok(LoweredTypeMember {
                    name,
                    type_reference: self.lower_member_value(&kind, optional, type_reference),
                    kind,
                })
            }
            AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                let optional = method.optional_token().is_some();
                let (name, kind) = self.lower_member_name(method.name()?, optional)?;
                let function = self
                    .lower_signature(
                        Some(name.clone()),
                        method.type_parameters(),
                        method.parameters()?,
                        method
                            .return_type_annotation()
                            .with_context(|| format!("method {name} is missing a return type"))?
                            .ty()?,
                    )
                    .with_context(|| format!("in method {name}"))?;
                let type_reference = self.register(LoweredTypeData::Function(function));
                Ok(LoweredTypeMember {
                    name,
                    type_reference: self.lower_member_value(&kind, optional, type_reference),
                    kind,
                })
            }
            _ => bail!("unsupported interface member: {:?}", member.syntax().kind()),
        }
    }

    fn merge_overloads(
        &mut self,
        previous: LoweredTypeReference,
        next: LoweredTypeReference,
    ) -> Result<LoweredTypeReference> {
        let LoweredTypeReference::Local(index) = previous else {
            bail!("overload must reference a local signature");
        };
        let mut signatures = match self.types.get(index).and_then(Option::as_ref) {
            Some(LoweredTypeData::Function(_)) => vec![LoweredTypeMember {
                name: Text::default(),
                kind: LoweredMemberKind::CallSignature,
                type_reference: previous,
            }],
            Some(LoweredTypeData::Object(signatures)) => signatures.to_vec(),
            _ => bail!("expected callable overload"),
        };
        signatures.push(LoweredTypeMember {
            name: Text::default(),
            kind: LoweredMemberKind::CallSignature,
            type_reference: next,
        });
        Ok(self.register(LoweredTypeData::Object(signatures.into_boxed_slice())))
    }

    fn lower_member_name(
        &mut self,
        name: AnyJsObjectMemberName,
        optional: bool,
    ) -> Result<(Text, LoweredMemberKind)> {
        if let AnyJsObjectMemberName::JsComputedMemberName(computed) = &name {
            let expression = computed.expression()?.omit_parentheses();
            if let AnyJsExpression::AnyJsLiteralExpression(literal) = &expression {
                let key = match literal {
                    biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(string) => {
                        LoweredTypeData::StringLiteral(Text::from(string.inner_string_text()?))
                    }
                    biome_js_syntax::AnyJsLiteralExpression::JsNumberLiteralExpression(number) => {
                        LoweredTypeData::NumberLiteral(Text::from(
                            number.value_token()?.token_text_trimmed(),
                        ))
                    }
                    _ => bail!("unsupported computed literal member key"),
                };
                return Ok((
                    Text::from(format!("[{}]", expression.syntax().text_trimmed())),
                    LoweredMemberKind::ComputedValue {
                        key_reference: self.register(key),
                    },
                ));
            }
            let computed = lower_symbol_computed_member_name(name)?;
            self.require_symbol_key(&computed)?;
            return Ok((
                computed.name,
                LoweredMemberKind::ComputedValue {
                    key_reference: computed.key_reference,
                },
            ));
        }
        Ok((
            lower_object_member_name(name)?,
            LoweredMemberKind::Named { optional },
        ))
    }

    fn lower_member_value(
        &mut self,
        kind: &LoweredMemberKind,
        optional: bool,
        reference: LoweredTypeReference,
    ) -> LoweredTypeReference {
        if optional && matches!(kind, LoweredMemberKind::ComputedValue { .. }) {
            let undefined = self.register(LoweredTypeData::Undefined);
            self.register(LoweredTypeData::Union(Box::new([reference, undefined])))
        } else {
            reference
        }
    }

    fn require_symbol_key(&mut self, computed: &ComputedMemberName) -> Result<()> {
        let group = self
            .manifest
            .global_group("Symbol")
            .context("missing Symbol declaration for computed key")?;
        let constructor =
            resolve_constructor_name("Symbol", group.declarations(), &mut self.sources)?;
        let group = self
            .manifest
            .global_group(constructor.text())
            .context("missing Symbol constructor declaration")?;
        for record in group.declarations() {
            let declaration = self
                .sources
                .find_interface_declaration(record)?
                .context("missing Symbol constructor interface")?;
            for member in declaration.members() {
                if let AnyTsTypeMember::TsPropertySignatureTypeMember(property) = member
                    && let AnyJsObjectMemberName::JsLiteralMemberName(name) = property.name()?
                    && format!("[Symbol.{}]", name.name()?) == computed.name.text()
                    && property.optional_token().is_none()
                    && is_unique_symbol_property(&property)?
                {
                    return Ok(());
                }
            }
        }
        bail!(
            "computed key {} requires a declared unique symbol property",
            computed.name
        )
    }

    fn lower_signature(
        &mut self,
        name: Option<Text>,
        type_parameters: Option<TsTypeParameters>,
        parameters: JsParameters,
        return_type: AnyTsReturnType,
    ) -> Result<LoweredFunction> {
        let Some(type_parameters) = type_parameters else {
            return self.lower_function(name, parameters, return_type);
        };
        let outer_parameters = self.declaration_parameters.clone();
        let outer_unbound = self.unbound_parameters.clone();
        let result = (|| {
            let mut names = BTreeSet::new();
            for parameter in type_parameters.items() {
                let parameter = parameter?;
                let name = Text::from(parameter.name()?.ident_token()?.token_text_trimmed());
                if !names.insert(name.clone()) {
                    bail!("duplicate signature type parameter {name}");
                }
            }
            self.unbound_parameters.extend(names);
            let mut references = Vec::new();
            for parameter in type_parameters.items() {
                let parameter = parameter?;
                if parameter
                    .modifiers()
                    .into_iter()
                    .any(|modifier| modifier.as_ts_const_modifier().is_none())
                {
                    bail!("variance modifiers on signature type parameters are not supported");
                }
                let name = Text::from(parameter.name()?.ident_token()?.token_text_trimmed());
                let constraint = parameter
                    .constraint()
                    .map(|constraint| self.lower_reference(&constraint.ty()?))
                    .transpose()
                    .with_context(|| format!("in constraint of {name}"))?;
                let default = parameter
                    .default()
                    .map(|default| self.lower_reference(&default.ty()?))
                    .transpose()
                    .with_context(|| format!("in default of {name}"))?;
                let reference = self.register(LoweredTypeData::GenericParameter {
                    is_const: parameter
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_ts_const_modifier().is_some()),
                    name: name.clone(),
                    constraint,
                    default,
                });
                self.unbound_parameters.remove(&name);
                self.declaration_parameters.insert(name, reference.clone());
                references.push(reference);
            }
            let mut function = self.lower_function(name, parameters, return_type)?;
            function.type_parameters = references.into_boxed_slice();
            Ok(function)
        })();
        self.declaration_parameters = outer_parameters;
        self.unbound_parameters = outer_unbound;
        result
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
                if let Some(ty) = self.lower_scoped_reference(reference)? {
                    return Ok(ty);
                }
                let biome_js_syntax::AnyTsName::JsReferenceIdentifier(name) = reference.name()?
                else {
                    bail!("unsupported qualified type reference");
                };
                let name = name.value_token()?;
                let name = name.text_trimmed();
                if name == "intrinsic"
                    && self.predefined_declarations
                    && !self.declaration_parameters.contains_key(name)
                {
                    return Ok(self.register(LoweredTypeData::UnknownKeyword));
                }
                if reference.type_arguments().is_none()
                    && !self.declaration_parameters.contains_key(name)
                    && let Some(scope) = &self.class_scope
                    && !scope.parameters.contains_key(name)
                    && let Some(group) = self.manifest.global_group(scope.name.text())
                    && group.has_role(GlobalDeclarationRole::Value)
                    && resolve_constructor_name(
                        scope.name.text(),
                        group.declarations(),
                        &mut self.sources,
                    )?
                    .text()
                        == name
                {
                    return Ok(LoweredTypeReference::Predefined(scope.reference));
                }
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
                    && !self.declaration_parameters.contains_key(name)
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
                if (self.predefined_declarations || predefined_type_reference(name).is_some())
                    && !self.declaration_parameters.contains_key(name)
                    && !self
                        .class_scope
                        .as_ref()
                        .is_some_and(|scope| scope.parameters.contains_key(name))
                {
                    let ty = self.predefined_reference(name)?;
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
            AnyTsType::TsConditionalType(conditional) => {
                // Match local inference's conservative union of both conditional branches.
                let yes = self.lower_reference(&conditional.true_type()?)?;
                let no = self.lower_reference(&conditional.false_type()?)?;
                Ok(self.register(LoweredTypeData::Union(Box::new([yes, no]))))
            }
            AnyTsType::TsIndexedAccessType(access) => {
                let object = self.lower_reference(&access.object_type()?)?;
                let index = self.lower_reference(&access.index_type()?)?;
                Ok(self.register(LoweredTypeData::IndexedAccess { object, index }))
            }
            AnyTsType::TsTypeOperatorType(operator)
                if operator.operator_token()?.kind() == T![keyof] =>
            {
                let ty = self.lower_reference(&operator.ty()?)?;
                Ok(self.register(LoweredTypeData::Keyof(ty)))
            }
            AnyTsType::TsTypeOperatorType(operator)
                if operator.operator_token()?.kind() == T![readonly] =>
            {
                let ty = operator.ty()?;
                if !matches!(ty, AnyTsType::TsArrayType(_) | AnyTsType::TsTupleType(_)) {
                    bail!("readonly requires an array or tuple type");
                }
                let ty = self.lower_reference(&ty)?;
                Ok(self.register(LoweredTypeData::Readonly(ty)))
            }
            AnyTsType::TsTupleType(tuple) => {
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
                let function = self.lower_signature(
                    None,
                    function.type_parameters(),
                    function.parameters()?,
                    function.return_type()?,
                )?;
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
        AnyTsType::TsNonPrimitiveType(_) => LoweredTypeData::ObjectKeyword,
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

pub(super) type MemberSelector = fn(&AnyTsTypeMember) -> Result<bool>;

/// Lowers instance properties and methods with declaration-derived
/// generic parameters. Constraints and defaults use supported member types and earlier parameters;
/// the first interface supplies them for merged declarations.
/// Value-side declarations are excluded. References to the
/// enclosing class and selected predefined types may carry type arguments.
/// Other external references and unsupported member shapes are errors.
/// `this` remains a keyword; lowering does not bind it to a call receiver.
/// Existing class members retain their projections; their declarations are not lowered again.
/// Computed constructor members are included alongside the caller's selected members.
/// They share the instance members' local type table and may reference predefined iterator
/// protocol types. Class type parameters are not in scope on the constructor side.
/// Local types are registered after their dependencies for runtime conversion in one pass.
pub(super) fn lower_class_members(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    class: &mut LoweredClass,
    class_reference: &'static str,
    constructors: Option<(&[DeclarationRecord], MemberSelector)>,
) -> Result<Box<[LoweredTypeData]>> {
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(source_files),
        namespace_scope: Vec::new(),
        scoped_types: BTreeMap::new(),
        active_declarations: BTreeSet::new(),
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
    let mut method_names = BTreeSet::new();
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
                    let default = parameter
                        .default()
                        .map(|default| lowerer.lower_reference(&default.ty()?))
                        .transpose()
                        .with_context(|| format!("in default of {}.{name}", class.name()))?;
                    let reference = lowerer.register(LoweredTypeData::GenericParameter {
                        is_const: parameter
                            .modifiers()
                            .into_iter()
                            .any(|modifier| modifier.as_ts_const_modifier().is_some()),
                        name: name.clone(),
                        constraint,
                        default,
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
            let is_method = matches!(member, AnyTsTypeMember::TsMethodSignatureTypeMember(_));
            let member = lowerer.lower_member(member).with_context(|| {
                format!("in {} from {}", class.name(), record.file_repo_relative)
            })?;
            if let Some(previous) = members
                .iter_mut()
                .find(|previous| previous.name == member.name)
            {
                if is_method && method_names.contains(&member.name) {
                    previous.type_reference = lowerer
                        .merge_overloads(previous.type_reference.clone(), member.type_reference)?;
                    continue;
                }
                bail!(
                    "unsupported duplicate member {}.{}",
                    class.name(),
                    member.name
                );
            }
            if is_method {
                method_names.insert(member.name.clone());
            }
            members.push(member);
        }
    }
    class.type_parameters =
        class_parameters.context("class must include an interface declaration")?;
    class.members = members.into_boxed_slice();
    if let Some((records, select_member)) = constructors {
        lowerer.predefined_declarations = true;
        if let Some(scope) = &mut lowerer.class_scope {
            scope.parameters.clear();
        }
        lowerer.lower_constructor_members(records, class, select_member, &[])?;
    }
    lowerer
        .types
        .into_iter()
        .collect::<Option<Box<[_]>>>()
        .context("unfilled class member type")
}

pub(super) fn lower_method_global(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    owner: &str,
    owner_reference: &'static str,
    id_constant: &'static str,
    method: TsMethodSignatureTypeMember,
) -> Result<LoweredGlobal> {
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(source_files),
        namespace_scope: Vec::new(),
        scoped_types: BTreeMap::new(),
        active_declarations: BTreeSet::new(),
        interfaces: BTreeMap::new(),
        pending: Vec::new(),
        types: Vec::new(),
        class_scope: Some(ClassScope {
            name: Text::from(owner.to_owned()),
            reference: owner_reference,
            parameters: BTreeMap::new(),
        }),
        declaration_parameters: BTreeMap::new(),
        unbound_parameters: BTreeSet::new(),
        predefined_declarations: true,
    };
    let name = lower_object_member_name(method.name()?)?;
    let function = lowerer.lower_signature(
        Some(name.clone()),
        method.type_parameters(),
        method.parameters()?,
        method
            .return_type_annotation()
            .with_context(|| format!("method {name} is missing a return type"))?
            .ty()?,
    )?;
    Ok(LoweredGlobal {
        name: Text::from(format!("{owner}.{name}")),
        id_constant: id_constant.into(),
        data: LoweredTypeData::Function(function),
        local_types: lowerer
            .types
            .into_iter()
            .collect::<Option<Box<[_]>>>()
            .context("unfilled method type")?,
    })
}

fn is_computed_member(member: &AnyTsTypeMember) -> Result<bool> {
    let name = match member {
        AnyTsTypeMember::TsPropertySignatureTypeMember(property) => property.name()?,
        AnyTsTypeMember::TsMethodSignatureTypeMember(method) => method.name()?,
        _ => return Ok(false),
    };
    Ok(matches!(
        name,
        AnyJsObjectMemberName::JsComputedMemberName(_)
    ))
}

fn supports_class_member(member: &AnyTsTypeMember, class: &LoweredClass) -> Result<bool> {
    let name = match member {
        AnyTsTypeMember::TsPropertySignatureTypeMember(property) => property.name()?,
        AnyTsTypeMember::TsMethodSignatureTypeMember(method) => method.name()?,
        _ => bail!("unsupported class member: {:?}", member.syntax().kind()),
    };
    match name {
        AnyJsObjectMemberName::JsComputedMemberName(_) => Ok(true),
        name => Ok(class
            .member(lower_object_member_name(name)?.text())
            .is_none()),
    }
}

/// Lowers selected constructor-interface members as statics and call/construct signatures of a class.
/// Computed members are included alongside the caller's selection.
/// The caller supplies predefined identities for unique symbols.
/// Other unique symbols use the runtime's symbol type. Function signatures use
/// the same translation as instance methods. Named method overloads become an object
/// with call signatures in declaration order; duplicate properties remain errors.
pub(super) fn lower_constructor_members(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
    records: &[DeclarationRecord],
    class: &mut LoweredClass,
    class_reference: &'static str,
    select_member: MemberSelector,
    predefined_symbols: &[(&str, &'static str)],
) -> Result<Box<[LoweredTypeData]>> {
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(source_files),
        namespace_scope: Vec::new(),
        scoped_types: BTreeMap::new(),
        active_declarations: BTreeSet::new(),
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
    lowerer.lower_constructor_members(records, class, select_member, predefined_symbols)?;
    lowerer
        .types
        .into_iter()
        .collect::<Option<Box<[_]>>>()
        .context("unfilled constructor member type")
}

impl DeclarationLowerer<'_> {
    fn lower_constructor_members(
        &mut self,
        records: &[DeclarationRecord],
        class: &mut LoweredClass,
        select_member: MemberSelector,
        predefined_symbols: &[(&str, &'static str)],
    ) -> Result<()> {
        let mut members = class.members.to_vec();
        let mut methods = BTreeMap::<Text, Vec<LoweredTypeReference>>::new();
        for record in records {
            let declaration = self
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
                if !select_member(&member)? && !is_computed_member(&member)? {
                    continue;
                }
                let is_method = matches!(member, AnyTsTypeMember::TsMethodSignatureTypeMember(_));
                let mut member = match member {
                    AnyTsTypeMember::TsPropertySignatureTypeMember(property)
                        if is_unique_symbol_property(&property)? =>
                    {
                        let name = lower_object_member_name(property.name()?)?;
                        let type_reference = predefined_symbols
                            .iter()
                            .find(|(member_name, _)| *member_name == name.text())
                            .map_or_else(
                                || self.register(LoweredTypeData::Symbol),
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
                    member => self.lower_member(member).with_context(|| {
                        format!(
                            "in {} statics from {}",
                            class.name(),
                            record.file_repo_relative
                        )
                    })?,
                };
                if matches!(
                    member.kind,
                    LoweredMemberKind::CallSignature | LoweredMemberKind::Constructor
                ) {
                    members.push(member);
                    continue;
                }
                member.kind = match member.kind {
                    LoweredMemberKind::Named { optional: false } => LoweredMemberKind::NamedStatic,
                    LoweredMemberKind::ComputedValue { key_reference } => {
                        LoweredMemberKind::ComputedStatic { key_reference }
                    }
                    _ => bail!(
                        "optional static member {}.{} is not supported",
                        class.name(),
                        member.name
                    ),
                };
                if members.iter().any(|previous: &LoweredTypeMember| {
                    previous.kind == member.kind && previous.name == member.name
                }) {
                    if is_method && let Some(signatures) = methods.get_mut(&member.name) {
                        signatures.push(member.type_reference);
                        continue;
                    }
                    bail!(
                        "duplicate static member {}.{} cannot be represented by one member",
                        class.name(),
                        member.name
                    );
                }
                if is_method {
                    methods.insert(member.name.clone(), vec![member.type_reference.clone()]);
                }
                members.push(member);
            }
        }
        for member in &mut members {
            if matches!(
                member.kind,
                LoweredMemberKind::NamedStatic | LoweredMemberKind::ComputedStatic { .. }
            ) && let Some(signatures) = methods.remove(&member.name)
                && signatures.len() > 1
            {
                member.type_reference = self.register(LoweredTypeData::Object(
                    signatures
                        .into_iter()
                        .map(|type_reference| LoweredTypeMember {
                            name: Text::default(),
                            kind: LoweredMemberKind::CallSignature,
                            type_reference,
                        })
                        .collect(),
                ));
            }
        }
        class.members = members.into_boxed_slice();
        Ok(())
    }
}

/// Selects named members without interpreting their signatures or types.
pub(super) fn select_named_members(member: &AnyTsTypeMember, names: &[&str]) -> Result<bool> {
    let name = match member {
        AnyTsTypeMember::TsMethodSignatureTypeMember(method) => method.name()?,
        AnyTsTypeMember::TsPropertySignatureTypeMember(property) => property.name()?,
        AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => getter.name()?,
        AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => setter.name()?,
        _ => return Ok(false),
    };
    if matches!(name, AnyJsObjectMemberName::JsComputedMemberName(_)) {
        return Ok(false);
    }
    Ok(names.contains(&lower_object_member_name(name)?.text()))
}

fn predefined_type_reference(name: &str) -> Option<&'static str> {
    PREDEFINED_DECLARATIONS
        .iter()
        .find_map(|&(declared, _, reference)| (declared == name).then_some(reference))
}

/// Selected declaration names and their existing runtime identities.
pub(in crate::generate_global_types) const PREDEFINED_DECLARATIONS: &[(&str, &str, &str)] = &[
    ("Array", "ARRAY_ID_GLOBAL_TYPE_ID", "GLOBAL_ARRAY_ID"),
    ("Date", "DATE_ID_GLOBAL_TYPE_ID", "GLOBAL_DATE_ID"),
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
    (
        "Iterable",
        "ITERABLE_ID_GLOBAL_TYPE_ID",
        "GLOBAL_ITERABLE_ID",
    ),
    (
        "Symbol.iterator",
        "SYMBOL_ITERATOR_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_ITERATOR_ID",
    ),
    (
        "RegExpExecArray",
        "REGEXP_EXEC_ARRAY_ID_GLOBAL_TYPE_ID",
        "GLOBAL_REGEXP_EXEC_ARRAY_ID",
    ),
    (
        "ArrayLike",
        "ARRAY_LIKE_ID_GLOBAL_TYPE_ID",
        "GLOBAL_ARRAY_LIKE_ID",
    ),
    (
        "Symbol.toStringTag",
        "SYMBOL_TO_STRING_TAG_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_TO_STRING_TAG_ID",
    ),
    (
        "Symbol.toPrimitive",
        "SYMBOL_TO_PRIMITIVE_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_TO_PRIMITIVE_ID",
    ),
    (
        "Symbol.match",
        "SYMBOL_MATCH_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_MATCH_ID",
    ),
    (
        "Symbol.replace",
        "SYMBOL_REPLACE_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_REPLACE_ID",
    ),
    (
        "Symbol.search",
        "SYMBOL_SEARCH_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_SEARCH_ID",
    ),
    (
        "Symbol.split",
        "SYMBOL_SPLIT_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_SPLIT_ID",
    ),
    (
        "Symbol.species",
        "SYMBOL_SPECIES_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_SPECIES_ID",
    ),
    (
        "Symbol.hasInstance",
        "SYMBOL_HAS_INSTANCE_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_HAS_INSTANCE_ID",
    ),
    (
        "Symbol.unscopables",
        "SYMBOL_UNSCOPABLES_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SYMBOL_UNSCOPABLES_ID",
    ),
    (
        "RegExpMatchArray",
        "REGEXP_MATCH_ARRAY_ID_GLOBAL_TYPE_ID",
        "GLOBAL_REGEXP_MATCH_ARRAY_ID",
    ),
    (
        "IteratorObject",
        "ITERATOR_OBJECT_ID_GLOBAL_TYPE_ID",
        "GLOBAL_ITERATOR_OBJECT_ID",
    ),
    (
        "MapIterator",
        "MAP_ITERATOR_ID_GLOBAL_TYPE_ID",
        "GLOBAL_MAP_ITERATOR_ID",
    ),
    (
        "SetIterator",
        "SET_ITERATOR_ID_GLOBAL_TYPE_ID",
        "GLOBAL_SET_ITERATOR_ID",
    ),
    (
        "BuiltinIteratorReturn",
        "BUILTIN_ITERATOR_RETURN_ID_GLOBAL_TYPE_ID",
        "GLOBAL_BUILTIN_ITERATOR_RETURN_ID",
    ),
    (
        "Disposable",
        "DISPOSABLE_ID_GLOBAL_TYPE_ID",
        "GLOBAL_DISPOSABLE_ID",
    ),
];

/// Lowers selected type-only declarations and their declared bases and members.
/// Constraints use supported member types and earlier type parameters.
/// Merged interfaces share the first declaration's type parameters.
/// Dependencies without predefined identities are errors.
/// Computed members support literal and declared predefined Symbol keys. Tuples support required unnamed elements.
pub(super) fn lower_predefined_declarations(
    manifest: &GlobalManifest,
    sources: &[DiscoveredFile],
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let mut lowerer = DeclarationLowerer {
        manifest,
        sources: ParsedSourceCache::new(sources),
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
    for &(name, id_constant, _) in PREDEFINED_DECLARATIONS {
        if !matches!(
            name,
            "ArrayLike"
                | "RegExpExecArray"
                | "IteratorYieldResult"
                | "IteratorReturnResult"
                | "IteratorResult"
                | "Iterator"
                | "Iterable"
                | "RegExpMatchArray"
                | "IteratorObject"
                | "MapIterator"
                | "SetIterator"
                | "BuiltinIteratorReturn"
        ) {
            continue;
        }
        let Some(group) = manifest.global_group(name) else {
            continue;
        };
        let record = group
            .declarations()
            .first()
            .context("missing declaration")?;
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
                    is_const: false,
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
            id_constant: id_constant.into(),
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
    fn const_signature_parameters_survive_lowering_and_emission() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for modifier in ["", "const "] {
            for signature in [
                format!("<{modifier}T>(value: T): T"),
                format!("method<{modifier}T>(value: T): T"),
                format!("callback: <{modifier}T>(value: T) => T"),
                format!("new <{modifier}T>(value: T): T"),
            ] {
                let file = DiscoveredFile {
                    path: CanonicalPath::from_within(
                        root,
                        "tests/fixtures/global-types/lowering.interfaces.d.ts",
                    )?,
                    repo_relative: "const.d.ts".to_owned(),
                    bytes: format!("interface Owner {{ {signature}; }}").into_bytes(),
                };
                let manifest = build_global_manifest(collect(&file).records);
                let table = lower_interfaces(&manifest, &[file], &["Owner"])?;
                let expected = !modifier.is_empty();
                let parameter = table
                    .types()
                    .iter()
                    .find_map(|ty| match ty {
                        LoweredTypeData::GenericParameter { is_const, .. } => Some(*is_const),
                        _ => None,
                    })
                    .expect("signature must declare a generic parameter");
                assert_eq!(parameter, expected);
                let emitted = crate::generate_global_types::emit::render_declarations(&table);
                assert!(emitted.contains(&format!("is_const: {expected}")));
            }
        }
        Ok(())
    }

    #[test]
    fn readonly_arrays_and_tuples_preserve_nested_elements() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for element in ["T", "string", "(T | number)"] {
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(
                    root,
                    "tests/fixtures/global-types/lowering.interfaces.d.ts",
                )?,
                repo_relative: "readonly.d.ts".to_owned(),
                bytes: format!(
                    "interface Owner<T> {{
                    element: {element};
                    array: {element}[];
                    frozenArray: readonly {element}[];
                    tuple: [{element}, {element}[]];
                    frozenTuple: readonly [{element}, {element}[]];
                    repeated: readonly [{element}, {element}[]];
                    nested: readonly (readonly [{element}, {element}[]])[];
                }}"
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
                lower_class_members(&manifest, &[file], &mut class, "GLOBAL_TEST_OWNER_ID", None)?;
            let member = |name| class.member(name).unwrap().type_reference();
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                &types[*index]
            };
            for (frozen, mutable) in [("frozenArray", "array"), ("frozenTuple", "tuple")] {
                assert_eq!(
                    local(member(frozen)),
                    &LoweredTypeData::Readonly(member(mutable).clone())
                );
                assert_ne!(member(frozen), member(mutable));
            }
            let LoweredTypeData::Tuple(elements) = local(member("tuple")) else {
                panic!("expected tuple")
            };
            assert_eq!(
                elements.as_ref(),
                [member("element").clone(), member("array").clone()]
            );
            assert_eq!(member("frozenTuple"), member("repeated"));
            let LoweredTypeData::Readonly(array) = local(member("nested")) else {
                panic!("expected readonly outer array")
            };
            let LoweredTypeData::InstanceOf {
                ty,
                type_parameters,
            } = local(array)
            else {
                panic!("expected array")
            };
            assert_eq!(ty, &LoweredTypeReference::Predefined("GLOBAL_ARRAY_ID"));
            assert_eq!(
                type_parameters.as_ref(),
                std::slice::from_ref(member("frozenTuple"))
            );
            for (index, ty) in types.iter().enumerate() {
                if let LoweredTypeData::Readonly(LoweredTypeReference::Local(inner)) = ty {
                    assert!(
                        *inner < index,
                        "readonly dependencies must precede their wrappers"
                    );
                }
            }
        }
        Ok(())
    }

    #[test]
    fn readonly_emission_preserves_the_operator_and_operand() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(
                root,
                "tests/fixtures/global-types/lowering.interfaces.d.ts",
            )?,
            repo_relative: "readonly.d.ts".to_owned(),
            bytes:
                b"interface Owner { array: readonly string[]; tuple: readonly [string, number]; }"
                    .to_vec(),
        };
        let manifest = build_global_manifest(collect(&file).records);
        let table = lower_interfaces(&manifest, &[file], &["Owner"])?;
        let emitted: syn::ExprCall = syn::parse_str(
            &crate::generate_global_types::emit::render_declarations(&table),
        )?;
        let syn::Expr::Array(entries) = &emitted.args[0] else {
            panic!("expected emitted table")
        };
        for (data, expression) in table.types().iter().zip(&entries.elems) {
            let LoweredTypeData::Readonly(LoweredTypeReference::Local(index)) = data else {
                continue;
            };
            let syn::Expr::Call(variant) = expression else {
                panic!("expected type operator variant")
            };
            let syn::Expr::Call(boxed) = &variant.args[0] else {
                panic!("expected boxed operator")
            };
            let syn::Expr::Struct(fields) = &boxed.args[0] else {
                panic!("expected operator data")
            };
            for (name, expected) in [
                ("operator", "crate::TypeOperator::Readonly".to_owned()),
                (
                    "ty",
                    format!("crate::RawTypeId::Local(crate::TypeId::new({index})).into()"),
                ),
            ] {
                let expression = &fields.fields.iter().find(|field| matches!(&field.member, syn::Member::Named(member) if member == name)).unwrap().expr;
                let expected: syn::Expr = syn::parse_str(&expected)?;
                assert_eq!(
                    quote::quote!(#expression).to_string(),
                    quote::quote!(#expected).to_string()
                );
            }
        }
        Ok(())
    }

    #[test]
    fn computed_protocol_methods_preserve_generic_arguments() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for element in ["Element", "Value"] {
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(root, "tests/fixtures/global-types/lowering.interfaces.d.ts")?,
                repo_relative: "protocol.d.ts".to_owned(),
                bytes: format!("
                    declare var Symbol: Keys;
                    interface Keys {{ readonly iterator: unique symbol; }}
                    interface Iterator<T> {{ value: T; }}
                    interface Iterable<{element}> {{ [Symbol.iterator](fallback: {element}): Iterator<{element}>; }}
                    interface Owner<T> {{ value: T; }}
                    interface Factory {{ new<U>(values: Iterable<U>): Owner<U>; }}
                ").into_bytes(),
            };
            let files = [file];
            let manifest = build_global_manifest(collect(&files[0]).records);
            let mut globals = Vec::new();
            lower_predefined_declarations(&manifest, &files, &mut globals)?;
            let global = globals
                .iter()
                .find(|global| global.name() == "Iterable")
                .unwrap();
            let LoweredTypeData::Interface(interface) = global.data() else {
                panic!("expected interface")
            };
            let member = interface.members().first().unwrap();
            assert_eq!(
                member.kind(),
                &LoweredMemberKind::ComputedValue {
                    key_reference: LoweredTypeReference::Predefined("GLOBAL_SYMBOL_ITERATOR_ID"),
                }
            );
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                &global.local_types()[*index]
            };
            let LoweredTypeData::Function(function) = local(member.type_reference()) else {
                panic!("expected method")
            };
            let LoweredTypeData::InstanceOf {
                ty,
                type_parameters,
            } = local(function.return_type())
            else {
                panic!("expected generic return")
            };
            assert_eq!(ty, &LoweredTypeReference::Predefined("GLOBAL_ITERATOR_ID"));
            assert_eq!(type_parameters.as_ref(), interface.type_parameters());
            assert_eq!(
                function.parameters()[0].type_reference(),
                &interface.type_parameters()[0]
            );
            let emitted = render_local_types(&globals);
            syn::parse_file(&emitted)?;
            let mut class = LoweredClass {
                name: Text::from("Owner"),
                type_parameters: Box::default(),
                members: Box::default(),
            };
            let types = lower_class_members(
                &manifest,
                &files,
                &mut class,
                "GLOBAL_TEST_OWNER_ID",
                Some((
                    manifest.global_group("Factory").unwrap().declarations(),
                    |member| {
                        Ok(matches!(
                            member,
                            AnyTsTypeMember::TsConstructSignatureTypeMember(_)
                        ))
                    },
                )),
            )?;
            let constructor = types
                .iter()
                .find_map(|ty| match ty {
                    LoweredTypeData::Constructor(constructor) => Some(constructor),
                    _ => None,
                })
                .unwrap();
            let LoweredTypeReference::Local(index) = constructor.parameters()[0].type_reference()
            else {
                panic!("expected local parameter")
            };
            let LoweredTypeData::InstanceOf {
                ty,
                type_parameters,
            } = &types[*index]
            else {
                panic!("expected protocol instance")
            };
            assert_eq!(ty, &LoweredTypeReference::Predefined("GLOBAL_ITERABLE_ID"));
            assert_eq!(type_parameters.as_ref(), constructor.type_parameters());
        }
        Ok(())
    }

    #[test]
    fn computed_keys_require_a_declared_symbol_identity() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for declaration in ["", "iterator: string;", "iterator?: unique symbol;"] {
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(root, "tests/fixtures/global-types/lowering.interfaces.d.ts")?,
                repo_relative: "keys.d.ts".to_owned(),
                bytes: format!("declare var Symbol: Keys; interface Keys {{ {declaration} }} interface Owner {{ [Symbol.iterator](): void; }}").into_bytes(),
            };
            let manifest = build_global_manifest(collect(&file).records);
            let error = lower_interfaces(&manifest, &[file], &["Owner"]).unwrap_err();
            assert!(
                format!("{error:#}").contains("requires a declared unique symbol property"),
                "{error:#}"
            );
        }
        Ok(())
    }

    #[test]
    fn global_constructors_share_instance_type_tables() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for owner in ["Map", "Set", "WeakMap", "Date"] {
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(
                    root,
                    "tests/fixtures/global-types/lowering.interfaces.d.ts",
                )?,
                repo_relative: "constructors.d.ts".to_owned(),
                bytes: format!(
                    "interface {owner}<T> {{ current: T; }}
                    declare var {owner}: Factory;
                    interface Factory {{ new<U>(value: U): {owner}<U>; }}
                    interface Factory {{ new<U>(value: readonly U[]): {owner}<U>; }}"
                )
                .into_bytes(),
            };
            let manifest = build_global_manifest(collect(&file).records);
            let lowered = super::lower_global_types(&manifest, &[file])?;
            let global = lowered
                .globals()
                .iter()
                .find(|global| global.name() == owner)
                .unwrap();
            let LoweredTypeData::Class(class) = global.data() else {
                panic!("expected class")
            };
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local reference")
                };
                &global.local_types()[*index]
            };
            assert_eq!(
                class.member("current").unwrap().type_reference(),
                &class.type_parameters()[0]
            );
            let constructors = class
                .members()
                .iter()
                .filter(|member| member.kind() == &LoweredMemberKind::Constructor)
                .collect::<Vec<_>>();
            assert_eq!(constructors.len(), 2);
            for member in constructors {
                let LoweredTypeData::Constructor(constructor) = local(member.type_reference())
                else {
                    panic!("expected constructor")
                };
                let LoweredTypeData::InstanceOf {
                    type_parameters, ..
                } = local(constructor.return_type().unwrap())
                else {
                    panic!("expected owner instance")
                };
                assert_eq!(type_parameters.as_ref(), constructor.type_parameters());
                assert!(matches!(
                    local(&constructor.type_parameters()[0]),
                    LoweredTypeData::GenericParameter { .. }
                ));
            }
            for (index, ty) in global.local_types().iter().enumerate() {
                assert!(!global.local_types()[..index].contains(ty));
            }
        }
        Ok(())
    }

    #[test]
    fn construct_signatures_preserve_generic_parameters_and_overloads() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(
                root,
                "tests/fixtures/global-types/lowering.interfaces.d.ts",
            )?,
            repo_relative: "constructors.d.ts".to_owned(),
            bytes: b"interface Factory {
                constructor: boolean;
                new <T extends string = string, U extends T = T>(value?: U): U;
                <T extends string = string, U extends T = T>(value?: U): U;
                new <V>(value: V): V;
            }"
            .to_vec(),
        };
        let files = [file];
        let manifest = build_global_manifest(collect(&files[0]).records);
        let table = lower_interfaces(&manifest, &files, &["Factory"])?;
        let LoweredTypeReference::Local(index) = table.interface_reference("Factory").unwrap()
        else {
            panic!("expected interface reference")
        };
        let LoweredTypeData::Interface(interface) = &table.types()[index] else {
            panic!("expected interface")
        };
        let mut class = LoweredClass {
            name: Text::from("Factory"),
            type_parameters: Box::default(),
            members: Box::default(),
        };
        let static_types = lower_constructor_members(
            &manifest,
            &files,
            manifest.global_group("Factory").unwrap().declarations(),
            &mut class,
            "GLOBAL_TEST_OWNER_ID",
            |_| Ok(true),
            &[],
        )?;
        for (members, types) in [
            (interface.members(), table.types()),
            (class.members(), static_types.as_ref()),
        ] {
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                &types[*index]
            };
            let calls = members
                .iter()
                .filter(|member| member.kind() == &LoweredMemberKind::Constructor)
                .collect::<Vec<_>>();
            let [first, second] = calls.as_slice() else {
                panic!("expected both constructors")
            };
            assert_ne!(first.type_reference(), second.type_reference());
            let LoweredTypeData::Constructor(constructor) = local(first.type_reference()) else {
                panic!("expected constructor")
            };
            let call = members
                .iter()
                .find(|member| member.kind() == &LoweredMemberKind::CallSignature)
                .unwrap();
            let LoweredTypeData::Function(function) = local(call.type_reference()) else {
                panic!("expected call signature")
            };
            assert_eq!(constructor.type_parameters(), function.type_parameters());
            assert_eq!(constructor.parameters(), function.parameters());
            assert_eq!(constructor.return_type(), Some(function.return_type()));
            for call in calls {
                let LoweredTypeData::Constructor(constructor) = local(call.type_reference()) else {
                    panic!("expected constructor")
                };
                let parameter = constructor.type_parameters().last().unwrap();
                assert_eq!(constructor.parameters()[0].type_reference(), parameter);
                assert_eq!(constructor.return_type(), Some(parameter));
            }
        }
        let emitted: syn::ExprCall = syn::parse_str(
            &crate::generate_global_types::emit::render_declarations(&table),
        )?;
        let syn::Expr::Array(entries) = &emitted.args[0] else {
            panic!("expected emitted table")
        };
        for (data, expression) in table.types().iter().zip(&entries.elems) {
            let LoweredTypeData::Constructor(constructor) = data else {
                continue;
            };
            let syn::Expr::Call(variant) = expression else {
                panic!("expected constructor variant")
            };
            let syn::Expr::Call(boxed) = &variant.args[0] else {
                panic!("expected boxed constructor")
            };
            let syn::Expr::Struct(fields) = &boxed.args[0] else {
                panic!("expected constructor data")
            };
            let parameters = fields.fields.iter().find(|field| matches!(&field.member, syn::Member::Named(name) if name == "type_parameters")).unwrap();
            let syn::Expr::Call(boxed) = &parameters.expr else {
                panic!("expected emitted generic parameters")
            };
            let syn::Expr::Array(references) = &boxed.args[0] else {
                panic!("expected generic parameter references")
            };
            assert_eq!(references.elems.len(), constructor.type_parameters().len());
            for (expression, reference) in
                references.elems.iter().zip(constructor.type_parameters())
            {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local generic parameter")
                };
                let expected: syn::Expr = syn::parse_str(&format!(
                    "crate::RawTypeId::Local(crate::TypeId::new({index})).into()"
                ))?;
                assert_eq!(
                    quote::quote!(#expression).to_string(),
                    quote::quote!(#expected).to_string()
                );
            }
        }
        Ok(())
    }

    #[test]
    fn construct_signatures_apply_type_arguments_to_the_owner() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(
                root,
                "tests/fixtures/global-types/lowering.interfaces.d.ts",
            )?,
            repo_relative: "constructors.d.ts".to_owned(),
            bytes: b"interface Factory { new <T = string>(value: T): Owner<T>; }".to_vec(),
        };
        let files = [file];
        let manifest = build_global_manifest(collect(&files[0]).records);
        let mut class = LoweredClass {
            name: Text::from("Owner"),
            type_parameters: Box::default(),
            members: Box::default(),
        };
        let types = lower_constructor_members(
            &manifest,
            &files,
            manifest.global_group("Factory").unwrap().declarations(),
            &mut class,
            "GLOBAL_TEST_OWNER_ID",
            |_| Ok(true),
            &[],
        )?;
        let local = |reference: &LoweredTypeReference| {
            let LoweredTypeReference::Local(index) = reference else {
                panic!("expected local type")
            };
            &types[*index]
        };
        let LoweredTypeData::Constructor(constructor) = local(class.members()[0].type_reference())
        else {
            panic!("expected constructor")
        };
        let LoweredTypeData::InstanceOf {
            ty,
            type_parameters,
        } = local(constructor.return_type().unwrap())
        else {
            panic!("expected instance")
        };
        assert_eq!(
            ty,
            &LoweredTypeReference::Predefined("GLOBAL_TEST_OWNER_ID")
        );
        assert_eq!(type_parameters.as_ref(), constructor.type_parameters());
        assert_eq!(
            constructor.parameters()[0].type_reference(),
            &type_parameters[0]
        );
        Ok(())
    }

    #[test]
    fn generic_function_types_and_calls_share_translation() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(
                root,
                "tests/fixtures/global-types/lowering.interfaces.d.ts",
            )?,
            repo_relative: "signatures.d.ts".to_owned(),
            bytes: b"interface Owner {
                identity: <T extends string = string>(value: T) => T;
                <T extends string = string>(value: T): T;
                <U extends boolean>(value: U): U;
            }"
            .to_vec(),
        };
        let files = [file];
        let manifest = build_global_manifest(collect(&files[0]).records);
        let table = lower_interfaces(&manifest, &files, &["Owner"])?;
        let LoweredTypeReference::Local(index) = table.interface_reference("Owner").unwrap() else {
            panic!("expected interface reference")
        };
        let LoweredTypeData::Interface(interface) = &table.types()[index] else {
            panic!("expected interface")
        };
        let mut class = LoweredClass {
            name: Text::from("Owner"),
            type_parameters: Box::default(),
            members: Box::default(),
        };
        let static_types = lower_constructor_members(
            &manifest,
            &files,
            manifest.global_group("Owner").unwrap().declarations(),
            &mut class,
            "GLOBAL_TEST_OWNER_ID",
            |_| Ok(true),
            &[],
        )?;
        for (members, types) in [
            (interface.members(), table.types()),
            (class.members(), static_types.as_ref()),
        ] {
            let identity = members
                .iter()
                .find(|member| member.name() == "identity")
                .unwrap();
            let calls = members
                .iter()
                .filter(|member| member.kind() == &LoweredMemberKind::CallSignature)
                .collect::<Vec<_>>();
            let [first, second] = calls.as_slice() else {
                panic!("expected both call signatures")
            };
            assert_eq!(first.type_reference(), identity.type_reference());
            assert_ne!(first.type_reference(), second.type_reference());
            for call in calls {
                let LoweredTypeReference::Local(index) = call.type_reference() else {
                    panic!("expected local function")
                };
                let LoweredTypeData::Function(function) = &types[*index] else {
                    panic!("expected function")
                };
                let [parameter] = function.type_parameters() else {
                    panic!("expected generic parameter")
                };
                assert_eq!(function.name(), None);
                assert_eq!(function.parameters()[0].type_reference(), parameter);
                assert_eq!(function.return_type(), parameter);
            }
        }
        Ok(())
    }

    #[test]
    fn nested_generic_functions_restore_outer_bindings() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(root, "tests/fixtures/global-types/lowering.interfaces.d.ts")?,
            repo_relative: "nested.d.ts".to_owned(),
            bytes: b"interface Owner {
                callback: <T extends string = string>(shadow: <T extends boolean>(value: T) => T, capture: <U extends T = T>(value: U) => T, after: T) => T;
            }".to_vec(),
        };
        let manifest = build_global_manifest(collect(&file).records);
        let table = lower_interfaces(&manifest, &[file], &["Owner"])?;
        let local = |reference: &LoweredTypeReference| {
            let LoweredTypeReference::Local(index) = reference else {
                panic!("expected local type")
            };
            &table.types()[*index]
        };
        let LoweredTypeData::Interface(interface) =
            local(&table.interface_reference("Owner").unwrap())
        else {
            panic!("expected interface")
        };
        let LoweredTypeData::Function(outer) =
            local(interface.member("callback").unwrap().type_reference())
        else {
            panic!("expected outer function")
        };
        let parameter = &outer.type_parameters()[0];
        let LoweredTypeData::GenericParameter {
            constraint,
            default,
            ..
        } = local(parameter)
        else {
            panic!("expected outer parameter")
        };
        assert_eq!(
            constraint,
            &Some(LoweredTypeReference::Predefined("GLOBAL_STRING_ID"))
        );
        assert_eq!(default, constraint);
        let LoweredTypeData::Function(shadow) = local(outer.parameters()[0].type_reference())
        else {
            panic!("expected shadowing callback")
        };
        let inner = &shadow.type_parameters()[0];
        assert_ne!(parameter, inner);
        assert_eq!(shadow.parameters()[0].type_reference(), inner);
        assert_eq!(shadow.return_type(), inner);
        let LoweredTypeData::GenericParameter {
            constraint: Some(constraint),
            default,
            ..
        } = local(inner)
        else {
            panic!("expected inner parameter")
        };
        assert_eq!(local(constraint), &LoweredTypeData::Boolean);
        assert_eq!(default, &None);
        let LoweredTypeData::Function(capture) = local(outer.parameters()[1].type_reference())
        else {
            panic!("expected capturing callback")
        };
        let captured = &capture.type_parameters()[0];
        let LoweredTypeData::GenericParameter {
            constraint,
            default,
            ..
        } = local(captured)
        else {
            panic!("expected capturing parameter")
        };
        assert_eq!(constraint.as_ref(), Some(parameter));
        assert_eq!(default, constraint);
        assert_eq!(capture.parameters()[0].type_reference(), captured);
        assert_eq!(capture.return_type(), parameter);
        assert_eq!(outer.parameters()[2].type_reference(), parameter);
        assert_eq!(outer.return_type(), parameter);
        Ok(())
    }

    #[test]
    fn method_generics_bind_constraints_defaults_and_callbacks() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for parameter in ["T", "Owner", "WeakKey"] {
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(root, "tests/fixtures/global-types/lowering.interfaces.d.ts")?,
                repo_relative: "methods.d.ts".to_owned(),
                bytes: format!("
                    interface Owner<T extends boolean> {{
                        outer: T;
                        transform<{parameter} extends string = string, U extends {parameter} = {parameter}>(value: U, callback: (value: {parameter}) => U): U;
                        capture<U extends T = T>(value: U): U;
                        after(value: T): T;
                    }}
                ").into_bytes(),
            };
            let manifest = build_global_manifest(collect(&file).records);
            let mut class = LoweredClass {
                name: Text::from("Owner"),
                type_parameters: Box::default(),
                members: Box::default(),
            };
            let types =
                lower_class_members(&manifest, &[file], &mut class, "GLOBAL_TEST_OWNER_ID", None)?;
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                &types[*index]
            };
            let LoweredTypeData::Function(method) =
                local(class.member("transform").unwrap().type_reference())
            else {
                panic!("expected method")
            };
            let [first, second] = method.type_parameters() else {
                panic!("expected method parameters")
            };
            let LoweredTypeData::GenericParameter {
                name,
                constraint,
                default,
                ..
            } = local(first)
            else {
                panic!("expected generic")
            };
            assert_eq!(name.text(), parameter);
            assert_eq!(
                constraint,
                &Some(LoweredTypeReference::Predefined("GLOBAL_STRING_ID"))
            );
            assert_eq!(default, constraint);
            let LoweredTypeData::GenericParameter {
                constraint,
                default,
                ..
            } = local(second)
            else {
                panic!("expected dependent generic")
            };
            assert_eq!(constraint.as_ref(), Some(first));
            assert_eq!(default, constraint);
            assert_eq!(method.parameters()[0].type_reference(), second);
            assert_eq!(method.return_type(), second);
            let LoweredTypeData::Function(callback) =
                local(method.parameters()[1].type_reference())
            else {
                panic!("expected callback")
            };
            assert_eq!(callback.parameters()[0].type_reference(), first);
            assert_eq!(callback.return_type(), second);
            assert!(callback.type_parameters().is_empty());
            let LoweredTypeData::Function(capture) =
                local(class.member("capture").unwrap().type_reference())
            else {
                panic!("expected capturing method")
            };
            let LoweredTypeData::GenericParameter {
                constraint,
                default,
                ..
            } = local(&capture.type_parameters()[0])
            else {
                panic!("expected capturing parameter")
            };
            assert_eq!(
                constraint.as_ref(),
                Some(class.member("outer").unwrap().type_reference())
            );
            assert_eq!(default, constraint);
            let LoweredTypeData::Function(after) =
                local(class.member("after").unwrap().type_reference())
            else {
                panic!("expected following method")
            };
            assert_eq!(
                after.return_type(),
                class.member("outer").unwrap().type_reference()
            );
            assert_eq!(after.parameters()[0].type_reference(), after.return_type());
            assert!(after.type_parameters().is_empty());
        }
        Ok(())
    }

    #[test]
    fn method_generics_translate_in_interfaces_and_statics() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let file = DiscoveredFile {
            path: CanonicalPath::from_within(root, "tests/fixtures/global-types/lowering.interfaces.d.ts")?,
            repo_relative: "methods.d.ts".to_owned(),
            bytes: b"interface Owner { identity<Value>(value: Value): Value; choose<Value extends boolean = boolean>(value?: Value): Value; }".to_vec(),
        };
        let files = [file];
        let manifest = build_global_manifest(collect(&files[0]).records);
        let table = lower_interfaces(&manifest, &files, &["Owner"])?;
        let LoweredTypeReference::Local(index) = table.interface_reference("Owner").unwrap() else {
            panic!("expected local interface")
        };
        let LoweredTypeData::Interface(interface) = &table.types()[index] else {
            panic!("expected interface")
        };
        let mut class = LoweredClass {
            name: Text::from("Owner"),
            type_parameters: Box::default(),
            members: Box::default(),
        };
        let static_types = lower_constructor_members(
            &manifest,
            &files,
            manifest.global_group("Owner").unwrap().declarations(),
            &mut class,
            "GLOBAL_TEST_OWNER_ID",
            |_| Ok(true),
            &[],
        )?;
        for (members, types) in [
            (interface.members(), table.types()),
            (class.members(), static_types.as_ref()),
        ] {
            let mut generics = Vec::new();
            for member in members {
                let LoweredTypeReference::Local(index) = member.type_reference() else {
                    panic!("expected local method")
                };
                let LoweredTypeData::Function(function) = &types[*index] else {
                    panic!("expected function")
                };
                let [generic] = function.type_parameters() else {
                    panic!("expected method generic")
                };
                assert_eq!(function.parameters()[0].type_reference(), generic);
                assert_eq!(function.return_type(), generic);
                let LoweredTypeReference::Local(parameter_index) = generic else {
                    panic!("expected local parameter")
                };
                assert!(parameter_index < index);
                generics.push(generic);
            }
            assert_ne!(
                generics[0], generics[1],
                "different constraints must retain distinct parameters"
            );
        }
        assert!(
            class
                .members()
                .iter()
                .all(|member| member.kind() == &LoweredMemberKind::NamedStatic)
        );
        Ok(())
    }

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
                lower_class_members(&manifest, &[file], &mut class, "GLOBAL_TEST_OWNER_ID", None)?;
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
    fn named_constructor_members_preserve_overloads() -> Result<()> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for (owner, method) in [("First", "create"), ("Second", "convert")] {
            let file = DiscoveredFile {
                path: CanonicalPath::from_within(
                    root,
                    "tests/fixtures/global-types/lowering.interfaces.d.ts",
                )?,
                repo_relative: "statics.d.ts".to_owned(),
                bytes: format!(
                    r#"
                    interface Factory {{
                        {method}<T>(value: T): T;
                        unrelated: Missing;
                    }}
                    interface Factory {{
                        {method}<T, U>(value: T, map: (value: T) => U, context?: any): U;
                    }}
                "#
                )
                .into_bytes(),
            };
            let manifest = build_global_manifest(collect(&file).records);
            let mut class = LoweredClass {
                name: Text::from(owner),
                type_parameters: Box::default(),
                members: Box::default(),
            };
            let types = lower_constructor_members(
                &manifest,
                &[file],
                manifest.global_group("Factory").unwrap().declarations(),
                &mut class,
                "GLOBAL_TEST_OWNER_ID",
                |member| select_named_members(member, &["create", "convert"]),
                &[],
            )?;
            let [member] = class.members() else {
                panic!("expected one selected static")
            };
            assert_eq!(member.name(), method);
            assert_eq!(member.kind(), &LoweredMemberKind::NamedStatic);
            let local = |reference: &LoweredTypeReference| {
                let LoweredTypeReference::Local(index) = reference else {
                    panic!("expected local type")
                };
                &types[*index]
            };
            let LoweredTypeData::Object(signatures) = local(member.type_reference()) else {
                panic!("expected overloads")
            };
            assert_eq!(signatures.len(), 2);
            for (index, signature) in signatures.iter().enumerate() {
                assert_eq!(signature.kind(), &LoweredMemberKind::CallSignature);
                let LoweredTypeData::Function(function) = local(signature.type_reference()) else {
                    panic!("expected function")
                };
                assert_eq!(function.type_parameters().len(), index + 1);
                assert_eq!(
                    function.parameters()[0].type_reference(),
                    &function.type_parameters()[0]
                );
                assert_eq!(
                    Some(function.return_type()),
                    function.type_parameters().last()
                );
                if index == 1 {
                    let LoweredTypeData::Function(callback) =
                        local(function.parameters()[1].type_reference())
                    else {
                        panic!("expected callback")
                    };
                    assert_eq!(
                        callback.parameters()[0].type_reference(),
                        &function.type_parameters()[0]
                    );
                    assert_eq!(callback.return_type(), function.return_type());
                    assert!(function.parameters()[2].is_optional());
                }
            }
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
                None,
            )?;
            if let Some(member) = class.members().first() {
                assert_eq!(member.type_reference(), &LoweredTypeReference::Local(0));
            }
            globals.push(LoweredGlobal {
                name: class.name.clone(),
                id_constant: id_constant.into(),
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
