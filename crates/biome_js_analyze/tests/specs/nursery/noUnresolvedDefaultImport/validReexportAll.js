/* should not generate diagnostics */
// `export *` doesn't re-export the default export.
export * from "./validDefaultExport.js";
export const c = "foo";
