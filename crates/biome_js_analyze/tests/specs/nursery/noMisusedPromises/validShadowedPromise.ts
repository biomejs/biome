/* should not generate diagnostics */

class Promise<T> {}
declare const value: Promise<void>;

if (value) {}
