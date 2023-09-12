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
