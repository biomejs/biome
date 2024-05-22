throw new Error();
throw new Error(error);
throw Error(error);

try {
	throw new Error();
} catch (e) {
	throw e;
}

function foo() {
	return new Error();
}
throw foo();

const fooObj = {
	bar: new Error(),
};
throw fooObj.bar;

const fooObj1 = {
	bar: new Error(),
};

throw fooObj1["bar"];

class CustomError extends Error {}
throw new CustomError();

class CustomError1 extends Error {}
class CustomError2 extends CustomError1 {}
throw new CustomError2();

throw (foo1 = new Error());
throw (1, 2, new Error());
throw foo ? new Error() : new Error();
function* fooGenerator() {
	let index = 0;
	throw yield index++;
}

async function fooAsync() {
	throw await bar;
}

// Cases below should be invalid, but are not.
// TODO. Need type inference to handle them properly.
throw new String("");
const err = "error";
throw err;
function fooFn(msg) {}
throw fooFn("error");
const fooObjInvalid = {
    msg: "error",
};
throw fooObjInvalid.msg;
const fooMsg = {
    msg: undefined,
};
throw fooMsg.msg;
class CustomError {}
throw new CustomError();
class Foo {}
class CustomError extends Foo {}
throw new CustomError();
{
    const Error = null;
    throw Error;
}
