/* should not generate diagnostics */

import { getPayload, getString } from "./api.ts";

// Normal assignments without `any`
const a = 1;
const b: string = "hello";
let c = true;
const d = getString();

// Explicit `any` annotation — developer opted in
const e: any = getPayload();

// Explicit `unknown` annotation — safe alternative
const f: unknown = getPayload();

// Variable without initializer
let g: string;

// Cross-module calls whose return type can't be resolved should not
// produce false positives. Biome doesn't parse .d.ts from node_modules,
// so these remain as unresolved TypeofExpression types.
import fs from "node:fs/promises";
const h = fs.readFileSync("test.txt", "utf-8");

import { someFunction } from "some-external-package";
const i = someFunction();
