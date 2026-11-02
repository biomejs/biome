/* should generate diagnostics */
const container: { [i: string]: 0 } = {};
declare const key: unknown;

delete container[key as string];
delete container[key!];
