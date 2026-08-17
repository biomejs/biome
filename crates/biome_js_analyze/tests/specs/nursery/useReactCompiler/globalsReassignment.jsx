// should generate diagnostics

let shared = 0;

function Component() {
    shared = shared + 1;

    return <div>{shared}</div>;
}
