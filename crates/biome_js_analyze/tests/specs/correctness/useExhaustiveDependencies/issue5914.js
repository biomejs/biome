import { useMemo } from "react";

function Component1() {
    const
        varA = Math.random(),
        // Incorrect diagnostic reported: varA should be removed from deps.
        varB = useMemo(() => varA, [varA]);
}

function Component2() {
    const
        varA = Math.random(),
        // Missing diagnostic reported: varA should be added to deps.
        varB = useMemo(() => varA, [])
}

function Component3() {
    const varA = Math.random();
    // Diagnostic is not expected to be generated here.
    const varB = useMemo(() => varA, [varA]);
}

function Component4() {
    const varA = Math.random();
    // Diagnostic is properly generated here.
    const varB = useMemo(() => varA, []);
}

