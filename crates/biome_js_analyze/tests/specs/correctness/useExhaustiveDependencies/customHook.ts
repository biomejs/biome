import { useEffect } from "react";

declare function useMyEffect(cb: Function, args?: any[]): void;
declare function useOtherEffect(args: any[], cb: Function): void;
declare function useEvenMoreEffect(blank: any, args: any[], cb: Function): void;
declare function doSomething(args: any[],blank: any, cb: Function): void;

function MyComponent() {
    let a = 1;
    useMyEffect(() => {
        console.log(a);
    }, [a]);

    useMyEffect(() => {
        console.log(a);
    }, []);

    useMyEffect(() => {
       console.log(a);
    }, "i'm not an array" as any);

    useMyEffect(() => {
       console.log(a);
    });
}

function OtherComponent() {
    let a = 1;
    let b = 1;
    useOtherEffect([a, b], () => {
        console.log(a, b);
    });

    useOtherEffect("also not an array" as any, () => {
       console.log(a-b);
    });
    // @ts-expect-error - biome should be able to lint incorrectly typed code
    useOtherEffect(() => {
       console.log(b);
    });
}

function EvenMoreComponent() {
    let a = 1;
    let b = 112345;
    useEvenMoreEffect([], ["foo"], () => {
        console.log(a % b);
    });

    useEvenMoreEffect("not an array", "also not an array" as any, () => {
       console.log(a);
    });
    useEvenMoreEffect([], [a, b], () => {
       console.log(a);
    });
}

function SomethingComponent() {
    let a = "1";
    let b = 112345;
    doSomething([], "apple", () => {
        console.log(a + b);
    });
    doSomething(["a b"], [a], () => {
       console.log(a);
    });
    doSomething("apple" as any, [], () => {
       console.log(a);
    });
    doSomething([a, b], [], () => {
       console.log(a);
    });
    // swap around the callback and array indices
    doSomething((() => {
       console.log(a);
    }) as any, [], [a, b] as any);
}
