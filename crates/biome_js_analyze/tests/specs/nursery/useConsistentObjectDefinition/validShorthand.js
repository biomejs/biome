const validShorthand = {
    // Property shorthand
    foo,
    bar,
    baz,

    // Method shorthand
    method() { return "method"; },
    async async() { return "async"; },
    *generator() { yield "gen"; },
    async *asyncGenerator() { yield "async gen"; },

    // String literal methods
    'quotedMethod'() { return "quoted"; },
    "doubleQuoted"() { return "double quoted"; },
    async 'asyncQuoted'() { return "async quoted"; },

    // Computed methods
    [computed]() { return "computed"; },
    async [computed]() { return "async computed"; },
    *[computed]() { yield "computed gen"; },

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

    // Accessors
    get getter() { return "getter"; },
    set setter(value) { this._setter = value; },

    ...spread,
};
