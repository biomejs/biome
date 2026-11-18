const a = makeFactory<Value>;
const b = obj.makeFactory<Value>;

// Parentheses around await/yield must be kept, otherwise the type
// arguments would bind to the inner expression and change the meaning.
async function withAwait() {
  const c = (await makeFactory)<Value>;
}

function* withYield() {
  const d = (yield* makeFactory)<Value>;
  const e = (yield makeFactory)<Value>;
}

// Parentheses around other lower-precedence expressions must be kept too.
const f = (cond ? a : b)<Value>;
const g = (x++)<Value>;

// Binary, logical and type-cast operands need them too.
const h = (a + b)<Value>;
const i = (a ?? b)<Value>;
const j = (a in b)<Value>;
const k = (a ** b)<Value>;
const l = (a as B)<Value>;
const m = (a satisfies B)<Value>;
const n = (<B>a)<Value>;
