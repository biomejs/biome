use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{AnyJsxChild, JsxElement, TextRange};
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Enforces that `audio` and `video` elements must have a `track` for captions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    /// ```jsx,expect_diagnostic
    /// 	<video />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// 	<audio>child</audio>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// 	<audio>
    /// 		<track kind="captions" {...props} />
    /// 	</audio>
    /// ```
    ///
    /// ```jsx
    /// 	<video muted {...props}></video>
    /// ```
    pub UseMediaCaption {
        version: "1.0.0",
        name: "useMediaCaption",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("media-has-caption")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseMediaCaption {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let has_audio_or_video = matches!(
            node.name_value_token().ok()?.text_trimmed(),
            "video" | "audio"
        );
        let has_muted = node.find_attribute_by_name("muted").is_some();
        let has_spread_prop = node
            .attributes()
            .into_iter()
            .any(|attr| attr.as_jsx_spread_attribute().is_some());

        if !has_audio_or_video || has_muted || has_spread_prop {
            return None;
        }

        match node {
            AnyJsxElement::JsxOpeningElement(_) => {
                let jsx_element = node.parent::<JsxElement>()?;
                let has_track = jsx_element
                    .children()
                    .into_iter()
                    .filter_map(|child| {
                        let any_jsx = match child {
                            AnyJsxChild::JsxElement(element) => {
                                Some(AnyJsxElement::from(element.opening_element().ok()?))
                            }
                            AnyJsxChild::JsxSelfClosingElement(element) => {
                                Some(AnyJsxElement::from(element))
                            }
                            _ => None,
                        }?;

                        let has_track = any_jsx.name_value_token().ok()?.text_trimmed() == "track";
                        let has_valid_kind = &any_jsx
                            .find_attribute_by_name("kind")?
                            .initializer()?
                            .value()
                            .ok()?
                            .as_jsx_string()?
                            .inner_string_text()
                            .ok()?
                            .to_ascii_lowercase_cow()
                            == "captions";

                        Some(has_track && has_valid_kind)
                    })
                    .any(|is_valid| is_valid);

                if !has_track {
                    return Some(jsx_element.range());
                }
            }
            _ => return Some(node.range()),
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {"Provide a "<Emphasis>"track"</Emphasis>" for captions when using "<Emphasis>"audio"</Emphasis>" or "<Emphasis>"video"</Emphasis>" elements."}.to_owned(),
        )
        .note("Captions support users with hearing-impairments. They should be a transcription or translation of the dialogue, sound effects, musical cues, and other relevant audio information.");

        Some(diagnostic)
    }
}
