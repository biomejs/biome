const obj = {
    foo: foo,
    bar: function () { return "bar"; },
    arrow: () => { "arrow" },
    get getter() { return "getter"; },
    set setter(value) { this._setter = value; }
};
