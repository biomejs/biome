/* should generate diagnostics */
const a1 = { ...(foo || {}) };
const a2 = { ...(foo ?? {}) };
const a3 = { ...(foo ?? (( {} ))) };
const a4 = { ...((( foo )) ?? (( {} ))) };
const a5 = { ...(( (( foo )) ?? (( {} )) )) };
const a6 = { ...foo || {} };
const a7 = { ...(foo || {}), };
const a8 = { bar, ...(foo || {}), baz: 1 };
async () => ({ ...((await foo) || {}) });
const b1 = { ...(0 || {}) };
const b2 = { ...(null || {}) };
const b3 = { ...(undefined || {}) };
const b4 = { ...("" || {}) };
const b5 = { ...((a && b) || {}) };
const b6 = { ...(a && b || {}) };
const b7 = { ...({} || {}) };
const b8 = { ...(foo.bar?.baz || {}) };
const c1 = { ...((foo ?? {}) || {}) };
const c2 = { ...({ ...(foo || {}) }) };
const c3 = { ...({ ...((0, foo) || {}) }) };
function d1(a = { ...(bar || {}) }) {}
// Comments that the fix keeps
const e1 = { ...(getObject(/* keep */) || {}) };
const e2 = { ...((/* keep */ foo) ?? {}) };
const e3 = { ...({ /* keep */ value: 1 } || {}) };
const e4 = { ...(getObject(// keep
) ?? {}) };
const e5 = { /* keep */ ...(foo || {}) /* keep */ };
// Comments that would be removed, so there is no fix
const f1 = { ...(foo || { /* keep */ }) };
const f2 = { ...((foo) /* keep */ ?? {}) };
const f3 = { ...(foo /* keep */ || {}) };
const f4 = { ...(foo || /* keep */ {}) };
const f5 = { ...(/* keep */ foo || {}) };
