[1, 2, 3].forEach(async value => {
  await fetch(`/${value}`);
});

new Promise<void>(async (resolve, reject) => {
  await fetch('/');
  resolve();
});
