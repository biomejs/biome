use crate::{AnyCssExpression, AnyCssValue, CssFunction};
use biome_rowan::{AstNodeList, AstSeparatedList, TokenText};

impl CssFunction {
    /// Returns `true` for the Vue `v-bind()` function used inside a Vue
    /// component's `<style>` block.
    ///
    /// Example: `width: v-bind(size)`.
    pub fn is_vue_v_bind(&self) -> bool {
        self.name().is_ok_and(|name| {
            name.as_css_identifier()
                .and_then(|name| name.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == "v-bind")
        })
    }

    /// Returns the root identifier of a Vue `v-bind()` argument, if any.
    ///
    /// Vue evaluates the argument as a JavaScript expression, so the returned
    /// text is the script binding the expression refers to:
    ///
    /// - `v-bind(size)` returns `size`
    /// - `v-bind('theme.color')` returns `theme`, the root of the path
    ///
    /// Returns `None` when the argument cannot reference a binding, such as for
    /// `v-bind()`, `v-bind(1px)`, `v-bind('')`, and `v-bind('.a')`.
    ///
    /// The returned [`TokenText`] is a view into the argument token, so no
    /// string is allocated.
    pub fn vue_v_bind_binding(&self) -> Option<TokenText> {
        if !self.is_vue_v_bind() {
            return None;
        }

        match single_value_argument(self)? {
            // `v-bind(size)`
            AnyCssValue::CssIdentifier(identifier) => {
                Some(identifier.value_token().ok()?.token_text_trimmed())
            }
            // `v-bind('theme.color')`, where Vue allows any JavaScript
            // expression. Only a plain path can be resolved to a binding, so
            // the root segment is used and anything else is ignored.
            AnyCssValue::CssString(string) => {
                let path = string.inner_string_text().ok()?;
                let root = path.split('.').next()?;

                is_js_identifier(root.text()).then_some(root)
            }
            _ => None,
        }
    }
}

/// Returns the only argument of a function when it is a single value.
///
/// This returns `None` for an empty argument list, for more than one argument,
/// and for arguments widened by operators, such as `calc(1px + 2px)`.
fn single_value_argument(function: &CssFunction) -> Option<AnyCssValue> {
    let mut arguments = function.items().iter();
    let argument = arguments.next()?.ok()?;
    if arguments.next().is_some() {
        return None;
    }

    // A lone value is parsed as a list of component values holding one item.
    let AnyCssExpression::CssListOfComponentValuesExpression(values) = argument else {
        return None;
    };
    let mut values = values.css_component_value_list().iter();
    let value = values.next()?;

    values.next().is_none().then_some(value)
}

/// Returns `true` for a text that is a plain JavaScript identifier.
///
/// This is deliberately conservative: it only accepts the identifier shape that
/// can be matched against a script binding, so `v-bind('a-b')` and
/// `v-bind('0')` are rejected.
fn is_js_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|char| char.is_alphabetic() || matches!(char, '_' | '$'))
        && chars.all(|char| char.is_alphanumeric() || matches!(char, '_' | '$'))
}

#[cfg(test)]
mod tests {
    use biome_css_factory::syntax::CssFunction;
    use biome_css_parser::{CssModulesKind, CssParserOptions, parse_css};
    use biome_rowan::AstNode;

    /// Parses `source` as a Vue `<style>` block and returns the binding of
    /// every `v-bind()` function in it.
    fn vue_v_bind_bindings(source: &str) -> Vec<String> {
        let options = CssParserOptions {
            css_modules: CssModulesKind::Vue,
            ..CssParserOptions::default()
        };
        let parsed = parse_css(source, Default::default(), options);

        parsed
            .syntax()
            .descendants()
            .filter_map(CssFunction::cast)
            .filter(CssFunction::is_vue_v_bind)
            .filter_map(|function| {
                function
                    .vue_v_bind_binding()
                    .map(|binding| binding.text().to_string())
            })
            .collect()
    }

    #[test]
    fn identifier_argument_is_the_binding() {
        assert_eq!(vue_v_bind_bindings(".a { width: v-bind(size); }"), ["size"]);
    }

    #[test]
    fn quoted_argument_uses_the_root_of_the_path() {
        assert_eq!(
            vue_v_bind_bindings(r#".a { width: v-bind('size'); color: v-bind("theme.color"); }"#),
            ["size", "theme"]
        );
    }

    #[test]
    fn argument_that_cannot_reference_a_binding_has_none() {
        // Neither an absent nor a non-identifier argument refers to a binding.
        assert!(
            vue_v_bind_bindings(
                r#".a { width: v-bind(); height: v-bind(1px); margin: v-bind(''); padding: v-bind('.a'); top: v-bind('0'); }"#
            )
            .is_empty()
        );
    }

    #[test]
    fn other_functions_have_no_binding() {
        assert!(vue_v_bind_bindings(".a { width: calc(100% - 10px); }").is_empty());
    }

    #[test]
    fn unquoted_path_argument_has_no_binding() {
        // Vue requires a quoted expression for a path, so the unquoted form
        // must not resolve to a binding.
        assert!(vue_v_bind_bindings(".a { color: v-bind(theme.color); }").is_empty());
    }
}
