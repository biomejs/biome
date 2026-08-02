//! Lowers collected global declaration groups into a codegen-friendly model.

use anyhow::{Context, Result, bail};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFormalParameter, AnyJsName, AnyJsObjectMemberName,
    AnyJsParameter, AnyJsRoot, AnyTsReturnType, AnyTsType, AnyTsTypeMember,
    AnyTsVariableAnnotation, JsFormalParameter, JsParameters, JsVariableDeclarator, T,
    TsCallSignatureTypeMember, TsConstructSignatureTypeMember, TsDeclarationModule, TsFunctionType,
    TsInterfaceDeclaration, TsMethodSignatureTypeMember, TsPropertySignatureTypeMember,
    TsTypeParameters,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, SyntaxResult, Text};

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
    Interface(LoweredInterface),
    Symbol,
}

/// Lowered class-like global.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredClass {
    name: Text,
    type_parameters: Box<[LoweredTypeReference]>,
    members: Box<[LoweredTypeMember]>,
}

impl LoweredClass {
    /// Class name.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// Class type parameters in declaration order.
    pub fn type_parameters(&self) -> &[LoweredTypeReference] {
        &self.type_parameters
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

/// Lowered interface data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredInterface {
    name: Text,
    members: Box<[LoweredTypeMember]>,
}

impl LoweredInterface {
    /// Interface name.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// Interface members in declaration order.
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
    is_async: bool,
    type_parameters: Box<[LoweredTypeReference]>,
    name: Option<Text>,
    parameters: Box<[LoweredFunctionParameter]>,
    return_type: LoweredTypeReference,
}

impl LoweredFunction {
    /// Returns whether this function is `async`.
    pub fn is_async(&self) -> bool {
        self.is_async
    }

    /// Function type parameters in declaration order.
    pub fn type_parameters(&self) -> &[LoweredTypeReference] {
        &self.type_parameters
    }

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

/// Parameter data shared by generated functions and constructors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredFunctionParameter {
    binding: LoweredFunctionParameterBinding,
    type_reference: LoweredTypeReference,
    is_optional: bool,
    is_rest: bool,
}

impl LoweredFunctionParameter {
    /// Returns the parameter form used by the emitter.
    pub fn binding(&self) -> &LoweredFunctionParameterBinding {
        &self.binding
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

/// Selects the emitted `FunctionParameter` variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweredFunctionParameterBinding {
    Named(Text),
    Pattern,
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
    ComputedValue { key_reference: LoweredTypeReference },
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
    let mut globals = Vec::new();

    lower_array_globals(manifest, &mut source_cache, &mut globals)?;
    lower_error_globals(manifest, &mut source_cache, &mut globals)?;
    lower_regexp_globals(manifest, &mut source_cache, &mut globals)?;
    lower_symbol_globals(manifest, &mut source_cache, &mut globals)?;
    lower_disposable_global(manifest, &mut source_cache, &mut globals, DISPOSABLE_GLOBAL)?;
    lower_disposable_global(
        manifest,
        &mut source_cache,
        &mut globals,
        ASYNC_DISPOSABLE_GLOBAL,
    )?;
    lower_memberless_class_global(manifest, &mut source_cache, &mut globals, DATE_GLOBAL)?;
    lower_memberless_class_global(manifest, &mut source_cache, &mut globals, MAP_GLOBAL)?;
    lower_memberless_class_global(manifest, &mut source_cache, &mut globals, SET_GLOBAL)?;
    lower_memberless_class_global(manifest, &mut source_cache, &mut globals, WEAK_MAP_GLOBAL)?;

    Ok(LoweredGlobalTypes {
        globals: globals.into_boxed_slice(),
    })
}

/// Lowers `Error` and its constructor and call helpers when present.
fn lower_error_globals(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let Some(error_group) = manifest.global_group("Error") else {
        return Ok(());
    };
    if !error_group.has_role(GlobalDeclarationRole::Type) {
        bail!("Error global must have a type-side declaration");
    }
    if !error_group.has_role(GlobalDeclarationRole::Value) {
        bail!("Error global must have a value-side declaration");
    }
    ensure_error_value_references_constructor(error_group.declarations(), source_cache)?;

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
    } = lower_error_constructor_signatures(error_constructor_group.declarations(), source_cache)?;

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

    globals.push(LoweredGlobal {
        name: Text::from("Error"),
        id_constant: "ERROR_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Class(LoweredClass {
            name: Text::from("Error"),
            type_parameters: Box::default(),
            members: members.into_boxed_slice(),
        }),
    });
    globals.push(LoweredGlobal {
        name: Text::from("Error.constructor"),
        id_constant: "ERROR_CONSTRUCTOR_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Constructor(constructor),
    });
    globals.push(LoweredGlobal {
        name: Text::from("Error.call"),
        id_constant: "ERROR_CALL_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Function(call),
    });

    Ok(())
}

/// Lowered pieces extracted from `interface ErrorConstructor`.
struct ErrorConstructorSignatures {
    constructor: LoweredConstructor,
    call: LoweredFunction,
    prototype: Option<LoweredTypeMember>,
}

/// Describes how one disposable interface (`Disposable`/`AsyncDisposable`) and its dispose
/// helper are lowered. Every field is a static string because it feeds generated Rust source.
#[derive(Clone, Copy)]
struct DisposableGlobalSpec {
    /// Interface name in the `.d.ts` source and the manifest group key.
    interface_name: &'static str,
    /// `GlobalTypeId` constant the lowered interface registers into.
    global_id_constant: &'static str,
    /// Display name of the single computed member (e.g. `[Symbol.dispose]`).
    member_name: &'static str,
    /// `GLOBAL_*` reference the computed member key must resolve to.
    symbol_id: &'static str,
    /// Display name of the synthesized dispose helper global.
    helper_name: &'static str,
    /// `GlobalTypeId` constant the dispose helper registers into.
    helper_id_constant: &'static str,
    /// `GLOBAL_*` reference the member's value type points at (the helper).
    helper_type_id: &'static str,
    /// Whether the helper returns `void` or `PromiseLike<void>`.
    return_kind: DisposableReturnKind,
}

/// Return shape of a dispose helper, mapping the `.d.ts` signature to the lowered return type.
#[derive(Clone, Copy)]
enum DisposableReturnKind {
    Void,
    PromiseLikeVoid,
}

impl DisposableReturnKind {
    /// Whether the lowered dispose helper is an `async` function.
    fn helper_is_async(self) -> bool {
        matches!(self, Self::PromiseLikeVoid)
    }

    /// Predefined ID constant the lowered return type must resolve to.
    fn return_type_id(self) -> &'static str {
        match self {
            Self::Void => "GLOBAL_VOID_ID",
            Self::PromiseLikeVoid => "GLOBAL_INSTANCEOF_PROMISE_ID",
        }
    }
}

/// Lowering spec for the `Disposable` interface and its `[Symbol.dispose](): void` helper.
const DISPOSABLE_GLOBAL: DisposableGlobalSpec = DisposableGlobalSpec {
    interface_name: "Disposable",
    global_id_constant: "DISPOSABLE_ID_GLOBAL_TYPE_ID",
    member_name: "[Symbol.dispose]",
    symbol_id: "GLOBAL_SYMBOL_DISPOSE_ID",
    helper_name: "Disposable[Symbol.dispose]",
    helper_id_constant: "DISPOSABLE_DISPOSE_ID_GLOBAL_TYPE_ID",
    helper_type_id: "GLOBAL_DISPOSABLE_DISPOSE_ID",
    return_kind: DisposableReturnKind::Void,
};

/// Lowering spec for `AsyncDisposable` and its `[Symbol.asyncDispose](): PromiseLike<void>` helper.
const ASYNC_DISPOSABLE_GLOBAL: DisposableGlobalSpec = DisposableGlobalSpec {
    interface_name: "AsyncDisposable",
    global_id_constant: "ASYNC_DISPOSABLE_ID_GLOBAL_TYPE_ID",
    member_name: "[Symbol.asyncDispose]",
    symbol_id: "GLOBAL_SYMBOL_ASYNC_DISPOSE_ID",
    helper_name: "AsyncDisposable[Symbol.asyncDispose]",
    helper_id_constant: "ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID",
    helper_type_id: "GLOBAL_ASYNC_DISPOSABLE_ASYNC_DISPOSE_ID",
    return_kind: DisposableReturnKind::PromiseLikeVoid,
};

/// Configuration for lowering a global interface to a class without members.
#[derive(Clone, Copy)]
struct MemberlessClassSpec {
    name: &'static str,
    id_constant: &'static str,
    type_parameter_ids: &'static [&'static str],
}

const DATE_GLOBAL: MemberlessClassSpec = MemberlessClassSpec {
    name: "Date",
    id_constant: "DATE_ID_GLOBAL_TYPE_ID",
    type_parameter_ids: &[],
};

const MAP_GLOBAL: MemberlessClassSpec = MemberlessClassSpec {
    name: "Map",
    id_constant: "MAP_ID_GLOBAL_TYPE_ID",
    type_parameter_ids: &["GLOBAL_T_ID", "GLOBAL_U_ID"],
};

const SET_GLOBAL: MemberlessClassSpec = MemberlessClassSpec {
    name: "Set",
    id_constant: "SET_ID_GLOBAL_TYPE_ID",
    type_parameter_ids: &["GLOBAL_T_ID"],
};

const WEAK_MAP_GLOBAL: MemberlessClassSpec = MemberlessClassSpec {
    name: "WeakMap",
    id_constant: "WEAK_MAP_ID_GLOBAL_TYPE_ID",
    type_parameter_ids: &["GLOBAL_T_ID", "GLOBAL_U_ID"],
};

const REGEXP_EXEC_RETURN_TYPE_VARIANT_COUNT: usize = 2;

/// Array members retained in the resolver's reduced projection.
#[derive(Clone, Copy)]
enum SelectedArrayMember {
    Filter,
    ForEach,
    Length,
    Map,
}

impl SelectedArrayMember {
    /// Returns the TypeScript spelling used in diagnostics.
    fn name(self) -> &'static str {
        match self {
            Self::Filter => "filter",
            Self::ForEach => "forEach",
            Self::Length => "length",
            Self::Map => "map",
        }
    }
}

/// Return shape required from a selected Array callback declaration.
#[derive(Clone, Copy)]
enum ArrayCallbackReturn<'a> {
    Reference(&'a str),
    Unknown,
    Void,
}

/// Validates selected members across merged declarations and builds the resolver projection.
fn lower_array_globals(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let Some(array_group) = manifest.global_group("Array") else {
        return Ok(());
    };
    if !array_group.has_role(GlobalDeclarationRole::Type) {
        bail!("Array global must have a type-side declaration");
    }

    let mut saw_interface = false;
    let mut saw_filter = false;
    let mut saw_for_each = false;
    let mut saw_length = false;
    let mut saw_map = false;

    for record in array_group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => {
                saw_interface = true;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in the Array global")
            }
            DeclarationKind::VariableDeclarator { .. } => continue,
            DeclarationKind::DeclareFunction | DeclarationKind::ImportEquals => {
                bail!("unsupported value-side Array declaration")
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
        let array_type_parameter = validate_array_interface_type_parameter(&declaration)?;

        for member in declaration.members() {
            match member {
                AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                    let Some(selected) = selected_array_member(method.name()?)? else {
                        continue;
                    };
                    match selected {
                        SelectedArrayMember::Filter => {
                            if method.type_parameters().is_some() {
                                continue;
                            }
                            if saw_filter {
                                bail!("Array has multiple non-generic filter overloads");
                            }
                            validate_array_filter(&method, array_type_parameter.text())?;
                            saw_filter = true;
                        }
                        SelectedArrayMember::ForEach => {
                            if saw_for_each {
                                bail!("Array has multiple forEach methods");
                            }
                            validate_array_for_each(&method, array_type_parameter.text())?;
                            saw_for_each = true;
                        }
                        SelectedArrayMember::Length => {
                            bail!("Array.length must be a property")
                        }
                        SelectedArrayMember::Map => {
                            if saw_map {
                                bail!("Array has multiple map methods");
                            }
                            validate_array_map(&method, array_type_parameter.text())?;
                            saw_map = true;
                        }
                    }
                }
                AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                    let Some(selected) = selected_array_member(property.name()?)? else {
                        continue;
                    };
                    if !matches!(selected, SelectedArrayMember::Length) {
                        bail!("Array.{} must be a method", selected.name());
                    }
                    if saw_length {
                        bail!("Array has multiple length properties");
                    }
                    validate_array_length(&property)?;
                    saw_length = true;
                }
                AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => {
                    reject_selected_array_accessor(getter.name()?)?;
                }
                AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => {
                    reject_selected_array_accessor(setter.name()?)?;
                }
                AnyTsTypeMember::JsBogusMember(_)
                | AnyTsTypeMember::TsCallSignatureTypeMember(_)
                | AnyTsTypeMember::TsConstructSignatureTypeMember(_)
                | AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {}
            }
        }
    }

    if !saw_interface {
        bail!("Array global must include an interface declaration");
    }
    if !saw_filter {
        bail!("Array is missing a non-generic filter overload");
    }
    if !saw_for_each {
        bail!("Array is missing forEach");
    }
    if !saw_length {
        bail!("Array is missing length");
    }
    if !saw_map {
        bail!("Array is missing map");
    }

    globals.push(LoweredGlobal {
        name: Text::from("Array"),
        id_constant: "ARRAY_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Class(LoweredClass {
            name: Text::from("Array"),
            type_parameters: Box::new([LoweredTypeReference::Predefined("GLOBAL_T_ID")]),
            members: Box::new([
                LoweredTypeMember {
                    name: Text::from("filter"),
                    kind: LoweredMemberKind::Named { optional: false },
                    type_reference: LoweredTypeReference::Predefined("GLOBAL_ARRAY_FILTER_ID"),
                },
                LoweredTypeMember {
                    name: Text::from("forEach"),
                    kind: LoweredMemberKind::Named { optional: false },
                    type_reference: LoweredTypeReference::Predefined("GLOBAL_ARRAY_FOREACH_ID"),
                },
                LoweredTypeMember {
                    name: Text::from("map"),
                    kind: LoweredMemberKind::Named { optional: false },
                    type_reference: LoweredTypeReference::Predefined("GLOBAL_ARRAY_MAP_ID"),
                },
                LoweredTypeMember {
                    name: Text::from("length"),
                    kind: LoweredMemberKind::Named { optional: false },
                    type_reference: LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID"),
                },
            ]),
        }),
    });
    globals.push(array_method_global(
        "Array.prototype.filter",
        "ARRAY_FILTER_ID_GLOBAL_TYPE_ID",
        Box::default(),
        "GLOBAL_CONDITIONAL_CALLBACK_ID",
        "GLOBAL_INSTANCEOF_ARRAY_T_ID",
    ));
    globals.push(array_method_global(
        "Array.prototype.forEach",
        "ARRAY_FOREACH_ID_GLOBAL_TYPE_ID",
        Box::default(),
        "GLOBAL_VOID_CALLBACK_ID",
        "GLOBAL_VOID_ID",
    ));
    globals.push(array_method_global(
        "Array.prototype.map",
        "ARRAY_MAP_ID_GLOBAL_TYPE_ID",
        Box::new([LoweredTypeReference::Predefined("GLOBAL_U_ID")]),
        "GLOBAL_MAP_CALLBACK_ID",
        "GLOBAL_INSTANCEOF_ARRAY_U_ID",
    ));

    Ok(())
}

/// Builds the resolver's single-callback shape, omitting the validated `thisArg`.
fn array_method_global(
    name: &'static str,
    id_constant: &'static str,
    type_parameters: Box<[LoweredTypeReference]>,
    parameter_type_id: &'static str,
    return_type_id: &'static str,
) -> LoweredGlobal {
    LoweredGlobal {
        name: Text::from(name),
        id_constant,
        data: LoweredTypeData::Function(LoweredFunction {
            is_async: false,
            type_parameters,
            name: Some(Text::from(name)),
            parameters: Box::new([LoweredFunctionParameter {
                binding: LoweredFunctionParameterBinding::Pattern,
                type_reference: LoweredTypeReference::Predefined(parameter_type_id),
                is_optional: false,
                is_rest: false,
            }]),
            return_type: LoweredTypeReference::Predefined(return_type_id),
        }),
    }
}

/// Requires unextended `Array<T>` with no modifiers, constraint, or default on `T`.
fn validate_array_interface_type_parameter(declaration: &TsInterfaceDeclaration) -> Result<Text> {
    if declaration.extends_clause().is_some() {
        bail!("Array interface extends clauses are not supported");
    }
    single_type_parameter_name(declaration.type_parameters(), "Array interface")
}

/// Recognizes only the Array members represented by the resolver projection.
fn selected_array_member(name: AnyJsObjectMemberName) -> Result<Option<SelectedArrayMember>> {
    let AnyJsObjectMemberName::JsLiteralMemberName(name) = name else {
        return Ok(None);
    };
    Ok(match name.name()?.text() {
        "filter" => Some(SelectedArrayMember::Filter),
        "forEach" => Some(SelectedArrayMember::ForEach),
        "length" => Some(SelectedArrayMember::Length),
        "map" => Some(SelectedArrayMember::Map),
        _ => None,
    })
}

/// Rejects selected members expressed as unsupported getter or setter declarations.
fn reject_selected_array_accessor(name: AnyJsObjectMemberName) -> Result<()> {
    if let Some(member) = selected_array_member(name)? {
        bail!(
            "Array.{} has an unsupported accessor declaration",
            member.name()
        );
    }
    Ok(())
}

/// Requires `length` to be a mutable, required `number` property.
fn validate_array_length(property: &TsPropertySignatureTypeMember) -> Result<()> {
    if property.readonly_token().is_some() {
        bail!("Array.length must not be readonly");
    }
    if property.optional_token().is_some() {
        bail!("Array.length must not be optional");
    }
    let type_node = property
        .type_annotation()
        .context("Array.length is missing a type annotation")?
        .ty()
        .context("Array.length has a malformed type annotation")?;
    if !matches!(type_node, AnyTsType::TsNumberType(_)) {
        bail!("Array.length must be number");
    }
    Ok(())
}

/// Validates the selected non-generic `filter` overload that preserves the element type.
fn validate_array_filter(
    method: &TsMethodSignatureTypeMember,
    array_type_parameter: &str,
) -> Result<()> {
    validate_array_method_parameters(
        method,
        "Array.filter",
        array_type_parameter,
        ArrayCallbackReturn::Unknown,
    )?;
    validate_array_method_array_return(method, "Array.filter", array_type_parameter)
}

/// Requires a non-generic `forEach` method and callback that both return `void`.
fn validate_array_for_each(
    method: &TsMethodSignatureTypeMember,
    array_type_parameter: &str,
) -> Result<()> {
    if method.type_parameters().is_some() {
        bail!("Array.forEach must not be generic");
    }
    validate_array_method_parameters(
        method,
        "Array.forEach",
        array_type_parameter,
        ArrayCallbackReturn::Void,
    )?;
    validate_array_method_void_return(method, "Array.forEach")
}

/// Requires `map` to use one type parameter for both callback and array results.
fn validate_array_map(
    method: &TsMethodSignatureTypeMember,
    array_type_parameter: &str,
) -> Result<()> {
    let map_type_parameter = single_type_parameter_name(method.type_parameters(), "Array.map")?;
    validate_array_method_parameters(
        method,
        "Array.map",
        array_type_parameter,
        ArrayCallbackReturn::Reference(map_type_parameter.text()),
    )?;
    validate_array_method_array_return(method, "Array.map", map_type_parameter.text())
}

/// Requires a callback followed by an optional `any`-typed `thisArg`.
fn validate_array_method_parameters(
    method: &TsMethodSignatureTypeMember,
    owner: &str,
    array_type_parameter: &str,
    callback_return: ArrayCallbackReturn,
) -> Result<()> {
    if method.optional_token().is_some() {
        bail!("{owner} must not be optional");
    }

    let mut parameters = method.parameters()?.items().into_iter();
    let expected_parameters = "a callback and an optional thisArg parameter";
    let callback_parameter =
        required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    let this_argument_parameter =
        required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    if parameters.next().is_some() {
        bail!("{owner} must have a callback and an optional thisArg parameter");
    }

    if callback_parameter.question_mark_token().is_some() {
        bail!("{owner} callback must not be optional");
    }
    let callback_type = callback_parameter
        .type_annotation()
        .with_context(|| format!("{owner} callback is missing a type annotation"))?
        .ty()
        .with_context(|| format!("{owner} callback has a malformed type annotation"))?;
    let AnyTsType::TsFunctionType(callback) = callback_type else {
        bail!("{owner} callback must be a function type");
    };
    validate_array_callback(&callback, owner, array_type_parameter, callback_return)?;

    if this_argument_parameter.question_mark_token().is_none() {
        bail!("{owner} thisArg must be optional");
    }
    let this_argument_type = this_argument_parameter
        .type_annotation()
        .with_context(|| format!("{owner} thisArg is missing a type annotation"))?
        .ty()
        .with_context(|| format!("{owner} thisArg has a malformed type annotation"))?;
    if !matches!(this_argument_type, AnyTsType::TsAnyType(_)) {
        bail!("{owner} thisArg must be any");
    }

    Ok(())
}

/// Extracts a plain formal parameter without decorators or an initializer.
fn required_formal_parameter(
    parameter: Option<SyntaxResult<AnyJsParameter>>,
    owner: &str,
    expected_parameters: &str,
) -> Result<JsFormalParameter> {
    let Some(parameter) = parameter else {
        bail!("{owner} must have {expected_parameters}");
    };
    let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(parameter)) =
        parameter.with_context(|| format!("{owner} has a malformed parameter"))?
    else {
        bail!("{owner} has an unsupported parameter");
    };
    if !parameter.decorators().is_empty() || parameter.initializer().is_some() {
        bail!("{owner} has an unsupported parameter");
    }
    Ok(parameter)
}

/// Requires a non-generic callback with the selected parameters and return shape.
fn validate_array_callback(
    callback: &TsFunctionType,
    owner: &str,
    array_type_parameter: &str,
    callback_return: ArrayCallbackReturn,
) -> Result<()> {
    if callback.type_parameters().is_some() {
        bail!("{owner} callback must not be generic");
    }

    let mut parameters = callback.parameters()?.items().into_iter();
    let expected_parameters = "a callback with three required parameters";
    let value_parameter = required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    let index_parameter = required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    let array_parameter = required_formal_parameter(parameters.next(), owner, expected_parameters)?;
    if parameters.next().is_some() {
        bail!("{owner} must have a callback with three required parameters");
    }
    let value_type = required_array_callback_parameter_type(&value_parameter, owner)?;
    validate_reference_type(&value_type, array_type_parameter, owner)?;
    let index_type = required_array_callback_parameter_type(&index_parameter, owner)?;
    if !matches!(index_type, AnyTsType::TsNumberType(_)) {
        bail!("{owner} callback index parameter must be number");
    }
    let array_type = required_array_callback_parameter_type(&array_parameter, owner)?;
    validate_array_type(&array_type, array_type_parameter, owner)?;

    let return_type = regular_return_type(callback.return_type()?, owner)?;
    match callback_return {
        ArrayCallbackReturn::Reference(type_parameter) => {
            validate_reference_type(&return_type, type_parameter, owner)?;
        }
        ArrayCallbackReturn::Unknown => {
            if !matches!(return_type, AnyTsType::TsUnknownType(_)) {
                bail!("{owner} callback must return unknown");
            }
        }
        ArrayCallbackReturn::Void => {
            if !matches!(return_type, AnyTsType::TsVoidType(_)) {
                bail!("{owner} callback must return void");
            }
        }
    }

    Ok(())
}

/// Returns the type annotation after rejecting an optional callback parameter.
fn required_array_callback_parameter_type(
    parameter: &JsFormalParameter,
    owner: &str,
) -> Result<AnyTsType> {
    if parameter.question_mark_token().is_some() {
        bail!("{owner} callback must have three required parameters");
    }
    parameter
        .type_annotation()
        .with_context(|| format!("{owner} callback parameter is missing a type annotation"))?
        .ty()
        .with_context(|| format!("{owner} callback parameter has a malformed type annotation"))
}

/// Requires a regular array return whose element matches the selected type parameter.
fn validate_array_method_array_return(
    method: &TsMethodSignatureTypeMember,
    owner: &str,
    element_type_parameter: &str,
) -> Result<()> {
    let return_type = method
        .return_type_annotation()
        .with_context(|| format!("{owner} is missing a return type"))?
        .ty()
        .with_context(|| format!("{owner} has a malformed return type"))?;
    let return_type = regular_return_type(return_type, owner)?;
    validate_array_type(&return_type, element_type_parameter, owner)
}

/// Requires an ordinary `void` return rather than a predicate or assertion.
fn validate_array_method_void_return(
    method: &TsMethodSignatureTypeMember,
    owner: &str,
) -> Result<()> {
    let return_type = method
        .return_type_annotation()
        .with_context(|| format!("{owner} is missing a return type"))?
        .ty()
        .with_context(|| format!("{owner} has a malformed return type"))?;
    let return_type = regular_return_type(return_type, owner)?;
    if !matches!(return_type, AnyTsType::TsVoidType(_)) {
        bail!("{owner} must return void");
    }
    Ok(())
}

/// Requires array syntax whose element references the expected type parameter.
fn validate_array_type(
    type_node: &AnyTsType,
    element_type_parameter: &str,
    owner: &str,
) -> Result<()> {
    let AnyTsType::TsArrayType(array_type) = type_node else {
        bail!("{owner} must use an array type");
    };
    validate_reference_type(&array_type.element_type()?, element_type_parameter, owner)
}

/// Requires an unqualified type reference without type arguments.
fn validate_reference_type(type_node: &AnyTsType, expected_name: &str, owner: &str) -> Result<()> {
    let AnyTsType::TsReferenceType(reference) = type_node else {
        bail!("{owner} must reference {expected_name}");
    };
    if reference.type_arguments().is_some() {
        bail!("{owner} must reference {expected_name} without type arguments");
    }
    let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = reference
        .name()
        .with_context(|| format!("{owner} has a missing type reference name"))?
    else {
        bail!("{owner} must reference {expected_name}");
    };
    if identifier.value_token()?.token_text_trimmed().text() != expected_name {
        bail!("{owner} must reference {expected_name}");
    }
    Ok(())
}

/// Rejects predicate and assertion return types.
fn regular_return_type(return_type: AnyTsReturnType, owner: &str) -> Result<AnyTsType> {
    match return_type {
        AnyTsReturnType::AnyTsType(type_node) => Ok(type_node),
        AnyTsReturnType::TsAssertsReturnType(_) | AnyTsReturnType::TsPredicateReturnType(_) => {
            bail!("{owner} must use a regular return type")
        }
    }
}

/// Extracts one unmodified type parameter without a constraint or default.
fn single_type_parameter_name(
    type_parameters: Option<TsTypeParameters>,
    owner: &str,
) -> Result<Text> {
    let type_parameters =
        type_parameters.with_context(|| format!("{owner} must have one type parameter"))?;
    let mut type_parameters = type_parameters.items().into_iter();
    let Some(type_parameter) = type_parameters.next() else {
        bail!("{owner} must have one type parameter");
    };
    let type_parameter =
        type_parameter.with_context(|| format!("{owner} has a malformed type parameter"))?;
    if type_parameters.next().is_some() {
        bail!("{owner} must have one type parameter");
    }
    if !type_parameter.modifiers().is_empty()
        || type_parameter.constraint().is_some()
        || type_parameter.default().is_some()
    {
        bail!("{owner} has an unsupported type parameter");
    }

    Ok(Text::from(
        type_parameter.name()?.ident_token()?.token_text_trimmed(),
    ))
}

struct ParsedSource<'a> {
    repo_relative: &'a str,
    module: TsDeclarationModule,
}

struct ParsedSourceCache<'a> {
    source_files: &'a [DiscoveredFile],
    parsed: Vec<ParsedSource<'a>>,
}

#[derive(Clone, Copy)]
enum SelectedSymbolMember {
    Dispose,
    AsyncDispose,
}

impl SelectedSymbolMember {
    fn name(self) -> &'static str {
        match self {
            Self::Dispose => "dispose",
            Self::AsyncDispose => "asyncDispose",
        }
    }
}

/// Lowers the predefined `Symbol` projection and its two well-known symbol helpers.
fn lower_symbol_globals(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let Some(symbol_group) = manifest.global_group("Symbol") else {
        return Ok(());
    };
    if !symbol_group.has_role(GlobalDeclarationRole::Type) {
        bail!("Symbol global must have a type-side declaration");
    }
    if !symbol_group.has_role(GlobalDeclarationRole::Value) {
        bail!("Symbol global must have a value-side declaration");
    }

    validate_symbol_declarations(symbol_group.declarations(), source_cache)?;

    let Some(constructor_group) = manifest.global_group("SymbolConstructor") else {
        bail!("Symbol global value side references missing SymbolConstructor group");
    };
    if !constructor_group.has_role(GlobalDeclarationRole::Type) {
        bail!("SymbolConstructor must have a type-side declaration");
    }
    validate_symbol_constructor_members(constructor_group.declarations(), source_cache)?;

    globals.push(LoweredGlobal {
        name: Text::from("Symbol"),
        id_constant: "SYMBOL_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Class(LoweredClass {
            name: Text::from("Symbol"),
            type_parameters: Box::default(),
            members: Box::new([
                LoweredTypeMember {
                    name: Text::from("dispose"),
                    kind: LoweredMemberKind::NamedStatic,
                    type_reference: LoweredTypeReference::Predefined("GLOBAL_SYMBOL_DISPOSE_ID"),
                },
                LoweredTypeMember {
                    name: Text::from("asyncDispose"),
                    kind: LoweredMemberKind::NamedStatic,
                    type_reference: LoweredTypeReference::Predefined(
                        "GLOBAL_SYMBOL_ASYNC_DISPOSE_ID",
                    ),
                },
            ]),
        }),
    });
    globals.push(LoweredGlobal {
        name: Text::from("Symbol.dispose"),
        id_constant: "SYMBOL_DISPOSE_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Symbol,
    });
    globals.push(LoweredGlobal {
        name: Text::from("Symbol.asyncDispose"),
        id_constant: "SYMBOL_ASYNC_DISPOSE_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Symbol,
    });

    Ok(())
}

fn validate_symbol_declarations(
    records: &[DeclarationRecord],
    source_cache: &mut ParsedSourceCache,
) -> Result<()> {
    for record in records {
        match &record.kind {
            DeclarationKind::Interface => {}
            DeclarationKind::VariableDeclarator { .. } => {
                validate_symbol_constructor_reference(record, source_cache)?;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in the Symbol global")
            }
            DeclarationKind::DeclareFunction | DeclarationKind::ImportEquals => {
                bail!(
                    "unsupported value-side Symbol declaration {:?}",
                    record.kind
                )
            }
        }
    }

    Ok(())
}

fn validate_symbol_constructor_reference(
    record: &DeclarationRecord,
    source_cache: &mut ParsedSourceCache,
) -> Result<()> {
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
        bail!("declare var Symbol is missing a type annotation");
    };
    let AnyTsVariableAnnotation::TsTypeAnnotation(annotation) = annotation else {
        bail!("declare var Symbol uses unsupported definite assignment annotation");
    };
    let AnyTsType::TsReferenceType(reference) = annotation.ty()? else {
        bail!("declare var Symbol must reference SymbolConstructor");
    };
    if reference.type_arguments().is_some() {
        bail!("declare var Symbol must reference SymbolConstructor without type arguments");
    }
    let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = reference
        .name()
        .context("declare var Symbol is missing a type reference name")?
    else {
        bail!("declare var Symbol must reference SymbolConstructor");
    };
    if identifier.value_token()?.token_text_trimmed().text() != "SymbolConstructor" {
        bail!("declare var Symbol must reference SymbolConstructor");
    }

    Ok(())
}

/// Validates the TypeScript `exec` signature before emitting the predefined `RegExp` projection.
fn lower_regexp_globals(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
) -> Result<()> {
    let Some(regexp_group) = manifest.global_group("RegExp") else {
        return Ok(());
    };
    if !regexp_group.has_role(GlobalDeclarationRole::Type) {
        bail!("RegExp global must have a type-side declaration");
    }

    let Some(exec_array_group) = manifest.global_group("RegExpExecArray") else {
        bail!("RegExp.exec references missing RegExpExecArray global");
    };
    if !exec_array_group.has_role(GlobalDeclarationRole::Type) {
        bail!("RegExpExecArray must have a type-side declaration");
    }

    let mut saw_interface = false;
    let mut saw_exec = false;
    for record in regexp_group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => {
                saw_interface = true;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in the RegExp global")
            }
            DeclarationKind::VariableDeclarator { .. } => continue,
            DeclarationKind::DeclareFunction | DeclarationKind::ImportEquals => {
                bail!("unsupported value-side RegExp declaration")
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
        if declaration.extends_clause().is_some() {
            bail!("RegExp interface extends clauses are not supported");
        }
        if declaration.type_parameters().is_some() {
            bail!("RegExp interface type parameters are not supported");
        }

        for member in declaration.members() {
            match member {
                AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
                    if is_regexp_exec_member(method.name()?)? {
                        if saw_exec {
                            bail!("RegExp has multiple exec methods");
                        }
                        validate_regexp_exec_method(&method)?;
                        saw_exec = true;
                    }
                }
                AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                    reject_regexp_exec_non_method(property.name()?)?;
                }
                AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => {
                    reject_regexp_exec_non_method(getter.name()?)?;
                }
                AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => {
                    reject_regexp_exec_non_method(setter.name()?)?;
                }
                AnyTsTypeMember::JsBogusMember(_)
                | AnyTsTypeMember::TsCallSignatureTypeMember(_)
                | AnyTsTypeMember::TsConstructSignatureTypeMember(_)
                | AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {}
            }
        }
    }

    if !saw_interface {
        bail!("RegExp global must include an interface declaration");
    }
    if !saw_exec {
        bail!("RegExp is missing exec");
    }

    globals.push(LoweredGlobal {
        name: Text::from("RegExp"),
        id_constant: "REGEXP_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Class(LoweredClass {
            name: Text::from("RegExp"),
            type_parameters: Box::default(),
            members: Box::new([LoweredTypeMember {
                name: Text::from("exec"),
                kind: LoweredMemberKind::Named { optional: false },
                type_reference: LoweredTypeReference::Predefined("GLOBAL_REGEXP_EXEC_ID"),
            }]),
        }),
    });
    globals.push(LoweredGlobal {
        name: Text::from("RegExp.exec"),
        id_constant: "REGEXP_EXEC_ID_GLOBAL_TYPE_ID",
        data: LoweredTypeData::Function(LoweredFunction {
            is_async: false,
            type_parameters: Box::default(),
            name: Some(Text::from("RegExp.exec")),
            parameters: Box::default(),
            return_type: LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_REGEXP_ID"),
        }),
    });

    Ok(())
}

/// Computed names are excluded from the selected `exec` declaration.
fn is_regexp_exec_member(name: AnyJsObjectMemberName) -> Result<bool> {
    let AnyJsObjectMemberName::JsLiteralMemberName(name) = name else {
        return Ok(false);
    };
    Ok(name.name()?.text() == "exec")
}

/// Prevents a property or accessor named `exec` from being silently ignored.
fn reject_regexp_exec_non_method(name: AnyJsObjectMemberName) -> Result<()> {
    if is_regexp_exec_member(name)? {
        bail!("RegExp.exec must be a method");
    }
    Ok(())
}

/// Accepts a required, non-generic `exec(string): RegExpExecArray | null` declaration.
fn validate_regexp_exec_method(method: &TsMethodSignatureTypeMember) -> Result<()> {
    if method.optional_token().is_some() {
        bail!("RegExp.exec must not be optional");
    }
    if method.type_parameters().is_some() {
        bail!("RegExp.exec must not be generic");
    }
    validate_regexp_exec_parameter(method.parameters()?)?;

    let return_type = method
        .return_type_annotation()
        .context("RegExp.exec is missing a return type")?
        .ty()
        .context("RegExp.exec has a malformed return type")?;
    let AnyTsReturnType::AnyTsType(AnyTsType::TsUnionType(union)) = return_type else {
        bail!("RegExp.exec must return RegExpExecArray | null");
    };

    let mut saw_exec_array = false;
    let mut saw_null = false;
    let mut return_type_variant_count = 0;
    for variant in union.types() {
        let variant = variant.context("RegExp.exec has a malformed return type variant")?;
        return_type_variant_count += 1;
        match variant {
            AnyTsType::TsNullLiteralType(_) => saw_null = true,
            AnyTsType::TsReferenceType(reference) => {
                if reference.type_arguments().is_some() {
                    bail!("RegExp.exec must return RegExpExecArray | null");
                }
                let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = reference
                    .name()
                    .context("RegExp.exec return type is missing a reference name")?
                else {
                    bail!("RegExp.exec must return RegExpExecArray | null");
                };
                if identifier.value_token()?.token_text_trimmed().text() != "RegExpExecArray" {
                    bail!("RegExp.exec must return RegExpExecArray | null");
                }
                saw_exec_array = true;
            }
            _ => bail!("RegExp.exec must return RegExpExecArray | null"),
        }
    }
    if return_type_variant_count != REGEXP_EXEC_RETURN_TYPE_VARIANT_COUNT
        || !saw_exec_array
        || !saw_null
    {
        bail!("RegExp.exec must return RegExpExecArray | null");
    }

    Ok(())
}

/// Requires exactly one non-optional `string` parameter.
fn validate_regexp_exec_parameter(parameters: JsParameters) -> Result<()> {
    let mut parameters = parameters.items().into_iter();
    let Some(parameter) = parameters.next() else {
        bail!("RegExp.exec must have one required string parameter");
    };
    if parameters.next().is_some() {
        bail!("RegExp.exec must have one required string parameter");
    }

    let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(parameter)) =
        parameter.context("RegExp.exec has a malformed parameter")?
    else {
        bail!("RegExp.exec must have one required string parameter");
    };
    if parameter.question_mark_token().is_some() {
        bail!("RegExp.exec must have one required string parameter");
    }
    let type_node = parameter
        .type_annotation()
        .context("RegExp.exec parameter is missing a type annotation")?
        .ty()
        .context("RegExp.exec parameter has a malformed type annotation")?;
    if !matches!(type_node, AnyTsType::TsStringType(_)) {
        bail!("RegExp.exec must have one required string parameter");
    }

    Ok(())
}

fn lower_memberless_class_global(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
    spec: MemberlessClassSpec,
) -> Result<()> {
    let Some(group) = manifest.global_group(spec.name) else {
        return Ok(());
    };
    if !group.has_role(GlobalDeclarationRole::Type) {
        bail!("{} global must have a type-side declaration", spec.name);
    }

    let mut saw_interface = false;
    for record in group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => {
                saw_interface = true;
                let declaration = source_cache
                    .find_interface_declaration(record)?
                    .with_context(|| {
                        format!(
                            "failed to find interface declaration {} at {:?}",
                            record.declared_name.text(),
                            record.text_range
                        )
                    })?;
                validate_memberless_class_interface(&declaration, spec)?;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in the {} global", spec.name)
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {}
        }
    }
    if !saw_interface {
        bail!("{} global must include an interface declaration", spec.name);
    }

    globals.push(LoweredGlobal {
        name: Text::from(spec.name),
        id_constant: spec.id_constant,
        data: LoweredTypeData::Class(LoweredClass {
            name: Text::from(spec.name),
            type_parameters: spec
                .type_parameter_ids
                .iter()
                .map(|id| LoweredTypeReference::Predefined(id))
                .collect(),
            members: Box::default(),
        }),
    });

    Ok(())
}

fn validate_memberless_class_interface(
    declaration: &TsInterfaceDeclaration,
    spec: MemberlessClassSpec,
) -> Result<()> {
    if declaration.extends_clause().is_some() {
        bail!("{} interface extends clauses are not supported", spec.name);
    }

    let mut type_parameter_count = 0;
    if let Some(type_parameters) = declaration.type_parameters() {
        for type_parameter in type_parameters.items() {
            type_parameter.with_context(|| {
                format!("{} interface has a malformed type parameter", spec.name)
            })?;
            type_parameter_count += 1;
        }
    }

    let expected_type_parameter_count = spec.type_parameter_ids.len();
    if type_parameter_count != expected_type_parameter_count {
        bail!(
            "{} interface has {type_parameter_count} type parameters, expected {expected_type_parameter_count}",
            spec.name
        );
    }

    Ok(())
}

/// Validates the two selected `unique symbol` properties across merged constructor interfaces.
fn validate_symbol_constructor_members(
    records: &[DeclarationRecord],
    source_cache: &mut ParsedSourceCache,
) -> Result<()> {
    let mut saw_dispose = false;
    let mut saw_async_dispose = false;

    for record in records {
        match &record.kind {
            DeclarationKind::Interface => {}
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in SymbolConstructor")
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {
                bail!("value-side SymbolConstructor declarations are not supported")
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
        for member in declaration.members() {
            match member {
                AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                    if let Some(member) = selected_symbol_member(property.name()?)? {
                        let saw_member = match member {
                            SelectedSymbolMember::Dispose => &mut saw_dispose,
                            SelectedSymbolMember::AsyncDispose => &mut saw_async_dispose,
                        };
                        if *saw_member {
                            bail!(
                                "SymbolConstructor has multiple {} properties",
                                member.name()
                            );
                        }
                        validate_unique_symbol_property(&property, member.name())?;
                        *saw_member = true;
                    }
                }
                AnyTsTypeMember::TsMethodSignatureTypeMember(member) => {
                    reject_selected_symbol_non_property(member.name()?)?;
                }
                AnyTsTypeMember::TsGetterSignatureTypeMember(member) => {
                    reject_selected_symbol_non_property(member.name()?)?;
                }
                AnyTsTypeMember::TsSetterSignatureTypeMember(member) => {
                    reject_selected_symbol_non_property(member.name()?)?;
                }
                AnyTsTypeMember::JsBogusMember(_)
                | AnyTsTypeMember::TsCallSignatureTypeMember(_)
                | AnyTsTypeMember::TsConstructSignatureTypeMember(_)
                | AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {}
            }
        }
    }

    if !saw_dispose {
        bail!("SymbolConstructor is missing dispose");
    }
    if !saw_async_dispose {
        bail!("SymbolConstructor is missing asyncDispose");
    }

    Ok(())
}

fn reject_selected_symbol_non_property(name: AnyJsObjectMemberName) -> Result<()> {
    if let Some(member) = selected_symbol_member(name)? {
        bail!("SymbolConstructor.{} must be a property", member.name());
    }
    Ok(())
}

fn selected_symbol_member(name: AnyJsObjectMemberName) -> Result<Option<SelectedSymbolMember>> {
    let AnyJsObjectMemberName::JsLiteralMemberName(name) = name else {
        return Ok(None);
    };

    Ok(match name.name()?.text() {
        "dispose" => Some(SelectedSymbolMember::Dispose),
        "asyncDispose" => Some(SelectedSymbolMember::AsyncDispose),
        _ => None,
    })
}

fn validate_unique_symbol_property(
    property: &TsPropertySignatureTypeMember,
    name: &str,
) -> Result<()> {
    if property.optional_token().is_some() {
        bail!("SymbolConstructor.{name} must not be optional");
    }
    let type_node = property
        .type_annotation()
        .with_context(|| format!("SymbolConstructor.{name} is missing a type annotation"))?
        .ty()
        .with_context(|| format!("SymbolConstructor.{name} has a malformed type annotation"))?;
    let AnyTsType::TsTypeOperatorType(operator) = type_node else {
        bail!("SymbolConstructor.{name} must be unique symbol");
    };
    if operator.operator_token()?.kind() != T![unique]
        || !matches!(operator.ty()?, AnyTsType::TsSymbolType(_))
    {
        bail!("SymbolConstructor.{name} must be unique symbol");
    }

    Ok(())
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

/// Lowers a disposable interface into its interface global plus the dispose helper global.
fn lower_disposable_global(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    globals: &mut Vec<LoweredGlobal>,
    spec: DisposableGlobalSpec,
) -> Result<()> {
    let Some(group) = manifest.global_group(spec.interface_name) else {
        return Ok(());
    };
    if !group.has_role(GlobalDeclarationRole::Type) {
        bail!(
            "{} global must have a type-side declaration",
            spec.interface_name
        );
    }

    let mut lowered_member = None;
    let mut saw_interface = false;
    for record in group.declarations() {
        match &record.kind {
            DeclarationKind::Interface => {
                saw_interface = true;
            }
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in {}", spec.interface_name)
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {
                bail!(
                    "value-side {} declarations are not supported",
                    spec.interface_name
                )
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
        if declaration.extends_clause().is_some() {
            bail!("{} extends clauses are not supported", spec.interface_name);
        }
        if declaration.type_parameters().is_some() {
            bail!("{} type parameters are not supported", spec.interface_name);
        }

        for member in declaration.members() {
            let lowered = lower_disposable_type_member(member, spec)?;
            if lowered_member.replace(lowered).is_some() {
                bail!("{} has multiple computed members", spec.interface_name);
            }
        }
    }
    if !saw_interface {
        bail!(
            "{} global must include an interface declaration",
            spec.interface_name
        );
    }

    let lowered_member = lowered_member
        .with_context(|| format!("{} is missing {}", spec.interface_name, spec.member_name))?;

    globals.push(LoweredGlobal {
        name: Text::from(spec.interface_name),
        id_constant: spec.global_id_constant,
        data: LoweredTypeData::Interface(LoweredInterface {
            name: Text::from(spec.interface_name),
            members: Box::new([lowered_member]),
        }),
    });
    globals.push(LoweredGlobal {
        name: Text::from(spec.helper_name),
        id_constant: spec.helper_id_constant,
        data: LoweredTypeData::Function(LoweredFunction {
            is_async: spec.return_kind.helper_is_async(),
            type_parameters: Box::default(),
            name: None,
            parameters: Box::default(),
            return_type: LoweredTypeReference::Predefined(spec.return_kind.return_type_id()),
        }),
    });

    Ok(())
}

/// Lowers the single member of a disposable interface. Only a computed method signature
/// (`[Symbol.dispose](): void`) is supported; every other member shape bails.
fn lower_disposable_type_member(
    member: AnyTsTypeMember,
    spec: DisposableGlobalSpec,
) -> Result<LoweredTypeMember> {
    match member {
        AnyTsTypeMember::TsMethodSignatureTypeMember(member) => {
            lower_disposable_method_signature(&member, spec)
        }
        AnyTsTypeMember::TsPropertySignatureTypeMember(_) => {
            bail!("properties are not supported in {}", spec.interface_name)
        }
        AnyTsTypeMember::TsCallSignatureTypeMember(_)
        | AnyTsTypeMember::TsConstructSignatureTypeMember(_) => {
            bail!("signatures are not supported in {}", spec.interface_name)
        }
        AnyTsTypeMember::JsBogusMember(_) => {
            bail!("bogus members are not supported in {}", spec.interface_name)
        }
        AnyTsTypeMember::TsGetterSignatureTypeMember(_) => {
            bail!(
                "getter signatures are not supported in {}",
                spec.interface_name
            )
        }
        AnyTsTypeMember::TsIndexSignatureTypeMember(_) => {
            bail!(
                "index signatures are not supported in {}",
                spec.interface_name
            )
        }
        AnyTsTypeMember::TsSetterSignatureTypeMember(_) => {
            bail!(
                "setter signatures are not supported in {}",
                spec.interface_name
            )
        }
    }
}

/// Lowers a `[Symbol.(async)Dispose](): <return>` method into a computed-value member whose
/// key is the well-known symbol and whose value type is the dispose helper. Bails on any
/// deviation from that exact shape (optional, generic, parameterized, wrong key, wrong return).
fn lower_disposable_method_signature(
    member: &TsMethodSignatureTypeMember,
    spec: DisposableGlobalSpec,
) -> Result<LoweredTypeMember> {
    if member.optional_token().is_some() {
        bail!("{} must not be optional", spec.member_name);
    }
    if member.type_parameters().is_some() {
        bail!("{} must not be generic", spec.member_name);
    }
    if member.parameters()?.items().into_iter().next().is_some() {
        bail!("{} must not declare parameters", spec.member_name);
    }

    let computed_member = lower_symbol_computed_member_name(member.name()?)?;
    if computed_member.name.text() != spec.member_name {
        bail!(
            "{} has unsupported computed member {}",
            spec.interface_name,
            computed_member.name
        );
    }
    if computed_member.key_reference != LoweredTypeReference::Predefined(spec.symbol_id) {
        bail!(
            "{} has unsupported computed key for {}",
            spec.interface_name,
            spec.member_name
        );
    }

    let return_type = member
        .return_type_annotation()
        .with_context(|| format!("{} is missing a return type", spec.member_name))?
        .ty()
        .with_context(|| format!("{} has malformed return type", spec.member_name))
        .and_then(|return_type_node| lower_disposable_return_type(&return_type_node, spec))?;
    if return_type != LoweredTypeReference::Predefined(spec.return_kind.return_type_id()) {
        bail!("{} has unsupported return type", spec.member_name);
    }

    Ok(LoweredTypeMember {
        name: computed_member.name,
        kind: LoweredMemberKind::ComputedValue {
            key_reference: computed_member.key_reference,
        },
        type_reference: LoweredTypeReference::Predefined(spec.helper_type_id),
    })
}

/// A lowered `[Symbol.<name>]` computed key: its display name and the `GLOBAL_*` symbol reference.
struct ComputedMemberName {
    name: Text,
    key_reference: LoweredTypeReference,
}

/// Lowers a `[Symbol.dispose]` / `[Symbol.asyncDispose]` computed member name into its display
/// name and well-known-symbol key reference. Bails on any non-well-known or non-`Symbol` key.
fn lower_symbol_computed_member_name(name: AnyJsObjectMemberName) -> Result<ComputedMemberName> {
    let AnyJsObjectMemberName::JsComputedMemberName(name) = name else {
        bail!("expected computed symbol member name")
    };
    let AnyJsExpression::JsStaticMemberExpression(expression) = name.expression()? else {
        bail!("computed member name must be a static member expression")
    };
    let AnyJsExpression::JsIdentifierExpression(object) = expression.object()? else {
        bail!("computed member object must be Symbol")
    };
    if object.name()?.value_token()?.token_text_trimmed().text() != "Symbol" {
        bail!("computed member object must be Symbol");
    }

    let AnyJsName::JsName(member_name) = expression.member()? else {
        bail!("computed Symbol member must be a public name")
    };
    let member_name = member_name.value_token()?.token_text_trimmed();
    match member_name.text() {
        "dispose" => Ok(ComputedMemberName {
            name: Text::from("[Symbol.dispose]"),
            key_reference: LoweredTypeReference::Predefined("GLOBAL_SYMBOL_DISPOSE_ID"),
        }),
        "asyncDispose" => Ok(ComputedMemberName {
            name: Text::from("[Symbol.asyncDispose]"),
            key_reference: LoweredTypeReference::Predefined("GLOBAL_SYMBOL_ASYNC_DISPOSE_ID"),
        }),
        name => bail!("unsupported Symbol computed member {name}"),
    }
}

/// Lowers a dispose helper's return type according to `spec.return_kind`: a plain `void`, or
/// the `PromiseLike<void>` special case handled by [`lower_promise_like_void_reference`].
fn lower_disposable_return_type(
    return_type_node: &AnyTsReturnType,
    spec: DisposableGlobalSpec,
) -> Result<LoweredTypeReference> {
    match return_type_node {
        AnyTsReturnType::AnyTsType(type_node) => match spec.return_kind {
            DisposableReturnKind::Void => lower_void_reference(type_node, spec),
            DisposableReturnKind::PromiseLikeVoid => lower_promise_like_void_reference(type_node),
        },
        AnyTsReturnType::TsAssertsReturnType(_) | AnyTsReturnType::TsPredicateReturnType(_) => {
            bail!(
                "predicate return types are not supported in {}",
                spec.member_name
            )
        }
    }
}

/// Lowers the `Disposable` dispose helper's `void` return type to `GLOBAL_VOID_ID`.
fn lower_void_reference(
    type_node: &AnyTsType,
    spec: DisposableGlobalSpec,
) -> Result<LoweredTypeReference> {
    if !matches!(type_node, AnyTsType::TsVoidType(_)) {
        bail!("{} return type must be void", spec.member_name);
    }
    Ok(LoweredTypeReference::Predefined("GLOBAL_VOID_ID"))
}

/// Lowers the `AsyncDisposable` dispose helper's `PromiseLike<void>` return type to
/// `GLOBAL_INSTANCEOF_PROMISE_ID`. This is a deliberate approximation: `PromiseLike` is not a
/// migrated global yet, so the helper resolves to `instanceof Promise` exactly like the previous
/// hand-written data did; the exact-shape check keeps any other return type from being lowered.
fn lower_promise_like_void_reference(type_node: &AnyTsType) -> Result<LoweredTypeReference> {
    let AnyTsType::TsReferenceType(reference) = type_node else {
        bail!("AsyncDisposable return type must be PromiseLike<void>");
    };
    let name = reference
        .name()
        .context("missing AsyncDisposable return type name")?;
    let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = name else {
        bail!("qualified AsyncDisposable return types are not supported");
    };
    if identifier.value_token()?.token_text_trimmed().text() != "PromiseLike" {
        bail!("AsyncDisposable return type must be PromiseLike<void>");
    }

    let type_arguments = reference
        .type_arguments()
        .context("PromiseLike return type is missing type arguments")?;
    let mut arguments = type_arguments.ts_type_argument_list().into_iter();
    let Some(argument) = arguments.next() else {
        bail!("PromiseLike return type is missing void type argument");
    };
    let argument = argument?;
    if arguments.next().is_some() {
        bail!("PromiseLike return type must have one type argument");
    }
    if !matches!(argument, AnyTsType::TsVoidType(_)) {
        bail!("PromiseLike return type must be PromiseLike<void>");
    }

    Ok(LoweredTypeReference::Predefined(
        "GLOBAL_INSTANCEOF_PROMISE_ID",
    ))
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
        match &record.kind {
            DeclarationKind::Interface => {}
            DeclarationKind::TypeAlias => {
                bail!("type aliases are not supported in ErrorConstructor")
            }
            DeclarationKind::DeclareFunction
            | DeclarationKind::VariableDeclarator { .. }
            | DeclarationKind::ImportEquals => {
                bail!("value-side ErrorConstructor declarations are not supported")
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
        if declaration.extends_clause().is_some() {
            bail!("ErrorConstructor extends clauses are not supported");
        }

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
        is_async: false,
        type_parameters: Box::default(),
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
                    binding: LoweredFunctionParameterBinding::Named(name),
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
                    binding: LoweredFunctionParameterBinding::Named(name),
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
