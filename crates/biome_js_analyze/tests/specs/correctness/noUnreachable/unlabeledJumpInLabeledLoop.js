// should generate diagnostics
function breakInLabeledFor() {
    outer: for (;;) {
        break;
        afterBreak();
    }
}

function breakInLabeledForOf(xs) {
    outer: for (const x of xs) {
        break;
        afterBreak();
    }
}

function breakInLabeledForIn(o) {
    outer: for (const k in o) {
        break;
        afterBreak();
    }
}

function breakInLabeledWhile() {
    outer: while (true) {
        break;
        afterBreak();
    }
}

function breakInLabeledDoWhile() {
    outer: do {
        break;
        afterBreak();
    } while (true);
}

function breakInLabeledSwitch(x) {
    outer: switch (x) {
        case 1:
            break;
            afterBreak();
    }
}

function breakInLabeledBlockInLoop() {
    for (;;) {
        block: {
            break;
        }
        afterBlock();
    }
}

function continueInLabeledFor(xs) {
    outer: for (let i = 0; i < xs.length; i++) {
        continue;
        afterContinue();
    }
}

function continueInLabeledWhile(x) {
    outer: while (x) {
        continue;
        afterContinue();
    }
}

function continueInLabeledDoWhile(x) {
    outer: do {
        continue;
        afterContinue();
    } while (x);
}
