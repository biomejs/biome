/* should not generate diagnostics */
function convert(values: Uint8Array) {
	return values.reduce((object, value) => ({...object, [value]: value}), {});
}
(values as Int32Array).reduce((object, value) => ({...object, [value]: value}), {});
(<Float32Array>values).reduce((object, value) => Object.assign(object, {[value]: value}), {});
function convertNonNull(values: Uint8Array) {
	return values!.reduce((object, value) => ({...object, [value]: value}), {});
}
(new Uint8Array(values) satisfies Uint8Array).reduce((object, value) => ({...object, [value]: value}), {});
const annotated: Float64Array = getValues();
annotated.reduce((object, value) => ({...object, [value]: value}), {});
