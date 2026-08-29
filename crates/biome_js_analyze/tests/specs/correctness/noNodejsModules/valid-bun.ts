/* should not generate diagnostics */
// Bun runtime built-ins are not Node.js modules, so they must not be flagged.
import { test } from "bun:test";
import { Database } from "bun:sqlite";
import { dlopen } from "bun:ffi";
import Bun from "bun";
