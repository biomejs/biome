export declare function make(url: string, options?: { modern?: true }): string;

/** @deprecated Use the modern options instead. */
export declare function make(url: string, options: { legacy: true }): string;

/** @deprecated Use `make` instead. */
export declare function legacyMake(url: string): string;

/** @deprecated Use `make` instead. */
export declare function legacyMake(url: URL): string;
