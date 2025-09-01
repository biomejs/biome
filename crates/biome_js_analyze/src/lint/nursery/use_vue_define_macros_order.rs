use std::collections::HashMap;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make::{self, js_module_item_list};
use biome_js_syntax::{
    AnyJsExpression, AnyJsModuleItem, JsModuleItemList, JsSyntaxKind, JsVariableDeclarator,
    JsVariableDeclaratorList, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TokenText, TriviaPieceKind};
use biome_rule_options::use_vue_define_macros_order::UseVueDefineMacrosOrderOptions;

use crate::JsRuleAction;

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
        version: "next",
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
        // TODO: Need to run this only on <script setup> blocks.
        let path = ctx.file_path();
        if path.extension() != Some("vue") {
            return None;
        }

        let order = &ctx.options().order;
        let order_map: HashMap<&str, usize> = order
            .iter()
            .enumerate()
            .map(|(idx, s)| (s.as_ref(), idx))
            .collect();

        struct FoundMacro {
            order_index: usize,
            fixable_statement_index: Option<usize>,
            range: TextRange,
            has_out_of_order_content_prior: bool,
        }

        let mut non_macro_found = false;
        let mut skippable_top_statements_end_index = 0;
        let mut found_macro: Option<FoundMacro> = None;

        for (index, statement) in ctx.query().into_iter().enumerate() {
            if is_skippable_before_macro(&statement) {
                if found_macro.is_none() && !non_macro_found {
                    skippable_top_statements_end_index = index + 1;
                }

                continue;
            }

            if let Some(name) = get_name_from_call_expr(&statement)
                && let Some(&order_index) = order_map.get(name.as_ref())
            {
                match &found_macro {
                    None => {
                        found_macro = Some(FoundMacro {
                            order_index,
                            fixable_statement_index: Some(index),
                            range: statement.range(),
                            has_out_of_order_content_prior: non_macro_found,
                        });
                    }
                    Some(c) if order_index < c.order_index => {
                        found_macro = Some(FoundMacro {
                            order_index,
                            fixable_statement_index: Some(index),
                            range: statement.range(),
                            has_out_of_order_content_prior: true,
                        });
                    }
                    _ => {
                        // Same/higher order, do nothing
                    }
                }
            } else if let Some(declarators) = get_declarators_from_statement(&statement) {
                // If there are multiple declarators, fixing is not trivial, so we won't suggest it.
                let fixable_statement_index = if (&declarators).into_iter().count() == 1 {
                    Some(index)
                } else {
                    None
                };

                for declarator in declarators {
                    let Some(declarator) = declarator.ok() else {
                        non_macro_found = true;
                        continue;
                    };

                    if let Some(name) = get_name_from_declarator(&declarator)
                        && let Some(&order_index) = order_map.get(name.as_ref())
                    {
                        let range = declarator.range();
                        match &found_macro {
                            None => {
                                found_macro = Some(FoundMacro {
                                    order_index,
                                    fixable_statement_index,
                                    range,
                                    has_out_of_order_content_prior: non_macro_found,
                                });
                            }
                            Some(c) => {
                                let has_out_of_order_content_prior = if order_index < c.order_index
                                {
                                    true
                                } else {
                                    c.has_out_of_order_content_prior || non_macro_found
                                };

                                found_macro = Some(FoundMacro {
                                    order_index,
                                    fixable_statement_index,
                                    range,
                                    has_out_of_order_content_prior,
                                });
                            }
                        }
                    } else {
                        non_macro_found = true;
                    }
                }
            } else {
                non_macro_found = true;
            }
        }

        if let Some(vue_macro) = found_macro
            && vue_macro.has_out_of_order_content_prior
        {
            return Some(MacroOrderIssue {
                range: vue_macro.range,
                name: order[vue_macro.order_index].clone(),
                move_from_to: vue_macro
                    .fixable_statement_index
                    .map(|index| (index, skippable_top_statements_end_index)),
            });
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let order = &ctx.options().order;

        let pretty_order = order
            .iter()
            .map(|s| s.as_ref())
            .collect::<Vec<&str>>()
            .join(", ");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    <Emphasis>{state.name}</Emphasis>" macro is out of order."
                },
            )
            .note(markup! {
                "Macros defined in <script setup> should be ordered as follows: "<Emphasis>{pretty_order}</Emphasis>
            })
            // and before skippable content
            .note(markup! {
                "and be placed before any non-macro statements, except for type declarations, imports, exports or debugger statements."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let (from, to) = state.move_from_to?;

        let root = ctx.query();
        let mut mutation = root.clone().begin();

        let mut statements: Vec<AnyJsModuleItem> = root.into_iter().collect();
        let statement = statements.remove(from);

        // Need to add newline after the moved statement, because it may not have one.
        let trailing_trivia = make::token(T![;])
            .with_trailing_trivia([(TriviaPieceKind::Newline, "\n")])
            .trailing_trivia()
            .pieces();

        let statement = statement.with_trailing_trivia_pieces(trailing_trivia)?;
        statements.insert(to, statement);

        let new_modules_list = js_module_item_list(statements);
        mutation.replace_node(root.clone(), new_modules_list);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Reorder macro "<Emphasis>{state.name}</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

pub struct MacroOrderIssue {
    range: TextRange,
    name: Box<str>,
    move_from_to: Option<(usize, usize)>,
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

/// What can appear before the macro without causing a violation.
fn is_skippable_before_macro(item: &AnyJsModuleItem) -> bool {
    matches!(
        item.syntax().kind(),
        JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION
            | JsSyntaxKind::TS_INTERFACE_DECLARATION
            | JsSyntaxKind::TS_MODULE_DECLARATION
            | JsSyntaxKind::JS_DEBUGGER_STATEMENT
            | JsSyntaxKind::JS_EMPTY_STATEMENT
            | JsSyntaxKind::JS_IMPORT
            | JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE
            | JsSyntaxKind::JS_EXPORT
    )
}
