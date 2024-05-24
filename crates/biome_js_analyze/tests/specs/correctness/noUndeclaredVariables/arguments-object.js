// valid
function foo() {
    console.log(arguments);
}

// invalid
console.log(arguments);

const bar = () => {
    console.log(arguments);
}