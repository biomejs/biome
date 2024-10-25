export type Fs = typeof import('fs', { with: { 'resolution-mode': 'import' } });
export type TypeFromRequire =
    import("pkg", { with: { "resolution-mode": "require" } }).TypeFromRequire;
export type TypeFromImport =
    import("pkg", { with: { "resolution-mode": "import" } }).TypeFromImport;