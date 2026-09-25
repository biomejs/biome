/* should not generate diagnostics */
declare const optional: { then(callback?: () => void): void };
declare const nullable: { then(callback: (() => void) | null): void };
declare const explicitThis: { then(this: unknown, callback: () => void): void };
declare const overloaded: { then(value: number): void; then(callback: () => void): void };
void optional;
void nullable;
void explicitThis;
void overloaded;
declare class ComputedThen {
    ["then"](value: number): void;
    ["then"](callback: () => void): void;
}
declare const computed: ComputedThen;
void computed;
