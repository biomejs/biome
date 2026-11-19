// should generate diagnostics
function annotated(): void {}
function generic<T>(value: T): T { return value; }
function other(value: string): string;
function unrelated() {}
{
    function other(value: number) { return value; }
}

