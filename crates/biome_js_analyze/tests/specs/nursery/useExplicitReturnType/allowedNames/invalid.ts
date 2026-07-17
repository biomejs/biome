/* should generate diagnostics */

// Names NOT in the allowed list
function hoge() {
  return;
}
const fooVar = () => {
  return;
};
const baz = function () {
  return;
};

// Destructured assignment doesn't match allowedNames
let [unlistedName] = [
  function () {
    return;
  },
];

// Computed member name doesn't match allowedNames
class X {
  [test] = function () {
    return;
  };
}

// Numeric key doesn't match allowedNames
const xObj = {
  1: function () {
    return;
  },
};

// Computed property name doesn't match string
const ignoredName = 'notIgnoredName';
class Foo {
  [ignoredName]() {}
}
