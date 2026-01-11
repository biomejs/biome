use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlElementList};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Enforces that `audio` and `video` elements must have a `track` for captions.
    ///
    /// Captions support users with hearing-impairments. They should be a transcription
    /// or translation of the dialogue, sound effects, musical cues, and other relevant
    /// audio information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <video src="video.mp4"></video>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <audio src="audio.mp3">
    ///     <source src="audio.ogg" type="audio/ogg" />
    /// </audio>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <video src="video.mp4">
    ///     <track kind="captions" src="captions.vtt" />
    /// </video>
    /// ```
    ///
    /// ```html
    /// <audio src="audio.mp3">
    ///     <track kind="captions" src="captions.vtt" />
    /// </audio>
    /// ```
    ///
    /// ```html
    /// <video muted src="video.mp4"></video>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.2.2](https://www.w3.org/WAI/WCAG21/Understanding/captions-prerecorded)
    /// - [WCAG 1.2.3](https://www.w3.org/WAI/WCAG21/Understanding/audio-description-or-media-alternative-prerecorded)
    ///
    pub UseMediaCaption {
        version: "next",
        name: "useMediaCaption",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("media-has-caption").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseMediaCaption {
    type Query = Ast<AnyHtmlElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // Check if element is audio or video
        let element_name = node.name()?;
        if element_name != "audio" && element_name != "video" {
            return None;
        }

        // If element has muted attribute, it's valid (no caption needed)
        if node.find_attribute_by_name("muted").is_some() {
            return None;
        }

        // Check for track element with kind="captions" in children
        let html_element = node.as_html_element()?;
        if html_element.opening_element().is_ok() && has_caption_track(&html_element.children()) {
            return None;
        }

        // No muted attribute and no caption track found - emit diagnostic
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Provide a "<Emphasis>"track"</Emphasis>" for captions when using "<Emphasis>"audio"</Emphasis>" or "<Emphasis>"video"</Emphasis>" elements."
            },
        )
        .note(markup! {
            "Captions support users with hearing-impairments. They should be a transcription or translation of the dialogue, sound effects, musical cues, and other relevant audio information."
        });

        Some(diagnostic)
    }
}

/// Checks if the given `HtmlElementList` has a `track` element with `kind="captions"`.
fn has_caption_track(html_child_list: &HtmlElementList) -> bool {
    html_child_list.into_iter().any(|child| {
        // Check if element is a track element (works for both HtmlElement and HtmlSelfClosingElement)
        let Some(name) = child.name() else {
            return false;
        };

        if name.text() != "track" {
            return false;
        }

        // Check if track has kind="captions"
        let Some(kind_attr) = child.find_attribute_by_name("kind") else {
            return false;
        };
        let Some(initializer) = kind_attr.initializer() else {
            return false;
        };
        let Ok(value) = initializer.value() else {
            return false;
        };
        let Some(string_value) = value.string_value() else {
            return false;
        };
        string_value.eq_ignore_ascii_case("captions")
    })
}
