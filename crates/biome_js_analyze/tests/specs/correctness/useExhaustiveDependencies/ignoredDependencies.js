import { useEffect } from "react";

// Make sure disabling a single dependency doesn't disable all.
function IgnoredDependencies1() {
    let a = 1;
    let b = 2;
    // biome-ignore lint/correctness/useExhaustiveDependencies(b): `b` is ignored, but there should still be a diagnostic for `a`
    useEffect(() => {
        console.log(a + b);
    }, []);
}

// Make sure an unnecessarily ignored dependency triggers a diagnostic.
function IgnoredDependencies2() {
    let a = 1;
    // biome-ignore lint/correctness/useExhaustiveDependencies(a): `a` is correctly specified, so we shouldn't ignore it
    useEffect(() => {
        console.log(a);
    }, [a]);
}
