use super::{call_compiler::named_args_from_node, compilation_context::NodeCompilationContext};
use crate::{CompileError, NodeLikeArgumentError};
use biome_grit_syntax::{AnyGritLiteral, AnyGritPattern, GritNodeLike};
use grit_pattern_matcher::pattern::{Point, Range};

pub(crate) struct RangeCompiler;

impl RangeCompiler {
    pub(crate) fn from_node(
        node: &GritNodeLike,
        context: &mut NodeCompilationContext,
    ) -> Result<Range, CompileError> {
        let mut start_line = None;
        let mut start_column = None;
        let mut end_line = None;
        let mut end_column = None;

        for (name, pattern) in named_args_from_node(node, "range", context)? {
            let bound = match name.as_str() {
                "start_line" => &mut start_line,
                "start_column" => &mut start_column,
                "end_line" => &mut end_line,
                "end_column" => &mut end_column,
                _ => {
                    return Err(NodeLikeArgumentError::UnknownArgument {
                        name: "range".to_string(),
                        argument: name,
                        valid_args: ["start_line", "start_column", "end_line", "end_column"]
                            .map(str::to_string)
                            .to_vec(),
                    }
                    .into());
                }
            };
            if bound.is_some() {
                return Err(NodeLikeArgumentError::DuplicateArguments { name }.into());
            }

            let AnyGritPattern::AnyGritLiteral(AnyGritLiteral::GritIntLiteral(literal)) = pattern
            else {
                return Err(CompileError::InvalidRange(format!(
                    "{name} must be an unsigned integer literal"
                )));
            };
            *bound = Some(
                literal
                    .value_token()?
                    .text_trimmed()
                    .parse::<u32>()
                    .map_err(|_| {
                        CompileError::InvalidRange(format!(
                            "{name} must be an integer between 0 and {}",
                            u32::MAX
                        ))
                    })?,
            );
        }

        Ok(Range {
            start: Point::new(start_line, start_column).map_err(|_| {
                CompileError::InvalidRange("start_column requires start_line".to_string())
            })?,
            end: Point::new(end_line, end_column).map_err(|_| {
                CompileError::InvalidRange("end_column requires end_line".to_string())
            })?,
        })
    }
}
