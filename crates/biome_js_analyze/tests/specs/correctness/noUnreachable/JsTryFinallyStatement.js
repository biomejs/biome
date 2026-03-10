// should generate diagnostics

function JsTryFinallyStatement1() {
    try {
        tryBlock();
    } catch (err) {
        catchClause();
    } finally {
        finallyClause();
        return;
    }

    afterFinallyReturn();
}

function JsTryFinallyStatement2() {
    return;

    try {
        tryBlock();
    } catch (err) {
        catchClause();
    } finally {
        finallyClause();
    }
}

function JsTryFinallyStatement3() {
    try {
        try {
            tryBlock1();
        } catch {
        } finally {
            return;
        }

        afterTryStatement1();
    } catch (err) {
        catchClause2();
    }

    afterTryStatement2();
}

function JsTryFinallyStatement4() {
    try {
        tryBlock1();
        return;
    } catch {
        return;
    } finally {
        if (value) {
            statement1();
        } else {
            statement2();
        }

        finallyClause();
    }

    afterTryStatement();
}

// https://github.com/biomejs/biome/issues/4946
// finally block should be reachable when there is a jump (break/continue/return) before it
function JsTryFinallyStatement5() {
    while (true) {
        try {
            break;
        } finally {
            console.log("reachable");
        }
        console.log("unreachable");
    }
}

function JsTryFinallyStatement6() {
    while (true) {
        try {
            continue;
        } finally {
            console.log("reachable");
        }
        console.log("unreachable");
    }
}

function JsTryFinallyStatement7() {
    try {
        return;
    } finally {
        console.log("reachable");
    }
    console.log("unreachable");
}

// finally itself contains a return: code after the try/finally is unreachable
// due to the finally's return (not the try's return)
function JsTryFinallyStatement8() {
    try {
        return 1;
    } finally {
        return 2;
    }
    console.log("unreachable");
}

// nested try/finally: both inner and outer finally blocks should be reachable
function JsTryFinallyStatement9() {
    while (true) {
        try {
            try {
                break;
            } finally {
                console.log("inner finally reachable");
            }
        } finally {
            console.log("outer finally reachable");
        }
        console.log("unreachable");
    }
}
