/* should not generate diagnostics */
const a = () => 0;
const b = () => "hello";
const c = (a, b) => a + b;
const d = () => ({ a: 1 });

function multiStatement() {
    const x = 1;
    return x;
}

const multiStatement2 = () => {
    console.log("hello");
};

const multiStatement3 = () => {
    let y = 2;
    y++;
    return y;
}

function noReturn() {
    // I do nothing
}

const noReturnArrow = () => {};

function conditionalReturn(a) {
    if (a) {
        return 1;
    }
    return 0;
}
