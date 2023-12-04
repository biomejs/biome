const fn = b => c => d => {
  return 3;
};

const foo = (a, b) => c => d => {
  return 3;
};

const bar = a => b => c => a + b + c

const mw = store => next => action => {
  return next(action)
}

const middleware = options => (req, res, next) => {
  // ...
};

// Ensure tail bodies only indent a single level when necessary
somePromise.then(({default: ComponentName}) => (props) => [longerSingleElement]);

somePromise.then((reallyLongArguments) => (makeTheChainFullyBreak) => (moreThanItWould) => [longerSingleElement]);

somePromise.then(({ reallyLongArguments }) => (makeTheChainFullyBreak) => [
  dontIndentTwice,
]);
somePromise.then(({ reallyLongArguments }) => (makeTheChainFullyBreak) => {
  dontIndentTwice();
});

somePromise.then(({ reallyLongArguments }) => (makeTheChainFullyBreak) => (andNowAllLines) => (keepGoing) =>
  dontIndentTwice());

  somePromise.then(
      ({ reallyLongArguments }) =>
        (makeTheChainFullyBreak) =>
        () => {
          dontIndentTwice();
        },
    );
  function foo() { 
      // Unmount clean up
      React.useLayoutEffect(() => () => {
        callSomeLongNamedFunction();
      });
    }

    function foo() { 
      // Unmount clean up
      React.useLayoutEffect(() => () => [hello, what, is, this, going, too, doehwharht]);
    }