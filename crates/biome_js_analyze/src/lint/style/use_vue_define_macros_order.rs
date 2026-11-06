use std::ops::Not;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make::{self, js_module_item_list};
use biome_js_syntax::{
    AnyJsExpression, AnyJsModuleItem, AnyJsStatement, JsDebuggerStatement, JsEmptyStatement,
    JsExport, JsExportNamedClause, JsImport, JsModuleItemList, JsSyntaxKind, JsVariableDeclarator,
    JsVariableDeclaratorList, T, TsInterfaceDeclaration, TsModuleDeclaration,
    TsTypeAliasDeclaration,
};
use biome_parser::token_set;
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet, TextRange, TokenText,
    TriviaPieceKind,
};
use biome_rule_options::use_vue_define_macros_order::UseVueDefineMacrosOrderOptions;
use either::Either;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

use crate::JsRuleAction;
use biome_languages::JsFileSource;

declare_lint_rule! {
    /// Enforce specific order of Vue compiler macros.
    ///
    /// This rule ensures consistent ordering of Vue's Composition API compiler macros in `<script setup>` blocks.
    /// Enforcing a consistent order improves code readability and maintainability by establishing a predictable structure.
    ///
    /// These macros must also appear before any other statements (except imports, type declarations, and debugger statements).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script lang="ts" setup>
    /// const emit = defineEmits(['update'])
    /// const props = defineProps<{ name: string }>()
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script lang="ts" setup>
    /// import { ref } from 'vue'
    ///
    /// const count = ref(0)
    /// const props = defineProps<{ name: string }>()
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script lang="ts" setup>
    /// const props = defineProps<{ name: string }>()
    /// const emit = defineEmits(['update'])
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script lang="ts" setup>
    /// import { ref } from 'vue'
    ///
    /// const model = defineModel()
    /// const props = defineProps<{ name: string }>()
    /// const emit = defineEmits(['update'])
    ///
    /// const count = ref(0)
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script lang="ts" setup>
    /// import { ref } from 'vue'
    ///
    /// interface Props {
    ///   value: string
    /// }
    ///
    /// const props = defineProps<Props>()
    /// const emit = defineEmits(['update'])
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `order`
    ///
    /// Allows specifying the order in which the Vue compiler macros should appear.
    ///
    /// **Default:** `["defineModel", "defineProps", "defineEmits"]`
    ///
    /// This is not limited to built-in macros, for example unplug-vue-router [definePage](https://uvr.esm.is/guide/extending-routes.html#definepage)
    /// can be listed here as well.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "order": ["definePage", "defineProps", "defineEmits", "defineModel"]
    ///   }
    /// }
    /// ```
    ///
    pub UseVueDefineMacrosOrder {
        version: "2.3.0",
        name: "useVueDefineMacrosOrder",
        language: "js",
        sources: &[RuleSource::EslintVueJs("define-macros-order").same()],
        recommended: false,
        domains: &[RuleDomain::Vue],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseVueDefineMacrosOrder {
    type Query = Ast<JsModuleItemList>;
    type State = MacroOrderIssue;
    type Signals = Option<Self::State>;
    type Options = UseVueDefineMacrosOrderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_vue_setup()
            .not()
        {
            return None;
        }

        let order_map: FxHashMap<&str, usize> = ctx
            .options()
            .order_or_default()
            .enumerate()
            .map(|(idx, s)| (s, idx))
            .collect();

        let mut non_macro_found = false;
        let mut skippable_top_statements_end_index = 0;
        let mut previous_order_index = 0;
        let mut first_issue = None;
        let mut macro_statements = SmallVec::new();
        let mut fixable = true;

        for (index, statement) in ctx.query().into_iter().enumerate() {
            if is_skippable_before_macro(&statement) {
                if macro_statements.is_empty() && !non_macro_found {
                    skippable_top_statements_end_index = index + 1;
                }
                continue;
            }

            let declarators = get_declarators_from_statement(&statement);
            let single_declarator = declarators.as_ref().is_none_or(|list| list.len() == 1);
            let calls = if let Some(declarators) = declarators {
                Either::Left(declarators.into_iter().map(|declarator| {
                    let declarator = declarator.ok()?;
                    Some((get_name_from_declarator(&declarator), declarator.range()))
                }))
            } else {
                Either::Right(std::iter::once(Some((
                    get_name_from_call_expr(&statement),
                    statement.range(),
                ))))
            };

            for call in calls {
                let Some((Some(name), range)) = call else {
                    non_macro_found = true;
                    continue;
                };
                let Some(&order_index) = order_map.get(name.as_ref()) else {
                    non_macro_found = true;
                    continue;
                };

                if first_issue.is_none() && (non_macro_found || order_index < previous_order_index)
                {
                    first_issue = Some((range, name));
                }
                previous_order_index = previous_order_index.max(order_index);
                fixable &= single_declarator;
                macro_statements.push((index, order_index));
            }
        }

        let (range, name) = first_issue?;
        Some(MacroOrderIssue {
            range,
            name,
            macro_statements: fixable.then_some(macro_statements),
            insert_at: skippable_top_statements_end_index,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let pretty_order = ctx
            .options()
            .order_or_default()
            .collect::<Vec<&str>>()
            .join(", ");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    <Emphasis>{state.name.text()}</Emphasis>" macro is out of order."
                },
            )
            .note(markup! {
                "Macros defined in <script setup> should be ordered as follows: "<Emphasis>{pretty_order}</Emphasis>
            })
            .note(markup! {
                "and be placed before any non-macro statements, except for type declarations, imports, exports or debugger statements."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let macro_statements = state.macro_statements.as_ref()?;
        let root = ctx.query();
        let mut mutation = root.clone().begin();
        let mut statements: Vec<_> = root.into_iter().map(Some).collect();
        let mut macros: Vec<_> = macro_statements
            .iter()
            .map(|&(index, order)| Some((order, statements[index].take()?)))
            .collect::<Option<_>>()?;
        macros.sort_by_key(|(order, _)| *order);

        let mut reordered: Vec<_> = statements.drain(..state.insert_at).flatten().collect();
        reordered.extend(macros.into_iter().map(|(_, statement)| statement));
        reordered.extend(statements.into_iter().flatten());

        let newline = make::token(T![;])
            .with_trailing_trivia([(TriviaPieceKind::Newline, "\n")])
            .trailing_trivia();
        for index in 1..reordered.len() {
            // New neighbors must not turn separate statements into a call or other expression.
            if token_set![
                T!['('],
                T!['['],
                T!['`'],
                T![+],
                T![-],
                T![<],
                JsSyntaxKind::JS_REGEX_LITERAL
            ]
            .contains(reordered[index].syntax().first_token()?.kind())
            {
                reordered[index - 1] = terminate_statement(&reordered[index - 1])?;
            }
            let trailing = reordered[index - 1].syntax().last_trailing_trivia()?;
            let leading = reordered[index].syntax().first_leading_trivia()?;
            if !trailing.pieces().any(|piece| piece.is_newline())
                && !leading
                    .pieces()
                    .take_while(|piece| !piece.is_comments())
                    .any(|piece| piece.is_newline())
            {
                reordered[index] = reordered[index]
                    .clone()
                    .prepend_trivia_pieces(newline.pieces())?;
            }
        }

        mutation.replace_node_discard_trivia(root.clone(), js_module_item_list(reordered));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Reorder compiler macros." }.to_owned(),
            mutation,
        ))
    }
}

pub struct MacroOrderIssue {
    range: TextRange,
    name: TokenText,
    macro_statements: Option<SmallVec<[(usize, usize); 4]>>,
    insert_at: usize,
}

fn get_name_from_call_expr(item: &AnyJsModuleItem) -> Option<TokenText> {
    let statement = item.as_any_js_statement()?;
    let expr_statement = statement.as_js_expression_statement()?;
    let expr = expr_statement.expression().ok()?;

    get_possible_macro_name(&expr)
}

fn get_name_from_declarator(declarator: &JsVariableDeclarator) -> Option<TokenText> {
    let initializer = declarator.initializer()?;
    let expr = initializer.expression().ok()?;
    let name = get_possible_macro_name(&expr)?;

    Some(name)
}

fn get_declarators_from_statement(item: &AnyJsModuleItem) -> Option<JsVariableDeclaratorList> {
    let statement = item.as_any_js_statement()?;
    let statement = statement.as_js_variable_statement()?;
    let declaration = statement.declaration().ok()?;

    Some(declaration.declarators())
}

fn get_possible_macro_name(expr: &AnyJsExpression) -> Option<TokenText> {
    let callee = expr.as_js_call_expression()?.callee().ok()?;
    let name = callee.get_callee_object_name()?.token_text_trimmed();

    if name == "withDefaults" {
        let args = expr.as_js_call_expression()?.arguments().ok()?;
        let first_arg = args.args().into_iter().next()?.ok()?;
        let first_arg_expr = first_arg.as_any_js_expression()?;
        let inner_name = get_possible_macro_name(first_arg_expr)?;

        if inner_name == "defineProps" {
            return Some(inner_name);
        } else {
            return None;
        }
    }

    Some(name)
}

fn terminate_statement(statement: &AnyJsModuleItem) -> Option<AnyJsModuleItem> {
    // Keep this local mutation from rebuilding the enclosing module.
    let statement = statement.clone_subtree();
    let Some(terminal) = statement
        .syntax()
        .last_token()?
        .parent()?
        .ancestors()
        .find_map(AnyJsStatement::cast)
    else {
        return Some(statement);
    };
    let trailing = terminal.syntax().last_trailing_trivia()?;
    let semicolon = make::token(T![;]).with_trailing_trivia_pieces(trailing.pieces());
    let new_terminal = match terminal.clone() {
        AnyJsStatement::JsExpressionStatement(terminal) if terminal.semicolon_token().is_none() => {
            terminal
                .with_trailing_trivia_pieces([])?
                .with_semicolon_token(Some(semicolon))
                .into()
        }
        AnyJsStatement::JsVariableStatement(terminal) if terminal.semicolon_token().is_none() => {
            terminal
                .with_trailing_trivia_pieces([])?
                .with_semicolon_token(Some(semicolon))
                .into()
        }
        AnyJsStatement::JsThrowStatement(terminal) if terminal.semicolon_token().is_none() => {
            terminal
                .with_trailing_trivia_pieces([])?
                .with_semicolon_token(Some(semicolon))
                .into()
        }
        _ => return Some(statement),
    };
    let mut mutation = statement.begin();
    mutation.replace_node_discard_trivia(terminal, new_terminal);
    AnyJsModuleItem::cast(mutation.commit())
}

/// Statements allowed before or between compiler macros.
fn is_skippable_before_macro(item: &AnyJsModuleItem) -> bool {
    const SKIPPABLE: SyntaxKindSet<biome_js_syntax::JsLanguage> = TsTypeAliasDeclaration::KIND_SET
        .union(TsInterfaceDeclaration::KIND_SET)
        .union(TsModuleDeclaration::KIND_SET)
        .union(JsDebuggerStatement::KIND_SET)
        .union(JsEmptyStatement::KIND_SET)
        .union(JsImport::KIND_SET)
        .union(JsExportNamedClause::KIND_SET)
        .union(JsExport::KIND_SET);
    SKIPPABLE.matches(item.syntax().kind())
}
