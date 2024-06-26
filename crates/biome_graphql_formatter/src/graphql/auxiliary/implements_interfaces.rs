use crate::prelude::*;
use biome_formatter::write;
use biome_graphql_syntax::{GraphqlImplementsInterfaces, GraphqlImplementsInterfacesFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGraphqlImplementsInterfaces;
impl FormatNodeRule<GraphqlImplementsInterfaces> for FormatGraphqlImplementsInterfaces {
    fn fmt_fields(
        &self,
        node: &GraphqlImplementsInterfaces,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        let GraphqlImplementsInterfacesFields {
            implements_token,
            amp_token,
            interfaces,
        } = node.as_fields();

        if let Some(amp_token) = amp_token {
            write!(f, [format_removed(&amp_token)])?;
        }

        write!(
            f,
            [implements_token.format(), space(), interfaces.format(),]
        )
    }
}
