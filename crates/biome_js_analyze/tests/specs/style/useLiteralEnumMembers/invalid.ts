enum InvalidLiterals {
  A = {},
  B = [],
  C = true,
  D = 1n,
}

enum InvalidTemplateLiteral {
  A = `foo ${0}`,
}

enum InvalidConstructor {
  A = new Set(),
}

enum InvalidExpression {
  A = delete 2,
  B = -a,
  C = void 2,
  D = !0,
}

const variable = 'Test';
enum InvalidVariable {
  A = 'TestStr',
  V = variable,
}

enum Valid {
  A,
}
enum InvalidEnumMember {
  A = Valid.A,
}

const x = 1;
enum Foo {
  A = x << 0,
  B = x >> 0,
  C = x >>> 0,
  D = x | 0,
  E = x & 0,
  F = x ^ 0,
  G = ~x,
}

enum InvalidRef {
  A = A,
  B = InvalidRef.B,
  C = InvalidRef["C"],
  D = E,
  E = InvalidRef.F,
  F = InvalidRef["G"],
  G
}

export {}