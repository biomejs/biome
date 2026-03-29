/* should not generate diagnostics */

function registry() {}
namespace registry {
    export const item = 1;
}
console.log(registry.item);
