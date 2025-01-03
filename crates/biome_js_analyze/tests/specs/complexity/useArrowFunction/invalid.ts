const f1 = async function<T> (x: T): Promise<T> {
    return x;
}

const f2 = async function<T> (x: T): Promise<object> {
    return {};
}

const f3 = /*a*/ async /*b*/ function /*c*/ <T> /*d*/ (x: T /*e*/)/*f*/: Promise<T>/*g*/ {
    return x;
} /* end */

const f4 = async function<T> (x: T): Promise<T> {
    return x;
} // Trailing comment

const f5 = async function<T> (x: T): Promise<T> {
    return x; // Comment
}

const f6 = function() {
    function inner () {
        return this;
    }
    return 0;
}

function f7() {
    const self = this;
    return function() {
        if (self instanceof Number) {
            return self;
        } else {
            return null;
        }
    };
}

const f10 = function(x) {
    return 0, 1;
}

const as = function () {} as () => void;
const satisfies = function () {} satisfies () => void;
const unary = +function () {};
const conditionalTest = function () {} ? true : false;
class ExtendsClause extends function() {} {};
const non_null_assertion = function () {}!;
const call = function () {}();
const staticMember = function(a) {}.bind(null, 0);
const computedMember = function(a) {}["bind"](null, 0);
const logical = false || function () {};
const binary = false + function () {};

const withDirective = function () {
	"use server";
	return 0;
}
