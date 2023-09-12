function JsForOfStatement1() {
    for (const key of value) {
        break;
        afterBreak();
    }
}

function JsForOfStatement2() {
    for (const key of value) {
        continue;
        afterContinue();
    }
}
