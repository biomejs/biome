use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlElementList, HtmlFileSource};
use biome_rowan::AstNode;
use biome_rule_options::use_media_caption::UseMediaCaptionOptions;

declare_lint_rule! {
    /// Enforces that `audio` and `video` elements must have a `track` for captions.
    ///
    /// Captions support users with hearing-impairments. They should be a transcription
    /// or translation of the dialogue, sound effects, musical cues, and other relevant
    /// audio information.
    ///
    /// :::note
    /// In `.html` files, this rule matches element names case-insensitively (e.g., `<VIDEO>`, `<video>`).
    ///
    /// In component-based frameworks (Vue, Svelte, Astro), only lowercase element names are checked.
    /// PascalCase variants like `<Video>` are assumed to be custom components and are ignored.
    /// :::
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
        version: "2.4.0",
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
    type Options = UseMediaCaptionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let source_type = ctx.source_type::<HtmlFileSource>();

        // Check if element is audio or video
        let element_name = node.name()?;
        let is_audio = if source_type.is_html() {
            element_name.text().eq_ignore_ascii_case("audio")
        } else {
            element_name.text() == "audio"
        };
        let is_video = if source_type.is_html() {
            element_name.text().eq_ignore_ascii_case("video")
        } else {
            element_name.text() == "video"
        };

        if !is_audio && !is_video {
            return None;
        }

        // Muted videos don't need captions (audio still requires captions)
        if is_video && node.find_attribute_by_name("muted").is_some() {
            return None;
        }

        // Check for track element with kind="captions" in children
        let html_element = node.as_html_element()?;
        // Skip analysis if we can't fully parse the element to avoid false positives
        if html_element.opening_element().is_err() {
            return None;
        }
        if has_caption_track(&html_element.children(), *source_type) {
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
fn has_caption_track(html_child_list: &HtmlElementList, source_type: HtmlFileSource) -> bool {
    html_child_list
        .into_iter()
        .find_map(|child| {
            let name = child.name()?;
            let is_track = if source_type.is_html() {
                name.text().eq_ignore_ascii_case("track")
            } else {
                name.text() == "track"
            };
            if !is_track {
                return None;
            }

            let kind_attr = child.find_attribute_by_name("kind")?;
            let initializer = kind_attr.initializer()?;
            let value = initializer.value().ok()?;
            let string_value = value.string_value()?;

            if string_value.eq_ignore_ascii_case("captions") {
                Some(())
            } else {
                None
            }
        })
        .is_some()
}
