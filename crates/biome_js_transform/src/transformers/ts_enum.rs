use crate::{declare_transformation, JsBatchMutation};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule};
use biome_js_factory::make::{
    ident, js_assignment_expression, js_call_argument_list, js_call_arguments, js_call_expression,
    js_computed_member_assignment, js_decorator_list, js_directive_list, js_expression_statement,
    js_formal_parameter, js_function_body, js_function_expression, js_identifier_assignment,
    js_identifier_binding, js_identifier_expression, js_logical_expression, js_module_item_list,
    js_number_literal_expression, js_object_expression, js_object_member_list, js_parameter_list,
    js_parameters, js_parenthesized_expression, js_reference_identifier, js_statement_list,
    js_string_literal, js_string_literal_expression, js_variable_declaration,
    js_variable_declarator, js_variable_declarator_list, js_variable_statement, token,
};
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsExpression, AnyJsFormalParameter, AnyJsLiteralExpression, AnyJsModuleItem, AnyJsParameter,
    AnyJsStatement, JsAssignmentExpression, JsComputedMemberAssignment, JsExpressionStatement,
    JsFunctionExpression, JsInitializerClause, JsLogicalExpression, JsModuleItemList,
    JsStatementList, JsVariableStatement, TsEnumDeclaration, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};

declare_transformation! {
    /// Transform a TypeScript [TsEnumDeclaration]
    pub(crate) TsEnum {
        version: "1.3.0",
        name: "transformEnum",
        language: "ts",
    }
}

#[derive(Debug)]
pub struct TsEnumMembers {
    name: String,
    member_names: Vec<(String, Option<JsInitializerClause>)>,
}

impl Rule for TsEnum {
    type Query = Ast<TsEnumDeclaration>;
    type State = TsEnumMembers;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut member_names = vec![];
        let id = node.id().ok()?;
        let name = id.to_trimmed_string();
        for member in node.members() {
            let member = member.ok()?;
            let key = member.name().ok()?.to_trimmed_string();
            let value = member.initializer().clone();
            member_names.push((key, value));
        }

        Some(TsEnumMembers { name, member_names })
    }

    fn transform(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsBatchMutation> {
        let node = ctx.query();
        let mut mutation = node.clone().begin();
        let parent = node.syntax().parent();
        if let Some(parent) = parent {
            if let Some(module_list) = JsModuleItemList::cast(parent) {
                let variable = make_variable(state);
                let function = make_function_caller(state);
                let statements = vec![
                    AnyJsModuleItem::AnyJsStatement(AnyJsStatement::JsVariableStatement(variable)),
                    AnyJsModuleItem::AnyJsStatement(AnyJsStatement::JsExpressionStatement(
                        function,
                    )),
                ];
                let new_modules_list = js_module_item_list(statements);
                mutation.replace_node(module_list, new_modules_list);
            }
        }

        Some(mutation)
    }
}

/// Out of an enum, this functions emits the generation of the:
///
/// ```ts
/// enum Foo {}
/// var Foo;
/// ```
fn make_variable(node: &TsEnumMembers) -> JsVariableStatement {
    let binding = js_variable_declarator(AnyJsBindingPattern::AnyJsBinding(
        AnyJsBinding::JsIdentifierBinding(js_identifier_binding(ident(node.name.as_str()))),
    ))
    .build();

    let list = js_variable_declarator_list([binding], []);
    js_variable_statement(
        js_variable_declaration(
            token(T![var]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            list,
        )
        .build(),
    )
    .with_semicolon_token(token(T![;]))
    .build()
}

fn make_function_caller(node: &TsEnumMembers) -> JsExpressionStatement {
    let callee = js_parenthesized_expression(
        token(T!['(']),
        AnyJsExpression::JsFunctionExpression(make_function(node)),
        token(T![')']),
    );
    let argument = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsLogicalExpression(
        make_logical_expression(node),
    ));
    let arguments = js_call_arguments(
        token(T!['(']),
        js_call_argument_list([argument], []),
        token(T![')']),
    );
    let expression = js_call_expression(
        AnyJsExpression::JsParenthesizedExpression(callee),
        arguments,
    )
    .build();
    js_expression_statement(AnyJsExpression::JsCallExpression(expression))
        .with_semicolon_token(token(T![;]))
        .build()
}

fn make_function(node: &TsEnumMembers) -> JsFunctionExpression {
    let parameters_list = js_parameter_list(
        [AnyJsParameter::AnyJsFormalParameter(
            AnyJsFormalParameter::JsFormalParameter(
                js_formal_parameter(
                    js_decorator_list(vec![]),
                    AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                        js_identifier_binding(ident(node.name.as_str())),
                    )),
                )
                .build(),
            ),
        )],
        [],
    );
    let parameters = js_parameters(token(T!['(']), parameters_list, token(T![')']));

    let body = js_function_body(
        token(T!['{']),
        js_directive_list([]),
        make_members(node),
        token(T!['}']),
    );
    js_function_expression(token(T![function]), parameters, body).build()
}

fn make_members(ts_enum: &TsEnumMembers) -> JsStatementList {
    let mut list = vec![];
    for (index, (name, value)) in ts_enum.member_names.iter().enumerate() {
        let value = value
            .as_ref()
            .and_then(|initializer| initializer.expression().ok())
            .unwrap_or_else(|| {
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(
                        js_number_literal_expression(ident(&index.to_string())),
                    ),
                )
            });
        list.push(AnyJsStatement::JsExpressionStatement(
            make_high_order_assignment(ts_enum.name.as_str(), name.as_str(), value),
        ));
    }

    js_statement_list(list)
}

fn make_logical_expression(node: &TsEnumMembers) -> JsLogicalExpression {
    let left = js_identifier_expression(js_reference_identifier(ident(node.name.as_str())));

    let expression = js_assignment_expression(
        AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
            js_identifier_assignment(ident(node.name.as_str())),
        )),
        token(T![=]),
        AnyJsExpression::JsObjectExpression(js_object_expression(
            token(T!['{']),
            js_object_member_list([], []),
            token(T!['}']),
        )),
    );

    let right = js_parenthesized_expression(
        token(T!['(']),
        AnyJsExpression::JsAssignmentExpression(expression),
        token(T![')']),
    );

    js_logical_expression(
        AnyJsExpression::JsIdentifierExpression(left),
        token(T![||]),
        AnyJsExpression::JsParenthesizedExpression(right),
    )
}

fn make_high_order_assignment(
    enum_name: &str,
    member_name: &str,
    member_value: AnyJsExpression,
) -> JsExpressionStatement {
    let left = js_computed_member_assignment(
        AnyJsExpression::JsIdentifierExpression(js_identifier_expression(js_reference_identifier(
            ident(enum_name),
        ))),
        token(T!['[']),
        AnyJsExpression::JsAssignmentExpression(make_assignment_expression_from_member(
            enum_name,
            member_name,
            member_value,
        )),
        token(T![']']),
    );
    let right = js_string_literal_expression(js_string_literal(member_name));

    let expression = js_assignment_expression(
        AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsComputedMemberAssignment(left)),
        token(T![=]),
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(
            right,
        )),
    );

    js_expression_statement(AnyJsExpression::JsAssignmentExpression(expression))
        .with_semicolon_token(token(T![;]))
        .build()
}

/// Makes
/// ```js
/// Foo["Lorem"] = 0
/// ```
fn make_assignment_expression_from_member(
    enum_name: &str,
    member_name: &str,
    member_value: AnyJsExpression,
) -> JsAssignmentExpression {
    let left = make_computed_member_assignment(enum_name, member_name);

    js_assignment_expression(
        AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsComputedMemberAssignment(left)),
        token(T![=]),
        member_value,
    )
}

/// Creates
/// ```js
/// Foo["Lorem"]
/// ```
fn make_computed_member_assignment(
    enum_name: &str,
    member_name: &str,
) -> JsComputedMemberAssignment {
    let object = js_identifier_expression(js_reference_identifier(ident(enum_name)));
    let member = js_string_literal_expression(js_string_literal(member_name));
    js_computed_member_assignment(
        AnyJsExpression::JsIdentifierExpression(object),
        token(T!['[']),
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(
            member,
        )),
        token(T![']']),
    )
}
