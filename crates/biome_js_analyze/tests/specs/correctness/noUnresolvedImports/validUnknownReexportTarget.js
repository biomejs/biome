/* should not generate diagnostics */
// The barrel re-exports from a package that cannot be resolved, so it is
// unknown whether the symbol exists; the import must not be flagged here.
import { fromMissingPackage } from "./unknown-reexport-barrel.js";

fromMissingPackage();
