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

type T5 =
    // comment
    test extends string ? test extends number ? unknown : unknown : undefined;

type Echo<T> = T

type T6 = test extends Echo<string> ? test extends number ? unknown : unknown : undefined;

type T7 = test extends Echo<string> ?  unknown : test extends number ? undefined : undefined;

type T8 = test extends Echo<string> ?
// something
    unknown : test extends number ? undefined :
    // else
        undefined;

type T9 = test extends Echo<string>
    // something
    ? unknown : test extends number ? undefined :
        // else
        undefined;

type T10 =
    // comment
    test extends Echo<string> ? test extends number ? unknown : unknown : undefined;
