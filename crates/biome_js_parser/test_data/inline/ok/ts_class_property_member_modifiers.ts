class Base {
  base1;
}
abstract class Test extends Base {
    declare a: string;
    private declare c: string;
    declare private d: string;
    declare private readonly e: string;
    private readonly declare f: string;
    declare private static readonly g: string;
    private static readonly declare h: string;
    protected readonly abstract i: string;
    protected abstract readonly j: string;
    protected override readonly base1: string;
    private static accessor readonly k: string;
    protected abstract accessor readonly l: string;
}
