use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize::json::unescape_json_string;
use biome_diagnostics::Severity;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonRoot};
use biome_package::{LICENSE_LIST, SpdxExpression};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_package_json_valid_license::UsePackageJsonValidLicenseOptions;

use crate::utils::is_package_json;

declare_lint_rule! {
    /// Require the `license` field in `package.json` to be a valid license.
    ///
    /// The `license` field tells people how they're allowed to use, change, and share a package.
    /// Package registries, such as the npm registry, and license checking tools read this field.
    /// They only understand a license that is written in a standard way. A misspelled or made-up
    /// name, such as `GPL3` or `Apache 2.0`, leaves the terms of the package unclear.
    ///
    /// This rule checks the `license` field of the outermost object in a `package.json` file. The
    /// field is also checked when it's spelled `licence`. The rule reports the field when its value
    /// isn't a string, when the string is empty, and when the string isn't one of the following:
    ///
    /// - An identifier from the [SPDX license list](https://spdx.org/licenses/), such as `MIT` or
    ///   `Apache-2.0`. SPDX is a standard that gives each common license a short, unique name.
    ///   Identifiers are case-sensitive, so `mit` isn't accepted.
    /// - An identifier followed by `+`, such as `LGPL-2.1+`, which means "this version of the
    ///   license or any later version".
    /// - Several identifiers combined with `AND`, `OR`, and parentheses, such as
    ///   `(MIT OR Apache-2.0)`. An identifier can also be followed by `WITH` and the name of an
    ///   exception to the license, such as `GPL-2.0-only WITH Classpath-exception-2.0`. The rule
    ///   doesn't check the name of the exception.
    /// - `UNLICENSED`, which means that the package isn't licensed for use by others.
    /// - `SEE LICENSE IN` followed by the name of a file, such as `SEE LICENSE IN LICENSE.md`, for a
    ///   license that isn't on the SPDX list. The rule doesn't check that the file exists.
    ///
    /// The rule doesn't report a `package.json` file that has no `license` field.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The value must be the SPDX identifier of the license:
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "license": "GPL3"
    /// }
    /// ```
    ///
    /// The value must be a string:
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "license": { "type": "MIT" }
    /// }
    /// ```
    ///
    /// The value can't be empty or contain only spaces:
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "license": ""
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///     "license": "GPL-3.0-only"
    /// }
    /// ```
    ///
    pub UsePackageJsonValidLicense {
        version: "next",
        name: "usePackageJsonValidLicense",
        language: "json",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintPackageJson("valid-license").same()],
    }
}

/// A `license` or `licence` field whose value is invalid.
pub struct InvalidLicense {
    member: JsonMember,
    kind: InvalidLicenseKind,
}

impl Rule for UsePackageJsonValidLicense {
    type Query = Ast<JsonRoot>;
    type State = InvalidLicense;
    type Signals = Box<[Self::State]>;
    type Options = UsePackageJsonValidLicenseOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !is_package_json(ctx.file_path()) {
            return Box::default();
        }
        let Ok(AnyJsonValue::JsonObjectValue(package)) = ctx.query().value() else {
            return Box::default();
        };

        // Every matching member is checked, so that a duplicated field can't hide an invalid
        // value.
        package
            .json_member_list()
            .iter()
            .flatten()
            .filter(is_license_member)
            .filter_map(|member| {
                let kind = InvalidLicenseKind::from_value(&member.value().ok()?)?;
                Some(InvalidLicense { member, kind })
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = unescape_json_string(state.member.name().ok()?.inner_string_text()?);
        let name = name.text();
        let range = state.member.value().ok()?.range();

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "The value of the "<Emphasis>{name}</Emphasis>" field "{state.kind}"."
            },
        )
        .note(markup! {
            "Package registries and license checking tools read this field to tell people how they're allowed to use the package. They only understand a license that is written in a standard way."
        });

        Some(match state.kind {
            InvalidLicenseKind::NotString | InvalidLicenseKind::Empty => diagnostic.note(markup! {
                "Write the license as a string that contains its SPDX identifier, for example "<Emphasis>"\"MIT\""</Emphasis>". You can find the identifier of your license at "<Hyperlink href="https://spdx.org/licenses/">"https://spdx.org/licenses/"</Hyperlink>"."
            }),
            InvalidLicenseKind::Unknown => diagnostic.note(markup! {
                "Replace the value with the SPDX identifier of the license, for example "<Emphasis>"MIT"</Emphasis>". You can find the identifier of your license at "<Hyperlink href="https://spdx.org/licenses/">"https://spdx.org/licenses/"</Hyperlink>". If the license isn't on that list, use "<Emphasis>"SEE LICENSE IN"</Emphasis>" followed by the name of the file that contains it. If others aren't allowed to use the package, use "<Emphasis>"UNLICENSED"</Emphasis>"."
            }),
        })
    }
}

/// The reason an [`InvalidLicense`] field is reported.
#[derive(Clone, Copy)]
enum InvalidLicenseKind {
    /// The value isn't a string.
    NotString,
    /// The value is a string that is empty or holds only whitespace.
    Empty,
    /// The value is a string that isn't an accepted license.
    Unknown,
}

impl biome_console::fmt::Display for InvalidLicenseKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::NotString => "isn't a string",
            Self::Empty => "is empty or contains only spaces",
            Self::Unknown => "isn't a valid license",
        })
    }
}

impl InvalidLicenseKind {
    /// Returns the reason `value` is invalid as the value of the `license` field, or `None` when
    /// the value is valid.
    ///
    /// A valid value is `UNLICENSED`, a reference to a license file such as
    /// `SEE LICENSE IN LICENSE.md`, or an SPDX expression whose license identifiers are all on the
    /// SPDX license list. Bogus values and metavariables return `None`.
    fn from_value(value: &AnyJsonValue) -> Option<Self> {
        let text = match value {
            AnyJsonValue::JsonStringValue(string) => {
                unescape_json_string(string.inner_string_text().ok()?)
            }
            // The parser already reports bogus values, and metavariables only appear in GritQL
            // patterns.
            AnyJsonValue::JsonBogusValue(_) | AnyJsonValue::JsonMetavariable(_) => return None,
            _ => return Some(Self::NotString),
        };
        let text = text.text();
        if text.trim().is_empty() {
            return Some(Self::Empty);
        }

        let is_valid = UNLICENSED_VALUES.contains(&text)
            || is_license_file_reference(text)
            || SpdxExpression::parse(text).is_some_and(|expression| {
                expression.license_ids().into_iter().all(is_known_license_id)
            });
        (!is_valid).then_some(Self::Unknown)
    }
}

fn is_license_member(member: &JsonMember) -> bool {
    member
        .name()
        .ok()
        .and_then(|name| name.inner_string_text())
        .is_some_and(|name| LICENSE_FIELD_NAMES.contains(&unescape_json_string(name).text()))
}

/// Returns `true` when `text` is `SEE LICENSE IN` followed by a file name. The British spelling
/// `SEE LICENCE IN` is accepted too.
fn is_license_file_reference(text: &str) -> bool {
    LICENSE_FILE_PREFIXES.iter().any(|prefix| {
        text.strip_prefix(prefix)
            .is_some_and(|file| !file.trim().is_empty())
    })
}

/// Returns `true` when `id` is on the SPDX license list, optionally followed by `+` to also
/// allow later versions of the license.
///
/// Custom identifiers such as `LicenseRef-Company` aren't on the list, so they return `false`.
fn is_known_license_id(id: &str) -> bool {
    LICENSE_LIST.is_valid(id)
        || id
            .strip_suffix('+')
            .is_some_and(|id| LICENSE_LIST.is_valid(id))
}

/// The names of the field that holds the license, with the American and British spellings.
const LICENSE_FIELD_NAMES: &[&str] = &["license", "licence"];

/// The values that mark a package as not licensed for use by others, with the American and
/// British spellings.
const UNLICENSED_VALUES: &[&str] = &["UNLICENSED", "UNLICENCED"];

/// The prefixes of a value that points to the file containing the license, with the American
/// and British spellings.
const LICENSE_FILE_PREFIXES: &[&str] = &["SEE LICENSE IN ", "SEE LICENCE IN "];
