/* should not generate diagnostics */
const a1 = [...(foo || [])];
const a2 = [...(foo || {})];
const a3 = [...(foo && {})];
const a4 = { ...(foo && {}) };
const a5 = { ...({} || foo) };
const a6 = { ...({} && foo) };
const a7 = { ...({} ?? foo) };
const a8 = { ...foo };
const a9 = { ...(foo ?? ({} || {})) };
const { ...b1 } = object;
function b2({ ...bar }) {}
const b3 = { ...(foo || {}).toString() };
const b4 = { ...fn(foo || {}) };
const b5 = call({}, ...(foo || {}));
const b6 = { ...(foo || { not: "empty" }) };
const b7 = { ...(foo || { ...{} }) };
const b8 = { ...(foo ? { bar: true } : {}) };
const b9 = { ...(foo ? foo : {}) };
const c1 = { ...(foo || []) };
const c2 = { ...(foo || ({}).bar) };
