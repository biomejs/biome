const invalidExplicit = {
    // Basic property shorthand violations
    foo: foo,
    bar: bar,
    baz: baz,

    // Method shorthand violations
    method: function () { return "method"; },
    async: async function () { return "async"; },
    generator: function* () { yield "gen"; },
    asyncGenerator: async function* () { yield "async gen"; },

    // Computed methods shorthand violations
    [computed]: function () { return "computed"; },
    [computed]: async function () { return "async computed"; },
    [computed]: function* () { yield "computed gen"; },
    ["computed-string"]: function () { return "computed string"; },
    ["comp" + "uted" + "-con" + "cat"]: function () { return "computed concat"; },
    [computed()]: function () { return "computed dynamic"; },
};
