use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{MarkupBuf, markup};
use biome_graphql_syntax::{
    AnyGraphqlPrimitiveType, AnyGraphqlType, GraphqlFieldDefinition, GraphqlFieldDefinitionList,
    GraphqlFieldsDefinition, GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension,
    GraphqlSyntaxToken,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_input_name::{CheckInputType, UseInputNameOptions};

declare_lint_rule! {
    /// Require mutation argument to be always called "input"
    ///
    /// Using the same name for all input parameters will make your schemas easier to consume and more predictable.
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
    /// With the option `checkInputType` on, the input type name requires to be called `<mutation name>Input`.
    /// This can either be "loose" (case-insensitive) or "strict" (case-sensitive).
    /// Using the name of the mutation in the input type name will make it easier to find the mutation that the input type belongs to.
    ///
    /// Default `"off"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "checkInputType": "loose"
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
    /// ```graphql,use_options
    /// type Mutation {
    ///   SetMessage(input: setMessageInput): String
    /// }
    /// ```
    ///
    /// ```graphql,use_options
    /// type Mutation {
    ///   SetMessage(input: SetMessageInput): String
    /// }
    /// ```
    ///
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "checkInputType": "strict"
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
    /// ```graphql,expect_diagnostic,use_options
    /// type Mutation {
    ///   SetMessage(input: setMessageInput): String
    /// }
    /// ```
    ///
    /// ```graphql,use_options
    /// type Mutation {
    ///   SetMessage(input: SetMessageInput): String
    /// }
    /// ```
    ///
    pub UseInputName {
        version: "next",
        name: "useInputName",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("input-name").same()],
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
            if let Some(type_def) = GraphqlObjectTypeDefinition::cast_ref(&parent) {
                return type_def.is_mutation();
            }
            if let Some(type_ext) = GraphqlObjectTypeExtension::cast_ref(&parent) {
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

            let check_input_type = ctx.options().check_input_type;
            if let Some(check_input_type) = check_input_type
                && check_input_type != CheckInputType::Off
            {
                let any_type = argument.ty().ok()?;

                let ty = find_input_type(any_type)?;
                let ty_string = ty.text_trimmed();

                let def_name = node.name().ok()?;
                let def_value_token = def_name.value_token().ok()?;

                let valid_str = format!("{}Input", def_value_token.text_trimmed());
                if (check_input_type == CheckInputType::Strict && ty_string != valid_str)
                    || (check_input_type == CheckInputType::Loose
                        && !ty_string.eq_ignore_ascii_case(&valid_str))
                {
                    return Some(UseInputNameState::InvalidTypeName(
                        argument.range(),
                        ty_string.to_string(),
                        valid_str,
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
                "Unexpected input name, expected the input name \""{ current }"\" to be named \"input\"."
            })
            .to_owned(),
            Self::InvalidTypeName(_, current, valid) => (markup! {
                "Unexpected input type name, expected the input type name \""{ current }"\" to be named \""{ valid }"\"."
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

fn find_input_type(any_type: AnyGraphqlType) -> Option<GraphqlSyntaxToken> {
    let mut current_type = any_type;

    loop {
        match current_type {
            AnyGraphqlType::AnyGraphqlPrimitiveType(primitive_type) => match primitive_type {
                AnyGraphqlPrimitiveType::GraphqlNameReference(name_ref) => {
                    return name_ref.value_token().ok();
                }
                AnyGraphqlPrimitiveType::GraphqlListType(list_type) => {
                    current_type = list_type.element().ok()?;
                }
            },
            AnyGraphqlType::GraphqlNonNullType(non_null_type) => {
                let base = non_null_type.base().ok()?;
                current_type = AnyGraphqlType::AnyGraphqlPrimitiveType(base);
            }
            _ => return None,
        }
    }
}
