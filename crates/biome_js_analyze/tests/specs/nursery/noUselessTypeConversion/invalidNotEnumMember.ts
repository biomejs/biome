/* should generate diagnostics */

// Only a member read from the enum has the type of the enum. A variable
// declared as `string`, a literal, and a member of an object that isn't an
// enum are converted needlessly.
enum Ids {
    A = "a",
}

const id: string = Ids.A;
id.toString();
"a".toString();

const NotAnEnum = { A: "a" } as const;
NotAnEnum.A.toString();

function shadowed() {
    const Ids = { A: "a" } as const;
    Ids.A.toString();
}
