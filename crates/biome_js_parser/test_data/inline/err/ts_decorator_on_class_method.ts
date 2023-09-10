class A {
    method(@dec x, second, @dec third = 'default') {}
    method(@dec.fn() x, second, @dec.fn() third = 'default') {}
    method(@dec() x, second, @dec() third = 'default') {}
    static method(@dec x, second, @dec third = 'default') {}
    static method(@dec.fn() x, second, @dec.fn() third = 'default') {}
    static method(@dec() x, second, @dec() third = 'default') {}
}
