const apply = (callback: (n: number) => number) => callback(1);
const fn = (flag: boolean) => ({ value: flag ? flag : { result: apply((n) => n) } });
