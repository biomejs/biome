type CustomType = unknown
function fn2(arg: CustomType) {
  return arg;
}


class Foo<T extends CustomType = String> extends Bar<CustomType> implements CustomType<Object> {
  constructor(foo: String | Object) {}

  exit(): CustomType<String> {
    const foo: String = 1 as CustomType;
  }
}


const foo: Bar = 1;

const identifier: InvalidUse = 'foo';