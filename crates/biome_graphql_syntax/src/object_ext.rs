use crate::{GraphqlObjectTypeDefinition, GraphqlObjectTypeExtension};

impl GraphqlObjectTypeDefinition {
    fn has_name(&self, expected: &str) -> bool {
        if let Some(name) = self.name().ok()
            && let Some(value_token) = name.value_token().ok()
            && value_token.text_trimmed() == expected
        {
            return true;
        }

        false
    }

    pub fn is_query(&self) -> bool {
        self.has_name("Query")
    }

    pub fn is_mutation(&self) -> bool {
        self.has_name("Mutation")
    }

    pub fn is_subscription(&self) -> bool {
        self.has_name("Subscription")
    }
}

impl GraphqlObjectTypeExtension {
    fn has_name(&self, expected: &str) -> bool {
        if let Some(name) = self.name().ok()
            && let Some(value_token) = name.value_token().ok()
            && value_token.text_trimmed() == expected
        {
            return true;
        }

        false
    }

    pub fn is_query(&self) -> bool {
        self.has_name("Query")
    }

    pub fn is_mutation(&self) -> bool {
        self.has_name("Mutation")
    }

    pub fn is_subscription(&self) -> bool {
        self.has_name("Subscription")
    }
}
