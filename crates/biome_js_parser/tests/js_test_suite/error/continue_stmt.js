function foo() { continue; }
while (true) {
  continue foo;
}
foo: {
  continue foo;
}
