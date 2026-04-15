/* should not generate diagnostics */

// JS files should not trigger the rule
function test() {
  return;
}

var fn = function () {
  return 1;
};

var arrowFn = () => 'test';
