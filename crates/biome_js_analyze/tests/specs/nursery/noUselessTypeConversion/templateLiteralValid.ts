/* should not generate diagnostics */
declare function tag(strings: TemplateStringsArray, ...values: unknown[]): number;

String(tag`hello`);
