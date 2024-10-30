let { ... } = a;
let { ...c = "default" } = a;
let { ...{a} } = b;
let { ...rest, other_assignment } = a;
let { ...rest2, } = a;
async function test() {
  let { ...await } = a;
}
