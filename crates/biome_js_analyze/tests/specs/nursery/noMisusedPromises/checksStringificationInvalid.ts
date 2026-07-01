const promise = Promise.resolve('value');

const a = `wtf ${promise}`;

const b = 'wtf ' + promise;

const c = promise + ' wtf';

const d = `${promise} and ${promise}`;

const getData = (): Promise<string> => fetch('/').then((r) => r.text());

const e = `data: ${getData()}`;

const f = 'data: ' + getData();
