/* should generate diagnostics */
const container = {};
const name = "name";
const getName = () => "aaa";
const nested = { foo: { bar: "bar" } };

delete container["aa" + "b"];
delete container[+7];
delete container[-Infinity];
delete container[+Infinity];
delete container[NaN];
delete container[name];
delete container[getName()];
delete container[nested.foo.bar];
delete container[+"Infinity"];
delete container[typeof 1];
delete container[`aaa`];
delete container[-"1"];
delete container[1n];
delete container?.[name];
delete (container[name]);
delete container.inner[name];
