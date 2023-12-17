function f (x) {
    if (x < 0) {
        throw new RangeError();
    } else {
        return x;
    }
}

function f (x) {
    f();
    if (x < 0) {
        throw new RangeError();
    } else {
        g();
    }
    h();
}

function f (x) {
    if (x < 0) {
        throw new RangeError();
    } else return x;
}

function f (x) {
    if (x < 0)
        throw new RangeError();
    else
        return x;
}

function f (x) {
    if (x < 0) {
        throw new RangeError();
    } else if (x === 0) {
        return 1;
    } else {
        return x;
    }
}

function f (x) {
    while (true) {
        if (x < 0) {
            break;
        } else {
            x -= g(x)
        }
    }
    return x;
}

function f (x) {
    while (true) {
        if (x < 0) {
            break;
        } else {
            x -= g(x)
        }
    }
    return x;
}

function f (x) {
    if (x > 0 && x < 5) {
        switch (x) {
            case 0:
            case 1:
                return 0;
            default:
                return x;
        }
    } else {
        return x;
    }
}

function f (x) { // 0
    // 1
    if (x > 0 && x < 5) {
        switch (x) {
            case 0:
            case 1:
                return 0;
            default:
                return x;
        }
    } /*a*/ else /*b*/ { // c
        // d
        return x; // e
        // f
    } // g
    // 2
} // 3

function f (x) {
    if (x > 0 && x < 5) {
        return 0;
    }
    // Some explanations
    else {
        return x;
    }
}
