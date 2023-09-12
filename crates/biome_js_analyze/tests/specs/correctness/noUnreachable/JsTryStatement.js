function JsTryStatement1() {
    try {
        tryBlock();
        return;
    } catch (err) {
        catchClause();
        return;
    }

    afterTryCatchReturn();
}

function JsTryStatement2() {
    try {
        tryBlock();
        return;
    } catch (err) {
        catchClause();
    }

    afterTryCatchReturn();
}

function JsTryStatement3() {
    return;

    try {
        tryBlock();
    } catch (err) {
        catchClause();
    }
}

function JsTryStatement4() {
    try {
    } catch (err) {
        catchClause();
    }
}
