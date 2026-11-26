/* should generate diagnostics */
b.map(((x) => ({ ...x })) as MyCustomMapper);
b.map((x) => ({ ...x }) as Foo);
b.map((x) => ({ ...x }) satisfies Foo);
b!.map((x) => ({ ...x }));
