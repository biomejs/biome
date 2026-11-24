// should not generate diagnostics
let output;
export function wrap($state) {
    output = $state.raw(new Date());
    output.setDate(1);
}
