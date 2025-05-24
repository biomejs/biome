/* should not generate diagnostics */
({ ...foo });
({ ...baz, foo: 'bar' });
Object.assign(foo, { bar: baz });
Object.assign(foo, bar);
Object.assign(foo, { bar, baz });
Object.assign(foo, { ...baz });
