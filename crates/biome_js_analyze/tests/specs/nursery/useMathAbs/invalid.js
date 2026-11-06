/* should generate diagnostics */
value < 0 ? -value : value;
value <= 0 ? -value : value;
value > 0 ? value : -value;
value >= 0 ? value : -value;
value < 0 ? 0 - value : value;
value <= 0 ? 0 - value : value;
value < 0 ? value * -1 : value;
value < 0 ? -1 * value : value;
object.value < 0 ? -object.value : object.value;
object["value"] < 0 ? -object.value : object[`value`];
this.value < 0 ? -this.value : this.value;
0 > value ? -value : value;
0 >= value ? -value : value;
0 < value ? value : -value;
0 <= value ? value : -value;
(value) < (0) ? (-(value)) : (value);
function returned() {return value < 0 ? -value : value;}
function returnedInParentheses() {return(value < 0 ? -value : value);}
function returnedAfterKeyword() {return(value) < 0 ? -value : value;}
const absolute = value > limit || value < -limit;
const absoluteInclusive = value >= limit || value <= -limit;
const absoluteReversed = value < -limit || value > limit;
const absoluteReversedInclusive = value <= -limit || value >= limit;
const absoluteFlipped = limit < value || -limit > value;
const absoluteFlippedInclusive = limit <= value || -limit >= value;
const absoluteSubtracted = value > limit || value < 0 - limit;
const absoluteMultiplied = value > limit || value < limit * -1;
const absoluteLiteral = value > 10 || value < -10;
const absoluteChained = value > limit || value < -limit || other;
function rangeReturned() {return value > limit || value < -limit;}
function rangeReturnedInParentheses() {return(value > limit || value < -limit);}
function rangeReturnedAfterKeyword() {return(value) > limit || value < -limit;}
const absoluteMember = object.value > limit || object.value < -limit;
const absoluteMemberLimit = value > object.limit || value < -object.limit;
if (number > POSITIVE_CONSTANT || number < -POSITIVE_CONSTANT) {
	console.log(number);
}
if (number >= POSITIVE_CONSTANT || number <= -POSITIVE_CONSTANT) {
	console.log(number);
}
function numberVariable() {
	const value = 1;
	return value < 0 ? -value : value;
}

// The comments at the edges are kept.
/* before */ value < 0 ? -value : value; /* after */
/* before */ value > limit || value < -limit; /* after */

// No fix is offered, because it would remove the comments.
value < 0 ? -/* comment */value : value;
value > limit || value < -/* comment */limit;
