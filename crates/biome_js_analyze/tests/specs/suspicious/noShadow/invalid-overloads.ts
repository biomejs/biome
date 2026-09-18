/* should generate diagnostics */

// Shadowing overload implementation parameter inside body with let/const
class ShadowInBody {
    constructor(a: string);
    constructor(a: number);
    constructor(a: unknown) {
        let a = 4;
    }
}

// Rest parameter in implementation shadows outer variable
const items = [1, 2, 3];
function restShadow(...items: string[]): void;
function restShadow(...items: number[]): void;
function restShadow(...items: (string | number)[]) {}
