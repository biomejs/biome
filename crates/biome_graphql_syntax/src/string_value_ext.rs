use crate::GraphqlStringValue;

impl GraphqlStringValue {
    /// Check if the string is a block string
    /// Block strings are enclosed by triple quotes
    /// and can span multiple lines
    /// Example:
    /// ```graphql
    /// """
    /// This is a block string
    /// that spans multiple lines
    /// """
    /// ```
    pub fn is_block(&self) -> bool {
        self.graphql_string_literal_token()
            .is_ok_and(|token| token.text_trimmed().starts_with("\"\"\""))
    }
}
