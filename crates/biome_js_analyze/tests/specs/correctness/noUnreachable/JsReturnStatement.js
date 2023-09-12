function JsReturnStatement1() {
    return;
    afterReturn();
}

function JsReturnStatement2() {
    return;
    return; // afterReturn
}
