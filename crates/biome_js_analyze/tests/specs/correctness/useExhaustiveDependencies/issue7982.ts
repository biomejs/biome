import { useState, useCallback } from "react";

function App() {
    const [count, setCount] = useState(0);

    // All three of these should generate a diagnostic.

    const correctError = useCallback(() => {
        return count * 2;
    }, []);

    // Reported as a bug for not generating a diagnostic.
    const falseNegative = useCallback(
        (() => {
            return count * 2;
        }) as Function,
        [],
    );

    const recommendedWorkaround = useCallback(() => {
        return count * 2;
    }, []) as Function;
}
