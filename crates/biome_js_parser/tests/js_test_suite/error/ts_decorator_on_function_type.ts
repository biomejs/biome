type I = (@dec x, second, @dec third = 'default') => string;
type I = (@dec.fn() x, second, @dec.fn() third = 'default') => string;
type I = (@dec() x, second, @dec() third = 'default') => string;
