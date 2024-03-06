function foo() {}

const bar = () => {};

function fooWithNestedEmptyFnBlock() {
  let a = 1;

  function shouldFail(){}

  return a
}


const barWithNestedEmptyFnBlock = () => {
  let a = 1;

  const shouldFail = () => {}

  return a
}

let someVar;
if (someVar) {
}

while (someVar) {
}

switch(someVar) {
}

try {
    doSomething();
} catch(ex) {

} finally {

}

class Foo {
  static {}
}

for(let i; i>0; i++){}

const ob = {}
for (key in ob) {}

const ar = []
for (val of ar) {}

function fooWithInternalEmptyBlocks(){
  let someVar;
  if (someVar) {}

  while (someVar) {
  }

  switch(someVar) {
  }

  try {
      doSomething();
  } catch(ex) {

  } finally {

  }
}