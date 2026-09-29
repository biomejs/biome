// should generate diagnostics
export const getters = {
    get whileBreak() { while (true) { break; } },
    get doWhileBreak() { do { break; } while (true); },
    get labeledBreak() { outer: while (true) { while (true) { break outer; } } },
    get dynamic() { while (condition) {} },
    get falsy() { do {} while (false); },
    get zero() { while (0.0) {} },
    get empty() { do {} while (""); },
};
