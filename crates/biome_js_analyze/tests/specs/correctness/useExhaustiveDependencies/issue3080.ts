import { useEffect } from 'react';

function ReactComponent({ foo, bar }: { foo: string; bar: number }) {
    // Properly generates diagnostics: foo and bar are missing from deps.
    useEffect(() => {
        console.log('foo', foo);
        console.log('bar', bar);
    }, []);

    function myEffect() {
        console.log('foo', foo);
        console.log('bar', bar);
    }

    // Missing diagnostics reported: foo and bar are missing from deps.
    useEffect(myEffect, []);
}
