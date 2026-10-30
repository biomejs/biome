/* should generate diagnostics */
pairs.reduce(object => ({...object, key}), {});
pairs.reduce(object => ({...object, /* keep this */ key}), {});
pairs.reduce(object => ({...object, key}), {},);
pairs.reduce(object => ({...object, key,}), {});
pairs.reduce(object => ({...object, key}), (( {} )));
pairs.reduce( (( object => ({...object, key}) )) , {});
pairs.reduce( (( (object) => ({...object, key}) )) , {});
pairs.reduce( (( (object,) => ({...object, key}) )) , {});
pairs.reduce(object => ({...object, [((key))] : ((value))}), {});
pairs.reduce(object => ({...object, 0: value}), {});
pairs.reduce(object => ({...object, true: value}), {});
pairs.reduce(object => ({...object, default: value}), {});
pairs.reduce(object => ({...object, "string": value}), {});
pairs.reduce(object => ({...object, ["__proto__"]: value}), {});
pairs.reduce(object => ({...object, [0]: value}), {});
pairs.reduce(object => ({...object, 0n: value}), {});
pairs.reduce(object => Object.assign(object, {key}), {});
pairs.reduce(object => Object.assign(object, {[key]: value,}), {});
pairs.reduce(object => globalThis.Object.assign(object, {[key]: value}), {});
pairs.reduce((object, element, index, array) => ({...object, key}), {});
pairs.reduce((object, [key, value], index, array,) => ({...object, [key]: value + index + array.length}), {});
pairs.reduce(object => ({...object, key: function (object) { return object; }}), {});
const object1 = pairs.reduce((object, [key, value]) => ({...object, [key]: value}), {});
pairs.reduce(object => ({...object, key}), /* keep */ {});
[1, 2].reduce((object, value) => ({...object, [value]: value}), {});
Array.from(values).reduce((object, value) => ({...object, [value]: value}), {});
(
	(pairs)
		.reduce(
			((object,) => ((
				Object.assign(
					((object)),
					({
						[((key))]: ((value)),
					})
				)
			))),
			({})
		)
);

_.fromPairs(pairs);
lodash.fromPairs(pairs);
(_.fromPairs)(pairs);

const object2 = {};
for (const [key, value] of pairs) {
	object2[key] = value;
}

const object3 = ({});
for (let [key, value] of ((pairs))) object3[(key)] = (value);

const object4 = {
	// Comment
};
for (const [key, value] of pairs) {
	object4[key] = value;
}

const object5 = {};
// Comment
for (const [key, value] of pairs) {
	object5[key] = value;
}

const object6 = {};
for (const [key, value] of pairs) {
	object6[key] = value;
} // Comment

const object7 = {}
for (const [key, value] of pairs) {
	object7[key] = value;
}
[1].forEach(foo);

// The fix would move the optional chain inside `Object.fromEntries()`.
a?.b.reduce((object, [key, value]) => ({...object, [key]: value}), {});

// The fix would call the local `Object` variable.
function shadowedObject(Object) {
	const object = {};
	for (const [key, value] of pairs) {
		object[key] = value;
	}
	return [_.fromPairs(pairs), pairs.reduce((object, [key, value]) => ({...object, [key]: value}), {})];
}
