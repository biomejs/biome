// =====================
// TypeScript spread patterns
// =====================

// Type assertion spread
<Foo {...(props as FooProps)} />;

// Non-null assertion spread
<Foo {...props!} />;

// Satisfies operator
<Foo {...(props satisfies Props)} />;

// Generic function call
<Foo {...getProps<T>(arg)} />;

// As const spread
<Foo {...({ a: 1 } as const)} />;

// Type assertion with member expression
<Foo {...(obj.props as FooProps)} />;

// Non-null assertion with call expression
<Foo {...getData()!} />;

// Complex type assertion
<Foo {...(condition ? a : b) as Props} />;

// Type assertion in spread children
<Foo>{...(items as Item[])}</Foo>;

// Non-null assertion in spread children
<Foo>{...items!}</Foo>;
