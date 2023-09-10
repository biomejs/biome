let async;
async function fn() {
  for await (async of [7]);
}
