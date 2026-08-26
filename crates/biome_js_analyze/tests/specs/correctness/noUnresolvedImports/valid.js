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

// Bun built-in modules with the `bun:` prefix must also never be flagged.
// See: https://github.com/biomejs/biome/issues/11475
import { test, expect } from "bun:test";
import { dlopen, FFIType } from "bun:ffi";
import { serve } from "bun:sqlite";
