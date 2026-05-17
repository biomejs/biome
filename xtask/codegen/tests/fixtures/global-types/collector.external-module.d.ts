declare module 'fs' {
    interface ReadOptions {}
    function readFile(path: string): void;
}

declare module "x" {
    namespace N {
        interface I {
            prop: string;
        }
    }
}
