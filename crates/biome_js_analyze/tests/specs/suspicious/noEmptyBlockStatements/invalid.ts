function fooEmptyTs() {}

const barEmptyTs = () => {};

function fooWithNestedEmptyFnBlockTs() {
  let a = 1;

  function shouldFail(){}

  return a
}


const barWithNestedEmptyFnBlockTs = () => {
  let a = 1;

  const shouldFail = () => {}

  return a
}

const someVarTs: string = '';
if (someVarTs) {
}

while (someVarTs) {
}

switch(someVarTs) {
}

const doSomething = () => null;
try {
    doSomething();
} catch(ex) {

} finally {

}

class FooEmptyStaticTs {
  static {}
}

for(let i; i>0; i++){}

const obTs = {}
for (const key in obTs) {}

const arTs = []
for (const val of arTs) {}

function fooWithInternalEmptyBlocksTs(){
  let someOtherVar: string = '';
  if (someOtherVar) {}

  while (someOtherVar) {
  }

  switch(someOtherVar) {
  }

  try {
      doSomething();
  } catch(ex) {

  } finally {

  }
}

export class FooBar {
  constructor(
    private foo: string,
  ) {
    function bar() { }
    bar();
  }
}
