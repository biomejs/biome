use crate::VueDirective;
use biome_string_case::StrLikeExtension;

const VUE_BUILTIN_DIRECTIVES: [&str; 15] = [
    "v-bind",
    "v-cloak",
    "v-else",
    "v-else-if",
    "v-for",
    "v-html",
    "v-if",
    "v-memo",
    "v-model",
    "v-on",
    "v-once",
    "v-pre",
    "v-show",
    "v-slot",
    "v-text",
];

impl VueDirective {
    /// Returns `true` when this directive's name exactly matches a built-in
    /// Vue directive, ignoring ASCII case.
    #[inline]
    pub fn is_builtin(&self) -> bool {
        self.name_token().is_ok_and(|t| {
            let name = t.text_trimmed().to_ascii_lowercase_cow();
            VUE_BUILTIN_DIRECTIVES.binary_search(&name.as_ref()).is_ok()
        })
    }

    #[inline]
    pub fn is_binding(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-bind"))
    }

    #[inline]
    pub fn is_two_way_binding(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-model"))
    }

    #[inline]
    pub fn is_event_listener(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-on"))
    }

    #[inline]
    pub fn is_for(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-for"))
    }

    #[inline]
    pub fn is_slot(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-slot"))
    }

    #[inline]
    pub fn is_if(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-if"))
    }
}

#[test]
fn test_builtin_directives_order() {
    assert!(VUE_BUILTIN_DIRECTIVES.is_sorted());
}
