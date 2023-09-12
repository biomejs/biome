function JsForInStatement1() {
    for (const key in value) {
        break;
        afterBreak();
    }
}

function JsForInStatement2() {
    for (const key in value) {
        continue;
        afterContinue();
    }
}
