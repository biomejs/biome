type test = string;

type T1 = test extends string ? test extends number ? unknown : unknown : undefined;

type T2 = test extends string ?  unknown : test extends number ? undefined : undefined;

type T3 = test extends string ?
// something
    unknown : test extends number ? undefined :
    // else
        undefined;

type T4 = test extends string
    // something
    ? unknown : test extends number ? undefined :
        // else
        undefined;