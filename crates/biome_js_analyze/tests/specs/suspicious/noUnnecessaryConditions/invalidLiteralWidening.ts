// A `const`, a `readonly` field, and an `as const` assertion keep the literal
// types of their values, as in TypeScript, so these tests are always truthy
// or always falsy.
const limit = 0;
export function hasLimit() {
    return limit ? "limited" : "unlimited";
}

export class Defaults {
    readonly empty = "";
    isEmpty() {
        return this.empty ? "no" : "yes";
    }
}

let enabled = true as const;
export function isEnabled() {
    return enabled ? "on" : "off";
}
