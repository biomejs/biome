/* should generate diagnostics */
new (a?.b as any)();
(a?.b as any)();
(a?.b as any).c;
(a?.b satisfies any).c;
(<any>a?.b)();
new (a?.b<string>)();
for (const x of a?.b as any[]);
