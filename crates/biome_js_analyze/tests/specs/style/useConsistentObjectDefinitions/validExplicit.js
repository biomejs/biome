/* should not generate diagnostics */
const validExplicit = {
    // Properties with explicit values
    foo: foo,
    bar: bar,
    baz: baz,

    // Methods with function expressions
    method: function () { return "method"; },
    async: async function () { return "async"; },
    generator: function* () { yield "gen"; },
    asyncGenerator: async function* () { yield "async gen"; },

    // Computed methods
    [computed]: function () { return "computed"; },
    [computed]: async function () { return "async computed"; },

    // Under this sections should go properties that can't be shorthanded
    // Meaning they are valid with either explicit or shorthand property option

    // String literals
    "stringLiteral": "stringLiteral",
    "quotedProperty": quotedProperty,
    'singleQuoted': singleQuoted,

    // Call expressions
    call: example(),
    callLiteral: "example"(),

    // Computed properties
    [dynamic()]: dynamicValue,
    [computed]: computed,
    ["computed-string"]: computedString,

    // Arrow functions
    arrow: () => "arrow",
    arrowWithBlock: () => { return "arrow block"; },
    asyncArrow: async () => "async arrow",

    // Named function expressions
    sameName: function sameName() { return "same name"; },
    otherName: function differentName() { return "different name"; },
    recursive: function recursive(n) { return n > 0 ? recursive(n - 1) : n; },
    asyncNamed: async function asyncNamed() { return "async named"; },
    generatorNamed: function* generatorNamed() { yield "named gen"; },
    [computedNamed]: function computedNamed() { return "computed named"; },

    // Accessors
    get getter() { return "getter"; },
    set setter(value) { this._setter = value; },

    ...spread,
};
