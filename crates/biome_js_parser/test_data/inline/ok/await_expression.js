async function test() {
  await inner();
  await (inner()) + await inner();
}
async function inner() {
  return 4;
}
await test();
