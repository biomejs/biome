use crate::frameworks::vue::vue_component::{VueComponent, VueComponentName, VueComponentQuery};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::use_vue_multi_word_component_names::UseVueMultiWordComponentNamesOptions;

declare_lint_rule! {
    /// Enforce multi-word component names in Vue components.
    ///
    /// Using a single-word component name (e.g. `App`, `Header`) can:
    /// - Conflict with native/custom HTML elements (present or future)
    /// - Reduce clarity/expressiveness
    ///
    /// This rule requires component names to be "multi-word".
    ///
    /// A name is considered multi-word when:
    /// - Kebab-case: contains at least one hyphen (`my-component`)
    /// - PascalCase / CamelCase: contains at least two capital letters (`MyComponent`); single-cap names like `App` or `Foo` are rejected
    ///
    /// Component names are extracted from the `name` property in Options API components, or inferred from the file name if not explicitly set.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   name: "Foo"
    /// };
    /// </script>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from "vue";
    /// export default defineComponent({
    ///   name: "Header"
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { createApp } from "vue";
    /// createApp({
    ///   name: "Widget"
    /// }).mount("#app");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   name: "MyComponent"
    /// };
    /// </script>
    /// ```
    ///
    /// ```js
    /// export default {
    ///   name: "my-component"
    /// };
    /// ```
    ///
    /// ```js
    /// defineComponent({
    ///   name: "MyComponent"
    /// });
    /// ```
    ///
    /// ```js
    /// createApp({ name: "MyApp" }).mount("#app");
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignores`
    ///
    /// Additional single-word component names to ignore (case-insensitive). The rule already ignores Vue built-in components and `App` by default.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignores": [
    ///       "Foo"
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <script>
    /// export default {
    ///   name: "Foo"
    /// };
    /// </script>
    /// ```
    ///
    pub UseVueMultiWordComponentNames {
        version: "2.2.3",
        name: "useVueMultiWordComponentNames",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("multi-word-component-names").inspired()],
    }
}

impl Rule for UseVueMultiWordComponentNames {
    type Query = VueComponentQuery;
    type State = Option<(TokenText, TextRange)>;
    type Signals = Option<Self::State>;
    type Options = UseVueMultiWordComponentNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // Build potential Vue component; bail if not a component or not in a Vue embedding
        let component = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type(),
            ctx.file_path(),
        )?;
        let component_name = component.name()?;
        if should_report(component_name.as_ref(), ctx.options()) {
            if let VueComponentName::FromComponent(name_token) = component_name {
                Some(Some(name_token))
            } else {
                // Name inferred from path; can't point to precise range, so flag whole component
                Some(None)
            }
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        // If the state is Some, we can point to the precise range of the name token. Otherwise, we have to flag the whole component.

        let range = state
            .as_ref()
            .map_or_else(|| ctx.query().range(), |token_text| token_text.1);
        let Some(component_name) = state
            .as_ref()
            .map(|token_text| token_text.0.text())
            .or_else(|| ctx.file_path().file_stem())
        else {
            // Can't determine component name; shouldn't happen since we had a name from the component before
            // but just in case, avoid crashing and don't report
            debug_assert!(false, "should never get here");
            return None;
        };
        let got_name_from_file_name = state.is_none();

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "This Component's name "<Emphasis>"\""{component_name}"\""</Emphasis>" only contains one word."
            },
        ).note(markup! {
            "Single-word component names can collide with HTML elements and are less descriptive."
        })
        .note(markup! {
            "Rename the component to have 2 or more words (e.g. \"FooItem\", or \"BarView\")."
        });

        if got_name_from_file_name {
            diagnostic = diagnostic.note(markup! {
                "The component name was inferred from the file name."
            });
        }
        Some(diagnostic)
    }
}

/// Built-in single-word names to ignore.
///
/// "app" is commonly used as the root component name, so we ignore it by default.
/// The others are actual Vue built-in components that are single-word.
const BUILTIN_IGNORES: &[&str] = &[
    "app",
    "component",
    "slot",
    "suspense",
    "teleport",
    "template",
    "transition",
];

/// Determines if the given component name should be reported (i.e. is invalid single-word).
fn should_report(name: &str, options: &UseVueMultiWordComponentNamesOptions) -> bool {
    if name.is_empty() {
        return true; // invalid, not covered by ignores
    }

    // We could binary search, but the list is so short that linear scan is probably faster
    if BUILTIN_IGNORES.iter().any(|s| s.eq_ignore_ascii_case(name)) {
        return false;
    }

    for user in &options.ignores {
        if name.eq_ignore_ascii_case(user) {
            return false;
        }
    }

    // Report if NOT multi-word
    !is_multi_word(name)
}

/// Multi-word detection without allocating an intermediate string:
/// - Hyphens (`-`) and underscores (`_`) act as explicit separators
/// - Transition from lowercase/digit to uppercase starts a new segment (inserts an implicit hyphen)
/// - Uppercase letter followed by lowercase also starts a new segment (handles cases like "UIButton")
fn is_multi_word(name: &str) -> bool {
    let mut segments = 0u8;
    let mut chars = name.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '-' | '_' => {
                // Explicit separators don't count as segments themselves

                // because there is a separator, we know there is a segment after this
                // no need to keep going
                return true;
            }
            _ => {
                // Start a new segment
                segments += 1;
                if segments > 1 {
                    return true;
                }

                // Skip to the end of this segment
                let mut prev_was_upper = ch.is_ascii_uppercase();
                while let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '-' | '_' => {
                            // End of segment due to separator
                            break;
                        }
                        c if c.is_ascii_uppercase() => {
                            if !prev_was_upper {
                                // lowercase/digit -> uppercase: start new segment
                                break;
                            }
                            // Check if this uppercase is followed by lowercase (like 'B' in "UIButton")
                            chars.next(); // consume this uppercase char
                            if let Some(&after_upper) = chars.peek()
                                && after_upper.is_ascii_lowercase()
                                && prev_was_upper
                            {
                                // This uppercase starts a new word (UI|Button pattern)
                                segments += 1;
                                if segments > 1 {
                                    return true;
                                }
                                prev_was_upper = true;
                                continue;
                            }
                            prev_was_upper = true;
                        }
                        _ => {
                            // lowercase or digit - continue in same segment
                            chars.next();
                            prev_was_upper = false;
                        }
                    }
                }
            }
        }
        if segments > 1 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_multi_word() {
        assert!(is_multi_word("MyComponent"));
        assert!(is_multi_word("my-component"));
        assert!(is_multi_word("myComponent"));
        assert!(is_multi_word("MyAppRoot"));
        assert!(is_multi_word("MYComponent"));
        assert!(is_multi_word("foo_bar")); // underscore counts as separator
        assert!(is_multi_word("Foo_BarBaz"));
        assert!(is_multi_word("UIButton"));
        assert!(is_multi_word("Foo_"));
        assert!(is_multi_word("_Foo"));
        assert!(is_multi_word("Foo-"));
        assert!(is_multi_word("-Foo"));

        assert!(!is_multi_word("App"));
        assert!(!is_multi_word("Foo"));
        assert!(!is_multi_word("BUTTON"));
    }

    #[test]
    fn test_should_report_builtin_and_defaults() {
        let options = UseVueMultiWordComponentNamesOptions::default();
        // Built-ins / defaults ignored
        assert!(!should_report("App", &options));
        assert!(!should_report("app", &options));
        assert!(!should_report("Component", &options));
        assert!(!should_report("component", &options));
        assert!(!should_report("Transition", &options));
        assert!(!should_report("transition-group", &options));
    }

    #[test]
    fn test_should_report_user_ignores() {
        let mut options = UseVueMultiWordComponentNamesOptions::default();
        options.ignores.push("FooBar".to_string()); // PascalCase
        options.ignores.push("widget".to_string()); // lowercase

        // PascalCase ignore and its kebab-case variant
        assert!(!should_report("FooBar", &options));
        assert!(!should_report("foo-bar", &options));

        // Lowercase ignore
        assert!(!should_report("widget", &options));

        // Non-ignored single-word should report
        assert!(should_report("Header", &options));

        // Multi-word never reports
        assert!(!should_report("MyWidget", &options));
    }

    #[test]
    fn test_should_report_edge_cases() {
        let options = UseVueMultiWordComponentNamesOptions::default();
        assert!(should_report("", &options)); // empty invalid
        assert!(should_report("X", &options)); // single-letter
    }
}
