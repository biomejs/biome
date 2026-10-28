use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize::json::unescape_json_string;
use biome_diagnostics::Severity;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonRoot};
use biome_rowan::AstNode;
use biome_rule_options::use_package_json_valid_engines::UsePackageJsonValidEnginesOptions;

use crate::utils::is_package_json;

declare_lint_rule! {
    /// Enforce that the `engines` field in `package.json` is valid.
    ///
    /// The `engines` field lists the programs that a package works with, such as Node.js or npm,
    /// and the versions of each program that the package supports. Package managers read this
    /// field to warn people who install the package with an unsupported version.
    ///
    /// The field must be an object. Each property name is the name of a program, and each property
    /// value is a version range: a string such as `>=20` that describes the supported versions.
    ///
    /// This rule reports the `engines` field when:
    ///
    /// - it is not an object;
    /// - one of its property values is not a string;
    /// - one of its property values is an empty string, or a string that contains only whitespace.
    ///
    /// The rule doesn't check whether a string is a valid version range.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "engines": {
    ///         "node": 24
    ///     }
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "engines": {
    ///         "node": ""
    ///     }
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "engines": ["node >=24"]
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///     "engines": {
    ///         "node": "^24.11.0",
    ///         "npm": ">=11"
    ///     }
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - To report empty property names, including those in `engines`, see [`noEmptyObjectKeys`](https://biomejs.dev/linter/rules/no-empty-object-keys/).
    ///
    pub UsePackageJsonValidEngines {
        version: "2.6.3",
        name: "usePackageJsonValidEngines",
        language: "json",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintPackageJson("valid-engines").same()],
    }
}

/// The reason why the `engines` field of a `package.json` file is invalid.
pub enum InvalidEngines {
    /// The `engines` field is not an object.
    NotObject,
    /// A property of `engines` has a value that is not a string.
    NonStringVersionRange,
    /// A property of `engines` has a string value that is empty or contains only whitespace.
    EmptyVersionRange,
}

impl Rule for UsePackageJsonValidEngines {
    type Query = Ast<JsonMember>;
    type State = InvalidEngines;
    type Signals = Option<Self::State>;
    type Options = UsePackageJsonValidEnginesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !is_package_json(ctx.file_path()) {
            return None;
        }

        let member = ctx.query();
        let value = member.value().ok()?;

        if is_engines_field(member) {
            return match value {
                AnyJsonValue::JsonObjectValue(_) => None,
                _ => Some(InvalidEngines::NotObject),
            };
        }

        if !parent_member(member).is_some_and(|parent| is_engines_field(&parent)) {
            return None;
        }

        let AnyJsonValue::JsonStringValue(version_range) = value else {
            return Some(InvalidEngines::NonStringVersionRange);
        };
        let version_range = unescape_json_string(version_range.inner_string_text().ok()?);
        version_range
            .trim()
            .is_empty()
            .then_some(InvalidEngines::EmptyVersionRange)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let member = ctx.query();
        let range = member.value().ok()?.range();
        // The quotes are kept so that the message stays readable when the name is empty.
        let name = member.name().ok()?.value_token()?;
        let name = name.text_trimmed();

        Some(match state {
            InvalidEngines::NotObject => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>"engines"</Emphasis>" field is not an object."
                },
            )
            .note(markup! {
                "Package managers read "<Emphasis>"engines"</Emphasis>" to find the versions of each program that the package supports. They expect an object that maps each program name to a version range."
            })
            .note(markup! {
                "Use an object, such as "<Emphasis>"{ \"node\": \">=24\" }"</Emphasis>"."
            }),
            InvalidEngines::NonStringVersionRange => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The version range for "<Emphasis>{name}</Emphasis>" in "<Emphasis>"engines"</Emphasis>" is not a string."
                },
            )
            .note(markup! {
                "Package managers compare the installed version of "<Emphasis>{name}</Emphasis>" with this value. They can only read a version range that is written as a string."
            })
            .note(markup! {
                "Write the supported versions as a string, such as "<Emphasis>"\">=24\""</Emphasis>"."
            }),
            InvalidEngines::EmptyVersionRange => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The version range for "<Emphasis>{name}</Emphasis>" in "<Emphasis>"engines"</Emphasis>" is empty."
                },
            )
            .note(markup! {
                "An empty version range doesn't tell package managers which versions of "<Emphasis>{name}</Emphasis>" the package supports."
            })
            .note(markup! {
                "Write the supported versions, such as "<Emphasis>"\">=24\""</Emphasis>", or remove this property."
            }),
        })
    }
}

/// Returns `true` if `member` is the `engines` property of the object at the root of the file.
fn is_engines_field(member: &JsonMember) -> bool {
    let is_root_property = member
        .syntax()
        .grand_parent()
        .and_then(|object| object.parent())
        .is_some_and(|parent| JsonRoot::can_cast(parent.kind()));

    is_root_property
        && member
            .name()
            .ok()
            .and_then(|name| name.inner_string_text())
            .is_some_and(|name| name.text() == "engines")
}

/// Returns the member whose value is the object that directly contains `member`.
fn parent_member(member: &JsonMember) -> Option<JsonMember> {
    member
        .syntax()
        .grand_parent()
        .and_then(|object| object.parent())
        .and_then(JsonMember::cast)
}
