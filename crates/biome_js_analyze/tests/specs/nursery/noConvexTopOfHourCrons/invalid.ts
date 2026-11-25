/* should generate diagnostics */
crons.hourly("as const", { minuteUTC: 0 } as const, internal.tasks.cleanup);
crons.hourly("satisfies", { minuteUTC: 0 } satisfies Schedule, internal.tasks.cleanup);
crons.hourly("value as const", { minuteUTC: 0 as const }, internal.tasks.cleanup);
crons.hourly("non-null", { minuteUTC: 0! }, internal.tasks.cleanup);
crons.hourly("type assertion", <Schedule>{ minuteUTC: 0 }, internal.tasks.cleanup);
crons.cron("as const", "0 3 * * *" as const, internal.tasks.nightly);
