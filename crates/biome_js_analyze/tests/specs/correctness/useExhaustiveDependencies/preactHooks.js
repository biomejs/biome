import { useCallback } from 'preact/compat';
import { useState } from 'preact/hooks';

function useCounter() {
    const [value, setValue] = useState(0);
    const increment = useCallback(() => {
        setValue(value + 1);
    }, []);
    return { value, increment };
}
