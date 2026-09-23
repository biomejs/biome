//! Lowers every global declaration group from its source declarations.
//!
//! A reference to another declaration points at that declaration's global identity, so
//! each global's local table only holds the anonymous types written inside it. Syntax
//! that the runtime model can't represent lowers to `unknown` and is recorded as a
//! [`LoweringGap`] instead of failing the run.

use std::collections::BTreeMap;

use biome_js_syntax::{
    AnyJsObjectMemberName, AnyTsTupleTypeElement, JsComputedMemberName,
    TsDeclareFunctionDeclaration, TsReferenceType, TsTypeAliasDeclaration, TsTypeofType,
};

use super::ids::{GlobalIds, GlobalSlot, GlobalSlotKind, join_path, scope_for};
use super::*;

/// A part of a declaration lowered to `unknown`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweringGap {
    /// Qualified name of the global containing the unsupported part.
    pub owner: String,
    /// What could not be lowered.
    pub detail: String,
}

/// Lowers global identities assigned by [`GlobalIds`].
pub(super) struct GenericLowerer<'a> {
    manifest: &'a GlobalManifest,
    ids: &'a GlobalIds,
    pub(super) sources: ParsedSourceCache<'a>,
    gaps: Vec<LoweringGap>,
}

impl<'a> GenericLowerer<'a> {
    pub(super) fn new(
        manifest: &'a GlobalManifest,
        ids: &'a GlobalIds,
        sources: ParsedSourceCache<'a>,
    ) -> Self {
        Self {
            manifest,
            ids,
            sources,
            gaps: Vec::new(),
        }
    }

    pub(super) fn into_gaps(self) -> Vec<LoweringGap> {
        self.gaps
    }

    /// Lowers the declarations behind one global identity.
    pub(super) fn lower_slot(&mut self, slot: &'a GlobalSlot) -> Result<LoweredGlobal> {
        let mut builder = GlobalBuilder::new(self, slot);
        let data = match slot.kind() {
            GlobalSlotKind::Type => builder.lower_type_group()?,
            GlobalSlotKind::Value => builder.lower_value_group()?,
            GlobalSlotKind::Class { constructor } => {
                LoweredTypeData::Class(builder.lower_class(constructor, true)?)
            }
            GlobalSlotKind::Namespace => builder.lower_namespace(),
            GlobalSlotKind::UniqueSymbol => LoweredTypeData::Symbol,
        };
        Ok(builder.finish(data))
    }

    /// Lowers a class's constructor signatures and static members without its instance
    /// members, for globals whose instance side is a hand-written projection.
    pub(super) fn lower_class_statics(
        &mut self,
        slot: &'a GlobalSlot,
    ) -> Result<(LoweredClass, Box<[LoweredTypeData]>)> {
        let GlobalSlotKind::Class { constructor } = slot.kind() else {
            return Ok((
                LoweredClass {
                    name: Text::from(slot.name()),
                    type_parameters: Box::default(),
                    extends: None,
                    implements: Box::default(),
                    members: Box::default(),
                },
                Box::default(),
            ));
        };
        let mut builder = GlobalBuilder::new(self, slot);
        let class = builder.lower_class(constructor, false)?;
        Ok((class, builder.types.into_boxed_slice()))
    }
}

/// State for lowering one global and its local type table.
struct GlobalBuilder<'l, 'a> {
    lowerer: &'l mut GenericLowerer<'a>,
    slot: &'a GlobalSlot,
    types: Vec<LoweredTypeData>,
    /// Type parameter scopes, innermost last.
    parameters: Vec<BTreeMap<Text, LoweredTypeReference>>,
    /// Qualified name of the interface whose members are being lowered.
    interface: Option<String>,
}

impl<'l, 'a> GlobalBuilder<'l, 'a> {
    fn new(lowerer: &'l mut GenericLowerer<'a>, slot: &'a GlobalSlot) -> Self {
        Self {
            lowerer,
            slot,
            types: Vec::new(),
            parameters: Vec::new(),
            interface: None,
        }
    }

    fn finish(self, data: LoweredTypeData) -> LoweredGlobal {
        LoweredGlobal {
            name: Text::from(self.slot.name()),
            id_constant: Text::from(self.slot.id_constant()),
            data,
            local_types: self.types.into_boxed_slice(),
            roles: GlobalRoles {
                type_name: self.slot.has_type_role(),
                value_name: self.slot.has_value_role(),
            },
        }
    }

    fn group(&self, name: &str) -> Option<&'a GlobalDeclarationGroup> {
        self.lowerer
            .manifest
            .group(&scope_for(self.slot.namespace()), name)
    }

    fn own_group(&self) -> Result<&'a GlobalDeclarationGroup> {
        self.group(self.slot.declared_name().text())
            .with_context(|| format!("missing declaration group for {}", self.slot.name()))
    }

    /// Records a gap and returns the `unknown` reference that replaces the unsupported part.
    fn unknown(&mut self, detail: impl Into<String>) -> LoweredTypeReference {
        self.gap(detail);
        LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID")
    }

    fn gap(&mut self, detail: impl Into<String>) {
        self.lowerer.gaps.push(LoweringGap {
            owner: self.slot.name(),
            detail: detail.into(),
        });
    }

    /// Reuses structurally equal entries without changing existing local indices.
    fn register(&mut self, data: LoweredTypeData) -> LoweredTypeReference {
        if let Some(index) = self.types.iter().position(|ty| ty == &data) {
            return LoweredTypeReference::Local(index);
        }
        self.types.push(data);
        LoweredTypeReference::Local(self.types.len() - 1)
    }

    fn global_reference(&self, index: usize) -> LoweredTypeReference {
        LoweredTypeReference::Global(Text::from(
            self.lowerer.ids.slot(index).reference_constant(),
        ))
    }

    // Declaration groups

    fn lower_type_group(&mut self) -> Result<LoweredTypeData> {
        let group = self.own_group()?;
        let interfaces = records_of_kind(group, |kind| kind == &DeclarationKind::Interface);
        let aliases = records_of_kind(group, |kind| kind == &DeclarationKind::TypeAlias);
        if !interfaces.is_empty() {
            if !aliases.is_empty() {
                self.gap("type alias merged with an interface is ignored");
            }
            let parts = self.collect_interface(&interfaces, MemberList::default(), false)?;
            let extends = parts
                .bases
                .iter()
                .map(|base| self.lower_base(base))
                .collect();
            let members = self.finish_members(parts.list);
            self.parameters.pop();
            return Ok(LoweredTypeData::Interface(LoweredInterface {
                name: Text::from(self.slot.name()),
                type_parameters: parts.type_parameters,
                extends,
                members,
            }));
        }
        let [alias, rest @ ..] = aliases.as_slice() else {
            bail!("{} has no type declaration", self.slot.name());
        };
        if !rest.is_empty() {
            self.gap("only the first of several type aliases is lowered");
        }
        let declaration = self
            .lowerer
            .sources
            .find_node(alias)?
            .and_then(TsTypeAliasDeclaration::cast)
            .context("missing type alias declaration")?;
        let type_parameters = self.declare_type_parameters(declaration.type_parameters());
        let ty = self.lower_type(&declaration.ty()?);
        self.parameters.pop();
        Ok(if type_parameters.is_empty() {
            LoweredTypeData::Reference(ty)
        } else {
            LoweredTypeData::InstanceOf {
                ty,
                type_parameters,
            }
        })
    }

    fn lower_value_group(&mut self) -> Result<LoweredTypeData> {
        let group = self.own_group()?;
        let functions = records_of_kind(group, |kind| kind == &DeclarationKind::DeclareFunction);
        let variables = records_of_kind(group, |kind| {
            matches!(kind, DeclarationKind::VariableDeclarator { .. })
        });
        if !functions.is_empty() {
            if !variables.is_empty() {
                self.gap("variables merged with functions are ignored");
            }
            return self.lower_functions(&functions);
        }
        let Some(variable) = variables.first() else {
            return Ok(LoweredTypeData::Reference(
                self.unknown("unsupported value declaration"),
            ));
        };
        if variables.len() > 1 {
            self.gap("only the first of several variable declarations is lowered");
        }
        let declarator = self
            .lowerer
            .sources
            .find_variable_declarator(variable)?
            .context("missing variable declaration")?;
        let reference = match declarator.variable_annotation() {
            Some(AnyTsVariableAnnotation::TsTypeAnnotation(annotation)) => {
                self.lower_type(&annotation.ty()?)
            }
            _ => self.unknown("variable without a type annotation"),
        };
        Ok(LoweredTypeData::Reference(reference))
    }

    fn lower_functions(&mut self, records: &[&DeclarationRecord]) -> Result<LoweredTypeData> {
        let mut functions = Vec::new();
        for record in records {
            let declaration = self
                .lowerer
                .sources
                .find_node(record)?
                .and_then(TsDeclareFunctionDeclaration::cast)
                .context("missing function declaration")?;
            let return_type = match declaration.return_type_annotation() {
                Some(annotation) => Some(annotation.ty()?),
                None => None,
            };
            let mut function = self.lower_signature(
                Some(self.slot.declared_name().clone()),
                declaration.type_parameters(),
                declaration.parameters()?,
                return_type,
            )?;
            function.is_async = declaration.async_token().is_some();
            functions.push(function);
        }
        Ok(match <[LoweredFunction; 1]>::try_from(functions) {
            Ok([function]) => LoweredTypeData::Function(function),
            Err(functions) => LoweredTypeData::Object(
                functions
                    .into_iter()
                    .map(|function| LoweredTypeMember {
                        name: Text::default(),
                        kind: LoweredMemberKind::CallSignature,
                        type_reference: self.register(LoweredTypeData::Function(function)),
                    })
                    .collect(),
            ),
        })
    }

    /// Lowers `interface X` as instance members and `interface C` behind
    /// `declare var X: C` as static members and signatures.
    fn lower_class(&mut self, constructor: &Text, include_instance: bool) -> Result<LoweredClass> {
        let group = self.own_group()?;
        let interfaces = records_of_kind(group, |kind| kind == &DeclarationKind::Interface);
        let mut class = LoweredClass {
            name: Text::from(self.slot.name()),
            type_parameters: Box::default(),
            extends: None,
            implements: Box::default(),
            members: Box::default(),
        };
        let mut members = MemberList::default();
        if include_instance {
            let parts = self.collect_interface(&interfaces, MemberList::default(), false)?;
            let mut implements = Vec::new();
            for base in &parts.bases {
                let base_slot = self.resolve_type_name(&type_name_path(&base.name()?)?);
                let is_class = base_slot.is_some_and(|index| {
                    matches!(
                        self.lowerer.ids.slot(index).kind(),
                        GlobalSlotKind::Class { .. }
                    )
                });
                if is_class && class.extends.is_none() && base.type_arguments().is_none() {
                    class.extends = base_slot.map(|index| self.global_reference(index));
                } else {
                    implements.push(self.lower_base(base));
                }
            }
            class.type_parameters = parts.type_parameters;
            class.implements = implements.into_boxed_slice();
            members = parts.list;
            // Class type parameters are not in scope for static members.
            self.parameters.pop();
        }

        let constructor_interfaces = self
            .group(constructor.text())
            .map(|group| records_of_kind(group, |kind| kind == &DeclarationKind::Interface))
            .unwrap_or_default();
        if constructor_interfaces.is_empty() {
            self.gap(format!("missing constructor interface {constructor}"));
        }
        let parts = self.collect_interface(&constructor_interfaces, members, true)?;
        if !parts.bases.is_empty() {
            self.gap(format!(
                "static members inherited by {constructor} are not lowered"
            ));
        }
        class.members = self.finish_members(parts.list);
        self.parameters.pop();
        Ok(class)
    }

    /// Lowers the values declared in a namespace as members of an object.
    fn lower_namespace(&mut self) -> LoweredTypeData {
        let path = self
            .slot
            .namespace()
            .iter()
            .cloned()
            .chain([self.slot.declared_name().clone()])
            .collect::<Vec<_>>();
        let scope = scope_for(&path);
        let mut members = Vec::new();
        for group in self.lowerer.manifest.groups_in_scope(&scope) {
            let mut member_path = path.clone();
            member_path.push(group.name().clone());
            if let Some(index) = self.lowerer.ids.value_slot(&join_path(&member_path)) {
                members.push(LoweredTypeMember {
                    name: group.name().clone(),
                    kind: LoweredMemberKind::Named { optional: false },
                    type_reference: self.global_reference(index),
                });
            }
        }
        for (index, slot) in self.lowerer.ids.slots().iter().enumerate() {
            if slot.kind() == &GlobalSlotKind::Namespace
                && slot.namespace() == path.as_slice()
                && !members
                    .iter()
                    .any(|member| &member.name == slot.declared_name())
            {
                members.push(LoweredTypeMember {
                    name: slot.declared_name().clone(),
                    kind: LoweredMemberKind::Named { optional: false },
                    type_reference: self.global_reference(index),
                });
            }
        }
        LoweredTypeData::Object(members.into_boxed_slice())
    }

    // Interfaces and members

    /// Collects merged interface declarations into `list`, leaving their type parameter
    /// scope pushed for the caller to pop.
    fn collect_interface(
        &mut self,
        records: &[&DeclarationRecord],
        mut list: MemberList,
        statics: bool,
    ) -> Result<InterfaceParts> {
        let mut type_parameters: Option<Box<[LoweredTypeReference]>> = None;
        let mut bases = Vec::new();
        let outer_interface = self.interface.take();
        for record in records {
            let declaration = self
                .lowerer
                .sources
                .find_interface_declaration(record)?
                .context("missing interface declaration")?;
            match &type_parameters {
                None => {
                    type_parameters =
                        Some(self.declare_type_parameters(declaration.type_parameters()));
                }
                Some(references) => {
                    self.rebind_type_parameters(declaration.type_parameters(), references)?;
                }
            }
            let mut path = self.slot.namespace().to_vec();
            path.push(Text::from(record.declared_name.clone()));
            self.interface = Some(join_path(&path));
            if let Some(clause) = declaration.extends_clause() {
                for base in clause.types() {
                    bases.push(base?);
                }
            }
            for member in declaration.members() {
                self.lower_member(member, &mut list, statics);
            }
        }
        self.interface = outer_interface;
        if type_parameters.is_none() {
            self.parameters.push(BTreeMap::new());
        }
        Ok(InterfaceParts {
            type_parameters: type_parameters.unwrap_or_default(),
            bases,
            list,
        })
    }

    fn lower_member(&mut self, member: AnyTsTypeMember, list: &mut MemberList, statics: bool) {
        if let Err(error) = self.try_lower_member(member, list, statics) {
            self.gap(format!("member skipped: {error:#}"));
        }
    }

    fn try_lower_member(
        &mut self,
        member: AnyTsTypeMember,
        list: &mut MemberList,
        statics: bool,
    ) -> Result<()> {
        match member {
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                let optional = property.optional_token().is_some();
                let Some(key) = self.lower_member_key(property.name()?, optional, statics)? else {
                    return Ok(());
                };
                let ty = if let Some(annotation) = property.type_annotation() {
                    let ty = annotation.ty()?;
                    self.lower_property_type(&key.name, &ty)
                } else {
                    self.register(LoweredTypeData::AnyKeyword)
                };
                let ty = self.optional_computed_value(&key, optional, ty);
                list.push_property(key, ty);
            }
            AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => {
                let Some(key) = self.lower_member_key(getter.name()?, false, statics)? else {
                    return Ok(());
                };
                let ty = match getter.type_annotation() {
                    Some(annotation) => self.lower_type(&annotation.ty()?),
                    None => self.unknown("getter without a return type"),
                };
                list.push_property(key, ty);
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => {
                let Some(key) = self.lower_member_key(setter.name()?, false, statics)? else {
                    return Ok(());
                };
                if list.contains(&key) {
                    return Ok(());
                }
                let ty = match setter.parameter()? {
                    AnyJsFormalParameter::JsFormalParameter(parameter) => {
                        match parameter.type_annotation() {
                            Some(annotation) => self.lower_type(&annotation.ty()?),
                            None => self.unknown("setter parameter without a type"),
                        }
                    }
                    _ => self.unknown("unsupported setter parameter"),
                };
                list.push_property(key, ty);
            }
            AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                let optional = method.optional_token().is_some();
                let Some(key) = self.lower_member_key(method.name()?, optional, statics)? else {
                    return Ok(());
                };
                let return_type = match method.return_type_annotation() {
                    Some(annotation) => Some(annotation.ty()?),
                    None => None,
                };
                let function = self.lower_signature(
                    Some(key.name.clone()),
                    method.type_parameters(),
                    method.parameters()?,
                    return_type,
                )?;
                let ty = self.register(LoweredTypeData::Function(function));
                list.push_method(key, ty);
            }
            AnyTsTypeMember::TsCallSignatureTypeMember(signature) => {
                let return_type = match signature.return_type_annotation() {
                    Some(annotation) => Some(annotation.ty()?),
                    None => None,
                };
                let function = self.lower_signature(
                    None,
                    signature.type_parameters(),
                    signature.parameters()?,
                    return_type,
                )?;
                let ty = self.register(LoweredTypeData::Function(function));
                list.push_signature(LoweredMemberKind::CallSignature, ty);
            }
            AnyTsTypeMember::TsConstructSignatureTypeMember(signature) => {
                let return_type = match signature.type_annotation() {
                    Some(annotation) => Some(AnyTsReturnType::AnyTsType(annotation.ty()?)),
                    None => None,
                };
                let function = self.lower_signature(
                    None,
                    signature.type_parameters(),
                    signature.parameters()?,
                    return_type,
                )?;
                let ty = self.register(LoweredTypeData::Constructor(LoweredConstructor {
                    type_parameters: function.type_parameters,
                    parameters: function.parameters,
                    return_type: Some(function.return_type),
                }));
                list.push_signature(LoweredMemberKind::Constructor, ty);
            }
            AnyTsTypeMember::TsIndexSignatureTypeMember(signature) => {
                if statics {
                    self.gap("static index signatures are not supported");
                    return Ok(());
                }
                let key = self.lower_type(&signature.parameter()?.type_annotation()?.ty()?);
                let ty = self.lower_type(&signature.type_annotation()?.ty()?);
                list.push_signature(LoweredMemberKind::IndexSignature { key_reference: key }, ty);
            }
            AnyTsTypeMember::JsBogusMember(_) | AnyTsTypeMember::JsMetavariable(_) => {
                bail!("unsupported member syntax")
            }
        }
        Ok(())
    }

    /// Lowers `unique symbol` properties to their global identities.
    fn lower_property_type(&mut self, name: &Text, ty: &AnyTsType) -> LoweredTypeReference {
        if is_unique_symbol_type(ty) {
            let slot = self
                .interface
                .as_deref()
                .and_then(|interface| self.lowerer.ids.unique_symbol(interface, name.text()));
            return match slot {
                Some(index) => self.global_reference(index),
                None => self.register(LoweredTypeData::Symbol),
            };
        }
        self.lower_type(ty)
    }

    fn optional_computed_value(
        &mut self,
        key: &MemberKey,
        optional: bool,
        reference: LoweredTypeReference,
    ) -> LoweredTypeReference {
        if optional && matches!(key.kind, LoweredMemberKind::ComputedValue { .. }) {
            let undefined = self.register(LoweredTypeData::Undefined);
            self.register(LoweredTypeData::Union(Box::new([reference, undefined])))
        } else {
            reference
        }
    }

    /// Returns `None` when a computed key can't be represented.
    fn lower_member_key(
        &mut self,
        name: AnyJsObjectMemberName,
        optional: bool,
        statics: bool,
    ) -> Result<Option<MemberKey>> {
        let key = match name {
            AnyJsObjectMemberName::JsComputedMemberName(computed) => {
                let Some((name, key_reference)) = self.lower_computed_key(&computed)? else {
                    return Ok(None);
                };
                let kind = if statics {
                    LoweredMemberKind::ComputedStatic { key_reference }
                } else {
                    LoweredMemberKind::ComputedValue { key_reference }
                };
                MemberKey { name, kind }
            }
            name => {
                let kind = if statics {
                    LoweredMemberKind::NamedStatic
                } else {
                    LoweredMemberKind::Named { optional }
                };
                MemberKey {
                    name: lower_object_member_name(name)?,
                    kind,
                }
            }
        };
        Ok(Some(key))
    }

    fn lower_computed_key(
        &mut self,
        computed: &JsComputedMemberName,
    ) -> Result<Option<(Text, LoweredTypeReference)>> {
        let expression = computed.expression()?.omit_parentheses();
        let name = Text::from(format!("[{}]", expression.syntax().text_trimmed()));
        let key = match &expression {
            AnyJsExpression::AnyJsLiteralExpression(
                biome_js_syntax::AnyJsLiteralExpression::JsStringLiteralExpression(string),
            ) => self.register(LoweredTypeData::StringLiteral(Text::from(
                string.inner_string_text()?,
            ))),
            AnyJsExpression::AnyJsLiteralExpression(
                biome_js_syntax::AnyJsLiteralExpression::JsNumberLiteralExpression(number),
            ) => self.register(LoweredTypeData::NumberLiteral(Text::from(
                number.value_token()?.token_text_trimmed(),
            ))),
            expression => {
                let path = expression_path(expression);
                let slot = path
                    .as_deref()
                    .and_then(|path| self.resolve_value_name(path))
                    .filter(|index| {
                        self.lowerer.ids.slot(*index).kind() == &GlobalSlotKind::UniqueSymbol
                    });
                match slot {
                    Some(index) => self.global_reference(index),
                    None => {
                        self.gap(format!("unsupported computed member key {name}"));
                        return Ok(None);
                    }
                }
            }
        };
        Ok(Some((name, key)))
    }

    /// Converts collected members, turning repeated methods into overloaded callables.
    fn finish_members(&mut self, list: MemberList) -> Box<[LoweredTypeMember]> {
        list.members
            .into_iter()
            .map(|(member, signatures)| {
                let type_reference = match <[LoweredTypeReference; 1]>::try_from(signatures) {
                    Ok([signature]) => signature,
                    Err(signatures) if signatures.is_empty() => member.type_reference,
                    Err(signatures) => self.register(LoweredTypeData::Object(
                        signatures
                            .into_iter()
                            .map(|type_reference| LoweredTypeMember {
                                name: Text::default(),
                                kind: LoweredMemberKind::CallSignature,
                                type_reference,
                            })
                            .collect(),
                    )),
                };
                LoweredTypeMember {
                    type_reference,
                    ..member
                }
            })
            .collect()
    }

    // Type parameters

    /// Pushes a scope with the declared type parameters and returns their references.
    fn declare_type_parameters(
        &mut self,
        parameters: Option<TsTypeParameters>,
    ) -> Box<[LoweredTypeReference]> {
        self.parameters.push(BTreeMap::new());
        let Some(parameters) = parameters else {
            return Box::default();
        };
        let unknown = LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID");
        let items = parameters.items().into_iter().flatten().collect::<Vec<_>>();
        // Constraints may only refer to earlier parameters, because local types must be
        // registered after their dependencies.
        let names = items
            .iter()
            .filter_map(|parameter| parameter_name(parameter).ok())
            .collect::<Vec<_>>();
        for name in &names {
            self.bind_parameter(name.clone(), unknown.clone());
        }
        let mut references = Vec::new();
        for parameter in items {
            let Ok(name) = parameter_name(&parameter) else {
                self.gap("malformed type parameter");
                continue;
            };
            let constraint = parameter
                .constraint()
                .and_then(|constraint| constraint.ty().ok())
                .map(|ty| self.lower_type(&ty));
            let default = parameter
                .default()
                .and_then(|default| default.ty().ok())
                .map(|ty| self.lower_type(&ty));
            let reference = self.register(LoweredTypeData::GenericParameter {
                is_const: parameter
                    .modifiers()
                    .into_iter()
                    .any(|modifier| modifier.as_ts_const_modifier().is_some()),
                name: name.clone(),
                constraint,
                default,
            });
            self.bind_parameter(name, reference.clone());
            references.push(reference);
        }
        references.into_boxed_slice()
    }

    /// Binds the parameters of a merged declaration to the first declaration's references.
    fn rebind_type_parameters(
        &mut self,
        parameters: Option<TsTypeParameters>,
        references: &[LoweredTypeReference],
    ) -> Result<()> {
        let names = parameters
            .map(|parameters| {
                parameters
                    .items()
                    .into_iter()
                    .map(|parameter| parameter_name(&parameter?))
                    .collect::<Result<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();
        if names.len() != references.len() {
            self.gap("merged declarations have different type parameter counts");
        }
        let scope = self
            .parameters
            .last_mut()
            .context("missing parameter scope")?;
        scope.clear();
        for (index, name) in names.into_iter().enumerate() {
            let reference = references
                .get(index)
                .cloned()
                .unwrap_or(LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID"));
            scope.insert(name, reference);
        }
        Ok(())
    }

    fn bind_parameter(&mut self, name: Text, reference: LoweredTypeReference) {
        if let Some(scope) = self.parameters.last_mut() {
            scope.insert(name, reference);
        }
    }

    fn type_parameter(&self, name: &str) -> Option<LoweredTypeReference> {
        self.parameters
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).cloned())
    }

    // Signatures

    fn lower_signature(
        &mut self,
        name: Option<Text>,
        type_parameters: Option<TsTypeParameters>,
        parameters: JsParameters,
        return_type: Option<AnyTsReturnType>,
    ) -> Result<LoweredFunction> {
        let type_parameters = self.declare_type_parameters(type_parameters);
        let result = self.lower_signature_body(name, parameters, return_type);
        self.parameters.pop();
        let mut function = result?;
        function.type_parameters = type_parameters;
        Ok(function)
    }

    fn lower_signature_body(
        &mut self,
        name: Option<Text>,
        parameters: JsParameters,
        return_type: Option<AnyTsReturnType>,
    ) -> Result<LoweredFunction> {
        let mut lowered = Vec::new();
        for parameter in parameters.items() {
            match parameter? {
                AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                    parameter,
                )) => {
                    let binding = match lower_binding_name(parameter.binding()?) {
                        Ok(name) => LoweredFunctionParameterBinding::Named(name),
                        Err(_) => LoweredFunctionParameterBinding::Pattern,
                    };
                    let type_reference = match parameter.type_annotation() {
                        Some(annotation) => self.lower_type(&annotation.ty()?),
                        None => self.unknown("parameter without a type annotation"),
                    };
                    lowered.push(LoweredFunctionParameter {
                        binding,
                        type_reference,
                        is_optional: parameter.question_mark_token().is_some()
                            || parameter.initializer().is_some(),
                        is_rest: false,
                    });
                }
                AnyJsParameter::JsRestParameter(parameter) => {
                    let binding = match lower_binding_name(parameter.binding()?) {
                        Ok(name) => LoweredFunctionParameterBinding::Named(name),
                        Err(_) => LoweredFunctionParameterBinding::Pattern,
                    };
                    let type_reference = match parameter.type_annotation() {
                        Some(annotation) => self.lower_type(&annotation.ty()?),
                        None => self.unknown("rest parameter without a type annotation"),
                    };
                    lowered.push(LoweredFunctionParameter {
                        binding,
                        type_reference,
                        is_optional: false,
                        is_rest: true,
                    });
                }
                // The runtime model has no `this` parameter; callers see the others.
                AnyJsParameter::TsThisParameter(_) => {}
                AnyJsParameter::AnyJsFormalParameter(_) => {
                    self.gap("unsupported formal parameter");
                }
            }
        }
        let return_type = match return_type {
            Some(AnyTsReturnType::AnyTsType(ty)) => self.lower_type(&ty),
            Some(AnyTsReturnType::TsPredicateReturnType(_)) => {
                self.register(LoweredTypeData::Boolean)
            }
            Some(AnyTsReturnType::TsAssertsReturnType(_)) => {
                LoweredTypeReference::Predefined("GLOBAL_VOID_ID")
            }
            None => self.unknown("signature without a return type"),
        };
        Ok(LoweredFunction {
            is_async: false,
            type_parameters: Box::default(),
            name,
            parameters: lowered.into_boxed_slice(),
            return_type,
        })
    }

    // Types

    fn lower_type(&mut self, ty: &AnyTsType) -> LoweredTypeReference {
        match self.try_lower_type(ty) {
            Ok(reference) => reference,
            Err(error) => self.unknown(format!("{error:#}")),
        }
    }

    fn try_lower_type(&mut self, ty: &AnyTsType) -> Result<LoweredTypeReference> {
        if let Some(reference) = lower_primitive_reference(ty) {
            return Ok(reference);
        }
        if let Some(data) = lower_scalar_type(ty)? {
            return Ok(self.register(data));
        }
        Ok(match ty {
            AnyTsType::TsThisType(_) => self.register(LoweredTypeData::ThisKeyword),
            AnyTsType::TsArrayType(array) => {
                let element = self.lower_type(&array.element_type()?);
                let array = self.array_reference();
                self.register(LoweredTypeData::InstanceOf {
                    ty: array,
                    type_parameters: Box::new([element]),
                })
            }
            AnyTsType::TsReferenceType(reference) => self.lower_reference_type(reference, true)?,
            AnyTsType::TsTypeofType(typeof_type) => self.lower_typeof_type(typeof_type)?,
            AnyTsType::TsConditionalType(conditional) => {
                // Match local inference's conservative union of both conditional branches.
                // Names bound by `infer` in the extends clause are unknown in both branches.
                let mut scope = BTreeMap::new();
                for node in conditional.extends_type()?.syntax().descendants() {
                    if let Some(infer) = biome_js_syntax::TsInferType::cast(node) {
                        scope.insert(
                            Text::from(infer.name()?.ident_token()?.token_text_trimmed()),
                            LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID"),
                        );
                    }
                }
                self.parameters.push(scope);
                let yes = self.lower_type(&conditional.true_type()?);
                let no = self.lower_type(&conditional.false_type()?);
                self.parameters.pop();
                self.register(LoweredTypeData::Union(Box::new([yes, no])))
            }
            AnyTsType::TsIndexedAccessType(access) => {
                let object = self.lower_type(&access.object_type()?);
                let index = self.lower_type(&access.index_type()?);
                self.register(LoweredTypeData::IndexedAccess { object, index })
            }
            AnyTsType::TsTypeOperatorType(operator) => {
                let kind = operator.operator_token()?.kind();
                let operand = operator.ty()?;
                if kind == T![keyof] {
                    let ty = self.lower_type(&operand);
                    self.register(LoweredTypeData::Keyof(ty))
                } else if kind == T![unique] {
                    self.register(LoweredTypeData::Symbol)
                } else if matches!(
                    operand,
                    AnyTsType::TsArrayType(_) | AnyTsType::TsTupleType(_)
                ) {
                    let ty = self.lower_type(&operand);
                    self.register(LoweredTypeData::Readonly(ty))
                } else {
                    self.lower_type(&operand)
                }
            }
            AnyTsType::TsTupleType(tuple) => {
                let mut elements = Vec::new();
                for element in tuple.elements() {
                    elements.push(match element? {
                        AnyTsTupleTypeElement::AnyTsType(ty) => LoweredTupleElement {
                            ty: self.lower_type(&ty),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        },
                        AnyTsTupleTypeElement::TsNamedTupleTypeElement(element) => {
                            LoweredTupleElement {
                                ty: self.lower_type(&element.ty()?),
                                name: Some(Text::from(
                                    element.name()?.value_token()?.token_text_trimmed(),
                                )),
                                is_optional: element.question_mark_token().is_some(),
                                is_rest: element.dotdotdot_token().is_some(),
                            }
                        }
                        AnyTsTupleTypeElement::TsOptionalTupleTypeElement(element) => {
                            LoweredTupleElement {
                                ty: self.lower_type(&element.ty()?),
                                name: None,
                                is_optional: true,
                                is_rest: false,
                            }
                        }
                        AnyTsTupleTypeElement::TsRestTupleTypeElement(element) => {
                            LoweredTupleElement {
                                ty: self.lower_type(&element.ty()?),
                                name: None,
                                is_optional: false,
                                is_rest: true,
                            }
                        }
                    });
                }
                self.register(LoweredTypeData::Tuple(elements.into_boxed_slice()))
            }
            AnyTsType::TsParenthesizedType(parenthesized) => self.lower_type(&parenthesized.ty()?),
            AnyTsType::TsUnionType(union) => {
                let types = union
                    .types()
                    .into_iter()
                    .map(|ty| Ok(self.lower_type(&ty?)))
                    .collect::<Result<Box<[_]>>>()?;
                self.register(LoweredTypeData::Union(types))
            }
            AnyTsType::TsIntersectionType(intersection) => {
                let types = intersection
                    .types()
                    .into_iter()
                    .map(|ty| Ok(self.lower_type(&ty?)))
                    .collect::<Result<Box<[_]>>>()?;
                self.register(LoweredTypeData::Intersection(types))
            }
            AnyTsType::TsFunctionType(function) => {
                let function = self.lower_signature(
                    None,
                    function.type_parameters(),
                    function.parameters()?,
                    Some(function.return_type()?),
                )?;
                self.register(LoweredTypeData::Function(function))
            }
            AnyTsType::TsConstructorType(constructor) => {
                let function = self.lower_signature(
                    None,
                    constructor.type_parameters(),
                    constructor.parameters()?,
                    Some(AnyTsReturnType::AnyTsType(constructor.return_type()?)),
                )?;
                self.register(LoweredTypeData::Constructor(LoweredConstructor {
                    type_parameters: function.type_parameters,
                    parameters: function.parameters,
                    return_type: Some(function.return_type),
                }))
            }
            AnyTsType::TsObjectType(object) => {
                let mut list = MemberList::default();
                for member in object.members() {
                    self.lower_member(member, &mut list, false);
                }
                let members = self.finish_members(list);
                self.register(LoweredTypeData::Object(members))
            }
            _ => self.unknown(format!("unsupported type syntax {:?}", ty.syntax().kind())),
        })
    }

    fn array_reference(&self) -> LoweredTypeReference {
        match self.lowerer.ids.type_slot("Array") {
            Some(index) => self.global_reference(index),
            None => LoweredTypeReference::Global(Text::from("GLOBAL_ARRAY_ID")),
        }
    }

    /// Lowers an `extends` clause entry, keeping constructor interfaces as interfaces.
    fn lower_base(&mut self, base: &TsReferenceType) -> LoweredTypeReference {
        match self.lower_reference_type(base, false) {
            Ok(reference) => reference,
            Err(error) => self.unknown(format!("{error:#}")),
        }
    }

    /// Lowers a named type. With `constructors_as_classes`, a class's constructor
    /// interface lowers to the class value, since that is the type of the value.
    fn lower_reference_type(
        &mut self,
        reference: &TsReferenceType,
        constructors_as_classes: bool,
    ) -> Result<LoweredTypeReference> {
        let path = type_name_path(&reference.name()?)?;
        let arguments = reference
            .type_arguments()
            .map(|arguments| {
                arguments
                    .ts_type_argument_list()
                    .into_iter()
                    .map(|ty| Ok(self.lower_type(&ty?)))
                    .collect::<Result<Box<[_]>>>()
            })
            .transpose()?
            .unwrap_or_default();
        if let [name] = path.as_slice() {
            if let Some(parameter) = self.type_parameter(name.text()) {
                if !arguments.is_empty() {
                    self.gap(format!("type arguments on type parameter {name}"));
                }
                return Ok(parameter);
            }
            if name.text() == "intrinsic" {
                return Ok(self.unknown("intrinsic type"));
            }
        }
        let Some(index) = self.resolve_type_name(&path) else {
            return Ok(self.unknown(format!("unresolved type reference {}", join_path(&path))));
        };
        if constructors_as_classes
            && arguments.is_empty()
            && let Some(class) = self
                .lowerer
                .ids
                .class_for_constructor(&self.lowerer.ids.slot(index).name())
        {
            return Ok(self.global_reference(class));
        }
        let ty = self.global_reference(index);
        Ok(self.register(LoweredTypeData::InstanceOf {
            ty,
            type_parameters: arguments,
        }))
    }

    fn lower_typeof_type(&mut self, typeof_type: &TsTypeofType) -> Result<LoweredTypeReference> {
        let path = type_name_path(&typeof_type.expression_name()?)?;
        Ok(match self.resolve_value_name(&path) {
            Some(index) => self.global_reference(index),
            None => self.unknown(format!("unresolved value {}", join_path(&path))),
        })
    }

    /// Resolves a type name from the innermost enclosing namespace outward.
    fn resolve_type_name(&self, path: &[Text]) -> Option<usize> {
        self.resolve_name(path, |ids, name| ids.type_slot(name))
    }

    fn resolve_value_name(&self, path: &[Text]) -> Option<usize> {
        self.resolve_name(path, |ids, name| ids.value_slot(name))
    }

    fn resolve_name(
        &self,
        path: &[Text],
        lookup: impl Fn(&GlobalIds, &str) -> Option<usize>,
    ) -> Option<usize> {
        let namespace = self.current_namespace();
        (0..=namespace.len()).rev().find_map(|length| {
            let mut qualified = namespace[..length].to_vec();
            qualified.extend_from_slice(path);
            lookup(self.lowerer.ids, &join_path(&qualified))
        })
    }

    /// Namespace used to resolve names written inside this global's declarations.
    fn current_namespace(&self) -> &'a [Text] {
        self.slot.namespace()
    }
}

/// Merged interface declarations lowered into their parts.
struct InterfaceParts {
    type_parameters: Box<[LoweredTypeReference]>,
    bases: Vec<TsReferenceType>,
    list: MemberList,
}

/// Member name and kind before its type is known.
struct MemberKey {
    name: Text,
    kind: LoweredMemberKind,
}

/// Members in declaration order. Method members collect one signature per overload.
#[derive(Clone, Default)]
struct MemberList {
    members: Vec<(LoweredTypeMember, Vec<LoweredTypeReference>)>,
}

impl MemberList {
    fn position(&self, key: &MemberKey) -> Option<usize> {
        self.members.iter().position(|(member, _)| {
            member.name == key.name && same_member_kind(&member.kind, &key.kind)
        })
    }

    fn contains(&self, key: &MemberKey) -> bool {
        self.position(key).is_some()
    }

    /// Keeps the first declaration of a property.
    fn push_property(&mut self, key: MemberKey, ty: LoweredTypeReference) {
        if self.contains(&key) {
            return;
        }
        self.members.push((
            LoweredTypeMember {
                name: key.name,
                kind: key.kind,
                type_reference: ty,
            },
            Vec::new(),
        ));
    }

    /// Adds an overload to an earlier method of the same name.
    fn push_method(&mut self, key: MemberKey, ty: LoweredTypeReference) {
        match self.position(&key) {
            Some(index) => {
                // A property that already owns this name keeps it.
                let (_, signatures) = &mut self.members[index];
                if !signatures.is_empty() {
                    signatures.push(ty);
                }
            }
            None => self.members.push((
                LoweredTypeMember {
                    name: key.name,
                    kind: key.kind,
                    type_reference: ty.clone(),
                },
                vec![ty],
            )),
        }
    }

    fn push_signature(&mut self, kind: LoweredMemberKind, ty: LoweredTypeReference) {
        let name = if kind == LoweredMemberKind::Constructor {
            Text::from("constructor")
        } else {
            Text::default()
        };
        self.members.push((
            LoweredTypeMember {
                name,
                kind,
                type_reference: ty,
            },
            Vec::new(),
        ));
    }
}

/// Whether two member kinds occupy the same slot on an object.
fn same_member_kind(left: &LoweredMemberKind, right: &LoweredMemberKind) -> bool {
    match (left, right) {
        (LoweredMemberKind::Named { .. }, LoweredMemberKind::Named { .. })
        | (LoweredMemberKind::NamedStatic, LoweredMemberKind::NamedStatic) => true,
        (
            LoweredMemberKind::ComputedValue {
                key_reference: left,
            },
            LoweredMemberKind::ComputedValue {
                key_reference: right,
            },
        )
        | (
            LoweredMemberKind::ComputedStatic {
                key_reference: left,
            },
            LoweredMemberKind::ComputedStatic {
                key_reference: right,
            },
        ) => left == right,
        _ => false,
    }
}

fn records_of_kind(
    group: &GlobalDeclarationGroup,
    matches: impl Fn(&DeclarationKind) -> bool,
) -> Vec<&DeclarationRecord> {
    group
        .declarations()
        .iter()
        .filter(|record| matches(&record.kind))
        .collect()
}

fn parameter_name(parameter: &biome_js_syntax::TsTypeParameter) -> Result<Text> {
    Ok(Text::from(
        parameter.name()?.ident_token()?.token_text_trimmed(),
    ))
}

fn is_unique_symbol_type(ty: &AnyTsType) -> bool {
    let AnyTsType::TsTypeOperatorType(operator) = ty else {
        return false;
    };
    operator
        .operator_token()
        .is_ok_and(|token| token.kind() == T![unique])
        && matches!(operator.ty(), Ok(AnyTsType::TsSymbolType(_)))
}

/// Splits `A.B.C` into its parts.
fn type_name_path(name: &AnyTsName) -> Result<Vec<Text>> {
    match name {
        AnyTsName::JsReferenceIdentifier(name) => {
            Ok(vec![Text::from(name.value_token()?.token_text_trimmed())])
        }
        AnyTsName::TsQualifiedName(name) => {
            let mut path = type_name_path(&name.left()?)?;
            path.push(Text::from(
                name.right()?.value_token()?.token_text_trimmed(),
            ));
            Ok(path)
        }
    }
}

/// Splits `A.B.C` in an expression into its parts.
fn expression_path(expression: &AnyJsExpression) -> Option<Vec<Text>> {
    match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => Some(vec![Text::from(
            identifier
                .name()
                .ok()?
                .value_token()
                .ok()?
                .token_text_trimmed(),
        )]),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let mut path = expression_path(&member.object().ok()?)?;
            let AnyJsName::JsName(name) = member.member().ok()? else {
                return None;
            };
            path.push(Text::from(name.value_token().ok()?.token_text_trimmed()));
            Some(path)
        }
        _ => None,
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
