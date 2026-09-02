import Bun from "bun";

// with bun: protocol
import { dlopen } from "bun:ffi";
import { jsc } from "bun:jsc";
import { Database } from "bun:sqlite";
import { test } from "bun:test";
import { feature } from "bun:bundle";

// dynamic import
import "bun";

// with bun: protocol
import "bun:ffi";
import "bun:jsc";
import "bun:sqlite";
import "bun:test";
import "bun:bundle";
