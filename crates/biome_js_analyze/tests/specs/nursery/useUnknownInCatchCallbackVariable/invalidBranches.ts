/* should generate diagnostics */
declare const condition: boolean;
declare const maybeNullishHandler: null | ((err: any) => void);

Promise.resolve("foo").then(() => {}, condition ? err => {} : err => {});

Promise.resolve("foo").catch(
	condition
		? ((err => {}, err => {}, maybeNullishHandler) ?? (err => {}))
		: (condition && ((err: unknown) => {})) || ((err: unknown) => {}),
);

Promise.resolve("foo").catch(
	condition
		? ((err => {}, err => {}, maybeNullishHandler) ?? ((err: unknown) => {}))
		: (condition && (err => {})) || ((err: unknown) => {}),
);
