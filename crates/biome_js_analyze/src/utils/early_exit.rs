//! Detection of statements after which the following code never runs.

use biome_js_syntax::{AnyJsStatement, AnyJsSwitchClause};
use biome_rowan::AstNodeList;

#[derive(Debug, Copy, Clone, Default)]
struct ScopeMetadata {
    // We are inside a breakable structure
    is_breakable: bool,
}

/// Returns `Some(())` if `statement` always ends with `return`, `throw`, `continue`, or a
/// `break` that leaves the enclosing statement, so code after it never runs.
///
/// The check only looks at the last statement of each block, both branches of an `if`, and
/// the cases of a `switch` that has a `default` case. Loops, labeled statements, and `try`
/// statements are treated as not exiting.
pub(crate) fn breaks_early(statement: AnyJsStatement) -> Option<()> {
    let mut stmt_stack = vec![(statement, ScopeMetadata::default())];
    while let Some((stmt, metadata)) = stmt_stack.pop() {
        match stmt {
            AnyJsStatement::JsBlockStatement(block_stmt) => {
                let Some(last) = block_stmt.statements().iter().last() else {
                    // empty statement block
                    return None;
                };
                stmt_stack.push((last, metadata));
            }
            AnyJsStatement::JsBreakStatement(_) => {
                if metadata.is_breakable {
                    // We are inside a breakable structure (switch statement)
                    // that we saw in a previous iteration.
                    return None;
                }
            }
            AnyJsStatement::JsContinueStatement(_)
            | AnyJsStatement::JsReturnStatement(_)
            | AnyJsStatement::JsThrowStatement(_) => {}
            AnyJsStatement::JsIfStatement(if_stmt) => {
                let Some(else_clause) = if_stmt.else_clause() else {
                    // No else clause
                    return None;
                };
                stmt_stack.push((if_stmt.consequent().ok()?, metadata));
                stmt_stack.push((else_clause.alternate().ok()?, metadata));
            }
            AnyJsStatement::JsSwitchStatement(switch_stmt) => {
                // To simplify, We do not take fallthroughs into account.
                // Thus, this can miss some useless else.
                let cases = switch_stmt.cases();
                let Some(last_case) = cases.last() else {
                    // Empty switch
                    return None;
                };
                if last_case.consequent().is_empty() {
                    return None;
                }
                // Without a `default` clause, no case may match and control
                // continues after the switch.
                if !cases
                    .iter()
                    .any(|clause| matches!(clause, AnyJsSwitchClause::JsDefaultClause(_)))
                {
                    return None;
                }
                for switch_clause in cases.iter() {
                    if let Some(last) = switch_clause.consequent().last() {
                        stmt_stack.push((last, ScopeMetadata { is_breakable: true }));
                    }
                }
            }
            _ => {
                // labeled statements, loops, try-catch, with statement, and others
                return None;
            }
        }
    }
    Some(())
}
