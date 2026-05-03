/* should not generate diagnostics */

it("should typecheck correctly", () => {
  expectTypeOf("foo").not.toEqualTypeOf("bar");
});
it("should be a string", () => {
  assertType<string>("foo");
});

describe("math", () => {
  test("1+1", () => {
    assert(1 + 1 === 2);
  });
});

type MyType<T> = (arg: T) => void;

describe("MyType", () => {
  it("should be contravariant", () => {
    // "foo" extends string => T<string> extends T<"foo">
    expectTypeOf("foo").toExtend<string>();
    expectTypeOf<MyType<string>>().toExtend<MyType<"foo">>();
  });
});
