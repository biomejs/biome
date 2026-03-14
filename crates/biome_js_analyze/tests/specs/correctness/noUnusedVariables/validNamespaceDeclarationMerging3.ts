/* should not generate diagnostics */

function foo() {}
namespace foo {
    export function bar() {
        foo();
    }
}
export default foo;
