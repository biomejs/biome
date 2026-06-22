use crate::VueDirective;

impl VueDirective {
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
    pub fn is_if(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-if"))
    }
}
