function nestedControlFlowStructures(num) {
    try {
        if (condition1) {                        // +1
            for (let i = 0; i < 10; i++) {       // +2 (nesting=1)
                while (condition2) { /* ... */}  // +3 (nesting=2)
            }
        }
    } catch (error) {                            // +1
        if (condition2) { /* ... */}             // +2 (nesting=1)
    }
}
