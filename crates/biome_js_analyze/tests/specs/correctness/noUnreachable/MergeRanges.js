function MergeRanges1() {
    return;

    const var1 = statement1();
    const var2 = statement2();

    label: {
        statement1();
        statement2();
    };

    do {
        statement1();
        statement2();
    } while (test);

    for (const a of b) {
        statement1();
        statement2();
    }

    for (const a in b) {
        statement1();
        statement2();
    }

    for (let i = 0; i < 10; i++) {
        statement1();
        statement2();
    }

    if (a) {
        statement1();
        statement2();
    } else if (b) {
        statement1();
        statement2();
    } else {
        statement1();
        statement2();
    }

    switch (value) {
        case 1:
            statement1();
            statement2();
            break;
        default:
            statement1();
            statement2();
            break;
    }

    try {
        statement1();
        statement2();
    } catch (err) {
        statement1();
        statement2();
    }

    try {
        statement1();
        statement2();
    } catch (err) {
        statement1();
        statement2();
    } finally {
        statement1();
        statement2();
    }
    
    while (cond) {
        statement1();
        statement2();
    }
}

function MergeRanges2() {
    while (true) {
        if (expr) {
            return;
            afterReturn();
        } else {
            continue;
            afterContinue();
        }
        
        const var1 = statement1();
        const var2 = statement2();

        if (a) {
            statement1();
            statement2();
        } else if (b) {
            statement1();
            statement2();
        } else {
            statement1();
            statement2();
        }

        label: {
            statement1();
            statement2();
        };
    }
}
