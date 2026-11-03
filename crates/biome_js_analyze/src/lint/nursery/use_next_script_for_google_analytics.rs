use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsxAttributeValue, JsTemplateExpression, JsxAttributeList,
    jsx_ext::AnyJsxElement,
};
use biome_rowan::AstNode;
use biome_rule_options::use_next_script_for_google_analytics::UseNextScriptForGoogleAnalyticsOptions;

declare_lint_rule! {
    /// Enforce the use of the `@next/third-parties/google` components to load Google Analytics and Google Tag Manager.
    ///
    /// Adding the Google Analytics or Google Tag Manager script with a plain `<script>` element
    /// can slow down your page. The `GoogleAnalytics` and `GoogleTagManager` components from
    /// `@next/third-parties/google` load these scripts after the page becomes interactive.
    ///
    /// This rule reports `<script>` elements that load one of these scripts, either through
    /// the `src` attribute or through inline code in `dangerouslySetInnerHTML`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <script async src="https://www.googletagmanager.com/gtag/js?id=G-XYZ" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <script
    ///   dangerouslySetInnerHTML={{
    ///     __html: `
    ///       (function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
    ///       new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
    ///       j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
    ///       'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
    ///       })(window,document,'script','dataLayer','GTM-XYZ');
    ///     `,
    ///   }}
    /// />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import { GoogleAnalytics, GoogleTagManager } from "@next/third-parties/google";
    ///
    /// export default function RootLayout({ children }) {
    ///   return (
    ///     <html lang="en">
    ///       <GoogleTagManager gtmId="GTM-XYZ" />
    ///       <body>{children}</body>
    ///       <GoogleAnalytics gaId="G-XYZ" />
    ///     </html>
    ///   );
    /// }
    /// ```
    ///
    pub UseNextScriptForGoogleAnalytics {
        version: "next",
        name: "useNextScriptForGoogleAnalytics",
        language: "jsx",
        sources: &[RuleSource::EslintNext("next-script-for-ga").inspired()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for UseNextScriptForGoogleAnalytics {
    type Query = Ast<AnyJsxElement>;
    type State = GoogleService;
    type Signals = Option<Self::State>;
    type Options = UseNextScriptForGoogleAnalyticsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name().ok()?;
        if name.as_jsx_name()?.value_token().ok()?.text_trimmed() != "script" {
            return None;
        }

        let attributes = element.attributes();
        find_service_in_src(&attributes).or_else(|| find_service_in_inner_html(&attributes))
    }

    fn diagnostic(ctx: &RuleContext<Self>, service: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"script"</Emphasis>" element loads "{service}"."
                },
            )
            .note(markup! {
                "Loading "{service}" with a plain "<Emphasis>"script"</Emphasis>" element can slow down your page."
            })
            .note(markup! {
                "Use the "<Emphasis>{service.component()}</Emphasis>" component from "<Emphasis>"@next/third-parties/google"</Emphasis>" instead, which loads the script after the page becomes interactive. See the "<Hyperlink href="https://nextjs.org/docs/messages/next-script-for-ga">"Next.js docs"</Hyperlink>" for more details."
            }),
        )
    }
}

/// The Google service loaded by a reported `<script>` element.
#[derive(Clone, Copy, Debug)]
pub enum GoogleService {
    Analytics,
    TagManager,
}

impl GoogleService {
    /// Returns the `@next/third-parties/google` component that loads this service.
    const fn component(self) -> &'static str {
        match self {
            Self::Analytics => "GoogleAnalytics",
            Self::TagManager => "GoogleTagManager",
        }
    }
}

impl biome_console::fmt::Display for GoogleService {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Analytics => "Google Analytics",
            Self::TagManager => "Google Tag Manager",
        })
    }
}

/// The script URLs of each service, without the protocol.
const SERVICE_URLS: &[(&str, GoogleService)] = &[
    (
        "www.google-analytics.com/analytics.js",
        GoogleService::Analytics,
    ),
    ("www.googletagmanager.com/gtag/js", GoogleService::Analytics),
    ("www.googletagmanager.com/gtm.js", GoogleService::TagManager),
];

/// Returns the service whose script URL appears in the `src` attribute.
fn find_service_in_src(attributes: &JsxAttributeList) -> Option<GoogleService> {
    let value = attributes.find_by_name("src")?.initializer()?.value().ok()?;
    match value {
        AnyJsxAttributeValue::JsxString(string) => {
            find_service_in_text(string.inner_string_text().ok()?.text())
        }
        AnyJsxAttributeValue::JsxExpressionAttributeValue(value) => {
            find_service_in_expression(value.expression().ok()?)
        }
        AnyJsxAttributeValue::JsTemplateExpression(template) => {
            find_service_in_template(&template)
        }
        AnyJsxAttributeValue::AnyJsxTag(_) => None,
    }
}

/// Returns the service whose script URL appears in the inline code passed as
/// `dangerouslySetInnerHTML={{ __html: ... }}`.
///
/// Only inline object literals are inspected, because the code passed through a
/// variable or a function call isn't known.
fn find_service_in_inner_html(attributes: &JsxAttributeList) -> Option<GoogleService> {
    let value = attributes
        .find_by_name("dangerouslySetInnerHTML")?
        .initializer()?
        .value()
        .ok()?;
    let object = value
        .as_jsx_expression_attribute_value()?
        .expression()
        .ok()?
        .omit_parentheses();
    let html = object
        .as_js_object_expression()?
        .find_member("__html")?
        .as_js_property_object_member()?
        .value()
        .ok()?;
    find_service_in_expression(html)
}

/// Returns the service whose script URL appears in a string or template literal.
fn find_service_in_expression(expression: AnyJsExpression) -> Option<GoogleService> {
    match expression.omit_parentheses() {
        AnyJsExpression::JsTemplateExpression(template) => find_service_in_template(&template),
        expression => find_service_in_text(expression.as_static_value()?.as_string_constant()?),
    }
}

/// Returns the service whose script URL appears in a static part of a template literal.
fn find_service_in_template(template: &JsTemplateExpression) -> Option<GoogleService> {
    template.elements().into_iter().find_map(|element| {
        let chunk = element.as_js_template_chunk_element()?;
        find_service_in_text(chunk.template_chunk_token().ok()?.text_trimmed())
    })
}

fn find_service_in_text(text: &str) -> Option<GoogleService> {
    SERVICE_URLS
        .iter()
        .find_map(|(url, service)| text.contains(url).then_some(*service))
}
