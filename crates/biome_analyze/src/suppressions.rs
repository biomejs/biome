use crate::{
    AnalyzerSuppression, AnalyzerSuppressionDiagnostic, AnalyzerSuppressionKind,
    AnalyzerSuppressionVariant, MetadataRegistry, RuleFilter, RuleKey,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_rowan::{TextRange, TextSize};
use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Default)]
pub struct TopLevelSuppression {
    pub(crate) suppress_all: bool,
    pub(crate) filters: FxHashSet<RuleFilter<'static>>,
    pub(crate) range: Option<TextRange>,
}

impl TopLevelSuppression {
    fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        filter: Option<RuleFilter<'static>>,
        current_range: TextRange,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        if suppression.is_top_level() && current_range.start() > TextSize::from(0) {
            let mut diagnostic = AnalyzerSuppressionDiagnostic::new(
                category!("suppressions/incorrect"),
                current_range,
                "Top level suppressions can only be used at the beginning of the file.",
            );
            if let Some(ignore_range) = suppression.ignore_range {
                diagnostic = diagnostic.note(
                        markup! {"Rename this to "<Emphasis>"biome-ignore"</Emphasis>" or move it to the top of the file"}
                            .to_owned(),
                        ignore_range,
                    );
            }

            return Err(diagnostic);
        }
        // The absence of a filter means that it's a suppression all
        match filter {
            None => self.suppress_all = true,
            Some(filter) => self.insert(filter),
        }
        self.range = Some(current_range);

        Ok(())
    }

    pub(crate) fn insert(&mut self, filter: RuleFilter<'static>) {
        self.filters.insert(filter);
    }

    pub(crate) fn suppressed_rule(&self, filter: &RuleKey) -> bool {
        self.filters.iter().any(|f| f == filter)
    }

    pub(crate) fn expand_range(&mut self, range: TextRange) {
        if let Some(current_range) = self.range.as_mut() {
            current_range.cover(range);
        } else {
            self.range = Some(range);
        }
    }

    pub(crate) fn has_filter(&self, filter: &RuleFilter) -> bool {
        self.filters.contains(filter)
    }
}

/// Single entry for a suppression comment in the `line_suppressions` buffer
#[derive(Default, Debug)]
pub(crate) struct LineSuppression {
    /// Line index this comment is suppressing lint rules for
    pub(crate) line_index: usize,
    /// Range of source text covered by the suppression comment
    pub(crate) comment_span: TextRange,
    /// Range of source text this comment is suppressing lint rules for
    pub(crate) text_range: TextRange,
    /// Set to true if this comment has set the `suppress_all` flag to true
    /// (must be restored to false on expiration)
    pub(crate) suppress_all: bool,
    /// List of all the rules this comment has started suppressing (must be
    /// removed from the suppressed set on expiration)
    pub(crate) suppressed_rules: FxHashSet<RuleFilter<'static>>,
    /// List of all the rule instances this comment has started suppressing.
    pub(crate) suppressed_instances: FxHashMap<String, RuleFilter<'static>>,
    /// Set to `true` when a signal matching this suppression was emitted and
    /// suppressed
    pub(crate) did_suppress_signal: bool,
    /// Set to `true` when this line suppresses a signal that was already suppressed by another entity e.g. top-level suppression
    pub(crate) already_suppressed: bool,
}

#[derive(Debug, Default)]
pub(crate) struct RangeSuppressions {
    pub(crate) suppressions: IndexMap<TextRange, RangeSuppression>,
}
impl RangeSuppressions {
    pub(crate) fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        text_range: TextRange,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        Ok(())
    }
    pub(crate) fn expand_range(&mut self, text_range: TextRange) {}
}

#[derive(Debug, Default)]
pub(crate) struct RangeSuppression {
    pub(crate) line_index: usize,

    /// A range that indicates the span
    pub(crate) range: TextRange,

    /// Set to `true` when this line suppresses a signal that was already suppressed by another entity e.g. top-level suppression
    pub(crate) already_suppressed: bool,

    /// The rules to suppress
    pub(crate) filters: FxHashSet<RuleFilter<'static>>,
}

impl RangeSuppressions {
    pub(crate) fn insert(&mut self, filter: RuleFilter<'static>, position: TextRange) {
        self.suppressions
            .entry(position)
            .and_modify(|range_suppression| {
                range_suppression.filters.insert(filter);
            })
            .or_insert_with(|| {
                let mut range_suppression = RangeSuppression::default();
                range_suppression.filters.insert(filter);
                range_suppression
            });
    }

    pub(crate) fn remove(&mut self, filter: &RuleFilter<'static>) {
        if let Some((_, range_suppression)) = self.suppressions.iter_mut().last() {
            range_suppression.filters.remove(filter);
        }
    }

    pub(crate) fn already_suppressed(&mut self, filter: &RuleFilter<'static>) {
        for (_, range_suppression) in self.suppressions.iter_mut() {
            if range_suppression.filters.contains(filter) {
                range_suppression.already_suppressed = true
            }
        }
    }

    pub(crate) fn suppressed_rule(&self, filter: &RuleKey, position: &TextRange) -> bool {
        self.suppressions
            .iter()
            .filter(|(p, _)| p.contains_range(*position))
            .any(|(_, range_suppression)| range_suppression.filters.iter().any(|f| f == filter))
    }

    pub(crate) fn has_filter_in_range(&self, filter: &RuleFilter, position: &TextRange) -> bool {
        for (range_position, range_suppression) in self.suppressions.iter().rev() {
            if range_position.contains_range(*position) {
                if range_suppression.filters.contains(filter) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct Suppressions<'analyzer> {
    pub(crate) line_index: usize,
    metadata: &'analyzer MetadataRegistry,
    last_suppression: Option<AnalyzerSuppressionVariant>,
    pub(crate) line_suppressions: Vec<LineSuppression>,
    pub(crate) top_level_suppression: TopLevelSuppression,
    pub(crate) range_suppressions: RangeSuppressions,
}

impl<'analyzer> Suppressions<'analyzer> {
    pub(crate) fn new(metadata: &'analyzer MetadataRegistry) -> Self {
        Self {
            line_index: 0,
            metadata,
            line_suppressions: vec![],
            top_level_suppression: TopLevelSuppression::default(),
            range_suppressions: RangeSuppressions::default(),
            last_suppression: None,
        }
    }

    pub(crate) fn cover(&mut self, text_range: TextRange) {
        if let Some(last_suppression) = self.line_suppressions.last_mut() {
            last_suppression.text_range = last_suppression.text_range.cover(text_range)
        }
    }

    fn push_line_suppression(
        &mut self,
        filter: Option<RuleFilter<'static>>,
        instance: Option<String>,
        current_range: TextRange,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        if let Some(suppression) = self.line_suppressions.last_mut() {
            match filter {
                None => {
                    suppression.suppress_all = true;
                }
                Some(filter) => {
                    suppression.suppressed_rules.insert(filter);
                    if let Some(instance) = instance {
                        suppression.suppressed_instances.insert(instance, filter);
                    }
                }
            }
        } else {
            let mut suppression = LineSuppression::default();
            suppression.comment_span = current_range;
            suppression.text_range = current_range;
            match filter {
                None => {
                    suppression.suppress_all = true;
                }
                Some(filter) => {
                    suppression.suppressed_rules.insert(filter);
                    if let Some(instance) = instance {
                        suppression.suppressed_instances.insert(instance, filter);
                    }
                }
            }
            self.line_suppressions.push(suppression);
        }

        Ok(())
    }

    fn rule_to_filter(
        &self,
        suppression_kind: &AnalyzerSuppressionKind,
        text_range: TextRange,
    ) -> Result<Option<RuleFilter<'static>>, AnalyzerSuppressionDiagnostic> {
        let rule = match suppression_kind {
            AnalyzerSuppressionKind::Everything => return Ok(None),
            AnalyzerSuppressionKind::Rule(rule) => rule,
            AnalyzerSuppressionKind::RuleInstance(rule, _) => rule,
        };

        let group_rule = rule.split_once('/');

        let filter = match group_rule {
            None => self.metadata.find_group(rule).map(RuleFilter::from),
            Some((group, rule)) => self.metadata.find_rule(group, rule).map(RuleFilter::from),
        };
        match filter {
            None => Err(match group_rule {
                Some((group, rule)) => AnalyzerSuppressionDiagnostic::new(
                    category!("suppressions/unknownRule"),
                    text_range,
                    format_args!("Unknown lint rule {group}/{rule} in suppression comment"),
                ),

                None => AnalyzerSuppressionDiagnostic::new(
                    category!("suppressions/unknownGroup"),
                    text_range,
                    format_args!("Unknown lint rule group {rule} in suppression comment"),
                ),
            }),
            Some(filter) => Ok(Some(filter)),
        }
    }

    fn rule_to_instance(
        &self,
        suppression_kind: &AnalyzerSuppressionKind,
        text_range: TextRange,
    ) -> Result<Option<String>, AnalyzerSuppressionDiagnostic> {
        match suppression_kind {
            AnalyzerSuppressionKind::Everything | AnalyzerSuppressionKind::Rule(_) => {
                Err(AnalyzerSuppressionDiagnostic::new(
                    category!("suppressions/incorrect"),
                    text_range,
                    "The current suppression doesn't support instances.",
                ))
            }
            AnalyzerSuppressionKind::RuleInstance(_, instances) => Ok(Some(instances.to_string())),
        }
    }

    pub(crate) fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        current_range: TextRange,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        let filter = self.rule_to_filter(&suppression.kind, current_range)?;
        let instances = self.rule_to_instance(&suppression.kind, current_range)?;
        self.last_suppression = Some(suppression.variant.clone());

        match suppression.variant {
            AnalyzerSuppressionVariant::Classic => {
                self.push_line_suppression(filter, instances, current_range)
            }
            AnalyzerSuppressionVariant::TopLevel => {
                self.top_level_suppression
                    .push_suppression(suppression, filter, current_range)
            }
            AnalyzerSuppressionVariant::RangeStart => self
                .range_suppressions
                .push_suppression(suppression, current_range),
            AnalyzerSuppressionVariant::RangeEnd => self
                .range_suppressions
                .push_suppression(suppression, current_range),
        }
    }

    pub(crate) fn update_line_index(&mut self, new_line_index: usize, text_range: TextRange) {
        self.line_index = new_line_index;
        if let Some(variant) = &self.last_suppression {
            match variant {
                AnalyzerSuppressionVariant::Classic => {
                    if let Some(last_suppression) = self.line_suppressions.last_mut() {
                        if last_suppression.line_index == new_line_index
                            || last_suppression.line_index + 1 == new_line_index
                        {
                            last_suppression.line_index = new_line_index;
                            last_suppression.text_range =
                                last_suppression.text_range.cover(text_range);
                        }
                    }
                }
                AnalyzerSuppressionVariant::TopLevel => {
                    self.top_level_suppression.expand_range(text_range);
                }
                AnalyzerSuppressionVariant::RangeStart => {
                    self.range_suppressions.expand_range(text_range)
                }
                AnalyzerSuppressionVariant::RangeEnd => {
                    self.range_suppressions.expand_range(text_range)
                }
            }
        }
    }
}
