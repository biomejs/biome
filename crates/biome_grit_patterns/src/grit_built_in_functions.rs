use crate::{
    grit_context::{GritExecContext, GritQueryContext},
    grit_resolved_pattern::GritResolvedPattern,
};
use biome_string_case::StrOnlyExtension;
use grit_pattern_matcher::{
    binding::Binding,
    constant::Constant,
    context::ExecContext,
    pattern::{
        get_absolute_file_name, CallBuiltIn, CallbackPattern, JoinFn, LazyBuiltIn, Pattern,
        ResolvedPattern, ResolvedSnippet, State,
    },
};
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs,
};
use path_absolutize::Absolutize;
use rand::{seq::SliceRandom, Rng};
use std::path::Path;
use std::{borrow::Cow, fmt::Debug, num::TryFromIntError};

pub type CallableFn = dyn for<'a> Fn(
        &'a [Option<Pattern<GritQueryContext>>],
        &'a GritExecContext<'a>,
        &mut State<'a, GritQueryContext>,
        &mut AnalysisLogs,
    ) -> GritResult<GritResolvedPattern<'a>>
    + Send
    + Sync;

pub type CallbackFn = dyn for<'a, 'b> Fn(
        &'b GritResolvedPattern<'a>,
        &'a GritExecContext<'a>,
        &mut State<'a, GritQueryContext>,
        &mut AnalysisLogs,
    ) -> GritResult<bool>
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
    ) -> GritResult<GritResolvedPattern<'a>> {
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

pub struct BuiltIns {
    built_ins: Vec<BuiltInFunction>,
    callbacks: Vec<Box<CallbackFn>>,
}

impl Debug for BuiltIns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "BuiltIns {{ built_ins: {:?} }}",
            self.built_ins
        ))
    }
}

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
    ) -> GritResult<GritResolvedPattern<'a>> {
        self.built_ins[call.index].call(&call.args, context, state, logs)
    }

    pub(crate) fn call_callback<'a>(
        &self,
        call: &'a CallbackPattern,
        context: &'a GritExecContext<'a>,
        binding: &GritResolvedPattern<'a>,
        state: &mut State<'a, GritQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        (self.callbacks[call.callback_index])(binding, context, state, logs)
    }

    /// Add an anonymous built-in, used for callbacks
    /// Returns a pattern that can be used to call the callback
    pub fn add_callback(&mut self, func: Box<CallbackFn>) -> Pattern<GritQueryContext> {
        self.callbacks.push(func);
        let index = self.callbacks.len() - 1;
        Pattern::CallbackPattern(Box::new(CallbackPattern::new(index)))
    }

    pub(crate) fn get_built_ins(&self) -> &[BuiltInFunction] {
        &self.built_ins
    }
}

impl From<Vec<BuiltInFunction>> for BuiltIns {
    fn from(built_ins: Vec<BuiltInFunction>) -> Self {
        Self {
            built_ins,
            callbacks: Vec::new(),
        }
    }
}

fn capitalize_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        return Err(GritPatternError::new(
            "capitalize() takes 1 argument: string",
        ));
    };

    let string = arg1.text(&state.files, context.language())?;
    Ok(ResolvedPattern::from_string(
        capitalize(&string).to_string(),
    ))
}

fn distinct_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        return Err(GritPatternError::new("distinct() takes 1 argument: list"));
    };

    match arg1 {
        GritResolvedPattern::List(list) => {
            let mut unique_list = Vec::new();
            for item in list {
                if !unique_list.contains(&item) {
                    unique_list.push(item);
                }
            }
            Ok(GritResolvedPattern::List(unique_list))
        }
        GritResolvedPattern::Binding(binding) => match binding.last() {
            Some(binding) => {
                let Some(list_items) = binding.list_items() else {
                    return Err(GritPatternError::new(
                        "distinct() requires a list as the first argument",
                    ));
                };

                let mut unique_list = Vec::new();
                for item in list_items {
                    let resolved = ResolvedPattern::from_node_binding(item);
                    if !unique_list.contains(&resolved) {
                        unique_list.push(resolved);
                    }
                }
                Ok(GritResolvedPattern::List(unique_list))
            }
            None => Ok(GritResolvedPattern::Binding(binding)),
        },
        _ => Err(GritPatternError::new(
            "distinct() requires a list as the first argument",
        )),
    }
}

fn join_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let mut args = args.into_iter();
    let (Some(Some(arg1)), Some(Some(arg2))) = (args.next(), args.next()) else {
        return Err(GritPatternError::new(
            "join() takes 2 arguments: list and separator",
        ));
    };

    let separator = arg2.text(&state.files, context.language())?;
    let join = if let Some(items) = arg1.get_list_items() {
        JoinFn::from_patterns(items.cloned(), separator.to_string())
    } else if let Some(items) = arg1.get_list_binding_items() {
        JoinFn::from_patterns(items, separator.to_string())
    } else {
        return Err(GritPatternError::new(
            "join() requires a list as the first argument",
        ));
    };

    let snippet = ResolvedSnippet::LazyFn(Box::new(LazyBuiltIn::Join(join)));
    Ok(ResolvedPattern::from_resolved_snippet(snippet))
}

fn length_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        return Err(GritPatternError::new(
            "length() takes 1 argument: list or string",
        ));
    };

    Ok(match arg1 {
        GritResolvedPattern::List(list) => ResolvedPattern::from_constant(Constant::Integer(
            list.len()
                .try_into()
                .map_err(|error: TryFromIntError| GritPatternError::new(error.to_string()))?,
        )),
        GritResolvedPattern::Binding(binding) => {
            match binding.last() {
                Some(resolved_pattern) => {
                    let length = if let Some(list_items) = resolved_pattern.list_items() {
                        list_items.count()
                    } else {
                        resolved_pattern.text(context.language())?.len()
                    };
                    ResolvedPattern::from_constant(Constant::Integer(length.try_into().map_err(
                        |error: TryFromIntError| GritPatternError::new(error.to_string()),
                    )?))
                }
                None => {
                    return Err(GritPatternError::new(
                        "length() requires a list or string as the first argument",
                    ))
                }
            }
        }
        resolved_pattern => {
            let Ok(text) = resolved_pattern.text(&state.files, context.language()) else {
                return Err(GritPatternError::new(
                    "length() requires a list or string as the first argument",
                ));
            };

            ResolvedPattern::from_constant(Constant::Integer(
                text.len()
                    .try_into()
                    .map_err(|error: TryFromIntError| GritPatternError::new(error.to_string()))?,
            ))
        }
    })
}

fn lowercase_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        return Err(GritPatternError::new(
            "lowercase() takes 1 argument: string",
        ));
    };

    let string = arg1.text(&state.files, context.language())?;
    Ok(ResolvedPattern::from_string(
        string.to_lowercase_cow().to_string(),
    ))
}

fn random_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
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
        _ => Err(GritPatternError::new(
            "random() takes 0 or 2 arguments: an optional start and end",
        )),
    }
}

/// Turns an arbitrary path into a resolved and normalized absolute path
fn resolve_path_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        return Err(GritPatternError::new("resolve() takes 1 argument: path"));
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
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        return Err(GritPatternError::new("shuffle() takes 1 argument: list"));
    };

    let mut list: Vec<_> = if let Some(items) = arg1.get_list_items() {
        items.cloned().collect()
    } else if let Some(items) = arg1.get_list_binding_items() {
        items.collect()
    } else {
        return Err(GritPatternError::new(
            "shuffle() requires a list as the first argument",
        ));
    };

    list.shuffle(state.get_rng());
    Ok(GritResolvedPattern::from_list_parts(list.into_iter()))
}

fn split_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let mut args = args.into_iter();
    let (Some(Some(arg1)), Some(Some(arg2))) = (args.next(), args.next()) else {
        return Err(GritPatternError::new(
            "split() takes 2 arguments: string and separator",
        ));
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
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        return Err(GritPatternError::new("text() takes 1 argument: string"));
    };

    let string = arg1.text(&state.files, context.language())?;
    Ok(ResolvedPattern::from_string(string.to_string()))
}

fn trim_fn<'a>(
    args: &'a [Option<Pattern<GritQueryContext>>],
    context: &'a GritExecContext<'a>,
    state: &mut State<'a, GritQueryContext>,
    logs: &mut AnalysisLogs,
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let mut args = args.into_iter();
    let (Some(Some(arg1)), Some(Some(arg2))) = (args.next(), args.next()) else {
        return Err(GritPatternError::new(
            "trim() takes 2 arguments: string and trim_chars",
        ));
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
) -> GritResult<GritResolvedPattern<'a>> {
    let args = GritResolvedPattern::from_patterns(args, state, context, logs)?;
    let Some(Some(arg1)) = args.into_iter().next() else {
        return Err(GritPatternError::new(
            "uppercase() takes 1 argument: string",
        ));
    };

    let string = arg1.text(&state.files, context.language())?;
    Ok(ResolvedPattern::from_string(string.to_uppercase()))
}

fn capitalize(s: &str) -> Cow<str> {
    if let Some(first_char) = s.chars().next() {
        if !first_char.is_uppercase() {
            let rest = &s[first_char.len_utf8()..];
            return Cow::Owned(first_char.to_ascii_uppercase().to_string() + rest);
        }
    }
    Cow::Borrowed(s)
}

fn resolve<'a>(target_path: Cow<'a, str>, from_file: Cow<'a, str>) -> GritResult<String> {
    let Some(source_path) = Path::new(from_file.as_ref()).parent() else {
        return Err(GritPatternError::new(format!(
            "could not get parent directory of file name {}",
            &from_file,
        )));
    };
    let our_path = Path::new(target_path.as_ref());
    let absolutized = our_path.absolutize_from(source_path)?;
    Ok(absolutized
        .to_str()
        .ok_or_else(|| {
            GritPatternError::new("could not build absolute path from file name {target_path}")
        })?
        .to_owned())
}
