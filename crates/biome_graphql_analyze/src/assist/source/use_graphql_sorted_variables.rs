use crate::GraphqlRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_graphql_factory::make;
use biome_graphql_syntax::{
    GraphqlLanguage, GraphqlSyntaxToken, GraphqlVariableDefinition, GraphqlVariableDefinitionList,
    GraphqlVariableDefinitions,
};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutationExt, SyntaxTriviaPiece, TokenText, chain_trivia_pieces,
};
use biome_rule_options::use_graphql_sorted_variables::UseGraphqlSortedVariablesOptions;
use biome_string_case::comparable_token::ComparableToken;
use std::cmp::Ordering;

declare_source_rule! {
    /// Sort the variable definitions of GraphQL operations in natural order.
    ///
    /// Keeping the variables of `query`, `mutation`, and `subscription` operations sorted
    /// makes long variable lists easier to scan and review.
    /// The order of variable definitions doesn't affect how an operation is executed.
    ///
    /// Variables are sorted by name in a [Natural order](https://en.wikipedia.org/wiki/Natural_sort_order),
    /// meaning that uppercase letters come before lowercase letters (e.g. `A` < `a` < `B` < `b`)
    /// and numbers are compared to their numerical value (e.g. `9` < `10`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diff
    /// query GetUser($userId: ID!, $includePosts: Boolean = false, $after: String) {
    ///   user(id: $userId) {
    ///     name
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query GetUser($after: String, $includePosts: Boolean = false, $userId: ID!) {
    ///   user(id: $userId) {
    ///     name
    ///   }
    /// }
    /// ```
    ///
    /// ## Caveats
    ///
    /// Comments above a variable and comments at the end of its line move with that variable.
    /// No fix is offered when a moved comment would end up on the same line as other code,
    /// because the comment would then hide that code or describe a different variable.
    /// Sort such variables manually, or put each variable on its own line.
    ///
    pub UseGraphqlSortedVariables {
        version: "2.6.0",
        name: "useGraphqlSortedVariables",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("alphabetize").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseGraphqlSortedVariables {
    type Query = Ast<GraphqlVariableDefinitions>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseGraphqlSortedVariablesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if is_variable_list_sorted(&node.elements()) {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "The variables of this operation are not sorted."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<GraphqlRuleAction> {
        let node = ctx.query();
        let list = node.elements();
        let r_paren_on_new_line = node
            .r_paren_token()
            .ok()?
            .leading_trivia()
            .pieces()
            .next()
            .is_some_and(|piece| piece.is_newline());
        let sorted = sort_variable_list(&list, r_paren_on_new_line)?;

        let mut mutation = ctx.root().begin();
        // `replace_node` would copy the trivia of the old list boundaries onto the new list,
        // overwriting the trivia computed by `sort_variable_list`.
        mutation.replace_node_discard_trivia(list, sorted);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort the variables." },
            mutation,
        ))
    }
}

/// The trivia around a variable definition, split between the trivia that moves with the
/// variable and the trivia that stays at its position in the list.
struct VariableTrivia {
    node: GraphqlVariableDefinition,
    name: Option<TokenText>,
    first_token: GraphqlSyntaxToken,
    /// The number of leading trivia pieces of `first_token` before the first comment.
    position_leading_len: usize,
    last_token: GraphqlSyntaxToken,
    /// Whether the trailing trivia of `last_token` contains an end-of-line comment, in which
    /// case the whole trailing trivia moves with the variable instead of staying in place.
    trailing_is_attached: bool,
}

impl VariableTrivia {
    fn new(node: GraphqlVariableDefinition) -> Option<Self> {
        let syntax = node.syntax();
        let first_token = syntax.first_token()?;
        let last_token = syntax.last_token()?;
        let leading = first_token.leading_trivia();
        let position_leading_len = leading
            .pieces()
            .position(|piece| piece.is_comments())
            .unwrap_or(leading.pieces().len());
        let trailing_is_attached = last_token
            .trailing_trivia()
            .pieces()
            .any(|piece| piece.is_comments());

        Some(Self {
            name: variable_name(&node),
            node,
            first_token,
            position_leading_len,
            last_token,
            trailing_is_attached,
        })
    }

    /// The leading trivia before its first comment. It stays at the position of the variable.
    fn position_leading(&self) -> impl ExactSizeIterator<Item = SyntaxTriviaPiece<GraphqlLanguage>> {
        self.first_token
            .leading_trivia()
            .pieces()
            .take(self.position_leading_len)
    }

    /// The leading trivia from its first comment onward. It moves with the variable.
    fn attached_leading(&self) -> impl ExactSizeIterator<Item = SyntaxTriviaPiece<GraphqlLanguage>> {
        self.first_token
            .leading_trivia()
            .pieces()
            .skip(self.position_leading_len)
    }

    fn has_attached_leading(&self) -> bool {
        self.position_leading_len < self.first_token.leading_trivia().pieces().len()
    }

    /// Whether this position in the list starts on a new line.
    ///
    /// Leading trivia is either empty or starts with a newline, because the trivia up to the
    /// end of a line belongs to the trailing trivia of the previous token.
    fn position_on_new_line(&self) -> bool {
        self.position_leading()
            .next()
            .is_some_and(|piece| piece.is_newline())
    }
}

/// Returns a sorted copy of `list`, or `None` when the sorted list can't be built without
/// placing a moved comment on the same line as other code.
fn sort_variable_list(
    list: &GraphqlVariableDefinitionList,
    r_paren_on_new_line: bool,
) -> Option<GraphqlVariableDefinitionList> {
    let variables = list
        .iter()
        .map(VariableTrivia::new)
        .collect::<Option<Vec<_>>>()?;

    let mut order: Vec<usize> = (0..variables.len()).collect();
    order.sort_by(|&a, &b| compare_names(variables[a].name.clone(), variables[b].name.clone()));

    let mut nodes = Vec::with_capacity(variables.len());
    for (position, &index) in order.iter().enumerate() {
        let slot = &variables[position];
        let variable = &variables[index];

        // A comment above the variable must stay on its own line. Otherwise it would follow
        // the previous token and describe a different variable.
        if variable.has_attached_leading() && !slot.position_on_new_line() {
            return None;
        }

        let node = variable
            .node
            .clone()
            .with_leading_trivia_pieces(chain_trivia_pieces(
                slot.position_leading(),
                variable.attached_leading(),
            ))?;
        let node = if variable.trailing_is_attached {
            // A line comment runs until the end of the line, so the code after it must start
            // on a new line. Otherwise the comment would swallow that code.
            let next_on_new_line = variables
                .get(position + 1)
                .map_or(r_paren_on_new_line, VariableTrivia::position_on_new_line);
            if !next_on_new_line {
                return None;
            }
            node.with_trailing_trivia_pieces(variable.last_token.trailing_trivia().pieces())?
        } else if slot.trailing_is_attached {
            node.with_trailing_trivia_pieces([])?
        } else {
            node.with_trailing_trivia_pieces(slot.last_token.trailing_trivia().pieces())?
        };
        nodes.push(node);
    }

    Some(make::graphql_variable_definition_list(nodes))
}

fn variable_name(node: &GraphqlVariableDefinition) -> Option<TokenText> {
    node.variable()
        .ok()?
        .name()
        .ok()?
        .value_token()
        .ok()
        .map(|token| token.token_text_trimmed())
}

/// Compares two names in natural order. A missing name sorts after every present name.
fn compare_names(a: Option<TokenText>, b: Option<TokenText>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => ComparableToken::new(a).ascii_nat_cmp(&ComparableToken::new(b)),
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

fn is_variable_list_sorted(list: &GraphqlVariableDefinitionList) -> bool {
    let mut previous: Option<GraphqlVariableDefinition> = None;
    for variable in list {
        if previous.as_ref().is_some_and(|previous| {
            compare_names(variable_name(previous), variable_name(&variable)) == Ordering::Greater
        }) {
            return false;
        }
        previous = Some(variable);
    }
    true
}
