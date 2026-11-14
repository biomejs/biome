/* should not generate diagnostics */
import * as importedDeep from "./validDeep.js";
import { e as renamedE } from "./validDeep.js";
export * from "./validNamedExports.js";
export { importedDeep, renamedE };
