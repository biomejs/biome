use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, AnyJsObjectMember,
    AnyJsObjectMemberName, JsCallExpression, JsPropertyObjectMember,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_convex_top_of_hour_crons::NoConvexTopOfHourCronsOptions;

declare_lint_rule! {
    /// Disallow scheduling Convex cron jobs at the start of the hour.
    ///
    /// Many apps receive the most traffic, webhooks, and scheduled work right at the start of
    /// the hour (minute `:00`). A cron job that runs at that time competes with this peak, so
    /// it's more likely to hit your app's limits.
    ///
    /// When you leave out `minuteUTC`, Convex picks a minute for you and spreads the runs
    /// across the hour. You can also choose a specific minute other than `0` when the job needs
    /// to run at a predictable time.
    ///
    /// This rule reports:
    /// - calls to `hourly()`, `daily()`, `weekly()`, and `monthly()` whose schedule sets
    ///   `minuteUTC` to `0`;
    /// - calls to `cron()` whose cron expression runs at minute `0`, for example `"0 3 * * *"`,
    ///   `"0,30 * * * *"`, or `"0-5 * * * *"`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// crons.hourly("cleanup", { minuteUTC: 0 }, internal.tasks.cleanup);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// crons.daily("report", { hourUTC: 9, minuteUTC: 0 }, internal.tasks.report);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// crons.cron("nightly", "0 3 * * *", internal.tasks.nightly);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// crons.hourly("cleanup", internal.tasks.cleanup);
    /// crons.daily("nightly", { hourUTC: 3 }, internal.tasks.nightly);
    /// crons.hourly("cleanup", { minuteUTC: 37 }, internal.tasks.cleanup);
    /// crons.cron("nightly", "17 3 * * *", internal.tasks.nightly);
    /// crons.interval("sync", { minutes: 15 }, internal.tasks.sync);
    /// ```
    ///
    pub NoConvexTopOfHourCrons {
        version: "2.6.10",
        name: "noConvexTopOfHourCrons",
        language: "js",
        sources: &[RuleSource::EslintConvex("no-top-of-hour-crons").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex],
    }
}

impl Rule for NoConvexTopOfHourCrons {
    type Query = Ast<JsCallExpression>;
    type State = TopOfHourSchedule;
    type Signals = Option<Self::State>;
    type Options = NoConvexTopOfHourCronsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?.omit_parentheses();
        let member = callee.as_js_static_member_expression()?.member().ok()?;
        let method = member.as_js_name()?.value_token().ok()?;
        let [Some(AnyJsCallArgument::AnyJsExpression(schedule))] =
            call.arguments().ok()?.get_arguments_by_index([1])
        else {
            return None;
        };
        let schedule = schedule.omit_type_wrappers()?;

        match method.text_trimmed() {
            "hourly" | "daily" | "weekly" | "monthly" => {
                let AnyJsExpression::JsObjectExpression(object) = schedule else {
                    return None;
                };
                object
                    .members()
                    .iter()
                    .flatten()
                    .find_map(|member| match member {
                        AnyJsObjectMember::JsPropertyObjectMember(property)
                            if is_zero_minute_utc(&property) =>
                        {
                            Some(TopOfHourSchedule::MinuteUtc(property))
                        }
                        _ => None,
                    })
            }
            "cron" => {
                // A tagged template, such as `String.raw`, can return any value.
                if schedule
                    .as_js_template_expression()
                    .is_some_and(|template| template.tag().is_some())
                {
                    return None;
                }
                let value = schedule.as_static_value()?;
                let mut fields = value.as_string_constant()?.split_whitespace();
                let minute = fields.next()?;
                // A cron expression has exactly five fields: minute, hour, day of the month,
                // month, and day of the week.
                if fields.count() != 4 || !minute_field_includes_zero(minute) {
                    return None;
                }
                Some(TopOfHourSchedule::CronExpression(schedule))
            }
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "This cron job runs at the start of the hour."
                },
            )
            .note(markup! {
                "Many apps receive the most traffic and scheduled work at minute "<Emphasis>":00"</Emphasis>", so a job that runs at that time is more likely to hit your app's limits."
            })
            .note(match state {
                TopOfHourSchedule::MinuteUtc(_) => markup! {
                    "Remove "<Emphasis>"minuteUTC"</Emphasis>" to let Convex pick a minute and spread the runs across the hour, or set it to a minute other than "<Emphasis>"0"</Emphasis>"."
                },
                TopOfHourSchedule::CronExpression(_) => markup! {
                    "Use "<Emphasis>"hourly()"</Emphasis>" or "<Emphasis>"daily()"</Emphasis>" without "<Emphasis>"minuteUTC"</Emphasis>" to let Convex pick a minute, use "<Emphasis>"interval()"</Emphasis>" if the job doesn't need to follow the clock, or use a minute other than "<Emphasis>"0"</Emphasis>" in the cron expression."
                },
            }),
        )
    }
}

/// The part of a cron job's schedule that makes it run at minute 0.
pub enum TopOfHourSchedule {
    /// A `minuteUTC: 0` property in the schedule object of `hourly()`, `daily()`, `weekly()`,
    /// or `monthly()`.
    MinuteUtc(JsPropertyObjectMember),
    /// The cron expression string passed to `cron()`.
    CronExpression(AnyJsExpression),
}

impl TopOfHourSchedule {
    fn range(&self) -> TextRange {
        match self {
            Self::MinuteUtc(property) => property.range(),
            Self::CronExpression(expression) => expression.range(),
        }
    }
}

/// Returns `true` for a non-computed `minuteUTC` property whose value is the number literal `0`.
fn is_zero_minute_utc(property: &JsPropertyObjectMember) -> bool {
    let Ok(name @ AnyJsObjectMemberName::JsLiteralMemberName(_)) = property.name() else {
        return false;
    };
    if name.name().is_none_or(|name| name.text() != "minuteUTC") {
        return false;
    }
    matches!(
        property.value().ok().and_then(AnyJsExpression::omit_type_wrappers),
        Some(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number)
        )) if number.as_number() == Some(0.0)
    )
}

/// Returns `true` when the minute field of a cron expression explicitly includes minute 0:
/// a `0` list item such as `0` or `0,30`, or a range or step that starts at 0 such as `0-5` or
/// `0/30`.
///
/// Wildcard steps such as `*/15` also run at minute 0, but they are allowed because the
/// alternative, a step with a non-zero start such as `7/15`, is uncommon and hard to read.
fn minute_field_includes_zero(field: &str) -> bool {
    field.split(',').any(|item| {
        let rest = item.trim_start_matches('0');
        rest.len() < item.len() && (rest.is_empty() || rest.starts_with(['-', '/']))
    })
}
