/* should not generate diagnostics */
const thing1 = foo ? bar : foobar;

let thing2;

if (foo) {
  thing2 = bar;
} else if (baz === qux) {
  thing2 = quxx;
} else {
  thing2 = foobar;
}

const thing3 = (foo ? bar : baz);

const thing4 = foo ? (bar) : (baz);

const thing5 = ((foo ? bar : baz));
