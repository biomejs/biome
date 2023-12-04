interface Example {
 (): string;
}

function foo(example: { (): number }): number {
 return example();
}