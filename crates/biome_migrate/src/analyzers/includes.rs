use std::str::FromStr;

use crate::{MigrationAction, declare_migration};
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleAction, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::{Applicability, category};
use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonArrayValue, JsonMember, JsonObjectValue, JsonRoot, T};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TextSize, TriviaPieceKind};

declare_migration! {
    pub(crate) Includes {
        version: "2.0.0",
        name: "includes",
    }
}

impl Rule for Includes {
    type Query = Ast<JsonRoot>;
    type State = State;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let Ok(AnyJsonValue::JsonObjectValue(root)) = root.value() else {
            return Vec::default();
        };
        let mut result = Vec::default();
        for root_member in root.json_member_list().into_iter().flatten() {
            let Ok(name) = root_member.name().and_then(|name| name.inner_string_text()) else {
                continue;
            };
            match name.text() {
                "files" | "formatter" | "linter" | "assist" => {
                    let Ok(AnyJsonValue::JsonObjectValue(object)) = root_member.value() else {
                        continue;
                    };
                    if let Ok(state) = object.try_into() {
                        result.push(state);
                    }
                }
                "overrides" => {
                    let Ok(AnyJsonValue::JsonArrayValue(overrides)) = root_member.value() else {
                        continue;
                    };
                    for override_item in overrides.elements() {
                        let Ok(AnyJsonValue::JsonObjectValue(override_item)) = override_item else {
                            continue;
                        };
                        if let Ok(state) = override_item.try_into() {
                            result.push(state);
                        }
                    }
                }
                _ => {}
            }
        }
        result
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        // We use the range of `include` if it exists or otherwise the range of `ignore`.
        let range = state
            .include
            .as_ref()
            .or(state.ignore.as_ref())
            .map(|node| node.range())
            .unwrap_or_default();
        let mut diagnostic = RuleDiagnostic::new(
            category!("migrate"),
            range,
            markup! {
                <Emphasis>"include"</Emphasis>" and "<Emphasis>"ignore"</Emphasis>" configurations have been replaced by the "<Emphasis>"includes"</Emphasis>" configuration."
            }
            .to_owned(),
        );
        for (glob_error_range, glob_error) in state
            .include
            .iter()
            .flat_map(validate_globs)
            .chain(state.ignore.iter().flat_map(validate_globs))
        {
            diagnostic = diagnostic.detail(glob_error_range, markup! {
                "This glob cannot be converted to the new glob format because it generates the follosing error: "{glob_error.to_string()}
            });
        }
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<MigrationAction> {
        let includes_name = make::json_member_name(make::json_string_literal("includes"));
        let includes_array = AnyJsonValue::JsonArrayValue(state.to_includes());
        let mut mutation = ctx.root().begin();
        if let Some(include) = &state.include {
            if let Some(ignore) = &state.ignore {
                mutation.remove_node(ignore.clone());
                if let Some(trailing_comma) = ignore
                    .syntax()
                    .last_token()
                    .and_then(|ignore_last_token| ignore_last_token.next_token())
                    .filter(|next_token| next_token.kind() == T![,])
                {
                    mutation.remove_token(trailing_comma);
                } else if let Some(leading_comma) = ignore
                    .syntax()
                    .first_token()
                    .and_then(|ignore_last_token| ignore_last_token.prev_token())
                    .filter(|prev_token| prev_token.kind() == T![,])
                {
                    mutation.remove_token(leading_comma);
                }
            }
            let member_name = include.name().ok()?;
            mutation.replace_node(member_name, includes_name);
            mutation.replace_node(include.value().ok()?, includes_array);
        } else if let Some(ignore) = &state.ignore {
            let member_name = ignore.name().ok()?;
            mutation.replace_node(member_name, includes_name);
            mutation.replace_node(ignore.value().ok()?, includes_array);
        } else {
            return None;
        }
        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::Always,
            markup! {
                "Use "<Emphasis>"includes"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Default)]
pub struct State {
    include: Option<JsonMember>,
    ignore: Option<JsonMember>,
}

impl State {
    fn to_includes(&self) -> JsonArrayValue {
        let mut globs = Vec::new();
        if let Some(AnyJsonValue::JsonArrayValue(array)) =
            self.include.as_ref().and_then(|x| x.value().ok())
        {
            for glob in array.elements().into_iter().flatten() {
                let AnyJsonValue::JsonStringValue(glob_value) = glob else {
                    continue;
                };
                let Ok(glob) = glob_value.inner_string_text() else {
                    continue;
                };
                let new_glob = to_biome_glob(&glob, false);
                // Skip globs that generate errors
                if biome_glob::Glob::from_str(&new_glob).is_err() {
                    continue;
                };
                // JSON escape
                let new_glob = new_glob.replace('\\', "\\\\");
                let new_glob = make::json_string_value(make::json_string_literal(&new_glob));
                globs.push(AnyJsonValue::JsonStringValue(new_glob));
            }
        }
        if let Some(AnyJsonValue::JsonArrayValue(array)) =
            self.ignore.as_ref().and_then(|x| x.value().ok())
        {
            if globs.is_empty() {
                let new_glob = make::json_string_value(make::json_string_literal("**"));
                globs.push(AnyJsonValue::JsonStringValue(new_glob));
            }
            for glob in array.elements().into_iter().flatten() {
                let AnyJsonValue::JsonStringValue(glob_value) = glob else {
                    continue;
                };
                let Ok(glob) = glob_value.inner_string_text() else {
                    continue;
                };
                let new_glob = to_biome_glob(&glob, true);
                // Skip globs that generate errors
                if biome_glob::Glob::from_str(&new_glob).is_err() {
                    continue;
                };
                // JSON escape
                let new_glob = new_glob.replace('\\', "\\\\");
                let new_glob = make::json_string_value(make::json_string_literal(&new_glob));
                globs.push(AnyJsonValue::JsonStringValue(new_glob));
            }
        }
        let separator_count = globs.len().checked_sub(1).unwrap_or_default();
        let separators = (0..separator_count)
            .map(|_| make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
        make::json_array_value(
            make::token(T!['[']),
            make::json_array_element_list(globs, separators),
            make::token(T![']']),
        )
    }
}

impl TryFrom<JsonObjectValue> for State {
    type Error = ();

    fn try_from(value: JsonObjectValue) -> Result<Self, Self::Error> {
        let mut result = Self::default();
        for member in value.json_member_list().into_iter().flatten() {
            let member_name = member.name().and_then(|name| name.inner_string_text());
            if member_name.as_ref().is_ok_and(|name| name == &"include") {
                result.include = Some(member);
            } else if member_name.as_ref().is_ok_and(|name| name == &"ignore") {
                result.ignore = Some(member);
            }
        }
        if result.include.is_some() || result.ignore.is_some() {
            Ok(result)
        } else {
            Err(())
        }
    }
}

fn validate_globs(member: &JsonMember) -> Box<[(TextRange, biome_glob::GlobError)]> {
    let Ok(AnyJsonValue::JsonArrayValue(array)) = member.value() else {
        return Default::default();
    };
    let mut result = Vec::new();
    for glob in array.elements().into_iter().flatten() {
        let AnyJsonValue::JsonStringValue(glob_value) = glob else {
            continue;
        };
        let Ok(glob) = glob_value.inner_string_text() else {
            continue;
        };
        let Err(glob_error) = biome_glob::Glob::from_str(&glob) else {
            continue;
        };
        if let biome_glob::GlobError::Regular { kind, .. } = &glob_error
            && matches!(kind, biome_glob::GlobErrorKind::UnsupportedCharacterClass)
        {
            // Ignore errors for characters classes because we escape them in `to_biome_glob`.
            continue;
        }
        let range = glob_value.range();
        let range = glob_error.index().map_or(range, |index| {
            TextRange::at(range.start() + TextSize::from(1 + index), 1u32.into())
        });
        result.push((range, glob_error));
    }
    result.into()
}

fn to_biome_glob(glob: &str, is_exception: bool) -> String {
    let mut result = glob.to_string();
    let mut bytes = glob.bytes().enumerate();
    let mut offset = 0;
    let mut prev_byte = None;
    while let Some((index, byte)) = bytes.next() {
        match byte {
            b'\\' => {
                // Ignore escaped character
                bytes.next();
            }
            b'[' | b']' => {
                // Escape `[` and `]`.
                result.insert(index + offset, '\\');
                offset += 1;
            }
            b'/' if prev_byte != Some(b'*') => {
                let mut look_ahead = bytes.clone().map(|(_, byte)| byte);
                if look_ahead.next() == Some(b'*') && look_ahead.next() != Some(b'*') {
                    // Convert `/*` to `**/*`
                    result.insert_str(index + offset, "/**");
                    offset += 3;
                }
            }
            _ => {}
        }
        prev_byte = Some(byte);
    }
    let result = if let Some(tail) = result.strip_prefix("./") {
        // Biome globs doesn't support `./`
        tail.to_string()
    } else if !result.starts_with("**") {
        format!("**/{result}")
    } else {
        result
    };
    if !is_exception
        && !result.ends_with('*')
        && std::path::Path::new(&result).extension().is_none()
    {
        // The glob tries to match against a directory.
        // In this case we add `**` at the end.
        // For example, `src` is turned into `src/**`
        if result.ends_with('/') {
            format!("{result}**")
        } else {
            format!("{result}/**")
        }
    } else if is_exception {
        format!("!{result}")
    } else {
        result
    }
}

#[test]
fn test_to_biome_glob() {
    assert_eq!(to_biome_glob("src", false), "**/src/**");
    assert_eq!(to_biome_glob("src/", false), "**/src/**");
    assert_eq!(to_biome_glob("./src", false), "src/**");
    assert_eq!(to_biome_glob("src/file.js", false), "**/src/file.js");
    assert_eq!(to_biome_glob("src/**", false), "**/src/**");
    assert_eq!(to_biome_glob("src/*", false), "**/src/**/*");
    assert_eq!(to_biome_glob("**", false), "**");
    assert_eq!(to_biome_glob("**/*", false), "**/*");
    assert_eq!(to_biome_glob("**/src", false), "**/src/**");

    assert_eq!(to_biome_glob("src", true), "!**/src");
    assert_eq!(to_biome_glob("src/", true), "!**/src/");
    assert_eq!(to_biome_glob("./src", true), "!src");
    assert_eq!(to_biome_glob("src/file.js", true), "!**/src/file.js");
    assert_eq!(to_biome_glob("src/**", true), "!**/src/**");
    assert_eq!(to_biome_glob("src/*", true), "!**/src/**/*");
    assert_eq!(to_biome_glob("**", true), "!**");
    assert_eq!(to_biome_glob("**/*", true), "!**/*");
    assert_eq!(to_biome_glob("**/src", true), "!**/src");
}
