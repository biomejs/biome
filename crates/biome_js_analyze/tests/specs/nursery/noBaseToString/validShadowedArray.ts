/* should not generate diagnostics */

class Array<T> {
    join(): string {
        return "custom";
    }
}

declare const values: Array<{}>;
values.join();
