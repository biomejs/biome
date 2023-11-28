interface I {
  new (): I;
}

interface I {
	constructor(): void;
}

// Works for generic type.
interface G {
  new (): G<T>;
}

type T = {
  constructor(): void;
};

class C {
  new(): C;
}

class C {
  new(): this;
}

declare abstract class C {
  new(): C;
}

interface I {
  constructor(): '';
}

interface I {
  new(): this;
}
