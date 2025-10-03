let arguments = 0;
function f() {
    console.log(arguments);

    for(let i = 0;i < arguments.length; ++i) {
        console.log(arguments[i]);
    }
}

function g() {
    function h() {
        console.log(arguments);
    }
    let arguments = 1;
}
