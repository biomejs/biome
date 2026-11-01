/* should not generate diagnostics */
Promise.all([await promise] as const);
Promise.all([promise as Promise<Foo>]);
