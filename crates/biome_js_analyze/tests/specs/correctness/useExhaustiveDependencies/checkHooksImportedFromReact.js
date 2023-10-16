function MyComponent1() {
  let a = 1;
  React.useEffect(() => {
      console.log(a);
  });

  // the rule doesn't show the warnings because the hooks are not imported from react.
  useEffect(() => {
    console.log(a);
  });
}

function MyComponent2() {
  let a = 1;
  const React = { useEffect() {} }
  // the rule doesn't show the warnings because `React` is defined by the user.
  React.useEffect(() => {
      console.log(a);
  });
}
