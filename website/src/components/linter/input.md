```js
function test(callback) {
  try {
    return callback();
  } catch (e) {
    console.log(e);
    throw e;
  }

  return 20;
}
```