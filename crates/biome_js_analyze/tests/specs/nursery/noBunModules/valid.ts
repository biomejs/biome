/* should not generate diagnostics */
import type Bun from "bun";
import type * as Bun from "bun";
import type { DatabaseOptions } from "bun:sqlite";
import { type DatabaseOptions } from "bun:sqlite";
declare module "bun:sqlite" { }

// Node.js runtime built-ins are not Bun modules, so they must not be flagged.
import assert from "node:assert";
import buffer from "node:buffer";
import child_process from "node:child_process";
import crypto from "node:crypto";
