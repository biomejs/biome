//! Lowers collected global declaration groups into a codegen-friendly model.
//!
//! [`ids::GlobalIds`] gives every global declaration group an identity. Each identity is
//! lowered from its declarations by the generic lowerer, except for the few globals whose
//! hand-written projections type inference depends on (see `overrides`).

mod generic;
pub mod ids;
mod overrides;

pub use generic::LoweringGap;

use std::collections::HashMap;

use anyhow::{Context, Result, bail};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFormalParameter, AnyJsName, AnyJsObjectMemberName,
    AnyJsParameter, AnyJsRoot, AnyTsName, AnyTsReturnType, AnyTsType, AnyTsTypeMember,
    AnyTsVariableAnnotation, JsParameters, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator, T,
    TsCallSignatureTypeMember, TsConstructSignatureTypeMember, TsDeclarationModule,
    TsInterfaceDeclaration, TsMethodSignatureTypeMember, TsPropertySignatureTypeMember,
    TsTypeParameters,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, Text, TextRange};

use crate::generate_global_types::{
    collect::{DeclarationKind, DeclarationRecord},
    manifest::{GlobalDeclarationGroup, GlobalDeclarationRole, GlobalManifest},
    source::DiscoveredFile,
};

/// Lowered globals in ID order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredGlobalTypes {
    globals: Box<[LoweredGlobal]>,
    gaps: Box<[LoweringGap]>,
}

impl LoweredGlobalTypes {
    /// Returns all lowered globals in ID order.
    pub fn globals(&self) -> &[LoweredGlobal] {
        &self.globals
    }

    /// Returns one lowered global by name, preferring the one that type annotations
    /// resolve to when a name has separate type and value globals.
    pub fn global(&self, name: &str) -> Option<&LoweredGlobal> {
        self.globals
            .iter()
            .find(|global| global.name() == name && global.roles.type_name)
            .or_else(|| self.globals.iter().find(|global| global.name() == name))
    }

    /// Declaration parts that were lowered to `unknown`.
    pub fn gaps(&self) -> &[LoweringGap] {
        &self.gaps
    }
}

/// One lowered global.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredGlobal {
    name: Text,
    id_constant: Text,
    data: LoweredTypeData,
    local_types: Box<[LoweredTypeData]>,
    roles: GlobalRoles,
}

impl LoweredGlobal {
    /// Supporting types addressed by local references, with dependencies before users.
    pub fn local_types(&self) -> &[LoweredTypeData] {
        &self.local_types
    }

    /// Qualified TypeScript name, or a descriptive name for helper globals.
    pub fn name(&self) -> &str {
        self.name.text()
    }

    /// `GlobalTypeId` constant name.
    pub fn id_constant(&self) -> &str {
        self.id_constant.text()
    }

    /// `RawTypeId` constant name.
    pub fn reference_constant(&self) -> String {
        format!(
            "GLOBAL_{}_ID",
            self.id_constant().trim_end_matches("_ID_GLOBAL_TYPE_ID")
        )
    }

    /// Lowered type data for this global.
    pub fn data(&self) -> &LoweredTypeData {
        &self.data
    }

    /// Names under which type inference can look this global up.
    pub fn roles(&self) -> GlobalRoles {
        self.roles
    }
}

/// Name lookups that resolve to a global.
///
/// Helper globals have neither role; only other globals refer to them.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GlobalRoles {
    /// Type annotations can name this global.
    pub type_name: bool,
    /// Expressions can name this global.
    pub value_name: bool,
}

/// Lowered type data variants supported by the generator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweredTypeData {
    AnyKeyword,
    BigInt,
    BigIntLiteral(Text),
    Boolean,
    BooleanLiteral(bool),
    NeverKeyword,
    Null,
    NumberLiteral(Text),
    ObjectKeyword,
    Object(Box<[LoweredTypeMember]>),
    Class(LoweredClass),
    Constructor(LoweredConstructor),
    Function(LoweredFunction),
    Interface(LoweredInterface),
    Symbol,
    StringLiteral(Text),
    Union(Box<[LoweredTypeReference]>),
    Intersection(Box<[LoweredTypeReference]>),
    Undefined,
    UnknownKeyword,
    ThisKeyword,
    GenericParameter {
        is_const: bool,
        name: Text,
        constraint: Option<LoweredTypeReference>,
        default: Option<LoweredTypeReference>,
    },
    Tuple(Box<[LoweredTupleElement]>),
    Readonly(LoweredTypeReference),
    Keyof(LoweredTypeReference),
    IndexedAccess {
        object: LoweredTypeReference,
        index: LoweredTypeReference,
    },
    InstanceOf {
        ty: LoweredTypeReference,
        type_parameters: Box<[LoweredTypeReference]>,
    },
    /// The referenced type itself, such as the type of a variable.
    Reference(LoweredTypeReference),
}

/// Lowered class-like global.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredClass {
    name: Text,
    type_parameters: Box<[LoweredTypeReference]>,
    extends: Option<LoweredTypeReference>,
    implements: Box<[LoweredTypeReference]>,
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

    /// Class being extended.
    pub fn extends(&self) -> Option<&LoweredTypeReference> {
        self.extends.as_ref()
    }

    /// Interfaces being implemented.
    pub fn implements(&self) -> &[LoweredTypeReference] {
        &self.implements
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
    type_parameters: Box<[LoweredTypeReference]>,
    extends: Box<[LoweredTypeReference]>,
    members: Box<[LoweredTypeMember]>,
}

impl LoweredInterface {
    pub fn type_parameters(&self) -> &[LoweredTypeReference] {
        &self.type_parameters
    }

    /// Base interfaces in declaration order.
    pub fn extends(&self) -> &[LoweredTypeReference] {
        &self.extends
    }

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
    type_parameters: Box<[LoweredTypeReference]>,
    parameters: Box<[LoweredFunctionParameter]>,
    return_type: Option<LoweredTypeReference>,
}

impl LoweredConstructor {
    /// Constructor type parameters in declaration order.
    pub fn type_parameters(&self) -> &[LoweredTypeReference] {
        &self.type_parameters
    }

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

/// One element of a lowered tuple.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredTupleElement {
    ty: LoweredTypeReference,
    name: Option<Text>,
    is_optional: bool,
    is_rest: bool,
}

impl LoweredTupleElement {
    /// Element type.
    pub fn ty(&self) -> &LoweredTypeReference {
        &self.ty
    }

    /// Element label, if the tuple names it.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(Text::text)
    }

    /// Returns whether the element is optional.
    pub fn is_optional(&self) -> bool {
        self.is_optional
    }

    /// Returns whether the element is a rest element.
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
    ComputedStatic { key_reference: LoweredTypeReference },
    IndexSignature { key_reference: LoweredTypeReference },
}

/// Lowered type reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweredTypeReference {
    /// A hand-written `RawTypeId` constant, such as `GLOBAL_STRING_KEYWORD_ID`.
    Predefined(&'static str),
    /// The `RawTypeId` constant of a generated global.
    Global(Text),
    /// An index into the owning global's local type table.
    Local(usize),
}

/// Lowers every global declaration group in the manifest.
pub fn lower_global_types(
    manifest: &GlobalManifest,
    source_files: &[DiscoveredFile],
) -> Result<LoweredGlobalTypes> {
    let mut sources = ParsedSourceCache::new(source_files);
    let ids = ids::GlobalIds::assign(manifest, &mut sources)?;
    let mut lowerer = generic::GenericLowerer::new(manifest, &ids, sources);
    let mut overridden = overrides::lower_overrides(manifest, &ids, &mut lowerer)?;

    let mut globals = Vec::with_capacity(ids.slots().len() + overridden.len());
    for slot in ids.slots() {
        let id_constant = slot.id_constant();
        let global = match overridden
            .iter()
            .position(|global| global.id_constant() == id_constant)
        {
            Some(index) => {
                let mut global = overridden.remove(index);
                global.roles = GlobalRoles {
                    type_name: slot.has_type_role(),
                    value_name: slot.has_value_role(),
                };
                global
            }
            None => lowerer
                .lower_slot(slot)
                .with_context(|| format!("while lowering {}", slot.name()))?,
        };
        globals.push(global);
    }
    // Helper globals referenced by the hand-written projections.
    globals.extend(overridden);

    let mut seen = std::collections::BTreeSet::new();
    for global in &globals {
        if !seen.insert(global.id_constant()) {
            bail!(
                "duplicate global type ID {} for {}",
                global.id_constant(),
                global.name()
            );
        }
    }

    Ok(LoweredGlobalTypes {
        globals: globals.into_boxed_slice(),
        gaps: lowerer.into_gaps().into_boxed_slice(),
    })
}

/// Parsed declaration files, indexed so collector records can find their nodes.
struct ParsedSourceCache<'a> {
    source_files: &'a [DiscoveredFile],
    parsed: HashMap<&'a str, ParsedSource>,
}

struct ParsedSource {
    module: TsDeclarationModule,
    /// Nodes of each requested kind, keyed by their trimmed range.
    nodes: HashMap<JsSyntaxKind, HashMap<TextRange, JsSyntaxNode>>,
}

impl<'a> ParsedSourceCache<'a> {
    fn new(source_files: &'a [DiscoveredFile]) -> Self {
        Self {
            source_files,
            parsed: HashMap::new(),
        }
    }

    fn parsed_source(&mut self, record: &DeclarationRecord) -> Result<&mut ParsedSource> {
        let repo_relative = record.file_repo_relative.as_ref();
        let source_file = self
            .source_files
            .iter()
            .find(|source_file| source_file.repo_relative == repo_relative)
            .with_context(|| {
                format!("collector record references missing source file {repo_relative}")
            })?;
        let key = source_file.repo_relative.as_str();
        if !self.parsed.contains_key(key) {
            let source = std::str::from_utf8(&source_file.bytes)
                .with_context(|| format!("{} is not valid UTF-8", source_file.repo_relative))?;
            let parsed = parse(source, JsFileSource::d_ts(), JsParserOptions::default());
            if parsed.has_errors() {
                bail!("parser diagnostics in {}", source_file.repo_relative);
            }
            let AnyJsRoot::TsDeclarationModule(module) = parsed.tree() else {
                bail!(
                    "{} is not a TypeScript declaration module",
                    source_file.repo_relative
                );
            };
            self.parsed.insert(
                key,
                ParsedSource {
                    module,
                    nodes: HashMap::new(),
                },
            );
        }
        Ok(self
            .parsed
            .get_mut(key)
            .expect("parsed source was inserted above"))
    }

    /// Finds the declaration node behind a collector record.
    fn find_node(&mut self, record: &DeclarationRecord) -> Result<Option<JsSyntaxNode>> {
        let source = self.parsed_source(record)?;
        let module = source.module.syntax().clone();
        let nodes = source.nodes.entry(record.syntax_kind).or_insert_with(|| {
            module
                .descendants()
                .filter(|node| node.kind() == record.syntax_kind)
                .map(|node| (node.text_trimmed_range(), node))
                .collect()
        });
        Ok(nodes.get(&record.text_range).cloned())
    }

    /// Finds the AST node for a collected interface record.
    fn find_interface_declaration(
        &mut self,
        record: &DeclarationRecord,
    ) -> Result<Option<TsInterfaceDeclaration>> {
        Ok(self
            .find_node(record)?
            .and_then(TsInterfaceDeclaration::cast))
    }

    /// Finds the AST node for a collected variable declarator.
    fn find_variable_declarator(
        &mut self,
        record: &DeclarationRecord,
    ) -> Result<Option<JsVariableDeclarator>> {
        Ok(self.find_node(record)?.and_then(JsVariableDeclarator::cast))
    }
}

fn is_unique_symbol_property(property: &TsPropertySignatureTypeMember) -> Result<bool> {
    let Some(annotation) = property.type_annotation() else {
        return Ok(false);
    };
    let AnyTsType::TsTypeOperatorType(operator) = annotation.ty()? else {
        return Ok(false);
    };
    Ok(operator.operator_token()?.kind() == T![unique]
        && matches!(operator.ty()?, AnyTsType::TsSymbolType(_)))
}

/// Extracts a simple identifier binding name.
fn lower_binding_name(binding: AnyJsBindingPattern) -> Result<Text> {
    let Some(binding) = binding.as_any_js_binding() else {
        bail!("unsupported destructured function parameter");
    };
    let Some(binding) = binding.as_js_identifier_binding() else {
        bail!("unsupported function parameter binding");
    };
    Ok(Text::from(binding.name_token()?.token_text_trimmed()))
}

/// Extracts a supported object member name.
fn lower_object_member_name(name: AnyJsObjectMemberName) -> Result<Text> {
    match name {
        AnyJsObjectMemberName::JsLiteralMemberName(name) => Ok(Text::from(name.name()?)),
        AnyJsObjectMemberName::JsComputedMemberName(_)
        | AnyJsObjectMemberName::JsMetavariable(_) => {
            bail!("unsupported computed or metavariable member name")
        }
    }
}

fn lower_primitive_reference(type_node: &AnyTsType) -> Option<LoweredTypeReference> {
    let id = match type_node {
        AnyTsType::TsStringType(_) => "GLOBAL_STRING_KEYWORD_ID",
        AnyTsType::TsNumberType(_) => "GLOBAL_NUMBER_KEYWORD_ID",
        AnyTsType::TsVoidType(_) => "GLOBAL_VOID_ID",
        _ => return None,
    };
    Some(LoweredTypeReference::Predefined(id))
}
