//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_graphql_syntax::{
    GraphqlSyntaxElement as SyntaxElement, GraphqlSyntaxNode as SyntaxNode,
    GraphqlSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn graphql_alias(value: GraphqlLiteralName, colon_token: SyntaxToken) -> GraphqlAlias {
    GraphqlAlias::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ALIAS,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
        ],
    ))
}
pub fn graphql_argument(
    name: GraphqlLiteralName,
    colon_token: SyntaxToken,
    value: AnyGraphqlValue,
) -> GraphqlArgument {
    GraphqlArgument::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ARGUMENT,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn graphql_arguments(
    l_paren_token: SyntaxToken,
    arguments: GraphqlArgumentList,
    r_paren_token: SyntaxToken,
) -> GraphqlArguments {
    GraphqlArguments::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(arguments.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn graphql_arguments_definition(
    l_paren_token: SyntaxToken,
    arguments: GraphqlArgumentDefinitionList,
    r_paren_token: SyntaxToken,
) -> GraphqlArgumentsDefinition {
    GraphqlArgumentsDefinition::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ARGUMENTS_DEFINITION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(arguments.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn graphql_boolean_value(value_token_token: SyntaxToken) -> GraphqlBooleanValue {
    GraphqlBooleanValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_BOOLEAN_VALUE,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn graphql_default_value(eq_token: SyntaxToken, value: AnyGraphqlValue) -> GraphqlDefaultValue {
    GraphqlDefaultValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_DEFAULT_VALUE,
        [
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn graphql_description(graphql_string_value: GraphqlStringValue) -> GraphqlDescription {
    GraphqlDescription::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_DESCRIPTION,
        [Some(SyntaxElement::Node(
            graphql_string_value.into_syntax(),
        ))],
    ))
}
pub fn graphql_directive(
    at_token: SyntaxToken,
    name: GraphqlNameReference,
) -> GraphqlDirectiveBuilder {
    GraphqlDirectiveBuilder {
        at_token,
        name,
        arguments: None,
    }
}
pub struct GraphqlDirectiveBuilder {
    at_token: SyntaxToken,
    name: GraphqlNameReference,
    arguments: Option<GraphqlArguments>,
}
impl GraphqlDirectiveBuilder {
    pub fn with_arguments(mut self, arguments: GraphqlArguments) -> Self {
        self.arguments = Some(arguments);
        self
    }
    pub fn build(self) -> GraphqlDirective {
        GraphqlDirective::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_DIRECTIVE,
            [
                Some(SyntaxElement::Token(self.at_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_directive_definition(
    directive_token: SyntaxToken,
    at_token: SyntaxToken,
    name: GraphqlNameBinding,
    on_token: SyntaxToken,
    locations: GraphqlDirectiveLocationList,
) -> GraphqlDirectiveDefinitionBuilder {
    GraphqlDirectiveDefinitionBuilder {
        directive_token,
        at_token,
        name,
        on_token,
        locations,
        description: None,
        arguments: None,
        repeatable_token: None,
        bitwise_or_token: None,
    }
}
pub struct GraphqlDirectiveDefinitionBuilder {
    directive_token: SyntaxToken,
    at_token: SyntaxToken,
    name: GraphqlNameBinding,
    on_token: SyntaxToken,
    locations: GraphqlDirectiveLocationList,
    description: Option<GraphqlDescription>,
    arguments: Option<GraphqlArgumentsDefinition>,
    repeatable_token: Option<SyntaxToken>,
    bitwise_or_token: Option<SyntaxToken>,
}
impl GraphqlDirectiveDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_arguments(mut self, arguments: GraphqlArgumentsDefinition) -> Self {
        self.arguments = Some(arguments);
        self
    }
    pub fn with_repeatable_token(mut self, repeatable_token: SyntaxToken) -> Self {
        self.repeatable_token = Some(repeatable_token);
        self
    }
    pub fn with_bitwise_or_token(mut self, bitwise_or_token: SyntaxToken) -> Self {
        self.bitwise_or_token = Some(bitwise_or_token);
        self
    }
    pub fn build(self) -> GraphqlDirectiveDefinition {
        GraphqlDirectiveDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_DIRECTIVE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.directive_token)),
                Some(SyntaxElement::Token(self.at_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.repeatable_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.on_token)),
                self.bitwise_or_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.locations.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_directive_location(value_token_token: SyntaxToken) -> GraphqlDirectiveLocation {
    GraphqlDirectiveLocation::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_DIRECTIVE_LOCATION,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn graphql_enum_type_definition(
    enum_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
) -> GraphqlEnumTypeDefinitionBuilder {
    GraphqlEnumTypeDefinitionBuilder {
        enum_token,
        name,
        directives,
        description: None,
        enum_values: None,
    }
}
pub struct GraphqlEnumTypeDefinitionBuilder {
    enum_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
    enum_values: Option<GraphqlEnumValuesDefinition>,
}
impl GraphqlEnumTypeDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_enum_values(mut self, enum_values: GraphqlEnumValuesDefinition) -> Self {
        self.enum_values = Some(enum_values);
        self
    }
    pub fn build(self) -> GraphqlEnumTypeDefinition {
        GraphqlEnumTypeDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_ENUM_TYPE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.enum_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.enum_values
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_enum_type_extension(
    extend_token: SyntaxToken,
    enum_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
) -> GraphqlEnumTypeExtensionBuilder {
    GraphqlEnumTypeExtensionBuilder {
        extend_token,
        enum_token,
        name,
        directives,
        enum_values: None,
    }
}
pub struct GraphqlEnumTypeExtensionBuilder {
    extend_token: SyntaxToken,
    enum_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
    enum_values: Option<GraphqlEnumValuesDefinition>,
}
impl GraphqlEnumTypeExtensionBuilder {
    pub fn with_enum_values(mut self, enum_values: GraphqlEnumValuesDefinition) -> Self {
        self.enum_values = Some(enum_values);
        self
    }
    pub fn build(self) -> GraphqlEnumTypeExtension {
        GraphqlEnumTypeExtension::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_ENUM_TYPE_EXTENSION,
            [
                Some(SyntaxElement::Token(self.extend_token)),
                Some(SyntaxElement::Token(self.enum_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.enum_values
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_enum_value(value: GraphqlLiteralName) -> GraphqlEnumValue {
    GraphqlEnumValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ENUM_VALUE,
        [Some(SyntaxElement::Node(value.into_syntax()))],
    ))
}
pub fn graphql_enum_value_definition(
    value: GraphqlLiteralName,
    directives: GraphqlDirectiveList,
) -> GraphqlEnumValueDefinitionBuilder {
    GraphqlEnumValueDefinitionBuilder {
        value,
        directives,
        description: None,
    }
}
pub struct GraphqlEnumValueDefinitionBuilder {
    value: GraphqlLiteralName,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
}
impl GraphqlEnumValueDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn build(self) -> GraphqlEnumValueDefinition {
        GraphqlEnumValueDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_ENUM_VALUE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_enum_values_definition(
    l_curly_token: SyntaxToken,
    values: GraphqlEnumValueList,
    r_curly_token: SyntaxToken,
) -> GraphqlEnumValuesDefinition {
    GraphqlEnumValuesDefinition::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ENUM_VALUES_DEFINITION,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(values.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn graphql_field(
    name: GraphqlLiteralName,
    directives: GraphqlDirectiveList,
) -> GraphqlFieldBuilder {
    GraphqlFieldBuilder {
        name,
        directives,
        alias: None,
        arguments: None,
        selection_set: None,
    }
}
pub struct GraphqlFieldBuilder {
    name: GraphqlLiteralName,
    directives: GraphqlDirectiveList,
    alias: Option<GraphqlAlias>,
    arguments: Option<GraphqlArguments>,
    selection_set: Option<GraphqlSelectionSet>,
}
impl GraphqlFieldBuilder {
    pub fn with_alias(mut self, alias: GraphqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn with_arguments(mut self, arguments: GraphqlArguments) -> Self {
        self.arguments = Some(arguments);
        self
    }
    pub fn with_selection_set(mut self, selection_set: GraphqlSelectionSet) -> Self {
        self.selection_set = Some(selection_set);
        self
    }
    pub fn build(self) -> GraphqlField {
        GraphqlField::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_FIELD,
            [
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.selection_set
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_field_definition(
    name: GraphqlLiteralName,
    colon_token: SyntaxToken,
    ty: AnyGraphqlType,
    directives: GraphqlDirectiveList,
) -> GraphqlFieldDefinitionBuilder {
    GraphqlFieldDefinitionBuilder {
        name,
        colon_token,
        ty,
        directives,
        description: None,
        arguments: None,
    }
}
pub struct GraphqlFieldDefinitionBuilder {
    name: GraphqlLiteralName,
    colon_token: SyntaxToken,
    ty: AnyGraphqlType,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
    arguments: Option<GraphqlArgumentsDefinition>,
}
impl GraphqlFieldDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_arguments(mut self, arguments: GraphqlArgumentsDefinition) -> Self {
        self.arguments = Some(arguments);
        self
    }
    pub fn build(self) -> GraphqlFieldDefinition {
        GraphqlFieldDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_FIELD_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.arguments
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_fields_definition(
    l_curly_token: SyntaxToken,
    fields: GraphqlFieldDefinitionList,
    r_curly_token: SyntaxToken,
) -> GraphqlFieldsDefinition {
    GraphqlFieldsDefinition::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_FIELDS_DEFINITION,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(fields.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn graphql_float_value(graphql_float_literal_token: SyntaxToken) -> GraphqlFloatValue {
    GraphqlFloatValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_FLOAT_VALUE,
        [Some(SyntaxElement::Token(graphql_float_literal_token))],
    ))
}
pub fn graphql_fragment_definition(
    fragment_token: SyntaxToken,
    name: GraphqlNameBinding,
    type_condition: GraphqlTypeCondition,
    directives: GraphqlDirectiveList,
    selection_set: GraphqlSelectionSet,
) -> GraphqlFragmentDefinition {
    GraphqlFragmentDefinition::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_FRAGMENT_DEFINITION,
        [
            Some(SyntaxElement::Token(fragment_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(type_condition.into_syntax())),
            Some(SyntaxElement::Node(directives.into_syntax())),
            Some(SyntaxElement::Node(selection_set.into_syntax())),
        ],
    ))
}
pub fn graphql_fragment_spread(
    dotdotdot_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
) -> GraphqlFragmentSpread {
    GraphqlFragmentSpread::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_FRAGMENT_SPREAD,
        [
            Some(SyntaxElement::Token(dotdotdot_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(directives.into_syntax())),
        ],
    ))
}
pub fn graphql_implements_interfaces(
    implements_token: SyntaxToken,
    interfaces: GraphqlImplementsInterfaceList,
) -> GraphqlImplementsInterfacesBuilder {
    GraphqlImplementsInterfacesBuilder {
        implements_token,
        interfaces,
        amp_token: None,
    }
}
pub struct GraphqlImplementsInterfacesBuilder {
    implements_token: SyntaxToken,
    interfaces: GraphqlImplementsInterfaceList,
    amp_token: Option<SyntaxToken>,
}
impl GraphqlImplementsInterfacesBuilder {
    pub fn with_amp_token(mut self, amp_token: SyntaxToken) -> Self {
        self.amp_token = Some(amp_token);
        self
    }
    pub fn build(self) -> GraphqlImplementsInterfaces {
        GraphqlImplementsInterfaces::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_IMPLEMENTS_INTERFACES,
            [
                Some(SyntaxElement::Token(self.implements_token)),
                self.amp_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.interfaces.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_inline_fragment(
    dotdotdot_token: SyntaxToken,
    directives: GraphqlDirectiveList,
    selection_set: GraphqlSelectionSet,
) -> GraphqlInlineFragmentBuilder {
    GraphqlInlineFragmentBuilder {
        dotdotdot_token,
        directives,
        selection_set,
        type_condition: None,
    }
}
pub struct GraphqlInlineFragmentBuilder {
    dotdotdot_token: SyntaxToken,
    directives: GraphqlDirectiveList,
    selection_set: GraphqlSelectionSet,
    type_condition: Option<GraphqlTypeCondition>,
}
impl GraphqlInlineFragmentBuilder {
    pub fn with_type_condition(mut self, type_condition: GraphqlTypeCondition) -> Self {
        self.type_condition = Some(type_condition);
        self
    }
    pub fn build(self) -> GraphqlInlineFragment {
        GraphqlInlineFragment::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_INLINE_FRAGMENT,
            [
                Some(SyntaxElement::Token(self.dotdotdot_token)),
                self.type_condition
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                Some(SyntaxElement::Node(self.selection_set.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_input_fields_definition(
    l_curly_token: SyntaxToken,
    fields: GraphqlInputFieldList,
    r_curly_token: SyntaxToken,
) -> GraphqlInputFieldsDefinition {
    GraphqlInputFieldsDefinition::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_INPUT_FIELDS_DEFINITION,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(fields.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn graphql_input_object_type_definition(
    input_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
) -> GraphqlInputObjectTypeDefinitionBuilder {
    GraphqlInputObjectTypeDefinitionBuilder {
        input_token,
        name,
        directives,
        description: None,
        input_fields: None,
    }
}
pub struct GraphqlInputObjectTypeDefinitionBuilder {
    input_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
    input_fields: Option<GraphqlInputFieldsDefinition>,
}
impl GraphqlInputObjectTypeDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_input_fields(mut self, input_fields: GraphqlInputFieldsDefinition) -> Self {
        self.input_fields = Some(input_fields);
        self
    }
    pub fn build(self) -> GraphqlInputObjectTypeDefinition {
        GraphqlInputObjectTypeDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.input_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.input_fields
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_input_object_type_extension(
    extend_token: SyntaxToken,
    input_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
) -> GraphqlInputObjectTypeExtensionBuilder {
    GraphqlInputObjectTypeExtensionBuilder {
        extend_token,
        input_token,
        name,
        directives,
        input_fields: None,
    }
}
pub struct GraphqlInputObjectTypeExtensionBuilder {
    extend_token: SyntaxToken,
    input_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
    input_fields: Option<GraphqlInputFieldsDefinition>,
}
impl GraphqlInputObjectTypeExtensionBuilder {
    pub fn with_input_fields(mut self, input_fields: GraphqlInputFieldsDefinition) -> Self {
        self.input_fields = Some(input_fields);
        self
    }
    pub fn build(self) -> GraphqlInputObjectTypeExtension {
        GraphqlInputObjectTypeExtension::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION,
            [
                Some(SyntaxElement::Token(self.extend_token)),
                Some(SyntaxElement::Token(self.input_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.input_fields
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_input_value_definition(
    name: GraphqlLiteralName,
    colon_token: SyntaxToken,
    ty: AnyGraphqlType,
    directives: GraphqlDirectiveList,
) -> GraphqlInputValueDefinitionBuilder {
    GraphqlInputValueDefinitionBuilder {
        name,
        colon_token,
        ty,
        directives,
        description: None,
        default: None,
    }
}
pub struct GraphqlInputValueDefinitionBuilder {
    name: GraphqlLiteralName,
    colon_token: SyntaxToken,
    ty: AnyGraphqlType,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
    default: Option<GraphqlDefaultValue>,
}
impl GraphqlInputValueDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_default(mut self, default: GraphqlDefaultValue) -> Self {
        self.default = Some(default);
        self
    }
    pub fn build(self) -> GraphqlInputValueDefinition {
        GraphqlInputValueDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_INPUT_VALUE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                self.default
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_int_value(graphql_int_literal_token: SyntaxToken) -> GraphqlIntValue {
    GraphqlIntValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_INT_VALUE,
        [Some(SyntaxElement::Token(graphql_int_literal_token))],
    ))
}
pub fn graphql_interface_type_definition(
    interface_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
) -> GraphqlInterfaceTypeDefinitionBuilder {
    GraphqlInterfaceTypeDefinitionBuilder {
        interface_token,
        name,
        directives,
        description: None,
        implements: None,
        fields: None,
    }
}
pub struct GraphqlInterfaceTypeDefinitionBuilder {
    interface_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
    implements: Option<GraphqlImplementsInterfaces>,
    fields: Option<GraphqlFieldsDefinition>,
}
impl GraphqlInterfaceTypeDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_implements(mut self, implements: GraphqlImplementsInterfaces) -> Self {
        self.implements = Some(implements);
        self
    }
    pub fn with_fields(mut self, fields: GraphqlFieldsDefinition) -> Self {
        self.fields = Some(fields);
        self
    }
    pub fn build(self) -> GraphqlInterfaceTypeDefinition {
        GraphqlInterfaceTypeDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_INTERFACE_TYPE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.interface_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.implements
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.fields
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_interface_type_extension(
    extend_token: SyntaxToken,
    interface_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
) -> GraphqlInterfaceTypeExtensionBuilder {
    GraphqlInterfaceTypeExtensionBuilder {
        extend_token,
        interface_token,
        name,
        directives,
        implements: None,
        fields: None,
    }
}
pub struct GraphqlInterfaceTypeExtensionBuilder {
    extend_token: SyntaxToken,
    interface_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
    implements: Option<GraphqlImplementsInterfaces>,
    fields: Option<GraphqlFieldsDefinition>,
}
impl GraphqlInterfaceTypeExtensionBuilder {
    pub fn with_implements(mut self, implements: GraphqlImplementsInterfaces) -> Self {
        self.implements = Some(implements);
        self
    }
    pub fn with_fields(mut self, fields: GraphqlFieldsDefinition) -> Self {
        self.fields = Some(fields);
        self
    }
    pub fn build(self) -> GraphqlInterfaceTypeExtension {
        GraphqlInterfaceTypeExtension::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_INTERFACE_TYPE_EXTENSION,
            [
                Some(SyntaxElement::Token(self.extend_token)),
                Some(SyntaxElement::Token(self.interface_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.implements
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.fields
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_list_type(
    l_brack_token: SyntaxToken,
    element: AnyGraphqlType,
    r_brack_token: SyntaxToken,
) -> GraphqlListType {
    GraphqlListType::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_LIST_TYPE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(element.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn graphql_list_value(
    l_brack_token: SyntaxToken,
    elements: GraphqlListValueElementList,
    r_brack_token: SyntaxToken,
) -> GraphqlListValue {
    GraphqlListValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_LIST_VALUE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn graphql_literal_name(value_token: SyntaxToken) -> GraphqlLiteralName {
    GraphqlLiteralName::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_LITERAL_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn graphql_name_binding(value_token: SyntaxToken) -> GraphqlNameBinding {
    GraphqlNameBinding::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_NAME_BINDING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn graphql_name_reference(value_token: SyntaxToken) -> GraphqlNameReference {
    GraphqlNameReference::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_NAME_REFERENCE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn graphql_non_null_type(
    base: AnyGraphqlPrimitiveType,
    excl_token: SyntaxToken,
) -> GraphqlNonNullType {
    GraphqlNonNullType::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_NON_NULL_TYPE,
        [
            Some(SyntaxElement::Node(base.into_syntax())),
            Some(SyntaxElement::Token(excl_token)),
        ],
    ))
}
pub fn graphql_null_value(null_token: SyntaxToken) -> GraphqlNullValue {
    GraphqlNullValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_NULL_VALUE,
        [Some(SyntaxElement::Token(null_token))],
    ))
}
pub fn graphql_object_field(
    name: GraphqlLiteralName,
    colon_token: SyntaxToken,
    value: AnyGraphqlValue,
) -> GraphqlObjectField {
    GraphqlObjectField::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_OBJECT_FIELD,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn graphql_object_type_definition(
    type_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
) -> GraphqlObjectTypeDefinitionBuilder {
    GraphqlObjectTypeDefinitionBuilder {
        type_token,
        name,
        directives,
        description: None,
        implements: None,
        fields: None,
    }
}
pub struct GraphqlObjectTypeDefinitionBuilder {
    type_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
    implements: Option<GraphqlImplementsInterfaces>,
    fields: Option<GraphqlFieldsDefinition>,
}
impl GraphqlObjectTypeDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_implements(mut self, implements: GraphqlImplementsInterfaces) -> Self {
        self.implements = Some(implements);
        self
    }
    pub fn with_fields(mut self, fields: GraphqlFieldsDefinition) -> Self {
        self.fields = Some(fields);
        self
    }
    pub fn build(self) -> GraphqlObjectTypeDefinition {
        GraphqlObjectTypeDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_OBJECT_TYPE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.type_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.implements
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.fields
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_object_type_extension(
    extend_token: SyntaxToken,
    type_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
) -> GraphqlObjectTypeExtensionBuilder {
    GraphqlObjectTypeExtensionBuilder {
        extend_token,
        type_token,
        name,
        directives,
        implements: None,
        fields: None,
    }
}
pub struct GraphqlObjectTypeExtensionBuilder {
    extend_token: SyntaxToken,
    type_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
    implements: Option<GraphqlImplementsInterfaces>,
    fields: Option<GraphqlFieldsDefinition>,
}
impl GraphqlObjectTypeExtensionBuilder {
    pub fn with_implements(mut self, implements: GraphqlImplementsInterfaces) -> Self {
        self.implements = Some(implements);
        self
    }
    pub fn with_fields(mut self, fields: GraphqlFieldsDefinition) -> Self {
        self.fields = Some(fields);
        self
    }
    pub fn build(self) -> GraphqlObjectTypeExtension {
        GraphqlObjectTypeExtension::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_OBJECT_TYPE_EXTENSION,
            [
                Some(SyntaxElement::Token(self.extend_token)),
                Some(SyntaxElement::Token(self.type_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.implements
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.fields
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_object_value(
    l_curly_token: SyntaxToken,
    members: GraphqlObjectValueMemberList,
    r_curly_token: SyntaxToken,
) -> GraphqlObjectValue {
    GraphqlObjectValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_OBJECT_VALUE,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(members.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn graphql_operation_definition(
    ty: GraphqlOperationType,
    directives: GraphqlDirectiveList,
    selection_set: GraphqlSelectionSet,
) -> GraphqlOperationDefinitionBuilder {
    GraphqlOperationDefinitionBuilder {
        ty,
        directives,
        selection_set,
        name: None,
        variables: None,
    }
}
pub struct GraphqlOperationDefinitionBuilder {
    ty: GraphqlOperationType,
    directives: GraphqlDirectiveList,
    selection_set: GraphqlSelectionSet,
    name: Option<GraphqlNameBinding>,
    variables: Option<GraphqlVariableDefinitions>,
}
impl GraphqlOperationDefinitionBuilder {
    pub fn with_name(mut self, name: GraphqlNameBinding) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_variables(mut self, variables: GraphqlVariableDefinitions) -> Self {
        self.variables = Some(variables);
        self
    }
    pub fn build(self) -> GraphqlOperationDefinition {
        GraphqlOperationDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_OPERATION_DEFINITION,
            [
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                self.name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.variables
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                Some(SyntaxElement::Node(self.selection_set.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_operation_type(value_token_token: SyntaxToken) -> GraphqlOperationType {
    GraphqlOperationType::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_OPERATION_TYPE,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn graphql_root(
    definitions: GraphqlDefinitionList,
    eof_token: SyntaxToken,
) -> GraphqlRootBuilder {
    GraphqlRootBuilder {
        definitions,
        eof_token,
        bom_token: None,
    }
}
pub struct GraphqlRootBuilder {
    definitions: GraphqlDefinitionList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl GraphqlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> GraphqlRoot {
        GraphqlRoot::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.definitions.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn graphql_root_operation_type_definition(
    operation_type: GraphqlOperationType,
    colon_token: SyntaxToken,
    named_type: GraphqlNameReference,
) -> GraphqlRootOperationTypeDefinition {
    GraphqlRootOperationTypeDefinition::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION,
        [
            Some(SyntaxElement::Node(operation_type.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(named_type.into_syntax())),
        ],
    ))
}
pub fn graphql_root_operation_types(
    l_curly_token: SyntaxToken,
    root_operation_type: GraphqlRootOperationTypeDefinitionList,
    r_curly_token: SyntaxToken,
) -> GraphqlRootOperationTypes {
    GraphqlRootOperationTypes::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ROOT_OPERATION_TYPES,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(root_operation_type.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn graphql_scalar_type_definition(
    scalar_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
) -> GraphqlScalarTypeDefinitionBuilder {
    GraphqlScalarTypeDefinitionBuilder {
        scalar_token,
        name,
        directives,
        description: None,
    }
}
pub struct GraphqlScalarTypeDefinitionBuilder {
    scalar_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
}
impl GraphqlScalarTypeDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn build(self) -> GraphqlScalarTypeDefinition {
        GraphqlScalarTypeDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_SCALAR_TYPE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.scalar_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_scalar_type_extension(
    extend_token: SyntaxToken,
    scalar_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
) -> GraphqlScalarTypeExtension {
    GraphqlScalarTypeExtension::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_SCALAR_TYPE_EXTENSION,
        [
            Some(SyntaxElement::Token(extend_token)),
            Some(SyntaxElement::Token(scalar_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(directives.into_syntax())),
        ],
    ))
}
pub fn graphql_schema_definition(
    schema_token: SyntaxToken,
    directives: GraphqlDirectiveList,
    root_operation_types: GraphqlRootOperationTypes,
) -> GraphqlSchemaDefinitionBuilder {
    GraphqlSchemaDefinitionBuilder {
        schema_token,
        directives,
        root_operation_types,
        description: None,
    }
}
pub struct GraphqlSchemaDefinitionBuilder {
    schema_token: SyntaxToken,
    directives: GraphqlDirectiveList,
    root_operation_types: GraphqlRootOperationTypes,
    description: Option<GraphqlDescription>,
}
impl GraphqlSchemaDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn build(self) -> GraphqlSchemaDefinition {
        GraphqlSchemaDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_SCHEMA_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.schema_token)),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                Some(SyntaxElement::Node(self.root_operation_types.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_schema_extension(
    extend_token: SyntaxToken,
    schema_token: SyntaxToken,
    directives: GraphqlDirectiveList,
) -> GraphqlSchemaExtensionBuilder {
    GraphqlSchemaExtensionBuilder {
        extend_token,
        schema_token,
        directives,
        root_operation_types: None,
    }
}
pub struct GraphqlSchemaExtensionBuilder {
    extend_token: SyntaxToken,
    schema_token: SyntaxToken,
    directives: GraphqlDirectiveList,
    root_operation_types: Option<GraphqlRootOperationTypes>,
}
impl GraphqlSchemaExtensionBuilder {
    pub fn with_root_operation_types(
        mut self,
        root_operation_types: GraphqlRootOperationTypes,
    ) -> Self {
        self.root_operation_types = Some(root_operation_types);
        self
    }
    pub fn build(self) -> GraphqlSchemaExtension {
        GraphqlSchemaExtension::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_SCHEMA_EXTENSION,
            [
                Some(SyntaxElement::Token(self.extend_token)),
                Some(SyntaxElement::Token(self.schema_token)),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.root_operation_types
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_selection_set(
    l_curly_token: SyntaxToken,
    selections: GraphqlSelectionList,
    r_curly_token: SyntaxToken,
) -> GraphqlSelectionSet {
    GraphqlSelectionSet::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_SELECTION_SET,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(selections.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn graphql_string_value(graphql_string_literal_token: SyntaxToken) -> GraphqlStringValue {
    GraphqlStringValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_STRING_VALUE,
        [Some(SyntaxElement::Token(graphql_string_literal_token))],
    ))
}
pub fn graphql_type_condition(
    on_token: SyntaxToken,
    ty: GraphqlNameReference,
) -> GraphqlTypeCondition {
    GraphqlTypeCondition::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_TYPE_CONDITION,
        [
            Some(SyntaxElement::Token(on_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn graphql_union_member_types(
    eq_token: SyntaxToken,
    members: GraphqlUnionMemberTypeList,
) -> GraphqlUnionMemberTypesBuilder {
    GraphqlUnionMemberTypesBuilder {
        eq_token,
        members,
        bitwise_or_token: None,
    }
}
pub struct GraphqlUnionMemberTypesBuilder {
    eq_token: SyntaxToken,
    members: GraphqlUnionMemberTypeList,
    bitwise_or_token: Option<SyntaxToken>,
}
impl GraphqlUnionMemberTypesBuilder {
    pub fn with_bitwise_or_token(mut self, bitwise_or_token: SyntaxToken) -> Self {
        self.bitwise_or_token = Some(bitwise_or_token);
        self
    }
    pub fn build(self) -> GraphqlUnionMemberTypes {
        GraphqlUnionMemberTypes::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_UNION_MEMBER_TYPES,
            [
                Some(SyntaxElement::Token(self.eq_token)),
                self.bitwise_or_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.members.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_union_type_definition(
    union_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
) -> GraphqlUnionTypeDefinitionBuilder {
    GraphqlUnionTypeDefinitionBuilder {
        union_token,
        name,
        directives,
        description: None,
        union_members: None,
    }
}
pub struct GraphqlUnionTypeDefinitionBuilder {
    union_token: SyntaxToken,
    name: GraphqlNameBinding,
    directives: GraphqlDirectiveList,
    description: Option<GraphqlDescription>,
    union_members: Option<GraphqlUnionMemberTypes>,
}
impl GraphqlUnionTypeDefinitionBuilder {
    pub fn with_description(mut self, description: GraphqlDescription) -> Self {
        self.description = Some(description);
        self
    }
    pub fn with_union_members(mut self, union_members: GraphqlUnionMemberTypes) -> Self {
        self.union_members = Some(union_members);
        self
    }
    pub fn build(self) -> GraphqlUnionTypeDefinition {
        GraphqlUnionTypeDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_UNION_TYPE_DEFINITION,
            [
                self.description
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.union_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.union_members
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_union_type_extension(
    extend_token: SyntaxToken,
    union_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
) -> GraphqlUnionTypeExtensionBuilder {
    GraphqlUnionTypeExtensionBuilder {
        extend_token,
        union_token,
        name,
        directives,
        union_members: None,
    }
}
pub struct GraphqlUnionTypeExtensionBuilder {
    extend_token: SyntaxToken,
    union_token: SyntaxToken,
    name: GraphqlNameReference,
    directives: GraphqlDirectiveList,
    union_members: Option<GraphqlUnionMemberTypes>,
}
impl GraphqlUnionTypeExtensionBuilder {
    pub fn with_union_members(mut self, union_members: GraphqlUnionMemberTypes) -> Self {
        self.union_members = Some(union_members);
        self
    }
    pub fn build(self) -> GraphqlUnionTypeExtension {
        GraphqlUnionTypeExtension::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_UNION_TYPE_EXTENSION,
            [
                Some(SyntaxElement::Token(self.extend_token)),
                Some(SyntaxElement::Token(self.union_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
                self.union_members
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_variable_binding(
    dollar_token: SyntaxToken,
    name: GraphqlLiteralName,
) -> GraphqlVariableBinding {
    GraphqlVariableBinding::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_VARIABLE_BINDING,
        [
            Some(SyntaxElement::Token(dollar_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn graphql_variable_definition(
    variable: GraphqlVariableBinding,
    colon_token: SyntaxToken,
    ty: AnyGraphqlType,
    directives: GraphqlDirectiveList,
) -> GraphqlVariableDefinitionBuilder {
    GraphqlVariableDefinitionBuilder {
        variable,
        colon_token,
        ty,
        directives,
        default: None,
    }
}
pub struct GraphqlVariableDefinitionBuilder {
    variable: GraphqlVariableBinding,
    colon_token: SyntaxToken,
    ty: AnyGraphqlType,
    directives: GraphqlDirectiveList,
    default: Option<GraphqlDefaultValue>,
}
impl GraphqlVariableDefinitionBuilder {
    pub fn with_default(mut self, default: GraphqlDefaultValue) -> Self {
        self.default = Some(default);
        self
    }
    pub fn build(self) -> GraphqlVariableDefinition {
        GraphqlVariableDefinition::unwrap_cast(SyntaxNode::new_detached(
            GraphqlSyntaxKind::GRAPHQL_VARIABLE_DEFINITION,
            [
                Some(SyntaxElement::Node(self.variable.into_syntax())),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                self.default
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.directives.into_syntax())),
            ],
        ))
    }
}
pub fn graphql_variable_definitions(
    l_paren_token: SyntaxToken,
    elements: GraphqlVariableDefinitionList,
    r_paren_token: SyntaxToken,
) -> GraphqlVariableDefinitions {
    GraphqlVariableDefinitions::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_VARIABLE_DEFINITIONS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn graphql_variable_reference(
    dollar_token: SyntaxToken,
    name: GraphqlLiteralName,
) -> GraphqlVariableReference {
    GraphqlVariableReference::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_VARIABLE_REFERENCE,
        [
            Some(SyntaxElement::Token(dollar_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn graphql_argument_definition_list<I>(items: I) -> GraphqlArgumentDefinitionList
where
    I: IntoIterator<Item = GraphqlInputValueDefinition>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlArgumentDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ARGUMENT_DEFINITION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_argument_list<I>(items: I) -> GraphqlArgumentList
where
    I: IntoIterator<Item = GraphqlArgument>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlArgumentList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ARGUMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_definition_list<I>(items: I) -> GraphqlDefinitionList
where
    I: IntoIterator<Item = AnyGraphqlDefinition>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_DEFINITION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_directive_list<I>(items: I) -> GraphqlDirectiveList
where
    I: IntoIterator<Item = GraphqlDirective>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlDirectiveList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_DIRECTIVE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_directive_location_list<I, S>(
    items: I,
    separators: S,
) -> GraphqlDirectiveLocationList
where
    I: IntoIterator<Item = GraphqlDirectiveLocation>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GraphqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GraphqlDirectiveLocationList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_DIRECTIVE_LOCATION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn graphql_enum_value_list<I>(items: I) -> GraphqlEnumValueList
where
    I: IntoIterator<Item = GraphqlEnumValueDefinition>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlEnumValueList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ENUM_VALUE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_field_definition_list<I>(items: I) -> GraphqlFieldDefinitionList
where
    I: IntoIterator<Item = GraphqlFieldDefinition>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlFieldDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_FIELD_DEFINITION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_implements_interface_list<I, S>(
    items: I,
    separators: S,
) -> GraphqlImplementsInterfaceList
where
    I: IntoIterator<Item = GraphqlNameReference>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GraphqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GraphqlImplementsInterfaceList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_IMPLEMENTS_INTERFACE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn graphql_input_field_list<I>(items: I) -> GraphqlInputFieldList
where
    I: IntoIterator<Item = GraphqlInputValueDefinition>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlInputFieldList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_INPUT_FIELD_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_list_value_element_list<I>(items: I) -> GraphqlListValueElementList
where
    I: IntoIterator<Item = AnyGraphqlValue>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlListValueElementList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_LIST_VALUE_ELEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_object_value_member_list<I>(items: I) -> GraphqlObjectValueMemberList
where
    I: IntoIterator<Item = GraphqlObjectField>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlObjectValueMemberList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_OBJECT_VALUE_MEMBER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_root_operation_type_definition_list<I>(
    items: I,
) -> GraphqlRootOperationTypeDefinitionList
where
    I: IntoIterator<Item = GraphqlRootOperationTypeDefinition>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlRootOperationTypeDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_selection_list<I>(items: I) -> GraphqlSelectionList
where
    I: IntoIterator<Item = AnyGraphqlSelection>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlSelectionList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_SELECTION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_union_member_type_list<I, S>(items: I, separators: S) -> GraphqlUnionMemberTypeList
where
    I: IntoIterator<Item = GraphqlNameReference>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = GraphqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    GraphqlUnionMemberTypeList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_UNION_MEMBER_TYPE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn graphql_variable_definition_list<I>(items: I) -> GraphqlVariableDefinitionList
where
    I: IntoIterator<Item = GraphqlVariableDefinition>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlVariableDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_VARIABLE_DEFINITION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn graphql_bogus<I>(slots: I) -> GraphqlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlBogus::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_BOGUS,
        slots,
    ))
}
pub fn graphql_bogus_definition<I>(slots: I) -> GraphqlBogusDefinition
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlBogusDefinition::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_BOGUS_DEFINITION,
        slots,
    ))
}
pub fn graphql_bogus_selection<I>(slots: I) -> GraphqlBogusSelection
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlBogusSelection::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_BOGUS_SELECTION,
        slots,
    ))
}
pub fn graphql_bogus_type<I>(slots: I) -> GraphqlBogusType
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlBogusType::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_BOGUS_TYPE,
        slots,
    ))
}
pub fn graphql_bogus_value<I>(slots: I) -> GraphqlBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    GraphqlBogusValue::unwrap_cast(SyntaxNode::new_detached(
        GraphqlSyntaxKind::GRAPHQL_BOGUS_VALUE,
        slots,
    ))
}
