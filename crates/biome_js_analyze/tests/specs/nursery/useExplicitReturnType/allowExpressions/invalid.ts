/* should generate diagnostics */

// Function declarations still require return types
function test() {
  return;
}

// Variable-assigned functions still require return types with allowExpressions
const foo = () => {};
const bar = function () {};

// Export default without return type
export default function () {}

// Class property functions still require return types
class Foo {
  public a = () => {};
  public b = function () {};
  public c = function test() {};

  static d = () => {};
  static e = function () {};
}
