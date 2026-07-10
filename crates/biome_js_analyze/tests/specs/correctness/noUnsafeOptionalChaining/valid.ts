/* should not generate diagnostics */
new (a?.b as any ?? Fallback)();
(a?.b as any)?.();
new (a?.b<string> ?? Fallback)();
