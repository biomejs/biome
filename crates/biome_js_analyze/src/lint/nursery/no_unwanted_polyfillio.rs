use std::sync::LazyLock;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};
use regex::Regex;

use crate::{
    nextjs::{is_next_import, NextUtility},
    services::semantic::Semantic,
};

declare_lint_rule! {
    /// Prevent duplicate polyfills from Polyfill.io.
    ///
    /// You are using polyfills from Polyfill.io and including polyfills already shipped with Next.js.
    /// This unnecessarily increases page weight which can affect loading performance.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <script src='https://polyfill.io/v3/polyfill.min.js?features=AbortController,Object.fromEntries'></script>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// import NextScript from 'next/script';
    ///
    /// export function MyApp({ Component, pageProps }) {
    ///   return <NextScript src='https://polyfill.io/v3/polyfill.min.js?features=Array.prototype.copyWithin' />
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <script src='https://polyfill.io/v3/polyfill.min.js?features=AbortController'></script>
    ///   <script src='https://polyfill.io/v3/polyfill.min.js?features=IntersectionObserver'></script>
    ///   <Script src='https://polyfill.io/v3/polyfill.min.js?features=IntersectionObserver' />
    ///   <Script src='https://polyfill-fastly.io/v3/polyfill.min.js?features=IntersectionObserver' />
    /// </>
    /// ```
    ///
    pub NoUnwantedPolyfillio {
        version: "next",
        name: "noUnwantedPolyfillio",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-unwanted-polyfillio")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Next],
    }
}

pub struct RuleState {
    range: TextRange,
    unwanted_features_message: Box<str>,
}

impl Rule for NoUnwantedPolyfillio {
    type Query = Semantic<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let jsx_element = ctx.query();
        let element_name = jsx_element.name().ok()?;
        let element_name = element_name.name_value_token().ok()?;

        let src_attribute = jsx_element.attributes().find_by_name("src")?;
        let src_attribute_value = src_attribute.initializer()?.value().ok()?;
        let src_attribute_str = src_attribute_value
            .as_jsx_string()?
            .inner_string_text()
            .ok()?;

        let features = match element_name.text_trimmed() {
            "script" => check_unwanted_polyfill(&src_attribute_str, NEXT_POLYFILLED_FEATURES)?,
            _ => {
                let semantic_model = ctx.model();
                let reference = jsx_element.name().ok()?;
                let reference = reference.as_jsx_reference_identifier()?;
                let binding = semantic_model.binding(reference)?;
                if is_next_import(&binding, NextUtility::Script) {
                    check_unwanted_polyfill(&src_attribute_str, NEXT_POLYFILLED_FEATURES)?
                } else {
                    return None;
                }
            }
        };
        Some(RuleState {
            range: src_attribute_value.range(),
            unwanted_features_message: unwanted_features_message(&features),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! { "Prevent duplicate polyfills from Polyfill.io" },
            )
            .note(markup! { {state.unwanted_features_message} " already shipped with Next.js." })
            .note(markup! { "This unnecessarily increases page weight which can affect loading performance." }),
        )
    }
}

// Keep in sync with next.js polyfills file : https://github.com/vercel/next.js/blob/master/packages/next-polyfill-nomodule/src/index.js
const NEXT_POLYFILLED_FEATURES: &[&str] = &[
    "Array.from",
    "Array.of",
    "Array.prototype.@@iterator",
    "Array.prototype.at",
    "Array.prototype.copyWithin",
    "Array.prototype.fill",
    "Array.prototype.find",
    "Array.prototype.findIndex",
    "Array.prototype.flat",
    "Array.prototype.flatMap",
    "Array.prototype.includes",
    "Function.prototype.name",
    "Map",
    "Number.EPSILON",
    "Number.Epsilon",
    "Number.MAX_SAFE_INTEGER",
    "Number.MIN_SAFE_INTEGER",
    "Number.isFinite",
    "Number.isInteger",
    "Number.isNaN",
    "Number.isSafeInteger",
    "Number.parseFloat",
    "Number.parseInt",
    "Object.assign",
    "Object.entries",
    "Object.fromEntries",
    "Object.getOwnPropertyDescriptor",
    "Object.getOwnPropertyDescriptors",
    "Object.hasOwn",
    "Object.is",
    "Object.keys",
    "Object.values",
    "Promise",
    "Promise.prototype.finally",
    "Reflect",
    "Set",
    "String.fromCodePoint",
    "String.prototype.@@iterator",
    "String.prototype.codePointAt",
    "String.prototype.endsWith",
    "String.prototype.includes",
    "String.prototype.padEnd",
    "String.prototype.padStart",
    "String.prototype.repeat",
    "String.prototype.startsWith",
    "String.prototype.trimEnd",
    "String.prototype.trimStart",
    "String.raw",
    "Symbol",
    "Symbol.asyncIterator",
    "URL",
    "URL.prototype.toJSON",
    "URLSearchParams",
    "WeakMap",
    "WeakSet",
    "es2015", // Should be covered by babel-preset-env instead.
    "es2016", // contains polyfilled 'Array.prototype.includes', 'String.prototype.padEnd' and 'String.prototype.padStart'
    "es2017", // contains polyfilled 'Object.entries', 'Object.getOwnPropertyDescriptors', 'Object.values', 'String.prototype.padEnd' and 'String.prototype.padStart'
    "es2018", // contains polyfilled 'Promise.prototype.finally' and ''Symbol.asyncIterator'
    "es2019", // Contains polyfilled 'Object.fromEntries' and polyfilled 'Array.prototype.flat', 'Array.prototype.flatMap', 'String.prototype.trimEnd' and 'String.prototype.trimStart'
    "es5",    // Should be covered by babel-preset-env instead.
    "es6",    // Should be covered by babel-preset-env instead.
    "es7", // contains polyfilled 'Array.prototype.includes', 'String.prototype.padEnd' and 'String.prototype.padStart'
    "fetch",
];

fn unwanted_features_message(features: &[&str]) -> Box<str> {
    let joined_features = features.join(", ");
    let duplicate_polyfills = format!(
        "{} {}",
        joined_features,
        if features.len() > 1 { "are" } else { "is" }
    );
    duplicate_polyfills.into_boxed_str()
}

static URL_SPLIT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r",|%2C").unwrap());

/// Returns the unwanted features if the src is a polyfill.io URL and the features are already polyfilled by Next.js.
fn check_unwanted_polyfill<'a>(
    src: &'a str,
    next_polyfilled_features: &[&str],
) -> Option<Vec<&'a str>> {
    let unwanted_sources = [
        "https://cdn.polyfill.io/v2/",
        "https://polyfill.io/v3/",
        "https://polyfill-fastly.net/",
        "https://polyfill-fastly.io/",
        "https://cdnjs.cloudflare.com/polyfill/",
    ];

    if !unwanted_sources
        .iter()
        .any(|prefix| src.starts_with(prefix))
    {
        return None;
    }

    // Manually parse the "features" query parameter
    let query_start = src.find('?')?;
    let query_string = &src[query_start + 1..];
    let features_param = query_string.split('&').find_map(|param| {
        let (key, value) = param.split_once('=')?;
        if key == "features" {
            Some(value)
        } else {
            None
        }
    })?;

    // Use regex to split by ',' or '%2C'
    let unwanted_features = URL_SPLIT_REGEX
        .split(features_param)
        .filter(|feature| next_polyfilled_features.binary_search(feature).is_ok())
        .collect::<Vec<_>>();

    if unwanted_features.is_empty() {
        None
    } else {
        Some(unwanted_features)
    }
}

#[test]
fn test_check_unwanted_polyfill() {
    // Multiple features
    let src = "https://polyfill.io/v3/polyfill.min.js?features=Array.prototype.includes,Array.prototype.flat";
    let actual = check_unwanted_polyfill(src, NEXT_POLYFILLED_FEATURES);
    let expected = Some(vec!["Array.prototype.includes", "Array.prototype.flat"]);
    assert_eq!(actual, expected);

    // Multiple features with encoded comma (%2C)
    let src = "https://polyfill.io/v3/polyfill.min.js?features=Array.prototype.includes%2CArray.prototype.flat";
    let actual = check_unwanted_polyfill(src, NEXT_POLYFILLED_FEATURES);
    let expected = Some(vec!["Array.prototype.includes", "Array.prototype.flat"]);
    assert_eq!(actual, expected);

    // No query parameters
    let src = "https://polyfill.io/v3/polyfill.min.js";
    let actual = check_unwanted_polyfill(src, NEXT_POLYFILLED_FEATURES);
    assert_eq!(actual, None);

    // Differente URL
    let src = "https://example.com/polyfill.min.js?features=Array.prototype.includes";
    let actual = check_unwanted_polyfill(src, NEXT_POLYFILLED_FEATURES);
    assert_eq!(actual, None);

    // Wanted polyfill
    let src = "https://polyfill.io/v3/polyfill.min.js?features=AbortController";
    let actual = check_unwanted_polyfill(src, NEXT_POLYFILLED_FEATURES);
    assert_eq!(actual, None);
}

#[test]
fn test_order() {
    assert!(NEXT_POLYFILLED_FEATURES.is_sorted());
}
