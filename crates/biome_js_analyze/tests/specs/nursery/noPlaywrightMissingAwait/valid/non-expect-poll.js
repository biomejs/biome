/* should NOT generate diagnostics - poll() on non-expect objects is valid */
const result = await someObj.poll().getData();
myService.poll().then(data => process(data));
