function f() {
    console.log(arguments);

    for(let i = 0;i < arguments.length; ++i) {
        console.log(arguments[i]);
    }
}

function f() {
    let arguments = 1;
    console.log(arguments);
}