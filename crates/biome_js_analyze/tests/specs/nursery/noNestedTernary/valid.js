/* should not generate diagnostics */
const thing = foo ? bar : foobar;

let thing;

if (foo) {
  thing = bar;
} else if (baz === qux) {
  thing = quxx;
} else {
  thing = foobar;
}