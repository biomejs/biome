//! The component layer: finds the Vue components in one JavaScript tree and
//! records their declarations in one shape per kind.
//!
//! Vue lets the same declaration be written many ways. A prop can be an entry
//! of an array, a member of an object, or a member of a type, and its default
//! can sit next to it, in `withDefaults()`, or in a destructuring pattern.
//! This pass reads each of those forms once, so that consumers of the model
//! never have to.

use super::builder::{ModelBuilder, NewSymbol, Role, synthetic_name, to_host};
use super::model::*;
use biome_js_semantic::SemanticModel as JsSemanticModel;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsArrayElement, AnyJsArrowFunctionParameters, AnyJsBinding,
    AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody,
    AnyJsLiteralExpression, AnyJsMemberExpression, AnyJsModuleItem, AnyJsNamedImportSpecifier,
    AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyJsParameter, AnyJsRoot, AnyJsStatement,
    AnyTsType, AnyTsTypeMember, JsCallExpression, JsExportDefaultExpressionClause,
    JsIdentifierBinding, JsImport, JsModule, JsObjectBindingPattern, JsObjectExpression,
    JsParameters, JsReferenceIdentifier, JsReturnStatement, JsSyntaxKind, JsSyntaxNode,
    TsIdentifierBinding, TsInterfaceDeclaration, TsTypeAliasDeclaration, TsTypeMemberList,
};
use biome_languages::JsFileSource;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, TextRange, TextSize, TokenText, WalkEvent,
};

/// One JavaScript tree to look for components in.
pub(crate) struct ScriptInput<'a> {
    pub(crate) root: &'a AnyJsRoot,
    pub(crate) js: &'a JsSemanticModel,
    pub(crate) source: JsFileSource,
    /// The host offset of position zero of the tree.
    pub(crate) offset: TextSize,
    pub(crate) snippet: Option<SnippetId>,
    /// The script block the tree is the content of, in a single-file
    /// component.
    pub(crate) block: Option<BlockId>,
    /// The `<script setup>` component of the same file, when this tree is
    /// the plain `<script>` block next to it. Vue merges the default export
    /// of this tree into that component, so its `name` and `inheritAttrs`
    /// are recorded on that component too.
    pub(crate) setup_component: Option<ComponentId>,
}

/// What the component layer found in one tree.
#[derive(Default)]
pub(crate) struct ScriptOutput {
    pub(crate) components: Vec<ComponentId>,
    pub(crate) module_scope: Option<ScopeId>,
    /// The component the tree exports as its default export.
    pub(crate) default_export: Option<ComponentId>,
}

const LIFECYCLE_OPTIONS: &[&str] = &[
    "activated",
    "beforeCreate",
    "beforeMount",
    "beforeUnmount",
    "beforeUpdate",
    "created",
    "deactivated",
    "errorCaptured",
    "mounted",
    "renderTracked",
    "renderTriggered",
    "serverPrefetch",
    "unmounted",
    "updated",
];

/// Composition API functions that register a lifecycle hook, and the name of
/// the hook each registers.
const LIFECYCLE_FUNCTIONS: &[(&str, &str)] = &[
    ("onActivated", "activated"),
    ("onBeforeMount", "beforeMount"),
    ("onBeforeUnmount", "beforeUnmount"),
    ("onBeforeUpdate", "beforeUpdate"),
    ("onDeactivated", "deactivated"),
    ("onErrorCaptured", "errorCaptured"),
    ("onMounted", "mounted"),
    ("onRenderTracked", "renderTracked"),
    ("onRenderTriggered", "renderTriggered"),
    ("onServerPrefetch", "serverPrefetch"),
    ("onUnmounted", "unmounted"),
    ("onUpdated", "updated"),
];

/// How many type aliases are followed to find the members of a type.
const MAX_TYPE_ALIAS_DEPTH: u8 = 16;

const WATCH_FUNCTIONS: &[&str] = &["watch", "watchEffect", "watchPostEffect", "watchSyncEffect"];

/// Finds the components in `input` and records them in `builder`.
pub(crate) fn collect_components(builder: &mut ModelBuilder, input: &ScriptInput) -> ScriptOutput {
    let mut collector = Collector {
        builder,
        input,
        output: ScriptOutput::default(),
        default_call: None,
    };
    collector.run();
    collector.output
}

struct Collector<'a, 'b> {
    builder: &'a mut ModelBuilder,
    input: &'a ScriptInput<'b>,
    output: ScriptOutput,
    /// The range of a `defineComponent()` call that is the default export.
    default_call: Option<TextRange>,
}

impl Collector<'_, '_> {
    fn run(&mut self) {
        let embedding = self.input.source.as_embedding_kind();
        let is_setup = embedding.is_vue_setup();
        let is_vue = embedding.is_vue();

        // The top-level bindings of a script block of a single-file component
        // are visible to the template, so they get a scope of their own.
        if is_vue {
            let scope = self
                .builder
                .add_scope(ScopeKind::Module(self.input.block), None);
            self.output.module_scope = Some(scope);
        }

        let module = match self.input.root {
            AnyJsRoot::JsModule(module) => Some(module),
            _ => None,
        };

        if is_setup {
            let Some(module) = module else {
                return;
            };
            let range = self.host(module.syntax().text_trimmed_range());
            let component =
                self.builder
                    .add_component(ComponentKind::Setup, range, self.input.snippet);
            self.output.components.push(component);
            self.collect_module_bindings(Some(component));
            self.collect_setup_statements(module.items().iter(), component);
        } else {
            self.collect_module_bindings(None);
        }

        // A default export is a component only in the plain block of a
        // single-file component. A call into Vue is one in any tree, so a
        // tree that never mentions Vue is not walked for calls.
        let has_default_export = is_vue && !is_setup;
        let has_calls = self.mentions_vue(module);
        if !has_default_export && !has_calls {
            return;
        }
        for node in self.input.root.syntax().descendants() {
            if let Some(clause) = JsExportDefaultExpressionClause::cast_ref(&node) {
                if has_default_export {
                    self.collect_default_export(&clause);
                }
            } else if has_calls && let Some(call) = JsCallExpression::cast_ref(&node) {
                self.collect_component_call(&call);
            }
        }
    }

    fn host(&self, range: TextRange) -> TextRange {
        to_host(range, self.input.offset)
    }

    /// Returns `true` when the tree imports from `vue` or uses the `Vue`
    /// global.
    fn mentions_vue(&self, module: Option<&JsModule>) -> bool {
        let imports_vue = module.is_some_and(|module| {
            module.items().iter().any(|item| match item {
                AnyJsModuleItem::JsImport(import) => import
                    .source_text()
                    .is_ok_and(|source| source.text() == "vue"),
                _ => false,
            })
        });
        imports_vue
            || self
                .input
                .js
                .all_unresolved_references()
                .any(|reference| reference.syntax().text_trimmed() == "Vue")
            || self
                .input
                .js
                .all_global_references()
                .any(|reference| reference.syntax().text_trimmed() == "Vue")
    }

    // -----------------------------------------------------------------------
    // Component detection
    // -----------------------------------------------------------------------

    fn collect_default_export(&mut self, clause: &JsExportDefaultExpressionClause) {
        let Ok(expression) = clause.expression() else {
            return;
        };
        let define_component_call = match expression.inner_expression() {
            Some(AnyJsExpression::JsCallExpression(call))
                if call
                    .callee()
                    .is_ok_and(|callee| self.is_vue_api(&callee, "defineComponent")) =>
            {
                Some(call)
            }
            _ => None,
        };
        if let Some(target) = self.input.setup_component {
            let options = match &define_component_call {
                Some(call) => call
                    .arguments()
                    .ok()
                    .and_then(|arguments| arguments.args().iter().next_back()?.ok())
                    .and_then(|argument| argument.as_any_js_expression().cloned()),
                None => Some(expression.clone()),
            };
            if let Some(AnyJsExpression::JsObjectExpression(object)) =
                options.and_then(|options| options.inner_expression())
            {
                self.record_merged_options(&object, target);
            }
        }
        if let Some(call) = define_component_call {
            // The call is recorded when the walk reaches it.
            self.default_call = Some(call.syntax().text_trimmed_range());
            return;
        }
        let range = self.host(expression.syntax().text_trimmed_range());
        let component =
            self.builder
                .add_component(ComponentKind::OptionsObject, range, self.input.snippet);
        self.output.components.push(component);
        self.output.default_export = Some(component);
        self.collect_options_expression(&expression, component);
    }

    /// Records on the `<script setup>` component what the default export of
    /// the plain `<script>` block next to it says about the component as a
    /// whole.
    fn record_merged_options(&mut self, object: &JsObjectExpression, target: ComponentId) {
        for member in object.members().iter().flatten() {
            let Some(name) = member.name() else {
                continue;
            };
            match name.text() {
                "props" | "emits" => {
                    self.builder.component_mut(target).macro_conflict = true;
                }
                // `defineOptions()` in the setup block wins over the plain
                // block, so only fill in what it left unset.
                "name" | "inheritAttrs" => match scalar_option(&member) {
                    Some(ScalarOption::Name(name, range)) => {
                        let range = self.host(range);
                        let target = self.builder.component_mut(target);
                        if target.name.is_none() {
                            target.name = Some((name, range));
                        }
                    }
                    Some(ScalarOption::InheritAttrs(value)) => {
                        let target = self.builder.component_mut(target);
                        if target.inherit_attrs == Tri::Unknown {
                            target.inherit_attrs = value;
                        }
                    }
                    None => {}
                },
                _ => {}
            }
        }
    }

    fn collect_component_call(&mut self, call: &JsCallExpression) {
        let Ok(callee) = call.callee() else {
            return;
        };
        let kind = if self.is_vue_api(&callee, "defineComponent") {
            ComponentKind::DefineComponent
        } else if self.is_vue_api(&callee, "createApp") {
            ComponentKind::CreateApp
        } else {
            return;
        };
        let local_range = call.syntax().text_trimmed_range();
        let is_default_export = self.default_call == Some(local_range);
        let range = self.host(local_range);
        let component = self.builder.add_component(kind, range, self.input.snippet);
        self.output.components.push(component);
        if is_default_export {
            self.output.default_export = Some(component);
        }

        let arguments: Vec<AnyJsExpression> = call
            .arguments()
            .ok()
            .into_iter()
            .flat_map(|arguments| arguments.args().iter().flatten())
            .filter_map(|argument| argument.as_any_js_expression().cloned())
            .collect();

        match kind {
            // createApp({ props: [...] })
            ComponentKind::CreateApp => {
                if let Some(options) = arguments.first() {
                    self.collect_options_expression(options, component);
                }
            }
            // defineComponent({ props: [...] })
            // defineComponent(setup, { props: [...] })
            _ => {
                if arguments.len() == 2
                    && let Some(setup) = arguments.first()
                {
                    self.collect_setup_function(expression_function(setup), component);
                }
                if let Some(options) = arguments.last() {
                    self.collect_options_expression(options, component);
                }
            }
        }
    }

    /// Returns `true` when `expression` refers to the function `api_name`
    /// exported by Vue, either imported from `vue` or reached through the
    /// `Vue` global.
    fn is_vue_api(&self, expression: &AnyJsExpression, api_name: &str) -> bool {
        let Some(expression) = expression.inner_expression() else {
            return false;
        };
        let js = self.input.js;
        if let Some(member) = AnyJsMemberExpression::cast_ref(expression.syntax()) {
            let Some(object) = member.object().ok() else {
                return false;
            };
            let object = object.omit_parentheses();
            let Some(reference) = object.as_js_reference_identifier() else {
                return false;
            };
            if member
                .member_name()
                .is_none_or(|name| name.text() != api_name)
            {
                return false;
            }
            return match js.binding(&reference) {
                Some(binding) => is_imported_from_vue(&binding.syntax()),
                None => reference.has_name("Vue"),
            };
        }
        let Some(reference) = expression.as_js_reference_identifier() else {
            return false;
        };
        let Some(binding) = js.binding(&reference) else {
            return false;
        };
        let syntax = binding.syntax();
        let Some(identifier) = JsIdentifierBinding::cast_ref(&syntax) else {
            return false;
        };
        let Some(specifier) = identifier.parent::<AnyJsNamedImportSpecifier>() else {
            return false;
        };
        let imported_name = match &specifier {
            AnyJsNamedImportSpecifier::JsNamedImportSpecifier(named) => named
                .name()
                .ok()
                .and_then(|name| name.value().ok())
                .map(|token| token.token_text_trimmed()),
            AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => identifier
                .name_token()
                .ok()
                .map(|token| token.token_text_trimmed()),
            AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
        };
        imported_name.is_some_and(|name| name.text() == api_name) && is_imported_from_vue(&syntax)
    }

    /// Returns `true` when `call` calls the compiler macro `name`, which is a
    /// global that nothing in the file declares.
    fn is_macro_call(&self, call: &JsCallExpression, name: &str) -> bool {
        self.callee_reference(call).is_some_and(|reference| {
            reference.has_name(name) && self.input.js.binding(&reference).is_none()
        })
    }

    /// Returns `true` when `call` calls a function named `name` that is
    /// either imported or not declared at all.
    ///
    /// Composition functions such as `ref` are imported from Vue, but
    /// projects with auto-imports use them without an import.
    fn is_composition_call(&self, call: &JsCallExpression, name: &str) -> bool {
        self.callee_reference(call).is_some_and(|reference| {
            reference.has_name(name)
                && self
                    .input
                    .js
                    .binding(&reference)
                    .is_none_or(|binding| binding.is_imported())
        })
    }

    fn callee_reference(&self, call: &JsCallExpression) -> Option<JsReferenceIdentifier> {
        call.callee()
            .ok()?
            .inner_expression()?
            .as_js_reference_identifier()
    }

    // -----------------------------------------------------------------------
    // Module scope
    // -----------------------------------------------------------------------

    /// Records the top-level bindings of a script block of a single-file
    /// component, which the template can refer to.
    fn collect_module_bindings(&mut self, component: Option<ComponentId>) {
        let Some(scope) = self.output.module_scope else {
            return;
        };
        let module_scope = self.input.js.scope(self.input.root.syntax());
        // The semantic model stores the bindings of a module in a child of
        // the global scope when the root is a module.
        let scopes = std::iter::once(module_scope.clone()).chain(
            module_scope
                .children()
                .filter(|child| child.syntax() == *self.input.root.syntax()),
        );
        let mut seen = Vec::new();
        for js_scope in scopes {
            for binding in js_scope.bindings() {
                let syntax = binding.syntax();
                let Some(name_token) = syntax.first_token() else {
                    continue;
                };
                let local_range = name_token.text_trimmed_range();
                if seen.contains(&local_range) {
                    continue;
                }
                seen.push(local_range);
                let is_import = binding.is_imported();
                let type_only = is_type_only_binding(&syntax);
                let range = self.host(local_range);
                let declaration = syntax
                    .ancestors()
                    .find(|ancestor| {
                        AnyJsStatement::can_cast(ancestor.kind())
                            || JsImport::can_cast(ancestor.kind())
                    })
                    .map_or(range, |node| self.host(node.text_trimmed_range()));
                let id = self.builder.add_symbol(NewSymbol {
                    name: name_token.token_text_trimmed(),
                    range,
                    declaration,
                    snippet: self.input.snippet,
                    scope,
                    namespace: Namespace::Binding,
                    kind: if is_import {
                        SymbolKind::Import
                    } else {
                        SymbolKind::Local
                    },
                    component,
                });
                self.builder.symbol_mut(id).type_only = type_only;
            }
        }
    }

    fn module_symbol_at(&self, range: TextRange) -> Option<SymbolId> {
        let scope = self.output.module_scope?;
        self.builder.data.scopes[scope.index()]
            .symbols
            .iter()
            .copied()
            .find(|id| self.builder.data.symbols[id.index()].range == range)
    }

    // -----------------------------------------------------------------------
    // Options objects
    // -----------------------------------------------------------------------

    fn collect_options_expression(&mut self, expression: &AnyJsExpression, component: ComponentId) {
        let Some(AnyJsExpression::JsObjectExpression(object)) = expression.inner_expression()
        else {
            self.builder.open_all(component, OpenReasons::NON_LITERAL);
            return;
        };
        self.collect_options_object(&object, component);
    }

    fn collect_options_object(&mut self, object: &JsObjectExpression, component: ComponentId) {
        for member in object.members().iter().flatten() {
            if matches!(member, AnyJsObjectMember::JsSpread(_)) {
                self.builder.open_all(component, OpenReasons::SPREAD);
                continue;
            }
            let Some(name) = member.name() else {
                continue;
            };
            match name.text() {
                "props" => self.collect_props_option(&member, component),
                "emits" => self.collect_emits_option(&member, component),
                "data" => self.collect_data_option(&member, component, SymbolKind::Data),
                "asyncData" => self.collect_data_option(&member, component, SymbolKind::AsyncData),
                "computed" => self.collect_computed_option(&member, component),
                "methods" => self.collect_methods_option(&member, component),
                "watch" => self.collect_watch_option(&member, component),
                "inject" => self.collect_inject_option(&member, component),
                "components" => self.collect_registrations(
                    &member,
                    component,
                    Namespace::Component,
                    SymbolKind::ComponentRegistration,
                ),
                "directives" => self.collect_registrations(
                    &member,
                    component,
                    Namespace::Directive,
                    SymbolKind::DirectiveRegistration,
                ),
                "setup" => self.collect_setup_function(member_function(&member), component),
                "name" | "inheritAttrs" => self.collect_scalar_option(&member, component),
                "extends" | "mixins" => self.builder.open_all(component, OpenReasons::EXTENDS),
                other => {
                    if LIFECYCLE_OPTIONS.contains(&other) {
                        self.add_instance_symbol(
                            component,
                            name.clone(),
                            member_name_range(&member),
                            member.syntax().text_trimmed_range(),
                            Namespace::Option,
                            SymbolKind::LifecycleHook,
                        );
                    }
                }
            }
        }
    }

    /// Reads the `name` or `inheritAttrs` option.
    fn collect_scalar_option(&mut self, member: &AnyJsObjectMember, component: ComponentId) {
        match scalar_option(member) {
            Some(ScalarOption::Name(name, range)) => {
                let range = self.host(range);
                self.builder.component_mut(component).name = Some((name, range));
            }
            Some(ScalarOption::InheritAttrs(value)) => {
                self.builder.component_mut(component).inherit_attrs = value;
            }
            None => {}
        }
    }

    /// Adds a declaration to the instance scope of `component`. Ranges are in
    /// the coordinates of the tree.
    fn add_instance_symbol(
        &mut self,
        component: ComponentId,
        name: TokenText,
        range: Option<TextRange>,
        declaration: TextRange,
        namespace: Namespace,
        kind: SymbolKind,
    ) -> SymbolId {
        let scope = self.builder.data.components[component.index()].instance;
        let declaration = self.host(declaration);
        self.builder.add_symbol(NewSymbol {
            name,
            range: range.map_or(declaration, |range| self.host(range)),
            declaration,
            snippet: self.input.snippet,
            scope,
            namespace,
            kind,
            component: Some(component),
        })
    }

    // --- props -------------------------------------------------------------

    fn collect_props_option(&mut self, member: &AnyJsObjectMember, component: ComponentId) {
        match member_value(member) {
            Some(expression) => {
                self.collect_runtime_props(&expression, component);
            }
            None => self
                .builder
                .open(component, Namespace::Binding, OpenReasons::NON_LITERAL),
        }
    }

    /// Reads props written as a runtime value: `['a']` or `{ a: String }`.
    fn collect_runtime_props(
        &mut self,
        expression: &AnyJsExpression,
        component: ComponentId,
    ) -> Vec<SymbolId> {
        let mut props = Vec::new();
        match expression.inner_expression() {
            Some(AnyJsExpression::JsArrayExpression(array)) => {
                for element in array.elements().iter().flatten() {
                    let AnyJsArrayElement::AnyJsExpression(
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(literal),
                        ),
                    ) = element
                    else {
                        self.builder
                            .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
                        continue;
                    };
                    let Ok(name) = literal.inner_string_text() else {
                        continue;
                    };
                    let range = literal.syntax().text_trimmed_range();
                    let id = self.add_instance_symbol(
                        component,
                        name,
                        Some(range),
                        range,
                        Namespace::Binding,
                        SymbolKind::Prop,
                    );
                    self.builder.symbol_mut(id).detail = Detail::Prop(PropDetail {
                        types: TypeSet::empty(),
                        required: Tri::No,
                        defaults: Box::default(),
                        validator: None,
                        form: PropForm::ArrayString,
                    });
                    props.push(id);
                }
            }
            Some(AnyJsExpression::JsObjectExpression(object)) => {
                for member in object.members().iter().flatten() {
                    if matches!(member, AnyJsObjectMember::JsSpread(_)) {
                        self.builder
                            .open(component, Namespace::Binding, OpenReasons::SPREAD);
                        continue;
                    }
                    // `{ size }` declares the prop `size` with options held by
                    // a variable. Any other member that is not `name: value`
                    // is not a declaration the model can read.
                    let value = match &member {
                        AnyJsObjectMember::JsPropertyObjectMember(property) => {
                            property.value().ok()
                        }
                        AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => None,
                        _ => {
                            self.builder.open(
                                component,
                                Namespace::Binding,
                                OpenReasons::NON_LITERAL,
                            );
                            continue;
                        }
                    };
                    let Some(name) = member.name() else {
                        self.builder
                            .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
                        continue;
                    };
                    let detail = match value {
                        Some(value) => self.runtime_prop_detail(Some(value)),
                        None => PropDetail {
                            types: TypeSet::of(PropType::Unknown),
                            required: Tri::Unknown,
                            defaults: Box::default(),
                            validator: None,
                            form: PropForm::Constructor,
                        },
                    };
                    let id = self.add_instance_symbol(
                        component,
                        name,
                        member_name_range(&member),
                        member.syntax().text_trimmed_range(),
                        Namespace::Binding,
                        SymbolKind::Prop,
                    );
                    self.builder.symbol_mut(id).detail = Detail::Prop(detail);
                    props.push(id);
                }
            }
            _ => self
                .builder
                .open(component, Namespace::Binding, OpenReasons::NON_LITERAL),
        }
        props
    }

    fn runtime_prop_detail(&self, value: Option<AnyJsExpression>) -> PropDetail {
        let value = value.and_then(|value| value.inner_expression());
        let Some(AnyJsExpression::JsObjectExpression(options)) = &value else {
            return PropDetail {
                types: value
                    .as_ref()
                    .map_or_else(TypeSet::empty, |value| self.constructor_types(value)),
                required: Tri::No,
                defaults: Box::default(),
                validator: None,
                form: PropForm::Constructor,
            };
        };
        let mut detail = PropDetail {
            types: TypeSet::empty(),
            required: Tri::No,
            defaults: Box::default(),
            validator: None,
            form: PropForm::Options,
        };
        for member in options.members().iter().flatten() {
            let Some(name) = member.name() else {
                continue;
            };
            match name.text() {
                "type" => {
                    if let Some(value) = member_value(&member) {
                        detail.types = self.constructor_types(&value);
                    }
                }
                "required" => {
                    detail.required = match member_value(&member) {
                        Some(AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsBooleanLiteralExpression(literal),
                        )) => literal.value_token().map_or(Tri::Unknown, |token| {
                            (token.text_trimmed() == "true").into()
                        }),
                        _ => Tri::Unknown,
                    };
                }
                "default" => {
                    let range = match &member {
                        AnyJsObjectMember::JsPropertyObjectMember(property) => property
                            .value()
                            .ok()
                            .map(|value| value.syntax().text_trimmed_range()),
                        AnyJsObjectMember::JsMethodObjectMember(method) => {
                            Some(method.syntax().text_trimmed_range())
                        }
                        AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
                            Some(shorthand.syntax().text_trimmed_range())
                        }
                        _ => None,
                    };
                    if let Some(range) = range {
                        detail.defaults = Box::new([PropDefaultData {
                            range: self.host(range),
                            source: DefaultSource::Inline,
                        }]);
                    }
                }
                "validator" => {
                    detail.validator = Some(self.host(member.syntax().text_trimmed_range()));
                }
                _ => {}
            }
        }
        detail
    }

    /// Reads the type of a prop written as a constructor (`Boolean`) or an
    /// array of constructors (`[Boolean, String]`).
    fn constructor_types(&self, expression: &AnyJsExpression) -> TypeSet {
        let mut types = TypeSet::empty();
        match expression.inner_expression() {
            Some(AnyJsExpression::JsArrayExpression(array)) => {
                for element in array.elements().iter().flatten() {
                    match element {
                        AnyJsArrayElement::AnyJsExpression(expression) => {
                            types.union(self.constructor_types(&expression));
                        }
                        _ => types.insert(PropType::Unknown),
                    }
                }
            }
            Some(AnyJsExpression::JsIdentifierExpression(identifier)) => {
                let ty = identifier
                    .name()
                    .ok()
                    .filter(|reference| self.input.js.binding(reference).is_none())
                    .and_then(|reference| reference.value_token().ok())
                    .map_or(PropType::Unknown, |token| match token.text_trimmed() {
                        "Boolean" => PropType::Boolean,
                        "String" => PropType::String,
                        "Number" => PropType::Number,
                        "Array" => PropType::Array,
                        "Object" => PropType::Object,
                        "Function" => PropType::Function,
                        "Symbol" => PropType::Symbol,
                        "Date" => PropType::Date,
                        "BigInt" => PropType::BigInt,
                        _ => PropType::Unknown,
                    });
                types.insert(ty);
            }
            _ => types.insert(PropType::Unknown),
        }
        types
    }

    /// Reads props written as a type: `defineProps<{ a: string }>()`.
    fn collect_type_props(&mut self, ty: &AnyTsType, component: ComponentId) -> Vec<SymbolId> {
        let mut props = Vec::new();
        let Some(members) = self.resolve_type_members(ty) else {
            self.builder
                .open(component, Namespace::Binding, OpenReasons::UNRESOLVED_TYPE);
            return props;
        };
        for member in members.iter() {
            // `{ title: string }` and `{ onClick(): void }` each declare a
            // prop. Other members, such as index signatures, declare props
            // whose names the model cannot list.
            let (name, types, is_optional) = match &member {
                AnyTsTypeMember::TsPropertySignatureTypeMember(property) => (
                    property.name().ok(),
                    property
                        .type_annotation()
                        .and_then(|annotation| annotation.ty().ok())
                        .map_or_else(TypeSet::empty, |ty| type_set_of(&ty)),
                    property.optional_token().is_some(),
                ),
                AnyTsTypeMember::TsMethodSignatureTypeMember(method) => (
                    method.name().ok(),
                    TypeSet::of(PropType::Function),
                    method.optional_token().is_some(),
                ),
                _ => {
                    self.builder
                        .open(component, Namespace::Binding, OpenReasons::UNRESOLVED_TYPE);
                    continue;
                }
            };
            let Some((name, name_range)) =
                name.and_then(|node| Some((node.name()?, node.syntax().text_trimmed_range())))
            else {
                self.builder
                    .open(component, Namespace::Binding, OpenReasons::UNRESOLVED_TYPE);
                continue;
            };
            let id = self.add_instance_symbol(
                component,
                name,
                Some(name_range),
                member.syntax().text_trimmed_range(),
                Namespace::Binding,
                SymbolKind::Prop,
            );
            self.builder.symbol_mut(id).detail = Detail::Prop(PropDetail {
                types,
                required: (!is_optional).into(),
                defaults: Box::default(),
                validator: None,
                form: PropForm::TypeMember,
            });
            props.push(id);
        }
        props
    }

    /// Returns the members of an object type, following references to type
    /// aliases and interfaces declared in the same tree.
    ///
    /// Returns `None` for a type whose members cannot all be listed: an
    /// imported type, an interface that extends another, or anything that is
    /// not an object type.
    fn resolve_type_members(&self, ty: &AnyTsType) -> Option<TsTypeMemberList> {
        self.resolve_type_members_within(ty, MAX_TYPE_ALIAS_DEPTH)
    }

    fn resolve_type_members_within(&self, ty: &AnyTsType, depth: u8) -> Option<TsTypeMemberList> {
        // Aliases can refer to each other in a cycle, which no object type
        // ends.
        let depth = depth.checked_sub(1)?;
        match ty {
            AnyTsType::TsObjectType(object) => Some(object.members()),
            AnyTsType::TsParenthesizedType(parenthesized) => {
                self.resolve_type_members_within(&parenthesized.ty().ok()?, depth)
            }
            AnyTsType::TsReferenceType(reference) => {
                let name = reference.name().ok()?;
                let binding = self.input.js.binding(name.as_js_reference_identifier()?)?;
                let identifier = TsIdentifierBinding::cast(binding.syntax())?;
                if let Some(alias) = identifier.parent::<TsTypeAliasDeclaration>() {
                    self.resolve_type_members_within(&alias.ty().ok()?, depth)
                } else {
                    identifier
                        .parent::<TsInterfaceDeclaration>()
                        .filter(|interface| interface.extends_clause().is_none())
                        .map(|interface| interface.members())
                }
            }
            _ => None,
        }
    }

    // --- emits -------------------------------------------------------------

    fn collect_emits_option(&mut self, member: &AnyJsObjectMember, component: ComponentId) {
        match member_value(member) {
            Some(expression) => self.collect_runtime_emits(&expression, component),
            None => self
                .builder
                .open(component, Namespace::Emit, OpenReasons::NON_LITERAL),
        }
    }

    fn collect_runtime_emits(&mut self, expression: &AnyJsExpression, component: ComponentId) {
        match expression.inner_expression() {
            Some(AnyJsExpression::JsArrayExpression(array)) => {
                for element in array.elements().iter().flatten() {
                    let AnyJsArrayElement::AnyJsExpression(
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(literal),
                        ),
                    ) = element
                    else {
                        self.builder
                            .open(component, Namespace::Emit, OpenReasons::NON_LITERAL);
                        continue;
                    };
                    let Ok(name) = literal.inner_string_text() else {
                        continue;
                    };
                    let range = literal.syntax().text_trimmed_range();
                    let id = self.add_instance_symbol(
                        component,
                        name,
                        Some(range),
                        range,
                        Namespace::Emit,
                        SymbolKind::Emit,
                    );
                    self.builder.symbol_mut(id).detail = Detail::Emit {
                        payload_params: None,
                        validator: None,
                        form: EmitForm::ArrayString,
                    };
                }
            }
            Some(AnyJsExpression::JsObjectExpression(object)) => {
                for member in object.members().iter().flatten() {
                    if matches!(member, AnyJsObjectMember::JsSpread(_)) {
                        self.builder
                            .open(component, Namespace::Emit, OpenReasons::SPREAD);
                        continue;
                    }
                    let Some(name) = member.name() else {
                        self.builder
                            .open(component, Namespace::Emit, OpenReasons::NON_LITERAL);
                        continue;
                    };
                    let function = member_function(&member);
                    let id = self.add_instance_symbol(
                        component,
                        name,
                        member_name_range(&member),
                        member.syntax().text_trimmed_range(),
                        Namespace::Emit,
                        SymbolKind::Emit,
                    );
                    self.builder.symbol_mut(id).detail = Detail::Emit {
                        payload_params: function
                            .as_ref()
                            .and_then(function_parameters)
                            .map(|(count, _)| count),
                        validator: function
                            .map(|function| self.host(function.syntax().text_trimmed_range())),
                        form: EmitForm::Object,
                    };
                }
            }
            _ => self
                .builder
                .open(component, Namespace::Emit, OpenReasons::NON_LITERAL),
        }
    }

    /// Reads emits written as a type: `defineEmits<{ change: [id: number] }>()`
    /// or `defineEmits<{ (e: 'change', id: number): void }>()`.
    fn collect_type_emits(&mut self, ty: &AnyTsType, component: ComponentId) {
        let Some(members) = self.resolve_type_members(ty) else {
            self.builder
                .open(component, Namespace::Emit, OpenReasons::UNRESOLVED_TYPE);
            return;
        };
        for member in members.iter() {
            match &member {
                AnyTsTypeMember::TsPropertySignatureTypeMember(property) => {
                    let Some(name) = property.name().ok().and_then(|name| name.name()) else {
                        continue;
                    };
                    let payload_params = property
                        .type_annotation()
                        .and_then(|annotation| annotation.ty().ok())
                        .and_then(|ty| match ty {
                            AnyTsType::TsTupleType(tuple) => Some(tuple.elements().len() as u8),
                            _ => None,
                        });
                    let id = self.add_instance_symbol(
                        component,
                        name,
                        property
                            .name()
                            .ok()
                            .map(|name| name.syntax().text_trimmed_range()),
                        property.syntax().text_trimmed_range(),
                        Namespace::Emit,
                        SymbolKind::Emit,
                    );
                    self.builder.symbol_mut(id).detail = Detail::Emit {
                        payload_params,
                        validator: None,
                        form: EmitForm::TypeMember,
                    };
                }
                AnyTsTypeMember::TsCallSignatureTypeMember(signature) => {
                    let Ok(parameters) = signature.parameters() else {
                        continue;
                    };
                    let count = parameters.items().len();
                    let Some(Ok(AnyJsParameter::AnyJsFormalParameter(first))) =
                        parameters.items().iter().next()
                    else {
                        continue;
                    };
                    let Some(event_type) = first
                        .as_js_formal_parameter()
                        .and_then(|parameter| parameter.type_annotation())
                        .and_then(|annotation| annotation.ty().ok())
                    else {
                        self.builder
                            .open(component, Namespace::Emit, OpenReasons::UNRESOLVED_TYPE);
                        continue;
                    };
                    let mut names = Vec::new();
                    if !string_literal_types(&event_type, &mut names) {
                        self.builder
                            .open(component, Namespace::Emit, OpenReasons::UNRESOLVED_TYPE);
                    }
                    for (name, range) in names {
                        let id = self.add_instance_symbol(
                            component,
                            name,
                            Some(range),
                            signature.syntax().text_trimmed_range(),
                            Namespace::Emit,
                            SymbolKind::Emit,
                        );
                        self.builder.symbol_mut(id).detail = Detail::Emit {
                            payload_params: Some(count.saturating_sub(1) as u8),
                            validator: None,
                            form: EmitForm::CallSignature,
                        };
                    }
                }
                _ => {}
            }
        }
    }

    // --- data, computed, methods, watch, inject ------------------------------

    fn collect_data_option(
        &mut self,
        member: &AnyJsObjectMember,
        component: ComponentId,
        kind: SymbolKind,
    ) {
        // `data: { ... }`, or a function whose every return is an object
        // literal. Anything else holds members the model cannot list.
        let (objects, is_readable) =
            match member_value(member).and_then(|value| value.inner_expression()) {
                Some(AnyJsExpression::JsObjectExpression(object)) => (vec![object], true),
                _ => member_function(member)
                    .map_or((Vec::new(), false), |function| returned_objects(&function)),
            };
        if !is_readable {
            self.builder
                .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
        }
        for object in objects {
            self.collect_object_members(&object, component, kind, None);
        }
    }

    /// Records each member of `object` as a declaration of `kind`. Members
    /// whose value is an object literal get their own members recorded as
    /// children, so nested data such as `user.name` can be resolved.
    fn collect_object_members(
        &mut self,
        object: &JsObjectExpression,
        component: ComponentId,
        kind: SymbolKind,
        parent: Option<SymbolId>,
    ) {
        for member in object.members().iter().flatten() {
            if matches!(member, AnyJsObjectMember::JsSpread(_)) {
                if parent.is_none() {
                    self.builder
                        .open(component, Namespace::Binding, OpenReasons::SPREAD);
                }
                continue;
            }
            let Some(name) = member.name() else {
                if parent.is_none() {
                    self.builder
                        .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
                }
                continue;
            };
            let id = self.add_instance_symbol(
                component,
                name,
                member_name_range(&member),
                member.syntax().text_trimmed_range(),
                Namespace::Binding,
                kind,
            );
            if let Some(parent) = parent {
                // Nested members are reached through their parent, not by a
                // bare name, so they do not belong in the instance scope.
                let scope = self.builder.data.symbols[id.index()].scope;
                self.builder.data.scopes[scope.index()]
                    .symbols
                    .retain(|symbol| *symbol != id);
                self.builder.symbol_mut(id).parent = Some(parent);
            }
            if kind == SymbolKind::Data
                && let Some(AnyJsExpression::JsObjectExpression(nested)) =
                    member_value(&member).and_then(|value| value.inner_expression())
            {
                self.collect_object_members(&nested, component, kind, Some(id));
            }
        }
    }

    fn collect_computed_option(&mut self, member: &AnyJsObjectMember, component: ComponentId) {
        let Some(AnyJsExpression::JsObjectExpression(object)) =
            member_value(member).and_then(|value| value.inner_expression())
        else {
            self.builder
                .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
            return;
        };
        for member in object.members().iter().flatten() {
            if matches!(member, AnyJsObjectMember::JsSpread(_)) {
                self.builder
                    .open(component, Namespace::Binding, OpenReasons::SPREAD);
                continue;
            }
            let Some(name) = member.name() else {
                self.builder
                    .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
                continue;
            };
            let mut getter = None;
            let mut setter = None;
            match &member {
                AnyJsObjectMember::JsMethodObjectMember(method) => {
                    getter = Some(method.syntax().text_trimmed_range());
                }
                _ => match member_value(&member).and_then(|value| value.inner_expression()) {
                    Some(AnyJsExpression::JsObjectExpression(accessors)) => {
                        for accessor in accessors.members().iter().flatten() {
                            match accessor.name().as_ref().map(TokenText::text) {
                                Some("get") => {
                                    getter = Some(accessor.syntax().text_trimmed_range());
                                }
                                Some("set") => {
                                    setter = Some(accessor.syntax().text_trimmed_range());
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(value) if is_function_expression(&value) => {
                        getter = Some(value.syntax().text_trimmed_range());
                    }
                    _ => {}
                },
            }
            let id = self.add_instance_symbol(
                component,
                name,
                member_name_range(&member),
                member.syntax().text_trimmed_range(),
                Namespace::Binding,
                SymbolKind::Computed,
            );
            self.builder.symbol_mut(id).detail = Detail::Computed {
                getter: getter.map(|range| self.host(range)),
                setter: setter.map(|range| self.host(range)),
            };
        }
    }

    fn collect_methods_option(&mut self, member: &AnyJsObjectMember, component: ComponentId) {
        let Some(AnyJsExpression::JsObjectExpression(object)) =
            member_value(member).and_then(|value| value.inner_expression())
        else {
            self.builder
                .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
            return;
        };
        for member in object.members().iter().flatten() {
            if matches!(member, AnyJsObjectMember::JsSpread(_)) {
                self.builder
                    .open(component, Namespace::Binding, OpenReasons::SPREAD);
                continue;
            }
            let Some(name) = member.name() else {
                self.builder
                    .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
                continue;
            };
            let function = member_function(&member);
            let id = self.add_instance_symbol(
                component,
                name,
                member_name_range(&member),
                member.syntax().text_trimmed_range(),
                Namespace::Binding,
                SymbolKind::Method,
            );
            if let Some((params, rest)) = function.as_ref().and_then(function_parameters) {
                self.builder.symbol_mut(id).detail = Detail::Method { params, rest };
            }
        }
    }

    fn collect_watch_option(&mut self, member: &AnyJsObjectMember, component: ComponentId) {
        let Some(AnyJsExpression::JsObjectExpression(object)) =
            member_value(member).and_then(|value| value.inner_expression())
        else {
            return;
        };
        for member in object.members().iter().flatten() {
            let Some(name) = member.name() else {
                continue;
            };
            self.add_instance_symbol(
                component,
                name,
                member_name_range(&member),
                member.syntax().text_trimmed_range(),
                Namespace::Option,
                SymbolKind::Watcher,
            );
        }
    }

    fn collect_inject_option(&mut self, member: &AnyJsObjectMember, component: ComponentId) {
        match member_value(member).and_then(|value| value.inner_expression()) {
            Some(AnyJsExpression::JsArrayExpression(array)) => {
                for element in array.elements().iter().flatten() {
                    let AnyJsArrayElement::AnyJsExpression(
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(literal),
                        ),
                    ) = element
                    else {
                        self.builder
                            .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
                        continue;
                    };
                    let Ok(name) = literal.inner_string_text() else {
                        continue;
                    };
                    let range = literal.syntax().text_trimmed_range();
                    self.add_instance_symbol(
                        component,
                        name,
                        Some(range),
                        range,
                        Namespace::Binding,
                        SymbolKind::Inject,
                    );
                }
            }
            Some(AnyJsExpression::JsObjectExpression(object)) => {
                for member in object.members().iter().flatten() {
                    let Some(name) = member.name() else {
                        self.builder
                            .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
                        continue;
                    };
                    self.add_instance_symbol(
                        component,
                        name,
                        member_name_range(&member),
                        member.syntax().text_trimmed_range(),
                        Namespace::Binding,
                        SymbolKind::Inject,
                    );
                }
            }
            _ => self
                .builder
                .open(component, Namespace::Binding, OpenReasons::NON_LITERAL),
        }
    }

    fn collect_registrations(
        &mut self,
        member: &AnyJsObjectMember,
        component: ComponentId,
        namespace: Namespace,
        kind: SymbolKind,
    ) {
        let Some(AnyJsExpression::JsObjectExpression(object)) =
            member_value(member).and_then(|value| value.inner_expression())
        else {
            self.builder
                .open(component, namespace, OpenReasons::NON_LITERAL);
            return;
        };
        for member in object.members().iter().flatten() {
            if matches!(member, AnyJsObjectMember::JsSpread(_)) {
                self.builder.open(component, namespace, OpenReasons::SPREAD);
                continue;
            }
            let Some(name) = member.name() else {
                self.builder
                    .open(component, namespace, OpenReasons::NON_LITERAL);
                continue;
            };
            self.add_instance_symbol(
                component,
                name,
                member_name_range(&member),
                member.syntax().text_trimmed_range(),
                namespace,
                kind,
            );
        }
    }

    // -----------------------------------------------------------------------
    // Setup functions
    // -----------------------------------------------------------------------

    /// Reads a `setup()` function: its parameters, the hooks and watchers it
    /// registers, and the members of the object it returns.
    fn collect_setup_function(
        &mut self,
        function: Option<AnyFunctionLike>,
        component: ComponentId,
    ) {
        // `setup: sharedSetup` exposes bindings the model cannot list.
        let Some(function) = function else {
            self.builder
                .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
            return;
        };

        if let Some(parameters) = parameter_patterns(&function) {
            if let Some(props) = parameters.first() {
                self.assign_props_role(props, component, &[]);
            }
            match parameters.get(1) {
                Some(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                    context,
                ))) => {
                    self.set_role(context, Origin::SetupContext, component, None);
                }
                Some(AnyJsBindingPattern::JsObjectBindingPattern(pattern)) => {
                    for (name, binding, _) in object_pattern_bindings(pattern) {
                        if name.text() == "emit" {
                            self.set_role(&binding, Origin::SetupEmit, component, None);
                        }
                    }
                }
                _ => {}
            }
        }

        for statement in function.statements().into_iter().flatten() {
            if let AnyJsStatement::JsExpressionStatement(statement) = &statement
                && let Ok(expression) = statement.expression()
            {
                self.collect_setup_call(&expression, component);
            }
        }

        // `return toRefs(state)` exposes bindings the model cannot list.
        let (objects, is_readable) = returned_objects(&function);
        if !is_readable {
            self.builder
                .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
        }
        for object in objects {
            for member in object.members().iter().flatten() {
                if matches!(member, AnyJsObjectMember::JsSpread(_)) {
                    self.builder
                        .open(component, Namespace::Binding, OpenReasons::SPREAD);
                    continue;
                }
                let Some(name) = member.name() else {
                    self.builder
                        .open(component, Namespace::Binding, OpenReasons::NON_LITERAL);
                    continue;
                };
                self.add_instance_symbol(
                    component,
                    name,
                    member_name_range(&member),
                    member.syntax().text_trimmed_range(),
                    Namespace::Binding,
                    SymbolKind::SetupReturn,
                );
            }
        }
    }

    /// Reads the top-level statements of a `<script setup>` block.
    fn collect_setup_statements(
        &mut self,
        items: impl Iterator<Item = AnyJsModuleItem>,
        component: ComponentId,
    ) {
        for item in items {
            let AnyJsModuleItem::AnyJsStatement(statement) = item else {
                continue;
            };
            match statement {
                AnyJsStatement::JsExpressionStatement(statement) => {
                    if let Ok(expression) = statement.expression() {
                        self.collect_macro(&expression, None, component);
                        self.collect_setup_call(&expression, component);
                    }
                }
                AnyJsStatement::JsVariableStatement(statement) => {
                    let Ok(declaration) = statement.declaration() else {
                        continue;
                    };
                    for declarator in declaration.declarators().iter().flatten() {
                        let Some(expression) = declarator
                            .initializer()
                            .and_then(|initializer| initializer.expression().ok())
                        else {
                            continue;
                        };
                        let pattern = declarator.id().ok();
                        self.collect_macro(&expression, pattern.as_ref(), component);
                        self.assign_origin(&expression, pattern.as_ref(), component);
                    }
                }
                _ => {}
            }
        }
    }

    /// Records the origin of a top-level `<script setup>` variable from the
    /// call that initializes it, as in `const count = ref(0)`.
    fn assign_origin(
        &mut self,
        expression: &AnyJsExpression,
        pattern: Option<&AnyJsBindingPattern>,
        component: ComponentId,
    ) {
        let Some(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding))) =
            pattern
        else {
            return;
        };
        let Some(AnyJsExpression::JsCallExpression(call)) = expression.inner_expression() else {
            return;
        };
        let origin = if self.is_composition_call(&call, "ref") {
            Origin::Ref
        } else if self.is_composition_call(&call, "shallowRef") {
            Origin::ShallowRef
        } else if self.is_composition_call(&call, "computed") {
            Origin::Computed
        } else if self.is_composition_call(&call, "useTemplateRef") {
            Origin::UseTemplateRef
        } else {
            return;
        };
        self.set_role(binding, origin, component, None);
        if origin == Origin::Computed
            && let Some(symbol) = self.binding_symbol(binding)
        {
            let getter = call
                .arguments()
                .ok()
                .and_then(|arguments| arguments.args().iter().next()?.ok())
                .map(|argument| self.host(argument.syntax().text_trimmed_range()));
            self.builder.symbol_mut(symbol).detail = Detail::Computed {
                getter,
                setter: None,
            };
        }
    }

    /// Records a lifecycle hook or watcher registered by a call at the top
    /// level of a setup function, such as `onMounted(() => {})`.
    fn collect_setup_call(&mut self, expression: &AnyJsExpression, component: ComponentId) {
        let Some(AnyJsExpression::JsCallExpression(call)) = expression.inner_expression() else {
            return;
        };
        let Some(reference) = self.callee_reference(&call) else {
            return;
        };
        let Ok(callee_token) = reference.value_token() else {
            return;
        };
        if self
            .input
            .js
            .binding(&reference)
            .is_some_and(|binding| !binding.is_imported())
        {
            return;
        }
        let callee = callee_token.text_trimmed();
        if let Some((_, hook)) = LIFECYCLE_FUNCTIONS.iter().find(|(name, _)| *name == callee) {
            self.add_instance_symbol(
                component,
                synthetic_name(hook),
                Some(callee_token.text_trimmed_range()),
                call.syntax().text_trimmed_range(),
                Namespace::Option,
                SymbolKind::LifecycleHook,
            );
        } else if WATCH_FUNCTIONS.contains(&callee) {
            self.add_instance_symbol(
                component,
                callee_token.token_text_trimmed(),
                Some(callee_token.text_trimmed_range()),
                call.syntax().text_trimmed_range(),
                Namespace::Option,
                SymbolKind::Watcher,
            );
        }
    }

    /// Reads a compiler macro call, alone or as the initializer of a variable
    /// declared with `pattern`.
    fn collect_macro(
        &mut self,
        expression: &AnyJsExpression,
        pattern: Option<&AnyJsBindingPattern>,
        component: ComponentId,
    ) {
        let Some(AnyJsExpression::JsCallExpression(call)) = expression.inner_expression() else {
            return;
        };

        // withDefaults(defineProps<Props>(), { ... })
        let (props_call, with_defaults) = if self.is_macro_call(&call, "withDefaults") {
            let arguments: Vec<_> = call
                .arguments()
                .ok()
                .into_iter()
                .flat_map(|arguments| arguments.args().iter().flatten())
                .collect();
            let inner = arguments
                .first()
                .and_then(AnyJsCallArgument::as_any_js_expression)
                .and_then(|argument| argument.inner_expression())
                .and_then(|argument| argument.as_js_call_expression().cloned());
            let defaults = arguments
                .get(1)
                .and_then(AnyJsCallArgument::as_any_js_expression)
                .and_then(|argument| argument.inner_expression())
                .and_then(|argument| argument.as_js_object_expression().cloned());
            match inner {
                Some(inner) if self.is_macro_call(&inner, "defineProps") => (inner, defaults),
                _ => return,
            }
        } else {
            (call.clone(), None)
        };

        if self.is_macro_call(&props_call, "defineProps") {
            self.collect_define_props(&props_call, with_defaults.as_ref(), pattern, component);
        } else if self.is_macro_call(&call, "defineEmits") {
            if let Some(argument) = first_argument(&call) {
                self.collect_runtime_emits(&argument, component);
            }
            if let Some(ty) = first_type_argument(&call) {
                self.collect_type_emits(&ty, component);
            }
            if let Some(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                binding,
            ))) = pattern
            {
                self.set_role(binding, Origin::DefineEmits, component, None);
            }
        } else if self.is_macro_call(&call, "defineModel") {
            let name = match first_argument(&call).and_then(|argument| argument.inner_expression())
            {
                Some(AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(literal),
                )) => literal
                    .inner_string_text()
                    .ok()
                    .map(|name| (name, Some(literal.syntax().text_trimmed_range()))),
                _ => None,
            };
            let (name, range) = name.unwrap_or_else(|| (synthetic_name("modelValue"), None));
            self.add_instance_symbol(
                component,
                name,
                range,
                call.syntax().text_trimmed_range(),
                Namespace::Binding,
                SymbolKind::Model,
            );
            if let Some(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                binding,
            ))) = pattern
            {
                self.set_role(binding, Origin::DefineModel, component, None);
            }
        } else if self.is_macro_call(&call, "defineSlots") {
            match first_type_argument(&call).and_then(|ty| self.resolve_type_members(&ty)) {
                Some(members) => {
                    for member in members.iter() {
                        let (name, name_range) = match &member {
                            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => (
                                property.name().ok().and_then(|name| name.name()),
                                property
                                    .name()
                                    .ok()
                                    .map(|name| name.syntax().text_trimmed_range()),
                            ),
                            AnyTsTypeMember::TsMethodSignatureTypeMember(method) => (
                                method.name().ok().and_then(|name| name.name()),
                                method
                                    .name()
                                    .ok()
                                    .map(|name| name.syntax().text_trimmed_range()),
                            ),
                            _ => (None, None),
                        };
                        let Some(name) = name else {
                            continue;
                        };
                        self.add_instance_symbol(
                            component,
                            name,
                            name_range,
                            member.syntax().text_trimmed_range(),
                            Namespace::Slot,
                            SymbolKind::Slot,
                        );
                    }
                }
                None => self
                    .builder
                    .open(component, Namespace::Slot, OpenReasons::UNRESOLVED_TYPE),
            }
            if let Some(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                binding,
            ))) = pattern
            {
                self.set_role(binding, Origin::DefineSlots, component, None);
            }
        } else if self.is_macro_call(&call, "defineOptions")
            && let Some(AnyJsExpression::JsObjectExpression(object)) =
                first_argument(&call).and_then(|argument| argument.inner_expression())
        {
            self.collect_options_object(&object, component);
        }
    }

    fn collect_define_props(
        &mut self,
        call: &JsCallExpression,
        with_defaults: Option<&JsObjectExpression>,
        pattern: Option<&AnyJsBindingPattern>,
        component: ComponentId,
    ) {
        let mut props = Vec::new();
        if let Some(argument) = first_argument(call) {
            props.extend(self.collect_runtime_props(&argument, component));
        }
        if let Some(ty) = first_type_argument(call) {
            props.extend(self.collect_type_props(&ty, component));
        }

        if let Some(defaults) = with_defaults {
            for member in defaults.members().iter().flatten() {
                let Some(name) = member.name() else {
                    continue;
                };
                let range = match &member {
                    AnyJsObjectMember::JsPropertyObjectMember(property) => property
                        .value()
                        .ok()
                        .map(|value| value.syntax().text_trimmed_range()),
                    other => Some(other.syntax().text_trimmed_range()),
                };
                if let Some(range) = range {
                    self.add_prop_default(&props, name.text(), range, DefaultSource::WithDefaults);
                }
            }
        }

        if let Some(pattern) = pattern {
            self.assign_props_role(pattern, component, &props);
        }
    }

    /// Records what the binding pattern that receives the props object
    /// stands for: the object itself, or individual props when destructured.
    fn assign_props_role(
        &mut self,
        pattern: &AnyJsBindingPattern,
        component: ComponentId,
        props: &[SymbolId],
    ) {
        match pattern {
            AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
                let origin = if props.is_empty()
                    && self.builder.data.components[component.index()].kind != ComponentKind::Setup
                {
                    Origin::SetupProps
                } else {
                    Origin::DefineProps
                };
                self.set_role(binding, origin, component, None);
            }
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                for (name, binding, default) in object_pattern_bindings(pattern) {
                    let prop = props
                        .iter()
                        .copied()
                        .chain(
                            self.builder.data.components[component.index()]
                                .symbols
                                .clone(),
                        )
                        .find(|id| {
                            let symbol = &self.builder.data.symbols[id.index()];
                            symbol.kind == SymbolKind::Prop && symbol.name.text() == name.text()
                        });
                    self.set_role(&binding, Origin::PropsDestructure, component, prop);
                    if let Some(default) = default {
                        self.add_prop_default(
                            props,
                            name.text(),
                            default,
                            DefaultSource::Destructure,
                        );
                    }
                }
            }
            _ => {}
        }
    }

    fn add_prop_default(
        &mut self,
        props: &[SymbolId],
        name: &str,
        range: TextRange,
        source: DefaultSource,
    ) {
        let range = self.host(range);
        let Some(id) = props
            .iter()
            .copied()
            .find(|id| self.builder.data.symbols[id.index()].name.text() == name)
        else {
            return;
        };
        if let Detail::Prop(detail) = &mut self.builder.symbol_mut(id).detail {
            let mut defaults = std::mem::take(&mut detail.defaults).into_vec();
            defaults.push(PropDefaultData { range, source });
            detail.defaults = defaults.into_boxed_slice();
        }
    }

    fn binding_symbol(&self, binding: &JsIdentifierBinding) -> Option<SymbolId> {
        let range = self.host(binding.name_token().ok()?.text_trimmed_range());
        self.module_symbol_at(range)
    }

    fn set_role(
        &mut self,
        binding: &JsIdentifierBinding,
        origin: Origin,
        component: ComponentId,
        prop: Option<SymbolId>,
    ) {
        let Ok(token) = binding.name_token() else {
            return;
        };
        let range = self.host(token.text_trimmed_range());
        self.builder.roles.insert(
            range,
            Role {
                origin,
                component,
                prop,
            },
        );
        if let Some(symbol) = self.module_symbol_at(range) {
            self.builder.symbol_mut(symbol).origin = origin;
        }
    }
}

fn is_imported_from_vue(binding: &JsSyntaxNode) -> bool {
    binding
        .ancestors()
        .skip(1)
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| source.text() == "vue")
}

/// Returns `true` for a binding that only exists in the type system: a type
/// alias, an interface, or a type-only import.
fn is_type_only_binding(binding: &JsSyntaxNode) -> bool {
    if TsIdentifierBinding::can_cast(binding.kind()) {
        return binding.parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION | JsSyntaxKind::TS_INTERFACE_DECLARATION
            )
        });
    }
    binding.ancestors().skip(1).any(|ancestor| {
        match ancestor.kind() {
            // import { type Foo } from "./foo"
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER
            | JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
            // import type Foo from "./foo"
            | JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE
            | JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE
            | JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => ancestor
                .children_with_tokens()
                .filter_map(|element| element.into_token())
                .any(|token| token.kind() == JsSyntaxKind::TYPE_KW),
            _ => false,
        }
    })
}

/// A component option whose value is a single literal.
enum ScalarOption {
    /// The component name and the range of the literal in the tree.
    Name(TokenText, TextRange),
    InheritAttrs(Tri),
}

/// Reads the `name` or `inheritAttrs` member of an options object.
fn scalar_option(member: &AnyJsObjectMember) -> Option<ScalarOption> {
    let value = member_value(member).and_then(|value| value.inner_expression());
    match member.name()?.text() {
        "name" => match value {
            Some(AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(literal),
            )) => Some(ScalarOption::Name(
                literal.inner_string_text().ok()?,
                literal.syntax().text_trimmed_range(),
            )),
            _ => None,
        },
        "inheritAttrs" => Some(ScalarOption::InheritAttrs(match value {
            Some(AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsBooleanLiteralExpression(literal),
            )) => literal.value_token().map_or(Tri::Unknown, |token| {
                (token.text_trimmed() == "true").into()
            }),
            _ => Tri::Unknown,
        })),
        _ => None,
    }
}

/// Returns the value of an object member written as `name: value`.
fn member_value(member: &AnyJsObjectMember) -> Option<AnyJsExpression> {
    match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => property.value().ok(),
        _ => None,
    }
}

fn member_name_range(member: &AnyJsObjectMember) -> Option<TextRange> {
    match member {
        AnyJsObjectMember::JsPropertyObjectMember(property) => property
            .name()
            .ok()
            .map(|name| name.syntax().text_trimmed_range()),
        AnyJsObjectMember::JsMethodObjectMember(method) => method
            .name()
            .ok()
            .map(|name| name.syntax().text_trimmed_range()),
        AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => shorthand
            .name()
            .ok()
            .map(|name| name.syntax().text_trimmed_range()),
        AnyJsObjectMember::JsGetterObjectMember(getter) => getter
            .name()
            .ok()
            .map(|name| name.syntax().text_trimmed_range()),
        AnyJsObjectMember::JsSetterObjectMember(setter) => setter
            .name()
            .ok()
            .map(|name| name.syntax().text_trimmed_range()),
        _ => None,
    }
}

fn is_function_expression(expression: &AnyJsExpression) -> bool {
    matches!(
        expression,
        AnyJsExpression::JsFunctionExpression(_) | AnyJsExpression::JsArrowFunctionExpression(_)
    )
}

fn first_argument(call: &JsCallExpression) -> Option<AnyJsExpression> {
    call.arguments()
        .ok()?
        .args()
        .iter()
        .next()?
        .ok()?
        .as_any_js_expression()
        .cloned()
}

fn first_type_argument(call: &JsCallExpression) -> Option<AnyTsType> {
    call.type_arguments()?
        .ts_type_argument_list()
        .iter()
        .next()?
        .ok()
}

/// Returns the function an object member holds: the member itself when it is
/// a method, or the function written as its value.
fn member_function(member: &AnyJsObjectMember) -> Option<AnyFunctionLike> {
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            Some(AnyFunctionLike::JsMethodObjectMember(method.clone()))
        }
        _ => expression_function(&member_value(member)?),
    }
}

/// Returns the function `expression` is, ignoring parentheses and type
/// assertions around it.
fn expression_function(expression: &AnyJsExpression) -> Option<AnyFunctionLike> {
    AnyFunctionLike::cast(expression.inner_expression()?.into_syntax())
}

/// Returns the parenthesized parameter list of a function.
fn parameter_list(function: &AnyFunctionLike) -> Option<JsParameters> {
    match function {
        AnyFunctionLike::AnyJsFunction(function) => function.parenthesized_parameters(),
        AnyFunctionLike::JsMethodObjectMember(method) => method.parameters().ok(),
        AnyFunctionLike::JsMethodClassMember(method) => method.parameters().ok(),
        AnyFunctionLike::JsConstructorClassMember(_) => None,
    }
}

/// Returns the parameter of an arrow function written without parentheses,
/// as in `value => value`.
fn bare_parameter(function: &AnyFunctionLike) -> Option<AnyJsBinding> {
    match function {
        AnyFunctionLike::AnyJsFunction(function) => match function.parameters().ok()? {
            AnyJsArrowFunctionParameters::AnyJsBinding(binding) => Some(binding),
            AnyJsArrowFunctionParameters::JsParameters(_) => None,
        },
        _ => None,
    }
}

/// Returns the number of parameters of a function and whether the last one
/// is a rest parameter.
fn function_parameters(function: &AnyFunctionLike) -> Option<(u8, bool)> {
    if let Some(parameters) = parameter_list(function) {
        let items = parameters.items();
        let rest = items
            .iter()
            .flatten()
            .any(|parameter| matches!(parameter, AnyJsParameter::JsRestParameter(_)));
        return Some((items.len() as u8, rest));
    }
    bare_parameter(function).map(|_| (1, false))
}

/// Returns the binding pattern of each parameter of a function.
fn parameter_patterns(function: &AnyFunctionLike) -> Option<Vec<AnyJsBindingPattern>> {
    if let Some(parameters) = parameter_list(function) {
        return Some(
            parameters
                .items()
                .iter()
                .flatten()
                .filter_map(|parameter| match parameter {
                    AnyJsParameter::AnyJsFormalParameter(parameter) => {
                        parameter.as_js_formal_parameter()?.binding().ok()
                    }
                    _ => None,
                })
                .collect(),
        );
    }
    bare_parameter(function).map(|binding| vec![AnyJsBindingPattern::AnyJsBinding(binding)])
}

/// Returns the name, binding and default value range of each property of an
/// object destructuring pattern.
fn object_pattern_bindings(
    pattern: &JsObjectBindingPattern,
) -> Vec<(TokenText, JsIdentifierBinding, Option<TextRange>)> {
    let mut result = Vec::new();
    for member in pattern.properties().iter().flatten() {
        match member {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                let Some(name) = property.member().ok().and_then(|member| member.name()) else {
                    continue;
                };
                let Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                    binding,
                ))) = property.pattern()
                else {
                    continue;
                };
                let default = property
                    .init()
                    .and_then(|init| init.expression().ok())
                    .map(|expression| expression.syntax().text_trimmed_range());
                result.push((name, binding, default));
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
                let Ok(AnyJsBinding::JsIdentifierBinding(binding)) = property.identifier() else {
                    continue;
                };
                let Ok(token) = binding.name_token() else {
                    continue;
                };
                let default = property
                    .init()
                    .and_then(|init| init.expression().ok())
                    .map(|expression| expression.syntax().text_trimmed_range());
                result.push((token.token_text_trimmed(), binding, default));
            }
            _ => {}
        }
    }
    result
}

/// Returns `true` for a node that starts a new function, whose `return`
/// statements belong to it and not to the function around it.
fn starts_function(kind: JsSyntaxKind) -> bool {
    AnyFunctionLike::can_cast(kind)
        || matches!(
            kind,
            JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
                | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
                | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
                | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
        )
}

/// Returns the object literals a function returns, and whether every value
/// it returns is one.
///
/// Nested functions are not entered.
fn returned_objects(function: &AnyFunctionLike) -> (Vec<JsObjectExpression>, bool) {
    let mut expressions = Vec::new();
    match function.body() {
        // `() => ({ ... })` returns its body.
        Ok(AnyJsFunctionBody::AnyJsExpression(expression)) => expressions.push(expression),
        Ok(AnyJsFunctionBody::JsFunctionBody(body)) => {
            let mut preorder = body.syntax().preorder();
            while let Some(event) = preorder.next() {
                let WalkEvent::Enter(node) = event else {
                    continue;
                };
                if starts_function(node.kind()) {
                    preorder.skip_subtree();
                    continue;
                }
                if let Some(statement) = JsReturnStatement::cast(node)
                    && let Some(argument) = statement.argument()
                {
                    expressions.push(argument);
                }
            }
        }
        Err(_) => return (Vec::new(), false),
    }
    let mut objects = Vec::new();
    let mut all_objects = true;
    for expression in expressions {
        match expression.inner_expression() {
            Some(AnyJsExpression::JsObjectExpression(object)) => objects.push(object),
            _ => all_objects = false,
        }
    }
    (objects, all_objects)
}

/// Collects the string literal types `ty` is made of. Returns `false` when
/// part of the type is not a string literal.
fn string_literal_types(ty: &AnyTsType, names: &mut Vec<(TokenText, TextRange)>) -> bool {
    match ty {
        AnyTsType::TsStringLiteralType(literal) => {
            let Ok(name) = literal.inner_string_text() else {
                return false;
            };
            names.push((name, literal.syntax().text_trimmed_range()));
            true
        }
        AnyTsType::TsUnionType(union) => union
            .types()
            .iter()
            .all(|ty| ty.is_ok_and(|ty| string_literal_types(&ty, names))),
        AnyTsType::TsParenthesizedType(parenthesized) => parenthesized
            .ty()
            .is_ok_and(|ty| string_literal_types(&ty, names)),
        _ => false,
    }
}

/// Maps a TypeScript type to the runtime types a prop of that type accepts.
fn type_set_of(ty: &AnyTsType) -> TypeSet {
    let mut types = TypeSet::empty();
    match ty {
        AnyTsType::TsBooleanType(_) | AnyTsType::TsBooleanLiteralType(_) => {
            types.insert(PropType::Boolean);
        }
        AnyTsType::TsStringType(_)
        | AnyTsType::TsStringLiteralType(_)
        | AnyTsType::TsTemplateLiteralType(_) => types.insert(PropType::String),
        AnyTsType::TsNumberType(_) | AnyTsType::TsNumberLiteralType(_) => {
            types.insert(PropType::Number);
        }
        AnyTsType::TsBigintType(_) | AnyTsType::TsBigintLiteralType(_) => {
            types.insert(PropType::BigInt);
        }
        AnyTsType::TsSymbolType(_) => types.insert(PropType::Symbol),
        AnyTsType::TsArrayType(_) | AnyTsType::TsTupleType(_) => types.insert(PropType::Array),
        AnyTsType::TsFunctionType(_) => types.insert(PropType::Function),
        AnyTsType::TsObjectType(_) => types.insert(PropType::Object),
        AnyTsType::TsParenthesizedType(parenthesized) => match parenthesized.ty() {
            Ok(ty) => types.union(type_set_of(&ty)),
            Err(_) => types.insert(PropType::Unknown),
        },
        AnyTsType::TsUnionType(union) => {
            for ty in union.types().iter() {
                match ty {
                    Ok(ty) => types.union(type_set_of(&ty)),
                    Err(_) => types.insert(PropType::Unknown),
                }
            }
        }
        _ => types.insert(PropType::Unknown),
    }
    types
}
