(foo++)?.();
async () => {
  (await foo)?.();
}
(+foo)?.();
+(+foo);
class Foo extends (+Bar) {}
class Foo extends (Bar ?? Baz) {}
const foo = class extends (Bar ?? Baz) {}
;(1)
;(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa);

(b + c)``;

const foo = { ...(a || b) };

async function *f() {
  await (a || b);
  yield (a && b);
}

const a = () => ({}?.() && a);

(list || list2)?.[(list || list2)];