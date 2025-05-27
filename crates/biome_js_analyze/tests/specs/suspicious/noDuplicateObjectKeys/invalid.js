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
