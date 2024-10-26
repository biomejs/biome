declare module a {
		function method(@dec x, second, @dec third = 'default') {}
		function method(@dec.fn() x, second, @dec.fn() third = 'default') {}
		function method(@dec() x, second, @dec() third = 'default') {}
}
declare function method(@dec x, second, @dec third = 'default')
declare function method(@dec.fn() x, second, @dec.fn() third = 'default')
declare function method(@dec() x, second, @dec() third = 'default')
