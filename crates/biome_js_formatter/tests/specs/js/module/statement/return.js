function f1() {
    return 1
}

function f2() {
    return 1,3,4
}

function f3() {
    return /* commment */'1'
}

function f4() {
    return (
        /* comment */
        '1'
    )
}

function f5() {
    return (
        /*
         * multiline comment 
         */ '1'
    )
}