const expr = function method(@dec x, second, @dec third = 'default') {}
const expr = function method(@dec.fn() x, second, @dec.fn() third = 'default') {}
const expr = function method(@dec() x, second, @dec() third = 'default') {}
