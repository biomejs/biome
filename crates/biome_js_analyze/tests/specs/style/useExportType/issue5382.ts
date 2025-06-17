const ErrorCode = {
	invalidArgument: "invalid-argument",
	internalError: "internal-error",
};

type ErrorCode = (typeof ErrorCode)[keyof typeof ErrorCode];

export {ErrorCode};
