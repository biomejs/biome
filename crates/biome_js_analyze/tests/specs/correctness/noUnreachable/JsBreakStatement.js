function JsBreakStatement1() {
    while (true) {
        break;
        break; // afterBreak
    }
}

function JsBreakStatement2() {
    while (true) {
        break;
        continue; // afterBreak
    }
}
