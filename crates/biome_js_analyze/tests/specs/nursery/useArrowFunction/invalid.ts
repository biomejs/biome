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
