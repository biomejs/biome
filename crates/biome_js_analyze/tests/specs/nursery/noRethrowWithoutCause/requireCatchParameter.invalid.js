
try {
    foo();
} catch {
    throw new Error("fail");
}


try {
    foo();
} catch {
    throw new Error("fail", { cause: err });
}
