use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssFunctionName, AnyCssProperty, AnyScssModuleMember,
    CssAttrFunction, CssCustomIdentifier, CssCustomPropertyDelimiter, CssCustomPropertyFunction,
    CssCustomPropertyParenthesizedBlock, CssDeclarationWithSemicolon, CssFunction, CssIfFunction,
    CssLanguage, CssParenthesizedExpression, CssSyntaxKind, CssSyntaxToken,
    ScssInterpolatedString, ScssLegacyIfFunction, ScssMapExpression, ScssNestingDeclaration,
    ScssParenthesizedExpression, ScssVariableDeclaration, T,
};
use biome_languages::CssFileSource;
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, SyntaxKindSet, TextRange, WalkEvent, declare_node_union};
use biome_rule_options::no_scss_deprecated_color_functions::NoScssDeprecatedColorFunctionsOptions;

declare_lint_rule! {
    /// Disallow the deprecated Sass functions that read one channel of a color.
    ///
    /// Sass has deprecated functions such as `red()`, `color.hue()` and `opacity()`, and plans to
    /// remove them. `color.channel()` replaces all of them. The rule also reports `color.alpha()`,
    /// which isn't deprecated but fails for colors outside the `rgb`, `hsl` and `hwb` color
    /// spaces, and `blackness()`, which isn't a Sass function.
    ///
    /// The rule checks the values of properties and Sass variables. It doesn't check the `filter`
    /// property, where `opacity()` is the CSS filter function.
    ///
    /// :::note
    /// Module functions are only recognized through the `color` namespace, as in `color.hue()`.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```scss,expect_diagnostic
    /// @use "sass:color";
    ///
    /// a {
    ///   opacity: color.alpha($brand);
    /// }
    /// ```
    ///
    /// ```scss,expect_diagnostic
    /// a {
    ///   opacity: opacity($brand);
    /// }
    /// ```
    ///
    /// ```scss,expect_diagnostic
    /// $hue: hue($brand);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```scss
    /// @use "sass:color";
    ///
    /// $hue: color.channel($brand, "hue", $space: hsl);
    ///
    /// a {
    ///   opacity: color.channel($brand, "alpha");
    ///   filter: opacity(50%);
    /// }
    /// ```
    ///
    pub NoScssDeprecatedColorFunctions {
        version: "2.6.10",
        name: "noScssDeprecatedColorFunctions",
        language: "css",
        recommended: false,
        sources: &[RuleSource::StylelintScss("function-color-channel").same()],
    }
}

declare_node_union! {
    pub AnyScssValueDeclaration =
        CssDeclarationWithSemicolon | ScssVariableDeclaration | ScssNestingDeclaration
}

impl Rule for NoScssDeprecatedColorFunctions {
    type Query = Ast<AnyScssValueDeclaration>;
    type State = ColorFunctionCall;
    type Signals = Box<[Self::State]>;
    type Options = NoScssDeprecatedColorFunctionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<CssFileSource>().is_scss() {
            return Box::default();
        }

        let (name, value) = match ctx.query() {
            AnyScssValueDeclaration::CssDeclarationWithSemicolon(declaration) => {
                let Some(AnyCssProperty::CssGenericProperty(property)) = declaration
                    .declaration()
                    .ok()
                    .and_then(|declaration| declaration.property().ok())
                else {
                    return Box::default();
                };
                let (Ok(name), Ok(value)) = (property.name(), property.value()) else {
                    return Box::default();
                };
                (Some(name), value.into_syntax())
            }
            AnyScssValueDeclaration::ScssVariableDeclaration(declaration) => {
                let Ok(value) = declaration.value() else {
                    return Box::default();
                };
                (None, value.into_syntax())
            }
            AnyScssValueDeclaration::ScssNestingDeclaration(declaration) => {
                let Ok(value) = declaration.value() else {
                    return Box::default();
                };
                (declaration.name().ok(), value.into_syntax())
            }
        };
        if name.is_some_and(|name| is_filter_property(&name)) {
            return Box::default();
        }

        let mut calls = Vec::new();
        let mut preorder = value.preorder_of_kinds(VALUE_SEARCH_KINDS);
        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if let Some(function) = CssFunction::cast_ref(&node) {
                calls.extend(find_function_call(&function));
            } else if let Some(function) = CssCustomPropertyFunction::cast_ref(&node) {
                calls.extend(find_custom_property_function_call(&function));
            } else {
                // Quoted strings are text, even when they contain `#{}`.
                preorder.skip_subtree();
            }
        }
        calls.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, call: &Self::State) -> Option<RuleDiagnostic> {
        let function = call.function;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                call.range,
                markup! {
                    "Use "<Emphasis>"color.channel()"</Emphasis>" instead of "<Emphasis>{function}"()"</Emphasis>"."
                },
            )
            .note(if function.is_module && function.channel == ColorChannel::Alpha {
                markup! {
                    "This function fails for colors outside the "<Emphasis>"rgb"</Emphasis>", "<Emphasis>"hsl"</Emphasis>" and "<Emphasis>"hwb"</Emphasis>" color spaces, such as colors made with "<Emphasis>"oklch()"</Emphasis>"."
                }
            } else if !function.is_module && function.channel == ColorChannel::Blackness {
                markup! {
                    "Sass has no global "<Emphasis>"blackness()"</Emphasis>" function, so it outputs this call unchanged instead of reading the blackness of the color."
                }
            } else {
                markup! {
                    "Sass has deprecated this function and plans to remove it in a future version."
                }
            })
            .note(markup! {
                "Replace it with "<Emphasis>{function.channel}</Emphasis>"."
            }),
        )
    }
}

/// A call to one of the reported functions.
pub struct ColorFunctionCall {
    /// The range of the function name, including the `color.` namespace of a module function.
    range: TextRange,
    function: ColorFunction,
}

/// One of the reported functions.
#[derive(Clone, Copy)]
struct ColorFunction {
    name: &'static str,
    /// Whether the function belongs to the `sass:color` module.
    is_module: bool,
    channel: ColorChannel,
}

impl ColorFunction {
    fn new(is_module: bool, name: &str) -> Option<Self> {
        let functions = if is_module {
            MODULE_FUNCTIONS
        } else {
            GLOBAL_FUNCTIONS
        };
        let index = functions
            .binary_search_by(|(function, _)| function.cmp(&name))
            .ok()?;
        let (name, channel) = functions[index];
        Some(Self {
            name,
            is_module,
            channel,
        })
    }
}

/// Writes the name of the function, including the `color.` namespace of a module function.
impl biome_console::fmt::Display for ColorFunction {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        if self.is_module {
            fmt.write_str("color.")?;
        }
        fmt.write_str(self.name)
    }
}

/// A color channel that one of the reported functions reads. The `Display` implementation writes
/// the matching `color.channel()` call, including the color space the function reads it in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ColorChannel {
    Alpha,
    Red,
    Green,
    Blue,
    Hue,
    Saturation,
    Lightness,
    Whiteness,
    Blackness,
}

/// Writes the `color.channel()` call that replaces the reported function.
impl biome_console::fmt::Display for ColorChannel {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Alpha => "color.channel($color, \"alpha\")",
            Self::Red => "color.channel($color, \"red\", $space: rgb)",
            Self::Green => "color.channel($color, \"green\", $space: rgb)",
            Self::Blue => "color.channel($color, \"blue\", $space: rgb)",
            Self::Hue => "color.channel($color, \"hue\", $space: hsl)",
            Self::Saturation => "color.channel($color, \"saturation\", $space: hsl)",
            Self::Lightness => "color.channel($color, \"lightness\", $space: hsl)",
            Self::Whiteness => "color.channel($color, \"whiteness\", $space: hwb)",
            Self::Blackness => "color.channel($color, \"blackness\", $space: hwb)",
        })
    }
}

/// Returns whether `name` is `filter`, including the `_filter` and `*filter` hacks for old
/// versions of Internet Explorer.
fn is_filter_property(name: &AnyCssDeclarationName) -> bool {
    let AnyCssDeclarationName::CssIdentifier(name) = name else {
        return false;
    };
    name.value_token().is_ok_and(|name| {
        let name = name.text_trimmed();
        let name = name
            .strip_prefix(['_', '*'])
            .unwrap_or(name);
        name == "filter"
    })
}

declare_node_union! {
    pub AnyCssColorFunction = CssFunction | CssCustomPropertyFunction
}

/// Returns whether `first_token`, the first token of the name of `function`, starts a new word
/// in the value: it follows a space, a `,`, a `(` or a similar separator. A name written right
/// after another character becomes part of a longer word, as in `#{alpha($c)}` or `1+alpha($c)`,
/// and isn't reported.
fn starts_word(first_token: &CssSyntaxToken, function: &AnyCssColorFunction) -> bool {
    if !first_token.leading_trivia().is_empty() {
        return true;
    }
    let Some(previous) = first_token.prev_token() else {
        return true;
    };
    if !previous.trailing_trivia().is_empty() {
        return true;
    }
    let kind = previous.kind();
    WORD_SEPARATOR_KINDS.contains(kind)
        || (CALC_OPERATOR_KINDS.contains(kind) && is_directly_in_calc(function))
}

/// Returns whether the innermost parentheses around `function` belong to a `calc()` call,
/// where `*` and `/` separate words.
fn is_directly_in_calc(function: &AnyCssColorFunction) -> bool {
    let Some(parent) = function
        .syntax()
        .ancestors()
        .skip(1)
        .find(|node| PARENTHESIZED_KINDS.matches(node.kind()))
    else {
        return false;
    };
    let name = match AnyCssColorFunction::cast(parent) {
        Some(AnyCssColorFunction::CssFunction(function)) => match function.name() {
            Ok(AnyCssFunctionName::CssIdentifier(name)) => name.value_token(),
            _ => return false,
        },
        Some(AnyCssColorFunction::CssCustomPropertyFunction(function)) => {
            match function.name() {
                Ok(name) => name.value_token(),
                Err(_) => return false,
            }
        }
        None => return false,
    };
    name.is_ok_and(|name| name.text_trimmed() == "calc")
}

fn find_function_call(function: &CssFunction) -> Option<ColorFunctionCall> {
    let name = function.name().ok()?;
    let (first_token, color_function) = match &name {
        AnyCssFunctionName::CssIdentifier(identifier) => {
            let token = identifier.value_token().ok()?;
            let color_function = ColorFunction::new(false, token.text_trimmed())?;
            (token, color_function)
        }
        AnyCssFunctionName::ScssModuleMemberAccess(access) => {
            let module = access.module().ok()?.value_token().ok()?;
            if module.text_trimmed() != "color" {
                return None;
            }
            let AnyScssModuleMember::CssIdentifier(member) = access.member().ok()? else {
                return None;
            };
            let color_function =
                ColorFunction::new(true, member.value_token().ok()?.text_trimmed())?;
            (module, color_function)
        }
        AnyCssFunctionName::ScssInterpolatedIdentifier(_) => return None,
    };
    if !starts_word(
        &first_token,
        &AnyCssColorFunction::CssFunction(function.clone()),
    ) {
        return None;
    }
    Some(ColorFunctionCall {
        range: name.range(),
        function: color_function,
    })
}

/// Finds a reported function in a custom property value. A custom property value isn't parsed
/// as a Sass expression, so `color.alpha($c)` there is the identifier `color`, a `.` delimiter
/// and the function `alpha($c)`.
fn find_custom_property_function_call(
    function: &CssCustomPropertyFunction,
) -> Option<ColorFunctionCall> {
    let name = function.name().ok()?;
    let name_token = name.value_token().ok()?;
    let delimiter = function
        .syntax()
        .prev_sibling()
        .and_then(CssCustomPropertyDelimiter::cast);
    let dot = delimiter
        .as_ref()
        .and_then(|delimiter| delimiter.value().ok())
        .filter(|dot| {
            dot.kind() == T![.]
                && dot.trailing_trivia().is_empty()
                && name_token.leading_trivia().is_empty()
        });
    let any_function = AnyCssColorFunction::CssCustomPropertyFunction(function.clone());
    let Some(dot) = dot else {
        let color_function = ColorFunction::new(false, name_token.text_trimmed())?;
        return starts_word(&name_token, &any_function).then(|| ColorFunctionCall {
            range: name.range(),
            function: color_function,
        });
    };

    // A name written right after a `.` is only a module function after `color`, as in
    // `color.alpha()`.
    let module = delimiter?
        .syntax()
        .prev_sibling()
        .and_then(CssCustomIdentifier::cast)?;
    let module_token = module.value_token().ok()?;
    if module_token.text_trimmed() != "color"
        || !module_token.trailing_trivia().is_empty()
        || !dot.leading_trivia().is_empty()
        || !starts_word(&module_token, &any_function)
    {
        return None;
    }
    Some(ColorFunctionCall {
        range: TextRange::new(module.range().start(), name.range().end()),
        function: ColorFunction::new(true, name_token.text_trimmed())?,
    })
}

/// The nodes the search of a value stops at: function calls, and quoted strings, whose `#{}`
/// contents aren't checked.
const VALUE_SEARCH_KINDS: SyntaxKindSet<CssLanguage> = CssFunction::KIND_SET
    .union(CssCustomPropertyFunction::KIND_SET)
    .union(ScssInterpolatedString::KIND_SET);

/// The tokens that end a word in a value, so a function name right after them is reported.
const WORD_SEPARATOR_KINDS: TokenSet<CssSyntaxKind> = token_set![
    T![,],
    T![:],
    T![::],
    T![/],
    T!['('],
    T![')'],
    CssSyntaxKind::CSS_STRING_LITERAL
];

/// The operators that also end a word directly inside `calc()`.
const CALC_OPERATOR_KINDS: TokenSet<CssSyntaxKind> = token_set![T![*], T![/]];

/// The nodes that wrap their contents in parentheses.
const PARENTHESIZED_KINDS: SyntaxKindSet<CssLanguage> = CssFunction::KIND_SET
    .union(CssCustomPropertyFunction::KIND_SET)
    .union(CssCustomPropertyParenthesizedBlock::KIND_SET)
    .union(CssParenthesizedExpression::KIND_SET)
    .union(ScssParenthesizedExpression::KIND_SET)
    .union(ScssMapExpression::KIND_SET)
    .union(CssIfFunction::KIND_SET)
    .union(ScssLegacyIfFunction::KIND_SET)
    .union(CssAttrFunction::KIND_SET);

/// The reported global functions, sorted by name.
const GLOBAL_FUNCTIONS: &[(&str, ColorChannel)] = &[
    ("alpha", ColorChannel::Alpha),
    ("blackness", ColorChannel::Blackness),
    ("blue", ColorChannel::Blue),
    ("green", ColorChannel::Green),
    ("hue", ColorChannel::Hue),
    ("lightness", ColorChannel::Lightness),
    ("opacity", ColorChannel::Alpha),
    ("red", ColorChannel::Red),
    ("saturation", ColorChannel::Saturation),
];

/// The reported functions of the `sass:color` module, sorted by name.
const MODULE_FUNCTIONS: &[(&str, ColorChannel)] = &[
    ("alpha", ColorChannel::Alpha),
    ("blackness", ColorChannel::Blackness),
    ("blue", ColorChannel::Blue),
    ("green", ColorChannel::Green),
    ("hue", ColorChannel::Hue),
    ("lightness", ColorChannel::Lightness),
    ("red", ColorChannel::Red),
    ("saturation", ColorChannel::Saturation),
    ("whiteness", ColorChannel::Whiteness),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functions_are_sorted() {
        for functions in [GLOBAL_FUNCTIONS, MODULE_FUNCTIONS] {
            for pair in functions.windows(2) {
                assert!(
                    pair[0].0 < pair[1].0,
                    "{} should sort before {}",
                    pair[0].0,
                    pair[1].0
                );
            }
        }
    }
}
