({
    method(@dec x, second, @dec third = 'default') {}
    method(@dec.fn() x, second, @dec.fn() third = 'default') {}
    method(@dec() x, second, @dec() third = 'default') {}
    set val(@dec x) {}
    set val(@dec.fn() x) {}
    set val(@dec() x) {}
})
