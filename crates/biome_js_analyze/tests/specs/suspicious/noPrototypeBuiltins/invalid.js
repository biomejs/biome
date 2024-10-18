foo.hasOwnProperty("bar");
foo.isPrototypeOf(bar);
foo.propertyIsEnumerable("bar");
foo.bar.hasOwnProperty("bar");
foo.bar.baz.isPrototypeOf("bar");
foo["hasOwnProperty"]("bar");
foo[`isPrototypeOf`]("bar").baz;
foo?.hasOwnProperty(bar);
(foo?.hasOwnProperty)("bar");
foo?.["hasOwnProperty"]("bar");
(foo?.[`hasOwnProperty`])("bar");
Object.hasOwnProperty.call(obj, 'foo');
Object.hasOwnProperty.call(obj, property);
Object.prototype.hasOwnProperty.call(obj, 'foo');
({}).hasOwnProperty.call(obj, 'foo');
Object/* comment */.prototype.hasOwnProperty.call(a, b);
const hasProperty = Object.prototype.hasOwnProperty.call(object, property);
const hasProperty1 = (( Object.prototype.hasOwnProperty.call(object, property) ));
const hasProperty2 = (( Object.prototype.hasOwnProperty.call ))(object, property);
const hasProperty3 = (( Object.prototype.hasOwnProperty )).call(object, property);
const hasProperty4 = (( Object.prototype )).hasOwnProperty.call(object, property);
const hasProperty5 = (( Object )).prototype.hasOwnProperty.call(object, property);
const hasProperty6 = {}.hasOwnProperty.call(object, property);
const hasProperty7 = {}.hasOwnProperty.call(object, property);
const hasProperty8 = (( {}.hasOwnProperty.call(object, property) ));
const hasProperty9 = (( {}.hasOwnProperty.call ))(object, property);
const hasProperty10 = (( {}.hasOwnProperty )).call(object, property);
const hasProperty11 = (( {} )).hasOwnProperty.call(object, property);
function foo(){return {}.hasOwnProperty.call(object, property)}
function foo(){return{}.hasOwnProperty.call(object, property)}
function foo(){return/*comment*/{}.hasOwnProperty.call(object, property)}
async function foo(){return await{}.hasOwnProperty.call(object, property)}
async function foo(){return await/*comment*/{}.hasOwnProperty.call(object, property)}
for (const x of{}.hasOwnProperty.call(object, property).toString());
for (const x of/*comment*/{}.hasOwnProperty.call(object, property).toString());
for (const x in{}.hasOwnProperty.call(object, property).toString());
for (const x in/*comment*/{}.hasOwnProperty.call(object, property).toString());
function foo(){return({}.hasOwnProperty.call)(object, property)}
Object['prototype']['hasOwnProperty']['call'](object, property);
Object[`prototype`][`hasOwnProperty`][`call`](object, property);
Object['hasOwnProperty']['call'](object, property);
Object[`hasOwnProperty`][`call`](object, property);
({})['hasOwnProperty']['call'](object, property);
({})[`hasOwnProperty`][`call`](object, property);