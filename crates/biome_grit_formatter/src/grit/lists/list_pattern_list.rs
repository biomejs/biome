use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::GritListPatternList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritListPatternList;
impl FormatRule<GritListPatternList> for FormatGritListPatternList {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritListPatternList, f: &mut GritFormatter) -> FormatResult<()> {
        let mut filler = f.fill();

        for (element, formatted) in node.iter().zip(node.format_separated(",")) {
            filler.entry(
                &format_once(|f| {
                    if get_lines_before(element?.syntax()) > 0 {
                        write!(f, [empty_line()])
                    } else {
                        write!(f, [soft_line_break_or_space()])
                    }
                }),
                &formatted,
            );
        }

        filler.finish()
    }
}
