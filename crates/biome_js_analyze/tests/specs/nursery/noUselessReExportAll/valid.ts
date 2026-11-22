/* should not generate diagnostics */
export * from "./modules/typesOnly";
export * from "./modules/declaration";
// The module graph doesn't record these exports.
export * from "./modules/importEquals";
export * from "./modules/qualifiedNamespace";
