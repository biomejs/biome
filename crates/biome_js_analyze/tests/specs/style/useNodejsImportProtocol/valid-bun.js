/* should not generate diagnostics */
// Bun runtime built-ins use the `bun:` scheme and must not be rewritten to `node:`.
import { test } from "bun:test";
import { Database } from "bun:sqlite";
