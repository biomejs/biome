mod language;
mod test_case;

use crate::language::FormatNode;
pub use crate::language::Parse;
pub use crate::test_case::TestCase;
use biome_formatter::Printed;

pub fn run_format(format_node: &FormatNode) -> Printed {
    let formatted = format_node.format_node().unwrap();
    let printed = formatted.print();
    drop(formatted);
    printed.expect("Document to be valid")
}
