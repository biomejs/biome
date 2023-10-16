function lambdas(array) {
    for (item of array) {              // +1, nesting = 1
        if (item) {                    // +2, nesting = 2
            const a = () => {          // nesting = 3
                if (item > 10) {       // +4, nesting = 4
                    return item;
                }
            };

            function b() {             // nesting = 3
                if (item > 10) {       // +4, nesting = 4
                    return item;
                }
            }

            const c = function() {     // nesting = 3
                if (item > 10) {       // +4, nesting = 4
                    return item;
                }
            };

            const d = function d() {   // nesting = 3
                if (item > 10) {       // +4, nesting = 4
                    return item;
                }
            };
        }
    }
}
