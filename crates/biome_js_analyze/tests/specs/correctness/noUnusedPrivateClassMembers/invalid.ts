class TsBioo {
	private unusedProperty = 5;

	private unusedMethod() {

	};
}

class TSUnusedPrivateConstructor {
	constructor(private nusedProperty = 3){

	}
}


class TsOnlyWrite {
	private usedOnlyInWrite = 5;

	method() {
		this.usedOnlyInWrite = 21;
	}
}

class TsSelfUpdate {
	private usedOnlyToUpdateItself = 5;

	method() {
		this.usedOnlyToUpdateItself++;
	}
}

class TsAccessor {
	private get unusedAccessor() { }
	private set unusedAccessor(value) { }
}
