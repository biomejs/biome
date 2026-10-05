use super::*;

pub(super) fn convert_binary_operator(operator: &str) -> Result<BinaryOperator> {
    Ok(match operator {
        "+" => BinaryOperator::Add,
        "-" => BinaryOperator::Sub,
        "*" => BinaryOperator::Mul,
        "/" => BinaryOperator::Div,
        "%" => BinaryOperator::Rem,
        "**" => BinaryOperator::Exp,
        "==" => BinaryOperator::Eq,
        "===" => BinaryOperator::StrictEq,
        "!=" => BinaryOperator::Neq,
        "!==" => BinaryOperator::StrictNeq,
        "<" => BinaryOperator::Lt,
        "<=" => BinaryOperator::Lte,
        ">" => BinaryOperator::Gt,
        ">=" => BinaryOperator::Gte,
        "<<" => BinaryOperator::Shl,
        ">>" => BinaryOperator::Shr,
        ">>>" => BinaryOperator::UShr,
        "|" => BinaryOperator::BitOr,
        "^" => BinaryOperator::BitXor,
        "&" => BinaryOperator::BitAnd,
        "in" => BinaryOperator::In,
        "instanceof" => BinaryOperator::Instanceof,
        _ => {
            return Err(ReactCompilerError::CompilerOutput(format!(
                "unsupported binary operator `{operator}`"
            )));
        }
    })
}

pub(super) fn convert_logical_operator(operator: &str) -> Result<LogicalOperator> {
    Ok(match operator {
        "||" => LogicalOperator::Or,
        "&&" => LogicalOperator::And,
        "??" => LogicalOperator::NullishCoalescing,
        _ => {
            return Err(ReactCompilerError::CompilerOutput(format!(
                "unsupported logical operator `{operator}`"
            )));
        }
    })
}

pub(super) fn convert_unary_operator(operator: &str) -> Result<UnaryOperator> {
    Ok(match operator {
        "-" => UnaryOperator::Neg,
        "+" => UnaryOperator::Plus,
        "!" => UnaryOperator::Not,
        "~" => UnaryOperator::BitNot,
        "typeof" => UnaryOperator::TypeOf,
        "void" => UnaryOperator::Void,
        "delete" => UnaryOperator::Delete,
        _ => {
            return Err(ReactCompilerError::CompilerOutput(format!(
                "unsupported unary operator `{operator}`"
            )));
        }
    })
}

pub(super) fn convert_update_operator(operator: &str) -> Result<UpdateOperator> {
    Ok(match operator {
        "++" => UpdateOperator::Increment,
        "--" => UpdateOperator::Decrement,
        _ => {
            return Err(ReactCompilerError::CompilerOutput(format!(
                "unsupported update operator `{operator}`"
            )));
        }
    })
}

pub(super) fn convert_assignment_operator(operator: &str) -> Result<AssignmentOperator> {
    Ok(match operator {
        "=" => AssignmentOperator::Assign,
        "+=" => AssignmentOperator::AddAssign,
        "-=" => AssignmentOperator::SubAssign,
        "*=" => AssignmentOperator::MulAssign,
        "/=" => AssignmentOperator::DivAssign,
        "%=" => AssignmentOperator::RemAssign,
        "**=" => AssignmentOperator::ExpAssign,
        "<<=" => AssignmentOperator::ShlAssign,
        ">>=" => AssignmentOperator::ShrAssign,
        ">>>=" => AssignmentOperator::UShrAssign,
        "|=" => AssignmentOperator::BitOrAssign,
        "^=" => AssignmentOperator::BitXorAssign,
        "&=" => AssignmentOperator::BitAndAssign,
        "||=" => AssignmentOperator::OrAssign,
        "&&=" => AssignmentOperator::AndAssign,
        "??=" => AssignmentOperator::NullishAssign,
        _ => {
            return Err(ReactCompilerError::CompilerOutput(format!(
                "unsupported assignment operator `{operator}`"
            )));
        }
    })
}
