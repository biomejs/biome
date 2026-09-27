/* should not generate diagnostics */

// https://github.com/biomejs/biome/issues/11903
// The trailing `return;` is required by TypeScript, because another code path of
// the function returns a value.
export function pick(flag: boolean) {
    if (flag) {
        return 1;
    }
    return;
}

// A declared return type that is not `void`, `undefined` or `any` cannot be
// satisfied by an implicit fallthrough.
export function annotatedWithUnion(flag: boolean): number | undefined {
    if (flag) {
        return 1;
    }
    return;
}

export function annotatedWithUnknown(): unknown {
    return;
}

export async function annotatedAsync(flag: boolean): Promise<string> | undefined {
    if (flag) {
        return "value";
    }
    return;
}

class Foo {
    // A getter declares its return type with a type annotation.
    get value(): string | undefined {
        if (this.flag) {
            return "value";
        }
        return;
    }

    flag = false;
}
