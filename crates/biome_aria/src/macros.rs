#[macro_export]
macro_rules! define_role {
    ( $( #[doc = $doc:literal] )+ $id:ident {
        PROPS: $p_value:expr,
        ROLES: $r_value:expr,
    }) => {
        $( #[doc = $doc] )*
        #[derive(Debug)]
        struct $id;

        impl $id {
            const PROPS: &[(&'static str, bool)] = &$p_value;
            const ROLES: &[&'static str] = &$r_value;
        }

        impl $crate::AriaRoleDefinition for $id {
            fn properties(&self) -> std::slice::Iter<(&str, bool)> {
                $id::PROPS.iter()
            }

            fn roles(&self) -> std::slice::Iter<&str> {
                $id::ROLES.iter()
            }
        }
    };
    ( $( #[doc = $doc:literal] )+ $id:ident {
        PROPS: $p_value:expr,
        ROLES: $r_value:expr,
        CONCEPTS: $c_value:expr,
    }) => {
        $( #[doc = $doc] )*
        #[derive(Debug)]
        struct $id;

        impl $id {
            const PROPS: &[(&'static str, bool)] = &$p_value;
            const ROLES: &[&'static str] = &$r_value;
            const CONCEPTS: &'static [(&'static str, &'static [(&'static str, &'static str)])] =
                $c_value;
        }

        impl $crate::AriaRoleDefinition for $id {
            fn properties(&self) -> std::slice::Iter<(&str, bool)> {
                $id::PROPS.iter()
            }

            fn roles(&self) -> std::slice::Iter<&str> {
                $id::ROLES.iter()
            }
        }

        impl AriaRoleDefinitionWithConcepts for $id {
            fn concepts_by_element_name<'a>(
                &self,
                element_name: &str,
            ) -> ElementsAndAttributes<'a> {
                for (concept_name, _attributes) in Self::CONCEPTS {
                    if *concept_name == element_name {
                        return Some(Self::CONCEPTS.iter());
                    }
                }
                None
            }
        }
    };
}

#[macro_export]
macro_rules! define_property {
    ( $id:ident {
        PROPERTY_TYPE: $property_type:literal,
        VALUES: $values:expr,
    }) => {
        #[derive(Debug)]
        struct $id;

        impl $id {
            const PROPERTY_TYPE: &'static str = &$property_type;
            const VALUES: &[&'static str] = &$values;
        }

        impl AriaPropertyDefinition for $id {
            fn values(&self) -> std::slice::Iter<&'static str> {
                $id::VALUES.iter()
            }

            fn property_type(&self) -> $crate::AriaPropertyTypeEnum {
                // SAFETY: PROPERTY_TYPE is internal and should not contain extraneous properties
                $crate::AriaPropertyTypeEnum::from_str($id::PROPERTY_TYPE).unwrap()
            }
        }
    };
}
