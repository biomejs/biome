type I = new(@dec x, second, @dec third = 'default') => string;
type I = abstract new(@dec.fn() x, second, @dec.fn() third = 'default') => string;
type I = abstract new(@dec() x, second, @dec() third = 'default') => string;
