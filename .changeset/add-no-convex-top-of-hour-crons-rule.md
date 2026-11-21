---
"@biomejs/biome": patch
---

Added the nursery rule [`noConvexTopOfHourCrons`](https://biomejs.dev/linter/rules/no-convex-top-of-hour-crons/), which reports Convex cron jobs scheduled at the start of the hour, the busiest time for most apps.

```js
crons.hourly("cleanup", { minuteUTC: 0 }, internal.tasks.cleanup);
```
