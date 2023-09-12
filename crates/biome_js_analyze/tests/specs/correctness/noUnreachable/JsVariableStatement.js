function JsVariableStatement1() {
    return;
    var variable;
}

function JsVariableStatement2() {
    return;
    var variable = initializer();
}

function JsVariableStatement3() {
    return;
    let variable;
}

function JsVariableStatement4() {
    return;
    let variable = initializer();
}

function JsVariableStatement5() {
    return;
    const variable = initializer();
}

function JsVariableStatement6() {
    return;
    var variable1 = initializer(),
        variable2 = initializer();
}
