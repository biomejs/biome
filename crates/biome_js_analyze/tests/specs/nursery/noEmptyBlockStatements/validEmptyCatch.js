/* should not generate diagnostics */
try {
  doSomething();
} catch(ex) {}

function foo(){
  try {
      doSomething();
  } catch(ex) {}
}

const bar = () => {
  try {
      doSomething();
  } catch(ex) {}
}