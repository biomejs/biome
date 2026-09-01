/* should not generate diagnostics */
<Foo onClick={function () { console.log('Hello!'); }} />

function Foo() {
  function onClick() {}
  return <Bar onClick={onClick} />;
}

function Foo() {
  const onClick = function () {};
  return <Bar onClick={onClick} />;
}
