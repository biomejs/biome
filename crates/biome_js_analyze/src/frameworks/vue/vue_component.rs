use crate::frameworks::vue::vue_call::{is_vue_api_reference, is_vue_compiler_macro_call};
use crate::services::semantic::Semantic;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsFunctionBody, AnyJsLiteralExpression,
    AnyJsObjectMember, AnyJsStatement, AnyTsType, AnyTsTypeMember, JsArrowFunctionExpression,
    JsCallExpression, JsDefaultImportSpecifier, JsExportDefaultExpressionClause, JsFileSource,
    JsFunctionDeclaration, JsFunctionExpression, JsIdentifierBinding, JsMethodObjectMember,
    JsNamedImportSpecifier, JsNamespaceImportSpecifier, JsPropertyObjectMember,
    JsShorthandNamedImportSpecifier, JsStringLiteralExpression, JsSyntaxKind, JsSyntaxNode,
    TsIdentifierBinding, TsInterfaceDeclaration, TsPropertySignatureTypeMember,
    TsTypeAliasDeclaration,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, TextRange, TokenText, declare_node_union,
};
use camino::Utf8Path;
use std::iter;

use crate::utils::rename::RenamableNode;
use enumflags2::{BitFlags, bitflags};

mod component_name;

pub use component_name::VueComponentName;

/// VueComponentQuery is a query type that can be used to find Vue components.
/// It can match any potential Vue component.
pub type VueComponentQuery = Semantic<AnyPotentialVueComponent>;

declare_node_union! {
    pub AnyPotentialVueComponent = JsExportDefaultExpressionClause
        | JsCallExpression
}

/// A filter to collect declarations from a vue component.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
pub enum VueDeclarationCollectionFilter {
    /// A prop of a Vue component.
    /// Can be defined via `props` option in Options API or `defineProps` in `<script setup>`.
    Prop = 1 << 0,
    /// A setup variable in a Vue component.
    /// Can be defined using `const`, `let` or `function`.
    /// FIXME: Not supported yet. Will be supported when `<script setup>` is implemented.
    Setup = 1 << 1,
    /// A setup import in a Vue component.
    /// FIXME: Not supported yet. Will be supported when `<script setup>` is implemented.
    SetupImport = 1 << 2,
    /// Data properties in a Vue component.
    /// Can be defined via `data` option in Options API.
    /// TODO: Support data() method, need control flow analysis for that.
    Data = 1 << 3,
    /// Nuxt.js Async Data properties in a Vue component.
    /// Can be defined via `asyncData` option in Options API.
    /// TODO: Support asyncData() method, need control flow analysis for that.
    AsyncData = 1 << 4,
    /// Methods in a Vue component.
    Method = 1 << 5,
    /// Computed properties in a Vue component.
    Computed = 1 << 6,
}

pub struct VueComponent<'a> {
    kind: AnyVueComponent,
    path: &'a Utf8Path,
}

impl<'a> VueComponent<'a> {
    pub fn new(path: &'a Utf8Path, kind: AnyVueComponent) -> Self {
        Self { path, kind }
    }

    pub fn kind(&self) -> &AnyVueComponent {
        &self.kind
    }

    pub fn from_potential_component(
        potential_component: &AnyPotentialVueComponent,
        model: &SemanticModel,
        source: &JsFileSource,
        path: &'a Utf8Path,
    ) -> Option<Self> {
        let component =
            AnyVueComponent::from_potential_component(potential_component, model, source)?;
        Some(Self::new(path, component))
    }

    /// The name of the component, if it can be determined.
    ///
    /// Derived from the file name if the name is not explicitly set in the component definition.
    pub fn name(&self) -> Option<VueComponentName<'a>> {
        self.kind()
            .component_name()
            .map(VueComponentName::FromComponent)
            .or_else(|| {
                // filename fallback only for Single-File Components
                if self.path.extension() == Some("vue") {
                    self.path.file_stem().map(VueComponentName::FromPath)
                } else {
                    None
                }
            })
    }
}

/// An abstraction over multiple ways to define a vue component.
/// Provides a list of declarations for a component.
pub enum AnyVueComponent {
    /// Options API style Vue component.
    /// ```html
    /// <script> export default { props: [ ... ], data: { ... }, ... }; </script>
    /// ```
    OptionsApi(VueOptionsApiComponent),
    /// A component may be defined right in the createApp call.
    /// ```js
    /// createApp({ props: [ ... ], ... });
    /// ```
    CreateApp(VueCreateApp),
    /// Define Component is one of the ways to define a Vue component.
    /// ```js
    /// defineComponent((...) => { ... }, { props: [ ... ], ... });
    /// defineComponent({ props: [ ... ], ... });
    /// ```
    DefineComponent(VueDefineComponent),
    /// Setup component is a Vue 3 component that uses the Composition API.
    /// ```html
    /// <script setup> defineProps({ ... }); const someData = { ... }; </script>
    /// ```
    Setup(VueSetupComponent),
}

impl AnyVueComponent {
    pub fn from_potential_component(
        potential_component: &AnyPotentialVueComponent,
        model: &SemanticModel,
        source: &JsFileSource,
    ) -> Option<Self> {
        match potential_component {
            // FIXME: <script setup> is not supported yet
            // AnyPotentialVueComponent::JsModule(_js_module) => {
            //     let embedding_kind = source.as_embedding_kind();
            //     if !embedding_kind.is_vue_setup() {
            //         return None;
            //     }
            //     Some(VueComponent::Setup(VueSetupComponent {
            //         model: model.clone(),
            //         js_module: js_module.clone(),
            //     }))
            // }
            AnyPotentialVueComponent::JsExportDefaultExpressionClause(
                default_expression_clause,
            ) => {
                if !source.as_embedding_kind().is_vue() {
                    return None;
                }
                Some(Self::OptionsApi(VueOptionsApiComponent {
                    default_expression_clause: default_expression_clause.clone(),
                }))
            }
            AnyPotentialVueComponent::JsCallExpression(call_expression) => {
                let callee = call_expression
                    .callee()
                    .ok()
                    .and_then(|callee| callee.inner_expression())?;
                if is_vue_api_reference(&callee, model, "defineComponent") {
                    Some(Self::DefineComponent(VueDefineComponent {
                        call_expression: call_expression.clone(),
                    }))
                } else if is_vue_api_reference(&callee, model, "createApp") {
                    Some(Self::CreateApp(VueCreateApp {
                        call_expression: call_expression.clone(),
                    }))
                } else if is_vue_compiler_macro_call(call_expression, model, "defineProps")
                    && source.as_embedding_kind().is_vue()
                {
                    // FIXME: Since <script setup> is not supported yet, we only handle props
                    Some(Self::Setup(VueSetupComponent {
                        model: model.clone(),
                        define_props_call: call_expression.clone(),
                    }))
                } else {
                    None
                }
            }
        }
    }
}

/// Options API style Vue component.
/// ```html
/// <script> export default { props: [ ... ], data: { ... }, ... }; </script>
/// ```
pub struct VueOptionsApiComponent {
    default_expression_clause: JsExportDefaultExpressionClause,
}

/// A component may be defined right in the createApp call.
/// ```js
/// createApp({ props: [ ... ], ... });
/// ```
pub struct VueCreateApp {
    call_expression: JsCallExpression,
}

/// Define Component is one of the ways to define a Vue component.
/// ```js
/// defineComponent((...) => { ... }, { props: [ ... ], ... });
/// defineComponent({ props: [ ... ], ... });
/// ```
pub struct VueDefineComponent {
    call_expression: JsCallExpression,
}

/// Setup component is a Vue 3 component that uses the Composition API.
/// ```html
/// <script setup> defineProps({ ... }); const someData = { ... }; </script>
/// ```
pub struct VueSetupComponent {
    model: SemanticModel,
    // FIXME: change when <script setup> is supported
    // js_module: JsModule,
    define_props_call: JsCallExpression,
}

pub trait VueComponentDeclarations {
    /// Returns a list of Vue declarations found in the component.
    /// This includes:
    /// * Props
    /// * Setup variables
    /// * Data properties
    /// * Methods
    /// * Computed properties
    fn declarations(&self, filter: BitFlags<VueDeclarationCollectionFilter>)
    -> Vec<VueDeclaration>;

    /// Returns the `data` group.
    ///
    /// This is either a `data()` method or an object expression containing data properties.
    /// Examples:
    /// ```js
    /// export default {
    ///   data() { return { ... } } // this
    /// };
    /// defineComponent({
    ///   data: { ... } // this
    /// });
    /// ```
    ///
    fn data_declarations_group(&self) -> Option<AnyVueDataDeclarationsGroup>;
}

declare_node_union! {
    pub AnyVueDataDeclarationsGroup = JsPropertyObjectMember | JsMethodObjectMember
}

impl VueComponentDeclarations for VueComponent<'_> {
    fn declarations(
        &'_ self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        self.kind.declarations(filter)
    }

    fn data_declarations_group(&self) -> Option<AnyVueDataDeclarationsGroup> {
        self.kind().data_declarations_group()
    }
}

impl VueComponentDeclarations for AnyVueComponent {
    fn declarations(
        &self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        match self {
            Self::OptionsApi(component) => component.declarations(filter),
            Self::CreateApp(component) => component.declarations(filter),
            Self::DefineComponent(component) => component.declarations(filter),
            Self::Setup(component) => component.declarations(filter),
        }
    }

    fn data_declarations_group(&self) -> Option<AnyVueDataDeclarationsGroup> {
        match self {
            Self::OptionsApi(component) => component.data_declarations_group(),
            Self::CreateApp(component) => component.data_declarations_group(),
            Self::DefineComponent(component) => component.data_declarations_group(),
            Self::Setup(component) => component.data_declarations_group(),
        }
    }
}

impl VueComponentDeclarations for VueSetupComponent {
    fn declarations(
        &self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        if !filter.contains(VueDeclarationCollectionFilter::Prop) {
            return vec![];
        }
        get_props_declarations_from_call(&self.define_props_call, &self.model)
        // FIXME: Uncomment when <script setup> is supported
        // let model = &self.model;
        // let mut result = Vec::new();
        // for item in self.js_module.items() {
        //     match item {
        //         AnyJsModuleItem::AnyJsStatement(statement) => match statement {
        //             AnyJsStatement::JsExpressionStatement(expression_statement) => {
        //                 if !filter.contains(VueDeclarationCollectionFilter::Prop) {
        //                     continue;
        //                 }
        //                 if let Ok(expression) = expression_statement.expression() {
        //                     result.extend(expression.collect_vue_declarations(model))
        //                 }
        //             }
        //             AnyJsStatement::JsVariableStatement(variable_statement) => {
        //                 if let Ok(declaration) = variable_statement.declaration() {
        //                     for declarator in declaration.declarators().iter().flatten() {
        //                         if filter.contains(VueDeclarationCollectionFilter::Setup) {
        //                             if let Ok(declarations) =
        //                                 declarator.id().map(|id| id.collect_vue_declarations(model))
        //                             {
        //                                 result.extend(declarations);
        //                             }
        //                         }
        //                         if filter.contains(VueDeclarationCollectionFilter::Prop) {
        //                             if let Some(declarations) = declarator
        //                                 .initializer()
        //                                 .and_then(|initializer| initializer.expression().ok())
        //                                 .map(|expression| {
        //                                     expression.collect_vue_declarations(model)
        //                                 })
        //                             {
        //                                 result.extend(declarations);
        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //             AnyJsStatement::JsFunctionDeclaration(function_declaration) => {
        //                 if !filter.contains(VueDeclarationCollectionFilter::Setup) {
        //                     continue;
        //                 }
        //                 result.push(VueDeclaration::Setup(
        //                     AnyVueSetupDeclaration::JsFunctionDeclaration(
        //                         function_declaration.clone(),
        //                     ),
        //                 ));
        //             }
        //             _ => {}
        //         },
        //         // Imports are automatically added to the setup scope
        //         AnyJsModuleItem::JsImport(import) => {
        //             if !filter.contains(VueDeclarationCollectionFilter::SetupImport) {
        //                 continue;
        //             }
        //             result.extend(
        //                 import
        //                     .import_clause()
        //                     .ok()
        //                     .map(|import_clause| import_clause.collect_vue_declarations(model))
        //                     .unwrap_or_default(),
        //             );
        //         }
        //         // Ignore exports in setup components
        //         AnyJsModuleItem::JsExport(_) => {}
        //     }
        // }
        // result
    }

    fn data_declarations_group(&self) -> Option<AnyVueDataDeclarationsGroup> {
        None
    }
}

impl VueOptionsApiBasedComponent for VueOptionsApiComponent {
    fn definition_expression(&self) -> Option<AnyJsExpression> {
        self.default_expression_clause.expression().ok()
    }

    fn setup_func(&self) -> Option<AnyJsExpression> {
        None
    }
}

impl VueOptionsApiBasedComponent for VueCreateApp {
    fn definition_expression(&self) -> Option<AnyJsExpression> {
        let args = self
            .call_expression
            .arguments()
            .ok()
            .map(|arguments| arguments.args())?;

        // createApp({ props: [...] });
        let first_argument = args.first()?.ok()?;
        first_argument.as_any_js_expression().cloned()
    }

    fn setup_func(&self) -> Option<AnyJsExpression> {
        None
    }
}

impl VueOptionsApiBasedComponent for VueDefineComponent {
    fn definition_expression(&self) -> Option<AnyJsExpression> {
        let args = self
            .call_expression
            .arguments()
            .ok()
            .map(|arguments| arguments.args())?;

        // defineComponent({ props: [...] });
        // defineComponent(setup, { props: [...] });
        let last_argument = args.last()?.ok()?;
        last_argument.as_any_js_expression().cloned()
    }

    fn setup_func(&self) -> Option<AnyJsExpression> {
        let args = self
            .call_expression
            .arguments()
            .ok()
            .map(|arguments| arguments.args())?;

        if args.len() == 2 {
            // defineComponent(setup, { props: [...] });
            let first_argument = args.first()?.ok()?;
            first_argument.as_any_js_expression().cloned()
        } else {
            // defineComponent({ props: [...] });
            // If there is only one argument, it is the definition object.
            // We don't have a setup function in this case.
            None
        }
    }
}

pub trait VueOptionsApiBasedComponent {
    /// Returns the potential object expression that defines the component.
    /// Examples:
    /// ```js
    /// defineComponent(
    ///     { props: [...] } // this
    /// );
    /// createApp(
    ///     { props: [...] } // this
    /// );
    /// export default { props: [...] }; // this
    /// ```
    fn definition_expression(&self) -> Option<AnyJsExpression>;

    /// Returns the expression representing the setup function, if it exists.
    ///
    /// Example:
    /// ```js
    /// defineComponent(
    ///    (props, context) => { ... }, // this
    ///    { props: [...] }
    /// );
    /// ```
    fn setup_func(&self) -> Option<AnyJsExpression>;

    fn iter_declaration_groups(&self) -> Box<dyn Iterator<Item = (TokenText, AnyJsObjectMember)>>
    where
        Self: Sized,
    {
        let Some(object_expression) = self
            .definition_expression()
            .and_then(|expression| expression.inner_expression())
            .and_then(|expression| expression.as_js_object_expression().cloned())
        else {
            return Box::new(std::iter::empty());
        };

        Box::new(
            object_expression
                .members()
                .into_iter()
                .flatten()
                .filter_map(|member| member.name().map(|name| (name, member))),
        )
    }
}

impl<T: VueOptionsApiBasedComponent> VueComponentDeclarations for T {
    fn declarations(
        &self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        let mut result = vec![];

        if filter.contains(VueDeclarationCollectionFilter::Setup)
            && let Some(setup_func) = self.setup_func()
        {
            result.extend(
                iter_func_return_properties(setup_func.syntax())
                    .map(AnyVueSetupDeclaration::JsPropertyObjectMember)
                    .map(VueDeclaration::Setup),
            );
        }

        for (name, group_object_member) in self.iter_declaration_groups() {
            match name.text() {
                "props" => {
                    if !filter.contains(VueDeclarationCollectionFilter::Prop) {
                        continue;
                    }
                    let AnyJsObjectMember::JsPropertyObjectMember(property) = group_object_member
                    else {
                        continue;
                    };
                    if let Ok(expression) = property.value() {
                        result.extend(collect_props_declarations_from_expression(&expression));
                    }
                }
                "computed" => {
                    if !filter.contains(VueDeclarationCollectionFilter::Computed) {
                        continue;
                    }
                    let AnyJsObjectMember::JsPropertyObjectMember(property) = group_object_member
                    else {
                        continue;
                    };
                    if let Ok(expression) = property.value() {
                        result.extend(collect_computed_and_method_declarations(&expression));
                    }
                }
                "methods" => {
                    if !filter.contains(VueDeclarationCollectionFilter::Method) {
                        continue;
                    }
                    let AnyJsObjectMember::JsPropertyObjectMember(property) = group_object_member
                    else {
                        continue;
                    };
                    if let Ok(expression) = property.value() {
                        result.extend(collect_computed_and_method_declarations(&expression));
                    }
                }
                "data" => {
                    if !filter.contains(VueDeclarationCollectionFilter::Data) {
                        continue;
                    }
                    result.extend(
                        iter_declaration_group_properties(group_object_member)
                            .map(VueDeclaration::Data),
                    );
                }
                "asyncData" => {
                    if !filter.contains(VueDeclarationCollectionFilter::AsyncData) {
                        continue;
                    }
                    result.extend(
                        iter_declaration_group_properties(group_object_member)
                            .map(VueDeclaration::AsyncData),
                    );
                }
                "setup" => {
                    if !filter.contains(VueDeclarationCollectionFilter::Setup) {
                        continue;
                    }
                    result.extend(
                        iter_declaration_group_properties(group_object_member)
                            .map(AnyVueSetupDeclaration::JsPropertyObjectMember)
                            .map(VueDeclaration::Setup),
                    );
                }
                _ => {}
            }
        }
        result
    }

    fn data_declarations_group(&self) -> Option<AnyVueDataDeclarationsGroup> {
        self.iter_declaration_groups().find_map(|(name, member)| {
            if name.text() == "data" {
                match member {
                    AnyJsObjectMember::JsMethodObjectMember(method) => {
                        Some(AnyVueDataDeclarationsGroup::JsMethodObjectMember(method))
                    }
                    AnyJsObjectMember::JsPropertyObjectMember(property) => Some(
                        AnyVueDataDeclarationsGroup::JsPropertyObjectMember(property),
                    ),
                    _ => None,
                }
            } else {
                None
            }
        })
    }
}

pub enum VueDeclaration {
    /// A prop of a Vue component.
    /// Can be defined via `props` option in Options API or `defineProps` in `<script setup>`.
    Prop(AnyVuePropDeclaration),
    /// A setup variable in a Vue component.
    /// Can be defined using `const`, `let` or `function`.
    /// FIXME: Not supported yet. Will be supported when `<script setup>` is implemented.
    Setup(AnyVueSetupDeclaration),
    /// A setup import in a Vue component.
    /// FIXME: Not supported yet. Will be supported when `<script setup>` is implemented.
    SetupImport(AnyVueSetupImportDeclaration),
    /// Data properties in a Vue component.
    /// Can be defined via `data` option in Options API.
    Data(JsPropertyObjectMember),
    /// Nuxt.js Async Data properties in a Vue component.
    /// Can be defined via `asyncData` option in Options API.
    AsyncData(JsPropertyObjectMember),
    /// Methods in a Vue component.
    Method(AnyVueMethod),
    /// Computed properties in a Vue component.
    Computed(AnyVueMethod),
}

pub trait VueDeclarationName {
    /// Returns the name of the declaration. I.e. name of a prop, setup variable, method, etc.
    fn declaration_name(&self) -> Option<TokenText>;
    /// Returns the range of the declaration name in the source code.
    fn declaration_name_range(&self) -> Option<TextRange>;
}

impl VueDeclarationName for VueDeclaration {
    fn declaration_name(&self) -> Option<TokenText> {
        match self {
            Self::Prop(prop) => prop.declaration_name(),
            Self::Setup(setup) => setup.declaration_name(),
            Self::SetupImport(setup) => setup.declaration_name(),
            Self::Data(object_property) | Self::AsyncData(object_property) => {
                object_property.declaration_name()
            }
            Self::Method(method_or_property) | Self::Computed(method_or_property) => {
                method_or_property.declaration_name()
            }
        }
    }

    fn declaration_name_range(&self) -> Option<TextRange> {
        match self {
            Self::Prop(prop) => prop.declaration_name_range(),
            Self::Setup(setup) => setup.declaration_name_range(),
            Self::SetupImport(setup) => setup.declaration_name_range(),
            Self::Data(object_property) | Self::AsyncData(object_property) => {
                object_property.declaration_name_range()
            }
            Self::Method(method_or_property) | Self::Computed(method_or_property) => {
                method_or_property.declaration_name_range()
            }
        }
    }
}

declare_node_union! {
    pub AnyVuePropDeclaration =
        JsStringLiteralExpression
        | JsPropertyObjectMember
        | TsPropertySignatureTypeMember
}

impl VueDeclarationName for AnyVuePropDeclaration {
    fn declaration_name(&self) -> Option<TokenText> {
        match self {
            Self::JsStringLiteralExpression(literal) => literal.inner_string_text().ok(),
            Self::JsPropertyObjectMember(property) => property.name().ok()?.name(),
            Self::TsPropertySignatureTypeMember(property) => property.name().ok()?.name(),
        }
    }

    fn declaration_name_range(&self) -> Option<TextRange> {
        Some(match self {
            Self::JsStringLiteralExpression(literal) => literal.range(),
            Self::JsPropertyObjectMember(property) => property.name().ok()?.range(),
            Self::TsPropertySignatureTypeMember(property) => property.name().ok()?.range(),
        })
    }
}

declare_node_union! {
    pub AnyVueMethod =
        JsMethodObjectMember
        | JsPropertyObjectMember
}

impl VueDeclarationName for AnyVueMethod {
    fn declaration_name(&self) -> Option<TokenText> {
        match self {
            Self::JsMethodObjectMember(method) => method.name().ok()?.name(),
            Self::JsPropertyObjectMember(property) => property.name().ok()?.name(),
        }
    }

    fn declaration_name_range(&self) -> Option<TextRange> {
        Some(match self {
            Self::JsMethodObjectMember(method) => method.name().ok()?.range(),
            Self::JsPropertyObjectMember(property) => property.name().ok()?.range(),
        })
    }
}

declare_node_union! {
    pub AnyVueSetupDeclaration =
        // Identifier binding as part of a variable declaration pattern
        JsIdentifierBinding
        | JsFunctionDeclaration
        | JsPropertyObjectMember
}

impl VueDeclarationName for AnyVueSetupDeclaration {
    fn declaration_name(&self) -> Option<TokenText> {
        match self {
            Self::JsIdentifierBinding(ident) => Some(ident.name_token().ok()?.token_text_trimmed()),
            Self::JsFunctionDeclaration(function) => Some(
                function
                    .id()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?
                    .token_text_trimmed(),
            ),
            Self::JsPropertyObjectMember(property) => property.name().ok()?.name(),
        }
    }

    fn declaration_name_range(&self) -> Option<TextRange> {
        Some(match self {
            Self::JsIdentifierBinding(ident) => ident.name_token().ok()?.text_trimmed_range(),
            Self::JsFunctionDeclaration(function) => function
                .id()
                .ok()?
                .as_js_identifier_binding()?
                .name_token()
                .ok()?
                .text_trimmed_range(),
            Self::JsPropertyObjectMember(property) => property.name().ok()?.range(),
        })
    }
}

declare_node_union! {
    /// Imports are exposed into vue template same as variables/functions
    pub AnyVueSetupImportDeclaration =
        JsShorthandNamedImportSpecifier
        | JsNamedImportSpecifier
        | JsDefaultImportSpecifier
        | JsNamespaceImportSpecifier

}

impl VueDeclarationName for AnyVueSetupImportDeclaration {
    fn declaration_name(&self) -> Option<TokenText> {
        let local_name = match self {
            Self::JsShorthandNamedImportSpecifier(import) => import.local_name(),
            Self::JsNamedImportSpecifier(import) => import.local_name(),
            Self::JsDefaultImportSpecifier(import) => import.local_name(),
            Self::JsNamespaceImportSpecifier(import) => import.local_name(),
        };
        Some(
            local_name
                .ok()?
                .as_js_identifier_binding()?
                .name_token()
                .ok()?
                .token_text(),
        )
    }

    fn declaration_name_range(&self) -> Option<TextRange> {
        let local_name = match self {
            Self::JsShorthandNamedImportSpecifier(import) => import.local_name(),
            Self::JsNamedImportSpecifier(import) => import.local_name(),
            Self::JsDefaultImportSpecifier(import) => import.local_name(),
            Self::JsNamespaceImportSpecifier(import) => import.local_name(),
        };
        Some(
            local_name
                .ok()?
                .as_js_identifier_binding()?
                .name_token()
                .ok()?
                .text_trimmed_range(),
        )
    }
}

impl VueDeclarationName for JsPropertyObjectMember {
    fn declaration_name(&self) -> Option<TokenText> {
        self.name().ok()?.name()
    }

    fn declaration_name_range(&self) -> Option<TextRange> {
        Some(self.name().ok()?.range())
    }
}

fn get_props_declarations_from_call(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> Vec<VueDeclaration> {
    let mut result = vec![];
    if let Ok(arguments) = call.arguments()
        && let Some(Ok(first_argument)) = arguments.args().first()
        && let Some(expression) = first_argument.as_any_js_expression()
    {
        result.extend(collect_props_declarations_from_expression(expression));
    }
    if let Some(type_arguments) = call.type_arguments()
        && let Some(Ok(props_type)) = type_arguments.ts_type_argument_list().iter().next()
    {
        result.extend(collect_props_declarations_from_type(&props_type, model));
    }
    result
}

fn collect_props_declarations_from_type(
    any_type: &AnyTsType,
    model: &SemanticModel,
) -> Vec<VueDeclaration> {
    match any_type {
        AnyTsType::TsObjectType(object_type) => {
            let mut result = vec![];
            for member in object_type.members() {
                if let AnyTsTypeMember::TsPropertySignatureTypeMember(property) = member {
                    result.push(VueDeclaration::Prop(
                        AnyVuePropDeclaration::TsPropertySignatureTypeMember(property.clone()),
                    ));
                }
            }
            result
        }
        AnyTsType::TsParenthesizedType(parenthesized_type) => parenthesized_type
            .ty()
            .map(|ty| collect_props_declarations_from_type(&ty, model))
            .unwrap_or_default(),
        AnyTsType::TsReferenceType(reference_type) => reference_type
            .name()
            .ok()
            .and_then(|name| {
                let reference_identifier = name.as_js_reference_identifier()?;
                let binding_syntax = reference_identifier.binding(model)?;
                if let Some(identifier_binding) = TsIdentifierBinding::cast_ref(&binding_syntax) {
                    if let Some(type_alias) = identifier_binding.parent::<TsTypeAliasDeclaration>()
                    {
                        let ty = type_alias.ty().ok()?;
                        return Some(collect_props_declarations_from_type(&ty, model));
                    } else if let Some(interface) =
                        identifier_binding.parent::<TsInterfaceDeclaration>()
                    {
                        let mut result = vec![];
                        for member in interface.members() {
                            if let AnyTsTypeMember::TsPropertySignatureTypeMember(property) = member
                            {
                                result.push(VueDeclaration::Prop(
                                    AnyVuePropDeclaration::TsPropertySignatureTypeMember(property),
                                ))
                            }
                        }
                        return Some(result);
                    }
                }
                None
            })
            .unwrap_or_default(),
        _ => vec![],
    }
}

fn collect_props_declarations_from_expression(expression: &AnyJsExpression) -> Vec<VueDeclaration> {
    match expression {
        AnyJsExpression::JsArrayExpression(array_expression) => array_expression
            .elements()
            .iter()
            .flatten()
            .filter_map(|item| {
                if let AnyJsArrayElement::AnyJsExpression(
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(
                            string_literal_expression,
                        ),
                    ),
                ) = item
                {
                    Some(VueDeclaration::Prop(
                        AnyVuePropDeclaration::JsStringLiteralExpression(
                            string_literal_expression.clone(),
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect(),
        AnyJsExpression::JsObjectExpression(object_expression) => object_expression
            .members()
            .iter()
            .flatten()
            .filter_map(|item| {
                let property = item.as_js_property_object_member()?;
                Some(VueDeclaration::Prop(
                    AnyVuePropDeclaration::JsPropertyObjectMember(property.clone()),
                ))
            })
            .collect(),
        _ => vec![],
    }
}

fn collect_computed_and_method_declarations(expression: &AnyJsExpression) -> Vec<VueDeclaration> {
    let AnyJsExpression::JsObjectExpression(object_expression) = expression else {
        return vec![];
    };
    object_expression
        .members()
        .iter()
        .flatten()
        .filter_map(|member| match member {
            AnyJsObjectMember::JsPropertyObjectMember(property) => Some(VueDeclaration::Computed(
                AnyVueMethod::JsPropertyObjectMember(property),
            )),
            AnyJsObjectMember::JsMethodObjectMember(method) => Some(VueDeclaration::Computed(
                AnyVueMethod::JsMethodObjectMember(method),
            )),
            _ => None,
        })
        .collect()
}

/// Iterates over properties in a declaration group.
/// This can be a group defined as an object expression or a method that returns an object.
/// For example:
/// ```js
/// export default {
///   data: {
///     someData: 'value',
///   },
///   async asyncData() {
///     return {
///        someAsyncData: 'value',
///     };
///   },
/// }
fn iter_declaration_group_properties(
    container: AnyJsObjectMember,
) -> Box<dyn Iterator<Item = JsPropertyObjectMember>> {
    match container {
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            if let Ok(value) = property.value()
                && let Some(expression) = value.inner_expression()
            {
                if let AnyJsExpression::JsObjectExpression(object_expression) = expression {
                    return Box::new(object_expression.members().iter().filter_map(|member| {
                        if let Ok(AnyJsObjectMember::JsPropertyObjectMember(property)) = member {
                            Some(property)
                        } else {
                            None
                        }
                    }));
                } else {
                    return Box::new(iter_func_return_properties(expression.syntax()));
                }
            }
        }
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            return Box::new(iter_func_return_properties(method.syntax()));
        }
        _ => {}
    }
    Box::new(iter::empty())
}

/// Finds all object properties in a function's return expressions.
fn iter_func_return_properties(
    func: &JsSyntaxNode,
) -> Box<dyn Iterator<Item = JsPropertyObjectMember>> {
    Box::new(
        iter_func_return_expressions(func)
            .filter_map(|expression| {
                if let Some(AnyJsExpression::JsObjectExpression(object_expression)) =
                    expression.inner_expression()
                {
                    Some(object_expression.members().iter().filter_map(|member| {
                        if let Ok(AnyJsObjectMember::JsPropertyObjectMember(property)) = member {
                            Some(property)
                        } else {
                            None
                        }
                    }))
                } else {
                    None
                }
            })
            .flatten(),
    )
}

/// Since we can't currently combine ControlFlowGraph with SemanticModel,
/// we need to analyze the function body to find return values.
fn iter_func_return_expressions(func: &JsSyntaxNode) -> Box<dyn Iterator<Item = AnyJsExpression>> {
    match func.kind() {
        JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => {
            let method = JsMethodObjectMember::cast_ref(func).unwrap();
            if let Ok(body) = method.body() {
                iter_func_block_return_expressions(body.statements().syntax())
            } else {
                Box::new(iter::empty())
            }
        }
        JsSyntaxKind::JS_FUNCTION_EXPRESSION => {
            let function_expression = JsFunctionExpression::cast_ref(func).unwrap();
            if let Ok(body) = function_expression.body() {
                iter_func_block_return_expressions(body.statements().syntax())
            } else {
                Box::new(iter::empty())
            }
        }
        JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
            let arrow_function = JsArrowFunctionExpression::cast_ref(func).unwrap();
            if let Ok(body) = arrow_function.body() {
                match body {
                    AnyJsFunctionBody::AnyJsExpression(expression) => {
                        Box::new(iter::once(expression.clone()))
                    }
                    AnyJsFunctionBody::JsFunctionBody(body) => {
                        iter_func_block_return_expressions(body.statements().syntax())
                    }
                }
            } else {
                Box::new(iter::empty())
            }
        }
        _ => Box::new(iter::empty()),
    }
}

/// Iterates over all return expressions in a function block.
/// Doesn't go into nested functions, but handles control flow statements like `if`, `try`, etc.
fn iter_func_block_return_expressions(
    syntax: &JsSyntaxNode,
) -> Box<dyn Iterator<Item = AnyJsExpression>> {
    if syntax.kind() == JsSyntaxKind::JS_STATEMENT_LIST {
        return Box::new(
            syntax
                .children()
                .flat_map(|child| iter_func_block_return_expressions(&child)),
        );
    }
    let Some(statement) = AnyJsStatement::cast_ref(syntax) else {
        return Box::new(iter::empty());
    };
    // Handle more specific cases first
    match statement {
        AnyJsStatement::JsReturnStatement(return_statement) => {
            return Box::new(return_statement.argument().into_iter());
        }
        AnyJsStatement::JsBlockStatement(block) => {
            return iter_func_block_return_expressions(block.statements().syntax());
        }
        AnyJsStatement::JsIfStatement(if_statement) => {
            return Box::new(
                if_statement
                    .consequent()
                    .into_iter()
                    .flat_map(|consequent| iter_func_block_return_expressions(consequent.syntax()))
                    .chain(
                        if_statement
                            .else_clause()
                            .into_iter()
                            .filter_map(|else_clause| {
                                else_clause
                                    .alternate()
                                    .ok()
                                    .map(|alt| iter_func_block_return_expressions(alt.syntax()))
                            })
                            .flatten(),
                    ),
            );
        }
        AnyJsStatement::JsTryStatement(try_statement) => {
            return Box::new(
                try_statement
                    .body()
                    .into_iter()
                    .flat_map(|body| iter_func_block_return_expressions(body.syntax()))
                    .chain(
                        try_statement
                            .catch_clause()
                            .into_iter()
                            .filter_map(|catch_clause| {
                                catch_clause
                                    .body()
                                    .ok()
                                    .map(|body| iter_func_block_return_expressions(body.syntax()))
                            })
                            .flatten(),
                    ),
            );
        }
        AnyJsStatement::JsTryFinallyStatement(try_finally_statement) => {
            return Box::new(
                try_finally_statement
                    .body()
                    .into_iter()
                    .flat_map(|body| iter_func_block_return_expressions(body.syntax()))
                    .chain(
                        try_finally_statement
                            .catch_clause()
                            .into_iter()
                            .filter_map(|catch_clause| {
                                catch_clause
                                    .body()
                                    .ok()
                                    .map(|body| iter_func_block_return_expressions(body.syntax()))
                            })
                            .flatten(),
                    )
                    .chain(
                        try_finally_statement
                            .finally_clause()
                            .into_iter()
                            .filter_map(|finally_clause| {
                                finally_clause
                                    .body()
                                    .ok()
                                    .map(|body| iter_func_block_return_expressions(body.syntax()))
                            })
                            .flatten(),
                    ),
            );
        }
        AnyJsStatement::JsSwitchStatement(switch_statement) => {
            return Box::new(
                switch_statement.cases().iter().flat_map(|case| {
                    iter_func_block_return_expressions(case.consequent().syntax())
                }),
            );
        }
        _ => {}
    }
    // Handle similar blocks, i.e. `do`, `for`, `while`, etc.
    let Some(statement_body) = (match statement {
        AnyJsStatement::JsDoWhileStatement(statement) => statement.body().ok(),
        AnyJsStatement::JsForInStatement(statement) => statement.body().ok(),
        AnyJsStatement::JsForOfStatement(statement) => statement.body().ok(),
        AnyJsStatement::JsForStatement(statement) => statement.body().ok(),
        AnyJsStatement::JsLabeledStatement(statement) => statement.body().ok(),
        AnyJsStatement::JsWhileStatement(statement) => statement.body().ok(),
        AnyJsStatement::JsWithStatement(statement) => statement.body().ok(),
        _ => None,
    }) else {
        return Box::new(iter::empty());
    };

    iter_func_block_return_expressions(statement_body.syntax())
}

// FIXME: Uncomment when <script setup> is supported
// trait VueCollectSetupDeclarations {
//     /// Returns a list of Vue setup declarations found in the component.
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration>;
// }
//
// impl VueCollectSetupDeclarations for JsVariableDeclarator {
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
//         let mut result = self
//             .id()
//             .map(|id| id.collect_vue_setup_declarations(model))
//             .unwrap_or_default();
//         result.extend(
//             self.initializer()
//                 .and_then(|initializer| initializer.expression().ok())
//                 .map(|expression| expression.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//         );
//         result
//     }
// }
//
// impl VueCollectSetupDeclarations for AnyJsExpression {
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
//         self.inner_expression()
//             .and_then(|expression| match expression {
//                 Self::JsCallExpression(call) => {
//                     if !is_vue_compiler_macro_call(&call, model, "defineProps") {
//                         return None;
//                     }
//                     Some(get_props_declarations_from_call(&call, model))
//                 }
//                 _ => None,
//             })
//             .unwrap_or_default()
//     }
// }
//
// impl VueCollectSetupDeclarations for AnyJsBinding {
//     fn collect_vue_setup_declarations(&self, _model: &SemanticModel) -> Vec<VueDeclaration> {
//         match self {
//             Self::JsIdentifierBinding(identifier) => {
//                 vec![VueDeclaration::Setup(
//                     AnyVueSetupDeclaration::JsIdentifierBinding(identifier.clone()),
//                 )]
//             }
//             _ => vec![],
//         }
//     }
// }
//
// impl VueCollectSetupDeclarations for AnyJsBindingPattern {
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
//         match self {
//             Self::AnyJsBinding(any_binding) => any_binding.collect_vue_setup_declarations(model),
//             Self::JsArrayBindingPattern(array_pattern) => {
//                 array_pattern.collect_vue_setup_declarations(model)
//             }
//             Self::JsObjectBindingPattern(object_pattern) => {
//                 object_pattern.collect_vue_setup_declarations(model)
//             }
//         }
//     }
// }
//
// impl VueCollectSetupDeclarations for JsObjectBindingPattern {
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
//         self.properties()
//             .iter()
//             .flatten()
//             .flat_map(|property| property.collect_vue_setup_declarations(model))
//             .collect()
//     }
// }
//
// impl VueCollectSetupDeclarations for AnyJsObjectBindingPatternMember {
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
//         match self {
//             Self::JsObjectBindingPatternProperty(property) => property
//                 .pattern()
//                 .map(|pattern| pattern.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//             Self::JsObjectBindingPatternRest(rest) => rest
//                 .binding()
//                 .map(|binding| binding.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//             Self::JsObjectBindingPatternShorthandProperty(property) => property
//                 .identifier()
//                 .map(|identifier| identifier.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//             _ => vec![],
//         }
//     }
// }
//
// impl VueCollectSetupDeclarations for JsArrayBindingPattern {
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
//         self.elements()
//             .iter()
//             .flatten()
//             .flat_map(|element| element.collect_vue_setup_declarations(model))
//             .collect()
//     }
// }
//
// impl VueCollectSetupDeclarations for AnyJsArrayBindingPatternElement {
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
//         match self {
//             Self::JsArrayBindingPatternElement(element) => element
//                 .pattern()
//                 .map(|pattern| pattern.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//             Self::JsArrayBindingPatternRestElement(rest) => rest
//                 .pattern()
//                 .map(|pattern| pattern.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//             Self::JsArrayHole(_) => vec![],
//         }
//     }
// }
//
// impl VueCollectSetupDeclarations for AnyJsImportClause {
//     fn collect_vue_setup_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
//         match self {
//             Self::JsImportCombinedClause(combined_clause) => {
//                 let mut declarations = Vec::new();
//                 if let Ok(default_specifier) = combined_clause.default_specifier() {
//                     declarations.extend(default_specifier.collect_vue_setup_declarations(model))
//                 }
//
//                 match combined_clause.specifier() {
//                     Ok(AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named_specifiers)) => {
//                         declarations.extend(named_specifiers.collect_vue_setup_declarations(model));
//                     }
//                     Ok(AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(namespace_specifier)) => {
//                         declarations
//                             .extend(namespace_specifier.collect_vue_setup_declarations(model));
//                     }
//                     _ => {}
//                 }
//                 declarations
//             }
//             Self::JsImportDefaultClause(default_clause) => default_clause
//                 .default_specifier()
//                 .ok()
//                 .map(|specifier| specifier.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//             Self::JsImportNamedClause(named_clause) => named_clause
//                 .named_specifiers()
//                 .ok()
//                 .map(|specifier| specifier.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//             Self::JsImportNamespaceClause(namespace_clause) => namespace_clause
//                 .namespace_specifier()
//                 .ok()
//                 .map(|specifier| specifier.collect_vue_setup_declarations(model))
//                 .unwrap_or_default(),
//             Self::JsImportBareClause(_) => vec![],
//         }
//     }
// }
//
// impl VueCollectSetupDeclarations for JsDefaultImportSpecifier {
//     fn collect_vue_setup_declarations(&self, _model: &SemanticModel) -> Vec<VueDeclaration> {
//         vec![VueDeclaration::SetupImport(
//             AnyVueSetupImportDeclaration::JsDefaultImportSpecifier(self.clone()),
//         )]
//     }
// }
//
// impl VueCollectSetupDeclarations for JsNamespaceImportSpecifier {
//     fn collect_vue_setup_declarations(&self, _model: &SemanticModel) -> Vec<VueDeclaration> {
//         vec![VueDeclaration::SetupImport(
//             AnyVueSetupImportDeclaration::JsNamespaceImportSpecifier(self.clone()),
//         )]
//     }
// }
//
// impl VueCollectSetupDeclarations for JsNamedImportSpecifiers {
//     fn collect_vue_setup_declarations(&self, _model: &SemanticModel) -> Vec<VueDeclaration> {
//         self.specifiers()
//             .iter()
//             .flatten()
//             .filter_map(|specifier| match specifier {
//                 AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
//                     Some(VueDeclaration::SetupImport(
//                         AnyVueSetupImportDeclaration::JsNamedImportSpecifier(specifier.clone()),
//                     ))
//                 }
//                 AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => {
//                     Some(VueDeclaration::SetupImport(
//                         AnyVueSetupImportDeclaration::JsShorthandNamedImportSpecifier(
//                             specifier.clone(),
//                         ),
//                     ))
//                 }
//                 AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
//             })
//             .collect()
//     }
// }
