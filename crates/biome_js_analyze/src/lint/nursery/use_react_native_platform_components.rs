use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsImportClause, AnyJsImportLike, AnyJsObjectBindingPatternMember,
    JsCallExpression, JsVariableDeclarator,
};
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::use_react_native_platform_components::UseReactNativePlatformComponentsOptions;

declare_lint_rule! {
    /// Ensure that platform-specific React Native components are only
    /// imported in files named for that platform.
    ///
    /// Some React Native components only work on one platform. For example,
    /// `ProgressBarAndroid` is Android-only and `ActivityIndicatorIOS` is
    /// iOS-only. These components should live in files with a matching
    /// platform suffix such as `.android.js` or `.ios.js`, so the React
    /// Native bundler can ship the right code to each platform.
    ///
    /// This rule reports an error when a platform-specific component is
    /// imported in a file that does not have the matching suffix, or when
    /// both Android and iOS components are imported in the same file.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// Importing an Android component in a non-Android file:
    ///
    /// ```js,expect_diagnostic
    /// import { ProgressBarAndroid } from "react-native";
    /// ```
    ///
    /// Importing an iOS component in a non-iOS file:
    ///
    /// ```js,expect_diagnostic
    /// import { ActivityIndicatorIOS } from "react-native";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { View } from "react-native";
    /// ```
    ///
    /// ## Options
    ///
    /// ### `androidPathPatterns`
    ///
    /// A list of glob patterns to identify Android-specific files.
    ///
    /// Default: `["**/*.android.{js,jsx,ts,tsx}"]`
    ///
    /// In the following example, Android files use `.droid.jsx` as their suffix instead of the default `.android.js`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "androidPathPatterns": ["**/*.droid.jsx"]
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options,file=Button.droid.jsx
    /// import { ProgressBarAndroid } from "react-native";
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options,file=Button.android.jsx
    /// import { ProgressBarAndroid } from "react-native";
    /// ```
    ///
    /// ### `iosPathPatterns`
    ///
    /// A list of glob patterns to identify iOS-specific files.
    ///
    /// Default: `["**/*.ios.{js,jsx,ts,tsx}"]`
    ///
    /// In the following example, iOS files use `.apple.jsx` as their suffix instead of the default `.ios.js`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "iosPathPatterns": ["**/*.apple.jsx"]
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options,file=Button.apple.jsx
    /// import { ActivityIndicatorIOS } from "react-native";
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options,file=Button.ios.jsx
    /// import { ActivityIndicatorIOS } from "react-native";
    /// ```
    ///
    pub UseReactNativePlatformComponents {
        version: "2.4.13",
        name: "useReactNativePlatformComponents",
        language: "js",
        sources: &[RuleSource::EslintReactNative("split-platform-components").inspired()],
        domains: &[RuleDomain::ReactNative],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseReactNativePlatformComponents {
    type Query = Ast<AnyJsImportLike>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = UseReactNativePlatformComponentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let module_name = node.inner_string_text();
        if module_name.as_ref().map(|t| t.text()) != Some("react-native") {
            return vec![];
        }

        let component_names = collect_imported_names(node);
        if component_names.is_empty() {
            return vec![];
        }

        let file_path = ctx.file_path().as_str();
        let options = ctx.options();

        let is_android_file = options
            .android_path_patterns()
            .iter()
            .any(|glob| glob.is_match(file_path));
        let is_ios_file = !is_android_file
            && options
                .ios_path_patterns()
                .iter()
                .any(|glob| glob.is_match(file_path));

        let mut has_android = false;
        let mut has_ios = false;
        for (_, name_text) in &component_names {
            let text = name_text.text();
            has_android |= text.contains("Android");
            has_ios |= text.contains("IOS");
        }
        let has_conflict = has_android && has_ios;

        let mut results: Vec<RuleState> = Vec::new();
        for (range, name_text) in component_names {
            let text = name_text.text();
            if text.contains("Android") && !is_android_file {
                results.push(RuleState {
                    kind: PlatformKind::Android,
                    range,
                    name: name_text,
                    has_conflict,
                });
            } else if text.contains("IOS") && !is_ios_file {
                results.push(RuleState {
                    kind: PlatformKind::Ios,
                    range,
                    name: name_text,
                    has_conflict,
                });
            }
        }

        results
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name.text();
        let diagnostic = if state.has_conflict {
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "iOS and Android components cannot be mixed in the same file."
                },
            )
            .note(markup! {
                <Emphasis>{name}</Emphasis>" is a platform-specific component."
            })
            .note(markup! {
                "Split iOS and Android components into separate platform-specific files."
            })
        } else {
            match state.kind {
                PlatformKind::Android => RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "Android component "<Emphasis>{name}</Emphasis>" is used outside of an Android-specific file."
                    },
                )
                .note(markup! {
                    "Platform-specific components produce incorrect bundles when imported in shared files."
                })
                .note(markup! {
                    "Move this import to a file with an Android-specific suffix (e.g. "<Emphasis>".android.js"</Emphasis>")."
                }),
                PlatformKind::Ios => RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "iOS component "<Emphasis>{name}</Emphasis>" is used outside of an iOS-specific file."
                    },
                )
                .note(markup! {
                    "Platform-specific components produce incorrect bundles when imported in shared files."
                })
                .note(markup! {
                    "Move this import to a file with an iOS-specific suffix (e.g. "<Emphasis>".ios.js"</Emphasis>")."
                }),
            }
        };

        Some(diagnostic)
    }
}

enum PlatformKind {
    Android,
    Ios,
}

pub struct RuleState {
    kind: PlatformKind,
    range: TextRange,
    name: TokenText,
    has_conflict: bool,
}

/// Collects the names of components imported from `react-native`.
///
/// Handles both ES module imports (`import { X } from 'react-native'`)
/// and CommonJS requires (`const { X } = require('react-native')`).
fn collect_imported_names(node: &AnyJsImportLike) -> Vec<(TextRange, TokenText)> {
    match node {
        AnyJsImportLike::JsModuleSource(source) => {
            let clause = source.parent::<AnyJsImportClause>();
            let Some(named_specifiers) = clause.and_then(|c| c.named_specifiers()) else {
                return Vec::new();
            };
            named_specifiers
                .specifiers()
                .into_iter()
                .flatten()
                .filter_map(|spec| {
                    let token = spec.imported_name()?;
                    Some((spec.range(), token.token_text_trimmed()))
                })
                .collect()
        }
        AnyJsImportLike::JsCallExpression(call) => collect_require_destructured_names(call),
        AnyJsImportLike::JsImportCallExpression(_) => Vec::new(),
    }
}

/// Extracts destructured property names from `const { X } = require('react-native')`.
fn collect_require_destructured_names(call: &JsCallExpression) -> Vec<(TextRange, TokenText)> {
    let Some(declarator) = call
        .syntax()
        .grand_parent()
        .and_then(JsVariableDeclarator::cast)
    else {
        return Vec::new();
    };

    let Ok(AnyJsBindingPattern::JsObjectBindingPattern(pattern)) = declarator.id() else {
        return Vec::new();
    };

    // This pattern doesn't handle nested bindings on purpose, as it's unlikely to happen.
    // However, if that happens, this code needs to be updated to handle it.
    pattern
        .properties()
        .into_iter()
        .flatten()
        .filter_map(|member| match &member {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(prop) => {
                let binding = prop.identifier().ok()?;
                let ident = binding.as_js_identifier_binding()?;
                let token = ident.name_token().ok()?;
                Some((prop.range(), token.token_text_trimmed()))
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(prop) => {
                let member = prop.member().ok()?;
                let ident = member.as_js_literal_member_name()?;
                let token = ident.value().ok()?;
                Some((prop.range(), token.token_text_trimmed()))
            }
            _ => None,
        })
        .collect()
}
