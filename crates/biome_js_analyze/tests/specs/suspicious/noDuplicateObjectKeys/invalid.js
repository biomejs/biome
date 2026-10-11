const O1 = {
    // Comment 1
    a: 0,
    // Comment 2
    a: 1
};
const O2 = {
    // Comment 1
    f: 0,
    // Comment 2
    f() {}
};
const O3 = {
    // Comment 1
    get prop() { return 0 },
    // Comment 2
    prop() {}
};
const O4 = {
    // Comment 1
    set prop(prop) {},
    // Comment 2
    prop() {}
};
const O5 = {
    // Comment 1
    get prop() { return 0 },
    // Comment 2
    get prop() { return 0 },
};
const O6 = {
    // Comment 1
    set prop(prop) {},
    // Comment 2
    set prop(prop) {},
};
const O7 = { 0x1: 1, 1: 2 };
const O8 = { 1.0: 1, 1: 2 };
const O9 = { 1e1: 1, 10: 2 };
const O10 = { 1_000: 1, 1000: 2 };
const O11 = { [1n]: 1, 1: 2 };
const O12 = { [0x10n]: 1, 16: 2 };
const O13 = { [0o101]: 1, 65: 2 };

