use crate::{
    grit_context::{GritExecContext, GritQueryContext},
    grit_resolved_pattern::GritResolvedPattern,
};
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::{
    binding::Binding,
    constant::Constant,
    context::ExecContext,
    pattern::{
        get_absolute_file_name, CallBuiltIn, JoinFn, LazyBuiltIn, Pattern, ResolvedPattern,
        ResolvedSnippet, State,
    },
};
use grit_util::AnalysisLogs;
use im::Vector;
use path_absolutize::Absolutize;
use rand::{seq::SliceRandom, Rng};
use std::borrow::Cow;
use std::path::Path;

pub type CallableFn = dyn for<'a> Fn(
        &'a [Option<Pattern<GritQueryContext>>],
        &'a GritExecContext<'a>,
        &mut State<'a, GritQueryContext>,
        &mut AnalysisLogs,
    ) -> Result<GritResolvedPattern<'a>>
    + Send
    + Sync;

pub struct BuiltInFunction {
    pub name: &'static str,
    pub params: Vec<&'static str>,
    pub(crate) func: Box<CallableFn>,
}

impl BuiltInFunction {
    fn call<'a>(
        &self,
        args: &'a [Option<Pattern<GritQueryContext>>],
        context: &'a GritExecContext<'a>,
        state: &mut State<'a, GritQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> Result<GritResolvedPattern<'a>> {
        (self.func)(args, context, state, logs)
    }

    pub fn new(name: &'static str, params: Vec<&'static str>, func: Box<CallableFn>) -> Self {
        Self { name, params, func }
    }
}

impl std::fmt::Debug for BuiltInFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BuiltInFunction")
            .field("name", &self.name)
            .field("params", &self.params)
            .finish()
    }
}

#[derive(Debug)]
pub struct BuiltIns(Vec<BuiltInFunction>);

impl Default for BuiltIns {
    fn default() -> Self {
        vec![
            BuiltInFunction::new("resolve", vec!["path"], Box::new(resolve_path_fn)),
            BuiltInFunction::new("capitalize", vec!["string"], Box::new(capitalize_fn)),
            BuiltInFunction::new("lowercase", vec!["string"], Box::new(lowercase_fn)),
            BuiltInFunction::new("uppercase", vec!["string"], Box::new(uppercase_fn)),
            BuiltInFunction::new("text", vec!["string"], Box::new(text_fn)),
            BuiltInFunction::new("trim", vec!["string", "trim_chars"], Box::new(trim_fn)),
            BuiltInFunction::new("join", vec!["list", "separator"], Box::new(join_fn)),
            BuiltInFunction::new("distinct", vec!["list"], Box::new(distinct_fn)),
            BuiltInFunction::new("length", vec!["target"], Box::new(length_fn)),
            BuiltInFunction::new("shuffle", vec!["list"], Box::new(shuffle_fn)),
            BuiltInFunction::new("random", vec!["floor", "ceiling"], Box::new(random_fn)),
            BuiltInFunction::new("split", vec!["string", "separator"], Box::new(split_fn)),
        ]
        .into()
    }
}

impl BuiltIns {
    pub(crate) fn call<'a>(
        &self,
        call: &'a CallBuiltIn<GritQueryContext>,
        context: &'a GritExecContext<'a>,
        state: &mut State<'a, GritQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> Result<GritResolvedPattern<'a>> {
        self.0[call.index].call(&call.args, context, state, logs)
    }

    pub(crate) fn get_built_ins(&self) -> &[BuiltInFunction] {
        &self.0
    }
}

impl From<Vec<BuiltInFunction>> for BuiltIns {
    fn from(built_ins: Vec<BuiltInFunction>) -> Self {
        Self(built_ins)
    }
}

fn capitalize_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        bail!("capitalize() takes 1 argument: string");
    };

    let string = arg1.text(&state.files, context.language())?;
    Ok(ResolvedPattern::from_string(capitalize(&string)))
}

fn distinct_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        bail!("distinct() takes 1 argument: list");
    };

    match arg1 {
        GritResolvedPattern::List(list) => {
            let mut unique_list = Vector::new();
            for item in list {
                if !unique_list.contains(&item) {
                    unique_list.push_back(item);
                }
            }
            Ok(GritResolvedPattern::List(unique_list))
        }
        GritResolvedPattern::Binding(binding) => match binding.last() {
            Some(binding) => {
                let Some(list_items) = binding.list_items() else {
                    bail!("distinct() requires a list as the first argument");
                };

                let mut unique_list = Vector::new();
                for item in list_items {
                    let resolved = ResolvedPattern::from_node_binding(item);
                    if !unique_list.contains(&resolved) {
                        unique_list.push_back(resolved);
                    }
                }
                Ok(GritResolvedPattern::List(unique_list))
            }
            None => Ok(GritResolvedPattern::Binding(binding)),
        },
        _ => bail!("distinct() requires a list as the first argument"),
    }
}

fn join_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let mut args = args.into_iter();
    let (Some(Some(arg1)), Some(Some(arg2))) = (args.next(), args.next()) else {
        bail!("join() takes 2 arguments: list and separator");
    };

    let separator = arg2.text(&state.files, context.language())?;
    let join = if let Some(items) = arg1.get_list_items() {
        JoinFn::from_patterns(items.cloned(), separator.to_string())
    } else if let Some(items) = arg1.get_list_binding_items() {
        JoinFn::from_patterns(items, separator.to_string())
    } else {
        bail!("join() requires a list as the first argument");
    };

    let snippet = ResolvedSnippet::LazyFn(Box::new(LazyBuiltIn::Join(join)));
    Ok(ResolvedPattern::from_resolved_snippet(snippet))
}

fn length_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        bail!("length() takes 1 argument: list or string");
    };

    Ok(match arg1 {
        GritResolvedPattern::List(list) => {
            ResolvedPattern::from_constant(Constant::Integer(list.len().try_into()?))
        }
        GritResolvedPattern::Binding(binding) => match binding.last() {
            Some(resolved_pattern) => {
                let length = if let Some(list_items) = resolved_pattern.list_items() {
                    list_items.count()
                } else {
                    resolved_pattern.text(context.language())?.len()
                };
                ResolvedPattern::from_constant(Constant::Integer(length.try_into()?))
            }
            None => bail!("length() requires a list or string as the first argument"),
        },
        resolved_pattern => {
            let Ok(text) = resolved_pattern.text(&state.files, context.language()) else {
                bail!("length() requires a list or string as the first argument");
            };

            ResolvedPattern::from_constant(Constant::Integer(text.len().try_into()?))
        }
    })
}

fn lowercase_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        bail!("lowercase() takes 1 argument: string");
    };

    let string = arg1.text(&state.files, context.language())?;
    Ok(ResolvedPattern::from_string(string.to_lowercase()))
}

fn random_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;

    match args.as_slice() {
        [Some(start), Some(end)] => {
            let start = start.text(&state.files, context.language())?;
            let end = end.text(&state.files, context.language())?;
            let start = start.parse::<i64>()?;
            let end = end.parse::<i64>()?;
            // Inclusive range
            let value = state.get_rng().gen_range(start..=end);
            Ok(ResolvedPattern::from_constant(Constant::Integer(value)))
        }
        [None, None] => {
            let value = state.get_rng().gen::<f64>();
            Ok(ResolvedPattern::from_constant(Constant::Float(value)))
        }
        _ => bail!("random() takes 0 or 2 arguments: an optional start and end"),
    }
}

/// Turns an arbitrary path into a resolved and normalized absolute path
fn resolve_path_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        bail!("resolve() takes 1 argument: path");
    };

    let current_file = get_absolute_file_name(state, context.language())?;
    let target_path = arg1.text(&state.files, context.language())?;

    let resolved_path = resolve(target_path, current_file.into())?;

    Ok(ResolvedPattern::from_string(resolved_path))
}

fn shuffle_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        bail!("shuffle() takes 1 argument: list");
    };

    let mut list: Vec<_> = if let Some(items) = arg1.get_list_items() {
        items.cloned().collect()
    } else if let Some(items) = arg1.get_list_binding_items() {
        items.collect()
    } else {
        bail!("shuffle() requires a list as the first argument");
    };

    list.shuffle(state.get_rng());
    Ok(GritResolvedPattern::from_list_parts(list.into_iter()))
}

fn split_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let mut args = args.into_iter();
    let (Some(Some(arg1)), Some(Some(arg2))) = (args.next(), args.next()) else {
        bail!("split() takes 2 arguments: string and separator");
    };

    let separator = arg2.text(&state.files, context.language())?;
    let separator = separator.as_ref();

    let string = arg1.text(&state.files, context.language())?;
    let parts = string.split(separator).map(|s| {
        ResolvedPattern::from_resolved_snippet(ResolvedSnippet::Text(s.to_string().into()))
    });
    Ok(ResolvedPattern::from_list_parts(parts))
}

fn text_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        bail!("text() takes 1 argument: string");
    };

    let string = arg1.text(&state.files, context.language())?;
    Ok(ResolvedPattern::from_string(string.to_string()))
}

fn trim_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let mut args = args.into_iter();
    let (Some(Some(arg1)), Some(Some(arg2))) = (args.next(), args.next()) else {
        bail!("trim() takes 2 arguments: string and trim_chars");
    };

    let trim_chars = arg2.text(&state.files, context.language())?;
    let trim_chars: Vec<char> = trim_chars.chars().collect();
    let trim_chars = trim_chars.as_slice();

    let string = arg1.text(&state.files, context.language())?;
    let string = string.trim_matches(trim_chars).to_string();
    Ok(ResolvedPattern::from_string(string))
}

fn uppercase_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> Result<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        bail!("uppercase() takes 1 argument: string");
    };

    let string = arg1.text(&state.files, context.language())?;
    Ok(ResolvedPattern::from_string(string.to_uppercase()))
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn resolve<'a>(target_path: Cow<'a, str>, from_file: Cow<'a, str>) -> Result<String> {
    let Some(source_path) = Path::new(from_file.as_ref()).parent() else {
        bail!("could not get parent directory of file name {}", &from_file);
    };
    let our_path = Path::new(target_path.as_ref());
    let absolutized = our_path.absolutize_from(source_path)?;
    Ok(absolutized
        .to_str()
        .ok_or_else(|| anyhow!("could not build absolute path from file name {target_path}"))?
        .to_owned())
}
