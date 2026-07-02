//! Lowers collected global declaration groups into a codegen-friendly model.

use anyhow::{Context, Result, bail};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsFormalParameter, AnyJsObjectMemberName, AnyJsParameter, AnyJsRoot,
    AnyTsReturnType, AnyTsType, AnyTsTypeMember, AnyTsVariableAnnotation, JsParameters,
    JsVariableDeclarator, TsCallSignatureTypeMember, TsConstructSignatureTypeMember,
    TsDeclarationModule, TsInterfaceDeclaration, TsPropertySignatureTypeMember,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, Text};

use crate::generate_global_types::{
    collect::{DeclarationKind, DeclarationRecord},
    manifest::{GlobalDeclarationRole, GlobalManifest},
    source::DiscoveredFile,
};

/// Lowered definitions selected for generated globals output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredGlobalTypes {
    globals: Box<[LoweredGlobal]>,
}

impl LoweredGlobalTypes {
    /// Returns all lowered globals in deterministic output order.
    pub fn globals(&self) -> &[LoweredGlobal] {
        &self.globals
    }

    /// Returns one lowered global by TypeScript global name.
    pub fn global(&self, name: &str) -> Option<&LoweredGlobal> {
        self.globals.iter().find(|global| global.name() == name)
    }
}

/// One lowered predefined global entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredGlobal {
    name: Text,
    id_constant: &'static str,
    data: LoweredTypeData,
}

impl LoweredGlobal {
    /// TypeScript global name.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// Rust constant used by `GlobalsResolverBuilder::set_type_data`.
    pub fn id_constant(&self) -> &'static str {
        self.id_constant
    }

    /// Lowered type data for this global.
    pub fn data(&self) -> &LoweredTypeData {
        &self.data
    }
}

/// Lowered type data variants supported by the generator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweredTypeData {
    Class(LoweredClass),
    Constructor(LoweredConstructor),
    Function(LoweredFunction),
}

/// Lowered class-like global.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredClass {
    name: Text,
    members: Box<[LoweredTypeMember]>,
}

impl LoweredClass {
    /// Class name.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// Class members in declaration order.
    pub fn members(&self) -> &[LoweredTypeMember] {
        &self.members
    }

    /// Returns the first member with `name`.
    pub fn member(&self, name: &str) -> Option<&LoweredTypeMember> {
        self.members
            .iter()
            .find(|member| member.name.text() == name)
    }
}

/// Lowered constructor helper data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredConstructor {
    parameters: Box<[LoweredFunctionParameter]>,
    return_type: Option<LoweredTypeReference>,
}

impl LoweredConstructor {
    /// Constructor parameters in declaration order.
    pub fn parameters(&self) -> &[LoweredFunctionParameter] {
        &self.parameters
    }

    /// Constructor return type.
    pub fn return_type(&self) -> Option<&LoweredTypeReference> {
        self.return_type.as_ref()
    }
}

/// Lowered function helper data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredFunction {
    name: Option<Text>,
    parameters: Box<[LoweredFunctionParameter]>,
    return_type: LoweredTypeReference,
}

impl LoweredFunction {
    /// Function name, if present.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(Text::text)
    }

    /// Function parameters in declaration order.
    pub fn parameters(&self) -> &[LoweredFunctionParameter] {
        &self.parameters
    }

    /// Function return type.
    pub fn return_type(&self) -> &LoweredTypeReference {
        &self.return_type
    }
}

/// Lowered named function parameter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredFunctionParameter {
    name: Text,
    type_reference: LoweredTypeReference,
    is_optional: bool,
    is_rest: bool,
}

impl LoweredFunctionParameter {
    /// Parameter binding name.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// Parameter type.
    pub fn type_reference(&self) -> &LoweredTypeReference {
        &self.type_reference
    }

    /// Returns whether this parameter is optional.
    pub fn is_optional(&self) -> bool {
        self.is_optional
    }

    /// Returns whether this parameter is a rest parameter.
    pub fn is_rest(&self) -> bool {
        self.is_rest
    }
}

/// Lowered class/interface member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredTypeMember {
    name: Text,
    kind: LoweredMemberKind,
    type_reference: LoweredTypeReference,
}

impl LoweredTypeMember {
    /// Member name.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// Member kind.
    pub fn kind(&self) -> &LoweredMemberKind {
        &self.kind
    }

    /// Member type.
    pub fn type_reference(&self) -> &LoweredTypeReference {
        &self.type_reference
    }
}

/// Lowered member kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweredMemberKind {
    Named { optional: bool },
    NamedStatic,
    Constructor,
    CallSignature,
}

/// Lowered type reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweredTypeReference {
    Predefined(&'static str),
}

/// Lowers supported global groups into generated global type definitions.
pub fn lower_global_types(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
) -> Result<LoweredGlobalTypes> {
    let mut source_cache = ParsedSourceCache::new(source_files);
    let Some(error_group) = manifest.global_group("Error") else {
        return Ok(LoweredGlobalTypes {
            globals: Box::default(),
        });
    };
    if !error_group.has_role(GlobalDeclarationRole::Type) {
        bail!("Error global must have a type-side declaration");
    }
    if !error_group.has_role(GlobalDeclarationRole::Value) {
        bail!("Error global must have a value-side declaration");
    }
    ensure_error_value_references_constructor(error_group.declarations(), &mut source_cache)?;

    let Some(error_constructor_group) = manifest.global_group("ErrorConstructor") else {
        bail!("Error global value side references missing ErrorConstructor group");
    };
    if !error_constructor_group.has_role(GlobalDeclarationRole::Type) {
        bail!("ErrorConstructor must have a type-side declaration");
    }

    let mut members = Vec::new();
    let mut saw_error_interface = false;
    for record in error_group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => {
                saw_error_interface = true;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in the Error global")
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {
                continue;
            }
        }
        let declaration = source_cache
            .find_interface_declaration(record)?
            .with_context(|| {
                format!(
                    "failed to find interface declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        lower_error_interface_members(&declaration, &mut members)?;
    }
    if !saw_error_interface {
        bail!("Error global must include an interface declaration");
    }

    let ErrorConstructorSignatures {
        constructor,
        call,
        prototype,
    } = lower_error_constructor_signatures(
        error_constructor_group.declarations(),
        &mut source_cache,
    )?;

    members.push(LoweredTypeMember {
        name: Text::from("constructor"),
        kind: LoweredMemberKind::Constructor,
        type_reference: LoweredTypeReference::Predefined("GLOBAL_ERROR_CONSTRUCTOR_ID"),
    });
    members.push(LoweredTypeMember {
        name: Text::from("call"),
        kind: LoweredMemberKind::CallSignature,
        type_reference: LoweredTypeReference::Predefined("GLOBAL_ERROR_CALL_ID"),
    });
    if let Some(prototype) = prototype {
        members.push(prototype);
    }

    Ok(LoweredGlobalTypes {
        globals: Box::new([
            LoweredGlobal {
                name: Text::from("Error"),
                id_constant: "ERROR_ID_GLOBAL_TYPE_ID",
                data: LoweredTypeData::Class(LoweredClass {
                    name: Text::from("Error"),
                    members: members.into_boxed_slice(),
                }),
            },
            LoweredGlobal {
                name: Text::from("Error.constructor"),
                id_constant: "ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID",
                data: LoweredTypeData::Constructor(constructor),
            },
            LoweredGlobal {
                name: Text::from("Error.call"),
                id_constant: "ERROR_CALL_ID_GLOBAL_TYPE_ID",
                data: LoweredTypeData::Function(call),
            },
        ]),
    })
}

/// Lowered pieces extracted from `interface ErrorConstructor`.
struct ErrorConstructorSignatures {
    constructor: LoweredConstructor,
    call: LoweredFunction,
    prototype: Option<LoweredTypeMember>,
}

struct ParsedSource<'a> {
    repo_relative: &'a str,
    module: TsDeclarationModule,
}

struct ParsedSourceCache<'a> {
    source_files: &'a [DiscoveredFile],
    parsed: Vec<ParsedSource<'a>>,
}

impl<'a> ParsedSourceCache<'a> {
    fn new(source_files: &'a [DiscoveredFile]) -> Self {
        Self {
            source_files,
            parsed: Vec::new(),
        }
    }

    fn module_for(&mut self, record: &DeclarationRecord) -> Result<&TsDeclarationModule> {
        let repo_relative = record.file_repo_relative.as_ref();
        if let Some(index) = self
            .parsed
            .iter()
            .position(|source| source.repo_relative == repo_relative)
        {
            return Ok(&self.parsed[index].module);
        }

        let source_file = self
            .source_files
            .iter()
            .find(|source_file| source_file.repo_relative == repo_relative)
            .with_context(|| {
                format!("collector record references missing source file {repo_relative}")
            })?;
        let source = std::str::from_utf8(&source_file.bytes)
            .with_context(|| format!("{} is not valid UTF-8", source_file.repo_relative))?;
        let parsed = parse(source, JsFileSource::d_ts(), JsParserOptions::default());
        let AnyJsRoot::TsDeclarationModule(module) = parsed.tree() else {
            bail!(
                "{} is not a TypeScript declaration module",
                source_file.repo_relative
            );
        };

        self.parsed.push(ParsedSource {
            repo_relative: source_file.repo_relative.as_str(),
            module,
        });

        self.parsed
            .last()
            .map(|parsed_source| &parsed_source.module)
            .context("parsed source cache did not retain pushed source")
    }

    /// Finds the AST node for a collected interface record.
    fn find_interface_declaration(
        &mut self,
        record: &DeclarationRecord,
    ) -> Result<Option<TsInterfaceDeclaration>> {
        let module = self.module_for(record)?;
        for node in module.syntax().descendants() {
            if node.kind() == record.syntax_kind && node.text_trimmed_range() == record.text_range {
                return Ok(TsInterfaceDeclaration::cast(node));
            }
        }

        Ok(None)
    }

    /// Finds the AST node for a collected variable declarator.
    fn find_variable_declarator(
        &mut self,
        record: &DeclarationRecord,
    ) -> Result<Option<JsVariableDeclarator>> {
        let module = self.module_for(record)?;
        for node in module.syntax().descendants() {
            if node.kind() == record.syntax_kind && node.text_trimmed_range() == record.text_range {
                return Ok(JsVariableDeclarator::cast(node));
            }
        }

        Ok(None)
    }
}

/// Lowers supported members from `interface Error`.
fn lower_error_interface_members(
    declaration: &TsInterfaceDeclaration,
    members: &mut Vec<LoweredTypeMember>,
) -> Result<()> {
    if declaration.extends_clause().is_some() {
        bail!("Error interface extends clauses are not supported");
    }

    for member in declaration.members() {
        if let Some(lowered) = lower_error_type_member(member)? {
            members.push(lowered);
        }
    }
    Ok(())
}

/// Lowers one supported `Error` instance member.
fn lower_error_type_member(member: AnyTsTypeMember) -> Result<Option<LoweredTypeMember>> {
    match member {
        AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
            let name = lower_object_member_name(property.name()?)?;
            let optional = property.optional_token().is_some();
            let type_reference = property
                .type_annotation()
                .with_context(|| format!("Error member {name} is missing a type annotation"))?
                .ty()
                .with_context(|| format!("Error member {name} has a malformed type annotation"))
                .and_then(|type_node| lower_type_reference(&type_node))?;
            let type_reference = if optional {
                lower_optional_error_member_reference(&name, type_reference)?
            } else {
                type_reference
            };
            Ok(Some(LoweredTypeMember {
                name,
                kind: LoweredMemberKind::Named { optional },
                type_reference,
            }))
        }
        AnyTsTypeMember::TsMethodSignatureTypeMember(_) => {
            bail!("method signatures are not supported in the Error global")
        }
        AnyTsTypeMember::TsCallSignatureTypeMember(_)
        | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => {
            bail!("Error global signatures must be declared on ErrorConstructor")
        }
        AnyTsTypeMember::JsBogusMember(_) => {
            bail!("bogus members are not supported in the Error global")
        }
        AnyTsTypeMember::TsGetterSignatureTypeMember(_) => {
            bail!("getter signatures are not supported in the Error global")
        }
        AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {
            bail!("index signatures are not supported in the Error global")
        }
        AnyTsTypeMember::TsSetterSignatureTypeMember(_) => {
            bail!("setter signatures are not supported in the Error global")
        }
    }
}

/// Validates supported optional `Error` members.
fn lower_optional_error_member_reference(
    name: &Text,
    type_reference: LoweredTypeReference,
) -> Result<LoweredTypeReference> {
    match (name.text(), type_reference) {
        ("stack", LoweredTypeReference::Predefined("GLOBAL_STRING_ID")) => {
            Ok(LoweredTypeReference::Predefined("GLOBAL_STRING_ID"))
        }
        (name, type_reference) => {
            bail!("unsupported optional Error member {name} with type {type_reference:?}")
        }
    }
}

/// Checks that `declare var Error` points at `ErrorConstructor`.
fn ensure_error_value_references_constructor(
    records: &[DeclarationRecord],
    source_cache: &mut ParsedSourceCache,
) -> Result<()> {
    let mut found_value_side = false;
    for record in records {
        match &record.kind {
            DeclarationKind::VariableDeclarator { .. } => {}
            DeclarationKind::DeclareFunction | DeclarationKind::ImportEquals => {
                bail!("unsupported value-side Error declaration {:?}", record.kind)
            }
            DeclarationKind::Interface | DeclarationKind::TypeAlias => {
                continue;
            }
        }
        let declarator = source_cache
            .find_variable_declarator(record)?
            .with_context(|| {
                format!(
                    "failed to find variable declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;
        let Some(annotation) = declarator.variable_annotation() else {
            bail!("declare var Error is missing a type annotation");
        };
        let AnyTsVariableAnnotation::TsTypeAnnotation(annotation) = annotation else {
            bail!("declare var Error uses unsupported definite assignment annotation");
        };
        let type_reference = lower_type_reference(&annotation.ty()?)?;
        if type_reference != LoweredTypeReference::Predefined("GLOBAL_ERROR_CONSTRUCTOR_ID") {
            bail!("declare var Error must reference ErrorConstructor, got {type_reference:?}");
        }
        found_value_side = true;
    }

    if !found_value_side {
        bail!("Error global must include declare var Error");
    }

    Ok(())
}

/// Lowers signatures and static members from `interface ErrorConstructor`.
fn lower_error_constructor_signatures(
    records: &[DeclarationRecord],
    source_cache: &mut ParsedSourceCache,
) -> Result<ErrorConstructorSignatures> {
    let mut constructor = None;
    let mut call = None;
    let mut prototype = None;

    for record in records {
        if !matches!(&record.kind, DeclarationKind::Interface) {
            continue;
        }
        let declaration = source_cache
            .find_interface_declaration(record)?
            .with_context(|| {
                format!(
                    "failed to find interface declaration {} at {:?}",
                    record.declared_name.text(),
                    record.text_range
                )
            })?;

        for member in declaration.members() {
            match member {
                AnyTsTypeMember::TsConstructSignatureTypeMember(member) => {
                    let lowered = lower_construct_signature(&member)?;
                    if constructor.replace(lowered).is_some() {
                        bail!("ErrorConstructor has multiple construct signatures");
                    }
                }
                AnyTsTypeMember::TsCallSignatureTypeMember(member) => {
                    let lowered = lower_call_signature(&member)?;
                    if call.replace(lowered).is_some() {
                        bail!("ErrorConstructor has multiple call signatures");
                    }
                }
                AnyTsTypeMember::TsPropertySignatureTypeMember(member) => {
                    let lowered = lower_error_constructor_property_member(&member)?;
                    if prototype.replace(lowered).is_some() {
                        bail!("ErrorConstructor has multiple prototype properties");
                    }
                }
                AnyTsTypeMember::TsMethodSignatureTypeMember(_) => {
                    bail!("method signatures are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::JsBogusMember(_) => {
                    bail!("bogus members are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::TsGetterSignatureTypeMember(_) => {
                    bail!("getter signatures are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {
                    bail!("index signatures are not supported in ErrorConstructor")
                }
                AnyTsTypeMember::TsSetterSignatureTypeMember(_) => {
                    bail!("setter signatures are not supported in ErrorConstructor")
                }
            }
        }
    }

    Ok(ErrorConstructorSignatures {
        constructor: constructor.context("ErrorConstructor is missing a construct signature")?,
        call: call.context("ErrorConstructor is missing a call signature")?,
        prototype,
    })
}

/// Lowers supported `ErrorConstructor` static properties.
fn lower_error_constructor_property_member(
    property: &TsPropertySignatureTypeMember,
) -> Result<LoweredTypeMember> {
    let name = lower_object_member_name(property.name()?)?;
    if name.text() != "prototype" {
        bail!("unsupported ErrorConstructor property {name}");
    }
    if property.optional_token().is_some() {
        bail!("ErrorConstructor.prototype must not be optional");
    }
    let type_reference = property
        .type_annotation()
        .context("ErrorConstructor.prototype is missing a type annotation")?
        .ty()
        .context("ErrorConstructor.prototype has malformed type annotation")
        .and_then(|type_node| lower_type_reference(&type_node))
        .map(instance_return_reference)?;

    Ok(LoweredTypeMember {
        name,
        kind: LoweredMemberKind::NamedStatic,
        type_reference,
    })
}

/// Lowers the `new Error(...)` construct signature.
fn lower_construct_signature(
    member: &TsConstructSignatureTypeMember,
) -> Result<LoweredConstructor> {
    if member.type_parameters().is_some() {
        bail!("generic Error constructor signatures are not supported");
    }
    let return_type = member
        .type_annotation()
        .context("ErrorConstructor construct signature is missing a return type")?
        .ty()
        .context("ErrorConstructor construct signature has malformed return type")
        .and_then(|type_node| lower_type_reference(&type_node))?;

    Ok(LoweredConstructor {
        parameters: lower_parameters(member.parameters()?)?,
        return_type: Some(return_type),
    })
}

/// Lowers the `Error(...)` call signature.
fn lower_call_signature(member: &TsCallSignatureTypeMember) -> Result<LoweredFunction> {
    if member.type_parameters().is_some() {
        bail!("generic Error call signatures are not supported");
    }
    let return_type = member
        .return_type_annotation()
        .context("ErrorConstructor call signature is missing a return type")?
        .ty()
        .context("ErrorConstructor call signature has malformed return type")
        .and_then(|return_type_node| lower_return_type_reference(&return_type_node))?;

    Ok(LoweredFunction {
        name: Some(Text::from("Error")),
        parameters: lower_parameters(member.parameters()?)?,
        return_type: instance_return_reference(return_type),
    })
}

/// Lowers function-like parameters for the `ErrorConstructor`.
fn lower_parameters(parameters: JsParameters) -> Result<Box<[LoweredFunctionParameter]>> {
    let mut lowered = Vec::new();
    for parameter in parameters.items() {
        match parameter? {
            AnyJsParameter::AnyJsFormalParameter(parameter) => {
                let AnyJsFormalParameter::JsFormalParameter(parameter) = parameter else {
                    bail!("unsupported ErrorConstructor formal parameter");
                };
                let name = lower_binding_name(parameter.binding()?)?;
                let is_optional = parameter.question_mark_token().is_some();
                let type_reference = parameter
                    .type_annotation()
                    .context("ErrorConstructor parameter is missing a type annotation")?
                    .ty()
                    .context("ErrorConstructor parameter has malformed type annotation")
                    .and_then(|type_node| lower_type_reference(&type_node))?;
                lowered.push(LoweredFunctionParameter {
                    name,
                    type_reference,
                    is_optional,
                    is_rest: false,
                });
            }
            AnyJsParameter::JsRestParameter(parameter) => {
                let name = lower_binding_name(parameter.binding()?)?;
                let type_reference = parameter
                    .type_annotation()
                    .context("ErrorConstructor rest parameter is missing a type annotation")?
                    .ty()
                    .context("ErrorConstructor rest parameter has malformed type annotation")
                    .and_then(|type_node| lower_type_reference(&type_node))?;
                lowered.push(LoweredFunctionParameter {
                    name,
                    type_reference,
                    is_optional: false,
                    is_rest: true,
                });
            }
            AnyJsParameter::TsThisParameter(_) => {
                bail!("this parameters are not supported in ErrorConstructor")
            }
        }
    }

    Ok(lowered.into_boxed_slice())
}

/// Extracts a simple identifier binding name.
fn lower_binding_name(binding: AnyJsBindingPattern) -> Result<Text> {
    let Some(binding) = binding.as_any_js_binding() else {
        bail!("unsupported destructured ErrorConstructor parameter");
    };
    let Some(binding) = binding.as_js_identifier_binding() else {
        bail!("unsupported ErrorConstructor parameter binding");
    };
    Ok(Text::from(binding.name_token()?.token_text_trimmed()))
}

/// Extracts a supported object member name.
fn lower_object_member_name(name: AnyJsObjectMemberName) -> Result<Text> {
    match name {
        AnyJsObjectMemberName::JsLiteralMemberName(name) => Ok(Text::from(name.name()?)),
        AnyJsObjectMemberName::JsComputedMemberName(_)
        | AnyJsObjectMemberName::JsMetavariable(_) => {
            bail!("unsupported computed or metavariable member name in Error global")
        }
    }
}

/// Maps a supported TypeScript type node to a lowered reference.
fn lower_type_reference(type_node: &AnyTsType) -> Result<LoweredTypeReference> {
    match type_node {
        AnyTsType::TsStringType(_) => Ok(LoweredTypeReference::Predefined("GLOBAL_STRING_ID")),
        AnyTsType::TsVoidType(_) => Ok(LoweredTypeReference::Predefined("GLOBAL_VOID_ID")),
        AnyTsType::TsReferenceType(reference) => {
            let name = reference.name().context("missing type reference name")?;
            let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = name else {
                bail!("qualified type references are not supported in Error global")
            };
            let name = Text::from(identifier.value_token()?.token_text_trimmed());
            Ok(match name.text() {
                "Error" => LoweredTypeReference::Predefined("GLOBAL_ERROR_ID"),
                "ErrorConstructor" => {
                    LoweredTypeReference::Predefined("GLOBAL_ERROR_CONSTRUCTOR_ID")
                }
                _ => bail!("unresolved type reference {name} in Error global"),
            })
        }
        _ => bail!("unsupported type reference in Error global: {type_node:?}"),
    }
}

/// Maps a supported return type node to a lowered reference.
fn lower_return_type_reference(return_type_node: &AnyTsReturnType) -> Result<LoweredTypeReference> {
    match return_type_node {
        AnyTsReturnType::AnyTsType(type_node) => lower_type_reference(type_node),
        AnyTsReturnType::TsAssertsReturnType(_) | AnyTsReturnType::TsPredicateReturnType(_) => {
            bail!("predicate return types are not supported in Error global")
        }
    }
}

/// Converts constructor returns from `Error` to `InstanceOf<Error>`.
fn instance_return_reference(reference: LoweredTypeReference) -> LoweredTypeReference {
    match reference {
        LoweredTypeReference::Predefined("GLOBAL_ERROR_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ERROR_ID")
        }
        reference => reference,
    }
}
