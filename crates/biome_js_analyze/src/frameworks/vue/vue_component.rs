use crate::frameworks::vue::vue_call::{is_vue_api_reference, is_vue_compiler_macro_call};
use crate::services::semantic::Semantic;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsArrayElement, AnyJsBinding, AnyJsBindingPattern,
    AnyJsCombinedSpecifier, AnyJsExpression, AnyJsImportClause, AnyJsLiteralExpression,
    AnyJsModuleItem, AnyJsNamedImportSpecifier, AnyJsObjectBindingPatternMember, AnyJsObjectMember,
    AnyJsStatement, AnyTsType, AnyTsTypeMember, JsArrayBindingPattern, JsCallExpression,
    JsDefaultImportSpecifier, JsExportDefaultExpressionClause, JsFileSource, JsFunctionDeclaration,
    JsIdentifierBinding, JsMethodObjectMember, JsModule, JsNamedImportSpecifier,
    JsNamedImportSpecifiers, JsNamespaceImportSpecifier, JsObjectBindingPattern,
    JsPropertyObjectMember, JsShorthandNamedImportSpecifier, JsStringLiteralExpression,
    JsVariableDeclarator, TsIdentifierBinding, TsInterfaceDeclaration,
    TsPropertySignatureTypeMember, TsTypeAliasDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText, declare_node_union};

use enumflags2::{BitFlags, bitflags};

pub type VueComponentQuery = Semantic<AnyPotentialVueComponent>;

declare_node_union! {
    pub AnyPotentialVueComponent =
        JsModule
        | JsExportDefaultExpressionClause
        | JsCallExpression
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
pub enum VueDeclarationCollectionFilter {
    Prop = 1 << 0,
    Setup = 1 << 1,
    SetupImport = 1 << 2,
    Data = 1 << 3,
    Method = 1 << 4,
    Computed = 1 << 5,
}

pub enum VueComponent {
    /// This is referred to as "normal script" Vue component.
    /// <script> export default { props: [ ... ], data: { ... }, ... }; </script>
    Normal(VueNormalComponent),
    /// A component may be defined right in the createApp call.
    /// createApp({ props: [ ... ], ... });
    CreateApp(VueCreateApp),
    /// Define Component is one of the ways to define a Vue component.
    /// defineComponent({ props: [ ... ], ... });
    DefineComponent(VueDefineComponent),
    /// Setup component is a Vue 3 component that uses the Composition API.
    /// <script setup> defineProps({ ... }); const someData = { ... }; </script>
    /// FIXME: Remove `allow(dead_code)` when <script setup> is supported.
    #[expect(dead_code)]
    Setup(VueSetupComponent),
}

impl VueComponent {
    pub fn from_potential_component(
        potential_component: &AnyPotentialVueComponent,
        model: &SemanticModel,
        source: &JsFileSource,
    ) -> Option<Self> {
        match potential_component {
            AnyPotentialVueComponent::JsModule(_js_module) => {
                // FIXME: <script setup> is not supported yet
                // let embedding_kind = source.as_embedding_kind();
                // if !embedding_kind.is_vue_setup() {
                //     return None;
                // }
                // Some(VueComponent::Setup(VueSetupComponent {
                //     model: model.clone(),
                //     js_module: js_module.clone(),
                // }))
                None
            }
            AnyPotentialVueComponent::JsExportDefaultExpressionClause(
                default_expression_clause,
            ) => {
                let embedding_kind = source.as_embedding_kind();
                if !embedding_kind.is_vue() {
                    return None;
                }
                Some(Self::Normal(VueNormalComponent {
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
                } else {
                    None
                }
            }
        }
    }
}

pub struct VueNormalComponent {
    default_expression_clause: JsExportDefaultExpressionClause,
}

pub struct VueCreateApp {
    call_expression: JsCallExpression,
}

pub struct VueDefineComponent {
    call_expression: JsCallExpression,
}

pub struct VueSetupComponent {
    model: SemanticModel,
    js_module: JsModule,
}

pub trait VueComponentDeclarations {
    fn declarations(&self, filter: BitFlags<VueDeclarationCollectionFilter>)
    -> Vec<VueDeclaration>;
}

impl VueComponentDeclarations for VueComponent {
    fn declarations(
        &self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        match self {
            Self::Normal(component) => component.declarations(filter),
            Self::CreateApp(component) => component.declarations(filter),
            Self::DefineComponent(component) => component.declarations(filter),
            Self::Setup(component) => component.declarations(filter),
        }
    }
}

impl VueComponentDeclarations for VueSetupComponent {
    fn declarations(
        &self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        let model = &self.model;
        let mut result = Vec::new();
        for item in self.js_module.items() {
            match item {
                AnyJsModuleItem::AnyJsStatement(statement) => match statement {
                    AnyJsStatement::JsExpressionStatement(expression_statement) => {
                        if !filter.contains(VueDeclarationCollectionFilter::Prop) {
                            continue;
                        }
                        if let Ok(expression) = expression_statement.expression() {
                            result.extend(expression.collect_vue_declarations(model))
                        }
                    }
                    AnyJsStatement::JsVariableStatement(variable_statement) => {
                        if let Ok(declaration) = variable_statement.declaration() {
                            for declarator in declaration.declarators().iter().flatten() {
                                if filter.contains(VueDeclarationCollectionFilter::Setup) {
                                    if let Ok(declarations) =
                                        declarator.id().map(|id| id.collect_vue_declarations(model))
                                    {
                                        result.extend(declarations);
                                    }
                                }
                                if filter.contains(VueDeclarationCollectionFilter::Prop) {
                                    if let Some(declarations) = declarator
                                        .initializer()
                                        .and_then(|initializer| initializer.expression().ok())
                                        .map(|expression| {
                                            expression.collect_vue_declarations(model)
                                        })
                                    {
                                        result.extend(declarations);
                                    }
                                }
                            }
                        }
                    }
                    AnyJsStatement::JsFunctionDeclaration(function_declaration) => {
                        if !filter.contains(VueDeclarationCollectionFilter::Setup) {
                            continue;
                        }
                        result.push(VueDeclaration::Setup(
                            AnyVueSetupDeclaration::JsFunctionDeclaration(
                                function_declaration.clone(),
                            ),
                        ));
                    }
                    _ => {}
                },
                // Imports are automatically added to the setup scope
                AnyJsModuleItem::JsImport(import) => {
                    if !filter.contains(VueDeclarationCollectionFilter::SetupImport) {
                        continue;
                    }
                    result.extend(
                        import
                            .import_clause()
                            .ok()
                            .map(|import_clause| import_clause.collect_vue_declarations(model))
                            .unwrap_or_default(),
                    );
                }
                // Ignore exports in setup components
                AnyJsModuleItem::JsExport(_) => {}
            }
        }
        result
    }
}

impl VueComponentDeclarations for VueNormalComponent {
    fn declarations(
        &self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        let Ok(component_definition) = self.default_expression_clause.expression() else {
            return vec![];
        };
        collect_vue_declarations_from_component_definition(&component_definition, filter)
    }
}

impl VueComponentDeclarations for VueCreateApp {
    fn declarations(
        &self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        let Some(args) = self
            .call_expression
            .arguments()
            .ok()
            .map(|arguments| arguments.args())
        else {
            return vec![];
        };
        // createApp({ props: [...] });
        let Some(Ok(first_argument)) = args.first() else {
            return vec![];
        };
        let Some(expression) = first_argument.as_any_js_expression() else {
            return vec![];
        };
        collect_vue_declarations_from_component_definition(expression, filter)
    }
}

impl VueComponentDeclarations for VueDefineComponent {
    fn declarations(
        &self,
        filter: BitFlags<VueDeclarationCollectionFilter>,
    ) -> Vec<VueDeclaration> {
        let Some(args) = self
            .call_expression
            .arguments()
            .ok()
            .map(|arguments| arguments.args())
        else {
            return vec![];
        };
        // defineComponent(setup, { props: [...] });
        let Some(Ok(first_argument)) = args.iter().nth(1) else {
            return vec![];
        };
        let Some(expression) = first_argument.as_any_js_expression() else {
            return vec![];
        };
        collect_vue_declarations_from_component_definition(expression, filter)
    }
}

pub enum VueDeclaration {
    Prop(AnyVuePropDeclaration),
    Setup(AnyVueSetupDeclaration),
    SetupImport(AnyVueSetupImportDeclaration),
    Data(JsPropertyObjectMember),
    Method(AnyVueMethod),
    Computed(AnyVueMethod),
}

pub trait VueDeclarationName {
    /// Returns the name of the declaration.
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
            Self::Data(object_property) => object_property.declaration_name(),
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
            Self::Data(object_property) => object_property.declaration_name_range(),
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
}

impl VueDeclarationName for AnyVueSetupDeclaration {
    fn declaration_name(&self) -> Option<TokenText> {
        match self {
            Self::JsIdentifierBinding(ident) => Some(ident.name_token().ok()?.token_text()),
            Self::JsFunctionDeclaration(function) => Some(
                function
                    .id()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?
                    .token_text(),
            ),
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

trait VueCollectDeclarations {
    /// Returns a list of Vue declarations found in the component.
    /// This includes props, setup variables, data properties, methods, and computed properties.
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration>;
}

fn collect_vue_declarations_from_component_definition(
    expression: &AnyJsExpression,
    filter: BitFlags<VueDeclarationCollectionFilter>,
) -> Vec<VueDeclaration> {
    println!(
        "expression {} {:?}",
        expression.to_trimmed_text().text(),
        expression.syntax()
    );
    let Some(AnyJsExpression::JsObjectExpression(object_expression)) =
        expression.inner_expression()
    else {
        return vec![];
    };
    println!(
        "inner_expression {}",
        object_expression.to_trimmed_text().text()
    );
    let mut result = Vec::new();
    for group_object_member in object_expression.members().iter().flatten() {
        let Some(prop_name) = group_object_member.name() else {
            continue;
        };
        match prop_name.text() {
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
                // FIXME: Support data() method, need control flow analysis for that
                let AnyJsObjectMember::JsPropertyObjectMember(property) = group_object_member
                else {
                    continue;
                };
                if let Ok(expression) = property.value() {
                    result.extend(collect_data_declarations(&expression));
                }
            }
            _ => {}
        }
    }
    result
}

impl VueCollectDeclarations for JsVariableDeclarator {
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
        let mut result = self
            .id()
            .map(|id| id.collect_vue_declarations(model))
            .unwrap_or_default();
        result.extend(
            self.initializer()
                .and_then(|initializer| initializer.expression().ok())
                .map(|expression| expression.collect_vue_declarations(model))
                .unwrap_or_default(),
        );
        result
    }
}

impl VueCollectDeclarations for AnyJsExpression {
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
        self.inner_expression()
            .and_then(|expression| match expression {
                Self::JsCallExpression(call) => {
                    if !is_vue_compiler_macro_call(&call, "defineProps") {
                        return None;
                    }
                    Some(get_props_declarations_from_call(&call, model))
                }
                _ => None,
            })
            .unwrap_or_default()
    }
}

fn get_props_declarations_from_call(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> Vec<VueDeclaration> {
    let mut result = vec![];
    if let Ok(arguments) = call.arguments() {
        if let Some(Ok(first_argument)) = arguments.args().first() {
            if let Some(expression) = first_argument.as_any_js_expression() {
                result.extend(collect_props_declarations_from_expression(expression));
            }
        }
    }
    if let Some(type_arguments) = call.type_arguments() {
        if let Some(Ok(props_type)) = type_arguments.ts_type_argument_list().iter().next() {
            result.extend(collect_props_declarations_from_type(&props_type, model));
        }
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
                let binding = model.binding(reference_identifier)?;
                let first_write = binding.all_writes().next()?;
                let write_syntax = first_write.syntax();
                if let Some(identifier_binding) = TsIdentifierBinding::cast_ref(write_syntax) {
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

fn collect_data_declarations(expression: &AnyJsExpression) -> Vec<VueDeclaration> {
    let AnyJsExpression::JsObjectExpression(object_expression) = expression else {
        return vec![];
    };
    object_expression
        .members()
        .iter()
        .flatten()
        .filter_map(|member| match member {
            AnyJsObjectMember::JsPropertyObjectMember(property) => {
                Some(VueDeclaration::Data(property))
            }
            _ => None,
        })
        .collect()
}

impl VueCollectDeclarations for AnyJsBinding {
    fn collect_vue_declarations(&self, _model: &SemanticModel) -> Vec<VueDeclaration> {
        match self {
            Self::JsIdentifierBinding(identifier) => {
                vec![VueDeclaration::Setup(
                    AnyVueSetupDeclaration::JsIdentifierBinding(identifier.clone()),
                )]
            }
            _ => vec![],
        }
    }
}

impl VueCollectDeclarations for AnyJsBindingPattern {
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
        match self {
            Self::AnyJsBinding(any_binding) => any_binding.collect_vue_declarations(model),
            Self::JsArrayBindingPattern(array_pattern) => {
                array_pattern.collect_vue_declarations(model)
            }
            Self::JsObjectBindingPattern(object_pattern) => {
                object_pattern.collect_vue_declarations(model)
            }
        }
    }
}

impl VueCollectDeclarations for JsObjectBindingPattern {
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
        self.properties()
            .iter()
            .flatten()
            .flat_map(|property| property.collect_vue_declarations(model))
            .collect()
    }
}

impl VueCollectDeclarations for AnyJsObjectBindingPatternMember {
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
        match self {
            Self::JsObjectBindingPatternProperty(property) => property
                .pattern()
                .map(|pattern| pattern.collect_vue_declarations(model))
                .unwrap_or_default(),
            Self::JsObjectBindingPatternRest(rest) => rest
                .binding()
                .map(|binding| binding.collect_vue_declarations(model))
                .unwrap_or_default(),
            Self::JsObjectBindingPatternShorthandProperty(property) => property
                .identifier()
                .map(|identifier| identifier.collect_vue_declarations(model))
                .unwrap_or_default(),
            _ => vec![],
        }
    }
}

impl VueCollectDeclarations for JsArrayBindingPattern {
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
        self.elements()
            .iter()
            .flatten()
            .flat_map(|element| element.collect_vue_declarations(model))
            .collect()
    }
}

impl VueCollectDeclarations for AnyJsArrayBindingPatternElement {
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
        match self {
            Self::JsArrayBindingPatternElement(element) => element
                .pattern()
                .map(|pattern| pattern.collect_vue_declarations(model))
                .unwrap_or_default(),
            Self::JsArrayBindingPatternRestElement(rest) => rest
                .pattern()
                .map(|pattern| pattern.collect_vue_declarations(model))
                .unwrap_or_default(),
            Self::JsArrayHole(_) => vec![],
        }
    }
}

impl VueCollectDeclarations for AnyJsImportClause {
    fn collect_vue_declarations(&self, model: &SemanticModel) -> Vec<VueDeclaration> {
        match self {
            Self::JsImportCombinedClause(combined_clause) => {
                let mut declarations = Vec::new();
                if let Ok(default_specifier) = combined_clause.default_specifier() {
                    declarations.extend(default_specifier.collect_vue_declarations(model))
                }

                match combined_clause.specifier() {
                    Ok(AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named_specifiers)) => {
                        declarations.extend(named_specifiers.collect_vue_declarations(model));
                    }
                    Ok(AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(namespace_specifier)) => {
                        declarations.extend(namespace_specifier.collect_vue_declarations(model));
                    }
                    _ => {}
                }
                declarations
            }
            Self::JsImportDefaultClause(default_clause) => default_clause
                .default_specifier()
                .ok()
                .map(|specifier| specifier.collect_vue_declarations(model))
                .unwrap_or_default(),
            Self::JsImportNamedClause(named_clause) => named_clause
                .named_specifiers()
                .ok()
                .map(|specifier| specifier.collect_vue_declarations(model))
                .unwrap_or_default(),
            Self::JsImportNamespaceClause(namespace_clause) => namespace_clause
                .namespace_specifier()
                .ok()
                .map(|specifier| specifier.collect_vue_declarations(model))
                .unwrap_or_default(),
            Self::JsImportBareClause(_) => vec![],
        }
    }
}

impl VueCollectDeclarations for JsDefaultImportSpecifier {
    fn collect_vue_declarations(&self, _model: &SemanticModel) -> Vec<VueDeclaration> {
        vec![VueDeclaration::SetupImport(
            AnyVueSetupImportDeclaration::JsDefaultImportSpecifier(self.clone()),
        )]
    }
}

impl VueCollectDeclarations for JsNamespaceImportSpecifier {
    fn collect_vue_declarations(&self, _model: &SemanticModel) -> Vec<VueDeclaration> {
        vec![VueDeclaration::SetupImport(
            AnyVueSetupImportDeclaration::JsNamespaceImportSpecifier(self.clone()),
        )]
    }
}

impl VueCollectDeclarations for JsNamedImportSpecifiers {
    fn collect_vue_declarations(&self, _model: &SemanticModel) -> Vec<VueDeclaration> {
        self.specifiers()
            .iter()
            .flatten()
            .filter_map(|specifier| match specifier {
                AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                    Some(VueDeclaration::SetupImport(
                        AnyVueSetupImportDeclaration::JsNamedImportSpecifier(specifier.clone()),
                    ))
                }
                AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => {
                    Some(VueDeclaration::SetupImport(
                        AnyVueSetupImportDeclaration::JsShorthandNamedImportSpecifier(
                            specifier.clone(),
                        ),
                    ))
                }
                AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
            })
            .collect()
    }
}
