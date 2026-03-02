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
