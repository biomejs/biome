function JsLabeledStatement1() {
    label: while (true) {
        if (true) {
            continue label;
        } else {
            break label;
        }

        afterLabelJump();
    }
}

function JsLabeledStatement2() {
    label: {
        beforeBreak();
        break label;
        afterBreak();
    }

    afterBlock();
}
