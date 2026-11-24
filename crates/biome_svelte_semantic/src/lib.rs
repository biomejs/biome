//! Cross-snippet resolution for scripts in a Svelte component. JavaScript
//! semantics resolve lexical references within each snippet first.

mod template;
pub use template::TemplateBindings;

use biome_js_semantic::{JsDeclarationKind, SemanticFlavor, SemanticModel, identifier_name};
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{
    AnyJsIdentifierReference, JsAssignmentExpression, JsAssignmentOperator, JsSyntaxKind,
    unescape_js_identifier,
};
use biome_languages::JsFileSource;
use biome_languages::javascript::{JsEmbeddingKind, SvelteEmbeddingKind};
use biome_rowan::{AstNode, Text, TextRange, TextSize, TokenText};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scope {
    Module,
    Instance,
    Template,
    IsolatedScript,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReferenceKind {
    Value,
    Type,
    Qualifier,
    TypeQuery,
}

impl ReferenceKind {
    fn from_reference(reference: &AnyJsIdentifierReference) -> Self {
        if !reference.is_only_type() {
            return Self::Value;
        }
        let syntax = reference.syntax();
        if syntax
            .ancestors()
            .skip(1)
            .find(|node| node.kind() != JsSyntaxKind::TS_QUALIFIED_NAME)
            .is_some_and(|outer| outer.kind() == JsSyntaxKind::TS_TYPEOF_TYPE)
        {
            Self::TypeQuery
        } else if syntax
            .parent()
            .is_some_and(|node| node.kind() == JsSyntaxKind::TS_QUALIFIED_NAME)
        {
            Self::Qualifier
        } else {
            Self::Type
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Declarations(SmallVec<[(TextRange, JsDeclarationKind); 1]>);

impl Declarations {
    fn insert(&mut self, range: TextRange, kind: JsDeclarationKind) {
        self.0.push((range, kind));
    }

    fn get(
        &self,
        kind: ReferenceKind,
    ) -> Option<(TextRange, impl Iterator<Item = TextRange> + '_)> {
        let kind = if kind == ReferenceKind::TypeQuery
            && self.0.iter().any(|(_, decl)| decl.declares_value())
        {
            ReferenceKind::Value
        } else {
            kind
        };
        let mut ranges = self.0.iter().filter_map(move |&(range, decl)| {
            let contributes = match kind {
                ReferenceKind::Value => decl.declares_value(),
                ReferenceKind::Type => decl.declares_type(),
                ReferenceKind::Qualifier => {
                    decl.declares_namespace()
                        || decl.is_import_declaration()
                        || decl == JsDeclarationKind::Enum
                }
                ReferenceKind::TypeQuery => decl.is_import_type_declaration(),
            };
            contributes.then_some(range)
        });
        Some((ranges.next()?, ranges))
    }
}

/// A JavaScript snippet and its start offset in the authored component.
pub struct SvelteSnippet {
    pub offset: TextSize,
    pub source: JsFileSource,
    pub js: SemanticModel,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SvelteRootSnippet {
    pub range: TextRange,
    pub name: Text,
    pub declaration: TextRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvelteEachKeyStatus {
    Keyed,
    Unkeyed,
    Unknown,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct SvelteTemplateNames {
    pub unknown: UnknownContent,
    pub generics: FxHashMap<Text, TextRange>,
    pattern_reads: Vec<Reference>,
    pub bindings: TemplateBindings,
    pub each_keys: FxHashMap<TextRange, SvelteEachKeyStatus>,
    bind_targets: FxHashMap<TextRange, Access>,
    pub root_snippets: Vec<SvelteRootSnippet>,
}

/// Semantic information missing from opaque regions of the component.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct UnknownContent {
    pub module_script: bool,
    pub instance_script: bool,
    pub generics: bool,
    pub root_snippets: bool,
    pub unreadable_snippets: FxHashSet<TextSize>,
    pub template_reads: Vec<TextSize>,
    pub template_declarations: Vec<TextRange>,
}

impl SvelteTemplateNames {
    pub fn register_bind_target(&mut self, range: TextRange, write_only: bool) {
        self.bind_targets.insert(
            range,
            if write_only {
                Access::Write
            } else {
                Access::ReadWrite
            },
        );
    }

    pub fn collect_pattern_reads(&mut self, js: &SemanticModel, offset: TextSize) {
        self.pattern_reads.extend(unresolved_references(js, offset));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Access {
    Read,
    Write,
    ReadWrite,
}

impl Access {
    fn from_reference(reference: &AnyJsIdentifierReference) -> Self {
        if !matches!(
            reference,
            AnyJsIdentifierReference::JsIdentifierAssignment(_)
        ) {
            return Self::Read;
        }
        let parent = reference.syntax().ancestors().skip(1).find(|node| {
            !matches!(
                node.kind(),
                JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT
                    | JsSyntaxKind::TS_AS_ASSIGNMENT
                    | JsSyntaxKind::TS_SATISFIES_ASSIGNMENT
                    | JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT
                    | JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT
            )
        });
        if parent.is_some_and(|node| {
            matches!(
                node.kind(),
                JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
            ) || JsAssignmentExpression::cast(node).is_some_and(|assignment| {
                assignment
                    .operator()
                    .is_ok_and(|operator| operator != JsAssignmentOperator::Assign)
            })
        }) {
            Self::ReadWrite
        } else {
            Self::Write
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Reference {
    range: TextRange,
    name: TokenText,
    kind: ReferenceKind,
    access: Access,
}

fn unresolved_references(
    js: &SemanticModel,
    offset: TextSize,
) -> impl Iterator<Item = Reference> + '_ {
    js.all_unresolved_references().filter_map(move |reference| {
        let tree = reference.tree();
        let token = tree.value_token().ok()?;
        Some(Reference {
            range: token.text_trimmed_range() + offset,
            name: token.token_text_trimmed(),
            kind: ReferenceKind::from_reference(&tree),
            access: Access::from_reference(&tree),
        })
    })
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BindingUses {
    pub value_read: bool,
    pub write: bool,
    pub type_read: bool,
}

impl BindingUses {
    fn record(&mut self, kind: ReferenceKind, access: Access) {
        if kind == ReferenceKind::Value {
            self.value_read |= access != Access::Write;
            self.write |= access != Access::Read;
        } else {
            self.type_read = true;
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct SvelteSemanticModel {
    scopes: FxHashMap<TextSize, Scope>,
    module: FxHashMap<Text, Declarations>,
    instance: FxHashMap<Text, Declarations>,
    instance_imports: FxHashMap<Text, Declarations>,
    uses: FxHashMap<TextRange, BindingUses>,
    unknown: UnknownContent,
    unknown_uses: FxHashSet<TextRange>,
    resolution: FxHashMap<TextRange, Resolution>,
    declaration_kinds: FxHashMap<TextRange, JsDeclarationKind>,
    const_writes: FxHashSet<TextRange>,
    generics: FxHashMap<Text, TextRange>,
    template_bindings: TemplateBindings,
    each_keys: FxHashMap<TextRange, SvelteEachKeyStatus>,
    instance_snippets: FxHashMap<Text, TextRange>,
    module_snippets: FxHashSet<Text>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    Resolved(TextRange),
    Unresolved,
    Unknown,
}

impl SvelteSemanticModel {
    pub fn new(
        snippets: &[SvelteSnippet],
        template_reads: impl Iterator<Item = (TextRange, TokenText)>,
        mut template: SvelteTemplateNames,
    ) -> Self {
        template.bindings.index();
        template.unknown.template_reads.sort_unstable();
        let mut roots = template.root_snippets;
        roots.sort_unstable_by_key(|root| root.range.start());
        let mut model = Self {
            instance_snippets: roots
                .iter()
                .map(|root| (root.name.clone(), root.declaration))
                .collect(),
            unknown: template.unknown,
            generics: template.generics,
            template_bindings: template.bindings,
            each_keys: template.each_keys,
            ..Self::default()
        };
        let mut constants = FxHashSet::default();
        for snippet in snippets {
            model.collect_script_declarations(snippet, &mut constants);
        }

        let js_references = snippets.iter().flat_map(|snippet| {
            let scope = model.scopes[&snippet.offset];
            unresolved_references(&snippet.js, snippet.offset)
                .map(move |reference| (scope, reference, Some(snippet.offset)))
        });
        let host_references = template_reads
            .map(|(range, name)| Reference {
                range,
                name,
                kind: ReferenceKind::Value,
                access: Access::Read,
            })
            .chain(template.pattern_reads)
            .map(|reference| (Scope::Template, reference, None));
        let mut captured = Vec::new();
        let mut uncertain_captures: Vec<_> = roots
            .iter()
            .filter(|root| {
                let first = model
                    .unknown
                    .template_reads
                    .partition_point(|&p| p < root.range.start());
                model
                    .unknown
                    .template_reads
                    .get(first)
                    .is_some_and(|&p| root.range.contains(p))
            })
            .map(|root| root.name.clone())
            .collect();
        let mut callers = FxHashMap::<_, Vec<_>>::default();
        let mut missing = Vec::new();
        let mut uses = FxHashMap::<_, BindingUses>::default();
        let mut resolution = FxHashMap::default();
        let mut unknown_uses = model.possible_uses_in_unknown_content();
        let mut const_writes = FxHashSet::default();
        for (
            scope,
            Reference {
                range,
                name,
                kind,
                access,
            },
            offset,
        ) in js_references.chain(host_references)
        {
            let access = template.bind_targets.get(&range).copied().unwrap_or(access);
            let name = identifier_name(name);
            let owner = if scope == Scope::Template {
                let end = roots.partition_point(|root| root.range.start() <= range.start());
                roots[..end]
                    .last()
                    .filter(|root| root.range.contains_range(range))
            } else {
                None
            };
            if offset.is_some_and(|offset| model.unknown.unreadable_snippets.contains(&offset)) {
                resolution.insert(range, Resolution::Unknown);
                continue;
            }
            let unknown_scope = (scope == Scope::Template)
                .then(|| model.unknown_template_scope(range.start()))
                .flatten();
            if scope == Scope::Template
                && let Some((local, visibility, constant)) =
                    model.template_lookup(&name, range.start(), kind)
            {
                if unknown_scope.is_some_and(|scope| visibility.contains_range(scope)) {
                    if let Some(owner) = owner {
                        uncertain_captures.push(owner.name.clone());
                    }
                    unknown_uses.insert(local);
                    resolution.insert(range, Resolution::Unknown);
                    continue;
                }
                uses.entry(local).or_default().record(kind, access);
                if constant && kind == ReferenceKind::Value && access != Access::Read {
                    const_writes.insert(range);
                }
                resolution.insert(range, Resolution::Resolved(local));
                if kind == ReferenceKind::Value
                    && let Some(owner) = owner
                {
                    if model.instance_snippets.get(&name) == Some(&local) {
                        callers.entry(name).or_default().push(owner.name.clone());
                    } else if !owner.range.contains_range(local) {
                        captured.push(owner.name.clone());
                    }
                }
                continue;
            }
            let direct = model.lookup(scope, &name, kind);
            let is_store = direct.is_none();
            let store = (is_store && kind == ReferenceKind::Value)
                .then(|| SemanticFlavor::Svelte.store_reference_name(&name))
                .flatten();
            let uncertain = unknown_scope.is_some()
                || model.lookup_is_uncertain(scope, &name, kind)
                || store.is_some_and(|store| model.lookup_is_uncertain(scope, store, kind));
            let declaration = direct.or_else(|| model.lookup(scope, store?, kind));
            if uncertain {
                if let Some(owner) = owner {
                    uncertain_captures.push(owner.name.clone());
                }
                if let Some((primary, others)) = declaration {
                    unknown_uses.insert(primary);
                    unknown_uses.extend(others);
                } else {
                    unknown_uses.extend(model.component_template_declaration(scope, &name, kind));
                }
                resolution.insert(range, Resolution::Unknown);
                continue;
            }
            if let Some((declaration, others)) = declaration {
                if !is_store && access != Access::Read && constants.contains(&declaration) {
                    const_writes.insert(range);
                }
                for range in std::iter::once(declaration).chain(others) {
                    uses.entry(range)
                        .or_default()
                        .record(kind, if is_store { Access::Read } else { access });
                }
                resolution.insert(range, Resolution::Resolved(declaration));
                if kind == ReferenceKind::Value
                    && let Some(owner) = owner
                    && (is_store
                        || model
                            .lookup(Scope::Module, &name, kind)
                            .map(|(primary, _)| primary)
                            != Some(declaration))
                {
                    captured.push(owner.name.clone());
                }
            } else {
                missing.push((scope, range, name, kind, access));
            }
        }
        model.uses = uses;
        model.resolution = resolution;
        model.unknown_uses = unknown_uses;
        model.const_writes = const_writes;
        model.module_snippets = model.instance_snippets.keys().cloned().collect();
        while let Some(name) = captured.pop() {
            if model.module_snippets.remove(&name)
                && let Some(dependents) = callers.remove(&name)
            {
                captured.extend(dependents);
            }
        }
        let mut maybe_module_snippets = FxHashSet::default();
        while let Some(name) = uncertain_captures.pop() {
            if model.module_snippets.remove(&name) {
                if let Some(dependents) = callers.remove(&name) {
                    uncertain_captures.extend(dependents);
                }
                maybe_module_snippets.insert(name);
            }
        }
        for (scope, range, name, namespace, access) in missing {
            if let Some(declaration) = model.component_template_declaration(scope, &name, namespace)
            {
                model
                    .uses
                    .entry(declaration)
                    .or_default()
                    .record(namespace, access);
                model
                    .resolution
                    .insert(range, Resolution::Resolved(declaration));
            } else if matches!(namespace, ReferenceKind::Value | ReferenceKind::TypeQuery)
                && ((model.unknown.root_snippets
                    && matches!(scope, Scope::Instance | Scope::Module))
                    || (scope == Scope::Module && maybe_module_snippets.contains(&name)))
            {
                model
                    .unknown_uses
                    .extend(model.instance_snippets.get(&name));
                model.resolution.insert(range, Resolution::Unknown);
            } else {
                model.resolution.insert(range, Resolution::Unresolved);
            }
        }
        model
    }

    fn collect_script_declarations(
        &mut self,
        SvelteSnippet { offset, source, js }: &SvelteSnippet,
        constants: &mut FxHashSet<TextRange>,
    ) {
        let scope = match source.as_embedding_kind() {
            JsEmbeddingKind::Svelte {
                embedding_kind: SvelteEmbeddingKind::Source,
                is_module_script: true,
                ..
            } => Scope::Module,
            JsEmbeddingKind::Svelte {
                embedding_kind: SvelteEmbeddingKind::Source,
                ..
            } => Scope::Instance,
            JsEmbeddingKind::Svelte { .. } => Scope::Template,
            _ => Scope::IsolatedScript,
        };
        self.scopes.insert(*offset, scope);
        if self.unknown.unreadable_snippets.contains(offset) {
            return;
        }
        let declarations = match scope {
            Scope::Module => &mut self.module,
            Scope::Instance => &mut self.instance,
            Scope::Template | Scope::IsolatedScript => return,
        };
        for binding in js.global_scope().bindings() {
            let tree = binding.tree();
            if let Ok(name) = tree.name_token() {
                let key = identifier_name(name.token_text_trimmed());
                let range = name.text_trimmed_range() + *offset;
                let kind = match binding.declaration_kind() {
                    JsDeclarationKind::Module
                        if matches!(
                            tree.declaration(),
                            Some(AnyJsBindingDeclaration::TsModuleDeclaration(_))
                        ) =>
                    {
                        JsDeclarationKind::Namespace
                    }
                    kind => kind,
                };
                self.declaration_kinds.insert(range, kind);
                if kind == JsDeclarationKind::Value
                    && let Some(decl) = tree.declaration()
                    && let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) =
                        decl.parent_binding_pattern_declaration().unwrap_or(decl)
                    && declarator.declaration().is_some_and(|decl| decl.is_const())
                {
                    constants.insert(range);
                }
                // Imports in either script are module-level imports.
                if scope == Scope::Instance && kind.is_import_declaration() {
                    self.instance_imports
                        .entry(key.clone())
                        .or_default()
                        .insert(range, kind);
                }
                declarations.entry(key).or_default().insert(range, kind);
            }
        }
    }

    fn unknown_template_scope(&self, position: TextSize) -> Option<TextRange> {
        self.unknown
            .template_declarations
            .iter()
            .copied()
            .filter(|scope| scope.contains(position))
            .min_by_key(|scope| scope.len())
    }

    fn lookup_is_uncertain(&self, scope: Scope, name: &str, kind: ReferenceKind) -> bool {
        let (inner, unknown_inner, unknown_outer) = match scope {
            Scope::IsolatedScript => return false,
            Scope::Module => (
                &self.module,
                self.unknown.module_script,
                self.unknown.instance_script,
            ),
            Scope::Instance | Scope::Template => (
                &self.instance,
                self.unknown.instance_script,
                self.unknown.module_script,
            ),
        };
        if !unknown_inner && !unknown_outer && !self.unknown.generics {
            return false;
        }
        if unknown_inner {
            return true;
        }
        if inner.get(name).and_then(|decls| decls.get(kind)).is_some() {
            return false;
        }
        unknown_outer
            || (kind == ReferenceKind::Type && scope != Scope::Module && self.unknown.generics)
    }

    fn possible_uses_in_unknown_content(&self) -> FxHashSet<TextRange> {
        let mut uses = FxHashSet::default();
        for (declarations, accessible) in [
            (
                &self.module,
                self.unknown.module_script || self.unknown.instance_script,
            ),
            (&self.instance, self.unknown.instance_script),
            (&self.instance_imports, self.unknown.module_script),
        ] {
            if accessible {
                uses.extend(
                    declarations
                        .values()
                        .flat_map(|decls| decls.0.iter().map(|&(range, _)| range)),
                );
            }
        }
        if self.unknown.instance_script {
            uses.extend(self.generics.values().copied());
        }
        if self.unknown.instance_script || self.unknown.module_script {
            uses.extend(self.instance_snippets.values().copied());
        }
        if self.unknown.template_reads.is_empty() {
            return uses;
        }
        let [hidden_values, hidden_types] = self
            .template_bindings
            .possible_uses(&self.unknown.template_reads, &mut uses);
        for name in self
            .module
            .keys()
            .chain(self.instance.keys())
            .chain(self.generics.keys())
        {
            for kind in [
                ReferenceKind::Value,
                ReferenceKind::Type,
                ReferenceKind::Qualifier,
                ReferenceKind::TypeQuery,
            ] {
                if match kind {
                    ReferenceKind::Type => hidden_types.contains(name),
                    ReferenceKind::Value | ReferenceKind::TypeQuery => hidden_values.contains(name),
                    ReferenceKind::Qualifier => false,
                } {
                    continue;
                }
                if let Some((primary, others)) = self.lookup(Scope::Template, name, kind) {
                    uses.insert(primary);
                    uses.extend(others);
                } else if let Some(local) =
                    self.component_template_declaration(Scope::Template, name, kind)
                {
                    uses.insert(local);
                }
            }
        }
        uses
    }

    fn lookup(
        &self,
        scope: Scope,
        name: &str,
        kind: ReferenceKind,
    ) -> Option<(TextRange, impl Iterator<Item = TextRange> + '_)> {
        let (inner, outer) = match scope {
            Scope::IsolatedScript => return None,
            Scope::Module => (&self.module, &self.instance_imports),
            Scope::Template | Scope::Instance => (&self.instance, &self.module),
        };
        if let Some(range) = inner
            .get(name)
            .and_then(|declarations| declarations.get(kind))
        {
            return Some(range);
        }
        if kind == ReferenceKind::Type
            && self
                .component_template_declaration(scope, name, kind)
                .is_some()
        {
            return None;
        }
        outer.get(name)?.get(kind)
    }

    fn template_lookup(
        &self,
        name: &str,
        position: TextSize,
        kind: ReferenceKind,
    ) -> Option<(TextRange, TextRange, bool)> {
        if kind == ReferenceKind::Qualifier {
            return None;
        }
        self.template_bindings
            .lookup(name, position, kind == ReferenceKind::Type)
    }

    fn component_template_declaration(
        &self,
        scope: Scope,
        name: &str,
        kind: ReferenceKind,
    ) -> Option<TextRange> {
        match kind {
            ReferenceKind::Type if matches!(scope, Scope::Instance | Scope::Template) => {
                self.generics.get(name).copied()
            }
            ReferenceKind::Value | ReferenceKind::TypeQuery
                if scope == Scope::Instance
                    || (scope == Scope::Module && self.module_snippets.contains(name)) =>
            {
                self.instance_snippets.get(name).copied()
            }
            _ => None,
        }
    }

    pub fn each_key_status(&self, opening_range: TextRange) -> Option<SvelteEachKeyStatus> {
        self.each_keys.get(&opening_range).copied()
    }

    /// Returns the visible script value declaration's host range, independently of script order.
    pub fn declaration(&self, snippet_offset: TextSize, name: &str) -> Option<TextRange> {
        let scope = *self.scopes.get(&snippet_offset)?;
        let name = unescape_js_identifier(name);
        if self.lookup_is_uncertain(scope, &name, ReferenceKind::Value) {
            return None;
        }
        self.lookup(scope, &name, ReferenceKind::Value)
            .map(|(primary, _)| primary)
    }

    /// Resolves a cross-snippet reference to its declaration. Both ranges
    /// are relative to the host; lexical references belong to the JavaScript model.
    pub fn declaration_for_reference(&self, range: TextRange) -> Option<TextRange> {
        match self.resolution(range)? {
            Resolution::Resolved(range) => Some(range),
            _ => None,
        }
    }

    /// Returns the kind of script declaration resolved by a cross-snippet reference.
    pub fn declaration_kind_for_reference(&self, range: TextRange) -> Option<JsDeclarationKind> {
        let declaration = self.declaration_for_reference(range)?;
        self.declaration_kinds.get(&declaration).copied()
    }

    pub fn is_const_write(&self, host_range: TextRange) -> bool {
        self.const_writes.contains(&host_range)
    }

    pub fn is_unresolved_reference(&self, range: TextRange) -> bool {
        self.resolution(range) == Some(Resolution::Unresolved)
    }

    pub fn resolution(&self, range: TextRange) -> Option<Resolution> {
        self.resolution.get(&range).copied()
    }

    /// Returns `None` when the visible declarations or the snippet are unknown.
    pub fn contains_binding(&self, offset: TextSize, name: &str) -> Option<bool> {
        let scope = *self.scopes.get(&offset)?;
        let name = unescape_js_identifier(name);
        if self.unknown.unreadable_snippets.contains(&offset)
            || (scope == Scope::Template && self.unknown_template_scope(offset).is_some())
        {
            return None;
        }
        let mut result = Some(false);
        for kind in [ReferenceKind::Type, ReferenceKind::Value] {
            if scope == Scope::Template && self.template_lookup(&name, offset, kind).is_some() {
                return Some(true);
            }
            if self.lookup_is_uncertain(scope, &name, kind) {
                result = None;
            } else if self.lookup(scope, &name, kind).is_some()
                || self
                    .component_template_declaration(scope, &name, kind)
                    .is_some()
            {
                return Some(true);
            }
        }
        result
    }

    /// Returns the uses outside the lexical JavaScript model, or `None` when unreadable content may use the declaration.
    pub fn binding_uses(&self, offset: TextSize, range: TextRange) -> Option<BindingUses> {
        self.scopes.get(&offset)?;
        if self.unknown.unreadable_snippets.contains(&offset)
            || self.unknown_uses.contains(&(range + offset))
        {
            return None;
        }
        Some(
            self.uses
                .get(&(range + offset))
                .copied()
                .unwrap_or_default(),
        )
    }

    /// Returns whether another snippet or the host template references this declaration.
    pub fn script_binding_used(&self, offset: TextSize, range: TextRange) -> Option<bool> {
        let uses = self.binding_uses(offset, range)?;
        Some(uses.value_read || uses.type_read || uses.write)
    }
}
