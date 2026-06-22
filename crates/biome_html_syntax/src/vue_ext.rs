use crate::VueDirective;

impl VueDirective {
    pub fn is_binding(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-bind"))
    }

    pub fn is_two_way_binding(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-model"))
    }

    pub fn is_event_listener(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-on"))
    }

    pub fn is_for(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-for"))
    }

    pub fn is_if(&self) -> bool {
        self.name_token()
            .is_ok_and(|t| t.text_trimmed().eq_ignore_ascii_case("v-if"))
    }
}
