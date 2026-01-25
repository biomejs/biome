use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyVueDirective, VueModifierList};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vue_valid_v_on::UseVueValidVOnOptions;
use phf::phf_set;

declare_lint_rule! {
    /// Enforce valid `v-on` directives with proper arguments, modifiers, and handlers.
    ///
    /// This rule reports v-on directives in the following cases:
    /// - The directive does not have an event name. E.g. `<div v-on="foo"></div>`
    /// - The directive has invalid modifiers. E.g. `<div v-on:click.bogus="foo"></div>`
    /// - The directive is missing a handler expression. E.g. `<div v-on:click></div>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <Foo v-on />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <Foo v-on:click="foo" />
    /// ```
    ///
    pub UseVueValidVOn {
        version: "2.3.6",
        name: "useVueValidVOn",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-on").same()],
    }
}

pub enum ViolationKind {
    MissingEventName,
    InvalidModifier(TextRange),
    MissingHandler,
}

impl Rule for UseVueValidVOn {
    type Query = Ast<AnyVueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVOnOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let options = ctx.options();

        match node {
            AnyVueDirective::VueDirective(vue_directive) => {
                if vue_directive.name_token().ok()?.text_trimmed() != "v-on" {
                    return None;
                }

                // Check for missing event name
                if vue_directive.arg().is_none() {
                    return Some(ViolationKind::MissingEventName);
                }

                // Check for invalid modifiers
                if let Some(invalid_range) =
                    find_invalid_modifiers(&vue_directive.modifiers(), options.modifiers.as_ref())
                {
                    return Some(ViolationKind::InvalidModifier(invalid_range));
                }

                // Check for missing handler
                if vue_directive.initializer().is_none() {
                    return Some(ViolationKind::MissingHandler);
                }

                None
            }
            AnyVueDirective::VueVOnShorthandDirective(dir) => {
                // Shorthand always has an argument (parser enforces this)

                // Check for invalid modifiers
                if let Some(invalid_range) =
                    find_invalid_modifiers(&dir.modifiers(), options.modifiers.as_ref())
                {
                    return Some(ViolationKind::InvalidModifier(invalid_range));
                }

                // Check for missing handler
                if dir.initializer().is_none() {
                    return Some(ViolationKind::MissingHandler);
                }

                None
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(match state {
            ViolationKind::MissingEventName => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The v-on directive is missing an event name."
                },
            )
            .note(markup! {
                    "Provide an event name after the colon, e.g. v-on:click=\"handler\"."
            }),
            ViolationKind::InvalidModifier(invalid_range) => {
                let mut allowed_modifiers = VALID_MODIFIERS
                    .iter()
                    .map(|modifier| (*modifier).to_string())
                    .collect::<Vec<_>>();
                if let Some(extra) = ctx.options().modifiers.as_ref() {
                    allowed_modifiers.extend(extra.iter().cloned());
                }

                RuleDiagnostic::new(
                    rule_category!(),
                    invalid_range,
                    markup! {
                        "Invalid v-on modifier."
                    },
                )
                .note(markup! {
                        "Remove or correct the invalid modifier."
                })
                .footer_list(
                    markup! {
                            "Allowed modifiers:"
                    },
                    allowed_modifiers,
                )
            }
            ViolationKind::MissingHandler => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The v-on directive for this event is missing a handler expression."
                },
            )
            .note(markup! {
                    "Add a handler, e.g. v-on:click=\"onClick\"."
            }),
        })
    }
}

fn find_invalid_modifiers(
    modifiers: &VueModifierList,
    additional_allowed_modifiers: Option<&Vec<String>>,
) -> Option<TextRange> {
    for modifier in modifiers {
        if let Ok(modifier_token) = modifier.modifier_token() {
            let modifier_text = modifier_token.text();
            if modifier_text.len() == 1 {
                // allow single character modifiers (e.g. key codes)
                continue;
            }
            if VALID_MODIFIERS.contains(modifier_text) {
                continue;
            }
            if VALID_KEY_ALIASES_LIST.contains(modifier_text) {
                continue;
            }
            if modifier_text.chars().all(|c| c.is_ascii_digit()) {
                // allow numeric key codes
                continue;
            }
            if let Some(additional) = additional_allowed_modifiers
                && additional.iter().any(|s| s.as_str() == modifier_text)
            {
                continue;
            }
            return Some(modifier.range());
        }
    }
    None
}

static VALID_MODIFIERS: phf::Set<&'static str> = phf_set! {
    "stop", "prevent", "capture", "self", "ctrl", "shift", "alt", "meta", "native", "once", "left",
    "right", "middle", "passive", "esc", "tab", "enter", "space", "up", "down", "delete", "exact",
};

/// See: https://github.com/vuejs/eslint-plugin-vue/blob/master/lib/utils/key-aliases.json
static VALID_KEY_ALIASES_LIST: phf::Set<&'static str> = phf_set! {
    "a-v-r-input",
    "a-v-r-power",
    "accept",
    "again",
    "all-candidates",
    "alphanumeric",
    "alt",
    "alt-graph",
    "app-switch",
    "arrow-down",
    "arrow-left",
    "arrow-right",
    "arrow-up",
    "attn",
    "audio-balance-left",
    "audio-balance-right",
    "audio-bass-boost-down",
    "audio-bass-boost-toggle",
    "audio-bass-boost-up",
    "audio-fader-front",
    "audio-fader-rear",
    "audio-surround-mode-next",
    "audio-treble-down",
    "audio-treble-up",
    "audio-volume-down",
    "audio-volume-mute",
    "audio-volume-up",
    "backspace",
    "brightness-down",
    "brightness-up",
    "browser-back",
    "browser-favorites",
    "browser-forward",
    "browser-home",
    "browser-refresh",
    "browser-search",
    "browser-stop",
    "call",
    "camera",
    "camera-focus",
    "cancel",
    "caps-lock",
    "channel-down",
    "channel-up",
    "clear",
    "close",
    "closed-caption-toggle",
    "code-input",
    "color-f0-red",
    "color-f1-green",
    "color-f2-yellow",
    "color-f3-blue",
    "color-f4-grey",
    "color-f5-brown",
    "compose",
    "context-menu",
    "control",
    "convert",
    "copy",
    "cr-sel",
    "cut",
    "d-v-r",
    "dead",
    "delete",
    "dimmer",
    "display-swap",
    "eisu",
    "eject",
    "end",
    "end-call",
    "enter",
    "erase-eof",
    "escape",
    "ex-sel",
    "execute",
    "exit",
    "f1",
    "f10",
    "f11",
    "f12",
    "f2",
    "f3",
    "f4",
    "f5",
    "f6",
    "f7",
    "f8",
    "f9",
    "favorite-clear0",
    "favorite-clear1",
    "favorite-clear2",
    "favorite-clear3",
    "favorite-recall0",
    "favorite-recall1",
    "favorite-recall2",
    "favorite-recall3",
    "favorite-store0",
    "favorite-store1",
    "favorite-store2",
    "favorite-store3",
    "final-mode",
    "find",
    "fn",
    "fn-lock",
    "go-back",
    "go-home",
    "group-first",
    "group-last",
    "group-next",
    "group-previous",
    "guide",
    "guide-next-day",
    "guide-previous-day",
    "hangul-mode",
    "hanja-mode",
    "hankaku",
    "headset-hook",
    "help",
    "hibernate",
    "hiragana",
    "hiragana-katakana",
    "home",
    "hyper",
    "info",
    "insert",
    "instant-replay",
    "junja-mode",
    "kana-mode",
    "kanji-mode",
    "katakana",
    "key11",
    "key12",
    "last-number-redial",
    "launch-application1",
    "launch-application2",
    "launch-calendar",
    "launch-contacts",
    "launch-mail",
    "launch-media-player",
    "launch-music-player",
    "launch-phone",
    "launch-screen-saver",
    "launch-spreadsheet",
    "launch-web-browser",
    "launch-web-cam",
    "launch-word-processor",
    "link",
    "list-program",
    "live-content",
    "lock",
    "log-off",
    "mail-forward",
    "mail-reply",
    "mail-send",
    "manner-mode",
    "media-apps",
    "media-close",
    "media-fast-forward",
    "media-last",
    "media-next-track",
    "media-pause",
    "media-play",
    "media-play-pause",
    "media-previous-track",
    "media-record",
    "media-rewind",
    "media-skip-backward",
    "media-skip-forward",
    "media-step-backward",
    "media-step-forward",
    "media-stop",
    "media-top-menu",
    "media-track-next",
    "media-track-previous",
    "meta",
    "microphone-toggle",
    "microphone-volume-down",
    "microphone-volume-mute",
    "microphone-volume-up",
    "mode-change",
    "navigate-in",
    "navigate-next",
    "navigate-out",
    "navigate-previous",
    "new",
    "next-candidate",
    "next-favorite-channel",
    "next-user-profile",
    "non-convert",
    "notification",
    "num-lock",
    "on-demand",
    "open",
    "page-down",
    "page-up",
    "pairing",
    "paste",
    "pause",
    "pin-p-down",
    "pin-p-move",
    "pin-p-toggle",
    "pin-p-up",
    "play-speed-down",
    "play-speed-reset",
    "play-speed-up",
    "power",
    "previous-candidate",
    "print",
    "print-screen",
    "process",
    "random-toggle",
    "rc-low-battery",
    "record-speed-next",
    "redo",
    "rf-bypass",
    "romaji",
    "s-t-b-input",
    "s-t-b-power",
    "save",
    "scan-channels-toggle",
    "screen-mode-next",
    "scroll-lock",
    "select",
    "settings",
    "shift",
    "single-candidate",
    "soft1",
    "soft2",
    "soft3",
    "soft4",
    "speech-correction-list",
    "speech-input-toggle",
    "spell-check",
    "split-screen-toggle",
    "standby",
    "subtitle",
    "super",
    "symbol",
    "symbol-lock",
    "t-v",
    "t-v-antenna-cable",
    "t-v-audio-description",
    "t-v-audio-description-mix-down",
    "t-v-audio-description-mix-up",
    "t-v-contents-menu",
    "t-v-data-service",
    "t-v-input",
    "t-v-input-component1",
    "t-v-input-component2",
    "t-v-input-composite1",
    "t-v-input-composite2",
    "t-v-input-h-d-m-i1",
    "t-v-input-h-d-m-i2",
    "t-v-input-h-d-m-i3",
    "t-v-input-h-d-m-i4",
    "t-v-input-v-g-a1",
    "t-v-media-context",
    "t-v-network",
    "t-v-number-entry",
    "t-v-power",
    "t-v-radio-service",
    "t-v-satellite",
    "t-v-satellite-b-s",
    "t-v-satellite-c-s",
    "t-v-satellite-toggle",
    "t-v-terrestrial-analog",
    "t-v-terrestrial-digital",
    "t-v-timer",
    "t-v3-d-mode",
    "tab",
    "teletext",
    "undo",
    "unidentified",
    "video-mode-next",
    "voice-dial",
    "wake-up",
    "wink",
    "zenkaku",
    "zenkaku-hankaku",
    "zoom-in",
    "zoom-out",
    "zoom-toggle",
};
