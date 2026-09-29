// should generate diagnostics
export function whileTrue() { while (true) {} return 1; }
export function doWhileTrue() { do {} while (true); return 1; }
export function whileTruthy() { while (1) {} return 1; }
export function doWhileTruthy() { do {} while ("yes"); return 1; }
export function parenthesized() { while ((true)) {} return 1; }
export function innerBreak() { while (true) { while (true) { break; } } return 1; }
export function continued() { do { continue; } while (true); return 1; }
