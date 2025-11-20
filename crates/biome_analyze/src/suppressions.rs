use std::cmp::Ordering;

use crate::{
    AnalyzerSuppression, AnalyzerSuppressionDiagnostic, AnalyzerSuppressionKind,
    AnalyzerSuppressionVariant, MetadataRegistry, RuleCategories, RuleCategory, RuleFilter,
    RuleKey,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_rowan::TextRange;
use rustc_hash::{FxHashMap, FxHashSet};

const PLUGIN_LINT_RULE_FILTER: RuleFilter<'static> = RuleFilter::Group("lint/plugin");

#[derive(Debug)]
pub struct TopLevelSuppression {
    /// Whether this suppression suppresses all filters
    pub(crate) suppressed_categories: RuleCategories,
    /// Filters for the current suppression
    pub(crate) filters_by_category: FxHashMap<RuleCategory, FxHashSet<RuleFilter<'static>>>,
    /// Whether this suppression suppresses all plugins
    pub(crate) suppress_all_plugins: bool,
    /// Current suppressed plugins
    pub(crate) plugins: FxHashSet<String>,
    /// The range of the comment
    pub(crate) comment_range: TextRange,

    /// The range covered by the current suppression.
    /// Eventually, it should hit the entire document
    pub(crate) range: TextRange,
}

impl Default for TopLevelSuppression {
    fn default() -> Self {
        Self {
            suppressed_categories: RuleCategories::empty(),
            filters_by_category: Default::default(),
            suppress_all_plugins: false,
            plugins: Default::default(),
            comment_range: Default::default(),
            range: Default::default(),
        }
    }
}

impl TopLevelSuppression {
    fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        filter: Option<RuleFilter<'static>>,
        comment_range: TextRange,
        is_leading_in_file: bool,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        if suppression.is_top_level() && !is_leading_in_file {
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
            None => self.suppressed_categories.insert(suppression.category),
            Some(PLUGIN_LINT_RULE_FILTER) => self.insert_plugin(&suppression.kind),
            Some(filter) => self.insert(suppression.category, filter),
        }
        self.comment_range = comment_range;
        Ok(())
    }

    pub(crate) fn insert(&mut self, rule_category: RuleCategory, filter: RuleFilter<'static>) {
        let filters = self.filters_by_category.entry(rule_category).or_default();
        filters.insert(filter);
    }

    pub(crate) fn insert_plugin(&mut self, kind: &AnalyzerSuppressionKind) {
        match kind {
            AnalyzerSuppressionKind::Plugin(Some(name)) => {
                self.plugins.insert((*name).to_string());
            }
            AnalyzerSuppressionKind::Plugin(None) => {
                self.suppress_all_plugins = true;
            }
            _ => {}
        }
    }

    pub(crate) fn contains_rule_key(&self, rule_category: &RuleCategory, filter: &RuleKey) -> bool {
        self.filters_by_category
            .get(rule_category)
            .is_some_and(|filters| filters.iter().any(|f| f == filter))
    }

    pub(crate) fn suppressed_plugin(&self, plugin_name: &str) -> bool {
        self.suppress_all_plugins || self.plugins.contains(plugin_name)
    }

    pub(crate) fn expand_range(&mut self, range: TextRange) {
        self.range.cover(range);
    }

    pub(crate) fn has_filter(&self, filter: &RuleFilter) -> bool {
        self.filters_by_category
            .values()
            .any(|filters| filters.contains(filter))
    }
}

/// Single entry for a suppression comment in the `line_suppressions` buffer
#[derive(Debug)]
pub(crate) struct LineSuppression {
    /// Line index this comment is suppressing lint rules for
    pub(crate) line_index: usize,
    /// Range of source text covered by the suppression comment
    pub(crate) comment_span: TextRange,
    /// Range of source text this comment is suppressing lint rules for
    pub(crate) text_range: TextRange,
    /// All rules from groups included here are ignored.
    pub(crate) suppressed_categories: RuleCategories,
    /// The rule this comment should be suppressing.
    pub(crate) suppressed_rule: Option<(RuleCategory, RuleFilter<'static>)>,
    /// An instance this comment should be suppressing.
    ///
    /// For example, this is `foo` in `// biome-ignore lint/correctness/xxx(foo): ...`
    pub(crate) suppressed_instance: Option<Box<str>>,
    /// List of plugins this comment has started suppressing
    pub(crate) suppressed_plugins: FxHashSet<String>,
    /// Set to true if this comment suppress all plugins
    pub(crate) suppress_all_plugins: bool,
    /// Set to `true` when a signal matching this suppression was emitted and
    /// suppressed
    pub(crate) did_suppress_signal: bool,
    /// Points to the previous suppression if this line suppresses a signal
    /// that was already suppressed by another entity (e.g. top-level suppression)
    pub(crate) already_suppressed: Option<TextRange>,
}

impl Default for LineSuppression {
    fn default() -> Self {
        Self {
            line_index: 0,
            comment_span: Default::default(),
            text_range: Default::default(),
            suppressed_categories: RuleCategories::empty(),
            suppressed_rule: Default::default(),
            suppressed_instance: Default::default(),
            suppressed_plugins: Default::default(),
            suppress_all_plugins: false,
            did_suppress_signal: false,
            already_suppressed: None,
        }
    }
}

impl LineSuppression {
    pub(crate) fn matches_rule(&self, rule_category: &RuleCategory, filter: &RuleKey) -> bool {
        self.suppressed_rule
            .as_ref()
            .is_some_and(|(c, f)| c == rule_category && f == filter)
    }
}

#[derive(Debug, Default)]
pub(crate) struct RangeSuppressions {
    pub(crate) suppressions: Vec<RangeSuppression>,
}

#[derive(Debug)]
pub(crate) struct RangeSuppression {
    /// Whether the current suppression should suppress all signals
    pub(crate) suppressed_categories: RuleCategories,

    /// The range of the `biome-ignore-start` suppressions
    pub(crate) start_comment_range: TextRange,

    /// A range that indicates how long this suppression has effect
    pub(crate) suppression_range: TextRange,

    /// Set to `true` when this line suppresses a signal that was already suppressed by another entity e.g. top-level suppression
    pub(crate) already_suppressed: Option<TextRange>,

    /// Whether this suppression has suppressed a signal
    pub(crate) did_suppress_signal: bool,

    /// Indicates if this suppression has found its end comment - if false, the suppression_range is not yet complete
    pub(crate) is_ended: bool,

    /// The rules to suppress, grouped by [`RuleCategory`]
    pub(crate) filters_by_category: FxHashMap<RuleCategory, FxHashSet<RuleFilter<'static>>>,

    /// List of plugins this comment has started suppressing
    pub(crate) suppressed_plugins: FxHashSet<String>,

    /// Set to true if this comment suppress all plugins
    pub(crate) suppress_all_plugins: bool,
}

impl Default for RangeSuppression {
    fn default() -> Self {
        Self {
            suppressed_categories: RuleCategories::empty(),
            start_comment_range: Default::default(),
            suppression_range: Default::default(),
            already_suppressed: None,
            did_suppress_signal: false,
            is_ended: false,
            filters_by_category: Default::default(),
            suppressed_plugins: Default::default(),
            suppress_all_plugins: Default::default(),
        }
    }
}

impl RangeSuppressions {
    /// Expands the range of all range suppressions
    pub(crate) fn expand_range(&mut self, text_range: TextRange) {
        for range_suppression in self.suppressions.iter_mut() {
            if !range_suppression.is_ended {
                range_suppression.suppression_range =
                    range_suppression.suppression_range.cover(text_range);
            }
        }
    }
    pub(crate) fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        filter: Option<RuleFilter<'static>>,
        plugin_name: Option<String>,
        text_range: TextRange,
        already_suppressed: Option<TextRange>,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        if suppression.is_range_start() {
            let mut range_suppression = RangeSuppression::default();
            match filter {
                None => range_suppression
                    .suppressed_categories
                    .insert(suppression.category),
                Some(PLUGIN_LINT_RULE_FILTER) => {
                    if let Some(plugin_name) = plugin_name {
                        range_suppression.suppressed_plugins.insert(plugin_name);
                    } else {
                        range_suppression.suppress_all_plugins = true;
                    }
                }
                Some(filter) => {
                    let filters = range_suppression
                        .filters_by_category
                        .entry(suppression.category)
                        .or_default();
                    filters.insert(filter);
                }
            }
            range_suppression.suppression_range = text_range;
            range_suppression.already_suppressed = already_suppressed;
            range_suppression.start_comment_range = text_range;
            self.suppressions.push(range_suppression);
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

            let range_suppression = match filter {
                None => {
                    self.suppressions.pop();
                    return Ok(());
                }
                Some(PLUGIN_LINT_RULE_FILTER) => self
                    .suppressions
                    .iter_mut()
                    .rev()
                    .filter(|s| !s.is_ended)
                    .find(|s| match &plugin_name {
                        Some(plugin_name) => s.suppressed_plugins.contains(plugin_name),
                        None => s.suppress_all_plugins,
                    }),
                Some(filter) => self
                    .suppressions
                    .iter_mut()
                    .rev()
                    .filter(|s| !s.is_ended)
                    .find(|s| {
                        s.filters_by_category
                            .get(&suppression.category)
                            .is_some_and(|filters| filters.contains(&filter))
                    }),
            };

            if let Some(existing_suppression) = range_suppression {
                // Mark this as ended and expand it by the text range of this comment
                existing_suppression.is_ended = true;
                existing_suppression.suppression_range =
                    existing_suppression.suppression_range.cover(text_range);
            } else {
                let message = markup! {
                    "Found a "<Emphasis>"biome-ignore-end"</Emphasis>" suppression without a "<Emphasis>"biome-ignore-start"</Emphasis>" suppression. This is invalid"
                };

                // This an error. We found a range end suppression without having a range start
                return Err(AnalyzerSuppressionDiagnostic::new(
                    category!("suppressions/incorrect"),
                    text_range,
                    message,
                )
                .hint(markup! {"Remove this suppression."}.to_owned()));
            }
        }
        Ok(())
    }

    /// Checks if there's suppression that suppresses the current rule in the range provided
    pub(crate) fn suppress_rule(
        &mut self,
        rule_category: &RuleCategory,
        filter: &RuleKey,
        position: &TextRange,
    ) -> bool {
        for range_suppression in self.suppressions.iter_mut().rev() {
            if range_suppression
                .suppression_range
                .contains_range(*position)
                && range_suppression
                    .filters_by_category
                    .get(rule_category)
                    .is_some_and(|filters| filters.iter().any(|f| f == filter))
            {
                range_suppression.did_suppress_signal = true;
                return true;
            }
        }
        false
    }

    /// Suppresses the plugin with the given `plugin_name` if there is a suppression comment
    /// for the given position.
    ///
    /// Returns `true` if a matching suppression comment was found, `false` otherwise.
    pub(crate) fn suppress_plugin(&mut self, plugin_name: &str, position: &TextRange) -> bool {
        for range_suppression in self.suppressions.iter_mut().rev() {
            if range_suppression
                .suppression_range
                .contains_range(*position)
                && (range_suppression.suppress_all_plugins
                    || range_suppression.suppressed_plugins.contains(plugin_name))
            {
                range_suppression.did_suppress_signal = true;
                return true;
            }
        }
        false
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
                && range_suppression
                    .filters_by_category
                    .values()
                    .any(|filters| filters.contains(filter))
            {
                return Some(range_suppression.suppression_range);
            }
        }

        None
    }

    /// Finalizes the suppressions after having evaluated the suppression source (i.e. a file)
    /// You would call then when you expect to be done adding suppressions to this object
    pub fn finalize(&self) -> Result<(), Vec<AnalyzerSuppressionDiagnostic>> {
        let mut errors = Vec::new();
        for suppression in self.suppressions.iter() {
            if !suppression.is_ended {
                let diagnostic = AnalyzerSuppressionDiagnostic::new(
                    category!("suppressions/incorrect"),
                    suppression.start_comment_range,
                    "Range suppressions must have a matching biome-ignore-end",
                );
                errors.push(diagnostic);
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
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
        plugin_name: Option<String>,
        instance: Option<String>,
        comment_range: TextRange,
        already_suppressed: Option<TextRange>,
        rule_category: RuleCategory,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        let mut suppression = LineSuppression {
            comment_span: comment_range,
            text_range: comment_range,
            line_index: self.line_index,
            already_suppressed,
            ..Default::default()
        };

        match filter {
            None => {
                suppression.suppressed_categories.insert(rule_category);
            }
            Some(PLUGIN_LINT_RULE_FILTER) => {
                if let Some(plugin_name) = plugin_name {
                    suppression.suppressed_plugins.insert(plugin_name);
                } else {
                    suppression.suppress_all_plugins = true;
                }
            }
            Some(filter) => {
                suppression.suppressed_rule = Some((rule_category, filter));
                suppression.suppressed_instance = instance.map(String::into_boxed_str);
            }
        }
        self.line_suppressions.push(suppression);

        Ok(())
    }

    /// Maps a [suppression](AnalyzerSuppressionKind) to a [RuleFilter]
    fn map_to_rule_filter(
        &self,
        suppression: &AnalyzerSuppression,
        text_range: TextRange,
    ) -> Result<Option<RuleFilter<'static>>, AnalyzerSuppressionDiagnostic> {
        let rule = match suppression.kind {
            AnalyzerSuppressionKind::Everything(_) => return Ok(None),
            AnalyzerSuppressionKind::Rule(rule) => rule,
            AnalyzerSuppressionKind::RuleInstance(rule, _) => rule,
            AnalyzerSuppressionKind::Plugin(_) => return Ok(Some(PLUGIN_LINT_RULE_FILTER)),
        };
        let is_action = suppression.category == RuleCategory::Action;

        let group_rule = rule.split_once('/');

        let filter = match group_rule {
            None => self.metadata.find_group(rule).map(RuleFilter::from),
            Some((group, rule)) => self.metadata.find_rule(group, rule).map(RuleFilter::from),
        };
        match filter {
            None => Err(match group_rule {
                Some((group, rule)) => {
                    if is_action {
                        AnalyzerSuppressionDiagnostic::new_unknown_assist_action(
                            group, rule, text_range,
                        )
                    } else {
                        AnalyzerSuppressionDiagnostic::new_unknown_lint_rule(
                            group, rule, text_range,
                        )
                    }
                }

                None => {
                    if is_action {
                        AnalyzerSuppressionDiagnostic::new_unknown_assist_group(rule, text_range)
                    } else {
                        AnalyzerSuppressionDiagnostic::new_unknown_lint_group(rule, text_range)
                    }
                }
            }),
            Some(filter) => Ok(Some(filter)),
        }
    }

    fn map_to_rule_instances(&self, suppression_kind: &AnalyzerSuppressionKind) -> Option<String> {
        match suppression_kind {
            AnalyzerSuppressionKind::Everything(_)
            | AnalyzerSuppressionKind::Rule(_)
            | AnalyzerSuppressionKind::Plugin(_) => None,
            AnalyzerSuppressionKind::RuleInstance(_, instances) => Some((*instances).to_string()),
        }
    }

    fn map_to_plugin_name(&self, suppression_kind: &AnalyzerSuppressionKind) -> Option<String> {
        match suppression_kind {
            AnalyzerSuppressionKind::Plugin(Some(plugin_name)) => Some((*plugin_name).to_string()),
            _ => None,
        }
    }

    pub(crate) fn push_suppression(
        &mut self,
        suppression: &AnalyzerSuppression,
        comment_range: TextRange,
        is_leading_in_file: bool,
    ) -> Result<(), AnalyzerSuppressionDiagnostic> {
        let filter = self.map_to_rule_filter(suppression, comment_range)?;
        let instances = self.map_to_rule_instances(&suppression.kind);
        let plugin_name: Option<String> = self.map_to_plugin_name(&suppression.kind);
        self.last_suppression = Some(suppression.variant.clone());
        let already_suppressed = self.already_suppressed(filter.as_ref(), &comment_range);
        match suppression.variant {
            AnalyzerSuppressionVariant::Line => self.push_line_suppression(
                filter,
                plugin_name,
                instances,
                comment_range,
                already_suppressed,
                suppression.category,
            ),
            AnalyzerSuppressionVariant::TopLevel => self.top_level_suppression.push_suppression(
                suppression,
                filter,
                comment_range,
                is_leading_in_file,
            ),
            AnalyzerSuppressionVariant::RangeStart | AnalyzerSuppressionVariant::RangeEnd => {
                self.range_suppressions.push_suppression(
                    suppression,
                    filter,
                    plugin_name,
                    comment_range,
                    already_suppressed,
                )
            }
        }
    }

    pub(crate) fn expand_range(&mut self, text_range: TextRange, line_index: usize) -> bool {
        self.top_level_suppression.expand_range(text_range);
        self.range_suppressions.expand_range(text_range);
        let mut found = false;
        for last_suppression in self.line_suppressions.iter_mut().rev() {
            if last_suppression.line_index == line_index {
                last_suppression.text_range = last_suppression.text_range.cover(text_range);
                self.line_index = line_index;
                found = true;
            } else {
                break;
            }
        }
        found
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
                    for last_suppression in self.line_suppressions.iter_mut().rev() {
                        if last_suppression.line_index == next_line_index
                            || last_suppression.line_index + 1 == next_line_index
                        {
                            last_suppression.line_index = next_line_index;
                            last_suppression.text_range =
                                last_suppression.text_range.cover(text_range);
                        } else {
                            break;
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

    /// Finalizes the suppressions after having evaluated the suppression source (i.e. a file)
    /// This exists to validate things like correctly ended range suppressions
    pub fn finalize(&self) -> Result<(), Vec<AnalyzerSuppressionDiagnostic>> {
        // Only range_suppressions have a finalize right now
        self.range_suppressions.finalize()
    }

    pub(crate) fn overlapping_line_suppressions(
        &mut self,
        target: &TextRange,
    ) -> &mut [LineSuppression] {
        let Ok(middle_index) = self.line_suppressions.binary_search_by(|s| {
            if s.text_range.end() < target.start() {
                Ordering::Less
            } else if target.end() < s.text_range.start() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }) else {
            return &mut [];
        };
        // Perf: normally just traversing in both directions should be faster - more than 2
        // comments in a row should be rare, and 2-3 extra comparisons are faster than
        // bisecting twice for left and right border.
        let mut left = middle_index;
        while left > 0 && self.line_suppressions[left - 1].text_range.end() >= target.start() {
            left -= 1;
        }
        let mut right = middle_index;
        while right < self.line_suppressions.len() - 1
            && self.line_suppressions[right + 1].text_range.start() <= target.end()
        {
            right += 1;
        }
        &mut self.line_suppressions[left..=right]
    }
}
