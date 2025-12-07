//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfTest;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfTest;
impl FormatRule<AnyCssIfTest> for FormatAnyCssIfTest {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfTest, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfTest::CssBogusIfTest(node) => node.format().fmt(f),
            AnyCssIfTest::CssIfMediaTest(node) => node.format().fmt(f),
            AnyCssIfTest::CssIfStyleTest(node) => node.format().fmt(f),
            AnyCssIfTest::CssIfSupportsTest(node) => node.format().fmt(f),
        }
    }
}
