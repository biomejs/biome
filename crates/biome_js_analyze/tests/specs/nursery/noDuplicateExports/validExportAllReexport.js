/* should not generate diagnostics */
// `reexportB.js` exports the same declarations as `b.js`.
export * from "./exportAll/b.js";
export * from "./exportAll/reexportB.js";
