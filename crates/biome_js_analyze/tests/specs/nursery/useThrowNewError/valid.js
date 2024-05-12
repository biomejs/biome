throw new Error();
new Error();
throw new TypeError();
throw new EvalError();
throw new RangeError();
throw new ReferenceError();
throw new SyntaxError();
throw new URIError();
throw new CustomError();
throw new FooBarBazError();
throw new ABCError();

// Not `FooError` like
throw getError();
// Not `CallExpression`
throw CustomError;
// Not `Identifier` / `MemberExpression`
throw getErrorConstructor()();
// `MemberExpression.computed`
throw lib[Error]();
// `MemberExpression.property` not `Identifier`
throw lib["Error"]();
// Not `FooError` like
throw lib.getError();
