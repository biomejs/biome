/* should not generate diagnostics */

function getWords(num) {
    switch (num) {                        // +1
        case 1:
            return "one";
        case 2:
            return "a couple";
        case 3:
            return "a few";
        default:
            return "lots";
    }
}
