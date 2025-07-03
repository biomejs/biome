const [result] = [(() => Promise.reject("destructuring bypass"))()];
result
