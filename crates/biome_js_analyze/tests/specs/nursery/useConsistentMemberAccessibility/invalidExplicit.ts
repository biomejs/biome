class InvalidExplicit {
  public constructor(readonly value: string) {}
}

class InvalidExplicit2 {
  x: number;
  public getX() {
    return this.x;
  }
}

class InvalidExplicit3 {
  private x: number;
  getX() {
    return this.x;
  }
}

class InvalidExplicit4 {
  x?: number;
  getX?() {
    return this.x;
  }
}

class InvalidExplicit5 {
  private x: number;
  constructor(x: number) {
    this.x = x;
  }
  get internalValue() {
    return this.x;
  }
  set internalValue(value: number) {
    this.x = value;
  }
}

class InvalidExplicit6 {
  x = 2;
}

class InvalidExplicit7 {
  constructor(public x: any[]) {}
}

abstract class InvalidExplicit8 {
  abstract x: string;
}
