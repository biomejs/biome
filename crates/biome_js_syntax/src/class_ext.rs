use crate::AnyJsClassMemberName;

impl AnyJsClassMemberName {
    /// Returns whether this class member name matches `expected`.
    ///
    /// Private names require their leading `#`. Computed names and
    /// metavariables do not match. Malformed literal and private names return
    /// `None`.
    pub fn class_member_name_matches(&self, expected: &str) -> Option<bool> {
        match self {
            Self::JsLiteralMemberName(name) => Some(name.value().ok()?.text_trimmed() == expected),
            Self::JsPrivateClassMemberName(name) => {
                let identifier = name.id_token().ok()?.token_text_trimmed();
                Some(
                    expected
                        .strip_prefix('#')
                        .is_some_and(|expected| identifier == expected),
                )
            }
            Self::JsComputedMemberName(_) | Self::JsMetavariable(_) => Some(false),
        }
    }
}

#[cfg(test)]
mod tests {

    use biome_js_factory::syntax::AnyJsClass;
    use biome_js_parser::{JsParserOptions, parse_module};
    use biome_rowan::AstNode;

    #[test]
    fn matches_public_and_private_class_member_names() {
        let parsed = parse_module(
            "class Example { public() {} #private() {} }",
            JsParserOptions::default(),
        );
        let class = parsed
            .tree()
            .syntax()
            .descendants()
            .find_map(AnyJsClass::cast)
            .expect("class must parse");
        let mut members = class.members().into_iter();
        let public = members
            .next()
            .expect("public method must exist")
            .name()
            .expect("public method name must be valid")
            .expect("public method must be named");
        let private = members
            .next()
            .expect("private method must exist")
            .name()
            .expect("private method name must be valid")
            .expect("private method must be named");

        assert_eq!(public.class_member_name_matches("public"), Some(true));
        assert_eq!(public.class_member_name_matches("#public"), Some(false));
        assert_eq!(private.class_member_name_matches("#private"), Some(true));
        assert_eq!(private.class_member_name_matches("private"), Some(false));
    }
}
