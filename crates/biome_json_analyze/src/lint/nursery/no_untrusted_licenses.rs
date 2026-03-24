use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_json_syntax::{JsonMember, JsonRoot};
use biome_package::{RejectReason, SpdxExpression, TrustConfig};
use biome_project_layout::ProjectLayout;
use biome_rowan::{AstSeparatedList, TextRange};
use biome_rule_options::no_untrusted_licenses::NoUntrustedLicensesOptions;
use camino::Utf8Path;

use crate::services::project_layout::PackageJsonFile;
use crate::utils::is_package_json;

declare_lint_rule! {
    /// Disallow dependencies with untrusted licenses.
    ///
    /// When you install a dependency, it comes with a license that defines how you can use it.
    /// Some licenses may not be compatible with your project's requirements. For example,
    /// a proprietary project may not be allowed to use copyleft-licensed dependencies, or
    /// your organization may require all dependencies to use OSI-approved licenses.
    ///
    /// This rule reads the `license` field from each dependency's `package.json` inside
    /// `node_modules` and checks it against the [SPDX license list](https://spdx.org/licenses/).
    /// It supports compound expressions like `MIT OR Apache-2.0`.
    ///
    /// By default, the rule flags dependencies that:
    /// - Have **no** `license` field.
    /// - Have a license that is **not** a valid SPDX identifier.
    /// - Have a license **deprecated** in the SPDX standard.
    ///
    /// :::note
    /// This rule catches only dependencies that are actually used in your project (i.e., imported by some code).
    /// Currently, the `WITH` specifier is currently not supported.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// A dependency whose `package.json` has `"license": "my-custom-license"` is
    /// flagged because the identifier is not part of the SPDX standard:
    ///
    /// ```json,ignore
    /// {
    ///     "dependencies": {
    ///         "untrusted-pkg": "^1.0.0"
    ///     }
    /// }
    /// ```
    ///
    /// A dependency whose `package.json` has no `license` field at all is also
    /// flagged:
    ///
    /// ```json,ignore
    /// {
    ///     "devDependencies": {
    ///         "no-license-pkg": "^1.0.0"
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// A dependency whose `package.json` has `"license": "MIT"` passes because
    /// MIT is a valid, non-deprecated SPDX identifier:
    ///
    /// ```json,ignore
    /// {
    ///     "dependencies": {
    ///         "trusted-pkg": "^1.0.0"
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allow`
    ///
    /// A list of extra license identifiers to accept, even if they are not part of
    /// the SPDX standard. This is useful for custom or proprietary licenses used
    /// inside your organization.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allow": ["LicenseRef-Company", "my-org-license"]
    ///     }
    /// }
    /// ```
    ///
    /// ### `deny`
    ///
    /// A list of license identifiers to explicitly reject, even if they are valid
    /// SPDX identifiers. This lets you block specific licenses that your project
    /// cannot use, for example, copyleft licenses in a proprietary codebase.
    ///
    /// Deny always takes precedence over allow and SPDX validity.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "deny": ["GPL-3.0-only", "AGPL-3.0-only"]
    ///     }
    /// }
    /// ```
    ///
    /// ### `requireOsiApproved`
    ///
    /// When enabled, only licenses that have been approved by the
    /// [Open Source Initiative](https://opensource.org/) are accepted.
    /// Licenses in the `allow` list bypass this check.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "requireOsiApproved": true
    ///     }
    /// }
    /// ```
    ///
    /// ### `requireFsfLibre`
    ///
    /// When enabled, only licenses that are recognized as free/libre by the
    /// [Free Software Foundation](https://www.gnu.org/licenses/license-list.html)
    /// are accepted. Licenses in the `allow` list bypass this check.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "requireFsfLibre": true
    ///     }
    /// }
    /// ```
    ///
    /// ### `ignoreDeprecated`
    ///
    /// When enabled, deprecated SPDX license identifiers are accepted without
    /// being flagged. By default, deprecated identifiers such as `GPL-2.0` (which
    /// should be `GPL-2.0-only` or `GPL-2.0-or-later`) produce a diagnostic.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreDeprecated": true
    ///     }
    /// }
    /// ```
    ///
    pub NoUntrustedLicenses {
        version: "next",
        name: "noUntrustedLicenses",
        language: "json",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Project],
    }
}

/// The dependency groups to check in `package.json`.
const DEPENDENCY_GROUPS: &[&str] = &[
    "dependencies",
    "devDependencies",
    "peerDependencies",
    "optionalDependencies",
];

/// Why a dependency was flagged.
pub enum Reason {
    /// No license field at all.
    Missing,
    /// License string present but rejected for a specific reason.
    Rejected(RejectReason, String),
}

pub struct RuleState {
    dep_name: String,
    dep_group: String,
    reason: Reason,
    range: TextRange,
}

impl Rule for NoUntrustedLicenses {
    type Query = PackageJsonFile<JsonRoot>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = NoUntrustedLicensesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let path = ctx.file_path();
        if !is_package_json(path) {
            return Vec::new();
        }

        let Some(project_layout) = ctx.project_layout() else {
            return Vec::new();
        };

        let Some(project_dir) = path.parent() else {
            return Vec::new();
        };

        let options = ctx.options();

        let trust_config = TrustConfig {
            allow: options.allow.as_deref().unwrap_or_default(),
            deny: options.deny.as_deref().unwrap_or_default(),
            require_osi_approved: options.require_osi_approved.unwrap_or_default(),
            require_fsf_libre: options.require_fsf_libre.unwrap_or_default(),
            ignore_deprecated: options.ignore_deprecated.unwrap_or_default(),
        };

        let root = ctx.query();
        let Some(value) = root.value().ok() else {
            return Vec::new();
        };
        let Some(object) = value.as_json_object_value() else {
            return Vec::new();
        };

        let mut signals = Vec::new();

        for member in object.json_member_list().iter().flatten() {
            let Some(name) = member.name().ok() else {
                continue;
            };
            let Some(name_text) = name.inner_string_text().and_then(|t| t.ok()) else {
                continue;
            };
            if !DEPENDENCY_GROUPS.contains(&name_text.text()) {
                continue;
            }
            let group_name = name_text.text();
            let Some(dep_value) = member.value().ok() else {
                continue;
            };
            let Some(dep_object) = dep_value.as_json_object_value() else {
                continue;
            };

            for dep_member in dep_object.json_member_list().iter().flatten() {
                let Some(state) = check_dep(
                    &dep_member,
                    group_name,
                    project_layout,
                    project_dir,
                    &trust_config,
                ) else {
                    continue;
                };
                signals.push(state);
            }
        }

        signals
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let dep = &state.dep_name;
        let group = &state.dep_group;

        let diagnostic = match &state.reason {
            Reason::Missing => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The dependency "<Emphasis>{dep}</Emphasis>" in "<Emphasis>{group}</Emphasis>" has no license specified."
                },
            )
            .note(markup! {
                "Dependencies without a license field may have unknown legal restrictions."
            }),
            Reason::Rejected(reject_reason, license) => match reject_reason {
                RejectReason::Denied => RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "The dependency "<Emphasis>{dep}</Emphasis>" in "<Emphasis>{group}</Emphasis>" has a denied license: "<Emphasis>{license}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "This license is explicitly listed in the deny option."
                }),
                RejectReason::Untrusted => RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "The dependency "<Emphasis>{dep}</Emphasis>" in "<Emphasis>{group}</Emphasis>" has an untrusted license: "<Emphasis>{license}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Ensure the license is acceptable for your project, or add it to the allow list."
                }),
                RejectReason::NotOsiApproved => RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "The dependency "<Emphasis>{dep}</Emphasis>" in "<Emphasis>{group}</Emphasis>" has a license that is not OSI-approved: "<Emphasis>{license}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Add the license to the allow list or remove it from from the project."
                }),
                RejectReason::NotFsfLibre => RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "The dependency "<Emphasis>{dep}</Emphasis>" in "<Emphasis>{group}</Emphasis>" has a license that is not FSF libre: "<Emphasis>{license}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Add the license to the allow list or remove it from from the project."
                }),
                RejectReason::Deprecated => RuleDiagnostic::new(
                    rule_category!(),
                    state.range,
                    markup! {
                        "The dependency "<Emphasis>{dep}</Emphasis>" in "<Emphasis>{group}</Emphasis>" has a deprecated SPDX license: "<Emphasis>{license}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "This license identifier is deprecated by the SPDX standard. Set ignoreDeprecated to true to accept deprecated licenses, or remove it from the project. Alternatively, contact the author(s) to update the license."
                }),
            },
        };

        Some(diagnostic)
    }
}

fn check_dep(
    dep_member: &JsonMember,
    group_name: &str,
    project_layout: &ProjectLayout,
    project_dir: &Utf8Path,
    trust_config: &TrustConfig<'_>,
) -> Option<RuleState> {
    let dep_name_node = dep_member.name().ok()?;
    let dep_name_token = dep_name_node.as_json_member_name()?.value_token().ok()?;
    let dep_name = dep_name_node.inner_string_text()?.ok()?.text().to_string();

    let dep_manifest = project_layout.get_dependency_manifest(project_dir, &dep_name)?;

    let reason = match dep_manifest.license.as_ref() {
        None => Reason::Missing,
        Some((license, _)) => {
            let license_str = license.to_string();
            match SpdxExpression::parse(&license_str) {
                Some(expr) => expr
                    .check_trust(trust_config)
                    .err()
                    .map(|r| Reason::Rejected(r, license_str))?,
                None => Reason::Rejected(RejectReason::Untrusted, license_str),
            }
        }
    };

    Some(RuleState {
        dep_name,
        dep_group: group_name.to_string(),
        reason,
        range: dep_name_token.text_trimmed_range(),
    })
}
