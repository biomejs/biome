// should generate diagnostics
namespace Global_NS {
    export const v = 1;
    const local_value = 2;
    console.log(local_value);
}
import global_namespace = Global_NS;
function my_global_fn(BAD_PARAM: number) {
    const local_value = BAD_PARAM;
    return local_value;
}
const Global_Const = 1;
let global_let = 2;
var global_var = 3;
class global_class {}
interface global_interface {}
type global_type = number;
enum global_enum { Value }
const { value: global_alias } = { value: 1 };
const [global_element] = [1];
const functionExpression = function inner_fn() { return inner_fn; };
const classExpression = class inner_class {};
{
    var hoisted_global = 1;
    let block_local = 2;
    console.log(block_local);
}
console.log(Global_NS.v, my_global_fn(Global_Const), global_let, global_var,
    global_class, global_enum.Value, global_alias, global_element, hoisted_global);
