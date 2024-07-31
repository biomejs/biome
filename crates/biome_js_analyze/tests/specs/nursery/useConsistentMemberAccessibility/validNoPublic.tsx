class ValidNoPublic {
  protected name: string;
  protected x?: string;
  getX() {
    return this.x;
  }
}

class ValidNoPublic2 {
  name: string;
  foo?: string;
  private x: string;
  getX() {
    return this.x;
  }
  get fooName(): string {
    return this.foo + ' ' + this.name;
  }
}

class ValidNoPublic3 {
  constructor(private x: number) {}
}

class ValidNoPublic4 {
  constructor(foo: number) {}
}

class ValidNoPublic5 {
  x = 2;
  private y = 2;
}

class ValidNoPublic6 {
  private x: any;
  constructor({ x }: { x: any }) {
    this.x = x;
  }
}
