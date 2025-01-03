use crate::{
    AnalyzerSuppression, AnalyzerSuppressionDiagnostic, AnalyzerSuppressionKind,
    AnalyzerSuppressionVariant, MetadataRegistry, RuleFilter, RuleKey,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_rowan::{TextRange, TextSize};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Default)]
pub struct TopLevelSuppression {
    /// Whether this suppression suppresses all filters
    pub(crate) suppress_all: bool,
    /// Filters for the current suppression
    pub(crate) filters: FxHashSet<RuleFilter<'static>>,
    /// The range of the comment
    pub(crate) comment_range: TextRange,

    /// The range covered by the current suppression.
    /// Eventually, it should hit the entire document
    pub(crate) range: TextRange,
}

impl TopLevelSuppression {
    fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        filter: Option<RuleFilter<'static>>,
        token_range: TextRange,
        comment_range: TextRange,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        if suppression.is_top_level() && token_range.start() > TextSize::from(0) {
            let mut diagnostic = AnalyzerSuppressionDiagnostic::new(
                category!("suppressions/incorrect"),
                comment_range,
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
        self.comment_range = comment_range;

        Ok(())
    }

    pub(crate) fn insert(&mut self, filter: RuleFilter<'static>) {
        self.filters.insert(filter);
    }

    pub(crate) fn suppressed_rule(&self, filter: &RuleKey) -> bool {
        self.filters.iter().any(|f| f == filter)
    }

    pub(crate) fn expand_range(&mut self, range: TextRange) {
        self.range.cover(range);
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
    pub(crate) already_suppressed: Option<TextRange>,
}

#[derive(Debug, Default)]
pub(crate) struct RangeSuppressions {
    pub(crate) suppressions: Vec<RangeSuppression>,
}

#[derive(Debug, Default)]
pub(crate) struct RangeSuppression {
    /// Whether the current suppression should suppress all signals
    pub(crate) suppress_all: bool,

    /// The range of the `biome-ignore-start` suppressions
    pub(crate) start_comment_range: TextRange,

    /// A range that indicates how long this suppression has effect
    pub(crate) suppression_range: TextRange,

    /// Set to `true` when this line suppresses a signal that was already suppressed by another entity e.g. top-level suppression
    pub(crate) already_suppressed: Option<TextRange>,

    /// Whether this suppression has suppressed a signal
    pub(crate) did_suppress_signal: bool,

    /// The rules to suppress
    pub(crate) filters: FxHashSet<RuleFilter<'static>>,
}

impl RangeSuppressions {
    /// Expands the range of all range suppressions
    pub(crate) fn expand_range(&mut self, text_range: TextRange) {
        for range_suppression in self.suppressions.iter_mut() {
            if !range_suppression.filters.is_empty() {
                range_suppression.suppression_range =
                    range_suppression.suppression_range.cover(text_range);
            }
        }
    }
    pub(crate) fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        filter: Option<RuleFilter<'static>>,
        text_range: TextRange,
        already_suppressed: Option<TextRange>,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        if suppression.is_range_start() {
            if let Some(range_suppression) = self.suppressions.last_mut() {
                match filter {
                    None => {
                        range_suppression.suppress_all = true;
                        range_suppression.already_suppressed = already_suppressed;
                    }
                    Some(filter) => {
                        range_suppression.filters.insert(filter);
                        range_suppression.already_suppressed = already_suppressed;
                    }
                }
            } else {
                let mut range_suppression = RangeSuppression::default();
                match filter {
                    None => range_suppression.suppress_all = true,
                    Some(filter) => {
                        range_suppression.filters.insert(filter);
                    }
                }
                range_suppression.suppression_range = text_range;
                range_suppression.already_suppressed = already_suppressed;
                range_suppression.start_comment_range = text_range;
                self.suppressions.push(range_suppression);
            }
        } else if suppression.is_range_end() {
            if self.suppressions.is_empty() {
                // This an error. We found a range end suppression without having a range start
                return Err(AnalyzerSuppressionDiagnostic::new(
                    category!("suppressions/incorrect"),
                    text_range,
                    markup!{"Found a "<Emphasis>"biome-range-end"</Emphasis>" suppression without a "<Emphasis>"biome-range-start"</Emphasis>" suppression. This is invalid"}
                ).hint(markup!{
                    "Remove this suppression."
                }.to_owned()));
            }

            match filter {
                None => {
                    self.suppressions.pop();
                }
                Some(filter) => {
                    // SAFETY: we checked if the vector isn't empty at the beginning
                    let range_suppression = self.suppressions.last_mut().unwrap();
                    let present = range_suppression.filters.remove(&filter);
                    // the user tried to remove a filter that wasn't added, let's fire a diagnostic
                    if !present {
                        // This an error. We found a range end suppression without having a range start
                        return Err(AnalyzerSuppressionDiagnostic::new(
                            category!("suppressions/incorrect"),
                            text_range,
                            markup!{"Found a "<Emphasis>"biome-range-end"</Emphasis>" suppression without a "<Emphasis>"biome-range-start"</Emphasis>" suppression. This is invalid"}
                        ).hint(markup!{
                            "Remove this suppression."
                        }.to_owned()));
                    }
                }
            }
        }
        Ok(())
    }

    /// Checks if there's suppression that suppresses the current rule in the range provided
    pub(crate) fn suppressed_rule(&mut self, filter: &RuleKey, position: &TextRange) -> bool {
        let range_suppression = self
            .suppressions
            .iter_mut()
            .rev()
            .find(|range_suppression| {
                range_suppression
                    .suppression_range
                    .contains_range(*position)
            });
        let range_suppression = range_suppression
            .filter(|range_suppression| range_suppression.filters.iter().any(|f| f == filter));
        if let Some(range_suppression) = range_suppression {
            range_suppression.did_suppress_signal = true;
            true
        } else {
            false
        }
    }

    /// Whether if the provided `filter` matches ones, given a range.
    pub(crate) fn matches_filter_in_range(
        &self,
        filter: &RuleFilter,
        position: &TextRange,
    ) -> Option<TextRange> {
        for range_suppression in self.suppressions.iter().rev() {
            if range_suppression
                .suppression_range
                .contains_range(*position)
                && range_suppression.filters.contains(filter)
            {
                return Some(range_suppression.suppression_range);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct Suppressions<'analyzer> {
    /// Current line index
    pub(crate) line_index: usize,
    /// Registry metadata, used to find match the rules
    metadata: &'analyzer MetadataRegistry,
    /// Used to track the last suppression pushed.
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

    fn push_line_suppression(
        &mut self,
        filter: Option<RuleFilter<'static>>,
        instance: Option<String>,
        current_range: TextRange,
        already_suppressed: Option<TextRange>,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        if let Some(suppression) = self.line_suppressions.last_mut() {
            if (suppression.line_index) == (self.line_index) {
                suppression.already_suppressed = already_suppressed;

                match filter {
                    None => {
                        suppression.suppress_all = true;
                        suppression.suppressed_rules.clear();
                        suppression.suppressed_instances.clear();
                    }
                    Some(filter) => {
                        suppression.suppressed_rules.insert(filter);
                        if let Some(instance) = instance {
                            suppression.suppressed_instances.insert(instance, filter);
                        }
                        suppression.suppress_all = false;
                    }
                }
                return Ok(());
            }
        }

        let mut suppression = LineSuppression {
            comment_span: current_range,
            text_range: current_range,
            line_index: self.line_index,
            already_suppressed,
            ..Default::default()
        };
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

        Ok(())
    }

    /// Maps a [suppression](AnalyzerSuppressionKind) to a [RuleFilter]
    fn map_to_rule_filter(
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

    fn map_to_rule_instances(&self, suppression_kind: &AnalyzerSuppressionKind) -> Option<String> {
        match suppression_kind {
            AnalyzerSuppressionKind::Everything | AnalyzerSuppressionKind::Rule(_) => None,
            AnalyzerSuppressionKind::RuleInstance(_, instances) => Some((*instances).to_string()),
        }
    }

    pub(crate) fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        comment_range: TextRange,
        token_range_not_trimmed: TextRange,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        let filter = self.map_to_rule_filter(&suppression.kind, comment_range)?;
        let instances = self.map_to_rule_instances(&suppression.kind);
        self.last_suppression = Some(suppression.variant.clone());
        let already_suppressed = self.already_suppressed(filter.as_ref(), &comment_range);
        match suppression.variant {
            AnalyzerSuppressionVariant::Line => {
                self.push_line_suppression(filter, instances, comment_range, already_suppressed)
            }
            AnalyzerSuppressionVariant::TopLevel => self.top_level_suppression.push_suppression(
                suppression,
                filter,
                token_range_not_trimmed,
                comment_range,
            ),
            AnalyzerSuppressionVariant::RangeStart | AnalyzerSuppressionVariant::RangeEnd => self
                .range_suppressions
                .push_suppression(suppression, filter, comment_range, already_suppressed),
        }
    }

    pub(crate) fn expand_range(&mut self, text_range: TextRange, line_index: usize) -> bool {
        self.top_level_suppression.expand_range(text_range);
        self.range_suppressions.expand_range(text_range);
        if let Some(last_suppression) = self.line_suppressions.last_mut() {
            if last_suppression.line_index == line_index {
                last_suppression.text_range = last_suppression.text_range.cover(text_range);
                self.line_index = line_index;
                return true;
            }
        }
        false
    }

    pub(crate) fn bump_line_index(&mut self, line_index: usize) {
        self.line_index = line_index;
    }

    /// If the last suppression was on the same or previous line, extend its range.
    pub(crate) fn overlap_last_suppression(
        &mut self,
        next_line_index: usize,
        text_range: TextRange,
    ) {
        if let Some(variant) = &self.last_suppression {
            match variant {
                AnalyzerSuppressionVariant::Line => {
                    if let Some(last_suppression) = self.line_suppressions.last_mut() {
                        if last_suppression.line_index == next_line_index
                            || last_suppression.line_index + 1 == next_line_index
                        {
                            last_suppression.line_index = next_line_index;
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

    /// Checks if there's top-level suppression or a range suppression that suppresses the given filter.
    /// If so, it returns the text range of that suppression.
    fn already_suppressed(
        &self,
        filter: Option<&RuleFilter>,
        range: &TextRange,
    ) -> Option<TextRange> {
        filter.and_then(|filter| {
            self.top_level_suppression
                .has_filter(filter)
                .then_some(self.top_level_suppression.comment_range)
                .or(self
                    .range_suppressions
                    .matches_filter_in_range(filter, range))
        })
    }
}
