/* should not generate diagnostics */
// These statements export the same declarations from `a.js`.
export * from "./exportAll/a.js";
export * from "./exportAll/a.js";
export * from "./exportAll/reexportA.js";
export * from "./exportAll/namedA.js";
export * from "./exportAll/importedA.js";
export * from "./exportAll/namespaceA1.js";
export * from "./exportAll/namespaceA2.js";

// The module's own exports are the same declarations from `a.js`.
export { x } from "./exportAll/a.js";
import { f } from "./exportAll/a.js";
export { f };

// `export *` doesn't re-export the default export.
export default 1;

export * from "./exportAll/cycleA.js";
export * from "./unresolved.js";
export const unresolved = 1;
