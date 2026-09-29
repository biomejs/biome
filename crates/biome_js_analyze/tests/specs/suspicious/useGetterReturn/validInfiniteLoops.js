// should not generate diagnostics
export const getters = {
    get whileTrue() { while (true) {} },
    get doWhileTrue() { do {} while (true); },
    get whileTruthy() { while (1) {} },
    get doWhileTruthy() { do {} while ("yes"); },
    get parenthesized() { while ((true)) {} },
    get innerBreak() { while (true) { while (true) { break; } } },
    get continued() { do { continue; } while (true); },
    get returned() { while (true) { return 1; } },
};
