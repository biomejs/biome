type A = {new (@dec x, second, @dec third = 'default'): string; }
type B = {method(@dec.fn() x, second, @dec.fn() third = 'default'): string; }
type C = {
 new(@dec() x, second, @dec() third = 'default'): string;
