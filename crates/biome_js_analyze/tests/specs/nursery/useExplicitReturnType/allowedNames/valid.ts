/* should not generate diagnostics */

// Function declarations with allowed names
function test1() {
  return;
}

const foo = function test2() {
  return;
};

// Const-assigned function with allowed name
const test1b = function () {
  return;
};
const foo2 = function () {
  return function test2() {};
};

// Arrow functions with allowed names via variable
const test1c = () => {
  return;
};
export const fooObj = {
  test2() {
    return 0;
  },
};

// Class with allowed names
class Test {
  constructor() {}
  get prop() {
    return 1;
  }
  set prop(val: number) {}
  method() {
    return;
  }
  arrow = () => 'arrow';
}

// Object member with allowed names
const x = {
  arrowFn: () => {
    return;
  },
  fn: function () {
    return;
  },
};
