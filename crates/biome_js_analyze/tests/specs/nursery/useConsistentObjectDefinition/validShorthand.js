const obj = {
    foo,
    bar() { return "bar"; },
    arrow: () => { "arrow" },
    get getter() { return "getter"; },
    set setter(value) { this._setter = value; }
};
