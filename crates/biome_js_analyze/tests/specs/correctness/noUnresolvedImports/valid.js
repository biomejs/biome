/* should not generate diagnostics */
import { foo } from "./foo.js";
import bar from "./bar";
import { image } from "./image.svg";

// Node.js built-in modules with the `node:` prefix must never be flagged.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { EventEmitter } from "node:events";
import * as crypto from "node:crypto";

// Bun built-in modules must never be flagged.
import Bun from "bun";
import { dlopen } from "bun:ffi";
import { jsc } from "bun:jsc";
import { Database } from "bun:sqlite";
import { test } from "bun:test";
import { feature } from "bun:bundle";

// JSR packages are downloaded on demand by Deno, so they must never be flagged.
import { assert } from "jsr:@std/assert";
import { camelCase } from "jsr:@luca/cases@^1";
import { serve } from "jsr:@std/http@1.0.0/file-server";
