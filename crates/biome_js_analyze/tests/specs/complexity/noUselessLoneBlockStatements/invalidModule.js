export {}

{}

if (foo) {
 bar();
 {
  baz();
 }
}

if (foo) {
 // this is bar
 bar();
 {
  // this is baz
  baz();
 }
}

function bar() {
 {
  baz();
 }
}

{
 function foo() {}
}

{
 // a comment
 function foo() {}
}

{
 aLabel: {
 }
}

class C {
 static {
  {
   foo();
  }
 }
}