/* should not generate diagnostics */
const doSomethingTsValid = () => null;
try {
  doSomethingTsValid();
} catch(ex) {}

function fooTsValid(){
  try {
    doSomethingTsValid();
  } catch(ex) {}
}

const barTsValid = () => {
  try {
    doSomethingTsValid();
  } catch(ex) {}
}