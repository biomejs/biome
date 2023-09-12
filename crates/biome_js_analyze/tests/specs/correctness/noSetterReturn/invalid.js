class Basic {
	set foo(x) {
		return x;
	}
}

const BasicObject = {
	set foo(x) {
		return x;
	},
};

class BlockReturn {
	set foo(x) {
		{
			return x;
		}
	}
}

class DoWhile {
	set foo(x) {
		do {
			return x;
		} while (true)
	}
}

class Else {
	set foo(x) {
		if (x) {
		} else {
			return x;
		}
	}
}

class ElseIf {
	set foo(x) {
		if (x) {
		} else if (x) {
			return x;
		}
	}
}

class If {
	set foo(x) {
		if (x) {
			return x;
		}
	}
}

class Labelled {
	set foo(x) {
		label: return x
	}
}

class ForOfBlock {
	set foo(xs) {
		for (x of xs) {
			return x;
		}
	}
}

class ForOfSingleStatement {
	set foo(xs) {
		for (x of xs) return x;
	}
}

class ForInBlock {
	set foo(xs) {
		for (x in xs) {
			return x;
		}
	}
}

class ForInSingleStatement {
	set foo(xs) {
		for (x in xs) return x;
	}
}

class ForBlock {
	set foo(x) {
		for (;;) {
			return x;
		}
	}
}

class ForSingleStatement {
	set foo(x) {
		for (;;) return x;
	}
}

class SwitchCaseReturn {
	set foo(x) {
		switch (x) {
			case 1:
				return x;
			default:
				break;
		}
	}
}

class SwitchDefaultReturn {
	set foo(x) {
		switch (x) {
			case 1:
				break;
			default:
				return x;
		}
	}
}

class TryReturnCatch {
	set foo(x) {
		try {
			return x;
		} catch {}
	}
}

class TryCatchReturn {
	set foo(x) {
		try {
		} catch {
			return x;
		}
	}
}

class TryReturnCatchFinally {
	set foo(x) {
		try {
			return x;
		} catch {
		} finally {
		}
	}
}

class TryCatchReturnFinally {
	set foo(x) {
		try {
		} catch {
			return x;
		} finally {
		}
	}
}

class TryCatchFinallyReturn {
	set foo(x) {
		try {
		} catch {
		} finally {
			return x;
		}
	}
}

class WhileBlock {
	set foo(x) {
		while (true) {
			return x;
		}
	}
}

class WhileSingleStatement {
	set foo(x) {
		while (true) return x;
	}
}
