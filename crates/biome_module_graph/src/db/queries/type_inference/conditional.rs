use super::{ExpressionCaseLiteralInput, ExpressionTypeInput};
use crate::ModuleDb;
use crate::db::type_inference::{
    classify_expression_case_literal, classify_expression_conditional,
};
use crate::module_graph::ModuleInfoKind;
use crate::type_inference::profiling::{
    TypeInferenceProfileOrigin, TypeInferenceQueryKind, execute_query,
};
use biome_js_type_info::interned_types::ConditionalType;

/// Classifies an expression using only raw type information when the result is
/// available without resolving complete local type tables.
#[salsa::tracked(cycle_result = infer_expression_conditional_type_cycle_result)]
pub fn infer_expression_conditional_type<'db>(
    db: &'db dyn ModuleDb,
    input: ExpressionTypeInput<'db>,
) -> Option<ConditionalType> {
    let module = input.module(db);
    let expression = input.expression(db);
    execute_query(
        TypeInferenceQueryKind::Lookups,
        TypeInferenceProfileOrigin::exact(module, expression),
        "infer_expression_conditional_type",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return None;
            };
            if !js_info.infer_types {
                return None;
            }

            classify_expression_conditional(db, module, expression)
        },
    )
}

fn infer_expression_conditional_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: ExpressionTypeInput<'db>,
) -> Option<ConditionalType> {
    None
}

/// Checks whether a raw expression type can equal a switch-case literal.
#[salsa::tracked(cycle_result = infer_expression_case_literal_cycle_result)]
pub fn infer_expression_case_literal<'db>(
    db: &'db dyn ModuleDb,
    input: ExpressionCaseLiteralInput<'db>,
) -> Option<bool> {
    let module = input.module(db);
    let expression = input.expression(db);
    let literal = input.literal(db).clone();
    execute_query(
        TypeInferenceQueryKind::Lookups,
        TypeInferenceProfileOrigin::exact(module, expression),
        "infer_expression_case_literal",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return None;
            };
            if !js_info.infer_types {
                return None;
            }

            classify_expression_case_literal(db, module, expression, &literal)
        },
    )
}

fn infer_expression_case_literal_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: ExpressionCaseLiteralInput<'db>,
) -> Option<bool> {
    None
}
