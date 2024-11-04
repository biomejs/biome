({ ...abcd } = a);
({ ...(abcd) } = a);
({ ...m.test } = c);
({ ...m[call()] } = c);
({ ...any.expression().b } = c);
({ b: { ...a } } = c);
