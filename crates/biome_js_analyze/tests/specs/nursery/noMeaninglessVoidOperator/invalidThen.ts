/* should generate diagnostics */
declare const parameterless: { then(): void };
declare const nonCallback: { then(value: number): void };
void parameterless;
void nonCallback;
