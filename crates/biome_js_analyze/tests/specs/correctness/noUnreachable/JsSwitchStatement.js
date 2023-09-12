function JsSwitchStatement1() {
    switch (value) {
        case 0:
            case0();
            break;
        default:
            caseDefault();
            break;
        case 1: // afterDefault
            afterDefault();
    }
}

function JsSwitchStatement2() {
    switch (value) {
        case 0:
            break;
            afterBreak();
    }
}

function JsSwitchStatement3() {
    return;
    switch (value) {}
}
