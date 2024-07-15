//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::GraphqlSyntaxNode::kind(&node) {
                $crate::GraphqlSyntaxKind::GRAPHQL_ALIAS => {
                    let $pattern = unsafe { $crate::GraphqlAlias::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ARGUMENT => {
                    let $pattern = unsafe { $crate::GraphqlArgument::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ARGUMENTS => {
                    let $pattern = unsafe { $crate::GraphqlArguments::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ARGUMENTS_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlArgumentsDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_BOOLEAN_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlBooleanValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_DEFAULT_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlDefaultValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_DESCRIPTION => {
                    let $pattern = unsafe { $crate::GraphqlDescription::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_DIRECTIVE => {
                    let $pattern = unsafe { $crate::GraphqlDirective::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_DIRECTIVE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlDirectiveDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_DIRECTIVE_LOCATION => {
                    let $pattern = unsafe { $crate::GraphqlDirectiveLocation::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ENUM_TYPE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlEnumTypeDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ENUM_TYPE_EXTENSION => {
                    let $pattern = unsafe { $crate::GraphqlEnumTypeExtension::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ENUM_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlEnumValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ENUM_VALUE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlEnumValueDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ENUM_VALUES_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlEnumValuesDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_FIELD => {
                    let $pattern = unsafe { $crate::GraphqlField::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_FIELD_DEFINITION => {
                    let $pattern = unsafe { $crate::GraphqlFieldDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_FIELDS_DEFINITION => {
                    let $pattern = unsafe { $crate::GraphqlFieldsDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_FLOAT_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlFloatValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_FRAGMENT_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlFragmentDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_FRAGMENT_SPREAD => {
                    let $pattern = unsafe { $crate::GraphqlFragmentSpread::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_IMPLEMENTS_INTERFACES => {
                    let $pattern =
                        unsafe { $crate::GraphqlImplementsInterfaces::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INLINE_FRAGMENT => {
                    let $pattern = unsafe { $crate::GraphqlInlineFragment::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INPUT_FIELDS_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlInputFieldsDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlInputObjectTypeDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION => {
                    let $pattern =
                        unsafe { $crate::GraphqlInputObjectTypeExtension::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INPUT_VALUE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlInputValueDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INT_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlIntValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INTERFACE_TYPE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlInterfaceTypeDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INTERFACE_TYPE_EXTENSION => {
                    let $pattern =
                        unsafe { $crate::GraphqlInterfaceTypeExtension::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_LIST_TYPE => {
                    let $pattern = unsafe { $crate::GraphqlListType::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_LIST_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlListValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_LITERAL_NAME => {
                    let $pattern = unsafe { $crate::GraphqlLiteralName::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_NAME_BINDING => {
                    let $pattern = unsafe { $crate::GraphqlNameBinding::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_NAME_REFERENCE => {
                    let $pattern = unsafe { $crate::GraphqlNameReference::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_NON_NULL_TYPE => {
                    let $pattern = unsafe { $crate::GraphqlNonNullType::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_NULL_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlNullValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_OBJECT_FIELD => {
                    let $pattern = unsafe { $crate::GraphqlObjectField::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_OBJECT_TYPE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlObjectTypeDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_OBJECT_TYPE_EXTENSION => {
                    let $pattern =
                        unsafe { $crate::GraphqlObjectTypeExtension::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_OBJECT_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlObjectValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_OPERATION_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlOperationDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_OPERATION_TYPE => {
                    let $pattern = unsafe { $crate::GraphqlOperationType::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ROOT => {
                    let $pattern = unsafe { $crate::GraphqlRoot::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlRootOperationTypeDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ROOT_OPERATION_TYPES => {
                    let $pattern =
                        unsafe { $crate::GraphqlRootOperationTypes::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_SCALAR_TYPE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlScalarTypeDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_SCALAR_TYPE_EXTENSION => {
                    let $pattern =
                        unsafe { $crate::GraphqlScalarTypeExtension::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_SCHEMA_DEFINITION => {
                    let $pattern = unsafe { $crate::GraphqlSchemaDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_SCHEMA_EXTENSION => {
                    let $pattern = unsafe { $crate::GraphqlSchemaExtension::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_SELECTION_SET => {
                    let $pattern = unsafe { $crate::GraphqlSelectionSet::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_STRING_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlStringValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_TYPE_CONDITION => {
                    let $pattern = unsafe { $crate::GraphqlTypeCondition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_UNION_MEMBER_TYPES => {
                    let $pattern = unsafe { $crate::GraphqlUnionMemberTypes::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_UNION_TYPE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlUnionTypeDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_UNION_TYPE_EXTENSION => {
                    let $pattern =
                        unsafe { $crate::GraphqlUnionTypeExtension::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_VARIABLE_BINDING => {
                    let $pattern = unsafe { $crate::GraphqlVariableBinding::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_VARIABLE_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::GraphqlVariableDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_VARIABLE_DEFINITIONS => {
                    let $pattern =
                        unsafe { $crate::GraphqlVariableDefinitions::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_VARIABLE_REFERENCE => {
                    let $pattern = unsafe { $crate::GraphqlVariableReference::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_BOGUS => {
                    let $pattern = unsafe { $crate::GraphqlBogus::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_BOGUS_DEFINITION => {
                    let $pattern = unsafe { $crate::GraphqlBogusDefinition::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_BOGUS_SELECTION => {
                    let $pattern = unsafe { $crate::GraphqlBogusSelection::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_BOGUS_TYPE => {
                    let $pattern = unsafe { $crate::GraphqlBogusType::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_BOGUS_VALUE => {
                    let $pattern = unsafe { $crate::GraphqlBogusValue::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ARGUMENT_DEFINITION_LIST => {
                    let $pattern =
                        unsafe { $crate::GraphqlArgumentDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::GraphqlArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_DEFINITION_LIST => {
                    let $pattern = unsafe { $crate::GraphqlDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_DIRECTIVE_LIST => {
                    let $pattern = unsafe { $crate::GraphqlDirectiveList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_DIRECTIVE_LOCATION_LIST => {
                    let $pattern =
                        unsafe { $crate::GraphqlDirectiveLocationList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ENUM_VALUE_LIST => {
                    let $pattern = unsafe { $crate::GraphqlEnumValueList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_FIELD_DEFINITION_LIST => {
                    let $pattern =
                        unsafe { $crate::GraphqlFieldDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_IMPLEMENTS_INTERFACE_LIST => {
                    let $pattern =
                        unsafe { $crate::GraphqlImplementsInterfaceList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_INPUT_FIELD_LIST => {
                    let $pattern = unsafe { $crate::GraphqlInputFieldList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_LIST_VALUE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::GraphqlListValueElementList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_OBJECT_VALUE_MEMBER_LIST => {
                    let $pattern =
                        unsafe { $crate::GraphqlObjectValueMemberList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION_LIST => {
                    let $pattern = unsafe {
                        $crate::GraphqlRootOperationTypeDefinitionList::new_unchecked(node)
                    };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_SELECTION_LIST => {
                    let $pattern = unsafe { $crate::GraphqlSelectionList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_UNION_MEMBER_TYPE_LIST => {
                    let $pattern =
                        unsafe { $crate::GraphqlUnionMemberTypeList::new_unchecked(node) };
                    $body
                }
                $crate::GraphqlSyntaxKind::GRAPHQL_VARIABLE_DEFINITION_LIST => {
                    let $pattern =
                        unsafe { $crate::GraphqlVariableDefinitionList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
