// should not generate diagnostics

const a = () => 1;

const b = async () => await Promise.resolve(1);

const c = () => ({ a: 1 });

const d = () => {
    const x = 1;
    return x;
}

const e = () => {
    // comment
    return 1;
}

const f = () => {
    "use strict";
    return 1;
}

const g = () => {
    return;
}
