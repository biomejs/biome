/* should generate diagnostics */
const fromArray = <div {...[1, 2, 3]} />;

class Component {}
const fromClass = <div {...Component} />;

declare const map: Map<string, number>;
const fromMap = <div {...map} />;

const promise = new Promise(() => {});
const fromPromise = <div {...promise} />;

const fromDate = <div {...new Date()} />;
