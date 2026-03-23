/* should generate diagnostics */

// Standalone unused namespace (no merged value declaration)
namespace Standalone {
    export type X = string;
}

// Both namespace and value are unused
const Both = () => {};
namespace Both {
    export type Y = number;
}

// Same-name binding in a nested scope must NOT suppress the diagnostic
namespace Shadowed {
    export type Z = boolean;
}
function useShadowed() {
    const Shadowed = 42;
    console.log(Shadowed);
}
