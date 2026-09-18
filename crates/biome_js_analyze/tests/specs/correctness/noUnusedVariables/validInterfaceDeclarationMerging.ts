/* should not generate diagnostics */

interface Things {
    foo: string;
}

interface Things {
    bar: string;
}

type Key = keyof Things;

interface Things {
    baz: string;
}

export function doStuff(key: Key) {
    return key;
}
