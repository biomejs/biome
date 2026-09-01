/* should not generate diagnostics */
<Foo onClick={() => console.log('Hello!')} />
<Foo onClick={() => alert("1337")} />

function Foo() {
  const onClick = () => {};
  return <Bar onClick={onClick} />;
}
