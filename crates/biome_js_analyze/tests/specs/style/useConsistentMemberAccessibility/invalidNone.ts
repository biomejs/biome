class InvalidNone {
  public constructor(public foo: string) {}
}

class InvalidNone2 {
  public constructor(private readonly foo: string) {}
}

class InvalidNone3 {
  public constructor(protected foo: string) {}
}

class InvalidNone4 {
  protected name: string;
  private x: number;
  public getX() {
    return this.x;
  }
}

class InvalidNone5 {
  protected name: string;
  protected foo?: string;
  public 'foo-bar'?: string;
}

class InvalidNone6 {
  public constructor({ x, y }: { x: number; y: number }) {}
}

class InvalidNone7 {
  protected name: string;
  protected x?: string;
  public getX() {
    return this.x;
  }
}

class InvalidNone8 {
  private x = 2;
}
