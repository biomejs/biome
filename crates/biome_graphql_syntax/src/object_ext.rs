use crate::{GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension};

impl GraphqlObjectTypeDefinition {
    pub fn is_mutation(&self) -> bool {
        if let Some(name) = self.name().ok()
            && let Some(value_token) = name.value_token().ok()
            && value_token.to_string() == "Mutation"
        {
            return true;
        }

        false
    }

    pub fn is_query(&self) -> bool {
        if let Some(name) = self.name().ok()
            && let Some(value_token) = name.value_token().ok()
            && value_token.to_string() == "Query"
        {
            return true;
        }

        false
    }
}

impl GraphqlObjectTypeExtension {
    pub fn is_mutation(&self) -> bool {
        if let Some(name) = self.name().ok()
            && let Some(value_token) = name.value_token().ok()
            && value_token.to_string() == "Mutation"
        {
            return true;
        }

        false
    }

    pub fn is_query(&self) -> bool {
        if let Some(name) = self.name().ok()
            && let Some(value_token) = name.value_token().ok()
            && value_token.to_string() == "Query"
        {
            return true;
        }

        false
    }
}
