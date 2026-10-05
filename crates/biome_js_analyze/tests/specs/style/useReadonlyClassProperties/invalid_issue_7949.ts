/* should generate diagnostics */

class SomeClass {
	private static VALUE = "ASDF";
}

class StaticAndInstanceMembersAreIndependent {
	private static VALUE = "ASDF";
	private VALUE = "ASDF";

	updateInstance() {
		this.VALUE = "updated";
	}
}

class StaticNamesAreIndependent {
	private static A = "A";
	private static B = "B";

	static update() {
		this.B = "updated";
	}
}

class MutableStatic {
	private static VALUE = "ASDF";

	static update() {
		this.VALUE = "updated";
	}
}

class MutableStaticInBlock {
	private static VALUE = "ASDF";

	static {
		this.VALUE = "updated";
	}
}

class MutableStaticThroughBlockAlias {
	private static VALUE = "ASDF";

	static {
		{
			const Class = this;
			Class.VALUE = "updated";
		}
	}
}

class MutableStaticInInitializer {
	private static VALUE = "ASDF";
	private static readonly UPDATED_VALUE = (this.VALUE = "updated");
}

class MutableComputedStatic {
	private static VALUE = "ASDF";

	static update() {
		this["VALUE"] = "updated";
	}
}

class MutableComputedClassNameStatic {
	private static VALUE = "ASDF";

	static update() {
		MutableComputedClassNameStatic["VALUE"] = "updated";
	}
}

class MutableStringNamedStatic {
	private static "VALUE" = "ASDF";

	static update() {
		this["VALUE"] = "updated";
	}
}

class MutableEmptyStringNamedStatic {
	private static "" = "ASDF";

	static update() {
		this[``] = "updated";
	}
}

class MutableNonNullStatic {
	private static VALUE = "ASDF";

	static update() {
		this.VALUE! = "updated";
	}
}

class MutableParenthesizedThisStatic {
	private static VALUE = "ASDF";

	static update() {
		(this).VALUE = "updated";
	}
}

class MutableNumericStatic {
	private static 1 = "ASDF";

	static update() {
		this[1.0] = "updated";
	}
}

class MutableThroughClassAlias {
	private static VALUE = "ASDF";

	static update() {
		const Class = MutableThroughClassAlias;
		Class.VALUE = "updated";
	}
}

class MutableThroughAssignedThisAlias {
	private static VALUE = "ASDF";

	static update() {
		let Class;
		Class = this;
		Class.VALUE = "updated";
	}
}

class MutableThroughAssignedClassAlias {
	private static VALUE = "ASDF";

	static update() {
		let Class;
		Class = MutableThroughAssignedClassAlias;
		Class.VALUE = "updated";
	}
}

class ThisPropertyIsNotAnAlias {
	private static VALUE = "ASDF";
	private static readonly HOLDER = { VALUE: "ASDF" };

	static update() {
		const holder = this.HOLDER;
		holder.VALUE = "updated";
	}
}

class MutableStaticInForOf {
	private static VALUE = "ASDF";

	static update(values: string[]) {
		for (this.VALUE of values) {
		}
	}
}

const MutableClassExpression = class {
	private static VALUE = "ASDF";

	static update() {
		MutableClassExpression.VALUE = "updated";
	}
};

const MutableParenthesizedClassExpression = (class {
	private static VALUE = "ASDF";

	static update() {
		MutableParenthesizedClassExpression.VALUE = "updated";
	}
});
