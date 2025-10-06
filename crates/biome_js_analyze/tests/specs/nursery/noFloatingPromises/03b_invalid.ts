const obj: { [key: string]: () => Promise<string> } = {
  asyncFunc,
}

async function asyncFunc() {
  return Promise.resolve("foobar")
}

obj.asyncFunc()
