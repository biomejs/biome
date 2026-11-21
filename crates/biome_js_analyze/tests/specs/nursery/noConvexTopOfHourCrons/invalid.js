/* should generate diagnostics */
crons.hourly("cleanup", { minuteUTC: 0 }, internal.tasks.cleanup);
crons.daily("report", { hourUTC: 9, minuteUTC: 0 }, internal.tasks.report);
crons.weekly("digest", { dayOfWeek: "monday", hourUTC: 9, minuteUTC: 0 }, internal.tasks.digest);
crons.monthly("invoice", { day: 1, hourUTC: 9, minuteUTC: 0 }, internal.tasks.invoice);
crons.hourly("quoted key", { "minuteUTC": 0 }, internal.tasks.cleanup);
crons.hourly("zero float", { minuteUTC: 0.0 }, internal.tasks.cleanup);
crons.hourly("parenthesized", ({ minuteUTC: (0) }), internal.tasks.cleanup);
(crons.hourly)("parenthesized callee", { minuteUTC: 0 }, internal.tasks.cleanup);
crons?.hourly("optional call", { minuteUTC: 0 }, internal.tasks.cleanup);
crons.cron("nightly", "0 3 * * *", internal.tasks.nightly);
crons.cron("every hour", "0 * * * *", internal.tasks.hourly);
crons.cron("list", "0,30 * * * *", internal.tasks.halfHourly);
crons.cron("list with zero last", "15,0 * * * *", internal.tasks.list);
crons.cron("range", "0-5 * * * *", internal.tasks.range);
crons.cron("step from zero", "0/30 * * * *", internal.tasks.step);
crons.cron("padded zero", "00 12 * * *", internal.tasks.padded);
crons.cron("extra whitespace", "  0   3 * * 1  ", internal.tasks.whitespace);
crons.cron("template", `0 3 * * *`, internal.tasks.template);
