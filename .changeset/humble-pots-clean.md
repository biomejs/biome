---
"@biomejs/biome": patch
---

Type inference performance has been significantly improved. Some popular libraries like Zod, Valibot, Arktype, Effect, Kysely, and Drizzle have gained a ~2-240x speedup in our benchmarks. This improvement affects all rules that use type information.
