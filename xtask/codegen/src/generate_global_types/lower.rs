//! Lowers collected global declaration groups into a codegen-friendly model.

use anyhow::{Context, Result, bail};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFormalParameter, AnyJsName, AnyJsObjectMemberName,
    AnyJsParameter, AnyJsRoot, AnyTsName, AnyTsReturnType, AnyTsType, AnyTsTypeMember,
    AnyTsVariableAnnotation, JsParameters, JsVariableDeclarator, TsCallSignatureTypeMember,
    TsConstructSignatureTypeMember, TsDeclarationModule, TsInterfaceDeclaration,
    TsMethodSignatureTypeMember, TsPropertySignatureTypeMember, TsTypeParameters,
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
    Interface(LoweredInterface),
}

/// Lowered class-like global.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredClass {
    name: Text,
    type_parameters: Box<[Text]>,
    members: Box<[LoweredTypeMember]>,
}

impl LoweredClass {
    /// Class name.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// Class type parameters in declaration order.
    pub fn type_parameters(&self) -> &[Text] {
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
    type_parameters: Box<[Text]>,
    members: Box<[LoweredTypeMember]>,
}

impl LoweredInterface {
    /// Interface name.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// Interface type parameters in declaration order.
    pub fn type_parameters(&self) -> &[Text] {
        &self.type_parameters
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
    name: Option<Text>,
    parameters: Box<[LoweredFunctionParameter]>,
    return_type: LoweredTypeReference,
}

impl LoweredFunction {
    /// Returns whether this function is `async`.
    pub fn is_async(&self) -> bool {
        self.is_async
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

    let mut globals = Vec::new();
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

    if let Some(disposable_globals) =
        lower_disposable_global(manifest, &mut source_cache, DISPOSABLE_GLOBAL)?
    {
        globals.extend(disposable_globals);
    }
    if let Some(async_disposable_globals) =
        lower_disposable_global(manifest, &mut source_cache, ASYNC_DISPOSABLE_GLOBAL)?
    {
        globals.extend(async_disposable_globals);
    }

    globals.push(lower_generic_interface(
        manifest,
        &mut source_cache,
        "Array",
        "ARRAY_ID_GLOBAL_TYPE_ID",
        true,
    )?);
    globals.push(lower_generic_interface(
        manifest,
        &mut source_cache,
        "Promise",
        "PROMISE_ID_GLOBAL_TYPE_ID",
        true,
    )?);
    globals.push(lower_generic_interface(
        manifest,
        &mut source_cache,
        "Map",
        "MAP_ID_GLOBAL_TYPE_ID",
        true,
    )?);
    globals.push(lower_generic_interface(
        manifest,
        &mut source_cache,
        "Set",
        "SET_ID_GLOBAL_TYPE_ID",
        true,
    )?);

    Ok(LoweredGlobalTypes {
        globals: globals.into_boxed_slice(),
    })
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

/// Lowers a disposable interface into its interface global plus the dispose helper global.
/// Returns `None` when the manifest has no group for `spec.interface_name`.
fn lower_disposable_global(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    spec: DisposableGlobalSpec,
) -> Result<Option<[LoweredGlobal; 2]>> {
    let Some(group) = manifest.global_group(spec.interface_name) else {
        return Ok(None);
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

    Ok(Some([
        LoweredGlobal {
            name: Text::from(spec.interface_name),
            id_constant: spec.global_id_constant,
            data: LoweredTypeData::Interface(LoweredInterface {
                name: Text::from(spec.interface_name),
                type_parameters: Box::default(),
                members: Box::new([lowered_member]),
            }),
        },
        LoweredGlobal {
            name: Text::from(spec.helper_name),
            id_constant: spec.helper_id_constant,
            data: LoweredTypeData::Function(LoweredFunction {
                is_async: spec.return_kind.helper_is_async(),
                name: None,
                parameters: Box::default(),
                return_type: LoweredTypeReference::Predefined(spec.return_kind.return_type_id()),
            }),
        },
    ]))
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
                .and_then(|type_node| lower_type_reference(&type_node, &[]))?;
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
        let type_reference = lower_type_reference(&annotation.ty()?, &[])?;
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
        .and_then(|type_node| lower_type_reference(&type_node, &[]))
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
        .and_then(|type_node| lower_type_reference(&type_node, &[]))
        .map(class_return_reference)?;

    Ok(LoweredConstructor {
        parameters: lower_parameters(member.parameters()?, &[])?,
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
        .and_then(|return_type_node| lower_return_type_reference(&return_type_node, &[]))?;

    Ok(LoweredFunction {
        is_async: false,
        name: Some(Text::from("Error")),
        parameters: lower_parameters(member.parameters()?, &[])?,
        return_type: instance_return_reference(return_type),
    })
}

/// Lowers function-like parameters.
fn lower_parameters(
    parameters: JsParameters,
    type_parameters: &[Text],
) -> Result<Box<[LoweredFunctionParameter]>> {
    let mut lowered = Vec::new();
    for parameter in parameters.items() {
        match parameter? {
            AnyJsParameter::AnyJsFormalParameter(parameter) => {
                let AnyJsFormalParameter::JsFormalParameter(parameter) = parameter else {
                    bail!("unsupported formal parameter");
                };
                let name = lower_binding_name(parameter.binding()?)?;
                let is_optional = parameter.question_mark_token().is_some();
                let type_reference = parameter
                    .type_annotation()
                    .context("parameter is missing a type annotation")?
                    .ty()
                    .context("parameter has malformed type annotation")
                    .and_then(|type_node| lower_type_reference(&type_node, type_parameters))?;
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
                    .context("rest parameter is missing a type annotation")?
                    .ty()
                    .context("rest parameter has malformed type annotation")
                    .and_then(|type_node| lower_type_reference(&type_node, type_parameters))?;
                lowered.push(LoweredFunctionParameter {
                    name,
                    type_reference,
                    is_optional: false,
                    is_rest: true,
                });
            }
            AnyJsParameter::TsThisParameter(_) => {
                // Skip 'this' parameters as Biome doesn't use them for global types yet
            }
        }
    }

    Ok(lowered.into_boxed_slice())
}

/// Lowers a generic interface.
fn lower_generic_interface(
    manifest: &GlobalManifest,
    source_cache: &mut ParsedSourceCache,
    interface_name: &'static str,
    global_id_constant: &'static str,
    as_class: bool,
) -> Result<LoweredGlobal> {
    let group = manifest
        .global_group(interface_name)
        .with_context(|| format!("missing global group for {interface_name}"))?;

    let mut members = Vec::new();
    let mut type_parameters: Box<[Text]> = Box::default();

    for record in group.declarations() {
        match record.kind {
            DeclarationKind::Interface => {
                let declaration = source_cache
                    .find_interface_declaration(record)?
                    .with_context(|| {
                        format!("missing interface declaration for {interface_name}")
                    })?;

                if let Some(params) = declaration.type_parameters() {
                    type_parameters = lower_type_parameters_decl(params)?;
                }

                for member in declaration.members() {
                    if let Some(lowered) =
                        lower_generic_member(member, interface_name, false, &type_parameters)?
                    {
                        members.push(lowered);
                    }
                }
            }
            DeclarationKind::VariableDeclarator { .. } if as_class => {
                let declarator = source_cache
                    .find_variable_declarator(record)?
                    .with_context(|| format!("missing variable declarator for {interface_name}"))?;

                if let Some(AnyTsVariableAnnotation::TsTypeAnnotation(annotation)) =
                    declarator.variable_annotation()
                    && let Ok(AnyTsType::TsReferenceType(reference)) = annotation.ty()
                    && let Ok(AnyTsName::JsReferenceIdentifier(ident)) = reference.name()
                {
                    let constructor_name = ident.value_token()?.token_text_trimmed();
                    let constructor_group = manifest.global_group(&constructor_name).with_context(|| {
                        format!("missing constructor group {constructor_name} for {interface_name}")
                    })?;

                    for constructor_record in constructor_group.declarations() {
                        if constructor_record.kind == DeclarationKind::Interface {
                            let constructor_decl = source_cache.find_interface_declaration(constructor_record)?.with_context(|| {
                                format!("missing constructor interface declaration for {constructor_name}")
                            })?;

                            for member in constructor_decl.members() {
                                if let Some(mut lowered) = lower_generic_member(
                                    member,
                                    interface_name,
                                    true,
                                    &[],
                                )? {
                                    // Static members on the value side are NamedStatic on the class side.
                                    // We skip members that are already defined as instance members to avoid collisions,
                                    // although in TS they usually don't collide.
                                    if members
                                        .iter()
                                        .all(|m| m.name.text() != lowered.name.text())
                                    {
                                        lowered.kind = LoweredMemberKind::NamedStatic;
                                        members.push(lowered);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if as_class {
        members.push(LoweredTypeMember {
            name: Text::from("constructor"),
            kind: LoweredMemberKind::Constructor,
            type_reference: LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID"),
        });
    }

    let data = if as_class {
        LoweredTypeData::Class(LoweredClass {
            name: Text::from(interface_name),
            type_parameters: type_parameters.clone(),
            members: members.into_boxed_slice(),
        })
    } else {
        LoweredTypeData::Interface(LoweredInterface {
            name: Text::from(interface_name),
            type_parameters,
            members: members.into_boxed_slice(),
        })
    };

    Ok(LoweredGlobal {
        name: Text::from(interface_name),
        id_constant: global_id_constant,
        data,
    })
}

/// Lowers a generic interface member.
fn lower_generic_member(
    member: AnyTsTypeMember,
    interface_name: &str,
    is_static: bool,
    type_parameters: &[Text],
) -> Result<Option<LoweredTypeMember>> {
    match member {
        AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
            let Ok(name) = lower_object_member_name(property.name()?) else {
                return Ok(None);
            };

            if let Some(predefined) =
                lookup_predefined_member_id(interface_name, name.text(), is_static)
            {
                return Ok(Some(LoweredTypeMember {
                    name,
                    kind: if is_static {
                        LoweredMemberKind::NamedStatic
                    } else {
                        LoweredMemberKind::Named {
                            optional: property.optional_token().is_some(),
                        }
                    },
                    type_reference: LoweredTypeReference::Predefined(predefined),
                }));
            }

            let optional = property.optional_token().is_some();
            let type_reference = property
                .type_annotation()
                .context("missing type annotation")?
                .ty()
                .context("malformed type annotation")
                .and_then(|ty| lower_type_reference(&ty, type_parameters))?;

            Ok(Some(LoweredTypeMember {
                name,
                kind: if is_static {
                    LoweredMemberKind::NamedStatic
                } else {
                    LoweredMemberKind::Named { optional }
                },
                type_reference,
            }))
        }
        AnyTsTypeMember::TsMethodSignatureTypeMember(method) => {
            let Ok(name) = lower_object_member_name(method.name()?) else {
                return Ok(None);
            };

            if let Some(predefined) =
                lookup_predefined_member_id(interface_name, name.text(), is_static)
            {
                return Ok(Some(LoweredTypeMember {
                    name,
                    kind: if is_static {
                        LoweredMemberKind::NamedStatic
                    } else {
                        LoweredMemberKind::Named {
                            optional: method.optional_token().is_some(),
                        }
                    },
                    type_reference: LoweredTypeReference::Predefined(predefined),
                }));
            }

            let optional = method.optional_token().is_some();
            let type_reference = method
                .return_type_annotation()
                .context("missing return type")?
                .ty()
                .context("malformed return type")
                .and_then(|ty| lower_return_type_reference(&ty, type_parameters))?;

            Ok(Some(LoweredTypeMember {
                name,
                kind: if is_static {
                    LoweredMemberKind::NamedStatic
                } else {
                    LoweredMemberKind::Named { optional }
                },
                type_reference,
            }))
        }
        _ => Ok(None),
    }
}

/// Extracts type parameter names from a declaration.
fn lower_type_parameters_decl(params: TsTypeParameters) -> Result<Box<[Text]>> {
    let mut lowered = Vec::new();
    for param in params.items() {
        let param = param?;
        let name = Text::from(param.name()?.ident_token()?.token_text_trimmed());
        lowered.push(name);
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
fn lower_type_reference(
    type_node: &AnyTsType,
    _type_parameters: &[Text],
) -> Result<LoweredTypeReference> {
    match type_node {
        AnyTsType::TsStringType(_) => Ok(LoweredTypeReference::Predefined("GLOBAL_STRING_ID")),
        AnyTsType::TsNumberType(_) => Ok(LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID")),
        AnyTsType::TsBooleanType(_) => Ok(LoweredTypeReference::Predefined("GLOBAL_BOOLEAN_ID")),
        AnyTsType::TsVoidType(_) => Ok(LoweredTypeReference::Predefined("GLOBAL_VOID_ID")),
        AnyTsType::TsAnyType(_) | AnyTsType::TsUnknownType(_) => {
            Ok(LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID"))
        }
        AnyTsType::TsUnionType(_)
        | AnyTsType::TsIntersectionType(_)
        | AnyTsType::TsObjectType(_)
        | AnyTsType::TsTupleType(_)
        | AnyTsType::TsThisType(_) => Ok(LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID")),
        AnyTsType::TsArrayType(_) => Ok(LoweredTypeReference::Predefined(
            "GLOBAL_INSTANCEOF_ARRAY_T_ID",
        )),
        AnyTsType::TsReferenceType(reference) => {
            let name = reference.name().context("missing type reference name")?;
            let biome_js_syntax::AnyTsName::JsReferenceIdentifier(identifier) = name else {
                bail!("qualified type references are not supported")
            };
            let name_text = Text::from(identifier.value_token()?.token_text_trimmed());
            Ok(match name_text.text() {
                "Array" => LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ARRAY_T_ID"),
                "Promise" => LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_PROMISE_ID"),
                "Map" => LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_MAP_ID"),
                "Set" => LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_SET_ID"),
                "Error" => LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ERROR_ID"),
                "ErrorConstructor" => {
                    LoweredTypeReference::Predefined("GLOBAL_ERROR_CONSTRUCTOR_ID")
                }
                "RegExp" => LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_REGEXP_ID"),
                "Symbol" => LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_SYMBOL_ID"),
                "Boolean" => LoweredTypeReference::Predefined("GLOBAL_BOOLEAN_ID"),
                "Number" => LoweredTypeReference::Predefined("GLOBAL_NUMBER_ID"),
                "String" => LoweredTypeReference::Predefined("GLOBAL_STRING_ID"),
                "T" => LoweredTypeReference::Predefined("GLOBAL_T_ID"),
                "U" => LoweredTypeReference::Predefined("GLOBAL_U_ID"),
                _ => LoweredTypeReference::Predefined("GLOBAL_UNKNOWN_ID"),
            })
        }
        _ => bail!("unsupported type reference: {type_node:?}"),
    }
}

/// Maps a supported return type node to a lowered reference.
fn lower_return_type_reference(
    return_type_node: &AnyTsReturnType,
    type_parameters: &[Text],
) -> Result<LoweredTypeReference> {
    match return_type_node {
        AnyTsReturnType::AnyTsType(type_node) => lower_type_reference(type_node, type_parameters),
        AnyTsReturnType::TsAssertsReturnType(_) | AnyTsReturnType::TsPredicateReturnType(_) => {
            Ok(LoweredTypeReference::Predefined("GLOBAL_BOOLEAN_ID"))
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

/// Converts constructor returns from `InstanceOf<Error>` to `Error` class.
fn class_return_reference(reference: LoweredTypeReference) -> LoweredTypeReference {
    match reference {
        LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ARRAY_T_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_ARRAY_ID")
        }
        LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_PROMISE_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_PROMISE_ID")
        }
        LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_MAP_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_MAP_ID")
        }
        LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_SET_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_SET_ID")
        }
        LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_ERROR_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_ERROR_ID")
        }
        LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_REGEXP_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_REGEXP_ID")
        }
        LoweredTypeReference::Predefined("GLOBAL_INSTANCEOF_SYMBOL_ID") => {
            LoweredTypeReference::Predefined("GLOBAL_SYMBOL_ID")
        }
        reference => reference,
    }
}

/// Returns the predefined global ID for a well-known member if available.
fn lookup_predefined_member_id(
    interface_name: &str,
    member_name: &str,
    is_static: bool,
) -> Option<&'static str> {
    match (interface_name, member_name, is_static) {
        ("Promise", "resolve", true) => Some("GLOBAL_PROMISE_RESOLVE_ID"),
        ("Promise", "all", true) => Some("GLOBAL_PROMISE_ALL_ID"),
        ("Promise", "allSettled", true) => Some("GLOBAL_PROMISE_ALL_SETTLED_ID"),
        ("Promise", "any", true) => Some("GLOBAL_PROMISE_ANY_ID"),
        ("Promise", "race", true) => Some("GLOBAL_PROMISE_RACE_ID"),
        ("Promise", "reject", true) => Some("GLOBAL_PROMISE_REJECT_ID"),
        ("Promise", "then", false) => Some("GLOBAL_PROMISE_THEN_ID"),
        ("Promise", "catch", false) => Some("GLOBAL_PROMISE_CATCH_ID"),
        ("Promise", "finally", false) => Some("GLOBAL_PROMISE_FINALLY_ID"),
        ("Array", "filter", false) => Some("GLOBAL_ARRAY_FILTER_ID"),
        ("Array", "forEach", false) => Some("GLOBAL_ARRAY_FOREACH_ID"),
        ("Array", "map", false) => Some("GLOBAL_ARRAY_MAP_ID"),
        ("Symbol", "dispose", true) => Some("GLOBAL_SYMBOL_DISPOSE_ID"),
        ("Symbol", "asyncDispose", true) => Some("GLOBAL_SYMBOL_ASYNC_DISPOSE_ID"),
        ("RegExp", "exec", false) => Some("GLOBAL_REGEXP_EXEC_ID"),
        _ => None,
    }
}
