use crate::grit_context::GritQueryContext;
use crate::grit_target_language::GritTargetLanguage;
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::effects::Effect;
use grit_pattern_matcher::pattern::{FileRegistry, ResolvedPattern, get_top_level_effects};
use grit_util::error::{GritPatternError, GritResult};
use grit_util::{AnalysisLogs, CodeRange, EffectKind};
use std::borrow::Cow;
use std::collections::HashMap;

/// Simplified linearization function that applies effects to produce rewritten
/// source text.
///
/// This is a simplified version of the upstream `linearize_binding` that skips
/// padding/indent alignment (not yet implemented for Biome's target languages).
#[expect(clippy::too_many_arguments)]
pub(crate) fn linearize_binding<'a>(
    language: &GritTargetLanguage,
    effects: &[Effect<'a, GritQueryContext>],
    files: &FileRegistry<'a, GritQueryContext>,
    memo: &mut HashMap<CodeRange, Option<String>>,
    source: &'a str,
    range: CodeRange,
    _distributed_indent: Option<usize>,
    logs: &mut AnalysisLogs,
) -> GritResult<Cow<'a, str>> {
    // Get only top-level effects within this range.
    let top_level_effects = get_top_level_effects(effects, memo, &range, language, logs)?;

    if top_level_effects.is_empty() {
        return Ok(Cow::Borrowed(
            &source[range.start as usize..range.end as usize],
        ));
    }

    // For each effect, compute the linearized replacement text.
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    for effect in &top_level_effects {
        let binding = &effect.binding;
        let binding_range = binding.code_range(language);

        if let Some(ref br) = binding_range {
            // Check memo cache for rewrites.
            if matches!(effect.kind, EffectKind::Rewrite) {
                match memo.get(br) {
                    Some(Some(cached_text)) => {
                        let byte_range = binding.range(language).ok_or_else(|| {
                            GritPatternError::new("expected binding to have a range")
                        })?;
                        replacements.push((byte_range.start, byte_range.end, cached_text.clone()));
                        continue;
                    }
                    // `None` marks an in-progress rewrite; skip to avoid recursive re-entry.
                    Some(None) => continue,
                    None => {
                        // Mark as "in progress" to prevent infinite recursion.
                        memo.insert(br.clone(), None);
                    }
                }
            }
        }

        // Recursively linearize the replacement pattern.
        let res = effect
            .pattern
            .linearized_text(language, effects, files, memo, false, logs)?;

        if let Some(ref br) = binding_range
            && matches!(effect.kind, EffectKind::Rewrite)
        {
            memo.insert(br.clone(), Some(res.to_string()));
        }

        let byte_range = binding
            .range(language)
            .ok_or_else(|| GritPatternError::new("expected binding to have a range"))?;
        replacements.push((byte_range.start, byte_range.end, res.into_owned()));
    }

    // Sort replacements by start offset.
    replacements.sort_by_key(|(start, _, _)| *start);

    // Walk source, copying gaps and inserting replacements.
    let mut result = String::with_capacity(range.end as usize - range.start as usize);
    let mut cursor = range.start as usize;

    for (start, end, replacement) in &replacements {
        if *start < cursor {
            return Err(GritPatternError::new(format!(
                "overlapping replacements detected: start={start}, cursor={cursor}"
            )));
        }
        if *start > cursor {
            result.push_str(&source[cursor..*start]);
        }
        result.push_str(replacement);
        cursor = *end;
    }

    // Copy any remaining source after the last replacement.
    if cursor < range.end as usize {
        result.push_str(&source[cursor..range.end as usize]);
    }

    memo.insert(range, Some(result.clone()));
    Ok(Cow::Owned(result))
}

/// Simplified apply_effects: applies accumulated effects to produce rewritten
/// source for a file. Returns the rewritten source as an owned String.
pub(crate) fn apply_effects<'a>(
    source: &'a str,
    effects: &[Effect<'a, GritQueryContext>],
    files: &FileRegistry<'a, GritQueryContext>,
    language: &GritTargetLanguage,
    logs: &mut AnalysisLogs,
) -> GritResult<String> {
    if effects.is_empty() {
        return Ok(source.to_string());
    }

    let mut memo: HashMap<CodeRange, Option<String>> = HashMap::new();
    let len = u32::try_from(source.len())
        .map_err(|_| GritPatternError::new("source file too large for GritQL linearization"))?;
    let range = CodeRange::new(0, len, source);

    let result = linearize_binding(
        language, effects, files, &mut memo, source, range, None, logs,
    )?;
    Ok(result.into_owned())
}
