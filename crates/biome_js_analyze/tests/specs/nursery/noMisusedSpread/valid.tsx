/* should not generate diagnostics */
const props = { a: 1, b: 2 };
const fromObject = <div {...props} />;

declare const objectOrAny: { a: number; b: number } | any;
const fromObjectOrAny = <div {...objectOrAny} />;
