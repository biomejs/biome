class InvalidNoPublic {
  public name: string;
  public x?: string;
  public getX() {
    return this.x;
  }
}

class InvalidNoPublic2 {
  protected name: string;
  public x?: string;
  getX() {
    return this.x;
  }
}

class InvalidNoPublic3 {
  public x: number;
  public getX() {
    return this.x;
  }
}

class InvalidNoPublic4 {
  protected constructor(public x: number) {}
  public foo(): string {
    return 'foo';
  }
}

class InvalidNoPublic5 {
  constructor(public readonly x: number) {}
}

class InvalidNoPublic6 {
  public x = 2;
  private y = 2;
}

class InvalidNoPublic7 {
  public /*public*/ constructor(private foo: string) {}
}

class InvalidNoPublic8 {
  @public
  public foo() {}
}

class InvalidNoPublic9 {
  @public
  public foo;
}

class InvalidNoPublic10 {
  public foo = '';
}

class InvalidNoPublic11 {
  constructor(public /* Hi there */ readonly foo) {}
}

class InvalidNoPublic12 {
  constructor(public readonly foo: string) {}
}

class InvalidNoPublic13 {
  public constructor() {}
}

class InvalidNoPublic14 {
  public /* */ constructor() {}
}

class InvalidNoPublic15 {
  public 'foo' = 1;
  public 'foo foo' = 2;
  public 'bar'() {}
  public 'bar bar'() {}
}

abstract class InvalidNoPublic16 {
  public abstract method(): string;
}

abstract class InvalidNoPublic17 {
  public abstract x: string;
}
