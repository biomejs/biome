/* should generate diagnostics */

interface Unused {
    first: string;
}

interface Unused {
    second: string;
}

const ValueOnly = 0;
interface ValueOnly {
    prop: string;
}
console.log(ValueOnly);

interface Shadowed {
    outer: string;
}
export function useShadowed() {
    interface Shadowed {
        inner: string;
    }
    type Key = keyof Shadowed;
    return null as Key;
}
