throw new Error();
throw Error();
throw new Error("");
throw new Error(``);

new AggregateError()
new EvalError()
new InternalError()
new RangeError()
new ReferenceError()
new SyntaxError()
new TypeError()
new URIError()

throw new Error([]);
throw new Error([foo]);
throw new Error({});
throw new Error({ foo });
throw new Error(1);
throw new Error(undefined);
throw new Error(null);
throw new Error(true);

new AggregateError(errors);
new AggregateError(errors);
new AggregateError(errors, "");
new AggregateError(errors, ``);
new AggregateError(errors, "", extraArgument);

new AggregateError(errors, []);
new AggregateError(errors, [foo]);
new AggregateError(errors, [0][0]);
new AggregateError(errors, {});
new AggregateError(errors, { foo });
