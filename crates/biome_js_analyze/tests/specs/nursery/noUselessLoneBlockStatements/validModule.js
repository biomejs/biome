export {}

while (foo) {
 bar();
}

if (foo) {
 if (bar) {
  baz();
 }
}

function bar() {
 baz();
}

{
 let x = 1;
}

{
 const y = 1;
}

{
 class Foo {}
}

aLabel: {
}

class C {
 static {
  lbl: {
   if (something) {
    break lbl;
   }

   foo();
  }
 }
}

if (x) {}

function g() {
 let i;
 for(i = 0; f(i); i++) {}
 return i;
}

{
 function foo() {}
}

if(x) {
  x += 1;
} else {
  x += 2;
}