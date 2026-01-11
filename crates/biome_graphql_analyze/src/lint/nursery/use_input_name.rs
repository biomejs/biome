use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{MarkupBuf, markup};
use biome_graphql_syntax::{
    AnyGraphqlPrimitiveType, AnyGraphqlType, GraphqlFieldDefinition, GraphqlFieldDefinitionList,
    GraphqlFieldsDefinition, GraphqlLanguage, GraphqlObjectTypeDefinition,
    GraphqlObjectTypeExtension,
};
use biome_rowan::{AstNode, SyntaxToken, TextRange};
use biome_rule_options::use_input_name::UseInputNameOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Require mutation argument to be always called "input"
    ///
    /// Using the same name for all input parameters will make your schemas easier to consume and more predictable.
    ///
    /// Optionally, when the option `checkInputType` has been enabled, the input type requires to be called `<mutation name>Input`.
    /// Using the name of the mutation in the input type name will make it easier to find the mutation that the input type belongs to.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// type Mutation {
    ///   SetMessage(message: InputMessage): String
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// type Mutation {
    ///   SetMessage(input: SetMessageInput): String
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `checkInputType`
    ///
    /// Check that the input type name follows the convention <mutationName>Input.
    ///
    /// Default `false`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "checkInputType": true
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic,use_options
    /// type Mutation {
    ///   SetMessage(input: InputMessage): String
    /// }
    /// ```
    ///
    /// ### `caseSensitiveInputType`
    ///
    /// Treat input type names as case-sensitive.
    ///
    /// Default `true`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "checkInputType": true,
    ///     "caseSensitiveInputType": true
    ///   }
    /// }
    /// ```
    ///
    /// ```graphql,expect_diagnostic,use_options
    /// type Mutation {
    ///   SetMessage(input: setMessageInput): String
    /// }
    /// ```
    ///
    pub UseInputName {
        version: "next",
        name: "useInputName",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("input-name").inspired()],
    }
}

impl Rule for UseInputName {
    type Query = Ast<GraphqlFieldDefinition>;
    type State = UseInputNameState;
    type Signals = Option<Self::State>;
    type Options = UseInputNameOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let def_list = node
            .syntax()
            .parent()
            .and_then(GraphqlFieldDefinitionList::cast)?;
        let fields_def = def_list
            .syntax()
            .parent()
            .and_then(GraphqlFieldsDefinition::cast)?;

        let is_mutation = fields_def.syntax().parent().is_some_and(|parent| {
            if let Some(type_def) = GraphqlObjectTypeDefinition::cast(parent.clone()) {
                return type_def.is_mutation();
            }
            if let Some(type_ext) = GraphqlObjectTypeExtension::cast(parent.clone()) {
                return type_ext.is_mutation();
            }

            false
        });

        if !is_mutation {
            return None;
        }

        let arguments = node.arguments()?;
        for argument in arguments.arguments() {
            let name = argument.name().ok()?;
            let value_token = name.value_token().ok()?;
            let current = value_token.text_trimmed();
            if current != "input" {
                return Some(UseInputNameState::InvalidName(
                    argument.range(),
                    current.to_string(),
                ));
            }

            let check_input_type = ctx.options().check_input_type();
            if check_input_type {
                let case_sensitive_input_type = ctx.options().case_sensitive_input_type();

                let any_type = argument.ty().ok()?;

                let ty = find_input_type(any_type)?;
                let ty_string = ty.text_trimmed();

                let def_name = node.name().ok()?;
                let def_value_token = def_name.value_token().ok()?;

                let valid_string = def_value_token.text_trimmed().to_string() + "Input";
                if (case_sensitive_input_type && ty_string != valid_string)
                    || ty_string.to_lowercase_cow() != valid_string.to_lowercase_cow()
                {
                    return Some(UseInputNameState::InvalidTypeName(
                        argument.range(),
                        ty_string.to_string(),
                        valid_string,
                    ));
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(rule_category!(), state.range(), state.message())
                .note(state.description()),
        )
    }
}

/// Representation of the various states
///
/// The `TextRange` of each variant represents the range of where the issue is found.
pub enum UseInputNameState {
    /// The input value name does not match "input"
    InvalidName(TextRange, String),
    /// The input value type name does not equal mutation name + "Input".
    InvalidTypeName(TextRange, String, String),
}

impl UseInputNameState {
    fn range(&self) -> &TextRange {
        match self {
            Self::InvalidName(range, _) | Self::InvalidTypeName(range, _, _) => range,
        }
    }

    fn message(&self) -> MarkupBuf {
        match self {
            Self::InvalidName(_, current) => (markup! {
                "Input \""{ current }"\" should be named \"input\"."
            })
            .to_owned(),
            Self::InvalidTypeName(_, current, valid) => (markup! {
                "Input type \""{ current }"\" name should be \""{ valid }"\"."
            })
            .to_owned(),
        }
    }

    fn description(&self) -> MarkupBuf {
        match self {
            Self::InvalidName(_, _) => (markup! {
                "Using the same name for all input parameters will make your schemas easier to consume and more predictable."
            })
            .to_owned(),
            Self::InvalidTypeName(_, _, _) => (markup! {
                "Using the name of the operation in the input type name will make it easier to find the operation that the input type belongs to."
            })
            .to_owned(),
        }
    }
}

fn find_input_type(any_type: AnyGraphqlType) -> Option<SyntaxToken<GraphqlLanguage>> {
    match any_type {
        AnyGraphqlType::AnyGraphqlPrimitiveType(primitive_type) => {
            find_input_type_primitive_type(primitive_type)
        }
        AnyGraphqlType::GraphqlNonNullType(non_null_type) => {
            let base = non_null_type.base().ok()?;
            find_input_type_primitive_type(base)
        }
        _ => None,
    }
}

fn find_input_type_primitive_type(
    primitive_type: AnyGraphqlPrimitiveType,
) -> Option<SyntaxToken<GraphqlLanguage>> {
    match primitive_type {
        AnyGraphqlPrimitiveType::GraphqlNameReference(name_ref) => name_ref.value_token().ok(),
        AnyGraphqlPrimitiveType::GraphqlListType(list_type) => {
            let any_type = list_type.element().ok()?;
            find_input_type(any_type)
        }
    }
}
