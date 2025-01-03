function test(a: string): void;
function test(a: string | undefined): void {}
function no_semi(a: string)
function no_semi(a: string) {}
async function async_overload(a: string)
async function async_overload(a: string) {}
