// should generate diagnostics
const values = [1, 2, 3];
values.some(() => null);
values.every(() => ({}));
values.filter((): string | undefined => undefined);
values.find((): boolean | undefined => undefined);
values.findIndex((): number | undefined => undefined);
values.findLast(() => null);
values.findLastIndex(() => null);
values["filter"](() => null);
values.some((() => null));
function predicate(): number | null { return null; }
values.filter(predicate);
