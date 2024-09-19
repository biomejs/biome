function test(a: number, b: number) {
  return;
}

function test() {
  return;
}

var fn = function () {
  return 1;
};

var arrowFn = () => 'test';

class Test {
  constructor() {}
  get prop() {
    return 1;
  }
  set prop() {}
  method() {
    return;
  }
  arrow = () => 'arrow';
  private method() {
    return;
  }
}