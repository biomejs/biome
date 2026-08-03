/* should not generate diagnostics */
const tuple = ["value", 1] as const;
const object = <const>{ value: "value" };

declare let mutable: unknown;
(mutable as const) = "value";
(<const>mutable) = "value";

const annotated: string = "value";
const checked = { value: "value" } satisfies { value: string };

function isString(value: unknown): value is string {
    return typeof value === "string";
}

function narrow(value: string | undefined) {
    if (value !== undefined) {
        return value.length;
    }
}
