/* should not generate diagnostics */
function test(this: Foo) {
  function test2(this: Bar) {}
}
