/* should not generate diagnostics */
const container: { [i: string]: 0 } = {};

delete container.aaa;
delete container[7];
delete container["aaa"];
