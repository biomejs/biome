import { useEffect, useState } from 'library-reexporting-react';
import * as ReactReexport from 'library-reexporting-react';

function MyComponent25() {
    const [calc, setCalc] = useState(0);
    // Built-in hooks such as `useEffect()` normally only get validated when
    // they're imported from the "react" library. Explicitly configuring them
    // in the hooks array (as if they are user-provided hooks) overrides this.
    useEffect(() => {
        if (calc === 0) {
            setCalc(1);
        }
    }, []);
}

function MyComponent26() {
    const [calc, setCalc] = ReactReexport.useState(0);
    // Built-in hooks such as `useEffect()` normally only get validated when
    // they're imported from the "react" library. Explicitly configuring them
    // in the hooks array (as if they are user-provided hooks) overrides this.
    ReactReexport.useEffect(() => {
        if (calc === 0) {
            setCalc(1);
        }
    }, []);
}
