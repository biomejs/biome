const invalidShorthand = {
    // Basic property explicit violations
    prop,
    shortProp,

    // Method explicit violations
    method() { return "method"; },
    async async() { return "async"; },
    *generator() { yield "gen"; },
    async *asyncGenerator() { yield "async gen"; },

    // Computed methods
    [computed]() { return "computed"; },
    async [computed]() { return "async computed"; },
    *[computed]() { yield "computed gen"; },
    ["computed-string"]() { return "computed string"; },
    ["comp" + "uted" + "-con" + "cat"]() { return "computed concat"; },
    [computed()]() { return "computed dynamic"; },

    // String literal methods
    'quotedMethod'() { return "quoted"; },
    "doubleQuoted"() { return "double quoted"; },
    async 'asyncQuoted'() { return "async quoted"; },
};
