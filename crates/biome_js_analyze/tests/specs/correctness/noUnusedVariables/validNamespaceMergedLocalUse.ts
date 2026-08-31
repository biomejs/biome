/* should not generate diagnostics */

function F() {}
namespace F {}

F();

const G = () => {};
namespace G {
    export type Props = { id: string };
}

G();
export type GProps = G.Props;

class H {}
namespace H {
    export type Options = { deep: boolean };
}

new H();
