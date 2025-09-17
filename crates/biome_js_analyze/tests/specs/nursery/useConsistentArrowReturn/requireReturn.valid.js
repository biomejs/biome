/* should not generate diagnostics */
const a = () => 0;
const b = () => "hello";
const c = (a, b) => a + b;
const d = () => {return { a: 1 }};

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

function foo() {
    return 0;
}

const noReturnArrow = () => {};

function conditionalReturn(a) {
    if (a) {
        return 1;
    }
    return 0;
}


const withComment = () => {
  // intentional comment
  return 1;
};


const emptyReturn = () => {
  return;
};

const withDirective = () => {
  "use strict";
  return 1;
};

const emptyReturnWithComment = () => {
  return; // explicitly empty
};

const withInlineComment = () => { return 1; /* inline */ };
const withBlockCommentBefore = () => { /* leading */ return 1; };
const withCommentBetweenReturnAndExpr = () => { return /* comment */ 1; };

const returnsSequenceArrow = () => (a, b)
const returnsAwaitArrow = async () => await fetchData()

