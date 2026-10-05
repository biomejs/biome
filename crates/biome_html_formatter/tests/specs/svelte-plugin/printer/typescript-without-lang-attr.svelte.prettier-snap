<script>
    interface A {
        b: string;
    }
    type Foo<T> = T extends true ? A & { b: boolean } : string;
    type SeussFish = `${Quantity | Color} fish`;
    const a: A = { b: "" };

    function foo(a: A): void {
        const foo = <Foo<true>>a;
        return null as any;
    }

    const bar: () => void = () => {};

    abstract class B<T> {
        t: T;
    }

    enum E {
        a = 1,
    }
</script>
