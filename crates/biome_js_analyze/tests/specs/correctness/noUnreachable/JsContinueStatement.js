function JsContinueStatement1() {
    while (true) {
        continue;
        continue; // afterContinue
    }
}

function JsContinueStatement2() {
    while (true) {
        continue;
        break; // afterContinue
    }
}
