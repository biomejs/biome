/* should generate diagnostics */
import foo from "./validNamedExports.js";
import foo2, { bar } from "./validNamedExports.js";
import foo3, * as namespace from "./validNamedExports.js";
import reexportedAll from "./validReexportAll.js";
import brokenReexport from "./validReexportMissingDefault.js";
import sideEffects from "./validSideEffects.js";
import reexportedAllUnresolved from "./validReexportAllUnresolved.js";
