/* should generate diagnostics */

export function f() {
    let h = () => h();
    const recursive = () => recursive();
    const parenthesized = () => (parenthesized());
    const asynchronous = async () => await asynchronous();
}
