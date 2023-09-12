function JsDoWhileStatement1() {
    do {
        break;
    } while (true); // afterBreak
}

function JsDoWhileStatement2() {
    do {
        continue;
        afterContinue();
    } while (true);
}
