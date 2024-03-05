import { useMemo } from 'react';

const data = {}
function MyComponent24({ idx }) {
    useMemo(() => {
        data[idx]
    }, [idx])
}