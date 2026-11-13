/* should not generate diagnostics */
const a1 = { ...(foo || ({} as Foo)) };
const a2 = { ...(foo ?? <Foo>{}) };
const a3 = { ...(foo || ({} satisfies Foo)) };
