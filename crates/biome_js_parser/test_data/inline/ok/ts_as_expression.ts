let x: any = "string";
let y = x as string;
let z = x as const;
let not_an_as_expression = x
as;
let precedence = "hello" as const + 3 as number as number;
