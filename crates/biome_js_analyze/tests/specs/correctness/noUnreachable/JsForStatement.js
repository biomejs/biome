function JsForStatement1() {
    for (let i = 0; i < 10; ++i) {
        break;
    }
}

function JsForStatement2() {
    for (;;) {}
    afterLoop();
}
