use crate::frameworks::{is_framework_lib_export, is_named_framework_lib_export};
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsImportLike, AnyJsObjectMember, JsCallExpression,
    JsSyntaxKind, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_react_native_valid_platform_colors::UseReactNativeValidPlatformColorsOptions;

declare_lint_rule! {
    /// Require `PlatformColor()` and `DynamicColorIOS()` calls to use values written directly in the call.
    ///
    /// React Native can optimize `PlatformColor()` and `DynamicColorIOS()` calls,
    /// but only when it can read their values without running the code.
    /// Values that come from variables or other expressions are only known when the app runs.
    ///
    /// This rule checks that:
    /// - `PlatformColor()` receives at least one argument, and every argument is a literal
    ///   such as a string. The last argument can also be an object with a single
    ///   `fallback` property whose value is a literal.
    /// - `DynamicColorIOS()` receives exactly one object, and every value in that object is
    ///   either a literal or a `PlatformColor()` call.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { PlatformColor } from "react-native";
    ///
    /// const color = PlatformColor();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { PlatformColor } from "react-native";
    ///
    /// const labelColor = "labelColor";
    /// const color = PlatformColor(labelColor);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { DynamicColorIOS } from "react-native";
    ///
    /// const colors = { light: "black", dark: "white" };
    /// const color = DynamicColorIOS(colors);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { DynamicColorIOS } from "react-native";
    ///
    /// const black = "black";
    /// const color = DynamicColorIOS({ light: black, dark: "white" });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { DynamicColorIOS, PlatformColor } from "react-native";
    ///
    /// const label = PlatformColor("labelColor");
    /// const accent = PlatformColor("controlAccentColor", "controlColor", { fallback: "red" });
    /// const text = DynamicColorIOS({ light: "black", dark: PlatformColor("labelColor") });
    /// ```
    ///
    pub UseReactNativeValidPlatformColors {
        version: "next",
        name: "useReactNativeValidPlatformColors",
        language: "js",
        sources: &[RuleSource::EslintReactNative("platform-colors").same()],
        domains: &[RuleDomain::ReactNative],
        recommended: true,
        severity: Severity::Error,
    }
}

pub enum RuleState {
    /// `PlatformColor()` was called without arguments.
    MissingPlatformColorArgument,
    /// The first argument of `PlatformColor()` that is not a literal, nor a trailing
    /// `{ fallback: <literal> }`.
    InvalidPlatformColorArgument(AnyJsCallArgument),
    /// `DynamicColorIOS()` was not called with exactly one object literal.
    InvalidDynamicColorArgument,
    /// The first member of the `DynamicColorIOS()` object that is not a literal or a
    /// `PlatformColor()` call.
    InvalidDynamicColorValue(AnyJsObjectMember),
}

impl Rule for UseReactNativeValidPlatformColors {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseReactNativeValidPlatformColorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let arguments = call.arguments().ok()?.args();

        match color_function(&call.callee().ok()?, model)? {
            ColorFunction::PlatformColor => {
                if arguments.is_empty() {
                    return Some(RuleState::MissingPlatformColorArgument);
                }
                let last_index = arguments.len() - 1;
                arguments.iter().enumerate().find_map(|(index, argument)| {
                    let argument = argument.ok()?;
                    let is_valid = argument.as_any_js_expression().is_some_and(|expression| {
                        let expression = expression.clone().omit_parentheses();
                        is_literal(&expression)
                            || (index == last_index && is_fallback_object(&expression))
                    });
                    (!is_valid).then_some(RuleState::InvalidPlatformColorArgument(argument))
                })
            }
            ColorFunction::DynamicColorIOS => {
                let mut arguments = arguments.iter();
                let object = match (arguments.next(), arguments.next()) {
                    (Some(Ok(AnyJsCallArgument::AnyJsExpression(expression))), None) => {
                        match expression.omit_parentheses() {
                            AnyJsExpression::JsObjectExpression(object) => object,
                            _ => return Some(RuleState::InvalidDynamicColorArgument),
                        }
                    }
                    _ => return Some(RuleState::InvalidDynamicColorArgument),
                };
                object
                    .members()
                    .iter()
                    .flatten()
                    .find(|member| !is_valid_dynamic_color_member(member, model))
                    .map(RuleState::InvalidDynamicColorValue)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();
        let diagnostic = match state {
            RuleState::MissingPlatformColorArgument => RuleDiagnostic::new(
                rule_category!(),
                call.range(),
                markup! {
                    <Emphasis>"PlatformColor()"</Emphasis>" is called without a color name."
                },
            )
            .note(markup! {
                "A platform color needs at least one color name to look up."
            })
            .note(markup! {
                "Pass at least one color name as a string, for example: "<Emphasis>"PlatformColor(\"labelColor\")"</Emphasis>"."
            }),
            RuleState::InvalidPlatformColorArgument(argument) => RuleDiagnostic::new(
                rule_category!(),
                argument.range(),
                markup! {
                    "This argument of "<Emphasis>"PlatformColor()"</Emphasis>" is not a literal value."
                },
            )
            .note(markup! {
                "React Native can only optimize platform colors whose values are written directly in the call."
            })
            .note(markup! {
                "Write the color name as a string. The last argument can also be an object like "<Emphasis>"{ fallback: \"red\" }"</Emphasis>", with a literal value."
            }),
            RuleState::InvalidDynamicColorArgument => RuleDiagnostic::new(
                rule_category!(),
                call.arguments().ok()?.range(),
                markup! {
                    <Emphasis>"DynamicColorIOS()"</Emphasis>" must be called with a single object written directly in the call."
                },
            )
            .note(markup! {
                "React Native can only optimize dynamic colors whose values are written directly in the call."
            })
            .note(markup! {
                "Pass an object like "<Emphasis>"{ light: \"black\", dark: \"white\" }"</Emphasis>"."
            }),
            RuleState::InvalidDynamicColorValue(member) => RuleDiagnostic::new(
                rule_category!(),
                member.range(),
                markup! {
                    "This value of "<Emphasis>"DynamicColorIOS()"</Emphasis>" is not a literal or a "<Emphasis>"PlatformColor()"</Emphasis>" call."
                },
            )
            .note(markup! {
                "React Native can only optimize dynamic colors whose values are written directly in the call."
            })
            .note(markup! {
                "Write the color as a string, or use a "<Emphasis>"PlatformColor()"</Emphasis>" call."
            }),
        };
        Some(diagnostic)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ColorFunction {
    PlatformColor,
    DynamicColorIOS,
}

impl ColorFunction {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "PlatformColor" => Some(Self::PlatformColor),
            "DynamicColorIOS" => Some(Self::DynamicColorIOS),
            _ => None,
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::PlatformColor => "PlatformColor",
            Self::DynamicColorIOS => "DynamicColorIOS",
        }
    }
}

/// Returns the color function that `callee` refers to.
///
/// A plain identifier matches when it is not declared in the file, or when it is imported
/// from `react-native` with `import` or a destructured `require()`. A member expression
/// such as `ReactNative.PlatformColor` matches when the object is a namespace or default
/// import of `react-native`, or the result of `require("react-native")`.
fn color_function(callee: &AnyJsExpression, model: &SemanticModel) -> Option<ColorFunction> {
    match callee.clone().omit_parentheses() {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let reference = identifier.name().ok()?;
            let function = ColorFunction::from_name(reference.value_token().ok()?.text_trimmed())?;
            let Some(binding) = model.binding(&reference) else {
                return Some(function);
            };
            let is_react_native = is_named_framework_lib_export(
                &binding,
                function.name(),
                REACT_NATIVE_PACKAGE_NAMES,
            )
            .unwrap_or(false)
                || is_react_native_require(&binding);
            is_react_native.then_some(function)
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let function = ColorFunction::from_name(
                member
                    .member()
                    .ok()?
                    .as_js_name()?
                    .value_token()
                    .ok()?
                    .text_trimmed(),
            )?;
            let object = member.object().ok()?.omit_parentheses();
            let reference = object.as_js_reference_identifier()?;
            let binding = model.binding(&reference)?;
            let is_react_native = is_framework_lib_export(&binding, REACT_NATIVE_PACKAGE_NAMES)
                || is_react_native_require(&binding);
            is_react_native.then_some(function)
        }
        _ => None,
    }
}

/// Returns `true` if `binding` is declared by `require("react-native")`, either directly
/// (`const ReactNative = require("react-native")`) or through destructuring
/// (`const { PlatformColor } = require("react-native")`).
fn is_react_native_require(binding: &Binding) -> bool {
    let Some(declaration) = binding.tree().declaration() else {
        return false;
    };
    let declaration = declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration);
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return false;
    };
    let Some(AnyJsExpression::JsCallExpression(call)) = declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
        .map(AnyJsExpression::omit_parentheses)
    else {
        return false;
    };
    AnyJsImportLike::JsCallExpression(call)
        .inner_string_text()
        .is_some_and(|source| REACT_NATIVE_PACKAGE_NAMES.contains(&source.text()))
}

const REACT_NATIVE_PACKAGE_NAMES: &[&str] = &["react-native"];

fn is_literal(expression: &AnyJsExpression) -> bool {
    matches!(expression, AnyJsExpression::AnyJsLiteralExpression(_))
}

/// Returns `true` for an object with exactly one member, `fallback: <literal>`.
fn is_fallback_object(expression: &AnyJsExpression) -> bool {
    let AnyJsExpression::JsObjectExpression(object) = expression else {
        return false;
    };
    let mut members = object.members().iter();
    let (Some(Ok(AnyJsObjectMember::JsPropertyObjectMember(member))), None) =
        (members.next(), members.next())
    else {
        return false;
    };
    let has_fallback_name = member
        .name()
        .ok()
        .and_then(|name| name.as_js_literal_member_name()?.value().ok())
        .is_some_and(|token| {
            token.kind() == JsSyntaxKind::IDENT && token.text_trimmed() == "fallback"
        });
    has_fallback_name
        && member
            .value()
            .is_ok_and(|value| is_literal(&value.omit_parentheses()))
}

fn is_valid_dynamic_color_member(member: &AnyJsObjectMember, model: &SemanticModel) -> bool {
    let AnyJsObjectMember::JsPropertyObjectMember(member) = member else {
        return false;
    };
    let Ok(value) = member.value() else {
        return false;
    };
    match value.omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(_) => true,
        AnyJsExpression::JsCallExpression(call) => call.callee().is_ok_and(|callee| {
            color_function(&callee, model) == Some(ColorFunction::PlatformColor)
        }),
        _ => false,
    }
}
