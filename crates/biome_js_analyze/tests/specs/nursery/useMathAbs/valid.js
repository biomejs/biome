/* should not generate diagnostics */
const absolute = Math.abs(value);
Math.abs(value) > limit;
Math.abs(value) >= limit;

// The branches are in the wrong order, or the condition doesn't compare with zero.
value < 0 ? value : -value;
value > 0 ? -value : value;
value < 1 ? -value : value;
value === 0 ? -value : value;
value < 0 ? -otherValue : value;
value < 0 ? -value : otherValue;
value < 0 ? +value : value;
value < 0 ? 1 - value : value;
value < 0 ? value * -2 : value;
value < -0 ? -value : value;

// Calls and calculations can return a different value each time.
(value + 1) < 0 ? -(value + 1) : value + 1;
getValue() < 0 ? -getValue() : getValue();

// Bigint values can't be passed to `Math.abs()`.
value < 0n ? -value : value;
value < BigInt(0) ? -value : value;
function bigintVariable() {
	const value = 1n;
	return value < 0 ? -value : value;
}
function bigintCallVariable() {
	const value = BigInt(1);
	return value < 0 ? -value : value;
}
function bigintDefault(value = 1n) {
	return value < 0 ? -value : value;
}
function bigintCallDefault(value = BigInt(1)) {
	return value < 0 ? -value : value;
}
function bigintDestructuredDefaults({ first = 1n, renamed: second = 1n }, [third = 1n]) {
	return [first < 0 ? -first : first, second < 0 ? -second : second, third < 0 ? -third : third];
}
function bigintRange() {
	const value = 1n;
	const limit = 2n;
	return value > limit || value < -limit;
}
value > 0n || value < -0n;
value > BigInt(0) || value < -BigInt(0);

// These conditions don't compare one value with a limit and its negative.
value > limit || value < limit;
value > limit || otherValue < -limit;
value > limit || value <= -limit;
value > limit || value < -otherLimit;
value > limit && value < -limit;
value > limit ?? value < -limit;
value < limit || value > -limit;
other || value > limit || value < -limit;

// `Math` is not the built-in object here.
function shadowedMath(Math, value, limit) {
	return [value < 0 ? -value : value, value > limit || value < -limit];
}
