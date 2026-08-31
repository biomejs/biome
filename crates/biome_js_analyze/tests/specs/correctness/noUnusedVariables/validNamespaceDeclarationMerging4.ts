/* should not generate diagnostics */

function F() {}
namespace F {}
F();

const G = () => {};
namespace G {
    export type Props = { id: string };
}
G();
