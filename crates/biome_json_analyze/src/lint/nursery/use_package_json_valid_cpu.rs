use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonRoot, TextRange};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_package_json_valid_cpu::UsePackageJsonValidCpuOptions;

use crate::utils::is_package_json;

declare_lint_rule! {
    /// Require the `cpu` field in `package.json` to be a valid list of processor types.
    ///
    /// The `cpu` field lists the processor types that a package can run on, such as `x64` or
    /// `arm64`. Package managers compare the field with the processor type of the machine to
    /// decide whether the package can be installed there. A value that isn't a processor type
    /// never matches, so the package may be installed on, or refused by, the wrong machines.
    ///
    /// This rule checks the `cpu` field of the outermost object in a `package.json` file. It
    /// reports the field when its value isn't an array. It also reports each entry of the array
    /// that isn't one of the following processor types, written as a string:
    ///
    /// - `arm`
    /// - `arm64`
    /// - `ia32`
    /// - `loong64`
    /// - `mips`
    /// - `mipsel`
    /// - `ppc64`
    /// - `riscv64`
    /// - `s390`
    /// - `s390x`
    /// - `x64`
    ///
    /// An entry can start with `!` to exclude a processor type. For example, `"!arm"` means that
    /// the package can't run on `arm` processors. The rule accepts these entries.
    ///
    /// The rule doesn't report a `package.json` file that has no `cpu` field.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The value of the `cpu` field must be an array:
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "cpu": "x64"
    /// }
    /// ```
    ///
    /// Every entry must be a string:
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "cpu": ["x64", 64]
    /// }
    /// ```
    ///
    /// An entry can't be empty or contain only spaces:
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "cpu": ["x64", ""]
    /// }
    /// ```
    ///
    /// Every entry must be one of the processor types listed above:
    ///
    /// ```json,expect_diagnostic,file=package.json
    /// {
    ///     "cpu": ["x86"]
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///     "cpu": ["x64", "arm64"]
    /// }
    /// ```
    ///
    pub UsePackageJsonValidCpu {
        version: "2.6.3",
        name: "usePackageJsonValidCpu",
        language: "json",
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintPackageJson("valid-cpu").same()],
    }
}

/// An invalid value found in a `cpu` field.
pub struct InvalidCpu {
    /// The range of the invalid value: the whole value of the field when it isn't an array,
    /// otherwise one entry of the array.
    range: TextRange,
    kind: InvalidCpuKind,
}

impl Rule for UsePackageJsonValidCpu {
    type Query = Ast<JsonRoot>;
    type State = InvalidCpu;
    type Signals = Box<[Self::State]>;
    type Options = UsePackageJsonValidCpuOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !is_package_json(ctx.file_path()) {
            return Box::default();
        }
        let Ok(AnyJsonValue::JsonObjectValue(package)) = ctx.query().value() else {
            return Box::default();
        };

        let mut signals = Vec::new();
        // Every `cpu` member is checked, so that a duplicated field can't hide an invalid value.
        for member in package.json_member_list().iter().flatten() {
            if !is_cpu_member(&member) {
                continue;
            }
            match member.value() {
                Ok(AnyJsonValue::JsonArrayValue(array)) => {
                    for entry in array.elements().iter().flatten() {
                        if let Some(kind) = InvalidCpuKind::from_entry(&entry) {
                            signals.push(InvalidCpu {
                                range: entry.range(),
                                kind,
                            });
                        }
                    }
                }
                // The parser already reports bogus values, and metavariables only appear in
                // GritQL patterns.
                Ok(AnyJsonValue::JsonBogusValue(_) | AnyJsonValue::JsonMetavariable(_))
                | Err(_) => {}
                Ok(value) => signals.push(InvalidCpu {
                    range: value.range(),
                    kind: InvalidCpuKind::NotArray,
                }),
            }
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range,
            match state.kind {
                InvalidCpuKind::NotArray => markup! {
                    "The value of the "<Emphasis>"cpu"</Emphasis>" field isn't an array."
                },
                InvalidCpuKind::NotString => markup! {
                    "This entry of the "<Emphasis>"cpu"</Emphasis>" field isn't a string."
                },
                InvalidCpuKind::Empty => markup! {
                    "This entry of the "<Emphasis>"cpu"</Emphasis>" field is empty or contains only spaces."
                },
                InvalidCpuKind::Unknown => markup! {
                    "This entry of the "<Emphasis>"cpu"</Emphasis>" field isn't a known processor type."
                },
            },
        );

        Some(match state.kind {
            InvalidCpuKind::NotArray => diagnostic
                .note(markup! {
                    "Package managers expect a list of the processor types that the package supports. They compare the list with the processor type of the machine to decide whether the package can be installed there."
                })
                .note(markup! {
                    "List the supported processor types in an array, for example "<Emphasis>"[\"x64\", \"arm64\"]"</Emphasis>"."
                }),
            InvalidCpuKind::NotString | InvalidCpuKind::Empty | InvalidCpuKind::Unknown => diagnostic
                .note(markup! {
                    "Package managers compare each entry with the processor type of the machine to decide whether the package can be installed there. An entry that isn't a processor type never matches, so the package may be installed on, or refused by, the wrong machines."
                })
                .footer_list(
                    markup! {
                        "Replace the entry with one of the following processor types, in quotes. To exclude a processor type instead, add "<Emphasis>"!"</Emphasis>" before its name."
                    },
                    KNOWN_PROCESSOR_TYPES,
                ),
        })
    }
}

/// The reason an [`InvalidCpu`] value is reported.
#[derive(Clone, Copy)]
enum InvalidCpuKind {
    /// The value of the `cpu` field isn't an array.
    NotArray,
    /// An entry of the array isn't a string.
    NotString,
    /// An entry of the array is a string that is empty or holds only whitespace.
    Empty,
    /// An entry of the array is a string that isn't in [`KNOWN_PROCESSOR_TYPES`].
    Unknown,
}

impl InvalidCpuKind {
    /// Returns the reason `entry` is invalid as an entry of the `cpu` array, or `None` when the
    /// entry is valid.
    ///
    /// A valid entry is one of [`KNOWN_PROCESSOR_TYPES`], optionally prefixed with `!`. The
    /// comparison is case-sensitive. Bogus values and metavariables return `None`.
    fn from_entry(entry: &AnyJsonValue) -> Option<Self> {
        let text = match entry {
            AnyJsonValue::JsonStringValue(string) => string.inner_string_text().ok()?,
            AnyJsonValue::JsonBogusValue(_) | AnyJsonValue::JsonMetavariable(_) => return None,
            _ => return Some(Self::NotString),
        };
        let text = text.text();
        if text.trim().is_empty() {
            return Some(Self::Empty);
        }

        let processor_type = text.strip_prefix('!').unwrap_or(text);
        KNOWN_PROCESSOR_TYPES
            .binary_search(&processor_type)
            .is_err()
            .then_some(Self::Unknown)
    }
}

fn is_cpu_member(member: &JsonMember) -> bool {
    member
        .name()
        .ok()
        .and_then(|name| name.inner_string_text())
        .is_some_and(|name| name.text() == "cpu")
}

/// The processor types accepted in the `cpu` field, sorted for binary search.
const KNOWN_PROCESSOR_TYPES: &[&str] = &[
    "arm", "arm64", "ia32", "loong64", "mips", "mipsel", "ppc64", "riscv64", "s390", "s390x", "x64",
];

#[cfg(test)]
mod tests {
    use super::KNOWN_PROCESSOR_TYPES;

    #[test]
    fn known_processor_types_are_sorted() {
        for pair in KNOWN_PROCESSOR_TYPES.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{:?} must come before {:?}",
                pair[1],
                pair[0]
            );
        }
    }
}
