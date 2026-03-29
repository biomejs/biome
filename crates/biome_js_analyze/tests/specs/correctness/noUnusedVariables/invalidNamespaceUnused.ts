/* should generate diagnostics */

namespace Standalone {
    export type X = string;
}

const Both = () => {};
namespace Both {
    export type Y = number;
}

namespace Shadowed {
    export type Z = boolean;
}
function useShadowed() {
    const Shadowed = 42;
    console.log(Shadowed);
}
