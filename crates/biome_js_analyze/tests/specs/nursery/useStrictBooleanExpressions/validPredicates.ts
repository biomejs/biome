// should not generate diagnostics
const values = [1, 2, 3];
values.some(() => true);
values.filter(() => 1);
values.find(() => "yes");
values.map(() => null);
values.forEach(() => null);
const custom = { filter(predicate: () => unknown) {} };
custom.filter(() => null);
declare const maybeCustom: { filter(predicate: () => unknown): void } | undefined;
maybeCustom?.filter(() => null);
declare const nothing: undefined;
nothing?.filter(() => null);
