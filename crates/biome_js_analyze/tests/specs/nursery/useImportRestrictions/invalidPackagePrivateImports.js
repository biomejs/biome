// Attempt to import from `foo.js` from outside its `sub` module.
import { fooPackageVariable } from "./sub/foo.js";

// Attempt to import from `bar.ts` from outside its `aunt` module.
import { barPackageVariable } from "../aunt/bar.ts";

// Assumed to resolve to a JS/TS file.
import { fooPackageVariable } from "./sub/foo";

// If the `sub/foo` module is inaccessible, so is its index file.
import { fooPackageVariable } from "./sub/foo/index.js";
